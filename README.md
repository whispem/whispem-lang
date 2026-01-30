# Whispem

**Whispem** is a minimal interpreted programming language designed for clarity, simplicity, and experimentation.  
It features a clean syntax, variable bindings, arithmetic expressions with operator precedence, and explicit grouping using parentheses.

> Whispem is implemented in Rust, but **Whispem itself is a standalone language**, independent of Rust in its design and usage.

---

## Features

- Simple and readable syntax
- Variable declarations with `let`
- Arithmetic expressions
  - `+`, `-`, `*`, `/`
  - Correct operator precedence
  - Explicit grouping with parentheses `( )`
- String and number literals
- `print` statement
- Line-based syntax
- Comments using `#`
- Deterministic execution model

---

## Example

### `examples/hello.wsp`

```wsp
# Whispem v0.5.0 example

let x = (10 + 5) * 2
print x
```

### Output

```text
30
```

---

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

### Parentheses

```wsp
let value = (10 + 5) * 2
print value
```

Multiplication and division have higher precedence than addition and subtraction.  
Parentheses allow explicit grouping of expressions.

### Printing

```wsp
print "Hello, world!"
print value
```

### Comments

```wsp
# This is a comment
let x = 1
```

---

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

---

## Project Structure

```text
whispem-lang/
├── src/
│   ├── ast.rs          # Abstract Syntax Tree definitions
│   ├── interpreter.rs # Expression and statement evaluator
│   ├── lexer.rs       # Tokenizer
│   ├── parser.rs      # Parser with operator precedence and parentheses
│   ├── token.rs       # Token definitions
│   └── main.rs        # Entry point
├── examples/
│   └── hello.wsp
├── CHANGELOG.md
├── Cargo.toml
└── README.md
```

---

## Current Status

**Version:** `0.5.0`

Whispem is in early development but already provides a solid and consistent core language.  
Expressions, operator precedence, and grouping are fully supported.

This project is suitable for:
- Learning how programming languages work
- Experimenting with interpreters and parsers
- Educational and exploratory use

---

## Roadmap

Planned future features include:

- Boolean values and comparisons
- Conditional statements (`if`)
- Functions
- Variable scope
- Error diagnostics and reporting
- Extended standard library

---

## License

This project is open-source and available under the MIT License.

---

## 💜 Author

Created with curiosity and intent by **Emilie**.

> Whispem is small by design — every feature exists because it is understood.
