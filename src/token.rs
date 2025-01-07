use crate::token_type::TokenType;

pub struct Token {
    pub token: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}

impl Token {
    pub fn new(token: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Token{
            token,
            lexeme,
            literal,
            line,
        }
    }
}