use crate::token::Token;

#[derive(Debug, Clone)]
pub struct RuntimeError {
    token: Token,
    message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: String) -> Self {
        RuntimeError { token, message }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}