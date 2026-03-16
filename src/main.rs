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

use chunk::{deserialise, serialise};
use compiler::Compiler;
use error::ErrorKind;
use lexer::Lexer;
use parser::Parser;
use vm::Vm;

use std::env;
use std::fs;
use std::path::Path;
use std::process;

// ─── CLI ──────────────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_] => repl::run_repl(),

        [_, flag, file] if flag == "--dump" => {
            let src = read_source(file);
            run_file(&src, file, true, vec![]);
        }

        [_, flag, file] if flag == "--compile" => {
            let src = read_source(file);
            let out = output_path(file, ".whbc");
            compile_to_file(&src, file, &out);
        }

        [_, flag, src_file, out_file] if flag == "--compile" => {
            let src = read_source(src_file);
            compile_to_file(&src, src_file, out_file);
        }

        [_, file, rest @ ..] => {
            let script_args: Vec<String> = rest.to_vec();
            if file.ends_with(".whbc") {
                run_bytecode_file(file, script_args);
            } else {
                let src = read_source(file);
                run_file(&src, file, false, script_args);
            }
        }

        _ => {
            eprintln!("Usage: whispem [--dump | --compile] [file.wsp] [args...]");
            process::exit(1);
        }
    }
}

// ─── Source → run ────────────────────────────────────────────────────────────

fn run_file(source: &str, filename: &str, dump: bool, script_args: Vec<String>) {
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
        for n in names {
            println!();
            fn_chunks[n].disassemble();
        }
        return;
    }
    let mut vm = Vm::new();
    vm.functions   = fn_chunks;
    vm.script_args = script_args;
    if let Err(e) = vm.run(main_chunk) {
        handle_vm_error(e, filename);
    }
}

// ─── Source → .whbc ──────────────────────────────────────────────────────────

fn compile_to_file(source: &str, src_name: &str, out_path: &str) {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t)  => t,
        Err(e) => { eprintln!("{}: {}", src_name, e); process::exit(1); }
    };
    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(p)  => p,
        Err(e) => { eprintln!("{}: {}", src_name, e); process::exit(1); }
    };
    let compiler = Compiler::new();
    let (main_chunk, fn_chunks) = match compiler.compile(program) {
        Ok(r)  => r,
        Err(e) => { eprintln!("{}: {}", src_name, e); process::exit(1); }
    };
    let bytes = match serialise(&main_chunk, &fn_chunks) {
        Ok(b)  => b,
        Err(e) => { eprintln!("{}: serialisation error: {}", src_name, e); process::exit(1); }
    };
    if let Err(e) = fs::write(out_path, &bytes) {
        eprintln!("Cannot write '{}': {}", out_path, e);
        process::exit(1);
    }
    eprintln!("Compiled {} → {} ({} bytes)", src_name, out_path, bytes.len());
}

// ─── .whbc → run ─────────────────────────────────────────────────────────────

fn run_bytecode_file(path: &str, script_args: Vec<String>) {
    let data = match fs::read(path) {
        Ok(d)  => d,
        Err(e) => { eprintln!("Cannot read '{}': {}", path, e); process::exit(1); }
    };
    let (main_chunk, fn_chunks) = match deserialise(&data) {
        Ok(r)  => r,
        Err(e) => { eprintln!("{}: {}", path, e); process::exit(1); }
    };
    let mut vm = Vm::new();
    vm.functions   = fn_chunks;
    vm.script_args = script_args;
    if let Err(e) = vm.run(main_chunk) {
        handle_vm_error(e, path);
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn read_source(filename: &str) -> String {
    fs::read_to_string(filename).unwrap_or_else(|e| {
        eprintln!("Error: Cannot read '{}': {}", filename, e);
        process::exit(1);
    })
}

fn output_path(path: &str, new_ext: &str) -> String {
    let p    = Path::new(path);
    let stem = p.file_stem().unwrap_or_default().to_string_lossy();
    let dir  = p.parent().map(|d| d.to_string_lossy().into_owned()).unwrap_or_default();
    if dir.is_empty() {
        format!("{}{}", stem, new_ext)
    } else {
        format!("{}/{}{}", dir, stem, new_ext)
    }
}

// Exit is not an error — propagate the exit code to the OS.
fn handle_vm_error(e: error::WhispemError, filename: &str) {
    match e.kind {
        ErrorKind::Exit(code) => process::exit(code as i32),
        _ => {
            eprintln!("{}: {}", filename, e);
            process::exit(1);
        }
    }
}

// ─── Test harness ─────────────────────────────────────────────────────────────

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
    };

    let raw    = Arc::try_unwrap(buf).unwrap().into_inner().unwrap();
    let output = String::from_utf8_lossy(&raw);
    let lines: Vec<String> = output.lines().map(str::to_owned).collect();

    result?;
    Ok(lines)
}

