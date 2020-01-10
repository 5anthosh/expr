use crate::value::Value;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::rc::Rc;

#[derive(Debug)]
pub enum ExprError {
    RunTimeMessage(String),
    ParserErrorMessage(String),
    LexicalErrorMessage(String),
    Return(Rc<Value>),
}

impl Display for ExprError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ExprError::RunTimeMessage(value) => write!(f, "Runtime Error : {}", value),
            ExprError::ParserErrorMessage(value) => write!(f, "Parsing Error: {}", value),
            ExprError::LexicalErrorMessage(value) => write!(f, "Lexical Error: {}", value),
            ExprError::Return(value) => write!(f, "Return value {}", value.to_string()),
        }
    }
}

impl Error for ExprError {}
