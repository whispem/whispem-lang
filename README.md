# Whispem

![Logo Whispem](https://imgur.com/YDjrAKR.png)

[![Version](https://img.shields.io/badge/version-5.0.0-cyan.svg)](https://github.com/whispem/whispem-lang/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Tests](https://img.shields.io/badge/tests-167%20passing-brightgreen.svg)

> *Whisper your intent. The machine listens.*

Whispem is a small, self-hosted programming language. The compiler is written in Whispem, compiles itself, and runs on a standalone C VM — no external dependencies beyond a C compiler. Rust serves as the reference implementation.

**Current version: 5.0.0** — closures · lambdas · f-strings · shared mutable upvalues · 130 Rust tests · 37 autonomous tests · zero warnings

---

## Quick start

```bash
cargo build --release
cargo run -- examples/hello.wsp
cargo run -- --compile examples/hello.wsp   # → examples/hello.whbc
cargo run -- --dump examples/hello.wsp
cargo run                                    # REPL
cargo test                                   # 130 Rust tests
./tests/run_tests.sh                         # 37 autonomous tests
```

> **Note:** The `.whbc` format changed in v5 (version byte `0x04`). Recompile any v4 bytecode files from source.

---

## What's new in v5.0.0

### Closures

Functions now capture variables from their enclosing scope. Captured variables are
shared — multiple closures created in the same scope see each other's mutations.

```wsp
fn make_counter() {
    let count = 0
    return fn() {
        let count = count + 1
        return count
    }
}
let c = make_counter()
print c()   # 1
print c()   # 2
print c()   # 3
```

```wsp
fn make_adder(n) {
    return fn(x) { return x + n }
}
let add5  = make_adder(5)
let add10 = make_adder(10)
print add5(3)    # 8
print add10(3)   # 13
```

### Lambdas

`fn(params) { body }` is now a first-class expression. Store it, pass it, return it.

```wsp
fn apply(f, x) { return f(x) }
print apply(fn(n) { return n * n }, 5)   # 25

let fns = [fn(x) { return x + 1 }, fn(x) { return x * 2 }]
print fns[0](10)   # 11
print fns[1](10)   # 20

print fn(x) { return x * 2 }(7)   # 14  (immediate call)
```

### F-strings

`f"..."` with `{expr}` interpolation. Any expression is valid inside braces.
Compiles to a chain of string concatenations — identical performance to `+`.

```wsp
let name  = "Em"
let score = 42
print f"Hello, {name}!"
print f"Score: {score}, doubled: {score * 2}"
print f"{length([1, 2, 3])} items"
```

---

## The language

```wsp
# Variables
let name = "Em"
let age  = 26

# Print
print f"Hello, {name}"

# Conditionals
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
for fruit in ["apple", "banana", "cherry"] {
    print fruit
}

# Functions
fn greet(person) {
    return f"Hello, {person}!"
}
print greet("world")

# Lambdas
let double = fn(x) { return x * 2 }
print double(7)   # 14

# Closures
fn make_adder(n) {
    return fn(x) { return x + n }
}
print make_adder(5)(3)   # 8

# Higher-order functions
fn apply(f, x) { return f(x) }
print apply(fn(n) { return n * n }, 6)   # 36

# Arrays
let nums = [1, 2, 3, 4, 5]
let nums = push(nums, 6)
print length(nums)   # 6

# Dicts
let person = {"name": "Em", "age": 26}
print person["name"]

# assert and type_of
assert(length(nums) > 0, "nums must not be empty")
assert(type_of(double) == "function")
print type_of(42)     # number
print type_of(double) # function
```

---

## Architecture

```
source code (.wsp)
    ↓  src/compiler.rs (Rust reference)
bytecode (.whbc)
    ↓  src/vm.rs (Rust reference)
output
```

The VM is a stack machine with **38 opcodes**. New in v5:

| Opcode | Code | Description |
|--------|------|-------------|
| `LOAD_UPVALUE` | `0x13` | Push upvalue from closure's upvalue list |
| `STORE_UPVALUE` | `0x14` | Write through shared upvalue cell |
| `CLOSE_UPVALUE` | `0x15` | Close upvalue onto heap (reserved) |
| `MAKE_CLOSURE` | `0x53` | Create `Value::Closure` with captured upvalues |

`--dump` disassembles all chunks including closures and lambdas:

```
== make_adder ==
0000     1  STORE                0    'n'
0002     2  MAKE_CLOSURE         1    '__lambda_2_0' (1 upvalues)
              [is_local=1 name='n']
0009     2  RETURN

== __lambda_2_0 ==
0000     2  STORE                0    'x'
0002     2  LOAD_UPVALUE         0
0004     2  LOAD                 0    'x'
0006     2  ADD
0007     2  RETURN
```

See [`docs/vm.md`](docs/vm.md) for the complete VM specification.

---

## CLI

```bash
whispem                          # interactive REPL
whispem file.wsp                 # run source file
whispem file.wsp arg1 arg2       # run with script arguments
whispem --dump file.wsp          # disassemble
whispem --compile file.wsp       # compile to file.whbc
whispem file.whbc                # run precompiled bytecode
```

---

## Testing

```bash
cargo test             # 130 Rust tests
./tests/run_tests.sh   # 37 autonomous tests
./tests/run_tests.sh   # 37 autonomous tests (C VM only, no Rust needed)
```

130 Rust tests cover: arithmetic, strings, booleans, comparisons, logic, control flow,
functions, recursion, forward calls, arrays, dictionaries, truthiness, error spans,
integration programs, bytecode round-trips, all v4 features, and all v5 features
(f-strings, lambdas, closures, shared mutable state, nested closures, independent
closure instances).

### Autonomous test suite setup

```bash
make                                               # build wvm if needed
cargo run --release -- --compile compiler/wsc.wsp  # recompile self-hosted compiler
./tests/run_tests.sh                               # 37 tests + bootstrap
```

The bootstrap test compiles `wsc.wsp` twice and verifies gen1 == gen2 (fixed point).
It works regardless of whether `wsc.whbc` was produced by Rust or by itself.

---

## Project layout

```
whispem/
├── src/
│   ├── main.rs        entry point · CLI · 130 Rust tests · 37 autonomous tests
│   ├── repl.rs        interactive REPL
│   ├── lexer.rs       tokeniser — else if collapse, f-string lexing
│   ├── token.rs       token types — FStr, ElseIf, Assert, TypeOf, Exit
│   ├── parser.rs      recursive descent — lambdas, f-string desugaring, CallExpr
│   ├── ast.rs         AST — Lambda, CallExpr, FStr, FStrPart
│   ├── error.rs       WhispemError · ErrorKind · Span
│   ├── value.rs       runtime values — Closure, Upvalue
│   ├── opcode.rs      38 opcodes
│   ├── chunk.rs       Chunk · serialise · deserialise · disassembler
│   ├── compiler.rs    AST → bytecode — upvalue analysis, closure compilation
│   └── vm.rs          VM loop · builtins · closure dispatch · upvalue cells
├── examples/          30+ example programs
├── docs/              vm.md · syntax.md · tutorial.md · examples.md · vision.md
├── Makefile           builds wvm from vm/wvm.c
├── CHANGELOG.md
└── README.md
```

---

## Roadmap

| Version | Goal |
|---------|------|
| [x] 1.5.0 | Tree-walking interpreter, full language, REPL |
| [x] 2.0.0 | Bytecode VM, compiler, `--dump` |
| [x] 2.5.0 | Error spans, arity checking, 72 tests, 0 warnings |
| [x] 3.0.0 | `.whbc` serialisation, self-hosted compiler, C VM, 125 tests |
| [x] 4.0.0 | `else if`, `assert`, `type_of`, `exit`, 147 tests |
| [x] 5.0.0 | Closures, lambdas, f-strings, 130 Rust + 37 autonomous tests |
| 6.0.0 | f-strings + closures in self-hosted compiler and C VM; `map` / `filter` / `reduce` |

---

## Philosophy

Whispem is intentionally small. The goal is a language whose entire implementation
can be read and understood in a single sitting. No magic, no hidden complexity.

In v5, the language became genuinely functional: you can pass functions, return
functions, and close over mutable state. The implementation stays readable —
the upvalue machinery is ~80 lines of compiler code and ~60 lines of VM code.

---

*Whispem v5.0.0 — Closures. Lambdas. F-strings.*