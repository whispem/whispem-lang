# Whispem

**Whispem** is a minimal interpreted programming language designed for clarity, simplicity, and experimentation.  
It features a clean syntax, variable bindings, arithmetic expressions with proper operator precedence, and basic output capabilities.

> Whispem is implemented in Rust, but **Whispem itself is a standalone language**, independent of Rust in its design and philosophy.

---

## Features

- Simple and readable syntax
- Variable declarations with `let`
- Arithmetic expressions
  - `+`, `-`, `*`, `/`
  - Correct operator precedence
- String and number literals
- `print` statement
- Line-based syntax
- Comments using `#`
- Deterministic execution model

---

## Example

### `examples/hello.wsp`

```wsp
# Whispem v0.4.0 example

let x = 10
let y = x + 5 * 2
print y

### Output

```text
20
```

## Language Overview

### Variables

```wsp
let a = 42
let name = "Whispem"
```

### Arithmetic Expressions

```wsp
let result = 10 + 5 * 2
print result
```

Multiplication and division have higher precedence than addition and subtraction.

### Printing

```wsp
print "Hello, world!"
print result
```

### Comments

```wsp
# This is a comment
let x = 1
```

## Getting Started

### Prerequisites

- Rust (stable)
- Cargo

### Clone the repository

```bash
git clone https://github.com/whispem/whispem-lang.git
cd whispem-lang
```

### Build the project

```bash
cargo build
```

### Run a Whispem file

```bash
cargo run examples/hello.wsp
```

## Project Structure

```text
whispem-lang/
├── src/
│   ├── ast.rs          # Abstract Syntax Tree definitions
│   ├── interpreter.rs # Expression and statement evaluator
│   ├── lexer.rs       # Tokenizer
│   ├── parser.rs      # Parser with operator precedence
│   ├── token.rs       # Token definitions
│   └── main.rs        # Entry point
├── examples/
│   └── hello.wsp
├── CHANGELOG.md
├── Cargo.toml
└── README.md
```

## Current Status

**Version:** `0.4.0`

Whispem is currently in early development.  
The core language features are stable, and the interpreter is fully functional for small programs.

This project is suitable for:
- Learning how programming languages work
- Experimenting with interpreters and parsers
- Educational and exploratory use

## Roadmap

Planned future features include:

- Parentheses in expressions
- Boolean values and comparisons
- Conditional statements (`if`)
- Functions
- Error diagnostics and reporting
- Extended standard library

## License

This project is open-source and available under the MIT License.

## 💜 Author

Created with curiosity and intent by **Emilie**.

> Whispem is small by design — every feature exists because it is understood.
