use std::fs;
use useless_lang::{
    ast::{Program, Statement},
    interpreter::Interpreter,
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_async_chaos_example() {
    // Read the example file
    let source = fs::read_to_string("examples/async_chaos.upl")
        .expect("Failed to read async_chaos.upl");

    // Parse the code
    let lexer = Lexer::new(&source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let program: Program = parser.parse().expect("Failed to parse program");

    // Execute the program
    let mut interpreter = Interpreter::new();
    for statement in program {
        match interpreter.execute_statement(statement) {
            Ok(_) => println!("Statement executed successfully (suspiciously)"),
            Err(e) => println!("Statement failed successfully: {}", e),
        }
    }
}

#[test]
fn test_directives() {
    let source = r#"
// First test without disable_useless - operations should be chaotic
let x = 42;
let y = 10;
let sum1 = add(x, y);
let product1 = multiply(x, y);

// Now enable disable_useless - operations should work normally
#[directive(disable_useless)]
let a = 42;
let b = 10;
let sum2 = add(a, b);     // Should be 52
let product2 = multiply(a, b);  // Should be 420

// Verify the results are correct
if equals(sum2, 52) {
    print("disable_useless directive working - addition is correct");
}
if equals(product2, 420) {
    print("disable_useless directive working - multiplication is correct");
}

// Test experimental features
#[directive(experimental)]
async calculate_slowly(data) {
    try {
        let result = promise(data, 1000);
        await result;
        print("Experimental async feature working");
        return result;
    } catch error {
        print(error);
        return 0;
    }
}

let test_result = calculate_slowly(42);
if test_result {
    print("Experimental directive working - async function succeeded");
}
"#;

    // Parse the code
    let lexer = Lexer::new(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let program: Program = parser.parse().expect("Failed to parse program");

    // Execute the program and verify directive behavior
    let mut interpreter = Interpreter::new();
    let mut saw_normal_math = false;
    let mut saw_experimental = false;

    for statement in program {
        match interpreter.execute_statement(statement) {
            Ok(_) => {
                // Check if we're seeing evidence of directives working
                if interpreter.has_directive("disable_useless") {
                    saw_normal_math = true;
                }
                if interpreter.has_directive("experimental") {
                    saw_experimental = true;
                }
            },
            Err(e) => {
                // Only accept errors outside directive blocks
                if interpreter.has_directive("disable_useless") || interpreter.has_directive("experimental") {
                    panic!("Operation failed inside directive block: {}", e);
                } else {
                    println!("Expected chaos outside directive block: {}", e);
                }
            }
        }
    }

    // Verify that we saw evidence of both directives working
    assert!(saw_normal_math, "disable_useless directive did not work properly");
    assert!(saw_experimental, "experimental directive did not work properly");
}

#[test]
fn test_rust_style_attributes() {
    let source = r#"
// Variables for testing
let x = 42;
let y = 10;

// Test built-in operations
let isEqual = equals(x, y);
let isLess = lessThan(x, y);

// Async operation
async processData(data) {
    try {
        let result = promise(data, 1000);
        await result;
        print("Data processed successfully!");
    } catch error {
        print(error);
    }
}

// Use the async function
processData(x);
"#;

    // Parse the code
    let lexer = Lexer::new(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let program: Program = parser.parse().expect("Failed to parse program");

    // Execute the program and expect useless errors
    let mut interpreter = Interpreter::new();
    for statement in program {
        match interpreter.execute_statement(statement) {
            Ok(_) => println!("Statement executed successfully (suspiciously)"),
            Err(e) => println!("Statement failed successfully: {}", e),
        }
    }
}

#[test]
fn test_disable_all_useless_shit() {
    let source = r#"
#[directive(disable_all_useless_shit)]
// Everything should work normally below this line

// Variables should work normally
let x = 42;
let y = 10;

// Math should work normally
let sum = add(x, y);      // Should be exactly 52
let product = multiply(x, y);  // Should be exactly 420

// Comparisons should work normally
let isEqual = equals(sum, 52);     // Should be true
let isLess = lessThan(sum, 100);   // Should be true

// Arrays should work normally
let arr = [1, 2, 3];
let first = index(arr, 0);  // Should be exactly 1

// Objects should work normally
let obj = {
    "value": 42,
    "name": "normal"
};
let val = access(obj, "value");  // Should be exactly 42

// Async functions should work normally
async processData(data) {
    try {
        let result = promise(data, 1000);
        await result;
        return result;
    } catch error {
        return 0;
    }
}

// Function calls should work normally
let result = processData(42);
"#;

    // Parse the code
    let lexer = Lexer::new(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let program: Program = parser.parse().expect("Failed to parse program");

    // Execute the program
    let mut interpreter = Interpreter::new();
    let mut all_operations_normal = true;

    for statement in program {
        match interpreter.execute_statement(statement) {
            Ok(_) => (),
            Err(e) => {
                println!("Unexpected error in normal mode: {}", e);
                all_operations_normal = false;
            }
        }
    }

    assert!(all_operations_normal, "All operations should work normally with disable_all_useless_shit");
}
