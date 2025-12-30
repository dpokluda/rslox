use crate::runtime_error::RuntimeError;
use crate::token::Token;

#[derive(Clone)]
pub struct Environment {
    values: std::collections::HashMap<String, crate::value::Value>, 
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: std::collections::HashMap::new(),
            enclosing: None,
        }
    }
    
    pub fn from_enclosing(enclosing: Environment) -> Self {
        Environment {
            values: std::collections::HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn define(&mut self, name: String, value: crate::value::Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<&crate::value::Value, RuntimeError> {
        if let Some(value) = self.values.get(name.lexeme()) {
            Ok(value)
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.get(name)
        } else {
            Err(RuntimeError::new(name.clone(), format!("Undefined variable '{}'.", name.lexeme())))
        }
    }
    
    pub fn assign(&mut self, name: &Token, value: crate::value::Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(name.lexeme()) {
            self.values.insert(name.lexeme().to_string(), value);
            Ok(())
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value)
        } else {
            Err(RuntimeError::new(name.clone(), format!("Undefined variable '{}'.", name.lexeme())))
        }
    }
}