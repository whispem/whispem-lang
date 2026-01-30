use std::env;
use std::fs;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;

mod lexer;
mod parser;
mod interpreter;
mod token;
mod ast;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Whispem");
        return;
    }

    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let mut lexer = Lexer::new(&input);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        if token == token::Token::EOF {
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
