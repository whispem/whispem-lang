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
        [_]                                 => repl::run_repl(),
        [_, file]                           => { let src = read(file); run_file(&src, file, false); }
        [_, flag, file] if flag == "--dump" => { let src = read(file); run_file(&src, file, true);  }
        _ => {
            eprintln!("Usage: whispem [--dump] [file.wsp]");
            process::exit(1);
        }
    }
}

fn run_file(source: &str, filename: &str, dump: bool) {
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

#[cfg(test)]
fn run_capturing(source: &str) -> Result<Vec<String>, String> {
    use crate::error::WhispemError;
    use std::sync::{Arc, Mutex};

    let mut lexer  = crate::lexer::Lexer::new(source);
    let tokens     = lexer.tokenize().map_err(|e: WhispemError| e.to_string())?;
    let mut parser = crate::parser::Parser::new(tokens);
    let program    = parser.parse_program().map_err(|e: WhispemError| e.to_string())?;
    let compiler   = crate::compiler::Compiler::new();
    let (main_chunk, fn_chunks) =
        compiler.compile(program).map_err(|e: WhispemError| e.to_string())?;

    let buf = Arc::new(Mutex::new(Vec::<u8>::new()));
    let result = {
        let mut vm = crate::vm::Vm::capturing(Arc::clone(&buf));
        vm.functions = fn_chunks;
        vm.run(main_chunk).map_err(|e: WhispemError| e.to_string())
        // vm drops here, Arc refcount back to 1
    };

    let raw = Arc::try_unwrap(buf).unwrap().into_inner().unwrap();
    let output = String::from_utf8_lossy(&raw);
    let lines: Vec<String> = output.lines().map(str::to_owned).collect();

    result?;
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::run_capturing;

    fn ok(src: &str) -> Vec<String> {
        run_capturing(src).unwrap_or_else(|e| panic!("Whispem error: {}", e))
    }
    fn err_msg(src: &str) -> String {
        run_capturing(src).expect_err("Expected an error but succeeded")
    }

    #[test] fn number_print()            { assert_eq!(ok("print 42"),          vec!["42"]); }
    #[test] fn float_print()             { assert_eq!(ok("print 3.14"),        vec!["3.14"]); }
    #[test] fn arithmetic_add()          { assert_eq!(ok("print 2 + 3"),       vec!["5"]); }
    #[test] fn arithmetic_sub()          { assert_eq!(ok("print 10 - 4"),      vec!["6"]); }
    #[test] fn arithmetic_mul()          { assert_eq!(ok("print 3 * 4"),       vec!["12"]); }
    #[test] fn arithmetic_div()          { assert_eq!(ok("print 10 / 4"),      vec!["2.5"]); }
    #[test] fn modulo_basic()            { assert_eq!(ok("print 10 % 3"),      vec!["1"]); }
    #[test] fn modulo_exact()            { assert_eq!(ok("print 15 % 5"),      vec!["0"]); }
    #[test] fn precedence_mul_over_add() { assert_eq!(ok("print 10 + 5 * 2"), vec!["20"]); }
    #[test] fn precedence_parens()       { assert_eq!(ok("print (10+5)*2"),    vec!["30"]); }
    #[test] fn unary_neg()               { assert_eq!(ok("let x=-7\nprint x"), vec!["-7"]); }
    #[test] fn div_by_zero_error()       { assert!(err_msg("print 1/0").contains("Division by zero")); }

    #[test] fn string_print()    { assert_eq!(ok("print \"hello\""),           vec!["hello"]); }
    #[test] fn string_concat()   { assert_eq!(ok("print \"a\"+\"b\""),         vec!["ab"]); }
    #[test] fn string_num_cat()  { assert_eq!(ok("print \"n=\"+42"),           vec!["n=42"]); }
    #[test] fn string_escape()   { assert_eq!(ok("print \"hi\\nthere\""),      vec!["hi","there"]); }
    #[test] fn string_length()   { assert_eq!(ok("print length(\"hello\")"),   vec!["5"]); }

    #[test] fn let_basic()  { assert_eq!(ok("let x=10\nprint x"),              vec!["10"]); }
    #[test] fn let_update() { assert_eq!(ok("let x=1\nlet x=x+1\nprint x"),   vec!["2"]); }

    #[test] fn bool_true()   { assert_eq!(ok("print true"),    vec!["true"]); }
    #[test] fn bool_false()  { assert_eq!(ok("print false"),   vec!["false"]); }
    #[test] fn cmp_lt()      { assert_eq!(ok("print 1<2"),     vec!["true"]); }
    #[test] fn cmp_gt()      { assert_eq!(ok("print 2>1"),     vec!["true"]); }
    #[test] fn cmp_eq()      { assert_eq!(ok("print 1==1"),    vec!["true"]); }
    #[test] fn cmp_neq()     { assert_eq!(ok("print 1!=2"),    vec!["true"]); }
    #[test] fn cmp_false()   { assert_eq!(ok("print 5<3"),     vec!["false"]); }

    #[test] fn logic_and_ff()      { assert_eq!(ok("print true and false"),  vec!["false"]); }
    #[test] fn logic_and_tt()      { assert_eq!(ok("print true and true"),   vec!["true"]); }
    #[test] fn logic_or_ft()       { assert_eq!(ok("print false or true"),   vec!["true"]); }
    #[test] fn logic_not()         { assert_eq!(ok("print not true"),         vec!["false"]); }
    #[test] fn short_circuit_and() { assert_eq!(ok("let r=false and (1==1)\nprint r"), vec!["false"]); }
    #[test] fn short_circuit_or()  { assert_eq!(ok("let r=true or (1==1)\nprint r"),  vec!["true"]); }

    #[test] fn if_true()  { assert_eq!(ok("if true { print \"yes\" }"),       vec!["yes"]); }
    #[test] fn if_false() { assert_eq!(ok("if false { print \"yes\" }"),      Vec::<String>::new()); }
    #[test] fn if_else_taken()     { assert_eq!(ok("if 10>5 { print \"big\" } else { print \"small\" }"), vec!["big"]); }
    #[test] fn if_else_not_taken() { assert_eq!(ok("if 1>5  { print \"big\" } else { print \"small\" }"), vec!["small"]); }

    #[test] fn while_basic() {
        assert_eq!(ok("let i=0\nwhile i<3 { print i\nlet i=i+1 }"), vec!["0","1","2"]);
    }
    #[test] fn for_array() { assert_eq!(ok("for n in [1,2,3] { print n }"),     vec!["1","2","3"]); }
    #[test] fn for_range()  { assert_eq!(ok("for i in range(0,4) { print i }"), vec!["0","1","2","3"]); }
    #[test] fn break_stops_loop() {
        assert_eq!(ok("for n in range(1,10) { if n>3 { break }\nprint n }"),   vec!["1","2","3"]);
    }
    #[test] fn continue_skips() {
        assert_eq!(ok("for n in range(1,6) { if n==3 { continue }\nprint n }"),vec!["1","2","4","5"]);
    }

    #[test] fn fn_basic()       { assert_eq!(ok("fn double(n) { return n*2 }\nprint double(7)"),  vec!["14"]); }
    #[test] fn fn_void()        { assert_eq!(ok("fn say(x) { print x }\nsay(\"hi\")"),            vec!["hi"]); }
    #[test] fn fn_recursion()   {
        assert_eq!(ok("fn fact(n) { if n<=1 { return 1 }\nreturn n*fact(n-1) }\nprint fact(5)"), vec!["120"]);
    }
    #[test] fn fn_forward_call(){ assert_eq!(ok("print triple(3)\nfn triple(n) { return n*3 }"),  vec!["9"]); }
    #[test] fn fn_reads_global(){
        assert_eq!(ok("let g=\"hi\"\nfn say(name) { print g+\" \"+name }\nsay(\"em\")"),          vec!["hi em"]);
    }
    #[test] fn fn_multi_params(){ assert_eq!(ok("fn add(a,b) { return a+b }\nprint add(3,4)"),   vec!["7"]); }
    #[test] fn fn_wrong_arity() { assert!(err_msg("fn f(x) { return x }\nf(1,2)").contains("expected 1")); }

    #[test] fn array_index()   { assert_eq!(ok("let a=[10,20,30]\nprint a[2]"),          vec!["30"]); }
    #[test] fn array_assign()  { assert_eq!(ok("let a=[1,2,3]\na[1]=99\nprint a[1]"),   vec!["99"]); }
    #[test] fn array_length()  { assert_eq!(ok("print length([1,2,3,4])"),               vec!["4"]); }
    #[test] fn array_push()    { assert_eq!(ok("let a=push([1,2],3)\nprint length(a)"),  vec!["3"]); }
    #[test] fn array_pop()     { assert_eq!(ok("print pop([1,2,3])"),                    vec!["3"]); }
    #[test] fn array_reverse() { assert_eq!(ok("print reverse([1,2,3])"),                vec!["[3, 2, 1]"]); }
    #[test] fn array_slice()   { assert_eq!(ok("print slice([10,20,30,40,50],1,4)"),     vec!["[20, 30, 40]"]); }
    #[test] fn array_range()   { assert_eq!(ok("print range(0,5)"),                      vec!["[0, 1, 2, 3, 4]"]); }
    #[test] fn array_oob()     { assert!(err_msg("let a=[1]\nprint a[5]").contains("out of bounds")); }

    #[test] fn dict_access()    { assert_eq!(ok("let d={\"a\":1}\nprint d[\"a\"]"),              vec!["1"]); }
    #[test] fn dict_assign()    { assert_eq!(ok("let d={\"x\":10}\nd[\"x\"]=99\nprint d[\"x\"]"),vec!["99"]); }
    #[test] fn dict_new_key()   { assert_eq!(ok("let d={}\nd[\"k\"]=42\nprint d[\"k\"]"),         vec!["42"]); }
    #[test] fn dict_has_key_t() { assert_eq!(ok("print has_key({\"a\":1},\"a\")"),                vec!["true"]); }
    #[test] fn dict_has_key_f() { assert_eq!(ok("print has_key({\"a\":1},\"z\")"),                vec!["false"]); }
    #[test] fn dict_keys_sorted(){ assert_eq!(ok("print keys({\"b\":2,\"a\":1,\"c\":3})"),        vec!["[a, b, c]"]); }
    #[test] fn dict_length()    { assert_eq!(ok("print length({\"a\":1,\"b\":2})"),                vec!["2"]); }

    #[test] fn falsy_zero()      { assert_eq!(ok("if 0 { print \"y\" } else { print \"n\" }"),    vec!["n"]); }
    #[test] fn falsy_empty_str() { assert_eq!(ok("if \"\" { print \"y\" } else { print \"n\" }"), vec!["n"]); }
    #[test] fn falsy_empty_arr() { assert_eq!(ok("if [] { print \"y\" } else { print \"n\" }"),   vec!["n"]); }
    #[test] fn truthy_nonzero()  { assert_eq!(ok("if 1 { print \"y\" }"),                          vec!["y"]); }

    #[test] fn error_has_span() {
        let e = err_msg("let x=1\nprint undefined_var");
        assert!(e.contains("line 2"), "Expected 'line 2' in: {}", e);
    }

    #[test]
    fn fizzbuzz_1_to_15() {
        let src = "for n in range(1,16) {\nif n%15==0 { print \"FizzBuzz\" } else {\nif n%3==0 { print \"Fizz\" } else {\nif n%5==0 { print \"Buzz\" } else { print n } } } }";
        let expected: Vec<&str> = vec!["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"];
        assert_eq!(ok(src), expected);
    }

    #[test]
    fn word_counter() {
        let src = "fn count(words) {\nlet c={}\nfor w in words {\nif has_key(c,w) { c[w]=c[w]+1 } else { c[w]=1 } }\nreturn c }\nlet r=count([\"a\",\"b\",\"a\",\"a\"])\nprint r[\"a\"]\nprint r[\"b\"]";
        assert_eq!(ok(src), vec!["3","1"]);
    }

    #[test]
    fn fibonacci() {
        let src = "fn fib(n) { if n<=1 { return n }\nreturn fib(n-1)+fib(n-2) }\nprint fib(10)";
        assert_eq!(ok(src), vec!["55"]);
    }
}