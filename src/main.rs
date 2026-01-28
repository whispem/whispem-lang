mod lexer;
mod parser;
mod ast;
mod interpreter;

use std::fs;
use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let source = fs::read_to_string("examples/hello.wsp")
        .expect("Failed to read file");

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.run(program);
}
