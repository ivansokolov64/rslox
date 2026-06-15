use crate::errors::{LoxError, ScannerError};
use crate::loxobject::LoxObject;
use crate::token::{Token, TokenType};

fn keyword(s: &str) -> Option<TokenType> {
    match s {
        "and" => Some(TokenType::And),
        "class" => Some(TokenType::Class),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "for" => Some(TokenType::For),
        "fun" => Some(TokenType::Fun),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}

pub struct Scanner {
    source: String,
    current: usize,
    start: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            current: 0,
            start: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxError> {
        while self.current < self.source.len() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.add_without_literal(TokenType::EOF)?;

        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.next();

        match c.unwrap() {
            // Single characters
            '(' => self.add_without_literal(TokenType::LeftParen),
            ')' => self.add_without_literal(TokenType::RightParen),
            '{' => self.add_without_literal(TokenType::LeftBrace),
            '}' => self.add_without_literal(TokenType::RightBrace),
            ',' => self.add_without_literal(TokenType::Comma),
            '.' => self.add_without_literal(TokenType::Dot),
            '-' => self.add_without_literal(TokenType::Minus),
            '+' => self.add_without_literal(TokenType::Plus),
            ';' => self.add_without_literal(TokenType::Semicolon),
            '*' => self.add_without_literal(TokenType::Star),
            '?' => self.add_without_literal(TokenType::Question),
            ':' => self.add_without_literal(TokenType::Colon),

            // One or two characters
            '!' => {
                let t = if self.match_next('=')? {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_without_literal(t)
            }
            '=' => {
                let t = if self.match_next('=')? {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_without_literal(t)
            }
            '<' => {
                let t = if self.match_next('=')? {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_without_literal(t)
            }
            '>' => {
                let t = if self.match_next('=')? {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_without_literal(t)
            }

            // Special handling of SLASH for comments or division
            '/' => {
                if self.match_next('/')? {
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.next();
                    }
                    Ok(())
                } else if self.match_next('*')? {
                    while !(self.peek() == Some('*') && self.peek_next() == Some('/'))
                        && !self.is_at_end()
                    {
                        self.next();
                    }
                    self.next(); // consume '*'
                    self.next(); // consume '/'
                    Ok(())
                } else {
                    self.add_without_literal(TokenType::Slash)
                }
            }

            '"' => self.string(),

            // White space, new lines, etc.
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => {
                self.line += 1;
                Ok(())
            }

            '0'..='9' => self.number(),

            c if c.is_alphabetic() => self.identifier(),

            o => Err(LoxError::ScannerError(
                self.line,
                ScannerError::UnexpectedCharacter(o),
            )),
        }
    }

    // Functions for handling specific tokens

    fn string(&mut self) -> Result<(), LoxError> {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1
            };
            self.next();
        }

        if self.is_at_end() {
            return Err(LoxError::ScannerError(
                self.line,
                ScannerError::StringNotTerminated,
            ));
        }

        self.next();

        let value: String = self.source[self.start + 1..self.current - 1].to_string();

        self.add_token(TokenType::String, LoxObject::String(value))
    }

    fn number(&mut self) -> Result<(), LoxError> {
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.next();
        }

        if self.peek() == Some('.') && self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
            self.next();

            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.next();
            }
        }

        match self.source[self.start..self.current].parse::<f64>() {
            Ok(number) => self.add_token(TokenType::Number, LoxObject::Number(number)),
            Err(_) => Err(LoxError::ScannerError(
                self.line,
                ScannerError::InvalidNumber,
            )),
        }
    }

    fn identifier(&mut self) -> Result<(), LoxError> {
        while self.peek().is_some_and(|c| c.is_ascii_alphanumeric()) {
            self.next();
        }

        let text = &self.source[self.start..self.current];

        match keyword(text) {
            None => self.add_without_literal(TokenType::Identifier),
            Some(token_type) => self.add_without_literal(token_type),
        }
    }

    fn add_without_literal(&mut self, token_type: TokenType) -> Result<(), LoxError> {
        self.add_token(token_type, LoxObject::Nil)
    }

    fn add_token(&mut self, token_type: TokenType, literal: LoxObject) -> Result<(), LoxError> {
        let text: String = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
        Ok(())
    }

    // Character processing

    fn match_next(&mut self, expected: char) -> Result<bool, LoxError> {
        match self.peek() {
            None => Ok(false),
            Some(c) if c != expected => Ok(false),
            _ => {
                self.next();
                Ok(true)
            }
        }
    }
    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return Some('\0');
        }
        self.source[self.current..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        self.source[self.current..].chars().nth(1)
    }

    fn next(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.current += c.unwrap().len_utf8();
        }
        c
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
