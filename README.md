# Whispem

Whispem is a minimalist interpreted programming language designed for clarity,
expressiveness, and calm readability.

It focuses on a simple syntax, explicit logic, and a smooth learning curve,
while remaining powerful enough to express real control flow and computation.

Whispem is implemented in Rust, but Whispem itself is a standalone language.

---

## Features

- Variables with `let`
- Arithmetic expressions with operator precedence
- Boolean values (`true`, `false`)
- Comparisons (`<`, `>`, `<=`, `>=`, `==`, `!=`)
- Logical operators (`and`, `or`, `not`)
- Conditional execution (`if / else`)
- While loops (`while`)
- Unary operators (`-`, `not`)
- Block syntax with `{ }`
- String literals with escape sequences
- Line-based syntax (no semicolons)
- Interpreter-based execution

---

## Installation

Clone the repository:

```bash
git clone https://github.com/whispem/whispem-lang.git
cd whispem-lang
```

---

## Build the project

```bash
cargo build --release
```

---

## Running a Whispem program

Run a `.wsp` file with:

```bash
cargo run examples/hello.wsp
```

If no file is provided:

```bash
cargo run
```

Output:

```text
Whispem
```

---

## Examples

### Hello World

```wsp
let message = "Hello, Whispem!"
print message
```

Output:

```text
Hello, Whispem!
```

### Conditionals

```wsp
let x = 10
let y = 20

if x < y {
    print y
} else {
    print x
}
```

Output:

```text
20
```

### While Loops

```wsp
let counter = 0

while counter < 5 {
    print counter
    let counter = counter + 1
}
```

Output:

```text
0
1
2
3
4
```

### Logical Operators

```wsp
let x = 10

if x > 5 and x < 15 {
    print "x is in range"
}

if not false {
    print "This always prints"
}
```

Output:

```text
x is in range
This always prints
```

---

## Documentation

All documentation is written in Markdown and lives in the repository:

- `docs/syntax.md` — language syntax and grammar
- `docs/vision.md` — philosophy and long-term vision
- `docs/examples.md` — runnable example programs
- `examples/` — executable `.wsp` files

---

## Project status

**Current version:** v0.7.0

Whispem now includes:
- variables and expressions
- booleans and comparisons
- conditional control flow (`if / else`)
- loops (`while`)
- logical operators (`and`, `or`, `not`)
- unary operators

This version marks a significant milestone with full control flow support
and logical reasoning capabilities.

---

## Design goals

Whispem is designed to be:

- simple to read
- easy to reason about
- small enough to understand entirely
- expressive without being verbose

Every feature must justify its existence.

---

## Roadmap

Planned features include:

- Functions and return values
- Break and continue for loops
- String concatenation
- Arrays or lists
- Better error reporting with line numbers
- A small standard library

---

## Why "Whispem"?

Because the language is meant to whisper intent,
not shout complexity.

---

## License

MIT
