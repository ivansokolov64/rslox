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
    Return(Token, Option<Expr>),
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
    pub fn execute(&self, envs: &mut EnvironmentStack) -> Result<Option<LoxObject>, LoxError> {
        match self {
            Stmt::Expression(expr) => {
                expr.evaluate(envs)?;
                Ok(None)
            }
            Stmt::Print(expr) => {
                let value = expr.evaluate(envs)?;
                println!("{}", value);
                Ok(None)
            }
            Stmt::Var(token, expression) => {
                let value: LoxObject = match expression {
                    None => LoxObject::Nil,
                    Some(expr) => expr.evaluate(envs)?,
                };
                envs.define(&token.lexeme, value);
                Ok(None)
            }
            Stmt::Block(stmts) => {
                envs.push_scope();

                let mut return_value: Option<LoxObject> = None;
                for stmt in stmts {
                    match stmt.execute(envs) {
                        Ok(None) => {}
                        Ok(Some(val)) => {
                            return_value = Some(val);
                            break;
                        }
                        Err(e) => {
                            envs.pop_scope();
                            return Err(e);
                        }
                    }
                }

                envs.pop_scope();
                Ok(return_value)
            }
            Stmt::If { condition, then_branch, else_branch } => {

                let cond = condition.evaluate(envs)?;

                if cond.into() {
                    then_branch.execute(envs)
                }
                else if let Some(else_stmt) = else_branch.as_ref() {
                    else_stmt.execute(envs)
                }
                else {
                    Ok(None)
                }



            }
            Stmt::While { condition, body } => {
                while bool::from(condition.evaluate(envs)?) {
                    if let Some(val) = body.execute(envs)? {
                        return Ok(Some(val))
                    }
                }
                Ok(None)
            }
            Stmt::Function { fun } => {
                let callable = LoxCallable::LoxFunction(fun.to_owned());
                envs.define(&fun.name.lexeme, LoxObject::Callable(Box::new(callable)));
                Ok(None)
            }
            Stmt::Return(_, value) => {

                let value = match value {
                    None => {
                        LoxObject::Nil
                    }
                    Some(expr) => {
                        expr.evaluate(envs)?
                    }
                };

                Ok(Some(value))

            }
        }
    }
}
