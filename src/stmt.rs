use crate::errors::LoxError;
use crate::expr::Expr;
use crate::interpreter::{Evaluate};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr)
}

impl Stmt {
    pub fn execute(&self) -> Result<(), LoxError> {
        match self {
            Stmt::Expression(expr) => {
                expr.evaluate()?;
                Ok(())
            }
            Stmt::Print(expr) => {
                let value = expr.evaluate()?;
                match value {
                    None => {
                        println!("none");
                    }
                    Some(v) => {
                        println!("{}", v);
                    }
                }
                Ok(())
            }
        }
    }
}