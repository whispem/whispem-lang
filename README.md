# Whispem

![Logo Whispem](https://imgur.com/YDjrAKR.png)

[![Version](https://img.shields.io/badge/version-2.0.0-cyan.svg)](https://github.com/whispem/whispem-lang/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> *Whisper your intent. The machine listens.*

Whispem is a small, readable programming language written in Rust.  
It is designed to be learnable in an afternoon and understandable in its entirety — including its own implementation.

**Current version: 2.0.0** — bytecode VM

---

## Quick start

```bash
# Run a file
cargo run -- examples/hello.wsp

# Interactive REPL
cargo run

# Inspect compiled bytecode
cargo run -- --dump examples/fizzbuzz.wsp
```

---

## The language

```wsp
# Variables
let name = "Em"
let age  = 25

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

# Functions
fn greet(person) {
    return "Hello, " + person + "!"
}
print greet("world")

# Arrays
let nums = [1, 2, 3, 4, 5]
let nums = push(nums, 6)
print length(nums)       # 6

# Dicts
let person = {"name": "Em", "age": 25}
print person["name"]
print has_key(person, "email")    # false

# Index assignment
let scores = [10, 20, 30]
scores[1] = 99
print scores                       # [10, 99, 30]
```

---

## Language reference

### Types

| Type     | Examples                        |
|----------|---------------------------------|
| `number` | `42`, `3.14`, `-7`             |
| `string` | `"hello"`, `""`                |
| `bool`   | `true`, `false`                |
| `array`  | `[1, "two", true]`             |
| `dict`   | `{"key": "value", "n": 42}`    |
| `none`   | returned by void functions      |

### Operators

```wsp
# Arithmetic
a + b   a - b   a * b   a / b   a % b

# Comparison
a == b   a != b   a < b   a <= b   a > b   a >= b

# Logic
a and b   a or b   not a

# String concatenation
"Hello" + " " + "world"
```

### Control flow

```wsp
# if / else
if condition {
    ...
} else {
    ...
}

# while
while condition {
    ...
}

# for
for item in collection {
    ...
}

# break / continue
while true {
    if done { break }
    if skip { continue }
    ...
}
```

### Functions

```wsp
fn add(a, b) {
    return a + b
}

# Functions are called by name
print add(3, 4)      # 7

# Recursive functions work
fn factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
```

### Built-in functions

| Function              | Description                                     |
|-----------------------|-------------------------------------------------|
| `length(x)`           | Length of array, string, or dict               |
| `push(arr, val)`      | Return new array with val appended             |
| `pop(arr)`            | Return last element (error if empty)           |
| `reverse(arr)`        | Return new reversed array                      |
| `slice(arr, s, e)`    | Sub-array `[s, e)`                             |
| `range(start, end)`   | Array of integers `[start, end)`               |
| `input(prompt?)`      | Read a line from stdin                         |
| `read_file(path)`     | Read file to string                            |
| `write_file(path, s)` | Write string to file                           |
| `keys(dict)`          | Sorted list of keys                            |
| `values(dict)`        | Values in key-sorted order                     |
| `has_key(dict, key)`  | Check if key exists                            |

### Comments

```wsp
# This is a comment
let x = 42   # inline comment
```

---

## Architecture

Whispem v2.0.0 uses a **bytecode virtual machine**:

```
source code
    ↓  Lexer     src/lexer.rs
tokens
    ↓  Parser    src/parser.rs
AST
    ↓  Compiler  src/compiler.rs
bytecode chunks
    ↓  VM        src/vm.rs
output
```

The VM is a stack machine with 31 opcodes. Every `fn` declaration compiles to its own `Chunk`. The `--dump` flag disassembles all chunks:

```
== <main> ==
0000     1  PUSH_CONST       1    '7'
0002     1  CALL             0    'double' (1 args)
0005     1  PRINT
0006     1  HALT

== double ==
0000     1  STORE            0    'n'
0002     2  LOAD             0    'n'
0004     2  PUSH_CONST       1    '2'
0006     2  MUL
0007     2  RETURN
0008     2  RETURN_NONE
```

See [`docs/vm.md`](docs/vm.md) for the complete VM specification.

---

## Project layout

```
whispem/
├── src/
│   ├── main.rs        entry point + CLI
│   ├── repl.rs        interactive REPL
│   ├── lexer.rs       tokeniser
│   ├── token.rs       token types
│   ├── parser.rs      recursive descent parser
│   ├── ast.rs         AST node types
│   ├── error.rs       error types
│   ├── value.rs       runtime value types
│   ├── opcode.rs      VM instruction set
│   ├── chunk.rs       bytecode chunk + disassembler
│   ├── compiler.rs    AST → bytecode compiler
│   └── vm.rs          VM execution loop + built-ins
├── docs/
│   └── vm.md          VM specification
├── examples/
│   ├── hello.wsp
│   ├── fizzbuzz.wsp
│   └── ...
├── CHANGELOG.md
└── README.md
```

---

## Roadmap

| Version | Goal                                                             |
|---------|------------------------------------------------------------------|
| [x] 1.5.0 | Tree-walking interpreter, full language, REPL                  |
| [x] 2.0.0 | Bytecode VM, compiler, `--dump`, `docs/vm.md`                  |
| 2.5.0   | Bytecode serialisation, richer error spans, test suite          |
| 3.0.0   | Self-hosting: Whispem compiler written in Whispem               |

---

## Philosophy

Whispem is intentionally small. The goal is a language whose entire implementation — lexer, parser, compiler, VM — can be read and understood in a single sitting. No magic, no hidden complexity.

Every design decision asks: *would a future Whispem program be able to do this too?*

---

*Whispem v2.0.0 — Simple. Explicit. Bootstrappable.*
