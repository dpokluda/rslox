use std::cell::RefCell;
use std::rc::Rc;
use crate::lox_callable::LoxCallable;
use crate::lox_class::LoxClass;
use crate::lox_instance::LoxInstance;

#[derive(Clone)]
pub enum
Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
    LoxCallable(Rc<dyn LoxCallable>),
    LoxClass(Rc<LoxClass>),
    LoxInstance(Rc<RefCell<LoxInstance>>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "nil"),
            Value::LoxCallable(callable) => write!(f, "{}", callable),
            Value::LoxClass(class) => write!(f, "{}", class),
            Value::LoxInstance(instance) => write!(f, "{}", instance.borrow()),
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "Number({})", n),
            Value::Boolean(b) => write!(f, "Boolean({})", b),
            Value::String(s) => write!(f, "String({})", s),
            Value::Nil => write!(f, "Nil"),
            Value::LoxCallable(callable) => write!(f, "LoxCallable({})", callable),
            Value::LoxClass(class) => write!(f, "LoxClass({})", class),
            Value::LoxInstance(instance) => write!(f, "LoxInstance({})", instance.borrow()),
        }
    }
}