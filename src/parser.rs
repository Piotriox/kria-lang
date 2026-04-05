use crate::lexer::Token;
use crate::ast::{Statement, Expression, Literal, BinaryOperator};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }
    
    fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }
    
    fn peek_token(&self) -> &Token {
        self.tokens.get(self.position + 1).unwrap_or(&Token::Eof)
    }
    
    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }
    
    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if std::mem::discriminant(self.current_token()) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, self.current_token()))
        }
    }
    
    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        
        while self.current_token() != &Token::Eof {
            statements.push(self.parse_statement()?);
        }
        
        Ok(statements)
    }
    
    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token() {
            Token::Set => self.parse_assignment(),
            Token::Print => self.parse_print(),
            _ => Err(format!("Unexpected token: {:?}", self.current_token())),
        }
    }
    
    fn parse_assignment(&mut self) -> Result<Statement, String> {
        self.expect(Token::Set)?;
        
        let name = match self.current_token() {
            Token::Identifier(n) => {
                let n_clone = n.clone();
                self.advance();
                n_clone
            }
            _ => return Err("Expected identifier after 'set'".to_string()),
        };
        
        self.expect(Token::Equal)?;
        let value = self.parse_expression()?;
        self.expect(Token::Semicolon)?;
        
        Ok(Statement::Assignment { name, value })
    }
    
    fn parse_print(&mut self) -> Result<Statement, String> {
        self.expect(Token::Print)?;
        self.expect(Token::LParen)?;
        let expr = self.parse_expression()?;
        self.expect(Token::RParen)?;
        self.expect(Token::Semicolon)?;
        
        Ok(Statement::Print(expr))
    }
    
    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_additive()
    }
    
    fn parse_additive(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_multiplicative()?;
        
        loop {
            match self.current_token() {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_multiplicative()?;
                    left = Expression::BinaryOp {
                        left: Box::new(left),
                        op: BinaryOperator::Add,
                        right: Box::new(right),
                    };
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_multiplicative()?;
                    left = Expression::BinaryOp {
                        left: Box::new(left),
                        op: BinaryOperator::Subtract,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_multiplicative(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_primary()?;
        
        loop {
            match self.current_token() {
                Token::Star => {
                    self.advance();
                    let right = self.parse_primary()?;
                    left = Expression::BinaryOp {
                        left: Box::new(left),
                        op: BinaryOperator::Multiply,
                        right: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_primary()?;
                    left = Expression::BinaryOp {
                        left: Box::new(left),
                        op: BinaryOperator::Divide,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_primary(&mut self) -> Result<Expression, String> {
        match self.current_token() {
            Token::Number(n) => {
                let n_val = *n;
                self.advance();
                Ok(Expression::Literal(Literal::Number(n_val)))
            }
            Token::String(s) => {
                let s_val = s.clone();
                self.advance();
                Ok(Expression::Literal(Literal::String(s_val)))
            }
            Token::True => {
                self.advance();
                Ok(Expression::Literal(Literal::Boolean(true)))
            }
            Token::False => {
                self.advance();
                Ok(Expression::Literal(Literal::Boolean(false)))
            }
            Token::Null => {
                self.advance();
                Ok(Expression::Literal(Literal::Null))
            }
            Token::Identifier(name) => {
                let name_val = name.clone();
                self.advance();
                Ok(Expression::Identifier(name_val))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token in expression: {:?}", self.current_token())),
        }
    }
}