#[cfg(test)]
fn run_via_bytecode(source: &str) -> Result<Vec<String>, String> {
    use crate::error::WhispemError;
    use std::sync::{Arc, Mutex};

    let mut lexer  = crate::lexer::Lexer::new(source);
    let tokens     = lexer.tokenize().map_err(|e: WhispemError| e.to_string())?;
    let mut parser = crate::parser::Parser::new(tokens);
    let program    = parser.parse_program().map_err(|e: WhispemError| e.to_string())?;
    let compiler   = crate::compiler::Compiler::new();
    let (main_chunk, fn_chunks) =
        compiler.compile(program).map_err(|e: WhispemError| e.to_string())?;

    let bytes = crate::chunk::serialise(&main_chunk, &fn_chunks)
        .map_err(|e: WhispemError| e.to_string())?;

    let (main2, fns2) = crate::chunk::deserialise(&bytes)
        .map_err(|e: WhispemError| e.to_string())?;

    let buf = Arc::new(Mutex::new(Vec::<u8>::new()));
    let result = {
        let mut vm = crate::vm::Vm::capturing(Arc::clone(&buf));
        vm.functions = fns2;
        vm.run(main2).map_err(|e: WhispemError| e.to_string())
    };

    let raw    = Arc::try_unwrap(buf).unwrap().into_inner().unwrap();
    let output = String::from_utf8_lossy(&raw);
    let lines: Vec<String> = output.lines().map(str::to_owned).collect();

    result?;
    Ok(lines)
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::{run_capturing, run_via_bytecode};

    fn ok(src: &str) -> Vec<String> {
        run_capturing(src).unwrap_or_else(|e| panic!("Whispem error: {}", e))
    }
    fn err_msg(src: &str) -> String {
        run_capturing(src).expect_err("Expected an error but succeeded")
    }
    fn ok_bc(src: &str) -> Vec<String> {
        run_via_bytecode(src).unwrap_or_else(|e| panic!("Bytecode round-trip error: {}", e))
    }

    // ── Arithmetic ────────────────────────────────────────────────────────
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

    // ── Strings ───────────────────────────────────────────────────────────
    #[test] fn string_print()   { assert_eq!(ok("print \"hello\""),          vec!["hello"]); }
    #[test] fn string_concat()  { assert_eq!(ok("print \"a\"+\"b\""),        vec!["ab"]); }
    #[test] fn string_num_cat() { assert_eq!(ok("print \"n=\"+42"),          vec!["n=42"]); }
    #[test] fn string_escape()  { assert_eq!(ok("print \"hi\\nthere\""),     vec!["hi","there"]); }
    #[test] fn string_length()  { assert_eq!(ok("print length(\"hello\")"),  vec!["5"]); }

    // ── Variables ─────────────────────────────────────────────────────────
    #[test] fn let_basic()  { assert_eq!(ok("let x=10\nprint x"),           vec!["10"]); }
    #[test] fn let_update() { assert_eq!(ok("let x=1\nlet x=x+1\nprint x"), vec!["2"]); }

    // ── Booleans & comparisons ────────────────────────────────────────────
    #[test] fn bool_true()   { assert_eq!(ok("print true"),   vec!["true"]); }
    #[test] fn bool_false()  { assert_eq!(ok("print false"),  vec!["false"]); }
    #[test] fn cmp_lt()      { assert_eq!(ok("print 1<2"),    vec!["true"]); }
    #[test] fn cmp_gt()      { assert_eq!(ok("print 2>1"),    vec!["true"]); }
    #[test] fn cmp_eq()      { assert_eq!(ok("print 1==1"),   vec!["true"]); }
    #[test] fn cmp_neq()     { assert_eq!(ok("print 1!=2"),   vec!["true"]); }
    #[test] fn cmp_false()   { assert_eq!(ok("print 5<3"),    vec!["false"]); }

    // ── Logic ─────────────────────────────────────────────────────────────
    #[test] fn logic_and_ff()      { assert_eq!(ok("print true and false"),  vec!["false"]); }
    #[test] fn logic_and_tt()      { assert_eq!(ok("print true and true"),   vec!["true"]); }
    #[test] fn logic_or_ft()       { assert_eq!(ok("print false or true"),   vec!["true"]); }
    #[test] fn logic_not()         { assert_eq!(ok("print not true"),         vec!["false"]); }
    #[test] fn short_circuit_and() { assert_eq!(ok("let r=false and (1==1)\nprint r"), vec!["false"]); }
    #[test] fn short_circuit_or()  { assert_eq!(ok("let r=true or (1==1)\nprint r"),  vec!["true"]); }

    // ── Control flow ──────────────────────────────────────────────────────
    #[test] fn if_true()           { assert_eq!(ok("if true { print \"yes\" }"),      vec!["yes"]); }
    #[test] fn if_false()          { assert_eq!(ok("if false { print \"yes\" }"),     Vec::<String>::new()); }
    #[test] fn if_else_taken()     { assert_eq!(ok("if 10>5 { print \"big\" } else { print \"small\" }"), vec!["big"]); }
    #[test] fn if_else_not_taken() { assert_eq!(ok("if 1>5  { print \"big\" } else { print \"small\" }"), vec!["small"]); }

    #[test] fn while_basic() {
        assert_eq!(ok("let i=0\nwhile i<3 { print i\nlet i=i+1 }"), vec!["0","1","2"]);
    }
    #[test] fn for_array()  { assert_eq!(ok("for n in [1,2,3] { print n }"),     vec!["1","2","3"]); }
    #[test] fn for_range()  { assert_eq!(ok("for i in range(0,4) { print i }"),  vec!["0","1","2","3"]); }
    #[test] fn break_stops_loop() {
        assert_eq!(ok("for n in range(1,10) { if n>3 { break }\nprint n }"),   vec!["1","2","3"]);
    }
    #[test] fn continue_skips() {
        assert_eq!(ok("for n in range(1,6) { if n==3 { continue }\nprint n }"),vec!["1","2","4","5"]);
    }

    // ── Functions ─────────────────────────────────────────────────────────
    #[test] fn fn_basic()        { assert_eq!(ok("fn double(n) { return n*2 }\nprint double(7)"), vec!["14"]); }
    #[test] fn fn_void()         { assert_eq!(ok("fn say(x) { print x }\nsay(\"hi\")"),           vec!["hi"]); }
    #[test] fn fn_recursion() {
        assert_eq!(ok("fn fact(n) { if n<=1 { return 1 }\nreturn n*fact(n-1) }\nprint fact(5)"), vec!["120"]);
    }
    #[test] fn fn_forward_call() { assert_eq!(ok("print triple(3)\nfn triple(n) { return n*3 }"),  vec!["9"]); }
    #[test] fn fn_reads_global() {
        assert_eq!(ok("let g=\"hi\"\nfn say(name) { print g+\" \"+name }\nsay(\"em\")"), vec!["hi em"]);
    }
    #[test] fn fn_multi_params() { assert_eq!(ok("fn add(a,b) { return a+b }\nprint add(3,4)"),   vec!["7"]); }
    #[test] fn fn_wrong_arity()  { assert!(err_msg("fn f(x) { return x }\nf(1,2)").contains("expected 1")); }
    #[test] fn fn_no_params()    { assert_eq!(ok("fn pi() { return 3 }\nprint pi()"),              vec!["3"]); }

    // ── Arrays ────────────────────────────────────────────────────────────
    #[test] fn array_index()   { assert_eq!(ok("let a=[10,20,30]\nprint a[2]"),         vec!["30"]); }
    #[test] fn array_assign()  { assert_eq!(ok("let a=[1,2,3]\na[1]=99\nprint a[1]"),  vec!["99"]); }
    #[test] fn array_length()  { assert_eq!(ok("print length([1,2,3,4])"),              vec!["4"]); }
    #[test] fn array_push()    { assert_eq!(ok("let a=push([1,2],3)\nprint length(a)"),vec!["3"]); }
    #[test] fn array_pop()     { assert_eq!(ok("print pop([1,2,3])"),                  vec!["3"]); }
    #[test] fn array_reverse() { assert_eq!(ok("print reverse([1,2,3])"),              vec!["[3, 2, 1]"]); }
    #[test] fn array_slice()   { assert_eq!(ok("print slice([10,20,30,40,50],1,4)"),   vec!["[20, 30, 40]"]); }
    #[test] fn array_range()   { assert_eq!(ok("print range(0,5)"),                    vec!["[0, 1, 2, 3, 4]"]); }
    #[test] fn array_oob()     { assert!(err_msg("let a=[1]\nprint a[5]").contains("out of bounds")); }

    // ── Dictionaries ──────────────────────────────────────────────────────
    #[test] fn dict_access()     { assert_eq!(ok("let d={\"a\":1}\nprint d[\"a\"]"),               vec!["1"]); }
    #[test] fn dict_assign()     { assert_eq!(ok("let d={\"x\":10}\nd[\"x\"]=99\nprint d[\"x\"]"), vec!["99"]); }
    #[test] fn dict_new_key()    { assert_eq!(ok("let d={}\nd[\"k\"]=42\nprint d[\"k\"]"),          vec!["42"]); }
    #[test] fn dict_has_key_t()  { assert_eq!(ok("print has_key({\"a\":1},\"a\")"),                 vec!["true"]); }
    #[test] fn dict_has_key_f()  { assert_eq!(ok("print has_key({\"a\":1},\"z\")"),                 vec!["false"]); }
    #[test] fn dict_keys_sorted(){ assert_eq!(ok("print keys({\"b\":2,\"a\":1,\"c\":3})"),          vec!["[a, b, c]"]); }
    #[test] fn dict_length()     { assert_eq!(ok("print length({\"a\":1,\"b\":2})"),                vec!["2"]); }
    #[test] fn dict_missing_key_error() {
        let e = err_msg("let d={\"a\":1}\nprint d[\"z\"]");
        assert!(e.contains("\"z\" not found in dict"), "got: {}", e);
    }

    // ── Truthiness ────────────────────────────────────────────────────────
    #[test] fn falsy_zero()      { assert_eq!(ok("if 0 { print \"y\" } else { print \"n\" }"),    vec!["n"]); }
    #[test] fn falsy_empty_str() { assert_eq!(ok("if \"\" { print \"y\" } else { print \"n\" }"), vec!["n"]); }
    #[test] fn falsy_empty_arr() { assert_eq!(ok("if [] { print \"y\" } else { print \"n\" }"),   vec!["n"]); }
    #[test] fn truthy_nonzero()  { assert_eq!(ok("if 1 { print \"y\" }"),                          vec!["y"]); }

    // ── Error spans ───────────────────────────────────────────────────────
    #[test] fn error_has_span() {
        let e = err_msg("let x=1\nprint undefined_var");
        assert!(e.contains("line 2"), "Expected 'line 2' in: {}", e);
    }

    // ── Integration ───────────────────────────────────────────────────────
    #[test]
    fn fizzbuzz_1_to_15() {
        let src = "\
for n in range(1,16) {
  if n%15==0 { print \"FizzBuzz\" } else {
  if n%3==0  { print \"Fizz\"     } else {
  if n%5==0  { print \"Buzz\"     } else { print n } } } }";
        let expected: Vec<&str> = vec![
            "1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz",
            "11","Fizz","13","14","FizzBuzz",
        ];
        assert_eq!(ok(src), expected);
    }

    #[test]
    fn word_counter() {
        let src = "\
fn count(words) {
  let c={}
  for w in words {
    if has_key(c,w) { c[w]=c[w]+1 } else { c[w]=1 }
  }
  return c
}
let r=count([\"a\",\"b\",\"a\",\"a\"])
print r[\"a\"]
print r[\"b\"]";
        assert_eq!(ok(src), vec!["3","1"]);
    }

    #[test]
    fn fibonacci() {
        let src = "fn fib(n) { if n<=1 { return n }\nreturn fib(n-1)+fib(n-2) }\nprint fib(10)";
        assert_eq!(ok(src), vec!["55"]);
    }

    // ── v3.0.0: bytecode serialisation round-trip ─────────────────────────

    #[test]
    fn bytecode_roundtrip_hello() {
        assert_eq!(ok_bc("print \"hello\""), vec!["hello"]);
    }
    #[test]
    fn bytecode_roundtrip_arithmetic() {
        assert_eq!(ok_bc("print 2+3"),    vec!["5"]);
        assert_eq!(ok_bc("print 10 % 3"), vec!["1"]);
    }
    #[test]
    fn bytecode_roundtrip_variable() {
        assert_eq!(ok_bc("let x=42\nprint x"), vec!["42"]);
    }
    #[test]
    fn bytecode_roundtrip_function() {
        assert_eq!(ok_bc("fn double(n){ return n*2 }\nprint double(7)"), vec!["14"]);
    }
    #[test]
    fn bytecode_roundtrip_loop() {
        assert_eq!(ok_bc("for i in range(0,3){ print i }"), vec!["0","1","2"]);
    }
    #[test]
    fn bytecode_roundtrip_array() {
        assert_eq!(ok_bc("print reverse([1,2,3])"), vec!["[3, 2, 1]"]);
    }
    #[test]
    fn bytecode_roundtrip_dict() {
        assert_eq!(ok_bc("let d={\"a\":1}\nprint d[\"a\"]"), vec!["1"]);
    }
    #[test]
    fn bytecode_roundtrip_fizzbuzz() {
        let src = "\
for n in range(1,16) {
  if n%15==0 { print \"FizzBuzz\" } else {
  if n%3==0  { print \"Fizz\"     } else {
  if n%5==0  { print \"Buzz\"     } else { print n } } } }";
        let expected: Vec<&str> = vec![
            "1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz",
            "11","Fizz","13","14","FizzBuzz",
        ];
        assert_eq!(ok_bc(src), expected);
    }
    #[test]
    fn bytecode_roundtrip_recursion() {
        let src = "fn fact(n){ if n<=1 { return 1 }\nreturn n*fact(n-1) }\nprint fact(6)";
        assert_eq!(ok_bc(src), vec!["720"]);
    }
    #[test]
    fn bytecode_roundtrip_global_in_fn() {
        assert_eq!(
            ok_bc("let g=\"world\"\nfn greet(){ print \"hello \"+g }\ngreet()"),
            vec!["hello world"]
        );
    }
    #[test]
    fn bytecode_magic_bad() {
        use crate::chunk::deserialise;
        let bad = b"BADC\x03\x00\x01";
        assert!(deserialise(bad).is_err());
    }
    #[test]
    fn bytecode_version_bad() {
        use crate::chunk::deserialise;
        let mut bad = b"WHBC\x02\x00\x01".to_vec();
        bad.extend_from_slice(&[0u8; 20]);
        assert!(deserialise(&bad).is_err());
    }
    #[test]
    fn bytecode_truncated() {
        use crate::chunk::deserialise;
        assert!(deserialise(b"WHBC").is_err());
    }

    #[test]
    fn output_path_basic() {
        use super::output_path;
        assert_eq!(output_path("hello.wsp",        ".whbc"), "hello.whbc");
        assert_eq!(output_path("examples/foo.wsp", ".whbc"), "examples/foo.whbc");
        assert_eq!(output_path("deep/dir/bar.wsp", ".whbc"), "deep/dir/bar.whbc");
    }

    #[test]
    fn char_at_basic() {
        assert_eq!(ok("print char_at(\"hello\", 0)"), vec!["h"]);
        assert_eq!(ok("print char_at(\"hello\", 4)"), vec!["o"]);
    }
    #[test]
    fn substr_basic() {
        assert_eq!(ok("print substr(\"hello world\", 6, 5)"), vec!["world"]);
        assert_eq!(ok("print substr(\"abc\", 0, 2)"),         vec!["ab"]);
    }
    #[test]
    fn ord_basic() {
        assert_eq!(ok("print ord(\"A\")"), vec!["65"]);
        assert_eq!(ok("print ord(\"a\")"), vec!["97"]);
        assert_eq!(ok("print ord(\"0\")"), vec!["48"]);
    }
    #[test]
    fn num_to_str_basic() {
        assert_eq!(ok("print num_to_str(42)"),   vec!["42"]);
        assert_eq!(ok("print num_to_str(3.14)"), vec!["3.14"]);
    }
    #[test]
    fn str_to_num_basic() {
        assert_eq!(ok("print str_to_num(\"42\")"),   vec!["42"]);
        assert_eq!(ok("print str_to_num(\"3.14\")"), vec!["3.14"]);
    }
    #[test]
    fn multiline_array_literal() {
        assert_eq!(ok("let a = [\n  1,\n  2,\n  3\n]\nprint length(a)"), vec!["3"]);
    }

    // ── v4.0.0: else if ───────────────────────────────────────────────────

    #[test]
    fn else_if_basic() {
        let src = "\
let x = 2
if x == 1 { print \"one\" }
else if x == 2 { print \"two\" }
else if x == 3 { print \"three\" }
else { print \"other\" }";
        assert_eq!(ok(src), vec!["two"]);
    }

    #[test]
    fn else_if_chain_last() {
        let src = "\
let x = 3
if x == 1 { print \"one\" }
else if x == 2 { print \"two\" }
else if x == 3 { print \"three\" }
else { print \"other\" }";
        assert_eq!(ok(src), vec!["three"]);
    }

    #[test]
    fn else_if_falls_to_else() {
        let src = "\
let x = 99
if x == 1 { print \"one\" }
else if x == 2 { print \"two\" }
else { print \"other\" }";
        assert_eq!(ok(src), vec!["other"]);
    }

    #[test]
    fn else_if_no_else() {
        let src = "\
let x = 5
if x == 1 { print \"one\" }
else if x == 2 { print \"two\" }";
        assert_eq!(ok(src), Vec::<String>::new());
    }

    #[test]
    fn else_if_fizzbuzz() {
        let src = "\
for n in range(1,16) {
    if n % 15 == 0 { print \"FizzBuzz\" }
    else if n % 3 == 0 { print \"Fizz\" }
    else if n % 5 == 0 { print \"Buzz\" }
    else { print n }
}";
        let expected: Vec<&str> = vec![
            "1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz",
            "11","Fizz","13","14","FizzBuzz",
        ];
        assert_eq!(ok(src), expected);
    }

    // ── v4.0.0: assert ────────────────────────────────────────────────────

    #[test]
    fn assert_passes() {
        assert_eq!(ok("assert(1 == 1, \"math broken\")\nprint \"ok\""), vec!["ok"]);
    }

    #[test]
    fn assert_passes_no_message() {
        assert_eq!(ok("assert(true)\nprint \"ok\""), vec!["ok"]);
    }

    #[test]
    fn assert_fails_with_message() {
        let e = err_msg("assert(1 == 2, \"one is not two\")");
        assert!(e.contains("one is not two"), "got: {}", e);
        assert!(e.contains("Assertion failed"), "got: {}", e);
    }

    #[test]
    fn assert_fails_default_message() {
        let e = err_msg("assert(false)");
        assert!(e.contains("Assertion failed"), "got: {}", e);
    }

    #[test]
    fn assert_falsy_values() {
        assert!(err_msg("assert(0)").contains("Assertion failed"));
        assert!(err_msg("assert(\"\")").contains("Assertion failed"));
        assert!(err_msg("assert([])").contains("Assertion failed"));
    }

    // ── v4.0.0: type_of ───────────────────────────────────────────────────

    #[test]
    fn type_of_primitives() {
        assert_eq!(ok("print type_of(42)"),      vec!["number"]);
        assert_eq!(ok("print type_of(3.14)"),    vec!["number"]);
        assert_eq!(ok("print type_of(\"hi\")"),  vec!["string"]);
        assert_eq!(ok("print type_of(true)"),    vec!["bool"]);
        assert_eq!(ok("print type_of(false)"),   vec!["bool"]);
    }

    #[test]
    fn type_of_collections() {
        assert_eq!(ok("print type_of([1,2,3])"),       vec!["array"]);
        assert_eq!(ok("print type_of({\"a\":1})"),     vec!["dict"]);
    }

    #[test]
    fn type_of_none() {
        assert_eq!(ok("fn f() {}\nprint type_of(f())"), vec!["none"]);
    }

    #[test]
    fn type_of_in_condition() {
        let src = "\
fn safe_double(x) {
    if type_of(x) != \"number\" { return \"error\" }
    return x * 2
}
print safe_double(5)
print safe_double(\"hi\")";
        assert_eq!(ok(src), vec!["10", "error"]);
    }

    // ── v4.0.0: exit ──────────────────────────────────────────────────────

    #[test]
    fn exit_stops_execution() {
        // exit() raises Exit error — run_capturing propagates it as a string
        let result = run_capturing("print \"before\"\nexit(0)\nprint \"after\"");
        // "before" should have been printed before exit fires
        // The error string should contain "exit(0)"
        match result {
            Ok(lines) => panic!("expected exit error, got {:?}", lines),
            Err(e)    => assert!(e.contains("exit(0)"), "got: {}", e),
        }
    }

    #[test]
    fn exit_with_code() {
        let result = run_capturing("exit(1)");
        match result {
            Ok(_)  => panic!("expected exit error"),
            Err(e) => assert!(e.contains("exit(1)"), "got: {}", e),
        }
    }
}