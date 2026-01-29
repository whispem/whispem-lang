# Whispem

Whispem is an experimental programming language built as a personal exploration
of language design and interpretation.

The goal of Whispem is to understand how a programming language works internally,
from source code to execution, while remaining small, readable, and deliberate.

---

## Status

- Version: v0.3.0
- Stability: Experimental
- Project type: Personal language project
- Implementation language: Rust

---

## Features (v0.3.0)

- Variable declarations using `let`
- Immutable variables
- Numeric values
- `print` statement
- Line-based syntax
- Comments using `#`
- Execution of source files (`.wsp`)
- Direct interpretation (no compilation step)

---

## Example

Source file:

    let x = 10
    print x

Command:

    cargo run examples/hello.wsp

Output:

    10

---

## How It Works

Whispem follows a classic language pipeline:

1. Read source file
2. Lexical analysis (tokens)
3. Parsing (AST)
4. Interpretation (execution)

Each step is implemented explicitly and kept minimal.

---

## Limitations

Whispem v0.3.0 does not support:

- Arithmetic expressions
- Conditionals
- Loops
- Functions
- Advanced error reporting

These limitations are intentional.

---

## Vision

Whispem is designed to remain understandable in its entirety.

Each version adds one small, deliberate improvement, prioritizing learning
and clarity over power or completeness.
