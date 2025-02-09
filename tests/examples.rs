use std::fs;
use useless_lang::{
    ast::{Program},
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
