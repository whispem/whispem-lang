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

    let input = fs::read_to_string(&args[1]).expect("Failed to read file");
    let mut lexer = Lexer::new(&input);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        if token == Token::EOF {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    let mut interpreter = Interpreter::new();
    interpreter.execute(program);
}