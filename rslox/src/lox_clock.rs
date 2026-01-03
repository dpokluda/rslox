use std::fmt::{Debug, Display, Formatter};
use crate::lox_callable::LoxCallable;
use crate::runtime_error::LoxRuntime;
use crate::value::Value;

pub struct LoxClock;

impl LoxClock {
    pub fn new() -> Self {
        LoxClock {}
    }
}

impl Display for LoxClock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn clock>")
    }
}

impl Debug for LoxClock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn clock>")
    }
}

impl LoxCallable for LoxClock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &mut crate::interpreter::Interpreter, _arguments: Vec<Value>) -> Result<Value, LoxRuntime> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");

        Ok(crate::value::Value::Number(now.as_secs_f64()))
    }
}