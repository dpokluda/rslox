use crate::token::Token;

pub enum LoxRuntime {
    Error(RuntimeError),
    Return(RuntimeReturn)
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    token: Token,
    message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: String) -> Self {
        RuntimeError {
            token,
            message,
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Runtime Error at '{}': {}", self.token.line(), self.token.lexeme(), self.message)
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeReturn {
    value: crate::value::Value,
}

impl RuntimeReturn {
    pub fn new(value: crate::value::Value) -> Self {
        RuntimeReturn {
            value,
        }
    }

    pub fn value(&self) -> &crate::value::Value {
        &self.value
    }
}

impl std::fmt::Display for RuntimeReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Return with value: {}", self.value)
    }
}
