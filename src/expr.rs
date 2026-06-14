use std::fmt;
use std::fmt::Formatter;
use crate::errors::{LoxError, RuntimeError};
use crate::interpreter::Interpreter;
use crate::token::{Token, TokenType};
use crate::loxobject::LoxObject;

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

                match operator.token_type {
                    TokenType::Comma => {
                        Ok(r)
                    },
                    TokenType::Greater => {
                        let (a, b) = numeric_operands(operator, l, r)?;
                        Ok(LoxObject::Boolean(a > b))

                    },
                    TokenType::GreaterEqual => {
                        let (a, b) = numeric_operands(operator, l, r)?;
                        Ok(LoxObject::Boolean(a >= b))
                    },
                    TokenType::Less => {
                        let (a, b) = numeric_operands(operator, l, r)?;
                        Ok(LoxObject::Boolean(a < b))
                    },
                    TokenType::LessEqual => {
                        let (a, b) = numeric_operands(operator, l, r)?;
                        Ok(LoxObject::Boolean(a <= b))
                    },
                    TokenType::Minus => {
                        let (a, b) = numeric_operands(operator, l, r)?;
                        Ok(LoxObject::Number(a - b))
                    },
                    TokenType::Slash => {
                        let (a, b) = numeric_operands(operator, l, r)?;

                        // Add division by zero error
                        match b {
                            0f64 => Err(LoxError::RuntimeError(operator.clone(), RuntimeError::DivisionByZero)),
                            _ => Ok(LoxObject::Number(a / b))
                        }

                    },
                    TokenType::Star => {
                        let (a, b) = numeric_operands(operator, l, r)?;
                        Ok(LoxObject::Number(a * b))
                    },
                    TokenType::Plus => {
                        if let LoxObject::String(str_l) = &l
                            && let LoxObject::String(str_r) = &r {
                            Ok(LoxObject::String(format!("{}{}", str_l, str_r)))
                        }
                        else {
                            let (a, b) = numeric_operands(operator, l, r)?;
                            Ok(LoxObject::Number(a + b))
                        }
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

fn numeric_operands(
    operator: &Token,
    l: LoxObject,
    r: LoxObject,
) -> Result<(f64, f64), LoxError> {
    let a = f64::try_from(l).map_err(|e| LoxError::RuntimeError(operator.clone(), e))?;
    let b = f64::try_from(r).map_err(|e| LoxError::RuntimeError(operator.clone(), e))?;
    Ok((a, b))
}