mod lexer;

use lexer::Lexer;

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
    for token in lexer {
        println!("{:?}", token);
    }
}
