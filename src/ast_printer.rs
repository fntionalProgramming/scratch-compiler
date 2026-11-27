use crate::token::Token;
use crate::literal::Literal;
use crate::token_types::TokenType;


enum Expr {
    Binary {
        
    },
    Grouping {

    },
    Unary, 
    // the value holding the current literal 
    Value(Literal),
}


struct Binary {

}
struct Grouping {}

struct Unary {}


// --------------------------------------------
//            AST_PRINTER
// --------------------------------------------
struct AstPrinter {}
impl AstPrinter {
    fn parenthesize(name: &str, exprs: &[Expr]) {
        
    }

}

fn main() {
    let example_expression = E
}