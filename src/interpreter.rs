use std::fmt::{Display, Formatter};
use std::ops::{Not};
use crate::errors::{LoxError, RuntimeError};
use crate::expr::Expr;
use crate::token::{Token, TokenType};


#[derive(Clone, Debug)]
pub enum LoxType {
    Number,
    String,
    Boolean,
    Nil
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoxObject {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil
}

impl Not for LoxObject {
    type Output = Result<Option<Self>, RuntimeError>;

    fn not(self) -> Self::Output {
        let b = bool::try_from(self)?;
        Ok(Some(LoxObject::Boolean(!b)))
    }
}


impl Display for LoxObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxObject::Number(n) => {
                write!(f, "{n}")
            }
            LoxObject::String(s) => {
                write!(f, "{s}")
            }
            LoxObject::Boolean(b) => {
                write!(f, "{b}")
            },
            LoxObject::Nil => {
                write!(f, "nil")
            }
        }
    }
}

impl TryFrom<LoxObject> for f64 {

    type Error = RuntimeError;
    fn try_from(value: LoxObject) -> Result<Self, Self::Error> {
        match value {
            LoxObject::Number(val) => Ok(val),
            t => Err(RuntimeError::InvalidOperand { expected: LoxType::Number, received: t })
        }
    }
}

impl TryFrom<LoxObject> for String {
    type Error = RuntimeError;

    fn try_from(value: LoxObject) -> Result<Self, Self::Error> {
        match value {
            LoxObject::String(string) => Ok(string),
            t => Err(RuntimeError::InvalidOperand { expected: LoxType::String, received: t })
        }
    }
}

impl TryFrom<LoxObject> for bool {
    type Error = RuntimeError;

    fn try_from(value: LoxObject) -> Result<Self, Self::Error> {
        match value {
            LoxObject::Boolean(bool) => Ok(bool),
            LoxObject::Nil => Ok(false),
            _ => Ok(true)
        }
    }
}


pub struct Interpreter {

}

impl Interpreter {

    pub fn new() -> Self {
        Self { }
    }
    pub fn interpret(&self, expr: Expr) -> Result<Option<LoxObject>, LoxError> {
        expr.evaluate()
    }
}


pub trait Evaluate<T> {
    fn evaluate(&self) -> Result<T, LoxError>;
}

impl Evaluate<Option<LoxObject>> for Expr {
    fn evaluate(&self) -> Result<Option<LoxObject>, LoxError> {
        match self {
            Expr::Binary {left, operator, right} => {

                let (Some(l), Some(r)) = (left.evaluate()?, right.evaluate()?) else {
                    return Err(LoxError::RuntimeError(operator.clone(), RuntimeError::EvaluationError(*left.clone())));
                };

                match operator.token_type {
                    TokenType::Comma => {
                        Ok(Some(r))
                    },
                    TokenType::Greater => {
                        let (a, b) = numeric_operands(operator, l, r)?;
                        Ok(Some(LoxObject::Boolean(a > b)))

                    },
                    TokenType::GreaterEqual => {
                        let (a, b) = numeric_operands(operator, l, r)?;
                        Ok(Some(LoxObject::Boolean(a >= b)))
                    },
                    TokenType::Less => {
                        let (a, b) = numeric_operands(operator, l, r)?;
                        Ok(Some(LoxObject::Boolean(a < b)))
                    },
                    TokenType::LessEqual => {
                        let (a, b) = numeric_operands(operator, l, r)?;
                        Ok(Some(LoxObject::Boolean(a <= b)))
                    },
                    TokenType::Minus => {
                        let (a, b) = numeric_operands(operator, l, r)?;
                        Ok(Some(LoxObject::Number(a - b)))
                    },
                    TokenType::Slash => {
                        let (a, b) = numeric_operands(operator, l, r)?;
                        Ok(Some(LoxObject::Number(a / b)))
                    },
                    TokenType::Star => {
                        let (a, b) = numeric_operands(operator, l, r)?;
                        Ok(Some(LoxObject::Number(a * b)))
                    },
                    TokenType::Plus => {
                        if let LoxObject::String(str_l) = &l
                            && let LoxObject::String(str_r) = &r {
                            Ok(Some(LoxObject::String(format!("{}{}", str_l, str_r))))
                        }
                        else {
                            let (a, b) = numeric_operands(operator, l, r)?;
                            Ok(Some(LoxObject::Number(a + b)))
                        }
                    },
                    TokenType::BangEqual => Ok(Some(LoxObject::Boolean(l != r))),
                    TokenType::EqualEqual => Ok(Some(LoxObject::Boolean(l == r))),
                    _ => Ok(None)
                }

            }
            Expr::Literal(expr) => {
                Ok(expr.to_owned())
            }
            Expr::Grouping(expr) => {
                expr.evaluate()
            }
            Expr::Unary {operator, right } => {
                let Some(r) = right.evaluate()? else {
                    return Ok(None)
                };

                match operator.token_type {
                    TokenType::Minus => {

                        match f64::try_from(r) {
                            Ok(n) => {
                                Ok(Some(LoxObject::Number(-n)))
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
                    _ => Ok(None)
                }

            }
            Expr::Ternary { if_expr, then_branch, else_branch, operator } => {
                let Some(cond) = if_expr.evaluate()? else {
                    return Ok(None)
                };

                let result = bool::try_from(cond)
                    .map_err(|e| LoxError::RuntimeError(operator.clone(), e))?;

                if result {
                    then_branch.evaluate()
                }
                else {
                    else_branch.evaluate()
                }



            }
        }
    }
}

fn numeric_operands(
    operator: &Token,
    l: LoxObject,
    r: LoxObject,
) -> Result<(f64, f64), LoxError> {
    let a = f64::try_from(l).map_err(|e| LoxError::RuntimeError(operator.clone(), e))?;
    let b = f64::try_from(r).map_err(|e| LoxError::RuntimeError(operator.clone(), e))?;
    Ok((a, b))
}