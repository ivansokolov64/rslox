use std::collections::HashMap;
use crate::errors::{LoxError, RuntimeError};
use crate::loxobject::LoxObject;
use crate::stmt::Stmt;
use crate::token::Token;

pub struct Interpreter {
    environment: Environment
}

pub struct Environment {
    values: HashMap<String, LoxObject>
}

impl Interpreter {

    pub fn new() -> Self {
        Self {
            environment: Environment::new()
        }
    }
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for statement in statements {
            statement.execute(&mut self.environment)?;
        }
        Ok(())
    }
}

impl Environment {

    fn new() -> Self {
        Self {
            values: HashMap::new()
        }
    }
    pub fn define(&mut self, name: &String, value: LoxObject) -> Result<(), LoxError> {
        self.values.insert(name.to_string(), value);
        Ok(())
    }

    pub fn get(&mut self, name: &Token) -> Result<LoxObject, LoxError> {
        match self.values.get(&name.lexeme) {
            None => {
                Err(LoxError::RuntimeError(name.clone(),
                                           RuntimeError::UndefinedVariable(name.lexeme.to_string())))
            }
            Some(obj) => {
                Ok(obj.to_owned())
            }
        }
    }

    pub fn assign(&mut self, name: &Token, value: LoxObject) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.to_string(), value);
            Ok(())
        }
        else {
            Err(LoxError::RuntimeError(name.clone(), RuntimeError::UndefinedVariable(name.lexeme.to_string())))
        }
    }
}