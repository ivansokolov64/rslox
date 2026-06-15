pub mod errors;
pub mod expr;
pub mod interpreter;
pub mod lox;
pub mod object;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod token;
pub mod callables;

use crate::errors::LoxError;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => match lox::run_file(args[1].clone()) {
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => match e {
                LoxError::ParseError(_, _) => ExitCode::from(65),
                LoxError::ScannerError(_, _) => ExitCode::from(65),
                LoxError::RuntimeError(_, _) => ExitCode::from(70),
                LoxError::IoError(_) => ExitCode::FAILURE,
            },
        },
        1 => match lox::run_prompt() {
            Ok(_) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE,
        },
        _ => {
            eprintln!("Usage: rslox [script]");
            ExitCode::FAILURE
        }
    }
}
