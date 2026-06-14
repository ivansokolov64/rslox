use crate::errors::{LoxError, ParseError};
use crate::expr::Expr;
use crate::token::{Token, TokenType};
use crate::loxobject::LoxObject;
use crate::stmt::Stmt;

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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {

        let mut statements: Vec<Stmt> = Vec::new();

        loop {
            match self.is_at_end() {
                Ok(v) => {
                    if v {
                        break;
                    }
                }
                Err(e) => {
                    return Err(LoxError::ParseError(self.peek().cloned(), e))
                }
            }

            match self.declaration() {
                Ok(next) => {
                    statements.push(next);
                }
                Err(e) => {
                    match &e {
                        ParseError::InvalidToken(_, _) | ParseError::ExpectExpression => {
                        self.synchronize().map_err(|e| LoxError::ParseError(None, e))?;
                        }
                        _ => {}
                    }

                    let err = LoxError::ParseError(self.previous().cloned(), e);
                    eprintln!("{err}");

                }
            }

        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_tokens(&[TokenType::Var])? {
            self.var_declaration()
        }
        else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {

        let name = match self.consume(TokenType::Identifier)? {
            Some(token) => token.clone(),
            None => return Err(ParseError::ExpectExpression),
        };

        let mut initializer: Option<Expr> = None;

        let has_initializer = self.match_tokens(&[TokenType::Equal])?;
        if has_initializer {
            initializer = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon)?;

        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_tokens(&[TokenType::Print])? {
            self.print_statement()
        }
        else if self.match_tokens(&[TokenType::LeftBrace])? {
            self.block()
        }
        else {
            self.expression_statement()
        }
    }

    fn block(&mut self) -> Result<Stmt, ParseError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(&TokenType::RightBrace)? && !self.is_at_end()? {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace)?;

        Ok(Stmt::Block(statements))
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon)?;
        Ok(Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon)?;

        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.comma()
    }

    fn comma(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.assignment()?;

        while self.match_tokens(&[TokenType::Comma])? {
            let operator = self.previous()
                .expect("previous() should exist after match_tokens()")
                .clone();

            let right = self.assignment()?;

            expr = Expr::Binary {
                left: Box::from(expr),
                operator,
                right: Box::from(right)
            }

        }
        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.ternary()?;

        if self.match_tokens(&[TokenType::Equal])? {

            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::from(value)
                })
            };

            Err(ParseError::InvalidAssignmentTarget)
        }
        else {
            Ok(expr)
        }

    }

    fn ternary(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        if self.match_tokens(&[TokenType::Question])? {
            let operator = self.previous()
                .expect("previous() should exist after match_tokens()")
                .clone();
            let then_branch = self.equality()?;
            self.consume(TokenType::Colon)?;
            let else_branch = self.ternary()?;

            expr = Expr::Ternary {
                if_expr: Box::from(expr),
                then_branch: Box::from(then_branch),
                else_branch: Box::from(else_branch),
                operator

            }

        }
        Ok(expr)


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
            return Ok(Expr::Literal(LoxObject::Boolean(false)));
        }

        if self.match_tokens(&[TokenType::True])? {
            return Ok(Expr::Literal(LoxObject::Boolean(true)));
        }

        if self.match_tokens(&[TokenType::Nil])? {
            return Ok(Expr::Literal(LoxObject::Nil));
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

        if self.match_tokens(&[TokenType::Identifier])? {

            let Some(token) = self.previous() else {
                return Err(ParseError::OutOfBounds)
            };

            return Ok(Expr::Variable(token.clone()));
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
                return Err(ParseError::OutOfBounds);
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
            let Some(t) = self.peek() else {
                return Err(ParseError::OutOfBounds)
            };
            Err(ParseError::InvalidToken(token_type, t.token_type.clone()))
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