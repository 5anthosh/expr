use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum ExprError {
    RunTimeMessage(String),
    ParserErrorMessage(String),
    LexicalErrorMessage(String),
}

impl Display for ExprError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ExprError::RunTimeMessage(value) => write!(f, "Runtime Error : {}", value),
            ExprError::ParserErrorMessage(value) => write!(f, "Parsing Error: {}", value),
            ExprError::LexicalErrorMessage(value) => write!(f, "Lexical Error: {}", value),
        }
    }
}

impl Error for ExprError {}
