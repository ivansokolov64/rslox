use crate::errors::RuntimeError;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Not, Sub};
use crate::callables::LoxCallable;

#[derive(Clone, Debug)]
pub enum LoxType {
    Number,
    String,
    Boolean,
    Nil,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoxObject {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Callable(LoxCallable)
}

impl Not for LoxObject {
    type Output = Result<Self, RuntimeError>;

    fn not(self) -> Self::Output {
        let b = bool::from(self);
        Ok(LoxObject::Boolean(!b))
    }
}

impl Add for LoxObject {
    type Output = Result<Self, RuntimeError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (LoxObject::Number(l), LoxObject::Number(r)) => Ok(LoxObject::Number(l + r)),
            (LoxObject::String(l), LoxObject::String(r)) => {
                Ok(LoxObject::String(format!("{}{}", l, r)))
            }
            (LoxObject::String(_), r) => Err(RuntimeError::InvalidOperand {
                expected: LoxType::String,
                received: r,
            }),
            (_, r) => Err(RuntimeError::InvalidOperand {
                expected: LoxType::Number,
                received: r,
            }),
        }
    }
}

impl Sub for LoxObject {
    type Output = Result<Self, RuntimeError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (LoxObject::Number(l), LoxObject::Number(r)) => Ok(LoxObject::Number(l - r)),
            (LoxObject::Number(_), r) => Err(RuntimeError::InvalidOperand {
                expected: LoxType::Number,
                received: r,
            }),
            (l, _) => Err(RuntimeError::InvalidOperand {
                expected: LoxType::Number,
                received: l,
            }),
        }
    }
}

impl Mul for LoxObject {
    type Output = Result<Self, RuntimeError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (LoxObject::Number(l), LoxObject::Number(r)) => Ok(LoxObject::Number(l * r)),
            (LoxObject::Number(_), r) => Err(RuntimeError::InvalidOperand {
                expected: LoxType::Number,
                received: r,
            }),
            (l, _) => Err(RuntimeError::InvalidOperand {
                expected: LoxType::Number,
                received: l,
            }),
        }
    }
}

impl Div for LoxObject {
    type Output = Result<Self, RuntimeError>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (LoxObject::Number(l), LoxObject::Number(r)) => {
                if r == 0f64 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(LoxObject::Number(l / r))
                }
            }
            (LoxObject::Number(_), r) => Err(RuntimeError::InvalidOperand {
                expected: LoxType::Number,
                received: r,
            }),
            (l, _) => Err(RuntimeError::InvalidOperand {
                expected: LoxType::Number,
                received: l,
            }),
        }
    }
}

impl PartialOrd for LoxObject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (LoxObject::Number(l), LoxObject::Number(r)) => l.partial_cmp(r),
            (_, _) => None,
        }
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
            }
            LoxObject::Nil => {
                write!(f, "nil")
            }
            LoxObject::Callable(c) => {
                write!(f, "{c}")
            }
        }
    }
}

impl TryFrom<LoxObject> for f64 {
    type Error = RuntimeError;
    fn try_from(value: LoxObject) -> Result<Self, Self::Error> {
        match value {
            LoxObject::Number(val) => Ok(val),
            t => Err(RuntimeError::InvalidOperand {
                expected: LoxType::Number,
                received: t,
            }),
        }
    }
}

impl TryFrom<LoxObject> for String {
    type Error = RuntimeError;

    fn try_from(value: LoxObject) -> Result<Self, Self::Error> {
        match value {
            LoxObject::String(string) => Ok(string),
            t => Err(RuntimeError::InvalidOperand {
                expected: LoxType::String,
                received: t,
            }),
        }
    }
}

impl From<LoxObject> for bool {

    fn from(value: LoxObject) -> Self {
        match value {
            LoxObject::Boolean(bool) => bool,
            LoxObject::Nil => false,
            _ => true
        }
    }
}
