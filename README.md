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
- Functions with parameters and return values (`fn`, `return`)
- **Arrays with indexing and built-in functions**
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

### Arrays
```wsp
let numbers = [1, 2, 3, 4, 5]
print numbers

let first = numbers[0]
print first

numbers[2] = 10
print numbers
```

Output:
```text
[1, 2, 3, 4, 5]
1
[1, 2, 10, 4, 5]
```

### Array Functions
```wsp
let items = [1, 2, 3]
let len = length(items)
print len

let new_items = push(items, 4)
print new_items
```

Output:
```text
3
[1, 2, 3, 4]
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

### Iterating Over Arrays
```wsp
let numbers = [10, 20, 30, 40]
let i = 0

while i < length(numbers) {
    print numbers[i]
    let i = i + 1
}
```

Output:
```text
10
20
30
40
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

**Current version:** v0.9.0

Whispem now includes:
- variables and expressions
- booleans and comparisons
- conditional control flow (`if / else`)
- loops (`while`)
- logical operators (`and`, `or`, `not`)
- functions with parameters and return values
- recursion support
- local variable scopes
- **arrays with indexing**
- **built-in functions: `length()`, `push()`**
- string concatenation

This version represents a major milestone: Whispem is now feature-complete
and ready for 1.0.0 consideration. All core language features are implemented.

---

## Design goals

Whispem is designed to be:

- simple to read
- easy to reason about
- small enough to understand entirely
- expressive without being verbose

Every feature must justify its existence.

---

## Roadmap to 1.0.0

Final features before 1.0.0:

- Break and continue for loops
- Better error reporting with line numbers
- More array operations (pop, slice, etc.)
- File I/O
- Standard library organization

---

## Why "Whispem"?

Because the language is meant to whisper intent,
not shout complexity.

---

## License

MIT
