use crate::error::LoxError;
use crate::token::Token;
use crate::literal::Literal;
use crate::token_types::TokenType;

// --------------------------------------------
//             CFG IMPLEMENTATION
// --------------------------------------------
//expression     → literal
//                | unary
//                | binary
//                | grouping ;

// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//                | "+"  | "-"  | "*" | "/" ;


enum Expr {
    Binary(Binary),
    Grouping(Grouping), 
    Unary(Unary),
    // the value or just the literal
    Literal(Literal),
}

struct Binary {
    // usage of Box to avoid recursive pointers
    left: Box<Expr>, 
    operator: Token,
    right: Box<Expr>
}

// parenthesize will be call on grouping
struct Grouping {  
    middle: Box<Expr>,
}
// Token is restructed to "-" and "!", this will be checked later when it comes to parsing
struct Unary {
    oeprator: Token,
    right: Box<Expr>
}

// --------------------------------------------
//            AST_PRINTER
// --------------------------------------------
struct AstPrinter {}
impl AstPrinter {
    // return a new String instead of reference
    // because imagine a bunch of references ting to one function
    fn parenthesize(name: &str, exprs: &[Expr])
    -> Result<String, LoxError>
    {
        // print the sub-expressions first, since their length isn't known until then
        let mut parts: Vec<String> = Vec::with_capacity(exprs.len());
        for expr in exprs {
            parts.push(AstPrinter::print(expr)?);
        }

        // "(" + name + (" " + part) for each part + ")"
        let mut capacity = 1 + name.len() + 1;
        for part in &parts {
            capacity += 1 + part.len();
        }

        let mut builder = String::with_capacity(capacity);
        builder.push('(');
        builder.push_str(name);
        for part in &parts {
            builder.push(' ');
            builder.push_str(part);
        }
        builder.push(')');
        return Ok(builder);
    }

    // turns any expression into its parenthesized text, e.g. (* (- 123) (group 45.67))
    fn print(expr: &Expr) -> Result<String, LoxError> {
        match expr {
            Expr::Binary(binary) => {
                // parenthesize takes a slice of Expr, so the two Boxes are borrowed one at a time
                let left = AstPrinter::print(&binary.left)?;
                let right = AstPrinter::print(&binary.right)?;
                let name = binary.operator.lexeme();
                let mut builder = String::with_capacity(1 + name.len() + 1 + left.len() + 1 + right.len() + 1);
                builder.push('(');
                builder.push_str(name);
                builder.push(' ');
                builder.push_str(&left);
                builder.push(' ');
                builder.push_str(&right);
                builder.push(')');
                return Ok(builder);
            }
            Expr::Grouping(grouping) => AstPrinter::parenthesize("group", std::slice::from_ref(&*grouping.middle)),
            Expr::Unary(unary) => AstPrinter::parenthesize(unary.oeprator.lexeme(), std::slice::from_ref(&*unary.right)),
            Expr::Literal(literal) => Ok(literal.to_string()),
        }
    }
}

// example
fn main() {
}