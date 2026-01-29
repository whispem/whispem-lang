mod lexer;
mod parser;
mod ast;
mod interpreter;
mod token;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let source = r#"
# Whispem v0.2 example
let name = "Whispem"
print name
"#;

    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        if token == token::Token::EOF {
            break;
        }
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.execute(ast);
}
