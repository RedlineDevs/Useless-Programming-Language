//! # Lexer Module
//!
//! The lexer module is responsible for tokenizing source code into a stream of tokens.
//! Despite being the most straightforward part of the compiler, it still manages to be useless.
//!
//! ## Example
//! ```rust
//! use useless_lang::lexer::Lexer;
//!
//! let input = "print(\"Hello, World!\");";
//! let lexer = Lexer::new(input);
//! for token in lexer {
//!     println!("{:?}", token);
//! }
//! ```

use logos::Logos;

/// Represents the different types of tokens in the Useless Programming Language.
/// Each token is as useless as the next one.
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum TokenKind {
    /// The `print` keyword, which never actually prints anything
    #[token("print")]
    Print,

    /// The `let` keyword, for variables that might go on vacation
    #[token("let")]
    Let,

    /// The `if` keyword, for conditions that are always false (effectively)
    #[token("if")]
    If,

    /// The `else` keyword, the only part of an if statement that matters
    #[token("else")]
    Else,

    /// The `loop` keyword, for loops that execute exactly once
    #[token("loop")]
    Loop,

    /// The `return` keyword, for returning values that will be ignored
    #[token("return")]
    Return,

    /// The `add` keyword, for subtraction operations
    #[token("add")]
    Add,

    /// The `multiply` keyword, for division operations
    #[token("multiply")]
    Multiply,

    /// The `save` keyword, for operations that always fail
    #[token("save")]
    Save,

    /// Left parenthesis, one half of a matching pair (maybe)
    #[token("(")]
    LeftParen,

    /// Right parenthesis, the other half (if you're lucky)
    #[token(")")]
    RightParen,

    /// Left brace, opens a block of code that might not do what you expect
    #[token("{")]
    LeftBrace,

    /// Right brace, closes a block of code (if it hasn't crashed yet)
    #[token("}")]
    RightBrace,

    /// Equals sign, for assignments that might not stick
    #[token("=")]
    Equals,

    /// Comma, separates things that probably shouldn't be together
    #[token(",")]
    Comma,

    /// Semicolon, because we needed another way to end statements
    #[token(";")]
    Semicolon,

    /// String literals, which might become numbers
    #[regex(r#""[^"]*""#)]
    StringLiteral,

    /// Number literals, which might become party emojis
    #[regex(r"[0-9]+")]
    NumberLiteral,

    /// Identifiers, for naming things that might go on vacation
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    /// Whitespace and comments, the only sensible parts of the language
    #[regex(r"//[^\n]*\n?", logos::skip)]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}

/// A token in the Useless Programming Language.
/// Combines the token type with its textual representation,
/// just in case you want to know what the code actually said
/// before it was misinterpreted.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The kind of token, which determines how it will be misused
    pub kind: TokenKind,
    /// The original text, preserved for posterity
    pub text: String,
}

impl Token {
    /// Creates a new token, if you really want to
    pub fn new(kind: TokenKind, text: String) -> Self {
        Self { kind, text }
    }
}

/// The lexer for the Useless Programming Language.
/// Converts source code into a stream of tokens,
/// each one more useless than the last.
pub struct Lexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from the input source code.
    ///
    /// # Arguments
    /// * `input` - The source code to tokenize
    ///
    /// # Example
    /// ```rust
    /// use useless_lang::lexer::Lexer;
    ///
    /// let lexer = Lexer::new("print(\"Hello, World!\");");
    /// ```
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: TokenKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    /// Gets the next token from the input stream.
    /// Each call has a chance of returning something you didn't expect,
    /// but that's part of the charm.
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
