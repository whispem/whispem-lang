# Whispem

> *"Code should whisper intent, not shout complexity."*

A minimalist programming language you can **fully understand** — built in Rust, designed for clarity.

![Logo Whispem](https://imgur.com/YDjrAKR.png)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-1.5.0-blue.svg)](https://github.com/whispem/whispem-lang/releases)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)

---

## What is Whispem?

Whispem is a **complete programming language small enough to learn in a weekend**.

Unlike most languages that grow complex over time, Whispem is intentionally minimal:
- **14 keywords** - that's the whole language
- **12 built-in functions** - everything you need, nothing you don't
- **5 data types** - numbers, strings, booleans, arrays, dictionaries
- **0 hidden behavior** - what you see is what you get

**Perfect for:**
- Learning how programming languages work
- Teaching programming concepts
- Understanding interpreters and compilers
- Quick scripting and automation
- Grasping language design principles

**Not trying to be:**
- A replacement for Python, JavaScript, or Rust
- Performance-focused or production web framework
- Feature-complete for every use case

---

## Quick Start (2 minutes)

```bash
# Clone and build
git clone https://github.com/whispem/whispem-lang.git
cd whispem-lang
cargo build --release
```

Create `hello.wsp`:
```wsp
let name = input("What's your name? ")
print "Hello, " + name + "! Welcome to Whispem."
```

Run it:
```bash
cargo run examples/hello.wsp
```

**Or launch the interactive REPL:**
```bash
cargo run
```

---

## What's New in v1.5.0

### Dictionaries
```wsp
let person = {"name": "Em", "city": "Marseille"}
print person["name"]          # Em

person["city"] = "Paris"      # assignment
print has_key(person, "name") # true
print keys(person)            # [city, name]
```

### Modulo Operator
```wsp
print 10 % 3    # 1

# Real FizzBuzz — finally!
for n in range(1, 101) {
    if n % 15 == 0 {
        print "FizzBuzz"
    } else {
        if n % 3 == 0 { print "Fizz" }
        else {
            if n % 5 == 0 { print "Buzz" }
            else { print n }
        }
    }
}
```

### Proper Error Messages
```
[line 3, col 12] Error: Undefined variable: 'counter'
[line 7, col 5]  Error: Array index 10 out of bounds (array length: 5)
[line 12, col 1] Error: Function 'add' expected 2 arguments, got 3
```

### Interactive REPL
```bash
$ cargo run
Whispem v1.5.0 — REPL
Type 'exit' or press Ctrl-C to quit.

>>> let x = 42
>>> print x
42
>>> exit
```

---

## Features

### Core Language
```wsp
# Variables and types
let x = 42
let name = "Whispem"
let is_valid = true
let data = [1, 2, 3, 4, 5]
let config = {"host": "localhost", "port": 8080}

# Expressions with proper precedence
let result = (10 + 5) * 2   # 30
let rest   = 17 % 5          # 2
```

### Control Flow
```wsp
if temperature > 20 {
    print "It's warm!"
} else {
    print "It's cool!"
}

while counter < 10 {
    print counter
    let counter = counter + 1
}

for num in range(1, 10) {
    if num % 2 == 0 { continue }
    if num > 7      { break }
    print num
}
```

### Functions
```wsp
fn greet(name) {
    return "Hello, " + name + "!"
}

fn fibonacci(n) {
    if n <= 1 { return n }
    return fibonacci(n - 1) + fibonacci(n - 2)
}
```

### Arrays
```wsp
let numbers = [1, 2, 3, 4, 5]
let last    = pop(numbers)
let rev     = reverse([1, 2, 3])
let mid     = slice([10, 20, 30, 40, 50], 1, 4)
let seq     = range(0, 10)
```

### Dictionaries
```wsp
let scores = {}
scores["Alice"] = 95
scores["Bob"]   = 87

print has_key(scores, "Alice")  # true
print keys(scores)              # [Alice, Bob]
print values(scores)            # [95, 87]
print length(scores)            # 2
```

### I/O
```wsp
let name    = input("Enter your name: ")
let content = read_file("data.txt")
write_file("output.txt", "Hello from Whispem!")
```

---

## Complete Example: Word Counter

```wsp
fn count_words(words) {
    let counts = {}
    for word in words {
        if has_key(counts, word) {
            counts[word] = counts[word] + 1
        } else {
            counts[word] = 1
        }
    }
    return counts
}

let words  = ["rust", "whispem", "rust", "language", "whispem", "rust"]
let counts = count_words(words)

let word_keys = keys(counts)
for word in word_keys {
    print word + ": " + counts[word]
}
```

---

## Documentation

Comprehensive docs in the `docs/` directory:

| Document | Description |
|----------|-------------|
| **[Tutorial](docs/tutorial.md)** | Step-by-step guide from zero to building apps |
| **[Syntax Reference](docs/syntax.md)** | Complete language syntax |
| **[Examples](docs/examples.md)** | 37+ runnable examples |
| **[Vision](docs/vision.md)** | Philosophy and design principles |
| **[My Journey](docs/journey.md)** | How I went from literature to building this |
| **[Changelog](CHANGELOG.md)** | Full version history |

---

## Project Status

**Current Version:** 1.5.0

### Complete Feature Set

- Variables and reassignment
- Five data types (numbers, strings, booleans, arrays, **dictionaries**)
- Arithmetic with operator precedence, including **modulo**
- Comparisons and logical operators
- If/else conditionals
- While and for loops, break and continue
- Functions with parameters, return values, recursion
- Arrays with full operations
- **Dictionaries with keys/values/has_key**
- String concatenation and escape sequences
- User input and file I/O
- **Helpful error messages with line and column numbers**
- **Interactive REPL**
- Complete documentation

---

## Quick Facts

| Metric | Value |
|--------|-------|
| **Keywords** | 14 |
| **Built-in functions** | 12 |
| **Data types** | 5 |
| **Example programs** | 37+ |
| **Documentation pages** | 5 comprehensive guides |
| **Lines of implementation** | ~1,800 (readable!) |
| **Time to learn basics** | 1-2 hours |
| **Time to master** | A weekend |

---

## Why I Built This

I'm Emilie (Em'), and I went from studying **literature and linguistics** to building programming languages in Rust — in just a few months.

