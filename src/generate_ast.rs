use std::{fs::{self, File}, io::{BufWriter, Write}, vec};

use crate::error::LoxError;

// AST code generation

pub(crate) const START: &str = "// START GENERATE_MODULES";
pub(crate) const END: &str = "// END GENERATE_MODULES";
// relative to where the program is run from; `cargo run` runs from the crate root
pub(crate) const MAIN_PATH: &str = "src/main.rs";

struct GenerateAst {}

impl GenerateAst {
    // generate all required AST
    pub fn main<'a>(&self, args: Vec<&'a str>) 
    -> Result<&'a str, LoxError> 
    {
        if args.len() != 1 {
            return Err(LoxError::FileGeneration);
        }
        let output_dir = args[0];
        let classes: Vec<(&str, &str)>= vec![
            ("Binary", "Expr left, Token operator, Expr right"),
            ("Grouping", "Expr expression"),
            ("Literal", "Object value"),
            ("Unary", "Token operator, Expr right"),
        ];
        // one call writes every class, so the file is only created once
        self.define_ast(output_dir, "Expr", &classes)?;
        return Ok(output_dir);
    }
    /*
    take all the current content in main.rs, copy it
    find the comments for mod, then add the new modules down below 
    
     */
    fn handle_mod(&self, mod_names: &[String])
    -> Result<(), LoxError>
    {
        let main_source = fs::read_to_string(MAIN_PATH)?;
        let mut lines: Vec<&str> = Vec::new();
        for line in main_source.lines() {
            lines.push(line);
        }

        // find the marker lines, trim so indentation or a trailing \r still matches
        let mut i = 0;
        while i < lines.len() && lines[i].trim() != START {
            i += 1;
        }
        if i == lines.len() {
            return Err(LoxError::FileGeneration);
        }
        let start = i;
        i = start + 1;
        while i < lines.len() && lines[i].trim() != END {
            i += 1;
        }
        if i == lines.len() {
            return Err(LoxError::FileGeneration);
        }
        let end = i;

        // keep everything up to and including START, drop whatever was between
        // the markers last time, put the new modules in, then keep END onwards.
        // this covers both cases: old modules there, or nothing there yet
        let mut new_lines: Vec<&str> = Vec::new();
        for i in 0..=start {
            new_lines.push(lines[i]);
        }
        for name in mod_names {
            new_lines.push(name);
        }
        for i in end..lines.len() {
            new_lines.push(lines[i]);
        }

        let mut new_source = new_lines.join("\n");
        new_source.push('\n');
        fs::write(MAIN_PATH, new_source)?;

        return Ok(());
    }
    // writes an enum with one variant per class, each holding that class's struct, e.g.
    // Binary(Binary),
    fn define_enum(&self, writer: &mut BufWriter<File>, base_name: &str, classes: &[(&str, &str)])
    -> Result<(), LoxError>{
        writeln!(writer, "enum {} {{", base_name)?;
        for (class_name, _) in classes {
            writeln!(writer, "    {}({}),", class_name, class_name)?;
        }
        writeln!(writer, "}}")?;
        return Ok(());
    }
    // writes a Visitor trait with one visit method per class, e.g.
    // fn visit_binary_expr(&mut self, expr: &Binary) -> R;
    fn define_visitor(&self, writer: &mut BufWriter<File>, base_name: &str, classes: &[(&str, &str)])
    -> Result<(), LoxError>{
        let base_lower = base_name.to_lowercase();
        writeln!(writer, "trait Visitor<R> {{")?;
        for (class_name, _) in classes {
            writeln!(
                writer,
                "    fn visit_{}_{}(&mut self, {}: &{}) -> R;",
                class_name.to_lowercase(),
                base_lower,
                base_lower,
                class_name
            )?;
        }
        writeln!(writer, "}}")?;
        return Ok(());
    }
    // define the structure of AST
    fn define_ast(&self, output_dir: &str, base_name: &str, classes: &[(&str, &str)]) 
    -> Result<(), LoxError>
    {
        let path = format!("{}/{}.rs", output_dir, base_name);
        let file = File::create(&path)?;
        let mut writer = BufWriter::new(file);

        // imports for the types the fields use.
        // the runtime value type is renamed to Object because the generated
        // file has its own struct called Literal
        writeln!(writer, "use crate::literal::Literal as Object;")?;
        writeln!(writer, "use crate::token::Token;")?;
        writeln!(writer)?;

        for class in classes {
            let class_name = &*class.0;
            let fields = &*class.1;
            self.define_type(&mut writer, base_name, class_name, fields)?;
            writeln!(writer)?;
        }
        self.define_enum(&mut writer, base_name, classes)?;
        writeln!(writer)?;
        self.define_visitor(&mut writer, base_name, classes)?;
        // write the exact mod_name in main
        // and build the module names
        let mut mod_names: Vec<String> = Vec::new();
        for (mod_name, _) in classes {
            mod_names.push(format!("mod {};", mod_name));
        }
        self.handle_mod(&mod_names)?;
         // flush so a failed write is reported here instead of being lost when the writer is dropped
        writer.flush()?;
        return Ok(());
    }
    // write and define the types
    // turns ("Binary", "Expr left, Token operator, Expr right") into a struct and its new()
    fn define_type(&self, writer: &mut BufWriter<File>, base_name: &str, class_name: &str, field_list: &str)
    -> Result<(), LoxError>
    {
        // split "Expr left, Token operator" into (name, rust type) pairs
        let mut fields: Vec<(&str, String)> = Vec::new();
        for field in field_list.split(", ") {
            let (type_name, name) = match field.split_once(' ') {
                Some(parts) => parts,
                None => return Err(LoxError::FileGeneration),
            };
            // the base type is the enum itself, so it has to be boxed or the type would be infinitely sized
            let rust_type = if type_name == base_name {
                format!("Box<{}>", base_name)
            } else {
                type_name.to_string()
            };
            fields.push((name, rust_type));
        }

        // the struct
        writeln!(writer, "struct {} {{", class_name)?;
        for (name, rust_type) in &fields {
            writeln!(writer, "    pub {}: {},", name, rust_type)?;
        }
        writeln!(writer, "}}")?;

        // the constructor
        writeln!(writer, "impl {} {{", class_name)?;
        writeln!(writer, "    fn new(")?;
        for i in 0..fields.len() {
            let (name, rust_type) = &fields[i];
            if i + 1 < fields.len() {
                writeln!(writer, "        {}: {},", name, rust_type)?;
            } else {
                writeln!(writer, "        {}: {}) -> Self {{", name, rust_type)?;
            }
        }
        let mut names = String::new();
        for i in 0..fields.len() {
            if i > 0 {
                names.push_str(", ");
            }
            names.push_str(fields[i].0);
        }
        writeln!(writer, "        {} {{ {} }}", class_name, names)?;
        writeln!(writer, "    }}")?;
        writeln!(writer, "}}")?;

        return Ok(());
    }
}