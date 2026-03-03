# Whispem

![Logo Whispem](https://imgur.com/YDjrAKR.png)

[![Version](https://img.shields.io/badge/version-3.0.0-cyan.svg)](https://github.com/whispem/whispem-lang/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tests](https://img.shields.io/badge/tests-125%20passing-brightgreen.svg)](https://github.com/whispem/whispem-lang/actions)

> *Whisper your intent. The machine listens.*

Whispem is a small, self-hosted programming language. The compiler is written in Whispem, compiles itself, and runs on a standalone C VM — no external dependencies beyond a C compiler. Rust serves as the reference implementation.

**Current version: 3.0.0** — self-hosted compiler · verified bootstrap · `.whbc` bytecode · standalone C VM with REPL and disassembler · 125 tests (93 + 32 autonomous) · zero warnings

---

## Quick start

The only dependency is a C compiler.

```bash
make                                         # build the VM from vm/wvm.c
./wvm compiler/wsc.whbc examples/hello.wsp   # compile + run a source file
./wvm examples/hello.whbc                    # run precompiled bytecode
./wvm --dump examples/hello.whbc             # inspect bytecode
./wvm                                        # interactive REPL
./tests/run_tests.sh                         # run the full test suite (32 tests)
```

### With the Rust reference implementation

```bash
cargo build --release
cargo run -- examples/hello.wsp              # run source file
cargo run -- --compile examples/hello.wsp    # compile to .whbc
cargo run -- --dump examples/hello.wsp       # disassemble
cargo run                                    # REPL
cargo test                                   # 93 tests
```

---

## What's new in v3.0.0

### Self-hosted compiler

`compiler/wsc.wsp` — 1618 lines of Whispem that implement the full compilation pipeline: lexer, recursive-descent parser, bytecode compiler, and binary serialiser. Source file goes in, `.whbc` bytecode comes out — byte-for-byte identical to the Rust compiler's output.

```bash
./wvm compiler/wsc.whbc examples/hello.wsp   # compile hello.wsp → hello.whbc
./wvm examples/hello.whbc                    # → Hello, Whispem!
```

### Verified bootstrap

The compiler compiles itself. The result compiles itself again. Both outputs are bit-identical:

```bash
./wvm compiler/wsc.whbc compiler/wsc.wsp    # gen1: compiler compiles itself
shasum compiler/wsc.whbc
# f090aa0f650a3b00e4286b332f82c0ba5c3b71d5

./wvm compiler/wsc.whbc compiler/wsc.wsp    # gen2: again
shasum compiler/wsc.whbc
# f090aa0f650a3b00e4286b332f82c0ba5c3b71d5  ← stable fixed point
```

The same SHA-1 holds on the Rust VM — the two runtimes are interchangeable.

### Standalone C VM

`vm/wvm.c` — a single-file C runtime (~2000 lines) that executes `.whbc` bytecode. Same 34 opcodes and 20 builtins as the Rust VM, with refcounted copy-on-write arrays and dicts. Includes an interactive REPL and a `--dump` disassembler. Byte-for-byte identical output on all programs.

### `.whbc` bytecode format

Binary format (magic `WHBC` + version byte), length-prefixed, with line numbers preserved for error reporting. Compile once, run anywhere:

```bash
./wvm compiler/wsc.whbc examples/fizzbuzz_proper.wsp   # compile
./wvm examples/fizzbuzz_proper.whbc                     # run — no recompilation
```

### `LOAD_GLOBAL` opcode

Functions emit explicit `LOAD_GLOBAL` instructions for global variable reads. The bytecode is self-describing: the opcode tells you whether a read is local or global.

---

## The language

```wsp
# Variables
let name = "Em"
let age  = 26

# Print
print "Hello, " + name

# Conditionals
if age >= 18 {
    print "adult"
} else {
    print "minor"
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

# Functions — with arity checking
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

# Short-circuit logic
let r = false and expensive_call()   # call never runs
let r = true  or  expensive_call()   # call never runs
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

See [`docs/vm.md`](docs/vm.md) for the complete VM specification including the `.whbc` binary format.

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
./tests/run_tests.sh             # autonomous test suite (32 tests)

# Rust reference implementation
whispem                          # interactive REPL
whispem file.wsp                 # run source file
whispem file.wsp arg1 arg2       # run with script arguments
whispem --dump file.wsp          # disassemble without running
whispem --compile file.wsp       # compile to file.whbc
whispem --compile file.wsp out.whbc   # compile with explicit output path
whispem file.whbc                # run precompiled bytecode
```

---

## Testing

```bash
./tests/run_tests.sh          # autonomous tests (no Rust needed, 32 tests)
cargo test                    # Rust reference tests (93 tests)
```

32 autonomous tests using only the C VM and the self-hosted compiler: compiles each `.wsp` example, runs the result, compares output to expected baselines, and verifies bootstrap stability.

93 Rust tests covering the entire language: arithmetic, strings, booleans, comparisons, logic, control flow, functions, recursion, forward calls, arrays, dictionaries, truthiness, error spans, integration programs (FizzBuzz, word counter, Fibonacci), and **13 bytecode round-trip tests** (v3.0.0).

---

## Project layout

```
whispem/
├── compiler/
│   ├── wsc.wsp        self-hosted compiler (1618 lines of Whispem)
│   └── wsc.whbc       bootstrapped bytecode (SHA-1 f090aa0...)
├── vm/
│   └── wvm.c          standalone C runtime (~2000 lines, no Rust needed)
├── src/
│   ├── main.rs        entry point · CLI · 93 Rust tests
│   ├── repl.rs        interactive REPL
│   ├── lexer.rs       tokeniser
│   ├── token.rs       token types
│   ├── parser.rs      recursive descent parser
│   ├── ast.rs         AST node types
│   ├── error.rs       WhispemError · ErrorKind · Span
│   ├── value.rs       runtime value types (Rc copy-on-write)
│   ├── opcode.rs      VM instruction set (34 opcodes)
│   ├── chunk.rs       Chunk · serialise · deserialise · disassembler
│   ├── compiler.rs    AST → bytecode · LOAD_GLOBAL emission
│   └── vm.rs          VM loop · LOAD_GLOBAL · injectable output
├── tests/
│   ├── run_tests.sh   autonomous test runner (C VM only)
│   └── expected/      expected output for each example
├── examples/
│   ├── hello.wsp
│   ├── fizzbuzz_proper.wsp
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
| 4.0.0 | `else if` syntax sugar, closures, column numbers in errors |

---

## Philosophy

Whispem is intentionally small. The goal is a language whose entire implementation — lexer, parser, compiler, VM — can be read and understood in a single sitting. No magic, no hidden complexity.

Every design decision asks: *would a future Whispem program be able to do this too?*

In v3.0.0, the answer became yes — the compiler is written in Whispem, it compiles itself to `.whbc` bytecode, and the standalone C runtime executes it. No Rust needed. The language hosts itself.

---

*Whispem v3.0.0 — Self-hosted. Standalone. Bootstrappable.*
