use std::env;
use std::fs;

mod lexer;
mod parser;
mod ast;
mod interpreter;
mod token;

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

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut interpreter = Interpreter::new();
    interpreter.execute(program);
}
