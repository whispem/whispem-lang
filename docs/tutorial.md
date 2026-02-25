# Whispem Tutorial

**Version 2.0.0**

Welcome to Whispem. This tutorial covers the entire language from first program to complete applications. By the end you'll know everything Whispem has — because everything it has fits in a single document.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Variables and Types](#variables-and-types)
3. [Expressions and Operators](#expressions-and-operators)
4. [Strings](#strings)
5. [Conditionals](#conditionals)
6. [Loops](#loops)
7. [Functions](#functions)
8. [Arrays](#arrays)
9. [Dictionaries](#dictionaries)
10. [User Input and File I/O](#user-input-and-file-io)
11. [Complete Examples](#complete-examples)
12. [Under the Hood](#under-the-hood)

---

## Getting Started

### Install

```bash
git clone https://github.com/whispem/whispem-lang.git
cd whispem-lang
cargo build --release
```

### Run a file

```bash
cargo run examples/hello.wsp
```

### Inspect compiled bytecode

```bash
cargo run -- --dump examples/hello.wsp
```

```
== <main> ==
0000     1  PUSH_CONST         0    'Hello, Whispem!'
0002     3  STORE              1    'message'
0004     4  LOAD               1    'message'
0006     4  PRINT
0007     4  HALT
```

### Interactive REPL

```bash
cargo run
```

```
Whispem v2.0.0 — REPL
Type 'exit' or press Ctrl-D to quit.

>>> let x = 42
>>> print x
42
>>> fn double(n) {
...     return n * 2
... }
>>> print double(x)
84
>>> exit
Bye!
```

Function definitions persist across REPL lines.

---

## Variables and Types

```wsp
let name    = "Whispem"
let version = 2.0
let ready   = true
```

Variables are declared with `let`. Types are inferred automatically. There are five types:

| Type | Examples |
|------|----------|
| `number` | `42`, `3.14`, `-7` |
| `string` | `"hello"`, `""` |
| `bool` | `true`, `false` |
| `array` | `[1, 2, 3]` |
| `dict` | `{"key": "value"}` |

To update a variable, use `let` again:

```wsp
let counter = 0
let counter = counter + 1
print counter   # 1
```

There is no bare assignment — only `let x = expr` and `x[i] = expr`.

---

## Expressions and Operators

### Arithmetic

```wsp
print 10 + 3    # 13
print 10 - 3    # 7
print 10 * 3    # 30
print 10 / 3    # 3.333...
print 10 % 3    # 1  ← modulo
```

Precedence follows standard math rules. Use parentheses when in doubt:

```wsp
print 10 + 5 * 2     # 20
print (10 + 5) * 2   # 30
```

### Comparisons

```wsp
print 10 == 10   # true
print 10 != 5    # true
print 10 > 5     # true
print 10 < 5     # false
```

### Logic

```wsp
print true and false   # false
print true or false    # true
print not true         # false
```

`and` and `or` short-circuit — they stop as soon as the result is known.

---

## Strings

```wsp
let greeting = "Hello"
let name = "Whispem"
print greeting + ", " + name + "!"   # Hello, Whispem!
```

### Escape sequences

```wsp
print "Line one\nLine two"
print "She said \"hello\""
print "Tab\there"
```

### Length

```wsp
print length("hello")   # 5
```

---

## Conditionals

```wsp
let temperature = 18

if temperature > 20 {
    print "warm"
} else {
    print "cool"
}
```

The `else` branch is optional. For multiple branches, nest `if` inside `else`:

```wsp
let score = 85

if score >= 90 {
    print "A"
} else {
    if score >= 80 {
        print "B"
    } else {
        print "C"
    }
}
```

---

## Loops

### While

```wsp
let i = 0
while i < 5 {
    print i
    let i = i + 1
}
# prints 0 1 2 3 4
```

### For

```wsp
for fruit in ["apple", "banana", "cherry"] {
    print fruit
}

for n in range(0, 5) {
    print n
}
# prints 0 1 2 3 4
```

`for` loops compile to a counter-based while loop internally — `range()` and array length are checked at each iteration.

### Break and continue

```wsp
for n in range(1, 20) {
    if n > 10 { break }
    if n % 2 == 0 { continue }
    print n   # 1 3 5 7 9
}
```

---

## Functions

```wsp
fn greet(name) {
    return "Hello, " + name + "!"
}

print greet("world")   # Hello, world!
```

Functions support recursion:

```wsp
fn factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

print factorial(5)    # 120
print factorial(10)   # 3628800
```

### Scope

Variables declared at the top level are globals. Variables inside a function are local to that function. Functions have read access to globals — but they cannot mutate them (there is no bare assignment statement).

```wsp
let greeting = "Hello"

fn say(name) {
    print greeting + ", " + name   # can read global
}

say("Em")   # Hello, Em
```

### Forward calls

Functions can be called before they are defined in the file — the compiler handles them in a first pass.

---

## Arrays

### Creating and indexing

```wsp
let fruits = ["apple", "banana", "cherry"]

print fruits[0]        # apple
print length(fruits)   # 3
```

### Modifying elements

```wsp
let scores = [10, 20, 30]
scores[1] = 99
print scores   # [10, 99, 30]
```

Index assignment is real mutation — the array is updated in place and written back.

### Built-in functions

```wsp
let nums = [1, 2, 3]

let nums  = push(nums, 4)              # [1, 2, 3, 4]
let last  = pop([1, 2, 3])             # 3
let rev   = reverse([1, 2, 3])         # [3, 2, 1]
let mid   = slice([1,2,3,4,5], 1, 4)   # [2, 3, 4]
let seq   = range(0, 5)                # [0, 1, 2, 3, 4]
```

### Iterating

```wsp
let total = 0
for n in [10, 20, 30, 40] {
    let total = total + n
}
print total   # 100
```

---

## Dictionaries

Dictionaries store key-value pairs. Keys are always strings.

### Creating and accessing

```wsp
let person = {"name": "Em", "city": "Marseille", "age": 26}

print person["name"]   # Em
print person["age"]    # 26
```

### Adding and updating keys

```wsp
person["city"] = "Paris"
person["job"]  = "developer"
```

### Built-in functions

```wsp
let d = {"b": 2, "a": 1, "c": 3}

print has_key(d, "a")   # true
print has_key(d, "z")   # false
print keys(d)           # [a, b, c]  (sorted)
print values(d)         # [1, 2, 3]  (sorted by key)
print length(d)         # 3
```

### Dicts in functions

```wsp
fn describe(person) {
    return person["name"] + " lives in " + person["city"]
}

let em = {"name": "Em", "city": "Marseille"}
print describe(em)   # Em lives in Marseille
```

### Word counter

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

let words  = ["rust", "whispem", "rust", "rust", "whispem"]
let counts = count_words(words)

for word in keys(counts) {
    print word + ": " + counts[word]
}
# rust: 3
# whispem: 2
```

---

## User Input and File I/O

```wsp
# Read from stdin
let name = input("What's your name? ")
print "Hello, " + name + "!"

# Write to file
write_file("output.txt", "Hello from Whispem!")

# Read from file
let content = read_file("output.txt")
print content
```

---

## Complete Examples

### FizzBuzz

```wsp
for n in range(1, 101) {
    if n % 15 == 0 {
        print "FizzBuzz"
    } else {
        if n % 3 == 0 {
            print "Fizz"
        } else {
            if n % 5 == 0 {
                print "Buzz"
            } else {
                print n
            }
        }
    }
}
```

### Phonebook

```wsp
fn add(book, name, number) {
    book[name] = number
    return book
}

fn lookup(book, name) {
    if has_key(book, name) {
        return book[name]
    }
    return "not found"
}

let phonebook = {}
let phonebook = add(phonebook, "Alice", "06 12 34 56 78")
let phonebook = add(phonebook, "Bob",   "07 98 76 54 32")

print lookup(phonebook, "Alice")    # 06 12 34 56 78
print lookup(phonebook, "Charlie")  # not found

for name in keys(phonebook) {
    print name + ": " + phonebook[name]
}
```

### Data processing

```wsp
fn filter(numbers, threshold) {
    let result = []
    for n in numbers {
        if n > threshold {
            let result = push(result, n)
        }
    }
    return result
}

fn sum(numbers) {
    let total = 0
    for n in numbers { let total = total + n }
    return total
}

fn average(numbers) {
    if length(numbers) == 0 { return 0 }
    return sum(numbers) / length(numbers)
}

let data = [3, 17, 2, 41, 8, 25, 6, 33]
let high = filter(data, 10)

print "Values above 10:"
print high

print "Average:"
print average(high)
```

---

## Under the Hood

Since v2.0.0, Whispem compiles source code to bytecode before executing it. The pipeline is:

```
Source → Lexer → Parser → AST → Compiler → Bytecode → VM
```

You can inspect the compiled bytecode of any program with `--dump`:

```bash
whispem --dump examples/fizzbuzz_proper.wsp
```

The VM is a stack machine with 31 opcodes and a separate call stack with one frame per active function call. See [`docs/vm.md`](vm.md) for the complete specification.

---

**Whispem v2.0.0**  
*You've seen the whole language. Everything Whispem has is in this document.*