**Timeline:**
- **October 27, 2025** — Wrote my first Rust "Hello World"
- **December 16, 2025** — Gave a public talk at Epitech Marseille
- **January 19, 2026** — Featured in *Programmez!* magazine
- **February 1, 2026** — Released Whispem 1.0.0
- **February 19, 2026** — Released Whispem 1.5.0 — dictionaries, modulo, REPL, proper errors

Read my full journey: **[docs/journey.md](docs/journey.md)**

---

## Community

- Report bugs: [GitHub Issues](https://github.com/whispem/whispem-lang/issues)
- Discuss: [GitHub Discussions](https://github.com/whispem/whispem-lang/discussions)
- **RAM** (Rust Aix-Marseille): [Discord](https://discord.gg/zgGWvVFJQg) | [LinkedIn](https://www.linkedin.com/company/rust-aix-marseille-ram)

---

## Design Principles

> *Every feature must justify its existence.*

1. **Clarity over cleverness** — Code reads like what it does
2. **Explicitness over magic** — No hidden behavior
3. **Small over large** — The whole language fits in your head
4. **Calm over chaos** — No syntactic noise
5. **Teachable over powerful** — Understanding first, features second

---

## License

MIT License — See [LICENSE](LICENSE) file for details.

---

**Made with Rust and care by Emilie Peretti (@whispem)**

*From literature student to language designer.*  
*If I can do this, so can you.*

---

Star this repo | Share it | Join RAM on Discord