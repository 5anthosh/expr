use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::rc::Rc;

use crate::lexer::token::Token;
use crate::value::Value;

#[derive(Debug)]
pub enum TullyError {
    RunTimeMessage(String),
    ParserErrorMessage(String),
    LexicalErrorMessage(String),
    Return(Rc<Value>),
}

impl Display for TullyError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TullyError::RunTimeMessage(value) => write!(f, "Runtime Error : {}", value),
            TullyError::ParserErrorMessage(value) => write!(f, "Parsing Error: {}", value),
            TullyError::LexicalErrorMessage(value) => write!(f, "Lexical Error: {}", value),
            TullyError::Return(value) => write!(f, "Return value {}", value.to_string()),
        }
    }
}

impl TullyError {
    pub fn lexical_error_message(line: usize, message: &str) -> TullyError {
        TullyError::LexicalErrorMessage(format!("{}  (line {})", message, line))
    }

    fn error(token: Option<&Token>, message: &str) -> String {
        if let Some(tt) = token {
            return format!("{} at {} (line {})", message, tt.lexeme, tt.line);
        }
        return format!("{} at end ", message);
    }

    pub fn parser_error_message(token: Option<&Token>, message: &str) -> TullyError {
        TullyError::ParserErrorMessage(TullyError::error(token, message))
    }

    pub fn runtime_error_message(token: &Token, message: &str) -> TullyError {
        TullyError::RunTimeMessage(format!(
            "{} at \"{}\" (line {})",
            message, token.lexeme, token.line
        ))
    }
}

impl Error for TullyError {}
