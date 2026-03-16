use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::vm::Vm;
use crate::error::ErrorKind;
use std::io::{self, Write};

pub fn run_repl() {
    println!("Whispem v4.0.0 — REPL");
    println!("Type 'exit' or press Ctrl-D to quit.\n");

    let mut vm = Vm::new();

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0)  => { println!(); break; }
            Err(e) => { eprintln!("Input error: {}", e); break; }
            Ok(_)  => {}
        }

        let trimmed = line.trim();
        if trimmed == "exit" || trimmed == "quit" {
            break;
        }
        if trimmed.is_empty() {
            continue;
        }

        let mut source = line.clone();
        if trimmed.ends_with('{') {
            loop {
                print!("... ");
                io::stdout().flush().unwrap();
                let mut cont = String::new();
                match io::stdin().read_line(&mut cont) {
                    Ok(0) | Err(_) => break,
                    Ok(_)          => {}
                }
                source.push_str(&cont);
                if cont.trim() == "}" { break; }
            }
        }

        if let Err(e) = run_source(&source, &mut vm) {
            match e.kind {
                ErrorKind::Exit(code) => {
                    println!("Bye!");
                    std::process::exit(code as i32);
                }
                _ => eprintln!("{}", e),
            }
        }
    }

    println!("Bye!");
}

fn run_source(source: &str, vm: &mut Vm) -> Result<(), crate::error::WhispemError> {
    let mut lexer  = Lexer::new(source);
    let tokens     = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let program    = parser.parse_program()?;
    let compiler   = Compiler::new();
    let (main_chunk, fn_chunks) = compiler.compile(program)?;
    for (name, chunk) in fn_chunks {
        vm.functions.insert(name, chunk);
    }
    vm.run(main_chunk)
}