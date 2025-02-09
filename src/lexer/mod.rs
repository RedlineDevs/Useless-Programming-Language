//! # Lexer Module
//!
//! The lexer module is responsible for breaking down source code into tokens.
//! It's like a shredder, but for code instead of documents.
//!
//! ## Example
//! ```rust
//! use useless_lang::lexer::{Lexer, Token};
//!
//! let input = "print(\"Hello, World!\");";
//! let lexer = Lexer::new(input);
//! let tokens: Vec<Token> = lexer.collect();
//! ```

use logos::Logos;

/// All the different kinds of tokens in our language.
/// Each one is special in its own useless way.
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum TokenKind {
    /// The print keyword, which opens random websites
    #[token("print")]
    Print,

    /// The let keyword, for variables that might go on vacation
    #[token("let")]
    Let,

    /// The if keyword, for conditions that always choose the else branch
    #[token("if")]
    If,

    /// The else keyword, because that's where we're going anyway
    #[token("else")]
    Else,

    /// The loop keyword, for code that runs exactly once
    #[token("loop")]
    Loop,

    /// The save keyword, which crashes the program
    #[token("save")]
    Save,

    /// The add function, which actually subtracts
    #[token("add")]
    Add,

    /// The multiply function, which actually divides
    #[token("multiply")]
    Multiply,

    /// Left parenthesis, the beginning of confusion
    #[token("(")]
    LeftParen,

    /// Right parenthesis, the end of confusion
    #[token(")")]
    RightParen,

    /// Left brace, where dreams begin
    #[token("{")]
    LeftBrace,

    /// Right brace, where dreams end
    #[token("}")]
    RightBrace,

    /// Semicolon, because we need more punctuation
    #[token(";")]
    Semicolon,

    /// Equals sign, for assignments that might not stick
    #[token("=")]
    Assignment,

    /// Comma, the separator of things that shouldn't be together
    #[token(",")]
    Comma,

    /// String literals, which might contain anything but what you wrote
    #[regex("\"[^\"]*\"")]
    StringLiteral,

    /// Number literals, which might not be the number you expect
    #[regex("[0-9]+")]
    NumberLiteral,

    /// Boolean literals, which might not be what you expect
    #[token("true")]
    True,

    #[token("false")]
    False,

    /// Identifiers, for naming things that won't behave
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    /// Whitespace and comments, the only predictable parts of the language
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    /// Comments, where you can write what you hope the code will do
    #[regex(r"//[^\n]*\n?", logos::skip)]
    Comment,

    /// Exit keyword
    #[token("exit")]
    Exit,

    /// Left bracket for arrays that might lose elements
    #[token("[")]
    LeftBracket,

    /// Right bracket for arrays that might have lost elements
    #[token("]")]
    RightBracket,

    /// Colon for object key-value pairs that might swap
    #[token(":")]
    Colon,

    /// Null keyword for values that might not be null
    #[token("null")]
    Null,

    /// Index operation that might return random elements
    #[token("index")]
    Index,

    /// Access operation that might return wrong fields
    #[token("access")]
    Access,

    /// Equals operation that might be random
    #[token("equals")]
    Equals,

    /// Less than operation that might be greater than
    #[token("lessThan")]
    LessThan,

    /// Promise keyword for operations that might never resolve
    #[token("promise")]
    Promise,

    /// Await keyword for promises that might change their mind
    #[token("await")]
    Await,

    /// Async keyword for functions that might go fishing
    #[token("async")]
    Async,

    /// Try keyword for blocks that might catch the wrong error
    #[token("try")]
    Try,

    /// Catch keyword for errors that might not have happened
    #[token("catch")]
    Catch,
}

/// A token in our language, consisting of its kind and the text it was parsed from.
/// The text might not match what you see in the source code.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The kind of token this is
    pub kind: TokenKind,
    /// The text that was parsed into this token
    pub text: String,
}

impl Token {
    /// Creates a new token with the given kind and text.
    /// Use sparingly, as tokens have a mind of their own.
    pub fn new(kind: TokenKind, text: String) -> Self {
        Self { kind, text }
    }
}

