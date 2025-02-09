pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;

// Re-export main types for easier access
pub use ast::{Expression, Literal, Statement, BinaryOp, Program};
pub use interpreter::{Interpreter, Value, RuntimeError};
pub use lexer::{Lexer, Token, TokenKind};
pub use parser::{Parser, ParseError};
