use std::fmt::{Display, Formatter};
use crate::errors::LoxError;
use crate::interpreter::{EnvironmentStack};
use crate::object::LoxObject;
use crate::stmt::Stmt;
use crate::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum LoxCallable {
    LoxFunction(LoxFunction),
    NativeFunction(NativeFunction),
}

#[derive(Clone, Debug)]
pub struct LoxFunction {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Box<Stmt>
}

impl PartialEq for LoxFunction {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub struct NativeFunction {
    pub name: &'static str,
    pub arity: usize,
    pub function: fn(&mut EnvironmentStack, Vec<LoxObject>) -> Result<LoxObject, LoxError>
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name) && self.arity.eq(&other.arity)
    }
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
        envs.push_scope();
        for (i, param) in self.params.iter().enumerate() {
            let arg = arguments.get(i).expect("Arity function not working correctly");
            envs.define(&param.lexeme, arg.clone());
        }


        let result = self.body.execute(envs);
        envs.pop_scope();
        match result? { 
            None => Ok(LoxObject::Nil),
            Some(val) => Ok(val)
        }

    }

    fn arity(&self) -> usize {
        self.params.len()
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
        match self {
            LoxCallable::LoxFunction(fun) => {
                write!(f, "{fun}")
            }
            LoxCallable::NativeFunction(fun) => {
                write!(f, "{fun}")
            }
        }
    }
}

impl Display for NativeFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn {}>", self.name)
    }
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name.lexeme)
    }
}
