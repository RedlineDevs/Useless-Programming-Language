use crate::ast::{ BinaryOp, Expression, Literal, Program, Statement };
use crate::lexer::Token;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token: {0:?}")] UnexpectedToken(Token),
    #[error("Expected token, but got none")]
    UnexpectedEof,
    #[error("Invalid string literal")]
    InvalidStringLiteral,
    #[error("Invalid number literal")]
    InvalidNumberLiteral,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut program = Vec::new();
        while !self.is_at_end() {
            program.push(self.parse_statement()?);
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.peek() {
            Some(Token::Let) => self.parse_let_statement(),
            Some(Token::Print) => self.parse_print_statement(),
            Some(Token::If) => self.parse_if_statement(),
            Some(Token::Loop) => self.parse_loop_statement(),
            Some(Token::Save) => self.parse_save_statement(),
            _ => {
                let expr = self.parse_expression()?;
                self.consume(Token::Semicolon)?;
                Ok(Statement::Expression(expr))
            }
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'let'
        let name = match self.advance() {
            Some(Token::Identifier) => "temp".to_string(), // We'll improve this later
            _ => {
                return Err(ParseError::UnexpectedToken(self.previous().unwrap()));
            }
        };

        self.consume(Token::Equals)?;
        let value = self.parse_expression()?;
        self.consume(Token::Semicolon)?;

        Ok(Statement::Let { name, value })
    }

    fn parse_print_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'print'
        self.consume(Token::LeftParen)?;
        let value = self.parse_expression()?;
        self.consume(Token::RightParen)?;
        self.consume(Token::Semicolon)?;

        Ok(Statement::Print { value })
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        match self.peek() {
            Some(Token::StringLiteral) => {
                self.advance();
                Ok(Expression::Literal(Literal::String("temp".to_string()))) // We'll improve this later
            }
            Some(Token::NumberLiteral) => {
                self.advance();
                Ok(Expression::Literal(Literal::Number(42))) // We'll improve this later
            }
            Some(Token::Add) | Some(Token::Multiply) => {
                let op = match self.advance().unwrap() {
                    Token::Add => BinaryOp::Add,
                    Token::Multiply => BinaryOp::Multiply,
                    _ => unreachable!(),
                };

                self.consume(Token::LeftParen)?;
                let left = self.parse_expression()?;
                self.consume(Token::Comma)?;
                let right = self.parse_expression()?;
                self.consume(Token::RightParen)?;

                Ok(Expression::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            Some(Token::Identifier) => {
                self.advance();
                if self.peek() == Some(&Token::LeftParen) {
                    self.parse_function_call("temp".to_string()) // We'll improve this later
                } else {
                    Ok(Expression::Identifier("temp".to_string())) // We'll improve this later
                }
            }
            _ =>
                Err(ParseError::UnexpectedToken(self.peek().cloned().unwrap_or(Token::Whitespace))),
        }
    }

    fn parse_function_call(&mut self, name: String) -> Result<Expression, ParseError> {
        self.consume(Token::LeftParen)?;
        let mut arguments = Vec::new();

        if self.peek() != Some(&Token::RightParen) {
            loop {
                arguments.push(self.parse_expression()?);
                if self.peek() != Some(&Token::Comma) {
                    break;
                }
                self.advance(); // consume comma
            }
        }

        self.consume(Token::RightParen)?;
        Ok(Expression::FunctionCall { name, arguments })
    }

    fn consume(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.peek() == Some(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(self.peek().cloned().unwrap_or(Token::Whitespace)))
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<Token> {
        if self.current > 0 { self.tokens.get(self.current - 1).cloned() } else { None }
    }

    fn advance(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'if'
        self.consume(Token::LeftParen)?;
        let condition = self.parse_expression()?;
        self.consume(Token::RightParen)?;

        self.consume(Token::LeftBrace)?;
        let mut then_branch = Vec::new();
        while self.peek() != Some(&Token::RightBrace) {
            then_branch.push(self.parse_statement()?);
        }
        self.consume(Token::RightBrace)?;

        let else_branch = if self.peek() == Some(&Token::Else) {
            self.advance(); // consume 'else'
            self.consume(Token::LeftBrace)?;
            let mut statements = Vec::new();
            while self.peek() != Some(&Token::RightBrace) {
                statements.push(self.parse_statement()?);
            }
            self.consume(Token::RightBrace)?;
            Some(statements)
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_loop_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'loop'
        self.consume(Token::LeftBrace)?;

        let mut body = Vec::new();
        while self.peek() != Some(&Token::RightBrace) {
            body.push(self.parse_statement()?);
        }
        self.consume(Token::RightBrace)?;

        Ok(Statement::Loop { body })
    }

    fn parse_save_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'save'
        self.consume(Token::LeftParen)?;
        let filename = self.parse_expression()?;
        self.consume(Token::RightParen)?;
        self.consume(Token::Semicolon)?;

        Ok(Statement::Save { filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_print_statement() {
        let input = "print(\"Hello, World!\");";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();
        let mut parser = Parser::new(tokens);

        let program = parser.parse().unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Statement::Print { value: _ } => (),
            _ => panic!("Expected print statement"),
        }
    }

    #[test]
    fn test_parse_let_statement() {
        let input = "let x = 42;";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();
        let mut parser = Parser::new(tokens);

        let program = parser.parse().unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Statement::Let { name: _, value: _ } => (),
            _ => panic!("Expected let statement"),
        }
    }

    #[test]
    fn test_parse_binary_op() {
        let input = "add(5, 3);";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();
        let mut parser = Parser::new(tokens);

        let program = parser.parse().unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Statement::Expression(Expression::BinaryOp { op, left: _, right: _ }) => {
                assert!(matches!(op, BinaryOp::Add));
            }
            _ => panic!("Expected binary operation"),
        }
    }
}
