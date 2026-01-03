use crate::runtime_error::LoxRuntime;
use crate::value::Value;

pub trait LoxCallable : std::fmt::Display + std::fmt::Debug {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut crate::interpreter::Interpreter, arguments: Vec<Value>) -> Result<Value, LoxRuntime>;
}