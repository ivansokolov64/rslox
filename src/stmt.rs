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
            Stmt::Var(token, expression) => {
                let value: LoxObject;
                match expression {
                    None => {
                        value = LoxObject::Nil;

                    }
                    Some(expr) => {
                        match expr.evaluate(environment)? {
                            None => {
                                value = LoxObject::Nil;
                            }
                            Some(v) => {
                                value = v
                            }
                        }
                    }
                }

                environment.define(token.lexeme.to_string(), value);
                Ok(())


            }
        }
    }
}