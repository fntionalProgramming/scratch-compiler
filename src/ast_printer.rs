use crate::error::LoxError;
use crate::literal::Literal;
use crate::token::Token;
use crate::token_types::TokenType;
use std::fmt;

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
    right: Box<Expr>,
}

// parenthesize will be call on grouping
struct Grouping {
    middle: Box<Expr>,
}
// Token is restructed to "-" and "!", this will be checked later when it comes to parsing
struct Unary {
    oeprator: Token,
    right: Box<Expr>,
}

// --------------------------------------------
//            AST_PRINTER
// --------------------------------------------
// holds the printed text, so building it happens once and Display just shows it
struct AstPrinter {
    text: String,
}

impl AstPrinter {
    pub fn new(expr: &Expr) -> Result<Self, LoxError> {
        Ok(AstPrinter {
            text: AstPrinter::parenthesize(expr)?,
        })
    }
    // return a new String instead of reference
    // because imagine a bunch of references ting to one function
    // turns any expression into its parenthesized text, e.g. (* (- 123) (group 45.67))
    // function is called when initialized AstPrinter
    fn parenthesize(expr: &Expr) 
    -> Result<String, LoxError> 
    {
        // pick the name and the sub-expressions from the kind of expression
        let (name, exprs): (&str, Vec<&Expr>) = match expr {
            Expr::Binary(binary) => (
                binary.operator.lexeme(),
                vec![&*binary.left, &*binary.right],
            ),
            Expr::Grouping(grouping) => ("group", vec![&*grouping.middle]),
            Expr::Unary(unary) => (unary.oeprator.lexeme(), vec![&*unary.right]),
            // a literal has nothing inside it, so it's just its value
            Expr::Literal(literal) => return Ok(literal.to_string()),
        };

        // print the sub-expressions first, since their length isn't known until then
        // recursively call to print the sub expression
        let mut parts: Vec<String> = Vec::with_capacity(exprs.len());
        for sub_expr in exprs {
            parts.push(AstPrinter::parenthesize(sub_expr)?);
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
}

impl fmt::Display for AstPrinter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // pad instead of write_str so widths like {:>30} still work
        f.pad(&self.text)
    }
}


// example
// and run this code from bin
pub fn main() {
    // -123
    let negative = Expr::Unary(Unary {
        oeprator: Token::new(TokenType::Minus, "-".to_string(), Literal::Nil, 1),
        right: Box::new(Expr::Literal(Literal::Number(123.0))),
    });
    // (45.67)
    let group = Expr::Grouping(Grouping {
        middle: Box::new(Expr::Literal(Literal::Number(45.67))),
    });
    // -123 * (45.67)
    let expression = Expr::Binary(Binary {
        left: Box::new(negative),
        operator: Token::new(TokenType::Star, "*".to_string(), Literal::Nil, 1),
        right: Box::new(group),
    });

    match AstPrinter::new(&expression) {
        Ok(printer) => println!("{}", printer),
        Err(error) => eprintln!("{}", error),
    }
}
