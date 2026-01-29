use std::env;
use std::fs;

mod lexer;
mod parser;
mod interpreter;
mod token;
mod ast;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Whispem");
        return;
    }

    let filename = &args[1];

    let source = fs::read_to_string(filename)
        .expect("Failed to read source file");

    // 1. Lexer → tokens
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();

    // 2. Parser → AST / statements
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    // 3. Interpreter → execute
    let mut interpreter = Interpreter::new();
    interpreter.execute(statements);
}
