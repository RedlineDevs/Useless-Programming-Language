//! # Parser Module
//!
//! The parser module is responsible for converting tokens into an Abstract Syntax Tree (AST).
//! It tries its best to make sense of your code, but don't get your hopes up.
//!
//! ## Example
//! ```rust
//! use useless_lang::parser::Parser;
//! use useless_lang::lexer::Lexer;
//!
//! let input = "print(\"Hello, World!\");";
//! let lexer = Lexer::new(input);
//! let tokens = lexer.collect();
//! let mut parser = Parser::new(tokens);
//! let ast = parser.parse().expect("Parser failed successfully");
//! ```

use crate::ast::{BinaryOp, Expression, Literal, Program, Statement};
use crate::lexer::{Token, TokenKind};
use thiserror::Error;

/// Errors that might occur during parsing.
/// These are the only predictable things about the language.
#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ParseError {
    /// Found a token we weren't expecting (which is all of them)
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),

    /// Reached the end of input prematurely (or did we?)
    #[error("Expected token, but got none")]
    UnexpectedEof,

    /// Found a string literal that's not quite right
    #[error("Invalid string literal")]
    InvalidStringLiteral,

    /// Found a number literal that's more creative than we can handle
    #[error("Invalid number literal")]
    InvalidNumberLiteral,
}

/// The parser for the Useless Programming Language.
/// It converts tokens into an AST, assuming you're lucky.
pub struct Parser {
    /// The tokens to parse (or misparse)
    tokens: Vec<Token>,
    /// Current position in the token stream
    current: usize,
}

