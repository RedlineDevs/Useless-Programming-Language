use logos::Logos;
use std::fmt;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum TokenKind {
    #[token("print")]
    Print,

    #[token("let")]
    Let,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("loop")]
    Loop,

    #[token("return")]
    Return,

    #[token("add")]
    Add,

    #[token("multiply")]
    Multiply,

    #[token("save")]
    Save,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token("=")]
    Equals,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[regex(r#""[^"]*""#)]
    StringLiteral,

    #[regex(r"[0-9]+")]
    NumberLiteral,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r"//[^\n]*\n?", logos::skip)]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}

impl Token {
    pub fn new(kind: TokenKind, text: String) -> Self {
        Self { kind, text }
    }
}

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: TokenKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(Ok(kind)) => {
                let text = self.inner.slice().to_string();
                Some(Token::new(kind, text))
            }
            Some(Err(_)) => self.next(),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = "print(\"Hello, World!\");";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::Print, "print".to_string()),
                Token::new(TokenKind::LeftParen, "(".to_string()),
                Token::new(TokenKind::StringLiteral, "\"Hello, World!\"".to_string()),
                Token::new(TokenKind::RightParen, ")".to_string()),
                Token::new(TokenKind::Semicolon, ";".to_string())
            ]
        );
    }

    #[test]
    fn test_let_statement() {
        let input = "let x = 42;";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::Let, "let".to_string()),
                Token::new(TokenKind::Identifier, "x".to_string()),
                Token::new(TokenKind::Equals, "=".to_string()),
                Token::new(TokenKind::NumberLiteral, "42".to_string()),
                Token::new(TokenKind::Semicolon, ";".to_string())
            ]
        );
    }
}
