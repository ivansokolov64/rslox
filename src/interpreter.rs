use crate::errors::{LoxError, RuntimeError};
use crate::object::LoxObject;
use crate::stmt::Stmt;
use crate::token::Token;
use std::collections::HashMap;
use crate::callables::{LoxCallable, NativeFunction};
use crate::natives::clock_native;

pub struct Interpreter {
    envs: EnvironmentStack,
}

pub struct Environment {
    values: HashMap<String, LoxObject>,
}

pub struct EnvironmentStack {
    envs: Vec<Environment>,
}

impl EnvironmentStack {
    pub fn new() -> Self {
        let mut global = Environment::new();

        let clock = LoxObject::Callable(Box::new(LoxCallable::NativeFunction(NativeFunction {
            name: "clock",
            arity: 0,
            function: clock_native,
        })));

        global.define("clock".to_string(), clock);

        Self {
            envs: vec![global],
        }
    }

    pub fn push_scope(&mut self) {
        self.envs.push(Environment::new())
    }

    pub fn pop_scope(&mut self) {
        self.envs.pop();
    }

    pub fn define(&mut self, name: &String, value: LoxObject) {
        self.envs
            .last_mut()
            .expect("Environment stack should never be empty")
            .define(name.to_string(), value)
    }

    pub fn get(&mut self, name: &Token) -> Result<LoxObject, LoxError> {
        for env in self.envs.iter().rev() {
            if let Some(val) = env.values.get(&name.lexeme) {
                return Ok(val.to_owned());
            }
        }

        Err(LoxError::RuntimeError(
            name.clone(),
            RuntimeError::UndefinedVariable(name.lexeme.to_string()),
        ))
    }

    pub fn assign(&mut self, name: &Token, value: LoxObject) -> Result<(), LoxError> {
        for env in self.envs.iter_mut().rev() {
            if env.values.contains_key(&name.lexeme) {
                env.values.insert(name.lexeme.to_string(), value);
                return Ok(());
            }
        }

        Err(LoxError::RuntimeError(
            name.clone(),
            RuntimeError::UndefinedVariable(name.lexeme.to_string()),
        ))
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            envs: EnvironmentStack::new(),
        }
    }
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for statement in statements {
            statement.execute(&mut self.envs)?;
        }
        Ok(())
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    fn define(&mut self, name: String, value: LoxObject) {
        self.values.insert(name, value);
    }
}
