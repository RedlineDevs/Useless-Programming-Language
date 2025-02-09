use useless_lang::{
    ast::{Expression, Literal, Statement, BinaryOp},
    interpreter::Interpreter,
};

#[test]
fn test_array_operations() {
    let mut interpreter = Interpreter::new();

    // Create an array
    let array_expr = Expression::Literal(Literal::Array(vec![
        Box::new(Expression::Literal(Literal::Number(1))),
        Box::new(Expression::Literal(Literal::Number(2))),
        Box::new(Expression::Literal(Literal::Number(3))),
    ]));

    // Store array in variable
    let store_array = Statement::Let {
        name: "test_array".to_string(),
        value: array_expr,
    };

    // Try to access array (might get random element or vacation error)
    let access_array = Expression::Access {
        object: Box::new(Expression::Identifier("test_array".to_string())),
        key: Box::new(Expression::Literal(Literal::Number(1))),
    };

    // Execute and verify results are appropriately chaotic
    interpreter.execute_statement(store_array).unwrap_or_else(|e| {
        println!("Array storage failed successfully: {}", e);
    });

    match interpreter.evaluate_expression(access_array) {
        Ok(value) => println!("Array access succeeded (suspiciously): {:?}", value),
        Err(e) => println!("Array access failed successfully: {}", e),
    }
}

#[test]
fn test_async_operations() {
    let mut interpreter = Interpreter::new();

    // Create an async function
    let async_fn = Statement::AsyncFunction {
        name: "test_async".to_string(),
        parameters: vec!["x".to_string()],
        body: vec![
            Statement::Expression(Expression::Literal(Literal::String("async test".to_string()))),
        ],
    };

    // Create a promise
    let promise_expr = Expression::Promise {
        value: Box::new(Expression::Literal(Literal::String("promise test".to_string()))),
        timeout: Some(Box::new(Expression::Literal(Literal::Number(1000)))),
    };

    // Try to execute async function
    interpreter.execute_statement(async_fn).unwrap_or_else(|e| {
        println!("Async function declaration failed successfully: {}", e);
    });

    // Try to resolve promise
    match interpreter.evaluate_expression(promise_expr) {
        Ok(value) => println!("Promise resolved (suspiciously): {:?}", value),
        Err(e) => println!("Promise rejected successfully: {}", e),
    }
}

#[test]
fn test_error_handling() {
    let mut interpreter = Interpreter::new();

    // Create a try-catch block
    let try_catch = Statement::TryCatch {
        try_block: vec![
            Statement::Expression(Expression::BinaryOp {
                op: BinaryOp::Multiply,
                left: Box::new(Expression::Literal(Literal::Number(10))),
                right: Box::new(Expression::Literal(Literal::Number(0))),
            }),
        ],
        error_var: "error".to_string(),
        catch_block: vec![
            Statement::Print {
                value: Expression::Identifier("error".to_string()),
            },
        ],
    };

    // Execute and verify error handling is appropriately chaotic
    interpreter.execute_statement(try_catch).unwrap_or_else(|e| {
        println!("Error handling failed successfully: {}", e);
    });
}

#[test]
fn test_binary_operations() {
    let mut interpreter = Interpreter::new();

    // Test Index operation
    let array = Expression::Literal(Literal::Array(vec![
        Box::new(Expression::Literal(Literal::Number(1))),
        Box::new(Expression::Literal(Literal::Number(2))),
    ]));
    let index_op = Expression::BinaryOp {
        op: BinaryOp::Index,
        left: Box::new(array),
        right: Box::new(Expression::Literal(Literal::Number(0))),
    };

    // Test Access operation
    let object = Expression::Literal(Literal::Object(vec![
        ("test".to_string(), Box::new(Expression::Literal(Literal::Number(42))))
    ]));
    let access_op = Expression::BinaryOp {
        op: BinaryOp::Access,
        left: Box::new(object),
        right: Box::new(Expression::Literal(Literal::String("test".to_string()))),
    };

    // Test Equals operation
    let equals_op = Expression::BinaryOp {
        op: BinaryOp::Equals,
        left: Box::new(Expression::Literal(Literal::Number(1))),
        right: Box::new(Expression::Literal(Literal::Number(1))),
    };

    // Test LessThan operation
    let less_than_op = Expression::BinaryOp {
        op: BinaryOp::LessThan,
        left: Box::new(Expression::Literal(Literal::Number(1))),
        right: Box::new(Expression::Literal(Literal::Number(2))),
    };

    // All operations should either succeed chaotically or fail spectacularly
    for op in [index_op, access_op, equals_op, less_than_op] {
        match interpreter.evaluate_expression(op) {
            Ok(_) => (), // Any result is fine
            Err(_) => (), // Errors are also fine
        }
    }
}

#[test]
fn test_async_features() {
    let mut interpreter = Interpreter::new();

    // Test Promise
    let promise = Expression::Promise {
        value: Box::new(Expression::Literal(Literal::String("test".to_string()))),
        timeout: Some(Box::new(Expression::Literal(Literal::Number(1000)))),
    };

    // Test Await
    let await_expr = Expression::Await {
        promise: Box::new(promise),
    };

    // Test AsyncFunction
    let async_fn = Statement::AsyncFunction {
        name: "test_async".to_string(),
        parameters: vec!["x".to_string()],
        body: vec![Statement::Expression(await_expr.clone())],
    };

    // Test TryCatch
    let try_catch = Statement::TryCatch {
        try_block: vec![Statement::Expression(await_expr)],
        error_var: "error".to_string(),
        catch_block: vec![],
    };

    // Execute async function
    match interpreter.execute_statement(async_fn) {
        Ok(_) => (), // Success is suspicious
        Err(_) => (), // Errors are expected
    }

    // Execute try-catch
    match interpreter.execute_statement(try_catch) {
        Ok(_) => (), // Success is suspicious
        Err(_) => (), // Errors are expected
    }
}

#[test]
fn test_null_literal() {
    let mut interpreter = Interpreter::new();
    let null_expr = Expression::Literal(Literal::Null);

    match interpreter.evaluate_expression(null_expr) {
        Ok(_) => (), // Null might be anything
        Err(_) => (), // Or it might error
    }
}
