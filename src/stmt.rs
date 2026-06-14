use crate::errors::LoxError;
use crate::expr::Expr;
use crate::interpreter::Interpreter;
use crate::loxobject::LoxObject;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
    Block(Vec<Stmt>)
}

impl Stmt {
    pub fn execute(&self, interpreter: &mut Interpreter) -> Result<(), LoxError> {
        match self {
            Stmt::Expression(expr) => {
                expr.evaluate(interpreter)?;
                Ok(())
            }
            Stmt::Print(expr) => {
                let value = expr.evaluate(interpreter)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Var(token, expression) => {
                let value: LoxObject = match expression {
                    None => {
                        LoxObject::Nil

                    }
                    Some(expr) => {
                        expr.evaluate(interpreter)?
                    }
                };
                interpreter.define(&token.lexeme, value);
                Ok(())


            }
            Stmt::Block(stmts) => {
                interpreter.push_scope();

                for stmt in stmts {
                    stmt.execute(interpreter)?;
                }

                interpreter.pop_scope();
                Ok(())
            }
        }
    }
}