# Whispem

![Logo Whispem](https://imgur.com/YDjrAKR.png)

[![Version](https://img.shields.io/badge/version-4.0.0-cyan.svg)](https://github.com/whispem/whispem-lang/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Tests](https://img.shields.io/badge/tests-147%20passing-brightgreen.svg)](https://github.com/whispem/whispem-lang/actions)

> *Whisper your intent. The machine listens.*

Whispem is a small, self-hosted programming language. The compiler is written in Whispem, compiles itself, and runs on a standalone C VM — no external dependencies beyond a C compiler. Rust serves as the reference implementation.

**Current version: 4.0.0** — `else if` · `assert` · `type_of` · `exit` · self-hosted compiler · verified bootstrap · `.whbc` bytecode · standalone C VM · 147 tests (110 Rust + 37 autonomous) · zero warnings

---

## Quick start

The only dependency is a C compiler.

```bash
make                                         # build the VM from vm/wvm.c
./wvm compiler/wsc.whbc examples/hello.wsp   # compile + run a source file
./wvm examples/hello.whbc                    # run precompiled bytecode
./wvm --dump examples/hello.whbc             # inspect bytecode
./wvm                                        # interactive REPL
./tests/run_tests.sh                         # run the autonomous test suite
```

### With the Rust reference implementation

```bash
cargo build --release
cargo run -- examples/hello.wsp              # run source file
cargo run -- --compile examples/hello.wsp    # compile to .whbc
cargo run -- --dump examples/hello.wsp       # disassemble
cargo run                                    # REPL
cargo test                                   # 110 Rust tests
```

---

## What's new in v4.0.0

### `else if`

`else if` is now proper syntax — no more nesting `if` inside `else`:

```wsp
if score >= 90 { print "A" }
else if score >= 80 { print "B" }
else if score >= 70 { print "C" }
else { print "F" }
```

Pure syntax sugar: the lexer collapses `else if` into a single token, the parser builds the same nested `If` AST nodes as before. Zero VM or bytecode impact.

### `assert(condition, message?)`

```wsp
assert(length(items) > 0, "list must not be empty")
assert(type_of(x) == "number")
```

Raises `Assertion failed: <message>` if the condition is falsy. Message is optional.

### `type_of(value)`

Returns the runtime type as a string: `"number"`, `"string"`, `"bool"`, `"array"`, `"dict"`, or `"none"`.

```wsp
fn safe_double(x) {
    if type_of(x) != "number" { return "error: expected number" }
    return x * 2
}
```

### `exit(code?)`

```wsp
if length(args()) == 0 {
    print "Usage: script.wsp <file>"
    exit(1)
}
```

Terminates with the given exit code (default `0`). Propagates cleanly through the call stack; the CLI passes the code to the OS rather than printing an error.

### Clearer dict error messages

Accessing a missing key now reads `key "foo" not found in dict` instead of the cryptic `undefined variable 'dict key "foo"'`.

---

## The language

```wsp
# Variables
let name = "Em"
let age  = 26

# Print
print "Hello, " + name

# Conditionals — now with else if
if age >= 18 {
    print "adult"
} else if age >= 13 {
    print "teen"
} else {
    print "child"
}

# While loop
let i = 0
while i < 5 {
    print i
    let i = i + 1
}

# For loop
let fruits = ["apple", "banana", "cherry"]
for fruit in fruits {
    print fruit
}

# Functions
fn greet(person) {
    return "Hello, " + person + "!"
}
print greet("world")

# Forward calls work
print triple(4)   # 12

fn triple(n) {
    return n * 3
}

# Arrays
let nums = [1, 2, 3, 4, 5]
let nums = push(nums, 6)
print length(nums)   # 6

# Dicts
let person = {"name": "Em", "age": 26}
print person["name"]
print has_key(person, "email")   # false

# Index assignment
let scores = [10, 20, 30]
scores[1] = 99
print scores   # [10, 99, 30]

# assert and type_of
assert(length(nums) > 0, "nums must not be empty")
print type_of(nums)   # array

# exit
if length(args()) == 0 {
    exit(1)
}
```

---

## Architecture

```
source code (.wsp)
    ↓  compiler/wsc.wsp (self-hosted)  or  src/compiler.rs (Rust)
bytecode (.whbc)
    ↓  vm/wvm.c (standalone)  or  src/vm.rs (Rust)
output
```

The VM is a stack machine with **34 opcodes**. The `--dump` flag disassembles all chunks:

```
== <main> ==
0000     1  PUSH_CONST           1    '7'
0003     1  CALL                 0    'double' (1 args)
0006     1  PRINT
0007     1  HALT

== double ==
0000     1  STORE                0    'n'
0002     2  LOAD                 0    'n'
0004     2  PUSH_CONST           1    '2'
0006     2  MUL
0007     2  RETURN
0008     2  RETURN_NONE
```

See [`docs/vm.md`](docs/vm.md) for the complete VM specification and the `.whbc` binary format.

---

## CLI

```bash
# Standalone toolchain (no Rust needed)
make                             # build wvm from vm/wvm.c
./wvm                            # interactive REPL
./wvm file.whbc                  # run precompiled bytecode
./wvm file.whbc arg1 arg2        # run with script arguments
./wvm compiler/wsc.whbc file.wsp # compile .wsp → .whbc, then run
./wvm --dump file.whbc           # disassemble without running
./tests/run_tests.sh             # autonomous test suite

# Rust reference implementation
whispem                          # interactive REPL
whispem file.wsp                 # run source file
whispem file.wsp arg1 arg2       # run with script arguments
whispem --dump file.wsp          # disassemble without running
whispem --compile file.wsp       # compile to file.whbc
whispem --compile file.wsp out.whbc
whispem file.whbc                # run precompiled bytecode
```

---

## Testing

```bash
./tests/run_tests.sh   # autonomous tests (no Rust needed)
cargo test             # 110 Rust tests
```

110 Rust tests cover the entire language: arithmetic, strings, booleans, comparisons, logic, control flow, functions, recursion, forward calls, arrays, dictionaries, truthiness, error spans, integration programs, bytecode round-trip tests, and all v4.0.0 features. 38 autonomous tests run via the C VM only, covering all example programs and bootstrap verification.

---

## Project layout

```
whispem/
├── compiler/
│   ├── wsc.wsp        self-hosted compiler (1724 lines of Whispem)
│   └── wsc.whbc       bootstrapped bytecode
├── vm/
│   └── wvm.c          standalone C runtime (~2000 lines, no Rust needed)
├── src/
│   ├── main.rs        entry point · CLI · 110 Rust tests
│   ├── repl.rs        interactive REPL
│   ├── lexer.rs       tokeniser — collapses else if
│   ├── token.rs       token types
│   ├── parser.rs      recursive descent parser — else if, assert, type_of, exit
│   ├── ast.rs         AST node types
│   ├── error.rs       WhispemError · ErrorKind · Span
│   ├── value.rs       runtime value types (Rc copy-on-write)
│   ├── opcode.rs      VM instruction set (34 opcodes)
│   ├── chunk.rs       Chunk · serialise · deserialise · disassembler
│   ├── compiler.rs    AST → bytecode
│   └── vm.rs          VM loop · builtins · assert · type_of · exit
├── tests/
│   ├── run_tests.sh   autonomous test runner (C VM only)
│   └── expected/      expected output for each example
├── examples/
│   └── ...            30+ example programs
├── docs/
│   ├── vm.md          VM spec · .whbc format
│   ├── syntax.md      language syntax reference
│   ├── tutorial.md    full language tutorial
│   ├── examples.md    annotated examples
│   ├── vision.md      design philosophy and roadmap
│   └── journey.md     the story from literature to self-hosting
├── Makefile           builds wvm from vm/wvm.c
├── CHANGELOG.md
└── README.md
```

---

## Roadmap

| Version | Goal |
|---------|------|
| [x] 1.5.0 | Tree-walking interpreter, full language, REPL |
| [x] 2.0.0 | Bytecode VM, compiler, `--dump`, `docs/vm.md` |
| [x] 2.5.0 | Error spans, arity, short-circuit fix, 72 tests, 0 warnings |
| [x] 3.0.0 | `.whbc` serialisation, `LOAD_GLOBAL`, `--compile`, self-hosted compiler, verified bootstrap, Rc COW, standalone C VM, 125 tests |
| [x] 4.0.0 | `else if`, `assert`, `type_of`, `exit`, dict error messages, 147 tests (110 Rust + 37 autonomous) |
| 5.0.0 | Closures, string interpolation |

---

## Philosophy

Whispem is intentionally small. The goal is a language whose entire implementation — lexer, parser, compiler, VM — can be read and understood in a single sitting. No magic, no hidden complexity.

Every design decision asks: *would a future Whispem program be able to do this too?*

In v3.0.0, the answer became yes — the compiler is written in Whispem, it compiles itself, and the standalone C runtime executes it. In v4.0.0, the language became a little cleaner to write: `else if` instead of nested braces, `assert` for correctness checks, `type_of` for defensive code, `exit` for scripts.

---

*Whispem v4.0.0 — Self-hosted. Standalone. Bootstrappable.*