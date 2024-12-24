//! # Lexer Module
//!
//! The lexer module is responsible for breaking down source code into tokens.
//! It's like a shredder, but for code instead of documents.
//!
//! ## Example
//! ```rust
//! use useless_lang::lexer::Lexer;
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
    Equals,

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
                Token::new(TokenKind::Equals, "=".to_string()),
                Token::new(TokenKind::NumberLiteral, "42".to_string()),
                Token::new(TokenKind::Semicolon, ";".to_string())
            ]
        );
    }
}
