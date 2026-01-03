use std::rc::Rc;
use crate::runtime_error::LoxRuntime;
use crate::value::Value;

pub struct LoxFunction {
    declaration: Box<crate::stmt::Function>,
}

impl LoxFunction {
    pub fn new(declaration: Box<crate::stmt::Function>) -> Self {
        LoxFunction { declaration }
    }
}

impl crate::lox_callable::LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.declaration.params().len()
    }

    fn call(&self, interpreter: &mut crate::interpreter::Interpreter, arguments: Vec<Value>) -> Result<Value, LoxRuntime> {
        let environment = Rc::new(std::cell::RefCell::new(crate::environment::Environment::from_enclosing(
            interpreter.globals())));


        for (i, param) in self.declaration.params().iter().enumerate() {
            environment.borrow_mut().define(param.lexeme().clone(), arguments[i].clone());
        }

        match interpreter.execute_block(&self.declaration.body(), environment) {
            Ok(_) => Ok(Value::Nil),
            Err(LoxRuntime::Return(return_value)) => Ok(return_value.value().clone()),
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