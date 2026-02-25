mod ast;
mod chunk;
mod compiler;
mod error;
mod lexer;
mod opcode;
mod parser;
mod repl;
mod token;
mod value;
mod vm;

use compiler::Compiler;
use lexer::Lexer;
use parser::Parser;
use vm::Vm;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_]                              => repl::run_repl(),
        [_, file]                        => { let src = read(file); run(&src, file, false); }
        [_, flag, file] if flag=="--dump"=> { let src = read(file); run(&src, file, true);  }
        _ => { eprintln!("Usage: whispem [--dump] [file.wsp]"); process::exit(1); }
    }
}

fn run(source: &str, filename: &str, dump: bool) {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t)  => t,
        Err(e) => { eprintln!("{}: {}", filename, e); process::exit(1); }
    };
    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(p)  => p,
        Err(e) => { eprintln!("{}: {}", filename, e); process::exit(1); }
    };
    let compiler = Compiler::new();
    let (main_chunk, fn_chunks) = match compiler.compile(program) {
        Ok(r)  => r,
        Err(e) => { eprintln!("{}: {}", filename, e); process::exit(1); }
    };
    if dump {
        main_chunk.disassemble();
        let mut names: Vec<&String> = fn_chunks.keys().collect();
        names.sort();
        for n in names { println!(); fn_chunks[n].disassemble(); }
        return;
    }
    let mut vm = Vm::new();
    vm.functions = fn_chunks;
    if let Err(e) = vm.run(main_chunk) {
        eprintln!("{}: {}", filename, e);
        process::exit(1);
    }
}

fn read(filename: &str) -> String {
    fs::read_to_string(filename).unwrap_or_else(|e| {
        eprintln!("Error: Cannot read '{}': {}", filename, e);
        process::exit(1);
    })
}