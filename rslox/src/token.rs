use std::hash::Hash;
use crate::literal::LiteralValue;
pub(crate) use crate::token_type::TokenType;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralValue>,
    line: u32,
} 

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<LiteralValue>, line: u32) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
    
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }
    
    pub fn lexeme(&self) -> &String {
        &self.lexeme
    }
    
    pub fn literal(&self) -> &Option<LiteralValue> {
        &self.literal
    }
    
    pub fn line(&self) -> u32 {
        self.line
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

impl Eq for LiteralValue {}

impl Hash for LiteralValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            LiteralValue::Number(n) => {
                state.write_u8(0);
                state.write(&n.to_ne_bytes());
            }
            LiteralValue::String(s) => {
                state.write_u8(1);
                s.hash(state);
            }
            LiteralValue::Boolean(b) => {
                state.write_u8(2);
                b.hash(state);
            }
            LiteralValue::Nil => {
                state.write_u8(3);
            }
        }
    }
}