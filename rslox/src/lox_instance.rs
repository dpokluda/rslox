use std::collections::HashMap;
use std::rc::Rc;
use crate::lox_class::LoxClass;
use crate::runtime_error::{LoxRuntime, RuntimeError};
use crate::token::Token;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct LoxInstance {
    class_: Rc<LoxClass>,
    fields: HashMap<String, Value>,
}

impl LoxInstance {
    pub fn new(class_: Rc<LoxClass>) -> Self {
        LoxInstance {
            class_,
            fields: HashMap::new(),
        }
    }

    pub fn class_(&self) -> &Rc<LoxClass> {
        &self.class_
    }

    pub fn get(&self, name: &Token) -> Result<Value, LoxRuntime> {
        if let Some(value) = self.fields.get(name.lexeme()) {
            Ok(value.clone())
        } else {
            Err(LoxRuntime::Error(RuntimeError::new(name.clone(), format!("Undefined property '{}'.", name.lexeme()))))
        }
    }
    
    pub fn set(&mut self, name: &Token, value: Value) {
        self.fields.insert(name.lexeme().to_string(), value);
    }
}

impl std::fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<instance of {}>", self.class_.name())
    }
}