use crate::error::TullyError;
use crate::lexer::token::{Token, TokenType};
use crate::tully::Tully;

pub struct Lexer {
    expr_chars: Vec<char>,
    length: usize,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(expr: &str) -> Lexer {
        let a: Vec<char> = expr.chars().collect();
        return Lexer {
            length: a.len(),
            expr_chars: a,
            start: 0,
            current: 0,
            line: 1,
        };
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.length;
    }

    fn eat(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.current += 1;
        self.expr_chars[self.current - 1]
    }

    fn peek(&self, n: usize) -> char {
        if self.current + n >= self.length {
            return '\0';
        }
        self.expr_chars[self.current + n]
    }

    fn lexeme(&self) -> String {
        self.expr_chars[self.start..self.current]
            .iter()
            .cloned()
            .collect::<String>()
    }

    fn token_type(&mut self, tt: TokenType) -> Token {
        let token: Token = Token::new(tt, self.lexeme(), self.line, self.start, self.current);
        self.start = self.current;
        return token;
    }

    fn space(&mut self) -> char {
        let mut c = self.eat();
        while c.is_whitespace() {
            if c == '\n' {
                self.line += 1;
            }
            self.start = self.current;
            c = self.eat();
        }
        return c;
    }

    pub fn next_token(&mut self) -> Result<Token, TullyError> {
        if self.is_at_end() {
            return Ok(Token::end_of_line());
        }
        self.scan_token()
    }

    fn number(&mut self) -> Token {
        while self.peek(0).is_digit(10) {
            self.eat();
        }
        if self.peek(0) == '.' && self.peek(1).is_digit(10) {
            self.eat();
            while self.peek(0).is_digit(10) {
                self.eat();
            }
        }
        return self.token_type(TokenType::Number);
    }

    fn identifier(&mut self) -> Token {
        while self.peek(0).is_alphanumeric() {
            self.eat();
        }

        let lexeme = self.lexeme();
        match Tully::keywords_to_token_type(&lexeme[..]) {
            Some(tt) => self.token_type(tt),
            None => self.token_type(TokenType::Identifier),
        }
    }

    fn scan_string(&mut self) -> Result<Token, TullyError> {
        while self.peek(0) != '"' && !self.is_at_end() {
            if self.peek(0) == '\n' {
                self.line += 1;
            }
            self.eat();
        }
        if self.is_at_end() {
            return Err(self.error("Unterminated string"));
        }
        self.eat();
        Ok(self.token_type(TokenType::String))
    }

    fn scan_token(&mut self) -> Result<Token, TullyError> {
        let c = self.space();
        match c {
            '+' => Ok(self.token_type(TokenType::Plus)),
            '-' => Ok(self.token_type(TokenType::Minus)),
            '*' => Ok(self.token_type(TokenType::Star)),
            '/' => Ok(self.token_type(TokenType::Slash)),
            '%' => Ok(self.token_type(TokenType::Percentage)),
            '(' => Ok(self.token_type(TokenType::OpenParen)),
            ')' => Ok(self.token_type(TokenType::CloseParen)),
            ';' => Ok(self.token_type(TokenType::SemiColon)),
            '{' => Ok(self.token_type(TokenType::OpenBrace)),
            '}' => Ok(self.token_type(TokenType::CloseBrace)),
            ',' => Ok(self.token_type(TokenType::COMMA)),
            '=' => {
                if self.peek(0) == '=' {
                    self.eat();
                    return Ok(self.token_type(TokenType::EqualEqual));
                }
                Ok(self.token_type(TokenType::Equal))
            }
            '!' => {
                if self.peek(0) == '=' {
                    self.eat();
                    return Ok(self.token_type(TokenType::BangEqual));
                }
                Ok(self.token_type(TokenType::Bang))
            }
            '>' => {
                if self.peek(0) == '=' {
                    self.eat();
                    return Ok(self.token_type(TokenType::GreaterEqual));
                }
                Ok(self.token_type(TokenType::Greater))
            }
            '<' => {
                if self.peek(0) == '=' {
                    self.eat();
                    return Ok(self.token_type(TokenType::LesserEqual));
                }
                Ok(self.token_type(TokenType::Lesser))
            }
            '"' => self.scan_string(),
            _ => {
                if c.is_digit(10) {
                    return Ok(self.number());
                }
                if c.is_alphanumeric() {
                    return Ok(self.identifier());
                }
                return Err(self.error(&format!("Unexpected character {}", c)));
            }
        }
    }

    fn error(&self, message: &str) -> TullyError {
        TullyError::lexical_error_message(self.line, message)
    }
}

impl<'a> Iterator for Lexer {
    type Item = Result<Token, TullyError>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        let token = match token {
            Ok(token) => token,
            Err(e) => return Some(Err(e)),
        };
        match token.tt {
            TokenType::EOL => None,
            _ => Some(Ok(token)),
        }
    }
}
