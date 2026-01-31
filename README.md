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
- **Functions with parameters and return values** (`fn`, `return`)
- Unary operators (`-`, `not`)
- Block syntax with `{ }`
- String literals with escape sequences
- String concatenation with `+`
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

### Functions
```wsp
fn greet(name) {
    print "Hello, " + name + "!"
}

greet("World")
greet("Whispem")
```

Output:
```text
Hello, World!
Hello, Whispem!
```

### Functions with Return Values
```wsp
fn add(a, b) {
    return a + b
}

let result = add(10, 20)
print result
```

Output:
```text
30
```

### Recursion
```wsp
fn factorial(n) {
    if n <= 1 {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}

print factorial(5)
```

Output:
```text
120
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

**Current version:** v0.8.0

Whispem now includes:
- variables and expressions
- booleans and comparisons
- conditional control flow (`if / else`)
- loops (`while`)
- logical operators (`and`, `or`, `not`)
- **functions with parameters and return values**
- **recursion support**
- **local variable scopes**
- string concatenation

This version represents a major milestone: Whispem is now a fully functional
programming language with all core features needed for real programs.

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

Planned features for v1.0.0:

- Break and continue for loops
- Better error reporting with line numbers
- Arrays or lists
- Standard library functions
- Module system

---

## Why "Whispem"?

Because the language is meant to whisper intent,
not shout complexity.

---

## License

MIT
