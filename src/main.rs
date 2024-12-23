mod ast;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let example_code =
        r#"
        print("Hello, World!");
        let x = add(5, 3);
        if (true) {
            print("True!");
        } else {
            print("False!");
        }
    "#;

    let lexer = Lexer::new(example_code);
    let tokens: Vec<_> = lexer.collect();
    println!("Tokens: {:#?}", tokens);

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => println!("AST: {:#?}", program),
        Err(e) => eprintln!("Parse error: {}", e),
    }
}
