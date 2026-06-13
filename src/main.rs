pub mod token;
pub mod scanner;
pub mod lox;
pub mod expr;
pub mod parser;
pub mod interpreter;
pub mod errors;
pub mod stmt;

use std::process::ExitCode;
use crate::errors::LoxError;

fn main() -> ExitCode {

    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 =>
            {
                match lox::run_file(args[1].clone()) {
                    Ok(_) => {
                        ExitCode::SUCCESS
                    }
                    Err(e) => {
                        match e {
                            LoxError::ParseError(_, _) => {
                                ExitCode::from(65)
                            }
                            LoxError::ScannerError(_, _) => {
                                ExitCode::from(65)
                            }
                            LoxError::RuntimeError(_, _) => {
                                ExitCode::from(70)
                            }
                            LoxError::IoError(_) => {
                                ExitCode::FAILURE
                            }
                        }

                    }
                }
            },
        1 => {
             match lox::run_prompt() {
                 Ok(_) => {
                     ExitCode::SUCCESS
                 }
                 Err(_) => {
                     ExitCode::FAILURE
                 }
             }
        },
        _ => {
            eprintln!("Usage: rslox [script]");
            ExitCode::FAILURE
        }
    }

}



