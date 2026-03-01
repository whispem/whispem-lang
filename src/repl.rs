use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::vm::Vm;
use std::io::{self, Write};

pub fn run_repl() {
    println!("Whispem v2.5.0 — REPL");
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

        // Simple multi-line support: keep reading until the block closes.
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
                if cont.trim() == "}" {
                    break;
                }
            }
        }

        if let Err(e) = run_source(&source, &mut vm) {
            eprintln!("{}", e);
        }
    }

    println!("Bye!");
}

fn run_source(source: &str, vm: &mut Vm) -> Result<(), String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| e.to_string())?;

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().map_err(|e| e.to_string())?;

    let compiler = Compiler::new();
    let (main_chunk, fn_chunks) = compiler.compile(program).map_err(|e| e.to_string())?;

    for (name, chunk) in fn_chunks {
        vm.functions.insert(name, chunk);
    }
    vm.run(main_chunk).map_err(|e| e.to_string())
}