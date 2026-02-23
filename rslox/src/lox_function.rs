use std::cell::RefCell;
use std::rc::Rc;
use lox_instance::LoxInstance;
use crate::environment::Environment;
use crate::lox_instance;
use crate::runtime_error::LoxRuntime;
use crate::stmt::Function;
use crate::value::Value;

pub struct LoxFunction {
    declaration: Box<Function>,
    closure: Rc<RefCell<Environment>>,
    is_initializer: bool,
}

impl LoxFunction {
    pub fn new(declaration: Box<Function>, closure: Rc<RefCell<Environment>>, is_initializer: bool ) -> Self {
        LoxFunction { declaration, closure, is_initializer }
    }

    pub fn bind(&self, instance: Rc<RefCell<LoxInstance>>) -> LoxFunction {
        let mut environment = Environment::from_enclosing(self.closure.clone());
        environment.define("this".to_string(), Value::LoxInstance(instance));
        LoxFunction {
            declaration: self.declaration.clone(),
            closure: Rc::new(RefCell::new(environment)),
            is_initializer: self.is_initializer,
        }
    }
}

impl crate::lox_callable::LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.declaration.params().len()
    }

    fn call(&self, interpreter: &mut crate::interpreter::Interpreter, arguments: Vec<Value>) -> Result<Value, LoxRuntime> {
        let environment = Rc::new(RefCell::new(Environment::from_enclosing(self.closure.clone())));

        for (i, param) in self.declaration.params().iter().enumerate() {
            environment.borrow_mut().define(param.lexeme().clone(), arguments[i].clone());
        }
        
        match interpreter.execute_block(&self.declaration.body(), environment) {
            Ok(_) => {
                if self.is_initializer {
                    Ok(self.closure.borrow().get_at(0, "this").unwrap())
                } else {
                    Ok(Value::Nil)
                }
            },
            Err(LoxRuntime::Return(return_value)) => {
                if self.is_initializer {
                    Ok(self.closure.borrow().get_at(0, "this").unwrap())
                } else {
                    Ok(return_value.value().clone())
                }
            },
            Err(err) => Err(err),
        }
    }
}

impl std::fmt::Debug for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.declaration.name().lexeme())
    }
}

impl std::fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.declaration.name().lexeme())
    }
}

impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        self.declaration.name() == other.declaration.name()
    }
}

impl Clone for LoxFunction {
    fn clone(&self) -> Self {
        LoxFunction {
            declaration: self.declaration.clone(),
            closure: self.closure.clone(),
            is_initializer: self.is_initializer,
        }
    }
}