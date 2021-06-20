use std::{iter::Peekable, str::Chars};

use phf::phf_map;

use crate::token::{DataType, Token, TokenType};

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

pub struct Scanner<'a> {
    iter: Peekable<Chars<'a>>,
    ch: char,
    line: usize,
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a> Scanner<'a> {
    pub fn new(source: String) -> Self {
        let iter = source.chars().peekable();
        let ch = if let Some(ch) = iter.next() { ch } else { '\0' };

        Self { iter, ch, line: 1 }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.next();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        self.tokens.clone()
    }

    fn take_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut s = String::new();
        if self.ch != '\0' {
            s.push(self.ch);
        }
        while let Some(c) = self.iter.next() {
            if !predicate(c) {
                self.ch = c;
                return s;
            }
            s.push(c);
        }
        self.ch = '\n';
        s
    }

    fn next(&mut self) -> Token {
        match self.ch {
            '(' => Token::new(TokenType::LeftParen, self.ch.to_string()),
            ')' => Token::new(TokenType::RightParen, self.ch.to_string()),
            '{' => Token::new(TokenType::LeftBrace, self.ch.to_string()),
            '}' => Token::new(TokenType::RightBrace, self.ch.to_string()),
            ',' => Token::new(TokenType::Comma, self.ch.to_string()),
            '.' => Token::new(TokenType::Dot, self.ch.to_string()),
            '-' => Token::new(TokenType::Minus, self.ch.to_string()),
            '+' => Token::new(TokenType::Plus, self.ch.to_string()),
            ';' => Token::new(TokenType::Semicolon, self.ch.to_string()),
            '*' => Token::new(TokenType::Star, self.ch.to_string()),
            '!' => {
                if self.match_char('=') {
                    Token::new(TokenType::BangEqual, "!=".to_string())
                } else {
                    Token::new(TokenType::Bang, "!".to_string())
                }
            }
            '=' => {
                if self.match_char('=') {
                    Token::new(TokenType::EqualEqual, "==".to_string())
                } else {
                    Token::new(TokenType::Equal, "=".to_string())
                }
            }
            '<' => {
                if self.match_char('=') {
                    Token::new(TokenType::LessEqual, "<=".to_string())
                } else {
                    Token::new(TokenType::Less, "<".to_string())
                }
            }
            '>' => {
                if self.match_char('=') {
                    Token::new(TokenType::GreaterEqual, ">=".to_string())
                } else {
                    Token::new(TokenType::Greater, ">".to_string())
                }
            }
            '/' => {
                if self.match_char('/') {
                    let s = self.take_while(|c| c != '\n' || c != '\0');
                    Token::new_with_literal(TokenType::Comment, "//".to_string(), s)
                } else {
                    Token::new(TokenType::Slash, "/".to_string())
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    // TODO: Log error
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numberic(self.peek()) {
            self.advance();
        }

        let value = &self.source[self.start..self.current];
        let token_type = if let Some(keyword) = KEYWORDS.get(value) {
            keyword.clone()
        } else {
            TokenType::Identifier
        };
        self.add_token(token_type);
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current].to_string();
        self.add_token_with_literal(
            TokenType::Number,
            value.parse::<f64>().map(|v| DataType::Number(v)).ok(),
        );
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // TODO: error
            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current + 1].to_string();
        self.add_token_with_literal(TokenType::String, Some(DataType::String(value)));
    }

    fn peek(&self) -> char {
        self.ch
    }

    fn peek_next(&self) -> char {
        if let Some(c) = self.iter.peek().cloned() {
            c
        } else {
            '\0'
        }
    }

    fn is_alpha_numberic(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let Some(c) = self.iter.next() {
            if c == expected {
                self.ch = c;
                return true;
            }
        }

        self.ch = '\0';
        return false;
    }

    fn is_at_end(&self) -> bool {
        self.ch == '\0'
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<DataType>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }
}
