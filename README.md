# Whispem

> *"Code should whisper intent, not shout complexity."*

A minimalist programming language you can **fully understand** — built in Rust, designed for clarity.

![Logo Whispem](https://imgur.com/YDjrAKR.png)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://github.com/whispem/whispem-lang/releases)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/whispem/whispem-lang/pulls)

---

## What is Whispem?

Whispem is a **complete programming language small enough to learn in a weekend**.

Unlike most languages that grow complex over time, Whispem is intentionally minimal:
- **14 keywords** - that's the whole language
- **9 built-in functions** - everything you need, nothing you don't
- **4 data types** - numbers, strings, booleans, arrays
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

##  Quick Start (2 minutes)

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

**That's it.** You just ran your first Whispem program!

---

##  Why Whispem?

Most languages are **too big to understand completely**. Whispem is different.

```wsp
# Everything is explicit. No magic.
fn factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

let result = factorial(5)
print result  # 120
```

### What Makes It Special

- Read the entire implementation in an afternoon
- Understand every feature in a day
- Build real programs immediately
- Learn by reading clean, simple code
- No semicolons, no ceremony, just clarity

---

##  Features

### Core Language
```wsp
# Variables and types
let x = 42
let name = "Whispem"
let is_valid = true
let data = [1, 2, 3, 4, 5]

# Expressions with proper precedence
let result = (10 + 5) * 2  # 30
```

### Control Flow
```wsp
# Conditionals
if temperature > 20 {
    print "It's warm!"
} else {
    print "It's cool!"
}

# Loops
while counter < 10 {
    print counter
    let counter = counter + 1
}

# For loops with break/continue
for num in range(1, 10) {
    if num == 5 {
        continue
    }
    if num > 7 {
        break
    }
    print num
}
```

### Functions
```wsp
fn greet(name) {
    return "Hello, " + name + "!"
}

fn fibonacci(n) {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

print greet("World")
print fibonacci(10)
```

### Arrays & Built-ins
```wsp
let numbers = [1, 2, 3, 4, 5]

# Array operations
let last = pop(numbers)
let reversed = reverse([1, 2, 3])
let middle = slice([10, 20, 30, 40, 50], 1, 4)
let sequence = range(0, 10)

# I/O
let user_input = input("Enter something: ")
let content = read_file("data.txt")
write_file("output.txt", "Hello from Whispem!")
```

---

##  Complete Example: Todo Manager

```wsp
fn add_task(tasks, task) {
    return push(tasks, task)
}

fn show_tasks(tasks) {
    if length(tasks) == 0 {
        print "No tasks yet!"
        return
    }
    
    print "Your Tasks:"
    let i = 1
    for task in tasks {
        print i
        print ". " + task
        let i = i + 1
    }
}

# Main program
let my_tasks = []
let my_tasks = add_task(my_tasks, "Learn Whispem")
let my_tasks = add_task(my_tasks, "Build something cool")
let my_tasks = add_task(my_tasks, "Share with others")

show_tasks(my_tasks)
```

---

##  Documentation

Comprehensive docs in the `docs/` directory:

| Document | Description |
|----------|-------------|
| **[Tutorial](docs/tutorial.md)** | Step-by-step guide from zero to building apps |
| **[Syntax Reference](docs/syntax.md)** | Complete language syntax |
| **[Examples](docs/examples.md)** | 31+ runnable examples |
| **[Vision](docs/vision.md)** | Philosophy and design principles |
| **[My Journey](docs/journey.md)** | How I went from literature to building this |

**Example Programs:** Check out `examples/` - everything from Hello World to data processing.

---

##  Learning Path

**New to programming?** Start here:

1. Read **[Tutorial](docs/tutorial.md)** (teaches programming + Whispem)
2. Run examples from `examples/` directory
3. Build your own small project
4. Read the **[Syntax Reference](docs/syntax.md)**
5. Explore the source code

**Want to understand language implementation?**

1. Read **[Vision](docs/vision.md)** for design philosophy
2. Study `src/lexer.rs` - tokenization
3. Study `src/parser.rs` - syntax trees
4. Study `src/interpreter.rs` - execution
5. Modify and experiment!

---

##  Project Status

**Current Version:** 1.0.0

### Complete Feature Set

- Variables and reassignment
- Four data types (numbers, strings, booleans, arrays)
- Arithmetic with operator precedence
- Comparisons and logical operators
- If/else conditionals
- While and for loops
- Break and continue statements
- Functions with parameters and return values
- Recursion support
- Arrays with full operations
- String concatenation and escape sequences
- User input and file I/O
- Helpful error messages with context
- Complete documentation

**Ready for:**
- Learning programming from scratch
- Teaching language design
- Educational projects
- Scripting and automation
- Understanding how interpreters work

---

##  Why I Built This

I'm Emilie (Em'), and I went from studying **literature and linguistics** to building programming languages in Rust—in just a few months.

