# Whispem

Whispem is a minimalist interpreted programming language designed for clarity,
expressiveness, and calm readability.

It focuses on a simple syntax, explicit logic, and a smooth learning curve,
while remaining powerful enough to express real control flow and computation.

Whispem is implemented in Rust, but Whispem itself is a standalone language.

---

## Features

- Variables with `let`
- Arithmetic expressions
- Boolean values (`true`, `false`)
- Comparisons (`<`, `>`, `<=`, `>=`, `==`, `!=`)
- Conditional execution (`if / else`)
- Block syntax with `{ }`
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
cargo build
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

## Example

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

---

## Documentation

All documentation is written in Markdown and lives in the repository:

- `docs/syntax.md` — language syntax and grammar
- `docs/vision.md` — philosophy and long-term vision
- `examples/` — runnable Whispem programs

---

## Project status

**Current version:** v0.6.0

Whispem now includes:
- variables
- expressions
- booleans
- comparisons
- conditional control flow (`if / else`)

This version marks the transition from an experimental project
to a fully expressive interpreted language core.

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

- Functions
- Return values
- Loops
- A small standard library
- Better error reporting

---

## Why “Whispem”?

Because the language is meant to whisper intent,
not shout complexity.

---

## License

MIT
