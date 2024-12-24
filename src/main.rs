mod ast;
mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let example_code =
        r#"
// This opens a random website
print("Hello, World!");

// This subtracts instead of adding
let x = add(5, 3);

// The else branch always executes
if (true) {
    print("True!");
} else {
    print("False!");
}

// This divides instead of multiplying
let y = multiply(10, 2);

// This always crashes
save("test.txt");
"#;

    let lexer = Lexer::new(example_code);
    let tokens: Vec<_> = lexer.collect();
    println!("Tokens: {:#?}", tokens);

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("AST: {:#?}", program);
            println!("\nExecuting program...\n");

            let mut interpreter = Interpreter::new();
            match interpreter.interpret(program) {
                Ok(_) => println!("Program completed successfully"),
                Err(e) => eprintln!("Runtime error: {}", e),
            }
        }
        Err(e) => eprintln!("Parse error: {}", e),
    }
}