**Timeline:**
- **October 27, 2025** - Wrote my first Rust "Hello World"
- **December 16, 2025** - Gave a public talk at Epitech Marseille
- **January 19, 2026** - Featured in *Programmez!* magazine
- **February 2026** - Released Whispem 1.0.0

I built Whispem because I wanted a language that:
- Doesn't hide complexity behind magic
- Can be understood completely
- Teaches by being simple, not by being simplified
- Proves that "minimal" doesn't mean "toy"

**If I could learn Rust and build this starting from zero technical background, you can too.**

Read my full journey: **[docs/journey.md](docs/journey.md)**

---

##  Community

### Get Involved

- Report bugs: [GitHub Issues](https://github.com/whispem/whispem-lang/issues)
- Discuss: [GitHub Discussions](https://github.com/whispem/whispem-lang/discussions)
- Star the repo if you find it useful

### Join Rust Aix-Marseille (RAM)

I also founded **RAM** - an inclusive Rust community in Provence (and beyond):
- Discord: [Join us](https://discord.gg/zgGWvVFJQg)
- LinkedIn: [Follow RAM](https://www.linkedin.com/company/rust-aix-marseille-ram)
- Regular meetups (online + IRL in Aix/Marseille)
- Welcoming to all levels - from total beginners to experts

---

##  What's Next?

### Post-1.0 Roadmap

Whispem 1.0.0 is feature-complete, but development continues:

- Standard library expansion
- Performance optimizations
- Self-hosting (Whispem written in Whispem)
- Module system for code organization
- VS Code extension with syntax highlighting
- Online playground for browser-based learning
- Package manager for sharing code

---

##  Design Principles

> *Every feature must justify its existence.*

Whispem values:

1. **Clarity over cleverness** - Code reads like what it does
2. **Explicitness over magic** - No hidden behavior
3. **Small over large** - The whole language fits in your head
4. **Calm over chaos** - No syntactic noise
5. **Teachable over powerful** - Understanding first, features second

---

##  Language Comparison

```wsp
# Whispem - 3 lines
fn greet(name) {
    print "Hello, " + name + "!"
}
```

**Lines of code to understand the core language:**
- Whispem: ~1,500 lines (lexer + parser + interpreter)
- Most languages: Tens of thousands or more

**Time to learn completely:**
- Whispem: A weekend
- Most languages: Months or years

---

##  Quick Facts

| Metric | Value |
|--------|-------|
| **Keywords** | 14 |
| **Built-in functions** | 9 |
| **Data types** | 4 |
| **Example programs** | 31+ |
| **Documentation pages** | 5 comprehensive guides |
| **Lines of implementation** | ~1,500 (readable!) |
| **Time to learn basics** | 1-2 hours |
| **Time to master** | A weekend |

---

##  What People Are Saying

> *"A programming language you can actually understand completely."*

> *"Perfect for learning how interpreters work."*

> *"Finally, a language that doesn't hide everything behind abstractions."*

> *"Great for teaching programming concepts."*

---

##  Example Projects

Get inspired by what you can build:

- **Calculator** - Math expressions with precedence
- **Todo Manager** - Task list with file persistence
- **Number Guessing Game** - Interactive game with user input
- **Data Analyzer** - Filter, sort, and aggregate data
- **Prime Number Generator** - Algorithm implementation
- **FizzBuzz** - Classic programming challenge
- **File Processor** - Read, transform, write files

See all examples: **[examples/](examples/)** directory

---

##  License

MIT License - See [LICENSE](LICENSE) file for details.

---

##  Acknowledgments

Built with:
- Rust - for making systems programming accessible
- The Rust community - for incredible support and resources
- Literature & linguistics - for teaching me how language works
- Curiosity - for making me ask "how does this work?"

Special thanks to everyone who believed a literature student could build programming languages.

---

##  Links

- **Repository:** [github.com/whispem/whispem-lang](https://github.com/whispem/whispem-lang)
- **Documentation:** [docs/](docs/)
- **Examples:** [examples/](examples/)
- **My Journey:** [docs/journey.md](docs/journey.md)
- **Rust Community:** [RAM Discord](https://discord.gg/zgGWvVFJQg)

---

##  Final Thought

> *"Whispem proves that small, simple, and understandable can also be complete and powerful."*

If you've ever wanted to:
- Understand how programming languages work
- Learn by building something real
- See behind the abstraction curtain
- Teach programming with clarity

**Whispem is for you.**

Start your journey today. **Clone it. Run it. Read it. Modify it. Build with it.**

---

**Made with Rust and care by Emilie Peretti (@whispem)**

*From literature student to language designer in 3.5 months.*  
*If I can do this, so can you.*

---

Star this repo | Share it with someone learning to code | Join the conversation on Discord
