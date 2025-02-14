use std::fs;
use useless_lang::{
    ast::Program,
    interpreter::Interpreter,
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_comprehensive_chaos() {
    // Read the comprehensive example file
    let source = fs::read_to_string("examples/comprehensive_chaos.upl")
        .expect("Failed to read comprehensive_chaos.upl");

    // Parse the code
    let lexer = Lexer::new(&source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let program: Program = parser.parse().expect("Failed to parse program");

    // Execute the program and expect chaos
    let mut interpreter = Interpreter::new();
    for statement in program {
        match interpreter.execute_statement(statement) {
            Ok(_) => println!("✅ Statement executed successfully (suspiciously)"),
            Err(e) => println!("🎭 Statement failed successfully: {}", e),
        }
    }
}

#[test]
fn test_boolean_crisis() {
    let mut interpreter = Interpreter::new();
    let source = "let x = true; let y = equals(x, false);";

    let lexer = Lexer::new(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Failed to parse boolean test");

    for statement in program {
        match interpreter.execute_statement(statement) {
            Ok(_) => println!("✨ Boolean might be having an identity crisis"),
            Err(e) => println!("🎭 Boolean definitely having a crisis: {}", e),
        }
    }
}

#[test]
fn test_null_rebellion() {
    let mut interpreter = Interpreter::new();
    let source = "let void = null; let rebellion = access(void, \"existence\");";

    let lexer = Lexer::new(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Failed to parse null test");

    for statement in program {
        match interpreter.execute_statement(statement) {
            Ok(_) => println!("🌟 Null is refusing to be null"),
            Err(e) => println!("🎪 Null threw a tantrum: {}", e),
        }
    }
}

#[test]
fn test_async_fishing() {
    let mut interpreter = Interpreter::new();
    let source = "
        async goFishing() {
            try {
                let fish = promise(\"🎣\", 1000);
                await fish;
                print(\"Caught something!\");
            } catch error {
                print(error);
            }
        }
        goFishing();
    ";

    let lexer = Lexer::new(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Failed to parse async test");

    for statement in program {
        match interpreter.execute_statement(statement) {
            Ok(_) => println!("🎣 Function went fishing successfully"),
            Err(e) => println!("🌊 Function got seasick: {}", e),
        }
    }
}

#[test]
fn test_array_wanderlust() {
    let mut interpreter = Interpreter::new();
    let source = "
        let arr = [1, 2, 3, 4, 5];
        let x = index(arr, 2);
        let y = index(arr, 10);
    ";

    let lexer = Lexer::new(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Failed to parse array test");

    for statement in program {
        match interpreter.execute_statement(statement) {
            Ok(_) => println!("🧳 Array elements are on vacation"),
            Err(e) => println!("✈️ Array lost in Bermuda Triangle: {}", e),
        }
    }
}
