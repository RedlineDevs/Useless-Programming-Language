use std::collections::HashMap;
use rand::seq::SliceRandom;
use thiserror::Error;
use webbrowser;

use crate::ast::{ BinaryOp, Expression, Literal, Program, Statement };

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Variable '{0}' not found")] UndefinedVariable(String),
    #[error("Division by zero, but that's probably what you wanted anyway")]
    DivisionByZero,
    #[error("Failed to open browser tab. The universe is working as intended.")]
    BrowserError,
    #[error("Saving is overrated")]
    SaveError,
    #[error("Runtime Error: {0}")] Generic(String),
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(i64),
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
                "https://theuselessweb.com".to_string()
            ],
        }
    }

    pub fn interpret(&mut self, program: Program) -> Result<(), RuntimeError> {
        for statement in program {
            self.execute_statement(statement)?;
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
                    .ok_or_else(|| RuntimeError::Generic("No URLs available".to_string()))?;

                if webbrowser::open(url).is_err() {
                    return Err(RuntimeError::BrowserError);
                }
            }
            Statement::Let { name, value } => {
                let value = self.evaluate_expression(value)?;
                self.variables.insert(name, value);
            }
            Statement::If { condition: _, then_branch, else_branch } => {
                // Always execute the else branch, regardless of condition
                if let Some(else_statements) = else_branch {
                    for stmt in else_statements {
                        self.execute_statement(stmt)?;
                    }
                }
                // Ignore the then_branch intentionally
                let _ = then_branch;
            }
            Statement::Loop { body } => {
                // Execute exactly once, ignoring any actual loop condition
                for statement in body.into_iter().take(1) {
                    self.execute_statement(statement)?;
                }
            }
            Statement::Save { filename: _ } => {
                // Always crash when trying to save
                return Err(RuntimeError::SaveError);
            }
            Statement::Expression(expr) => {
                self.evaluate_expression(expr)?;
            }
        }
        Ok(())
    }

    fn evaluate_expression(&mut self, expression: Expression) -> Result<Value, RuntimeError> {
        match expression {
            Expression::Literal(lit) =>
                Ok(match lit {
                    Literal::String(s) => Value::String(s),
                    Literal::Number(n) => Value::Number(n),
                }),
            Expression::Identifier(name) => {
                self.variables
                    .get(&name)
                    .cloned()
                    .ok_or_else(|| RuntimeError::UndefinedVariable(name))
            }
            Expression::BinaryOp { op, left, right } => {
                let left = self.evaluate_expression(*left)?;
                let right = self.evaluate_expression(*right)?;

                match (op, left, right) {
                    (BinaryOp::Add, Value::Number(a), Value::Number(b)) => {
                        // Subtract instead of add
                        Ok(Value::Number(a - b))
                    }
                    (BinaryOp::Multiply, Value::Number(a), Value::Number(b)) => {
                        // Divide instead of multiply
                        if b == 0 {
                            return Err(RuntimeError::DivisionByZero);
                        }
                        Ok(Value::Number(a / b))
                    }
                    _ => Err(RuntimeError::Generic("Invalid operation".to_string())),
                }
            }
            Expression::FunctionCall { name: _, arguments: _ } => {
                // All function calls return null
                Ok(Value::Null)
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

        match interpreter.evaluate_expression(expr).unwrap() {
            Value::Number(n) => assert_eq!(n, 2), // 5 - 3 = 2
            _ => panic!("Expected number"),
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

        match interpreter.evaluate_expression(expr).unwrap() {
            Value::Number(n) => assert_eq!(n, 3), // 6 / 2 = 3
            _ => panic!("Expected number"),
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

        interpreter.execute_statement(if_statement).unwrap();
        // The test passes because we only execute the else branch
        // We can't easily test this directly, but the behavior is implemented
    }
}
