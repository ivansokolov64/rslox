use std::fmt::{Display, Formatter};
use crate::errors::LoxError;
use crate::interpreter::{EnvironmentStack};
use crate::object::LoxObject;

#[derive(Clone, Debug, PartialEq)]
pub enum LoxCallable {
    LoxFunction(LoxFunction),
    NativeFunction(NativeFunction),
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoxFunction {

}

#[derive(Clone, Debug, PartialEq)]
pub struct NativeFunction {
    pub name: &'static str,
    pub arity: usize,
    pub function: fn(&mut EnvironmentStack, Vec<LoxObject>) -> Result<LoxObject, LoxError>
}

pub trait Call {
    fn call(&self, envs: &mut EnvironmentStack, arguments: Vec<LoxObject>) -> Result<LoxObject, LoxError>;
    fn arity(&self) -> usize;
}

impl Call for LoxCallable {
    fn call(&self, envs: &mut EnvironmentStack, arguments: Vec<LoxObject>) -> Result<LoxObject, LoxError> {
        match self {
            LoxCallable::LoxFunction(f) => {
                f.call(envs, arguments)
            }
            LoxCallable::NativeFunction(f) => {
                f.call(envs, arguments)
            }
        }
    }

    fn arity(&self) -> usize {
        match self {
            LoxCallable::LoxFunction(f) => {
                f.arity()
            }
            LoxCallable::NativeFunction(f) => {
                f.arity()
            }
        }
    }
}

impl Call for LoxFunction {
    fn call(&self, envs: &mut EnvironmentStack, arguments: Vec<LoxObject>) -> Result<LoxObject, LoxError> {
        todo!()
    }

    fn arity(&self) -> usize {
        todo!()
    }
}


impl Call for NativeFunction {
    fn call(&self, envs: &mut EnvironmentStack, arguments: Vec<LoxObject>) -> Result<LoxObject, LoxError> {
        (self.function)(envs, arguments)
    }

    fn arity(&self) -> usize {
        self.arity
    }
}


impl Display for LoxCallable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

