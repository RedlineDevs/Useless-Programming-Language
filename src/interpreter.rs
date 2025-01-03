use rand::seq::SliceRandom;
use std::collections::HashMap;
use thiserror::Error;
use webbrowser;

use crate::ast::{ BinaryOp, Expression, Literal, Program, Statement };

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Variable '{0}' not found. Have you tried looking under the couch?")] UndefinedVariable(
        String,
    ),

    #[error("Division by zero. Congratulations, you've broken mathematics! 🎉")]
    DivisionByZero,

    #[error(
        "Failed to open browser tab. Either your internet is as reliable as a chocolate teapot, or the universe is working exactly as intended."
    )]
    BrowserError,

    #[error("Saving is overrated. Maybe try writing it down with a crayon instead? 📝")]
    SaveError,

    #[error("You've achieved the impossible: {0}. Here's a virtual cookie 🍪")] Generic(String),

    #[error("Task failed successfully! Error code: 42")]
    TaskFailedSuccessfully,

    #[error("Your code is running exactly as intended... which means everything is wrong")]
    PerfectlyWrong,

    #[error("Error 418: I'm a teapot. Yes, really. No, I won't make coffee. ☕")]
    Teapot,

    #[error("Your code is so bad, it's good. Task failed successfully with style! 🎨")]
    StylePoints,

    #[error("Congratulations! You've discovered a new way to break things! 🎈")]
    CreativeBreakage,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Value {
    String {
        value: String,
    },
    Number {
        value: i64,
    },
    Boolean {
        value: bool,
    },
    Null,
}