/// The lexer for our language.
/// It breaks down your code into tokens, whether you like it or not.
pub struct Lexer<'a> {
    /// The underlying logos lexer
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from the given input string.
    /// What comes out might not be what went in.
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: TokenKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    /// Gets the next token from the input.
    /// Returns None when there are no more tokens, or when the lexer gets bored.
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(Ok(kind)) => Some(Token::new(kind, self.inner.slice().to_string())),
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
                Token::new(TokenKind::Assignment, "=".to_string()),
                Token::new(TokenKind::NumberLiteral, "42".to_string()),
                Token::new(TokenKind::Semicolon, ";".to_string())
            ]
        );
    }

    #[test]
    fn test_array_and_object_literals() {
        let input = "[1, 2, 3] {\"key\": 42}";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::LeftBracket, "[".to_string()),
                Token::new(TokenKind::NumberLiteral, "1".to_string()),
                Token::new(TokenKind::Comma, ",".to_string()),
                Token::new(TokenKind::NumberLiteral, "2".to_string()),
                Token::new(TokenKind::Comma, ",".to_string()),
                Token::new(TokenKind::NumberLiteral, "3".to_string()),
                Token::new(TokenKind::RightBracket, "]".to_string()),
                Token::new(TokenKind::LeftBrace, "{".to_string()),
                Token::new(TokenKind::StringLiteral, "\"key\"".to_string()),
                Token::new(TokenKind::Colon, ":".to_string()),
                Token::new(TokenKind::NumberLiteral, "42".to_string()),
                Token::new(TokenKind::RightBrace, "}".to_string()),
            ]
        );
    }

    #[test]
    fn test_binary_operations() {
        let input = "index(arr, 0) access(obj, \"key\") equals(1, 1) lessThan(1, 2)";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::Index, "index".to_string()),
                Token::new(TokenKind::LeftParen, "(".to_string()),
                Token::new(TokenKind::Identifier, "arr".to_string()),
                Token::new(TokenKind::Comma, ",".to_string()),
                Token::new(TokenKind::NumberLiteral, "0".to_string()),
                Token::new(TokenKind::RightParen, ")".to_string()),
                Token::new(TokenKind::Access, "access".to_string()),
                Token::new(TokenKind::LeftParen, "(".to_string()),
                Token::new(TokenKind::Identifier, "obj".to_string()),
                Token::new(TokenKind::Comma, ",".to_string()),
                Token::new(TokenKind::StringLiteral, "\"key\"".to_string()),
                Token::new(TokenKind::RightParen, ")".to_string()),
                Token::new(TokenKind::Equals, "equals".to_string()),
                Token::new(TokenKind::LeftParen, "(".to_string()),
                Token::new(TokenKind::NumberLiteral, "1".to_string()),
                Token::new(TokenKind::Comma, ",".to_string()),
                Token::new(TokenKind::NumberLiteral, "1".to_string()),
                Token::new(TokenKind::RightParen, ")".to_string()),
                Token::new(TokenKind::LessThan, "lessThan".to_string()),
                Token::new(TokenKind::LeftParen, "(".to_string()),
                Token::new(TokenKind::NumberLiteral, "1".to_string()),
                Token::new(TokenKind::Comma, ",".to_string()),
                Token::new(TokenKind::NumberLiteral, "2".to_string()),
                Token::new(TokenKind::RightParen, ")".to_string()),
            ]
        );
    }

    #[test]
    fn test_async_features() {
        let input = "async fn() { await promise(42, 1000); }";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::Async, "async".to_string()),
                Token::new(TokenKind::Identifier, "fn".to_string()),
                Token::new(TokenKind::LeftParen, "(".to_string()),
                Token::new(TokenKind::RightParen, ")".to_string()),
                Token::new(TokenKind::LeftBrace, "{".to_string()),
                Token::new(TokenKind::Await, "await".to_string()),
                Token::new(TokenKind::Promise, "promise".to_string()),
                Token::new(TokenKind::LeftParen, "(".to_string()),
                Token::new(TokenKind::NumberLiteral, "42".to_string()),
                Token::new(TokenKind::Comma, ",".to_string()),
                Token::new(TokenKind::NumberLiteral, "1000".to_string()),
                Token::new(TokenKind::RightParen, ")".to_string()),
                Token::new(TokenKind::Semicolon, ";".to_string()),
                Token::new(TokenKind::RightBrace, "}".to_string()),
            ]
        );
    }

    #[test]
    fn test_error_handling() {
        let input = "try { null; } catch err { print(err); }";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::Try, "try".to_string()),
                Token::new(TokenKind::LeftBrace, "{".to_string()),
                Token::new(TokenKind::Null, "null".to_string()),
                Token::new(TokenKind::Semicolon, ";".to_string()),
                Token::new(TokenKind::RightBrace, "}".to_string()),
                Token::new(TokenKind::Catch, "catch".to_string()),
                Token::new(TokenKind::Identifier, "err".to_string()),
                Token::new(TokenKind::LeftBrace, "{".to_string()),
                Token::new(TokenKind::Print, "print".to_string()),
                Token::new(TokenKind::LeftParen, "(".to_string()),
                Token::new(TokenKind::Identifier, "err".to_string()),
                Token::new(TokenKind::RightParen, ")".to_string()),
                Token::new(TokenKind::Semicolon, ";".to_string()),
                Token::new(TokenKind::RightBrace, "}".to_string()),
            ]
        );
    }
}
