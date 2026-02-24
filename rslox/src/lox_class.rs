use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::interpreter::Interpreter;
use crate::lox_callable::LoxCallable;
use crate::lox_function::LoxFunction;
use crate::lox_instance::LoxInstance;
use crate::runtime_error::LoxRuntime;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct LoxClass {
    name: String,
    superclass: Option<Rc<LoxClass>>,
    methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(name: String, superclass: Option<Rc<LoxClass>>, methods: HashMap<String, LoxFunction>) -> Self {
        LoxClass {
            name,
            superclass,
            methods,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn find_method(&self, name: &str) -> Option<LoxFunction> {
        if let Some(method) = self.methods.get(name) {
            Some(method.clone())
        } else if let Some(superclass) = &self.superclass {
            superclass.find_method(name)
        } else {
            None
        }
    }
}

impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        let initializer = self.find_method("init");
        if let Some(init) = initializer {
            init.arity()
        } else {
            0
        }
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, LoxRuntime> {
        let instance = LoxInstance::new(Rc::new(self.clone()));
        let initializer = self.find_method("init");
        if let Some(init) = initializer {
            init.bind(Rc::new(RefCell::new(instance.clone()))).call(interpreter, arguments)?;
        }

        Ok(Value::LoxInstance(Rc::new(RefCell::new(instance))))
    }
}

impl std::fmt::Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<class {} instance>", self.name)
    }
}