use crate::expr::Expr;
use crate::object::{LoxObject, LoxType};
use crate::token::{Token, TokenType};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::{fmt, io};

#[derive(Debug)]
pub enum ScannerError {
    UnexpectedCharacter(char),
    StringNotTerminated,
    InvalidNumber,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidToken(TokenType, TokenType),
    OutOfBounds,
    ExpectExpression,
    InvalidAssignmentTarget,
    TooManyArguments
}

#[derive(Debug)]
pub enum RuntimeError {
    InvalidOperand {
        expected: LoxType,
        received: LoxObject,
    },
    EvaluationError(Expr),
    NonTruthyValue(LoxObject),
    NoneEval,
    DivisionByZero,
    UndefinedVariable(String),
    InvalidCallable
}

#[derive(Debug)]
pub enum LoxError {
    ParseError(Option<Token>, ParseError),
    ScannerError(usize, ScannerError),
    RuntimeError(Token, RuntimeError),
    IoError(io::Error),
}

impl From<io::Error> for LoxError {
    fn from(e: io::Error) -> Self {
        LoxError::IoError(e)
    }
}

impl Error for ParseError {}
impl Error for ScannerError {}
impl Error for RuntimeError {}

impl Error for LoxError {}

impl Display for ScannerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ScannerError::UnexpectedCharacter(c) => {
                write!(f, "Unexpected character {}", c)
            }
            ScannerError::StringNotTerminated => {
                write!(f, "String not terminated'")
            }
            ScannerError::InvalidNumber => {
                write!(f, "Error parsing number")
            }
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidToken(t1, t2) => {
                write!(
                    f,
                    "Encountered an invalid token; expected {:?} but received {:?}",
                    t1, t2
                )
            }
            ParseError::OutOfBounds => {
                write!(f, "Attempting to read token which is out of bounds")
            }
            ParseError::ExpectExpression => {
                write!(f, "Expected an expression")
            }
            ParseError::InvalidAssignmentTarget => {
                write!(f, "Invalid assignment target")
            }
            ParseError::TooManyArguments => {
                write!(f, "Number of arguments cannot exceed 255")
            }
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::InvalidOperand { expected, received } => {
                write!(
                    f,
                    "Invalid operands. Expected: {:?}, Received: {:?}",
                    expected, received
                )
            }
            RuntimeError::EvaluationError(expr) => {
                write!(f, "Could not evaluate expression: {expr}")
            }
            RuntimeError::NonTruthyValue(obj) => {
                write!(f, "Attempting boolean operation on non-truthy value {obj}")
            }
            RuntimeError::NoneEval => {
                write!(f, "Expression evaluates to None")
            }
            RuntimeError::DivisionByZero => {
                write!(f, "Division by zero")
            }
            RuntimeError::UndefinedVariable(name) => {
                write!(f, "Undefined variable {name}")
            }
            RuntimeError::InvalidCallable => {
                write!(f, "Can only call functions and classes.")
            }
        }
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LoxError::ParseError(token, e) => {
                match token {
                    None => {
                        write!(f, "[line NaN] Parser Error: ")?;
                    }
                    Some(t) => {
                        write!(f, "[line {}] Parser Error: ", t.line)?;
                        match t.token_type {
                            TokenType::EOF => write!(f, "at end: ")?,
                            _ => write!(f, "at {}: ", t.lexeme)?,
                        }
                    }
                }
                write!(f, "'{e}'")
            }
            LoxError::ScannerError(line, e) => {
                write!(f, "[line {}] Scanner Error: ", line)?;
                write!(f, "'{e}'")
            }
            LoxError::RuntimeError(token, e) => {
                write!(f, "[line {}] Runtime Error: ", token.line)?;
                match token.token_type {
                    TokenType::EOF => write!(f, "at end: ")?,
                    _ => write!(f, "at {}: ", token.lexeme)?,
                }
                write!(f, "'{e}'")
            }
            LoxError::IoError(e) => {
                write!(f, "'{e}'")
            }
        }
    }
}
