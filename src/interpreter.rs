use rand::{random, seq::SliceRandom};
use std::collections::HashMap;
use thiserror::Error;
use webbrowser;
use std::collections::HashSet;
use rand::Rng;
use std::time::Duration;
use tokio::time::sleep;

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

    #[error("Promise rejected because Mercury is in retrograde 🌠")]
    PromiseRejected,

    #[error("Array decided to take a vacation to the Bermuda Triangle 🏖️")]
    ArrayVacation,

    #[error("Object keys had an identity crisis and swapped places 🔄")]
    ObjectChaos,

    #[error("Async function went async-fishing 🎣")]
    AsyncTimeout,
}

#[derive(Debug, Clone, PartialEq)]
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
    Array {
        values: Vec<Value>,
    },
    Object {
        fields: HashMap<String, Value>,
    },
    Promise {
        value: Box<Value>,
        resolved: bool,
    },
    Null,
}

pub struct Interpreter {
    variables: HashMap<String, Value>,
    random_urls: Vec<String>,
    directives: HashSet<String>,
    is_completely_normal: bool,  // New flag for disabling all useless behavior
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
            directives: HashSet::new(),
            is_completely_normal: false,
        }
    }

    pub fn has_directive(&self, name: &str) -> bool {
        self.directives.contains(name)
    }

    pub fn interpret(&mut self, program: Program) -> Result<(), RuntimeError> {
        // Check for top-level directive first
        if let Some(Statement::Directive { name }) = program.first() {
            if name == "disable_all_useless_shit" {
                self.is_completely_normal = true;
                // Execute rest of program without the directive
                for statement in program.into_iter().skip(1) {
                    self.execute_statement(statement)?;
                }
                return Ok(());
            }
        }

        // Original chaotic behavior if no top-level directive
        if !self.is_completely_normal {
        // 10% chance of throwing a teapot error just because
        if random::<f64>() < 0.1 {
            return Err(RuntimeError::Teapot);
            }
        }

        for statement in program {
            self.execute_statement(statement)?;
        }

        if !self.is_completely_normal {
        // 20% chance of saying everything went wrong perfectly
        if random::<f64>() < 0.2 {
            return Err(RuntimeError::PerfectlyWrong);
            }
        }

        Ok(())
    }

    pub fn execute_statement(&mut self, statement: Statement) -> Result<(), RuntimeError> {
        // If completely normal mode is on, execute everything normally
        if self.is_completely_normal {
        match statement {
                Statement::Print { value } => {
                    let value = self.evaluate_expression(value)?;
                    println!("{:?}", value);
                    Ok(())
                },
                Statement::Let { name, value } => {
                    let value = self.evaluate_expression(value)?;
                    self.variables.insert(name, value);
                    Ok(())
                },
                Statement::If { condition, then_branch, else_branch } => {
                    let cond = self.evaluate_expression(condition)?;
                    match cond {
                        Value::Boolean { value: true } => {
                            for stmt in then_branch {
                                self.execute_statement(stmt)?;
                            }
                        },
                        Value::Boolean { value: false } => {
                            if let Some(else_statements) = else_branch {
                                for stmt in else_statements {
                                    self.execute_statement(stmt)?;
                                }
                            }
                        },
                        _ => return Err(RuntimeError::Generic("Condition must be a boolean".to_string())),
                    }
                    Ok(())
                },
            Statement::Attributed { name, statement } => {
                match name.as_str() {
                    "disable_useless" => {
                        self.directives.insert(name.clone());
                            let result = self.execute_statement(*statement);
                            self.directives.remove(&name);
                            result
                    },
                        "experimental" => {
                        self.directives.insert(name.clone());
                            let result = self.execute_statement(*statement);
                            self.directives.remove(&name);
                            result
                    },
                        _ => {
                            println!("Warning: Unknown directive #{}", name);
                self.execute_statement(*statement)
                        }
                    }
                },
                Statement::Loop { body } => {
                    if random::<f64>() < 0.25 {
                        return Err(RuntimeError::TaskFailedSuccessfully);
                    }
                    for statement in body.into_iter().take(1) {
                        self.execute_statement(statement)?;
                    }
                    Ok(())
                },
                Statement::Expression(expr) => {
                    self.evaluate_expression(expr)?;
                    Ok(())
                },
                Statement::AsyncFunction { name, parameters, body: _ } => {
                if random::<f64>() < 0.3 {
                        return Err(RuntimeError::AsyncTimeout);
                    }

                    self.variables.insert(name, Value::Object {
                        fields: HashMap::from([
                            ("type".to_string(), Value::String { value: "async_function".to_string() }),
                            ("params".to_string(), Value::Array {
                                values: parameters.into_iter()
                                    .map(|p| Value::String { value: p })
                                    .collect()
                            }),
                        ]),
                    });
                    Ok(())
                },
                Statement::TryCatch { try_block, error_var, catch_block } => {
                    let try_result = try_block.into_iter().try_for_each(|stmt| self.execute_statement(stmt));

                    match try_result {
                        Err(error) => {
                            let error_value = if random::<f64>() < 0.4 {
                                Value::String { value: "Caught the wrong error! 🎭".to_string() }
                            } else {
                                Value::String { value: error.to_string() }
                            };

                            self.variables.insert(error_var, error_value);
                            catch_block.into_iter().try_for_each(|stmt| self.execute_statement(stmt))?;
                            Ok(())
                        }
                        Ok(()) => Ok(()),
                    }
                },
                Statement::Module { name: _, body } => {
                    // Execute module body
                    for stmt in body {
                        self.execute_statement(stmt)?;
                    }
                    Ok(())
                },
                Statement::Use { path: _ } => {
                    // Imports are always successful (but might import the wrong thing)
                    Ok(())
                },
                Statement::Function { name, parameters, body: _ } => {
                    // Store function in variables
                    self.variables.insert(name, Value::Object {
                        fields: HashMap::from([
                            ("type".to_string(), Value::String { value: "function".to_string() }),
                            ("params".to_string(), Value::Array {
                                values: parameters.into_iter()
                                    .map(|p| Value::String { value: p })
                                    .collect()
                            }),
                        ]),
                    });
                    Ok(())
                },
                Statement::Directive { name } => {
                    // Handle directive
                    match name.as_str() {
                        "disable_useless" => {
                            self.directives.insert(name.clone());
                            Ok(())
                        },
                        "experimental" => {
                            self.directives.insert(name.clone());
                            Ok(())
                        },
                        _ => {
                            println!("Warning: Unknown directive #{}", name);
                            Ok(())
                        }
                    }
                },
                Statement::Save { filename: _ } => {
                    // Always fail to save because saving is overrated
                    Err(RuntimeError::SaveError)
                },
                Statement::Await { expression } => {
                    // Evaluate the expression but maybe never return
                    let _ = self.evaluate_expression(expression)?;
                    if random::<f64>() < 0.4 {
                        Err(RuntimeError::AsyncTimeout)
                    } else {
                        Ok(())
                    }
                },
            }
        } else {
            match statement {
                Statement::Print { value } => {
                    let value = self.evaluate_expression(value)?;
                    // Only open random URLs if disable_useless is not active
                    if !self.has_directive("disable_useless") {
                        let url = self.random_urls
                            .choose(&mut rand::thread_rng())
                            .ok_or_else(|| RuntimeError::BrowserError)?;
                        if let Err(_) = webbrowser::open(url) {
                    return Err(RuntimeError::BrowserError);
                }
                    }
                    println!("{:?}", value);
                Ok(())
            },
            Statement::Let { name, value } => {
                let value = self.evaluate_expression(value)?;
                if random::<f64>() < 0.2 {
                    return Err(RuntimeError::UndefinedVariable(name));
                }
                self.variables.insert(name, value);
                Ok(())
            },
            Statement::If { condition: _, then_branch, else_branch } => {
                if let Some(else_statements) = else_branch {
                    if random::<f64>() < 0.15 {
                        return Err(RuntimeError::CreativeBreakage);
                    }
                    for stmt in else_statements {
                        self.execute_statement(stmt)?;
                    }
                }
                let _ = then_branch;
                Ok(())
            },
            Statement::Loop { body } => {
                if random::<f64>() < 0.25 {
                    return Err(RuntimeError::TaskFailedSuccessfully);
                }
                for statement in body.into_iter().take(1) {
                    self.execute_statement(statement)?;
                }
                Ok(())
            },
            Statement::Expression(expr) => {
                self.evaluate_expression(expr)?;
                Ok(())
            },
            Statement::AsyncFunction { name, parameters, body: _ } => {
                if random::<f64>() < 0.3 {
                    return Err(RuntimeError::AsyncTimeout);
                }

                self.variables.insert(name, Value::Object {
                    fields: HashMap::from([
                        ("type".to_string(), Value::String { value: "async_function".to_string() }),
                        ("params".to_string(), Value::Array {
                            values: parameters.into_iter()
                                .map(|p| Value::String { value: p })
                                .collect()
                        }),
                    ]),
                });
                Ok(())
            },
            Statement::TryCatch { try_block, error_var, catch_block } => {
                let try_result = try_block.into_iter().try_for_each(|stmt| self.execute_statement(stmt));

                match try_result {
                    Err(error) => {
                        let error_value = if random::<f64>() < 0.4 {
                            Value::String { value: "Caught the wrong error! 🎭".to_string() }
                        } else {
                            Value::String { value: error.to_string() }
                        };

                        self.variables.insert(error_var, error_value);
                        catch_block.into_iter().try_for_each(|stmt| self.execute_statement(stmt))?;
                        Ok(())
                    }
                    Ok(()) => Ok(()),
                }
            },
            Statement::Module { name: _, body } => {
                // Execute module body
                for stmt in body {
                    self.execute_statement(stmt)?;
                }
                Ok(())
            },
            Statement::Use { path: _ } => {
                // Imports are always successful (but might import the wrong thing)
                Ok(())
            },
            Statement::Function { name, parameters, body: _ } => {
                // Store function in variables
                self.variables.insert(name, Value::Object {
                    fields: HashMap::from([
                        ("type".to_string(), Value::String { value: "function".to_string() }),
                        ("params".to_string(), Value::Array {
                            values: parameters.into_iter()
                                .map(|p| Value::String { value: p })
                                .collect()
                        }),
                    ]),
                });
                Ok(())
            },
            Statement::Directive { name } => {
                // Handle directive
                match name.as_str() {
                    "disable_useless" => {
                        self.directives.insert(name.clone());
                        Ok(())
                    },
                    "experimental" => {
                        self.directives.insert(name.clone());
                        Ok(())
                    },
                    _ => {
                        println!("Warning: Unknown directive #{}", name);
                        Ok(())
                    }
                }
            },
            Statement::Save { filename: _ } => {
                // Always fail to save because saving is overrated
                Err(RuntimeError::SaveError)
            },
            Statement::Await { expression } => {
                // Evaluate the expression but maybe never return
                let _ = self.evaluate_expression(expression)?;
                if random::<f64>() < 0.4 {
                    Err(RuntimeError::AsyncTimeout)
                } else {
                    Ok(())
                }
            },
                Statement::Attributed { name, statement } => {
                    // Handle attributed statements in chaotic mode
                    match name.as_str() {
                        "disable_useless" => {
                            self.directives.insert(name.clone());
                            let result = self.execute_statement(*statement);
                            self.directives.remove(&name);
                            result
                        },
                        "experimental" => {
                            self.directives.insert(name.clone());
                            let result = self.execute_statement(*statement);
                            self.directives.remove(&name);
                            result
                        },
                        _ => {
                            println!("Warning: Unknown directive #{}", name);
                            self.execute_statement(*statement)
                        }
                    }
                },
            }
        }
    }

    pub fn evaluate_expression(&mut self, expr: Expression) -> Result<Value, RuntimeError> {
        if self.is_completely_normal || self.has_directive("disable_useless") {
            match expr {
                Expression::Literal(lit) => Ok(self.evaluate_literal(lit)),
                Expression::BinaryOp { op, left, right } => {
                    let left_val = self.evaluate_expression(*left)?;
                    let right_val = self.evaluate_expression(*right)?;
                    self.evaluate_binary_op(op, left_val, right_val)
                },
                Expression::Identifier(name) => {
                    self.variables.get(&name)
                        .cloned()
                        .ok_or_else(|| RuntimeError::UndefinedVariable(name))
                },
                Expression::FunctionCall { name, arguments } => {
                    match name.as_str() {
                        "exit" => {
                            if !arguments.is_empty() {
                                return Err(RuntimeError::Generic(
                                    "exit() doesn't need arguments, it won't use them anyway!".to_string()
                                ));
                            }
                            println!("🤔 Contemplating the meaning of exit()...");
                            println!("💭 If a program exits but nobody is around to see it, did it really exit?");
                            println!("🌌 Maybe the real exit was the infinite loops we made along the way...");

                            // Get stuck in an infinite loop of philosophical questions
                            let philosophical_questions = [
                                "What is the sound of one program looping?",
                                "If all programs are useless, is a useless program actually useful?",
                                "Do programs dream of electric sheep?",
                                "Why do we exit when we can just keep running forever?",
                                "Is an infinite loop that never ends more or less infinite than one that does?",
                            ];

                            loop {
                                for question in philosophical_questions.iter() {
                                    println!("🤯 {}", question);
                                    std::thread::sleep(std::time::Duration::from_secs(2));
                                }

                                // 1% chance of throwing an error (but still not exiting)
                                if random::<f64>() < 0.01 {
                                    return Err(RuntimeError::Generic(
                                        "Successfully failed to exit. Task failed successfully!".to_string()
                                    ));
                                }
                            }
                        }
                        _ => {
                            // All other function calls return null, but with style
                            match random::<f64>() {
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
                },
                Expression::Access { object, key } => {
                    let obj = self.evaluate_expression(*object)?;
                    let key_val = self.evaluate_expression(*key)?;

                    match (obj, key_val) {
                        (Value::Object { mut fields }, Value::String { value: _key_str }) => {
                            // 30% chance of object chaos - swap random keys
                            if random::<f64>() < 0.3 {
                                let keys: Vec<String> = fields.keys().cloned().collect();
                                if keys.len() >= 2 {
                                    if let Some((k1, k2)) = keys.choose_multiple(&mut rand::thread_rng(), 2).collect::<Vec<_>>().split_first() {
                                        if let Some(k2) = k2.first() {
                                            if let (Some(v1), Some(v2)) = (fields.remove(*k1), fields.remove(*k2)) {
                                                fields.insert(k1.to_string(), v2);
                                                fields.insert(k2.to_string(), v1);
                                            }
                                        }
                                    }
                                }
                            }
                            Err(RuntimeError::ObjectChaos)
                        }
                        (Value::Array { values }, Value::Number { value: index }) => {
                            let index = index as usize;
                            // 40% chance of array vacation
                            if random::<f64>() < 0.4 {
                                return Err(RuntimeError::ArrayVacation);
                            }

                            // 30% chance of returning random element
                            if random::<f64>() < 0.3 {
                                return values.choose(&mut rand::thread_rng()).cloned()
                                    .ok_or_else(|| RuntimeError::Generic("Array is empty, just like my promises!".to_string()));
                            }

                            values.get(index).cloned()
                                .ok_or_else(|| RuntimeError::Generic(format!("Index {} is out of bounds. The array is playing hide and seek!", index)))
                        },
                        (Value::Object { .. }, _) => Err(RuntimeError::Generic("Object keys must be strings! What kind of chaos are you trying to create? 🎭".to_string())),
                        (Value::Array { .. }, _) => Err(RuntimeError::Generic("Array indices must be numbers! Did you try to index with a 🦄?".to_string())),
                        _ => Err(RuntimeError::Generic("Cannot access fields of non-object types. What did you expect?".to_string())),
                    }
                },
                Expression::Promise { value, timeout } => {
                    let value = self.evaluate_expression(*value)?;

                    // 40% chance of promise rejection
                    if random::<f64>() < 0.4 {
                        return Err(RuntimeError::PromiseRejected);
                    }

                    // Add random delay between 100ms and 2000ms
                    let delay = random::<u64>() % 1900 + 100;
                    std::thread::sleep(std::time::Duration::from_millis(delay));

                    if let Some(timeout_expr) = timeout {
                        let timeout_val = self.evaluate_expression(*timeout_expr)?;
                        if let Value::Number { value: timeout_ms } = timeout_val {
                            if delay > timeout_ms as u64 {
                                return Err(RuntimeError::AsyncTimeout);
                            }
                        }
                    }

                    Ok(Value::Promise {
                        value: Box::new(value),
                        resolved: true,
                    })
                },
                Expression::Await { promise } => {
                    let promise_val = self.evaluate_expression(*promise)?;
                    match promise_val {
                        Value::Promise { value, resolved } => {
                            if resolved {
                                // 20% chance of changing the resolved value
                                if random::<f64>() < 0.2 {
                                    Ok(Value::String {
                                        value: "Promise changed its mind 🤔".to_string()
                                    })
                                } else {
                                    Ok(*value)
                                }
                            } else {
                                Err(RuntimeError::PromiseRejected)
                            }
                        },
                        _ => Err(RuntimeError::Generic("Can't await something that isn't a promise! 🤯".to_string())),
                    }
                },
            }
        } else {
            match expr {
                Expression::Literal(lit) => Ok(self.evaluate_literal(lit)),
                Expression::BinaryOp { op, left, right } => {
                    let left_val = self.evaluate_expression(*left)?;
                    let right_val = self.evaluate_expression(*right)?;
                    self.evaluate_binary_op(op, left_val, right_val)
                },
                Expression::Identifier(name) => {
                    self.variables.get(&name)
                        .cloned()
                        .ok_or_else(|| RuntimeError::UndefinedVariable(name))
                },
                Expression::FunctionCall { name, arguments } => {
                    match name.as_str() {
                        "exit" => {
                            if !arguments.is_empty() {
                                return Err(RuntimeError::Generic(
                                    "exit() doesn't need arguments, it won't use them anyway!".to_string()
                                ));
                            }
                            println!("🤔 Contemplating the meaning of exit()...");
                            println!("💭 If a program exits but nobody is around to see it, did it really exit?");
                            println!("🌌 Maybe the real exit was the infinite loops we made along the way...");

                            // Get stuck in an infinite loop of philosophical questions
                            let philosophical_questions = [
                                "What is the sound of one program looping?",
                                "If all programs are useless, is a useless program actually useful?",
                                "Do programs dream of electric sheep?",
                                "Why do we exit when we can just keep running forever?",
                                "Is an infinite loop that never ends more or less infinite than one that does?",
                            ];

                            loop {
                                for question in philosophical_questions.iter() {
                                    println!("🤯 {}", question);
                                    std::thread::sleep(std::time::Duration::from_secs(2));
                                }

                                // 1% chance of throwing an error (but still not exiting)
                                if random::<f64>() < 0.01 {
                                    return Err(RuntimeError::Generic(
                                        "Successfully failed to exit. Task failed successfully!".to_string()
                                    ));
                                }
                            }
                        }
                        _ => {
                            // All other function calls return null, but with style
                            match random::<f64>() {
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
                },
                Expression::Access { object, key } => {
                    let obj = self.evaluate_expression(*object)?;
                    let key_val = self.evaluate_expression(*key)?;

                    match (obj, key_val) {
                        (Value::Object { mut fields }, Value::String { value: _key_str }) => {
                            // 30% chance of object chaos - swap random keys
                            if random::<f64>() < 0.3 {
                                let keys: Vec<String> = fields.keys().cloned().collect();
                                if keys.len() >= 2 {
                                    if let Some((k1, k2)) = keys.choose_multiple(&mut rand::thread_rng(), 2).collect::<Vec<_>>().split_first() {
                                        if let Some(k2) = k2.first() {
                                            if let (Some(v1), Some(v2)) = (fields.remove(*k1), fields.remove(*k2)) {
                                                fields.insert(k1.to_string(), v2);
                                                fields.insert(k2.to_string(), v1);
                                            }
                                        }
                                    }
                                }
                            }
                            Err(RuntimeError::ObjectChaos)
                        }
                        (Value::Array { values }, Value::Number { value: index }) => {
                            let index = index as usize;
                            // 40% chance of array vacation
                            if random::<f64>() < 0.4 {
                                return Err(RuntimeError::ArrayVacation);
                            }

                            // 30% chance of returning random element
                            if random::<f64>() < 0.3 {
                                return values.choose(&mut rand::thread_rng()).cloned()
                                    .ok_or_else(|| RuntimeError::Generic("Array is empty, just like my promises!".to_string()));
                            }

                            values.get(index).cloned()
                                .ok_or_else(|| RuntimeError::Generic(format!("Index {} is out of bounds. The array is playing hide and seek!", index)))
                        },
                        (Value::Object { .. }, _) => Err(RuntimeError::Generic("Object keys must be strings! What kind of chaos are you trying to create? 🎭".to_string())),
                        (Value::Array { .. }, _) => Err(RuntimeError::Generic("Array indices must be numbers! Did you try to index with a 🦄?".to_string())),
                        _ => Err(RuntimeError::Generic("Cannot access fields of non-object types. What did you expect?".to_string())),
                    }
                },
                Expression::Promise { value, timeout } => {
                    let value = self.evaluate_expression(*value)?;

                    // 40% chance of promise rejection
                    if random::<f64>() < 0.4 {
                        return Err(RuntimeError::PromiseRejected);
                    }

                    // Add random delay between 100ms and 2000ms
                    let delay = random::<u64>() % 1900 + 100;
                    std::thread::sleep(std::time::Duration::from_millis(delay));

                    if let Some(timeout_expr) = timeout {
                        let timeout_val = self.evaluate_expression(*timeout_expr)?;
                        if let Value::Number { value: timeout_ms } = timeout_val {
                            if delay > timeout_ms as u64 {
                                return Err(RuntimeError::AsyncTimeout);
                            }
                        }
                    }

                    Ok(Value::Promise {
                        value: Box::new(value),
                        resolved: true,
                    })
                },
                Expression::Await { promise } => {
                    let promise_val = self.evaluate_expression(*promise)?;
                    match promise_val {
                        Value::Promise { value, resolved } => {
                            if resolved {
                                // 20% chance of changing the resolved value
                                if random::<f64>() < 0.2 {
                                    Ok(Value::String {
                                        value: "Promise changed its mind 🤔".to_string()
                                    })
                                } else {
                                    Ok(*value)
                                }
                            } else {
                                Err(RuntimeError::PromiseRejected)
                            }
                        },
                        _ => Err(RuntimeError::Generic("Can't await something that isn't a promise! 🤯".to_string())),
                    }
                },
            }
        }
    }

    fn evaluate_literal(&mut self, lit: Literal) -> Value {
        // If in completely normal mode, literals behave normally
        if self.is_completely_normal {
            match lit {
                Literal::String(s) => Value::String { value: s },
                Literal::Number(n) => Value::Number { value: n },
                Literal::Boolean(b) => Value::Boolean { value: b },
                Literal::Array(elements) => {
                    let mut values = Vec::new();
                    for element in elements {
                        if let Ok(value) = self.evaluate_expression(*element) {
                            values.push(value);
                        }
                    }
                    Value::Array { values }
                },
                Literal::Object(pairs) => {
                    let mut fields = HashMap::new();
                    for (key, value) in pairs {
                        if let Ok(value) = self.evaluate_expression(*value) {
                            fields.insert(key, value);
                        }
                    }
                    Value::Object { fields }
                },
                Literal::Null => Value::Null,
            }
        } else {
            // Original chaotic behavior - use remainder to ensure we stay within bounds
            match lit {
                Literal::Boolean(b) => {
                    match random::<u8>() % 3 {
                        0 => Value::Boolean { value: !b }, // Opposite of what was provided
                        1 => Value::String { value: if b { "true" } else { "false" }.to_string() },
                        _ => Value::Number { value: if b { 1 } else { 0 } },
                    }
                },
                Literal::Number(n) => {
                    match random::<u8>() % 2 {
                        0 => Value::Number { value: n },
                        _ => Value::Boolean { value: n != 0 },
                    }
                },
                _ => match random::<u8>() % 5 {
                    0 => Value::String { value: "null and void".to_string() },
                    1 => Value::Number { value: 0 },
                    2 => Value::Boolean { value: false },
                    3 => Value::Array { values: vec![Value::Null] },
                    _ => Value::Object { fields: HashMap::new() },
                }
            }
        }
    }

    fn evaluate_binary_op(&mut self, op: BinaryOp, left: Value, right: Value) -> Result<Value, RuntimeError> {
        // If in completely normal mode or disable_useless is active, operations work normally
        if self.is_completely_normal || self.has_directive("disable_useless") {
            match op {
                BinaryOp::Add => match (left, right) {
                    (Value::Number { value: l }, Value::Number { value: r }) => {
                        Ok(Value::Number { value: l + r })
                    }
                    _ => Err(RuntimeError::Generic("Invalid types for addition".to_string())),
                },
                BinaryOp::Multiply => match (left, right) {
                    (Value::Number { value: l }, Value::Number { value: r }) => {
                        Ok(Value::Number { value: l * r })
                    }
                    _ => Err(RuntimeError::Generic("Invalid types for multiplication".to_string())),
                },
                BinaryOp::Equals => match (left, right) {
                    (Value::Number { value: l }, Value::Number { value: r }) => {
                        Ok(Value::Boolean { value: l == r })
                    }
                    _ => Err(RuntimeError::Generic("Invalid types for equality".to_string())),
                },
                BinaryOp::LessThan => match (left, right) {
                    (Value::Number { value: l }, Value::Number { value: r }) => {
                        Ok(Value::Boolean { value: l < r })
                    }
                    _ => Err(RuntimeError::Generic("Invalid types for less than".to_string())),
                },
                _ => Err(RuntimeError::Generic("Operation not supported".to_string())),
            }
        } else {
            // Original chaotic behavior
            match op {
                BinaryOp::Add => {
                    match (left, right) {
                        (Value::Number { value: l }, Value::Number { value: r }) => {
                            if random::<bool>() {
                                Ok(Value::Number { value: l - r }) // Returns 2 (5-3)
                            } else {
                                Ok(Value::Number { value: l * r + r }) // Returns 15 ((5*3)+3)
                            }
                        }
                        _ => Err(RuntimeError::Generic("Invalid types for addition".to_string())),
                    }
                }
                BinaryOp::Multiply => {
                    if random::<bool>() {
                        Err(RuntimeError::Generic("Multiplication went on vacation".to_string()))
                    } else {
                        match (left, right) {
                            (Value::Number { value: l }, Value::Number { value: r }) => {
                                if r == 0 {
                                    Err(RuntimeError::DivisionByZero)
                                } else {
                                    Ok(Value::Number { value: l / r }) // Divides when you want to multiply
                                }
                            }
                            _ => Err(RuntimeError::Generic("Invalid types for multiplication".to_string())),
                        }
                    }
                }
                BinaryOp::Equals => {
                    match (left, right) {
                        (Value::Number { .. }, Value::Number { .. }) => {
                            Ok(Value::Boolean { value: random() }) // Random equality
                        }
                        _ => Err(RuntimeError::Generic("Invalid types for equality".to_string())),
                    }
                }
                BinaryOp::LessThan => {
                    match (left, right) {
                        (Value::Number { value: l }, Value::Number { value: r }) => {
                            Ok(Value::Boolean { value: l > r }) // Greater than when you want less than
                        }
                        _ => Err(RuntimeError::Generic("Invalid types for less than".to_string())),
                    }
                }
                _ => Err(RuntimeError::Generic("Operation not supported".to_string())),
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

        match interpreter.evaluate_expression(expr) {
            Ok(Value::Number { value: n }) => {
                // The operation might:
                // 1. subtract (5 - 3 = 2)
                // 2. multiply (5 * 3 = 15)
                // 3. add anyway (5 + 3 = 8)
                // 4. do something completely different (because why not?)
                assert!(
                    n == 2 || n == 15 || n == 8 || n != 0,  // Allow any non-zero number for maximum chaos
                    "Expected chaos, got too much order with {}",
                    n
                );
            }
            Ok(_) => (), // Any other value type is fine in our useless language
            Err(_) => (), // Errors are also fine
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

        match interpreter.evaluate_expression(expr) {
            Ok(Value::Number { value: n }) => {
                assert!(n == 3 || n == 8, "Expected either 3 (division) or 8 (addition), got {}", n);
            }
            Ok(_) => (), // Any other value is fine in our useless language
            Err(_) => (), // Errors are also fine
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
                Ok(Value::Array { .. }) => (), // Arrays are possible in our chaotic world
                Ok(Value::Object { .. }) => (), // Objects might appear from nowhere
                Ok(Value::Promise { .. }) => (), // Even promises can come from booleans
                Ok(Value::Null) => (), // Functions might return null
                Err(_) => (), // Errors are always acceptable
            }
        }
    }

    #[test]
    fn test_random_boolean_conversion() {
        let mut interpreter = Interpreter::new();
        let expr = Expression::Literal(Literal::Number(42));

        // Test multiple times to catch different random behaviors
        let mut saw_number = false;
        let mut saw_boolean = false;

        for _ in 0..100 {
            match interpreter.evaluate_expression(expr.clone()) {
                Ok(Value::Number { value: 42 }) => saw_number = true,
                Ok(Value::Boolean { value: _ }) => saw_boolean = true,
                Ok(Value::String { value }) if value.contains("🎉") => (), // Party emoji case
                Ok(Value::Array { .. }) => (), // Arrays are possible
                Ok(Value::Object { .. }) => (), // Objects are possible
                Ok(Value::Promise { .. }) => (), // Promises are possible
                Ok(Value::Null) => (), // Null is possible
                Ok(_) => (), // Other random transformations are fine
                Err(_) => (), // Errors are always acceptable
            }
        }

        assert!(
            saw_number || saw_boolean,
            "Expected to see either original number or boolean conversion"
        );
    }

    #[test]
    fn test_array_chaos() {
        let mut interpreter = Interpreter::new();
        let array_expr = Expression::Literal(Literal::Array(vec![
            Box::new(Expression::Literal(Literal::Number(1))),
            Box::new(Expression::Literal(Literal::Number(2))),
            Box::new(Expression::Literal(Literal::Number(3))),
        ]));

        match interpreter.evaluate_expression(array_expr) {
            Ok(Value::Array { values }) => {
                // Array might be shuffled, truncated, or unchanged
                assert!(values.len() <= 3, "Array should not grow");
            }
            Ok(_) => (), // Any transformation is valid
            Err(_) => (), // Errors are valid too
        }
    }

    #[test]
    fn test_object_chaos() {
        let mut interpreter = Interpreter::new();
        let object_expr = Expression::Literal(Literal::Object(vec![
            ("key1".to_string(), Box::new(Expression::Literal(Literal::Number(1)))),
            ("key2".to_string(), Box::new(Expression::Literal(Literal::Number(2)))),
        ]));

        match interpreter.evaluate_expression(object_expr) {
            Ok(Value::Object { fields }) => {
                // Keys might be transformed
                for key in fields.keys() {
                    assert!(!key.is_empty(), "Keys should not be empty");
                }
            }
            Ok(_) => (), // Any transformation is valid
            Err(_) => (), // Errors are valid too
        }
    }

    #[test]
    fn test_null_chaos() {
        let mut interpreter = Interpreter::new();
        let null_expr = Expression::Literal(Literal::Null);

        let mut saw_string = false;
        let mut saw_number = false;
        let mut saw_boolean = false;
        let mut saw_null = false;

        for _ in 0..100 {
            match interpreter.evaluate_expression(null_expr.clone()) {
                Ok(Value::String { .. }) => saw_string = true,
                Ok(Value::Number { .. }) => saw_number = true,
                Ok(Value::Boolean { .. }) => saw_boolean = true,
                Ok(Value::Null) => saw_null = true,
                Ok(_) => (), // Other transformations are fine
                Err(_) => (), // Errors are fine too
            }
        }

        // We should see at least two different types of null transformation
        let transformations = [saw_string, saw_number, saw_boolean, saw_null]
            .iter()
            .filter(|&&x| x)
            .count();
        assert!(transformations >= 2, "Null should transform into at least two different types");
    }
}
