use std::cell::RefCell;
use std::rc::Rc;
use crate::interpreter::Interpreter;
use crate::lox_callable::LoxCallable;
use crate::lox_instance::LoxInstance;
use crate::runtime_error::LoxRuntime;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: String,
}

impl LoxClass {
    pub fn new(name: String) -> Self {
        LoxClass {
            name,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, LoxRuntime> {
        let instance = LoxInstance::new(Rc::new(self.clone()));
        Ok(Value::LoxInstance(Rc::new(RefCell::new(instance))))
    }
}

impl std::fmt::Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<class {} instance>", self.name)
    }
}