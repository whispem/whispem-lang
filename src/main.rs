mod ast;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod repl;
mod token;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl::run_repl(),
        2 => {
            let filename = &args[1];
            let source = fs::read_to_string(filename).unwrap_or_else(|e| {
                eprintln!("Error: Cannot read '{}': {}", filename, e);
                process::exit(1);
            });
            run_file(&source, filename);
        }
        _ => {
            eprintln!("Usage: whispem [file.wsp]");
            process::exit(1);
        }
    }
}

fn run_file(source: &str, filename: &str) {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => { eprintln!("{}: {}", filename, e); process::exit(1); }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => { eprintln!("{}: {}", filename, e); process::exit(1); }
    };

    let mut interpreter = Interpreter::new();
    if let Err(e) = interpreter.execute(program) {
        eprintln!("{}: {}", filename, e);
        process::exit(1);
    }
}