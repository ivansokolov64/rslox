use std::fmt;
use std::fmt::Formatter;
use crate::token::Token;
use crate::interpreter::LoxObject;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Literal(Option<LoxObject>),
    Grouping(Box<Expr>),
    Unary {
        operator: Token,
        right: Box<Expr>
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary { left, operator, right } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Literal(literal) => {
                match literal {
                    None => {
                        write!(f, "nil")
                    }
                    Some(lit) => {
                        write!(f, "{lit}")
                    }
                }
            }
            Expr::Grouping(expr) => {
                write!(f, "(group {expr})")
            }
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
        }
    }
}