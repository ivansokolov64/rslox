use crate::errors::LoxError;
use crate::expr::Expr;
use crate::interpreter::EnvironmentStack;
use crate::loxobject::LoxObject;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
    Block(Vec<Stmt>),
}

impl Stmt {
    pub fn execute(&self, envs: &mut EnvironmentStack) -> Result<(), LoxError> {
        match self {
            Stmt::Expression(expr) => {
                expr.evaluate(envs)?;
                Ok(())
            }
            Stmt::Print(expr) => {
                let value = expr.evaluate(envs)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Var(token, expression) => {
                let value: LoxObject = match expression {
                    None => LoxObject::Nil,
                    Some(expr) => expr.evaluate(envs)?,
                };
                envs.define(&token.lexeme, value);
                Ok(())
            }
            Stmt::Block(stmts) => {
                envs.push_scope();

                for stmt in stmts {
                    stmt.execute(envs)?;
                }

                envs.pop_scope();
                Ok(())
            }
        }
    }
}
