
// Run a file or the REPL

use std::io;
use std::io::{BufReader, ErrorKind, Read, Write};
use crate::expr::Expr;
use crate::parser::{ParseError, Parser};
use crate::scanner::Scanner;
use crate::token::{Token, TokenType};


pub fn run_file(path: String) -> io::Result<()> {

    let file = std::fs::File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    run(buf)
}

pub fn run_prompt() -> io::Result<()> {
    let mut buf = String::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("Error flushing stdout");

        buf.clear();
        io::stdin().read_line(&mut buf)?;
        run(buf.clone())?;
    }

}


// Execute a line of source code
// For now, just print the tokens

fn run(source: String) -> io::Result<()> {
    let mut scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();

    let mut parser = Parser::new(tokens);

    match parser.parse() {
        Ok(expr) => {
          println!("{expr}");
          Ok(())
        },
        Err(e) => {
            Err(io::Error::new(io::ErrorKind::Other, "Parsing error"))
        }
    }


}


// Error types

pub fn error(line: usize, message: &str) {
    report(line, "runtime", message.to_string());
}

pub fn parse_error(error: &ParseError) {
    match error {
        ParseError::InvalidToken(t, m) => {
            match t.token_type {
                TokenType::EOF => report(t.line, " at end", m.to_string()),
                _ => {
                    let location = format!(" at '{}'", t.lexeme);
                    report(t.line, &location, m.to_string())
                }
            }
        }
        ParseError::OutOfBounds(l) => {
            report(l.clone(), "runtime", error.to_string())
        }
        ParseError::ExpectExpression(t) => {
            report(t.line, "runtime", error.to_string())
        }
    }
}


pub fn report(line: usize, loc: &str, message: String) {
    eprintln!("[line {}] Error {}: '{}'", line, loc, message);
}