use crate::errors::{LoxError, RuntimeError};
use crate::interpreter::{EnvironmentStack};
use crate::object::{LoxObject, LoxType};
use crate::token::{Token, TokenType};
use std::fmt;
use std::fmt::Formatter;
use crate::callables::Call;

#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Literal(LoxObject),
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Grouping(Box<Expr>),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Ternary {
        if_expr: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },
    Variable(Token),
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Literal(literal) => {
                write!(f, "{literal}")
            }
            Expr::Grouping(expr) => {
                write!(f, "(group {expr})")
            }
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
            Expr::Ternary {
                if_expr,
                then_branch,
                else_branch,
            } => {
                write!(
                    f,
                    "(ternary {if_expr} {then_branch} {else_branch})")
            }
            Expr::Variable(token) => {
                write!(f, "(variable {token}")
            }
            Expr::Assign { name, value } => {
                write!(f, "(assign {name} {value})")
            }
            Expr::Logical { left, operator, right } => {
                write!(f, "(logical {left} {operator} {right})")
            }
            Expr::Call { callee, paren, arguments: _arguments } => {
                write!(f, "(call {callee} {paren})")
            }
        }
    }
}

impl Expr {
    pub fn evaluate(&self, envs: &mut EnvironmentStack) -> Result<LoxObject, LoxError> {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let (l, r) = (left.evaluate(envs)?, right.evaluate(envs)?);

                let cmp_result = l.partial_cmp(&r).ok_or_else(|| {
                    LoxError::RuntimeError(
                        operator.clone(),
                        RuntimeError::InvalidOperand {
                            expected: LoxType::Number,
                            received: r.clone(),
                        },
                    )
                });

                match operator.token_type {
                    TokenType::Comma => Ok(r),
                    TokenType::Greater => Ok(LoxObject::Boolean(cmp_result?.is_gt())),
                    TokenType::GreaterEqual => Ok(LoxObject::Boolean(cmp_result?.is_ge())),
                    TokenType::Less => Ok(LoxObject::Boolean(cmp_result?.is_lt())),
                    TokenType::LessEqual => Ok(LoxObject::Boolean(cmp_result?.is_le())),
                    TokenType::Minus => {
                        (l - r).map_err(|e| LoxError::RuntimeError(operator.clone(), e))
                    }
                    TokenType::Slash => {
                        (l / r).map_err(|e| LoxError::RuntimeError(operator.clone(), e))
                    }
                    TokenType::Star => {
                        (l * r).map_err(|e| LoxError::RuntimeError(operator.clone(), e))
                    }
                    TokenType::Plus => {
                        (l + r).map_err(|e| LoxError::RuntimeError(operator.clone(), e))
                    }
                    TokenType::BangEqual => Ok(LoxObject::Boolean(l != r)),
                    TokenType::EqualEqual => Ok(LoxObject::Boolean(l == r)),
                    _ => Ok(LoxObject::Nil),
                }
            }
            Expr::Literal(expr) => Ok(expr.to_owned()),
            Expr::Grouping(expr) => expr.evaluate(envs),
            Expr::Unary { operator, right } => {
                let r = right.evaluate(envs)?;

                match operator.token_type {
                    TokenType::Minus => match f64::try_from(r) {
                        Ok(n) => Ok(LoxObject::Number(-n)),
                        Err(e) => Err(LoxError::RuntimeError(operator.clone(), e)),
                    },
                    TokenType::Bang => {
                        let res = (!r).map_err(|e| LoxError::RuntimeError(operator.clone(), e))?;
                        Ok(res)
                    }
                    _ => Ok(LoxObject::Nil),
                }
            }
            Expr::Ternary {
                if_expr,
                then_branch,
                else_branch,
            } => {
                let cond = if_expr.evaluate(envs)?;

                if cond.into() {
                    then_branch.evaluate(envs)
                } else {
                    else_branch.evaluate(envs)
                }
            }
            Expr::Variable(token) => envs.get(token),
            Expr::Assign { name, value } => {
                let value = value.evaluate(envs)?;
                envs.assign(name, value.clone())?;
                Ok(value)
            }
            Expr::Logical { left, operator, right } => {
                let l = left.evaluate(envs)?;
                let truthy_l = bool::from(l.clone());

                if let TokenType::Or = &operator.token_type {
                    if truthy_l {
                        return Ok(l);
                    }
                }
                else {
                    if !truthy_l {
                        return Ok(l);
                    }
                }

                right.evaluate(envs)
            }
            Expr::Call { callee, paren, arguments } => {
                let callee = callee.evaluate(envs)?;

                let arguments: Vec<LoxObject> = arguments.iter().map(|a| a.evaluate(envs))
                    .collect::<Result<Vec<_>, _>>()?;
                
                match callee {
                    LoxObject::Callable(lc) => lc.call(envs, arguments),
                    _ => Err(LoxError::RuntimeError(paren.clone(), RuntimeError::InvalidCallable))
                }
            }
        }
    }
}