impl Parser {
    /// Creates a new parser from a vector of tokens.
    /// Use at your own risk.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Attempts to parse a complete program.
    /// Returns a Result containing either a Program or a ParseError.
    /// The Program might not do what you want, but at least it's valid syntax!
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut program = Vec::new();
        while !self.is_at_end() {
            program.push(self.parse_statement()?);
        }
        Ok(program)
    }

    /// Parses a single statement.
    /// Each statement has an equal chance of doing something unexpected.
    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        // Parse attributes that may precede the statement
        let mut attributes = Vec::new();
        while self.peek().map(|t| &t.kind) == Some(&TokenKind::Attribute) {
            let token = self.advance().unwrap();
            // Extract attribute name and optional parameters
            let content = &token.text[2..token.text.len()-1];
            if let Some(paren_idx) = content.find('(') {
                let name = content[..paren_idx].to_string();
                let params = content[paren_idx+1..content.len()-1].to_string();
                attributes.push((name, Some(params)));
            } else {
                attributes.push((content.to_string(), None));
            }
        }

        let statement = match self.peek().map(|t| &t.kind) {
            Some(TokenKind::Module) => self.parse_module()?,
            Some(TokenKind::Use) => self.parse_use()?,
            Some(TokenKind::Let) => self.parse_let_statement()?,
            Some(TokenKind::Directive) => {
                let token = self.advance().unwrap();
                let name = token.text[11..token.text.len()-2].to_string();
                Statement::Attributed {
                    name,
                    statement: Box::new(self.parse_statement()?)
                }
            },
            Some(TokenKind::Print) => self.parse_print_statement()?,
            Some(TokenKind::If) => self.parse_if_statement()?,
            Some(TokenKind::Loop) => self.parse_loop_statement()?,
            Some(TokenKind::Save) => {
                self.advance(); // consume save
                let filename = match self.advance() {
                    Some(token) if token.kind == TokenKind::StringLiteral => token.text.trim_matches('"').to_string(),
                    _ => return Err(ParseError::UnexpectedToken(self.previous().unwrap())),
                };
                self.consume(&TokenKind::Semicolon)?;
                Statement::Save { filename }
            },
            Some(TokenKind::Exit) => {
                self.advance();  // consume 'exit'
                self.consume(&TokenKind::LeftParen)?;  // expect (
                self.consume(&TokenKind::RightParen)?;  // expect )
                self.consume(&TokenKind::Semicolon)?;  // expect semicolon
                Statement::Expression(Expression::FunctionCall {
                    name: "exit".to_string(),
                    arguments: vec![],
                })
            },
            Some(TokenKind::Async) => {
                self.advance(); // consume async
                let name = match self.advance() {
                    Some(token) if token.kind == TokenKind::Identifier => token.text,
                    _ => return Err(ParseError::UnexpectedToken(self.previous().unwrap())),
                };

                self.consume(&TokenKind::LeftParen)?;
                let mut parameters = Vec::new();
                while self.peek().map(|t| &t.kind) != Some(&TokenKind::RightParen) {
                    match self.advance() {
                        Some(token) if token.kind == TokenKind::Identifier => {
                            parameters.push(token.text);
                        },
                        _ => return Err(ParseError::UnexpectedToken(self.previous().unwrap())),
                    }
                    if self.peek().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                        self.advance(); // consume comma
                    }
                }
                self.consume(&TokenKind::RightParen)?;

                self.consume(&TokenKind::LeftBrace)?;
                let mut body = Vec::new();
                while self.peek().map(|t| &t.kind) != Some(&TokenKind::RightBrace) {
                    body.push(self.parse_statement()?);
                }
                self.consume(&TokenKind::RightBrace)?;

                Statement::AsyncFunction { name, parameters, body }
            },
            Some(TokenKind::Try) => {
                self.advance(); // consume try
                self.consume(&TokenKind::LeftBrace)?;
                let mut try_block = Vec::new();
                while self.peek().map(|t| &t.kind) != Some(&TokenKind::RightBrace) {
                    try_block.push(self.parse_statement()?);
                }
                self.consume(&TokenKind::RightBrace)?;

                self.consume(&TokenKind::Catch)?;
                let error_var = match self.advance() {
                    Some(token) if token.kind == TokenKind::Identifier => token.text,
                    _ => return Err(ParseError::UnexpectedToken(self.previous().unwrap())),
                };

                self.consume(&TokenKind::LeftBrace)?;
                let mut catch_block = Vec::new();
                while self.peek().map(|t| &t.kind) != Some(&TokenKind::RightBrace) {
                    catch_block.push(self.parse_statement()?);
                }
                self.consume(&TokenKind::RightBrace)?;

                Statement::TryCatch {
                    try_block,
                    error_var,
                    catch_block,
                }
            },
            Some(TokenKind::Await) => {
                self.advance(); // consume await
                let expression = self.parse_expression()?;
                self.consume(&TokenKind::Semicolon)?;
                Statement::Await { expression }
            },
            Some(TokenKind::Identifier) => {
                let name = match self.advance() {
                    Some(token) if token.kind == TokenKind::Identifier => token.text,
                    _ => return Err(ParseError::UnexpectedToken(self.previous().unwrap())),
                };

                // Check if this is a function declaration or call
                if self.peek().map(|t| &t.kind) == Some(&TokenKind::LeftParen) {
                    self.consume(&TokenKind::LeftParen)?;
                    let mut arguments = Vec::new();
                    while self.peek().map(|t| &t.kind) != Some(&TokenKind::RightParen) {
                        arguments.push(self.parse_expression()?);
                        if self.peek().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                            self.advance(); // consume comma
                        }
                    }
                    self.consume(&TokenKind::RightParen)?;

                    // If followed by { it's a function declaration
                    if self.peek().map(|t| &t.kind) == Some(&TokenKind::LeftBrace) {
                        self.consume(&TokenKind::LeftBrace)?;
                        let mut body = Vec::new();
                        while self.peek().map(|t| &t.kind) != Some(&TokenKind::RightBrace) {
                            body.push(self.parse_statement()?);
                        }
                        self.consume(&TokenKind::RightBrace)?;
                        Ok(Statement::Function {
                            name,
                            parameters: arguments.into_iter()
                                .filter_map(|arg| match arg {
                                    Expression::Identifier(name) => Some(name),
                                    _ => None,
                                })
                                .collect(),
                            body
                        })
                    } else {
                        // Otherwise it's a function call
                        self.consume(&TokenKind::Semicolon)?;
                        Ok(Statement::Expression(Expression::FunctionCall {
                            name,
                            arguments,
                        }))
                    }
                } else {
                    // Not a function, treat as expression
                    let expr = Expression::Identifier(name);
                    self.consume(&TokenKind::Semicolon)?;
                    Ok(Statement::Expression(expr))
                }
            }?,
            _ => {
                let expr = self.parse_expression()?;
                self.consume(&TokenKind::Semicolon)?;
                Statement::Expression(expr)
            }
        };

        // If we have attributes, wrap the statement
        if !attributes.is_empty() {
            Ok(Statement::Attributed {
                name: attributes[0].0.clone(),
                statement: Box::new(statement)
            })
        } else {
            Ok(statement)
        }
    }

    /// Parses a let statement, which might let your variables go on vacation.
    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'let'
        let name = match self.advance() {
            Some(token) if token.kind == TokenKind::Identifier => token.text,
            _ => {
                return Err(ParseError::UnexpectedToken(self.previous().unwrap()));
            }
        };

        self.consume(&TokenKind::Assignment)?;
        let value = self.parse_expression()?;
        self.consume(&TokenKind::Semicolon)?;

        Ok(Statement::Let { name, value })
    }

    /// Parses a print statement that will open random websites.
    fn parse_print_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'print'
        self.consume(&TokenKind::LeftParen)?;
        let value = self.parse_expression()?;
        self.consume(&TokenKind::RightParen)?;
        self.consume(&TokenKind::Semicolon)?;

        Ok(Statement::Print { value })
    }

    /// Parses an expression, which might evaluate to something entirely different.
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        match self.peek().map(|t| &t.kind) {
            Some(TokenKind::StringLiteral) => {
                let token = self.advance().unwrap();
                let content = token.text.trim_matches('"').to_string();
                Ok(Expression::Literal(Literal::String(content)))
            }
            Some(TokenKind::NumberLiteral) => {
                let token = self.advance().unwrap();
                let number = token
                    .text
                    .parse::<i64>()
                    .map_err(|_| ParseError::InvalidNumberLiteral)?;
                Ok(Expression::Literal(Literal::Number(number)))
            }
            Some(TokenKind::True) => {
                self.advance();
                Ok(Expression::Literal(Literal::Boolean(true)))
            }
            Some(TokenKind::False) => {
                self.advance();
                Ok(Expression::Literal(Literal::Boolean(false)))
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
            Some(TokenKind::LeftBracket) => {
                self.advance(); // consume [
                let mut elements = Vec::new();

                while self.peek().map(|t| &t.kind) != Some(&TokenKind::RightBracket) {
                    elements.push(Box::new(self.parse_expression()?));
                    if self.peek().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                        self.advance(); // consume comma
                    }
                }

                self.consume(&TokenKind::RightBracket)?;
                Ok(Expression::Literal(Literal::Array(elements)))
            },
            Some(TokenKind::LeftBrace) => {
                self.advance(); // consume {
                let mut pairs = Vec::new();

                while self.peek().map(|t| &t.kind) != Some(&TokenKind::RightBrace) {
                    let key = match self.advance() {
                        Some(token) if token.kind == TokenKind::StringLiteral => {
                            token.text.trim_matches('"').to_string()
                        },
                        _ => return Err(ParseError::UnexpectedToken(self.previous().unwrap())),
                    };

                    self.consume(&TokenKind::Colon)?;
                    let value = Box::new(self.parse_expression()?);
                    pairs.push((key, value));

                    if self.peek().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                        self.advance(); // consume comma
                    }
                }

                self.consume(&TokenKind::RightBrace)?;
                Ok(Expression::Literal(Literal::Object(pairs)))
            },
            Some(TokenKind::Null) => {
                self.advance();
                Ok(Expression::Literal(Literal::Null))
            },
            Some(TokenKind::Index) => {
                self.advance();
                self.consume(&TokenKind::LeftParen)?;
                let array = self.parse_expression()?;
                self.consume(&TokenKind::Comma)?;
                let index = self.parse_expression()?;
                self.consume(&TokenKind::RightParen)?;

                Ok(Expression::BinaryOp {
                    op: BinaryOp::Index,
                    left: Box::new(array),
                    right: Box::new(index),
                })
            },
            Some(TokenKind::Access) => {
                self.advance();
                self.consume(&TokenKind::LeftParen)?;
                let object = Box::new(self.parse_expression()?);
                self.consume(&TokenKind::Comma)?;
                let key = Box::new(self.parse_expression()?);
                self.consume(&TokenKind::RightParen)?;

                Ok(Expression::Access { object, key })
            },
            Some(TokenKind::Equals) => {
                self.advance();
                self.consume(&TokenKind::LeftParen)?;
                let left = self.parse_expression()?;
                self.consume(&TokenKind::Comma)?;
                let right = self.parse_expression()?;
                self.consume(&TokenKind::RightParen)?;

                Ok(Expression::BinaryOp {
                    op: BinaryOp::Equals,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            },
            Some(TokenKind::LessThan) => {
                self.advance();
                self.consume(&TokenKind::LeftParen)?;
                let left = self.parse_expression()?;
                self.consume(&TokenKind::Comma)?;
                let right = self.parse_expression()?;
                self.consume(&TokenKind::RightParen)?;

                Ok(Expression::BinaryOp {
                    op: BinaryOp::LessThan,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            },
            Some(TokenKind::Promise) => {
                self.advance();
                self.consume(&TokenKind::LeftParen)?;
                let value = Box::new(self.parse_expression()?);
                let timeout = if self.peek().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                    self.advance(); // consume comma
                    Some(Box::new(self.parse_expression()?))
                } else {
                    None
                };
                self.consume(&TokenKind::RightParen)?;

                Ok(Expression::Promise { value, timeout })
            },
            Some(TokenKind::Await) => {
                self.advance();
                self.consume(&TokenKind::LeftParen)?;
                let promise = Box::new(self.parse_expression()?);
                self.consume(&TokenKind::RightParen)?;

                Ok(Expression::Await { promise })
            },
            _ => Err(ParseError::UnexpectedToken(
                self.peek()
                    .cloned()
                    .unwrap_or_else(|| Token::new(TokenKind::Whitespace, String::new())),
            )),
        }
    }

    /// Parses a function call that might return null or go for coffee.
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

    /// Consumes a token if it matches the expected kind.
    /// Otherwise, returns an error that might make you question your life choices.
    fn consume(&mut self, expected: &TokenKind) -> Result<(), ParseError> {
        if self.peek().map(|t| &t.kind) == Some(expected) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(
                self.peek()
                    .cloned()
                    .unwrap_or_else(|| Token::new(TokenKind::Whitespace, String::new())),
            ))
        }
    }

    /// Checks if we've reached the end of input.
    /// One of the few functions that does exactly what it says.
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    /// Peeks at the next token without consuming it.
    /// What you see might not be what you get.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    /// Returns the previously consumed token.
    /// Useful for error messages that nobody will read.
    fn previous(&self) -> Option<Token> {
        if self.current > 0 {
            self.tokens.get(self.current - 1).cloned()
        } else {
            None
        }
    }

    /// Advances to the next token.
    /// One small step for the parser, one giant leap into confusion.
    fn advance(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Parses an if statement that always executes the else branch.
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

    /// Parses a loop statement that executes exactly once.
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

    /// Parses a function declaration
    fn parse_function(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'fn'
        let name = match self.advance() {
            Some(token) if token.kind == TokenKind::Identifier => token.text,
            _ => return Err(ParseError::UnexpectedToken(self.previous().unwrap())),
        };

        self.consume(&TokenKind::LeftParen)?;
        let mut parameters = Vec::new();
        while self.peek().map(|t| &t.kind) != Some(&TokenKind::RightParen) {
            match self.advance() {
                Some(token) if token.kind == TokenKind::Identifier => {
                    parameters.push(token.text);
                },
                _ => return Err(ParseError::UnexpectedToken(self.previous().unwrap())),
            }
            if self.peek().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                self.advance(); // consume comma
            }
        }
        self.consume(&TokenKind::RightParen)?;

        self.consume(&TokenKind::LeftBrace)?;
        let mut body = Vec::new();
        while self.peek().map(|t| &t.kind) != Some(&TokenKind::RightBrace) {
            body.push(self.parse_statement()?);
        }
        self.consume(&TokenKind::RightBrace)?;

        Ok(Statement::Function { name, parameters, body })
    }

    /// Parses a module declaration
    fn parse_module(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'mod'
        let name = match self.advance() {
            Some(token) if token.kind == TokenKind::Identifier => token.text,
            _ => return Err(ParseError::UnexpectedToken(self.previous().unwrap())),
        };

        self.consume(&TokenKind::LeftBrace)?;
        let mut body = Vec::new();
        while self.peek().map(|t| &t.kind) != Some(&TokenKind::RightBrace) {
            body.push(self.parse_statement()?);
        }
        self.consume(&TokenKind::RightBrace)?;

        Ok(Statement::Module { name, body })
    }

    /// Parses a use statement
    fn parse_use(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'use'
        let path = self.parse_use_path()?;
        self.consume(&TokenKind::Semicolon)?;
        Ok(Statement::Use { path })
    }

    /// Parses a use path (e.g., normal::mode or experimental::features)
    fn parse_use_path(&mut self) -> Result<String, ParseError> {
        let mut path = Vec::new();
        loop {
            match self.advance() {
                Some(token) if token.kind == TokenKind::Identifier => {
                    path.push(token.text);
                },
                _ => return Err(ParseError::UnexpectedToken(self.previous().unwrap())),
            }

            if self.peek().map(|t| &t.kind) != Some(&TokenKind::DoubleColon) {
                break;
            }
            self.advance(); // consume '::'
        }
        Ok(path.join("::"))
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
            Statement::Expression(Expression::BinaryOp {
                op,
                left: _,
                right: _,
            }) => {
                assert!(matches!(op, BinaryOp::Add));
            }
            _ => panic!("Expected binary operation"),
        }
    }
}
