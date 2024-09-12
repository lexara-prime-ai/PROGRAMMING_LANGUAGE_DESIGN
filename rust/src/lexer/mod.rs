use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
pub enum Token {
    Eof,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
    Unknown(char),
}

pub struct Lexer {
    pub input: String,
    pub index: usize,
    pub current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            index: 0,
            current_char: None,
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.index >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = self.input.chars().nth(self.index);
            self.index += 1;
        }
    }

    pub fn get_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char {
            Some(c) if c.is_alphabetic() => self.read_identifier_or_keyword(),
            Some(c) if c.is_digit(10) || c == '.' => self.read_number(),
            Some('#') => {
                self.skip_comment();
                self.get_token()
            }
            Some(c) => {
                self.read_char();
                Token::Unknown(c)
            }
            None => Token::Eof,
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    pub fn skip_comment(&mut self) {
        while let Some(c) = self.current_char {
            if c == '\n' || c == '\r' {
                break;
            }
            self.read_char();
        }
    }

    pub fn read_identifier_or_keyword(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() {
                identifier.push(c);
                self.read_char();
            } else {
                break;
            }
        }

        match identifier.as_str() {
            "def" => Token::Def,
            "extern" => Token::Extern,
            _ => Token::Identifier(identifier),
        }
    }

    pub fn read_number(&mut self) -> Token {
        let mut number_str = String::new();
        while let Some(c) = self.current_char {
            if c.is_digit(10) || c == '.' {
                number_str.push(c);
                self.read_char();
            } else {
                break;
            }
        }

        let number = number_str.parse::<f64>().unwrap_or(0.0);
        Token::Number(number)
    }
}
