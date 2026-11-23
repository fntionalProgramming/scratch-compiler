use std::fs;
use std::io;
use std::io::Write;
use std::process::exit;

mod token_types;
mod token;
mod scanner;

// take all the stuff in token_types
use token_types::TokenType::*;
use token::Token;
use scanner::Scanner;


pub struct Lox {
    // states if there is an error or no
    had_error: bool
}

impl Default for Lox {
    fn default() -> Self {
        Self {
            had_error: false,
        }
    }
}

impl Lox {
    pub fn main(&mut self, args: String) {
        if args.len() > 1 {
            println!("Usage: jlox [script]");
            exit(64);
        } else if args.len() == 1 {
            let mut path = String::from("");
            let mut chars_handled = args.chars();
            let mut i = 0;
            // since the otherway of getting the first string in a String is too hard
            // we grab it by copying
            // also notice the code will panic if we type a space as the first character
            while i <= args.len() - 1 {
                let c = chars_handled.nth(i).unwrap();
                if c == ' ' {
                    break;
                } else {
                    path.push_str(&c.to_string());
                }
                i += 1;
            }
            Lox::run_file(&self, path);
        } else {
            Lox::run_prompt(self);
        };
    }

    // run
    // scan the token
    fn run(source: &str) {
        let scanner = Scanner::from(source);
        let tokens = scanner.scan_tokens();
        // for now just print the tokens
        for token in &tokens {
            println!("{}", token);
        }
    }
    // run_file
    // take the file path name, then go into that file path, scan then rum
    fn run_file(&self, path: String) -> io::Result<()> {
        let source = fs::read_to_string(path)?;
        Lox::run(&source);
        if self.had_error {
            exit(65);        
        };
        Ok(())
    }
    // run_prompt
    // take in itself, then 
    fn run_prompt(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut line = String::new();

        loop {
            print!("> ");
            io::stdout().flush()?;

            line.clear();
            if stdin.read_line(&mut line)? == 0 {
                break; // EOF
            }
            Lox::run(line.trim_end());
            self.had_error = false;
        }
        Ok(())
    }

    // utilities for reporting errors
    // report essentially just print where the error is
    fn report(
        &mut self,
        line: i32,
        location: &str,
        message: &str,
    ) {
        println!("[Line: {}], Error: {} {}", line, location, message);
        self.had_error = true;
    } 
    // no idea why this exists
    pub fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }
}



fn main() {
    println!("Hello world");
}
