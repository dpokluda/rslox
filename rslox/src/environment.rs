use crate::runtime_error::{LoxRuntime, RuntimeError};
use crate::token::{Token, TokenType};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Environment {
    values: std::collections::HashMap<String, crate::value::Value>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: std::collections::HashMap::new(),
            enclosing: None,
        }
    }

    pub fn from_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: std::collections::HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: crate::value::Value) {
        self.values.insert(name, value);
    }

    pub fn get_at(&self, distance: usize, name: &str) -> Result<crate::value::Value, LoxRuntime> {
        let environment = self.ancestor(distance);
        let env_borrow = environment.borrow();
        if let Some(value) = env_borrow.values.get(name) {
            Ok(value.clone())
        } else {
            Err(LoxRuntime::Error(RuntimeError::new(Token::new(TokenType::Identifier, name.to_string(), None, 0), format!("Undefined variable '{}'.", name))))
        }
    }
    
    pub fn assign_at(&mut self, distance: usize, name: &Token, value: crate::value::Value) -> Result<(), LoxRuntime> {
        let environment = self.ancestor(distance);
        let mut env_borrow = environment.borrow_mut();
        if env_borrow.values.contains_key(name.lexeme()) {
            env_borrow.values.insert(name.lexeme().to_string(), value);
            Ok(())
        } else {
            Err(LoxRuntime::Error(RuntimeError::new(name.clone(), format!("Undefined variable '{}'.", name.lexeme()))))
        }
    }

    fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
        let mut environment = Rc::new(RefCell::new(self.clone()));
        for _ in 0..distance {
            let enc = environment.borrow().enclosing.as_ref().unwrap().clone();
            environment = enc;
        }
        environment
    }

    pub fn get(&self, name: &Token) -> Result<crate::value::Value, LoxRuntime> {
        if let Some(value) = self.values.get(name.lexeme()) {
            Ok(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(name)
        } else {
            Err(LoxRuntime::Error(RuntimeError::new(name.clone(), format!("Undefined variable '{}'.", name.lexeme()))))
        }
    }

    pub fn assign(&mut self, name: &Token, value: crate::value::Value) -> Result<(), LoxRuntime> {
        if self.values.contains_key(name.lexeme()) {
            self.values.insert(name.lexeme().to_string(), value);
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(LoxRuntime::Error(RuntimeError::new(name.clone(), format!("Undefined variable '{}'.", name.lexeme()))))
        }
    }
}
