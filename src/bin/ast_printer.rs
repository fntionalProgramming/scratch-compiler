// lets `cargo run --bin ast_printer` run the main() inside src/ast_printer.rs.
// it loads the same files main.rs uses, so `crate::token` and the rest still work there
#![allow(dead_code)]

#[path = "../error.rs"]
mod error;
#[path = "../literal.rs"]
mod literal;
#[path = "../token.rs"]
mod token;
#[path = "../token_types.rs"]
mod token_types;
#[path = "../ast_printer.rs"]
mod ast_printer;

fn main() {
    ast_printer::main();
}
