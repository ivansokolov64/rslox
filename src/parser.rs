use crate::errors::{LoxError, ParseError};
use crate::expr::Expr;
use crate::token::{Token, TokenType};
use crate::interpreter::LoxObject;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Result<Expr, LoxError> {

        match self.expression() {
            Ok(expr) => {
                Ok(expr)
            }
            Err(e) => {
                Err(LoxError::ParseError(self.peek().cloned(), e))
            }
        }
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {

        let mut expr = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual])? {

            let operator = self
                .previous()
                .expect("previous() should exist after match_tokens()")
                .clone();

            let right = self.comparison()?;

            expr = Expr::Binary {
                left: Box::from(expr),
                operator,
                right: Box::from(right)
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_tokens(&[TokenType::Greater, TokenType::GreaterEqual,
            TokenType::Less, TokenType::LessEqual])? {
                let operator = self
                    .previous()
                    .expect("previous() should exist after match_tokens()")
                    .clone();
                let right = self.term()?;

            expr = Expr::Binary {
                left: Box::from(expr),
                operator,
                right: Box::from(right)
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {

        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus])? {
            let operator = self
                .previous()
                .expect("previous() should exist after match_tokens()")
                .clone();
            let right = self.factor()?;

            expr = Expr::Binary {
                left: Box::from(expr),
                operator,
                right: Box::from(right)
            }
        }

        Ok(expr)

    }

    fn factor(&mut self) -> Result<Expr, ParseError> {

        let mut expr = self.unary()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star])? {
            let operator = self
                .previous()
                .expect("previous() should exist after match_tokens()")
                .clone();

            let right = self.unary()?;

            expr = Expr::Binary {
                left: Box::from(expr),
                operator,
                right: Box::from(right)
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus])? {
            let operator = self
                .previous()
                .expect("previous() should exist after match_tokens()")
                .clone();

            let right = self.unary()?;

            Ok(Expr::Unary {
                operator,
                right: Box::from(right)
            })
        }
        else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_tokens(&[TokenType::False])? {
            return Ok(Expr::Literal(Some(LoxObject::Boolean(false))));
        }

        if self.match_tokens(&[TokenType::True])? {
            return Ok(Expr::Literal(Some(LoxObject::Boolean(true))));
        }

        if self.match_tokens(&[TokenType::Nil])? {
            return Ok(Expr::Literal(Some(LoxObject::Nil)));
        }

        if self.match_tokens(&[TokenType::Number, TokenType::String])? {
            let Some(token) = self.previous() else {
                return Err(ParseError::OutOfBounds)
            };
            return Ok(Expr::Literal(token.literal.clone()));
        }

        if self.match_tokens(&[TokenType::LeftParen])? {
            let expr = self.expression()?;

            self.consume(TokenType::RightParen)?;

            return Ok(Expr::Grouping(Box::new(expr)));
        }

        match self.peek() {
            None => {
                Err(ParseError::OutOfBounds)
            }
            Some(_) => {
                Err(ParseError::ExpectExpression)
            }
        }


    }


    // Primitive operations

    fn synchronize(&mut self) -> Result<(), ParseError> {
        self.next()?;

        while !self.is_at_end()? {
            let Some(token) = self.previous() else {
                return Err(ParseError::OutOfBounds)
            };

            if let TokenType::Semicolon = token.token_type {
                return Ok(())
            }

            let Some(token) = self.peek() else {
                return Err(ParseError::OutOfBounds)
            };

            match token.token_type {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For | TokenType::If |
                TokenType::While | TokenType::Print | TokenType::Return
                    => return Ok(()),
                _ => self.next()?,
            };
        }

        Ok(())
    }

    fn consume(&mut self, token_type: TokenType) -> Result<Option<&Token>, ParseError> {
        if self.check(&token_type)? {
            self.next()
        }
        else {
            let Some(_) = self.peek() else {
                return Err(ParseError::OutOfBounds)
            };
            Err(ParseError::InvalidToken)
        }
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> Result<bool, ParseError> {
        for token_type in types {
            if self.check(token_type)? {
                self.next()?;
                return Ok(true)
            }
        }
        Ok(false)

    }

    fn check(&self, token_type: &TokenType) -> Result<bool, ParseError> {
        if self.is_at_end()? {
            Ok(false)
        } else {
            match self.peek() {
                None => {
                    panic!()
                }
                Some(token) => {
                    Ok(token.token_type == token_type.clone())
                }
            }
        }
    }

    fn next(&mut self) -> Result<Option<&Token>, ParseError> {
        if !self.is_at_end()? {
            self.current += 1;
        }
        Ok(self.previous())
    }

    fn is_at_end(&self) -> Result<bool, ParseError> {
        match self.peek() {
            None => {
                Err(ParseError::OutOfBounds)
            }
            Some(token) => {
                match token.token_type {
                    TokenType::EOF => Ok(true),
                    _ => Ok(false)
                }
            }
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }


}