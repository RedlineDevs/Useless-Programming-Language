use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
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

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: Token::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(Ok(token)) => Some(token),
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
                Token::Print,
                Token::LeftParen,
                Token::StringLiteral,
                Token::RightParen,
                Token::Semicolon
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
                Token::Let,
                Token::Identifier,
                Token::Equals,
                Token::NumberLiteral,
                Token::Semicolon
            ]
        );
    }
}
