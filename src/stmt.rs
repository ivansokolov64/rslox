use crate::errors::LoxError;
use crate::expr::Expr;
use crate::interpreter::{Environment};
use crate::loxobject::LoxObject;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>)
}

impl Stmt {
    pub fn execute(&self, environment: &mut Environment) -> Result<(), LoxError> {
        match self {
            Stmt::Expression(expr) => {
                expr.evaluate(environment)?;
                Ok(())
            }
            Stmt::Print(expr) => {
                let value = expr.evaluate(environment)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Var(token, expression) => {
                let value: LoxObject = match expression {
                    None => {
                        LoxObject::Nil

                    }
                    Some(expr) => {
                        expr.evaluate(environment)?
                    }
                };
                environment.define(&token.lexeme, value)?;
                Ok(())


            }
        }
    }
}