# Whispem

Whispem is an experimental programming language created as a personal exploration
of language design and implementation.

The project focuses on understanding how a programming language works internally:
from reading source code, to interpreting its meaning, to executing instructions.

Whispem prioritizes clarity, simplicity, and intention over performance or feature
completeness.

---

## Status

- Version: v0.2.0
- Stability: Experimental
- Project type: Personal language experiment
- Implementation language: Rust

---

## Philosophy

Whispem is built on a few simple ideas:

- Code should be easy to read and reason about
- The language should be small enough to understand entirely
- Every feature must be intentional
- Learning is more important than optimization

Whispem is not meant to replace existing languages.
It exists to explore how languages are made.

---

## Language Features (v0.2.0)

- Variable declarations with `let`
- Immutable variables
- Numeric values
- String literals
- A `print` statement
- Line-based syntax
- Comments using `#`
- Direct interpretation (no compilation step)

---

## Example

```whispem
# Whispem example
let name = "Whispem"
print name
```

## Output

    Whispem

---

## Syntax Overview

### Variables

Variables are declared using `let`.

    let x = 10
    let message = "Hello"

Variables are immutable in this version.

---

### Print

The `print` statement evaluates and outputs a value.

    print x
    print message

---

### Types

Whispem currently supports:

- Number (floating-point)
- String

---

### Comments

Comments begin with `#` and continue until the end of the line.

    # This is a comment
    let version = "0.2.0"

---

## Limitations

Whispem v0.2.0 does not support:

- Arithmetic expressions  
- Conditionals  
- Loops  
- Functions  
- Advanced error reporting  

These limitations are intentional.

---

## Vision

Whispem is designed to remain small, understandable, and deliberate.

Future versions may introduce new features gradually, but only if they preserve
the language’s clarity and learning value.

The goal is not to build a powerful tool, but a language that can be fully
understood by its creator.
