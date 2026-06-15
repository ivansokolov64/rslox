use std::fmt;
use std::fmt::Formatter;
use crate::errors::{LoxError, RuntimeError};
use crate::interpreter::Interpreter;
use crate::token::{Token, TokenType};
use crate::loxobject::{LoxObject, LoxType};

#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Literal(LoxObject),
    Grouping(Box<Expr>),
    Unary {
        operator: Token,
        right: Box<Expr>
    },
    Ternary {
        if_expr: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
        operator: Token
    },
    Variable(Token)
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary { left, operator, right } => {
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
            Expr::Ternary { if_expr, then_branch, else_branch , operator} => {
                write!(f, "(ternary {} {if_expr} {then_branch} {else_branch})", operator.lexeme)
            }
            Expr::Variable(token) => {
                write!(f, "(variable {token}")
            }
            Expr::Assign { name, value } => {
                write!(f, "(assign {name} {value})")
            }
        }
    }
}

impl Expr {
    pub fn evaluate(&self, interpreter: &mut Interpreter) -> Result<LoxObject, LoxError> {
        match self {
            Expr::Binary {left, operator, right} => {

                let (l, r) = (left.evaluate(interpreter)?, right.evaluate(interpreter)?);

                let cmp_result = l.partial_cmp(&r)
                    .ok_or_else(|| LoxError::RuntimeError(operator.clone(), RuntimeError::InvalidOperand {expected: LoxType::Number, received: r.clone() }));

                match operator.token_type {
                    TokenType::Comma => {
                        Ok(r)
                    },
                    TokenType::Greater => {
                        Ok(LoxObject::Boolean(cmp_result?.is_gt()))
                    },
                    TokenType::GreaterEqual => {
                        Ok(LoxObject::Boolean(cmp_result?.is_ge()))
                    },
                    TokenType::Less => {
                        Ok(LoxObject::Boolean(cmp_result?.is_lt()))
                    },
                    TokenType::LessEqual => {
                        Ok(LoxObject::Boolean(cmp_result?.is_le()))
                    },
                    TokenType::Minus => {
                        (l-r).map_err(|e| LoxError::RuntimeError(operator.clone(), e))
                    },
                    TokenType::Slash => {
                        (l/r).map_err(|e| LoxError::RuntimeError(operator.clone(), e))
                    },
                    TokenType::Star => {
                        (l*r).map_err(|e| LoxError::RuntimeError(operator.clone(), e))
                    },
                    TokenType::Plus => {
                        (l+r).map_err(|e| LoxError::RuntimeError(operator.clone(), e))
                    },
                    TokenType::BangEqual => Ok(LoxObject::Boolean(l != r)),
                    TokenType::EqualEqual => Ok(LoxObject::Boolean(l == r)),
                    _ => Ok(LoxObject::Nil)
                }

            }
            Expr::Literal(expr) => {
                Ok(expr.to_owned())
            }
            Expr::Grouping(expr) => {
                expr.evaluate(interpreter)
            }
            Expr::Unary {operator, right } => {
                let r = right.evaluate(interpreter)?;

                match operator.token_type {
                    TokenType::Minus => {

                        match f64::try_from(r) {
                            Ok(n) => {
                                Ok(LoxObject::Number(-n))
                            }
                            Err(e) => {
                                Err(LoxError::RuntimeError(operator.clone(), e))
                            }
                        }

                    },
                    TokenType::Bang => {
                        let res = (!r).map_err(|e| LoxError::RuntimeError(operator.clone(), e))?;
                        Ok(res)
                    }
                    _ => Ok(LoxObject::Nil)
                }

            }
            Expr::Ternary { if_expr, then_branch, else_branch, operator } => {
                let cond = if_expr.evaluate(interpreter)?;

                let result = bool::try_from(cond)
                    .map_err(|e| LoxError::RuntimeError(operator.clone(), e))?;

                if result {
                    then_branch.evaluate(interpreter)
                }
                else {
                    else_branch.evaluate(interpreter)
                }

            }
            Expr::Variable(token) => {
                interpreter.get(token)
            }
            Expr::Assign { name, value } => {
                let value = value.evaluate(interpreter)?;
                interpreter.assign(name, value.clone())?;
                Ok(value)
            }
        }
    }
}
