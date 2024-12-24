use crate::ast::{ BinaryOp, Expression, Literal, Program, Statement };
use crate::lexer::{ Token, TokenKind };
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
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
        match self.peek().map(|t| &t.kind) {
            Some(TokenKind::Let) => self.parse_let_statement(),
            Some(TokenKind::Print) => self.parse_print_statement(),
            Some(TokenKind::If) => self.parse_if_statement(),
            Some(TokenKind::Loop) => self.parse_loop_statement(),
            Some(TokenKind::Save) => self.parse_save_statement(),
            _ => {
                let expr = self.parse_expression()?;
                self.consume(&TokenKind::Semicolon)?;
                Ok(Statement::Expression(expr))
            }
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'let'
        let name = match self.advance() {
            Some(token) if token.kind == TokenKind::Identifier => token.text,
            _ => {
                return Err(ParseError::UnexpectedToken(self.previous().unwrap()));
            }
        };

        self.consume(&TokenKind::Equals)?;
        let value = self.parse_expression()?;
        self.consume(&TokenKind::Semicolon)?;

        Ok(Statement::Let { name, value })
    }

    fn parse_print_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'print'
        self.consume(&TokenKind::LeftParen)?;
        let value = self.parse_expression()?;
        self.consume(&TokenKind::RightParen)?;
        self.consume(&TokenKind::Semicolon)?;

        Ok(Statement::Print { value })
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        match self.peek().map(|t| &t.kind) {
            Some(TokenKind::StringLiteral) => {
                let token = self.advance().unwrap();
                let content = token.text.trim_matches('"').to_string();
                Ok(Expression::Literal(Literal::String(content)))
            }
            Some(TokenKind::NumberLiteral) => {
                let token = self.advance().unwrap();
                let number = token.text
                    .parse::<i64>()
                    .map_err(|_| ParseError::InvalidNumberLiteral)?;
                Ok(Expression::Literal(Literal::Number(number)))
            }
            Some(TokenKind::Add) | Some(TokenKind::Multiply) => {
                let op = match self.advance().unwrap().kind {
                    TokenKind::Add => BinaryOp::Add,
                    TokenKind::Multiply => BinaryOp::Multiply,
                    _ => unreachable!(),
                };

                self.consume(&TokenKind::LeftParen)?;
                let left = self.parse_expression()?;
                self.consume(&TokenKind::Comma)?;
                let right = self.parse_expression()?;
                self.consume(&TokenKind::RightParen)?;

                Ok(Expression::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            Some(TokenKind::Identifier) => {
                let token = self.advance().unwrap();
                let name = token.text;
                if self.peek().map(|t| &t.kind) == Some(&TokenKind::LeftParen) {
                    self.parse_function_call(name)
                } else {
                    Ok(Expression::Identifier(name))
                }
            }
            _ =>
                Err(
                    ParseError::UnexpectedToken(
                        self
                            .peek()
                            .cloned()
                            .unwrap_or_else(|| Token::new(TokenKind::Whitespace, String::new()))
                    )
                ),
        }
    }

    fn parse_function_call(&mut self, name: String) -> Result<Expression, ParseError> {
        self.consume(&TokenKind::LeftParen)?;
        let mut arguments = Vec::new();

        if self.peek().map(|t| &t.kind) != Some(&TokenKind::RightParen) {
            loop {
                arguments.push(self.parse_expression()?);
                if self.peek().map(|t| &t.kind) != Some(&TokenKind::Comma) {
                    break;
                }
                self.advance(); // consume comma
            }
        }

        self.consume(&TokenKind::RightParen)?;
        Ok(Expression::FunctionCall { name, arguments })
    }

    fn consume(&mut self, expected: &TokenKind) -> Result<(), ParseError> {
        if self.peek().map(|t| &t.kind) == Some(expected) {
            self.advance();
            Ok(())
        } else {
            Err(
                ParseError::UnexpectedToken(
                    self
                        .peek()
                        .cloned()
                        .unwrap_or_else(|| Token::new(TokenKind::Whitespace, String::new()))
                )
            )
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
        self.consume(&TokenKind::LeftParen)?;
        let condition = self.parse_expression()?;
        self.consume(&TokenKind::RightParen)?;

        self.consume(&TokenKind::LeftBrace)?;
        let mut then_branch = Vec::new();
        while self.peek().map(|t| &t.kind) != Some(&TokenKind::RightBrace) {
            then_branch.push(self.parse_statement()?);
        }
        self.consume(&TokenKind::RightBrace)?;

        let else_branch = if self.peek().map(|t| &t.kind) == Some(&TokenKind::Else) {
            self.advance(); // consume 'else'
            self.consume(&TokenKind::LeftBrace)?;
            let mut statements = Vec::new();
            while self.peek().map(|t| &t.kind) != Some(&TokenKind::RightBrace) {
                statements.push(self.parse_statement()?);
            }
            self.consume(&TokenKind::RightBrace)?;
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
        self.consume(&TokenKind::LeftBrace)?;

        let mut body = Vec::new();
        while self.peek().map(|t| &t.kind) != Some(&TokenKind::RightBrace) {
            body.push(self.parse_statement()?);
        }
        self.consume(&TokenKind::RightBrace)?;

        Ok(Statement::Loop { body })
    }

    fn parse_save_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'save'
        self.consume(&TokenKind::LeftParen)?;
        let filename = self.parse_expression()?;
        self.consume(&TokenKind::RightParen)?;
        self.consume(&TokenKind::Semicolon)?;

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