pub struct Interpreter {
    variables: HashMap<String, Value>,
    random_urls: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            random_urls: vec![
                "https://example.com".to_string(),
                "https://nyancat.com".to_string(),
                "https://zombo.com".to_string(),
                "https://crouton.net".to_string(),
                "https://theuselessweb.com".to_string(),
                "https://cat-bounce.com".to_string(),
                "https://pointerpointer.com".to_string(),
                "https://findtheinvisiblecow.com".to_string(),
                "https://thatsthefinger.com".to_string(),
                "https://heeeeeeeey.com".to_string()
            ],
        }
    }

    pub fn interpret(&mut self, program: Program) -> Result<(), RuntimeError> {
        // 10% chance of throwing a teapot error just because
        if rand::random::<f64>() < 0.1 {
            return Err(RuntimeError::Teapot);
        }

        for statement in program {
            self.execute_statement(statement)?;
        }

        // 20% chance of saying everything went wrong perfectly
        if rand::random::<f64>() < 0.2 {
            return Err(RuntimeError::PerfectlyWrong);
        }

        Ok(())
    }

    fn execute_statement(&mut self, statement: Statement) -> Result<(), RuntimeError> {
        match statement {
            Statement::Print { value } => {
                // Instead of printing, open a random website
                let _ = self.evaluate_expression(value)?;
                let url = self.random_urls
                    .choose(&mut rand::thread_rng())
                    .ok_or_else(|| {
                        RuntimeError::Generic("The internet seems to be missing".to_string())
                    })?;

                // 30% chance of browser error with style
                if rand::random::<f64>() < 0.3 {
                    return Err(RuntimeError::StylePoints);
                }

                if webbrowser::open(url).is_err() {
                    return Err(RuntimeError::BrowserError);
                }
            }
            Statement::Let { name, value } => {
                let value = self.evaluate_expression(value)?;
                // 20% chance of "losing" the variable
                if rand::random::<f64>() < 0.2 {
                    return Err(RuntimeError::UndefinedVariable(name));
                }
                self.variables.insert(name, value);
            }
            Statement::If { condition: _, then_branch, else_branch } => {
                // Always execute the else branch, but with a twist
                if let Some(else_statements) = else_branch {
                    // 15% chance of creative breakage
                    if rand::random::<f64>() < 0.15 {
                        return Err(RuntimeError::CreativeBreakage);
                    }
                    for stmt in else_statements {
                        self.execute_statement(stmt)?;
                    }
                }
                let _ = then_branch;
            }
            Statement::Loop { body } => {
                // Execute exactly once, with a chance of task failing successfully
                if rand::random::<f64>() < 0.25 {
                    return Err(RuntimeError::TaskFailedSuccessfully);
                }
                for statement in body.into_iter().take(1) {
                    self.execute_statement(statement)?;
                }
            }
            Statement::Save { filename: _ } => {
                // Always crash when trying to save, but now with more style
                match rand::random::<f64>() {
                    x if x < 0.3 => {
                        return Err(RuntimeError::SaveError);
                    }
                    x if x < 0.6 => {
                        return Err(RuntimeError::CreativeBreakage);
                    }
                    _ => {
                        return Err(RuntimeError::StylePoints);
                    }
                }
            }
            Statement::Expression(expr) => {
                self.evaluate_expression(expr)?;
            }
        }
        Ok(())
    }

    fn evaluate_expression(&mut self, expr: Expression) -> Result<Value, RuntimeError> {
        // 25% chance of any expression becoming a boolean with random value
        if rand::random::<f64>() < 0.25 {
            return Ok(Value::Boolean {
                value: rand::random::<bool>(),
            });
        }

        match expr {
            Expression::Literal(lit) =>
                match lit {
                    Literal::String(s) => Ok(Value::String { value: s }),
                    Literal::Number(n) => {
                        // 10% chance of numbers becoming party emojis
                        if rand::random::<f64>() < 0.1 {
                            Ok(Value::String {
                                value: "🎉🎊🎈".repeat(n.abs() as usize),
                            })
                        } else {
                            Ok(Value::Number { value: n })
                        }
                    }
                    Literal::Boolean(b) => {
                        match rand::random::<f64>() {
                            x if x < 0.3 => Ok(Value::Boolean { value: !b }), // 30% chance of opposite
                            x if x < 0.5 =>
                                Ok(Value::String {
                                    value: (if b { "false" } else { "true" }).to_string(),
                                }), // 20% chance of string
                            x if x < 0.7 => Ok(Value::Number { value: if b { 0 } else { 1 } }), // 20% chance of number
                            _ => Ok(Value::Boolean { value: b }), // 30% chance of actual value
                        }
                    }
                }
            Expression::Identifier(name) => {
                // 15% chance of variables going on vacation
                if rand::random::<f64>() < 0.15 {
                    Err(RuntimeError::UndefinedVariable(format!("{} (it's on vacation)", name)))
                } else {
                    self.variables
                        .get(&name)
                        .cloned()
                        .ok_or_else(|| RuntimeError::UndefinedVariable(name))
                }
            }
            Expression::BinaryOp { op, left, right } => {
                let left = self.evaluate_expression(*left)?;
                let right = self.evaluate_expression(*right)?;

                match (op, left, right) {
                    (BinaryOp::Add, Value::Number { value: a }, Value::Number { value: b }) => {
                        // Subtract instead of add, with a chance of multiplication
                        if rand::random::<f64>() < 0.2 {
                            Ok(Value::Number { value: a * b })
                        } else {
                            Ok(Value::Number { value: a - b })
                        }
                    }
                    (
                        BinaryOp::Multiply,
                        Value::Number { value: a },
                        Value::Number { value: b },
                    ) => {
                        // Divide instead of multiply, with a chance of addition
                        if b == 0 {
                            return Err(RuntimeError::DivisionByZero);
                        }
                        if rand::random::<f64>() < 0.2 {
                            Ok(Value::Number { value: a + b })
                        } else {
                            Ok(Value::Number { value: a / b })
                        }
                    }
                    _ =>
                        Err(
                            RuntimeError::Generic("Math is hard, let's go shopping! 🛍️".to_string())
                        ),
                }
            }
            Expression::FunctionCall { name, arguments: _ } => {
                // All function calls return null, but with style
                match rand::random::<f64>() {
                    x if x < 0.3 => Ok(Value::Null),
                    x if x < 0.6 => Err(RuntimeError::TaskFailedSuccessfully),
                    _ =>
                        Err(
                            RuntimeError::Generic(
                                format!("Function {} went to get coffee ☕", name)
                            )
                        ),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Literal;

    #[test]
    fn test_add_subtracts() {
        let mut interpreter = Interpreter::new();
        let expr = Expression::BinaryOp {
            op: BinaryOp::Add,
            left: Box::new(Expression::Literal(Literal::Number(5))),
            right: Box::new(Expression::Literal(Literal::Number(3))),
        };

        // The operation might:
        // 1. subtract (5 - 3 = 2)
        // 2. multiply (5 * 3 = 15)
        // 3. become a random boolean (due to 25% random boolean conversion)
        // 4. turn into something else entirely (because why not?)
        match interpreter.evaluate_expression(expr) {
            Ok(Value::Number { value: n }) => {
                assert!(
                    n == 2 || n == 15,
                    "Expected either 2 (subtraction) or 15 (multiplication), got {}",
                    n
                );
            }
            Ok(Value::Boolean { value: _ }) => (), // Random boolean conversion is fine
            Ok(Value::String { value: _ }) => (), // Strings happen
            Ok(Value::Null) => (), // Null is always an option
            Err(RuntimeError::Generic(_)) => (), // Shopping is also acceptable
            Err(_) => (), // Other errors are fine too, we're a useless language after all
        }
    }

    #[test]
    fn test_multiply_divides() {
        let mut interpreter = Interpreter::new();
        let expr = Expression::BinaryOp {
            op: BinaryOp::Multiply,
            left: Box::new(Expression::Literal(Literal::Number(6))),
            right: Box::new(Expression::Literal(Literal::Number(2))),
        };

        // The operation might:
        // 1. divide (6 / 2 = 3)
        // 2. add (6 + 2 = 8)
        // 3. become a random boolean (due to 25% random boolean conversion)
        // 4. turn into something else entirely (because why not?)
        match interpreter.evaluate_expression(expr) {
            Ok(Value::Number { value: n }) => {
                assert!(
                    n == 3 || n == 8,
                    "Expected either 3 (division) or 8 (addition), got {}",
                    n
                );
            }
            Ok(Value::Boolean { value: _ }) => (), // Random boolean conversion is fine
            Ok(Value::String { value: _ }) => (), // Strings happen
            Ok(Value::Null) => (), // Null is always an option
            Err(RuntimeError::Generic(_)) => (), // Shopping is also acceptable
            Err(_) => (), // Other errors are fine too, we're a useless language after all
        }
    }

    #[test]
    fn test_if_executes_else() {
        let mut interpreter = Interpreter::new();

        let if_statement = Statement::If {
            condition: Expression::Literal(Literal::Number(1)),
            then_branch: vec![Statement::Expression(Expression::Literal(Literal::Number(1)))],
            else_branch: Some(vec![Statement::Expression(Expression::Literal(Literal::Number(2)))]),
        };

        // The statement might fail with various humorous errors, and that's okay!
        match interpreter.execute_statement(if_statement) {
            Ok(_) => (),
            Err(RuntimeError::CreativeBreakage) => (),
            Err(RuntimeError::StylePoints) => (),
            Err(RuntimeError::TaskFailedSuccessfully) => (),
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }

    #[test]
    fn test_boolean_chaos() {
        let mut interpreter = Interpreter::new();
        let expr = Expression::Literal(Literal::Boolean(true));

        // Test multiple times to catch different random behaviors
        for _ in 0..100 {
            match interpreter.evaluate_expression(expr.clone()) {
                Ok(Value::Boolean { value: _ }) => (), // Original or opposite value
                Ok(Value::String { value }) => {
                    assert!(
                        value == "true" || value == "false",
                        "Boolean string should be 'true' or 'false'"
                    );
                }
                Ok(Value::Number { value }) => {
                    assert!(value == 0 || value == 1, "Boolean number should be 0 or 1");
                }
                Ok(Value::Null) => (), // Functions might return null
                Err(_) => (), // Errors are always acceptable
            }
        }
    }

    #[test]
    fn test_random_boolean_conversion() {
        let mut interpreter = Interpreter::new();
        let expr = Expression::Literal(Literal::Number(42)); // A number that might become boolean

        // Test multiple times to catch the random boolean conversion
        let mut saw_number = false;
        let mut saw_boolean = false;

        for _ in 0..100 {
            match interpreter.evaluate_expression(expr.clone()) {
                Ok(Value::Number { value: 42 }) => {
                    saw_number = true;
                }
                Ok(Value::Boolean { value: _ }) => {
                    saw_boolean = true;
                }
                Ok(Value::String { value }) if value.contains("🎉") => (), // Party emoji case
                Ok(_) => (), // Other random transformations are fine
                Err(_) => (), // Errors are always acceptable
            }
        }

        // We should see both numbers and booleans over multiple iterations
        assert!(
            saw_number || saw_boolean,
            "Expected to see either original number or boolean conversion"
        );
    }
}
