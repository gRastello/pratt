mod expr;
mod lexer;
mod parser;
mod pretty_printer;
mod token;

use crate::expr::walk_expr;
use crate::lexer::*;
use crate::parser::*;
use crate::pretty_printer::*;
use std::io;
use std::io::prelude::*;

/// Collection of possible errors.
enum PrettError {
    ScanError(Vec<ScanError>),
    ParseError(ParseError),
}

impl PrettError {
    /// Print an error to screen.
    fn print(&self) {
        match self {
            PrettError::ScanError(errs) => {
                for err in errs.iter() {
                    eprintln!(
                        "Scanning error at column {}: {}",
                        err.position, err.description
                    )
                }
            }
            PrettError::ParseError(err) => eprintln!(
                "Parsing error at '{}': {}",
                err.token.lexeme, err.description
            ),
        }
    }
}

impl From<Vec<ScanError>> for PrettError {
    fn from(errors: Vec<ScanError>) -> Self {
        PrettError::ScanError(errors)
    }
}

impl From<ParseError> for PrettError {
    fn from(error: ParseError) -> Self {
        PrettError::ParseError(error)
    }
}

fn main() {
    run_repl();
}

/// Run the REPL.
fn run_repl() {
    let stdin = io::stdin();

    for l in stdin.lock().lines() {
        if let Ok(line) = l {
            if let Err(error) = run(line) {
                error.print();
            }
        }
    }
}

/// Run an actual piece of source code.
fn run(source: String) -> Result<(), PrettError> {
    // Tokenize.
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens()?;

    // for t in tokens {
    //     println!("{:?}", t);
    // }
    
    // Parse.
    let mut parser = Parser::new(tokens);
    let parsed_expr = parser.parse()?;

    // Print the expr with a pretty printer.
    let mut pretty_printer = PrettyPrinter::new();
    walk_expr(&mut pretty_printer, &parsed_expr);
    println!("{}", pretty_printer.pretty_text);

    Ok(())
}
