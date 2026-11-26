// --------------------------------------------------------
//       ERRORS
// --------------------------------------------------------
// every way the interpreter can fail lives here, along with its exit code
// exit codes follow the BSD sysexits.h convention, same as crafting interpreters

use std::error::Error;
use std::fmt;
use std::io;
use std::path::PathBuf;

const EX_USAGE: u8 = 64;
const EX_DATAERR: u8 = 65;
const EX_NOINPUT: u8 = 66;
const EX_IOERR: u8 = 74;

// what went wrong while scanning
#[derive(Debug, Clone, PartialEq)]
pub enum ScanErrorKind {
    UnexpectedCharacter(char),
    UnterminatedString,
    NumberTooLarge,
}

impl fmt::Display for ScanErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanErrorKind::UnexpectedCharacter(c) => write!(f, "Unexpected character {c:?}."),
            ScanErrorKind::UnterminatedString => write!(f, "Unterminated string."),
            ScanErrorKind::NumberTooLarge => write!(f, "Number is too large."),
        }
    }
}

// what went wrong while scanning, and where the offending token starts
#[derive(Debug, Clone, PartialEq)]
pub struct ScanError {
    kind: ScanErrorKind,
    line: usize,
    column: usize,
}

impl ScanError {
    pub fn new(kind: ScanErrorKind, line: usize, column: usize) -> Self {
        ScanError { kind, line, column }
    }
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Line: {}, Column: {}] Error: {}",
            self.line, self.column, self.kind
        )
    }
}



impl Error for ScanError {}
// every error that can reach the top of the program
#[derive(Debug)]
pub enum LoxError {
    Usage,
    ReadFile { path: PathBuf, source: io::Error },
    Io(io::Error),
    // the scanner keeps going after an error, so it can report all of them at once
    Scan(Vec<ScanError>),
}

impl LoxError {
    pub fn exit_code(&self) -> u8 {
        match self {
            LoxError::Usage => EX_USAGE,
            LoxError::ReadFile { .. } => EX_NOINPUT,
            LoxError::Io(_) => EX_IOERR,
            LoxError::Scan(_) => EX_DATAERR,
        }
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxError::Usage => write!(f, "Usage: main [script]"),
            LoxError::ReadFile { path, source } => {
                write!(f, "Error: could not read '{}': {}", path.display(), source)
            }
            LoxError::Io(error) => write!(f, "Error: {error}"),
            LoxError::Scan(errors) => {
                for (i, error) in errors.iter().enumerate() {
                    if i > 0 {
                        writeln!(f)?;
                    }
                    write!(f, "{error}")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for LoxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LoxError::ReadFile { source, .. } => Some(source),
            LoxError::Io(error) => Some(error),
            LoxError::Usage | LoxError::Scan(_) => None,
        }
    }
}

impl From<io::Error> for LoxError {
    fn from(error: io::Error) -> Self {
        LoxError::Io(error)
    }
}

impl From<Vec<ScanError>> for LoxError {
    fn from(errors: Vec<ScanError>) -> Self {
        LoxError::Scan(errors)
    }
}
