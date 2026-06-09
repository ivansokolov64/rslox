
// Run a file or the REPL

use std::io;
use std::io::{BufReader, Read, Write};
use crate::expr::Expr;
use crate::interpreter::{Evaluate, Interpreter};
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::Token;



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


    let tokens: Vec<Token>;

    match scanner.scan_tokens() {
        Ok(scanned) => {
            tokens = scanned;
        }
        Err(e) => {
            eprintln!("{e}");
            return Ok(())
        }
    }

    let mut parser = Parser::new(tokens);

    let expression: Expr;

    match parser.parse() {
        Ok(expr) => {
          expression = expr;
        },
        Err(e) => {
            eprintln!("{e}");
            return Ok(())
        }
    }

    let interpreter = Interpreter::new();

    match interpreter.interpret(expression) {
        Ok(result) => {
            match result {
                None => {
                    println!("none")
                }
                Some(obj) => {
                    println!("{obj}")
                }
            }

        }
        Err(e) => {
            eprintln!("{e}");
        }
    }

    Ok(())
}
