use std::fmt::{Display, Formatter};
use std::ops::Not;
use crate::errors::RuntimeError;

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
