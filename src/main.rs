// START MODULES
mod error;
mod literal;
mod token_types;
mod token;
mod scanner;
mod generate_ast;
// END USER_DEFINED_MODULES
// START GENERATE_MODULES
mod expr;
// END GENERATE_MODULES

use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;
use std::sync::OnceLock;
use error::LoxError;
use scanner::Scanner;
use generate_ast::{END, MAIN_PATH, START};

// check that main.rs has the START and END marker lines, with START first.
// the file is only read the first time this is called; after that the saved answer is returned.
// if main.rs can't be read, this returns false
fn has_module_markers() -> bool {
    static HAS_MARKERS: OnceLock<bool> = OnceLock::new();
    *HAS_MARKERS.get_or_init(|| {
        let source = match fs::read_to_string(MAIN_PATH) {
            Ok(source) => source,
            Err(_) => return false,
        };
        let mut found_start = false;
        for line in source.lines() {
            if line.trim() == START {
                found_start = true;
            } else if found_start && line.trim() == END {
                return true;
            }
        }
        false
    })
}


pub struct Lox;

impl Lox {
    // no arguments opens the prompt, one argument runs that file
    pub fn main(args: &[OsString]) -> Result<(), LoxError> {
        match args {
            [] => Lox::run_prompt(),
            [path] => Lox::run_file(Path::new(path)),
            _ => Err(LoxError::Usage),
        }
    }

    // run
    // scan the token
    fn run(source: &str) -> Result<(), LoxError> {
        let tokens = Scanner::new(source).scan_tokens()?;
        // for now just print the tokens
        let mut stdout = io::stdout().lock();
        for token in &tokens {
            writeln!(stdout, "{}", token)?;
        }
        Ok(())
    }
    // run_file
    // take the file path name, then go into that file path, scan then rum
    fn run_file(path: &Path) -> Result<(), LoxError> {
        let source = fs::read_to_string(path).map_err(|source| LoxError::ReadFile {
            path: path.to_path_buf(),
            source,
        })?;
        Lox::run(&source)
    }
    // run_prompt
    // a mistake in the code is reported and the prompt keeps going
    fn run_prompt() -> Result<(), LoxError> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut line = String::new();

        loop {
            write!(stdout, "> ")?;
            stdout.flush()?;

            line.clear();
            if stdin.read_line(&mut line)? == 0 {
                writeln!(stdout)?;
                return Ok(()); // EOF
            }
            match Lox::run(line.trim_end()) {
                Err(error @ LoxError::Scan(_)) => Lox::report(&error),
                result => result?,
            }
        }
    }

    // report essentially just print the error
    // this ignores write failures since there is nowhere left to report them
    fn report(error: &LoxError) {
        let _ = writeln!(io::stderr(), "{}", error);
    }
}



fn main() -> ExitCode {
    let args: Vec<OsString> = env::args_os().skip(1).collect();
    match Lox::main(&args) {
        Ok(()) => ExitCode::SUCCESS,
        // the reader closed the pipe, for example `main file.lox | head`
        Err(LoxError::Io(error)) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(error) => {
            Lox::report(&error);
            ExitCode::from(error.exit_code())
        }
    }
}
