use std::{fs::File, io::BufWriter, io::Write};

use crate::error::LoxError;

// AST code generation

struct GenerateAst {}

impl GenerateAst {
    pub fn main<'a>(&self, args: Vec<&'a str>) -> Result<&'a str, LoxError> 
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
    fn define_ast(&self, output_dir: &str, base_name: &str, classes: &[(&str, &str)]) -> Result<(), LoxError>
    {
        let lines = vec![
            "struct Expr {",
        ];
        let path = format!("{}/{}.rs", output_dir, base_name);
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // write to the file
        for line in lines {
            writeln!(writer, "{}", line)?;
        }
        writeln!(writer, "{}", "}")?;
        for class in classes {
            let class_name = &*class.0;
            let fields = &*class.1;
            
        }
        // flush so a failed write is reported here instead of being lost when the writer is dropped
        writer.flush()?;
        return Ok(());
    }
    fn define_type(writer: &BufWriter<File>, base_name: &str, class_name: &str, field_list: &str) {
        
    }
}