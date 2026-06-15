use crate::callables::{LoxCallable, LoxFunction};
use crate::errors::LoxError;
use crate::expr::Expr;
use crate::interpreter::EnvironmentStack;
use crate::object::LoxObject;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
    While {
        condition: Expr,
        body: Box<Stmt>
    },
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Option<Stmt>>
    },
    Function {
       fun: LoxFunction
    }
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
            Stmt::If { condition, then_branch, else_branch } => {

                let cond = condition.evaluate(envs)?;

                if cond.into() {
                    then_branch.execute(envs)?;
                }
                else if let Some(else_stmt) = else_branch.as_ref() {
                    else_stmt.execute(envs)?;
                }

                Ok(())


            }
            Stmt::While { condition, body } => {
                while bool::from(condition.evaluate(envs)?) {
                    body.execute(envs)?;
                }
                Ok(())
            }
            Stmt::Function { fun } => {
                let callable = LoxCallable::LoxFunction(fun.to_owned());
                envs.define(&fun.name.lexeme, LoxObject::Callable(Box::new(callable)));
                Ok(())
            }
        }
    }
}
