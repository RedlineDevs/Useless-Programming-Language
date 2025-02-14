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
    /// Print statement that might print something else
    Print {
        /// The value to print (maybe)
        value: Expression,
    },
    /// Let statement for variables that might go on vacation
    Let {
        /// The name of the variable
        name: String,
        /// The value to assign (for now)
        value: Expression,
    },
    /// Expression statement for when you just want chaos
    Expression(Expression),
    /// If statement that always executes the else branch
    If {
        /// The condition that will be ignored
        condition: Expression,
        /// The then branch that won't be executed
        then_branch: Vec<Statement>,
        /// The else branch that will always be executed
        else_branch: Option<Vec<Statement>>,
    },
    /// Loop statement that executes exactly once
    Loop {
        /// The body of the loop
        body: Vec<Statement>,
    },
    /// Function declaration that might not work
    Function {
        /// The name of the function
        name: String,
        /// The parameters that might be ignored
        parameters: Vec<String>,
        /// The body that might not execute
        body: Vec<Statement>,
    },
    /// Async function that might never resolve
    AsyncFunction {
        /// The name of the function
        name: String,
        /// The parameters that might be ignored
        parameters: Vec<String>,
        /// The body that might not execute
        body: Vec<Statement>,
    },
    /// Try-catch block that might catch the wrong error
    TryCatch {
        /// The try block that might fail
        try_block: Vec<Statement>,
        /// The error variable name
        error_var: String,
        /// The catch block that might catch the wrong error
        catch_block: Vec<Statement>,
    },
    /// Module declaration for organizing chaos
    Module {
        /// The name of the module
        name: String,
        /// The module body
        body: Vec<Statement>,
    },
    /// Use statement for importing more chaos
    Use {
        /// The path to import
        path: String,
    },
    /// Directive for controlling language behavior
    Directive {
        /// The name of the directive
        name: String,
    },
    /// Save statement for persisting chaos
    Save {
        /// The filename to save to
        filename: String,
    },
    /// Await expression for asynchronous chaos
    Await {
        /// The expression to await
        expression: Expression,
    },
    /// Attributed statement for directives
    Attributed {
        /// The name of the directive
        name: String,
        /// The statement being attributed
        statement: Box<Statement>,
    },
}

/// A complete Useless program, ready to misbehave.
pub type Program = Vec<Statement>;
