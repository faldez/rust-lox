use std::fmt::Display;

use crate::token_type::TokenType;

#[derive(Clone)]
pub enum DataType {
    Boolean(bool),
    Number(f64),
    String(String),
    Nil,
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Boolean(val) => {
                write!(f, "{}", val)
            },
            DataType::Number(val) => {
                write!(f, "{}", val)
            },
            DataType::String(val) =>{
                write!(f, "{}", val)
            },
            DataType::Nil => {
                write!(f, "Nil")
            },
        }
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<DataType>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<DataType>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(literal) = &self.literal {
            write!(f, "{:?} {} {}", self.token_type, self.lexeme, literal)
        } else {
            write!(f, "{:?} {} None", self.token_type, self.lexeme)
        }
    }
}

impl Into<String> for Token {
    fn into(self) -> String {
        if let Some(literal) = self.literal {
            format!("{:?} {} {}", self.token_type, self.lexeme, literal)
        } else {
            format!("{:?} {} None", self.token_type, self.lexeme)
        }
    }
}
