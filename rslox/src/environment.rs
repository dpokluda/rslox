use crate::runtime_error::RuntimeError;
use crate::token::Token;
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

    pub fn get(&self, name: &Token) -> Result<crate::value::Value, RuntimeError> {
        if let Some(value) = self.values.get(name.lexeme()) {
            Ok(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(name)
        } else {
            Err(RuntimeError::new(name.clone(), format!("Undefined variable '{}'.", name.lexeme())))
        }
    }

    pub fn assign(&mut self, name: &Token, value: crate::value::Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(name.lexeme()) {
            self.values.insert(name.lexeme().to_string(), value);
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(RuntimeError::new(name.clone(), format!("Undefined variable '{}'.", name.lexeme())))
        }
    }
}
