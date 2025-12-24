use crate::token::Token;

pub struct Scanner {
    source: String
}

impl Scanner { 
    pub fn new(source: String) -> Self {
        Scanner { source }
    }
    
    pub fn scan_tokens(&self) -> Vec<Token> {
        // Placeholder implementation
        vec![]
    }
}