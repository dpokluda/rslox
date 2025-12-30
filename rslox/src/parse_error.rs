use crate::token::Token;

#[derive(Debug, Clone)]
pub struct ParseError {
    token: Token,
    message: String,
}

impl ParseError {
    pub fn new(token: Token, message: String) -> Self {
        ParseError { 
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

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Error at '{}': {}", self.token.line(), self.token.lexeme(), self.message)
    }
}