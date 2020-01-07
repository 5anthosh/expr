use crate::lexer::token::TokenType;

pub struct Tully;

impl Tully {
    pub fn keywords_to_token_type(keyword: &str) -> Option<TokenType> {
        match keyword {
            "true" => Some(TokenType::True),
            "false" => Some(TokenType::False),
            "nil" => Some(TokenType::Nil),
            "print" => Some(TokenType::Print),
            _ => None,
        }
    }
}
