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
use token::Token;

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
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        if token == Token::EOF {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }

    // 2. Parser → statements
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    // 3. Interpreter → execute
    let mut interpreter = Interpreter::new();
    interpreter.execute(statements);
}
