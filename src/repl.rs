use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::{self, Write};

pub fn run_repl() {
    println!("Whispem v1.5.0 — REPL");
    println!("Type 'exit' or press Ctrl-C to quit.\n");

    let mut interpreter = Interpreter::new();

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                // EOF (Ctrl-D)
                println!();
                break;
            }
            Err(e) => {
                eprintln!("Input error: {}", e);
                break;
            }
            Ok(_) => {}
        }

        let trimmed = line.trim();
        if trimmed == "exit" || trimmed == "quit" {
            break;
        }
        if trimmed.is_empty() {
            continue;
        }

        // Allow multi-line blocks: if the line ends with '{', keep reading.
        let mut source = line.clone();
        if needs_more_input(trimmed) {
            loop {
                print_prompt("... ");
                io::stdout().flush().unwrap();
                let mut continuation = String::new();
                match io::stdin().read_line(&mut continuation) {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {}
                }
                source.push_str(&continuation);
                // Stop when we see a closing brace on its own line.
                if continuation.trim() == "}" {
                    break;
                }
            }
        }

        match run_source(&source, &mut interpreter) {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e),
        }
    }

    println!("Bye!");
}

fn needs_more_input(line: &str) -> bool {
    line.ends_with('{')
}

fn print_prompt(s: &str) {
    print!("{}", s);
}

pub fn run_source(source: &str, interpreter: &mut Interpreter) -> Result<(), String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| e.to_string())?;

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().map_err(|e| e.to_string())?;

    interpreter.execute(program).map_err(|e| e.to_string())
}