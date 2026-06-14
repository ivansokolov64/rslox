use std::collections::HashMap;
use crate::errors::{LoxError, RuntimeError};
use crate::loxobject::LoxObject;
use crate::stmt::Stmt;
use crate::token::Token;

pub struct Interpreter {
    environments: Vec<Environment>
}

pub struct Environment {
    values: HashMap<String, LoxObject>,
}



impl Interpreter {

    pub fn new() -> Self {
        Self {
            environments: vec![Environment::new()]
        }
    }
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for statement in statements {
            statement.execute(self)?;
        }
        Ok(())
    }

    pub fn define(&mut self, name: &String, value: LoxObject) {

        self.environments.last_mut()
            .expect("environment stack should never be empty")
            .define(name.to_string(), value)
    }

    pub fn get(&mut self, name: &Token) -> Result<LoxObject, LoxError> {

        for env in self.environments.iter().rev() {
            if let Some(val) = env.values.get(&name.lexeme) {
                return Ok(val.to_owned())
            }
        }

        Err(LoxError::RuntimeError(name.clone(),
                                   RuntimeError::UndefinedVariable(name.lexeme.to_string())))

    }

    pub fn assign(&mut self, name: &Token, value: LoxObject) -> Result<(), LoxError> {

        for env in self.environments.iter_mut().rev() {
            if env.values.contains_key(&name.lexeme) {
                env.values.insert(name.lexeme.to_string(), value);
                return Ok(())
            }
        }

        Err(LoxError::RuntimeError(name.clone(), RuntimeError::UndefinedVariable(name.lexeme.to_string())))

    }

    pub fn push_scope(&mut self) {
        self.environments.push(Environment::new())
    }

    pub fn pop_scope(&mut self) {
        self.environments.pop();
    }


}

impl Environment {

    pub fn new() -> Self {
        Self {
            values: HashMap::new()
        }
    }


    fn define(&mut self, name: String, value: LoxObject) {
        self.values.insert(name, value);
    }

}