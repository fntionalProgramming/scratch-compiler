use crate::error::LoxError;

// AST code generation

struct GenerateAst {}

impl GenerateAst {
    fn main(&self, args: Vec<&str>) {
        if args.len() != 1 {
            // create a new lox error object 
            let err = LoxError::ReadFile { path: (), source: () };
        }
    }
}