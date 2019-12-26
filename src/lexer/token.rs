pub enum TokenType {
    PLUS,
    STAR,
    MINUS,
    SLASH,
    PS,
}

pub struct Token {
    tt: TokenType,
    lexeme: String,

}

