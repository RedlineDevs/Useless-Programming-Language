use std::env;
use std::fs;
use std::process;

mod ast;
mod interpreter;
mod lexer;
mod parser;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Usage: useless-lang <file.upl>");
        eprintln!("Example: useless-lang examples/hello.upl");
        process::exit(1);
    };

    let source_code = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            process::exit(1);
        }
    };

    let lexer = Lexer::new(&source_code);
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
