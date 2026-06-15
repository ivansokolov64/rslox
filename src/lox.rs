// Run a file or the REPL

use crate::errors::LoxError;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::Token;
use std::io;
use std::io::{BufReader, Read, Write};

pub fn run_file(path: String) -> Result<(), LoxError> {
    let mut interpreter = Interpreter::new();
    let file = std::fs::File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    run(buf, &mut interpreter)
}

pub fn run_prompt() -> Result<(), LoxError> {
    let mut buf = String::new();
    let mut interpreter = Interpreter::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("Error flushing stdout");

        buf.clear();
        io::stdin().read_line(&mut buf)?;
        let _ = run(buf.clone(), &mut interpreter);
    }
}

// Execute a line of source code
// For now, just print the tokens

fn run(source: String, interpreter: &mut Interpreter) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);

    let tokens: Vec<Token>;

    match scanner.scan_tokens() {
        Ok(scanned) => {
            tokens = scanned;
        }
        Err(e) => {
            eprintln!("{e}");
            return Err(e);
        }
    }

    let mut parser = Parser::new(tokens);

    let statements = match parser.parse() {
        Ok(statements) => statements,
        Err(e) => {
            eprintln!("{e}");
            return Err(e);
        }
    };

    match interpreter.interpret(statements) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{e}");
            Err(e)
        }
    }
}
