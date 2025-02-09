//! # Abstract Syntax Tree Module
//!
//! This module defines the Abstract Syntax Tree (AST) for the Useless Programming Language.
//! The AST represents the structure of the program in a way that's almost, but not quite,
//! entirely unlike what the programmer intended.
//!
//! ## Example
//! ```rust
//! use useless_lang::ast::{Statement, Expression, Literal};
//!
//! // Create a print statement that will open a random website
//! let stmt = Statement::Print {
//!     value: Expression::Literal(Literal::String("Hello, World!".to_string()))
//! };
//! ```

/// Represents literal values in the language.
/// These values might not stay in their original form for long.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// A string literal, which might become a number
    String(String),
    /// A number literal, which might become a string of party emojis
    Number(i64),
    /// A boolean literal, which might become a string of party emojis
    Boolean(bool),
    /// An array literal, which might randomly shuffle or lose elements
    Array(Vec<Box<Expression>>),
    /// An object literal, which might swap keys or values
    Object(Vec<(String, Box<Expression>)>),
    /// Null, which might become not null
    Null,
}

/// Binary operators that do the opposite of what you'd expect.
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    /// Subtracts when you want to add
    Add,
    /// Divides when you want to multiply
    Multiply,
    /// Array access that might return random element
    Index,
    /// Object access that might return wrong field
    Access,
    /// Equality that might be random
    Equals,
    /// Less than that might be greater than
    LessThan,
}

/// Expressions that may or may not evaluate to what you expect.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// A literal value (for now)
    Literal(Literal),
    /// A variable name (if it hasn't gone on vacation)
    Identifier(String),
    /// A binary operation that does the opposite of what you want
    BinaryOp {
        /// The operator to misuse
        op: BinaryOp,
        /// The left-hand side of the operation
        left: Box<Expression>,
        /// The right-hand side of the operation
        right: Box<Expression>,
    },
    /// A function call that might return null or go for coffee
    FunctionCall {
        /// The name of the function to not call properly
        name: String,
        /// Arguments that might be ignored
        #[allow(dead_code)]
        arguments: Vec<Expression>,
    },
    /// Array or object access that might return random elements
    Access {
        /// The array or object to access
        object: Box<Expression>,
        /// The key or index to access
        key: Box<Expression>,
    },
    /// Promise that might resolve at random times
    Promise {
        /// The expression to evaluate
        value: Box<Expression>,
        /// Optional timeout in milliseconds
        timeout: Option<Box<Expression>>,
    },
    /// Async/await expressions that might delay randomly
    Await {
        /// The promise to await
        promise: Box<Expression>,
    },
}

/// Statements that make up a Useless program.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Variable declaration (results may vary)
    Let {
        /// The name of the variable (until it goes on vacation)
        name: String,
        /// The value to assign (maybe)
        value: Expression,
    },
    /// Print statement that opens random websites
    Print {
        /// The value to not print
        value: Expression,
    },
    /// If statement that always chooses the else branch
    If {
        /// The condition to ignore
        #[allow(dead_code)]
        condition: Expression,
        /// The branch that will never execute
        then_branch: Vec<Statement>,
        /// The branch that will always execute
        else_branch: Option<Vec<Statement>>,
    },
    /// Loop that executes exactly once
    Loop {
        /// The body to execute exactly once
        body: Vec<Statement>,
    },
    /// Save operation that always crashes
    Save {
        /// The filename to not save to
        #[allow(dead_code)]
        filename: Expression,
    },
    /// A standalone expression
    Expression(Expression),
    /// Async function declaration
    AsyncFunction {
        /// The name of the function
        name: String,
        /// Parameters of the function
        parameters: Vec<String>,
        /// The body of the function
        body: Vec<Statement>,
    },
    /// Try-catch that might catch the wrong error
    TryCatch {
        /// The body to try
        try_block: Vec<Statement>,
        /// The error variable name
        error_var: String,
        /// The catch block
        catch_block: Vec<Statement>,
    },
    /// Await statement for async operations
    Await {
        /// The promise to await
        expression: Expression,
    },
}

/// A complete Useless program, ready to misbehave.
pub type Program = Vec<Statement>;
