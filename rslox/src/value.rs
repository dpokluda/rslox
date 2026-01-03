use std::rc::Rc;
use crate::lox_callable::LoxCallable;

#[derive(Clone)]
pub enum
Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
    LoxCallable(Rc<dyn LoxCallable>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "nil"),
            Value::LoxCallable(callable) => write!(f, "{}", callable),
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
        }
    }
}