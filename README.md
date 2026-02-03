# Whispem
<img width="4000" height="1600" alt="whispem-lang-4klogo" src="https://github.com/user-attachments/assets/6078ca73-68d9-4a76-8bde-8ed1d284aa4b" />


**Version 1.0.0** - Production Ready 🎉

Whispem is a minimalist interpreted programming language designed for clarity,
expressiveness, and calm readability.

It focuses on a simple syntax, explicit logic, and a smooth learning curve,
while remaining powerful enough to express real control flow and computation.

Whispem is implemented in Rust, but Whispem itself is a standalone language.

---

## Features

### Core Language
- Variables with `let`
- Arithmetic expressions with operator precedence
- Boolean values (`true`, `false`)
- Comparisons (`<`, `>`, `<=`, `>=`, `==`, `!=`)
- Logical operators (`and`, `or`, `not`)
- Unary operators (`-`, `not`)
- String literals with escape sequences
- String concatenation with `+`

### Control Flow
- Conditional execution (`if / else`)
- While loops (`while`)
- **For loops (`for item in array`)**
- **Break and continue statements**

### Functions
- Function declarations (`fn`)
- Parameters and return values
- Recursion support
- Local variable scopes

### Data Structures
- Arrays with indexing `[1, 2, 3]`
- Mixed-type arrays
- Nested arrays
- Dynamic array building

### Built-in Functions
- `length(array/string)` - get length
- `push(array, item)` - append element
- **`pop(array)` - remove last element**
- **`reverse(array)` - reverse array**
- **`slice(array, start, end)` - get sub-array**
- **`range(start, end)` - generate number sequence**
- **`input(prompt)` - read user input**
- **`read_file(filename)` - read file**
- **`write_file(filename, content)` - write file**

### Developer Experience
- **Better error messages with context**
- Line-based syntax (no semicolons)
- Clean, intuitive syntax
- Comprehensive documentation

---

## Installation

Clone the repository:
```bash
git clone https://github.com/whispem/whispem-lang.git
cd whispem-lang
```

Build the project:
```bash
cargo build --release
```

The binary will be at `target/release/whispem`.

---

## Quick Start

Create a file `hello.wsp`:
```wsp
let name = input("What's your name? ")
print "Hello, " + name + "!"
```

Run it:
```bash
cargo run hello.wsp
```

---

## Examples

### Hello World
```wsp
let message = "Hello, Whispem!"
print message
```

### User Input
```wsp
let name = input("Enter your name: ")
let age = input("Enter your age: ")
print "Hello " + name + ", you are " + age + " years old!"
```

### For Loops
```wsp
for num in [1, 2, 3, 4, 5] {
    print num
}

for i in range(0, 10) {
    print i
}
```

### Break and Continue
```wsp
for num in range(1, 20) {
    if num > 10 {
        break
    }
    if num == 5 {
        continue
    }
    print num
}
```

### Arrays with New Functions
```wsp
let numbers = [1, 2, 3, 4, 5]

# Pop last element
let last = pop(numbers)
print last  # 5

# Reverse array
let reversed = reverse([1, 2, 3])
print reversed  # [3, 2, 1]

# Slice array
let middle = slice([1, 2, 3, 4, 5], 1, 4)
print middle  # [2, 3, 4]
```

### File I/O
```wsp
# Write to file
let data = "Hello from Whispem!"
write_file("output.txt", data)

# Read from file
let content = read_file("output.txt")
print content
```

### Functions
```wsp
fn factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

print factorial(5)  # 120
```

### Complete Program
```wsp
# Simple task manager
fn add_task(tasks, task) {
    return push(tasks, task)
}

fn show_tasks(tasks) {
    print "Your tasks:"
    for task in tasks {
        print "- " + task
    }
}

let my_tasks = []
let my_tasks = add_task(my_tasks, "Learn Whispem")
let my_tasks = add_task(my_tasks, "Write code")
let my_tasks = add_task(my_tasks, "Build something cool")

show_tasks(my_tasks)
```

---

## Documentation

Comprehensive documentation is available in the `docs/` directory:

- `docs/syntax.md` — Complete language syntax reference
- `docs/vision.md` — Philosophy and design principles
- `docs/examples.md` — Extensive code examples
- `docs/tutorial.md` — Step-by-step tutorial for beginners
- `examples/` — Runnable `.wsp` example programs

---

## Project Status

**Current version:** v1.0.0 - Production Ready! 🎉

Whispem 1.0.0 is feature-complete and stable. It includes:
- All core language features
- Comprehensive built-in functions
- File I/O capabilities
- Interactive user input
- Excellent error messages
- Complete documentation

**Ready for:**
- Learning programming
- Teaching language design
- Scripting and automation
- Educational projects
- Rapid prototyping

---

## Design Goals

Whispem is designed to be:

- **Simple to read** - Code looks like what it does
- **Easy to reason about** - No hidden behavior
- **Small enough to understand entirely** - The whole language fits in your head
- **Expressive without being verbose** - Say what you mean, clearly

Every feature must justify its existence.

---

## What's Next?

Whispem 1.0.0 is feature-complete, but development continues:

### Post-1.0 Roadmap
- Standard library expansion
- Performance optimizations
- Self-hosting (Whispem written in Whispem)
- Module system
- Package manager
- VS Code extension
- Online playground

---

## Community

- **Report bugs:** [GitHub Issues](https://github.com/whispem/whispem-lang/issues)
- **Contribute:** [Contributing Guide](CONTRIBUTING.md)
- **Discuss:** [GitHub Discussions](https://github.com/whispem/whispem-lang/discussions)

---

## Why "Whispem"?

Because the language is meant to whisper intent,
not shout complexity.

Code should be quiet, clear, and calm.

---

## License

MIT License - See [LICENSE](LICENSE) file for details.

---

## Acknowledgments

Whispem was created as an exploration of minimalist language design.
Special thanks to the Rust community and all contributors.

**Whispem 1.0.0 - Simple. Clear. Complete.** ✨
