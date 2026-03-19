# Whispem Tutorial

**Version 5.0.0**

Welcome to Whispem. This tutorial covers the entire language from first program to complete applications. By the end you'll know everything Whispem has — because everything it has fits in a single document.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Variables and Types](#variables-and-types)
3. [Expressions and Operators](#expressions-and-operators)
4. [Strings and F-strings](#strings-and-f-strings)
5. [Conditionals](#conditionals)
6. [Loops](#loops)
7. [Functions](#functions)
8. [Lambdas](#lambdas)
9. [Closures](#closures)
10. [Arrays](#arrays)
11. [Dictionaries](#dictionaries)
12. [User Input and File I/O](#user-input-and-file-io)
13. [Introspection and Control](#introspection-and-control)
14. [Complete Examples](#complete-examples)
15. [Under the Hood](#under-the-hood)

---

## Getting Started

### Install

```bash
cargo build --release
```

### Run a file

```bash
cargo run -- examples/hello.wsp
```

### Inspect compiled bytecode

```bash
cargo run -- --dump examples/hello.wsp
```

```
== <main> ==
0000     1  PUSH_CONST           0    'Hello, Whispem!'
0002     1  STORE                1    'message'
0004     2  LOAD                 1    'message'
0006     2  PRINT
0007     2  HALT
```

### Interactive REPL

```bash
cargo run
```

```
Whispem v5.0.0 — REPL
Type 'exit' or press Ctrl-D to quit.

>>> let x = 42
>>> print x
42
```

### Run the test suite

```bash
cargo test             # 130 Rust tests
./tests/run_tests.sh   # 37 autonomous tests
```

---

## Variables and Types

```wsp
let name    = "Whispem"
let version = 5.0
let ready   = true
```

Variables are declared with `let`. Types are inferred automatically. There are seven types:

| Type | Examples | `type_of` |
|------|----------|-----------|
| `number` | `42`, `3.14`, `-7` | `"number"` |
| `string` | `"hello"`, `""` | `"string"` |
| `bool` | `true`, `false` | `"bool"` |
| `array` | `[1, 2, 3]` | `"array"` |
| `dict` | `{"key": "value"}` | `"dict"` |
| `function` | `fn(x){return x}`, closures | `"function"` |
| `none` | returned by void functions | `"none"` |

To update a variable, use `let` again:

```wsp
let counter = 0
let counter = counter + 1
print counter   # 1
```

---

## Expressions and Operators

### Arithmetic

```wsp
print 10 + 3    # 13
print 10 - 3    # 7
print 10 * 3    # 30
print 10 / 3    # 3.333...
print 10 % 3    # 1
```

### Comparisons and logic

```wsp
print 10 == 10   # true
print 10 != 5    # true
print true and false   # false
print true or false    # true
print not true         # false
```

`and` and `or` short-circuit:

```wsp
let r = false and expensive_call()   # false — call never runs
let r = true  or  expensive_call()   # true  — call never runs
```

---

## Strings and F-strings

### Plain strings

```wsp
let greeting = "Hello"
let name = "Whispem"
print greeting + ", " + name + "!"
```

### F-strings

`f"..."` strings with `{expr}` interpolation:

```wsp
let name  = "Em"
let score = 42
print f"Hello, {name}!"
print f"Score: {score}, doubled: {score * 2}"
print f"{length([1, 2, 3])} items"
```

Any expression works inside `{}` — variables, arithmetic, function calls. F-strings compile to `+` concatenation chains, so there's no performance overhead.

### Escape sequences

```wsp
print "Line one\nLine two"
print "She said \"hello\""
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

`else if` chains (native syntax):

```wsp
let score = 85

if score >= 90 {
    print "A"
} else if score >= 80 {
    print "B"
} else if score >= 70 {
    print "C"
} else {
    print "F"
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
```

### For

```wsp
for fruit in ["apple", "banana", "cherry"] {
    print fruit
}

for n in range(0, 5) {
    print n
}
```

### Break and continue

```wsp
for n in range(1, 20) {
    if n > 10 { break }
    if n % 2 == 0 { continue }
    print n
}
```

---

## Functions

```wsp
fn greet(name) {
    return "Hello, " + name + "!"
}
print greet("world")
```

### Recursion

```wsp
fn factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
print factorial(5)    # 120
```

### Forward calls

```wsp
print triple(4)   # 12 — works even though triple is defined below

fn triple(n) {
    return n * 3
}
```

### Scope

Variables at the top level are globals. Variables inside a function are local. Functions can read globals but cannot mutate them.

---

## Lambdas

`fn(params) { body }` is a first-class expression:

```wsp
# Store in a variable
let double = fn(x) { return x * 2 }
print double(7)   # 14

# Pass as an argument
fn apply(f, x) { return f(x) }
print apply(fn(n) { return n * n }, 5)   # 25

# Return from a function
fn make_double() { return fn(x) { return x * 2 } }
print make_double()(7)   # 14

# Store in an array
let ops = [fn(x) { return x + 1 }, fn(x) { return x * 2 }]
print ops[0](10)   # 11
print ops[1](10)   # 20

# Call immediately
print fn(x) { return x * 2 }(7)   # 14
```

---

## Closures

A lambda defined inside a function captures variables from the enclosing scope:

```wsp
fn make_adder(n) {
    return fn(x) { return x + n }
}
let add5 = make_adder(5)
print add5(3)    # 8
print add5(10)   # 15
```

Captured variables are **shared and mutable**:

```wsp
fn make_counter() {
    let count = 0
    return fn() {
        let count = count + 1
        return count
    }
}
let c = make_counter()
print c()   # 1
print c()   # 2
print c()   # 3
```

Each call to `make_counter()` creates an independent counter. Two closures in the same scope share the same cell:

```wsp
fn make_pair() {
    let n = 0
    let inc = fn() { let n = n + 1 }
    let get = fn() { return n }
    return [inc, get]
}
let p = make_pair()
p[0]()
p[0]()
print p[1]()   # 2
```

Closures can be nested:

```wsp
fn outer(a) {
    return fn(b) {
        return fn(c) { return a + b + c }
    }
}
print outer(1)(2)(3)   # 6
```

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

### Built-in functions

```wsp
let nums = [1, 2, 3]
let nums = push(nums, 4)              # [1, 2, 3, 4]
let last = pop([1, 2, 3])             # 3
let rev  = reverse([1, 2, 3])         # [3, 2, 1]
let mid  = slice([1,2,3,4,5], 1, 4)  # [2, 3, 4]
let seq  = range(0, 5)               # [0, 1, 2, 3, 4]
```

### Iterating

```wsp
let total = 0
for n in [10, 20, 30, 40] {
    let total = total + n
}
print total   # 100
```

### Arrays of functions

```wsp
let transforms = [
    fn(x) { return x + 1 },
    fn(x) { return x * 2 },
    fn(x) { return x * x },
]
for f in transforms {
    print f(5)   # 6, 10, 25
}
```

---

## Dictionaries

```wsp
let person = {"name": "Em", "city": "Marseille", "age": 26}
print person["name"]   # Em
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
print keys(d)           # [a, b, c]  (sorted)
print values(d)         # [1, 2, 3]
print length(d)         # 3
```

---

## User Input and File I/O

```wsp
let name = input("What's your name? ")
print f"Hello, {name}!"

write_file("output.txt", "Hello from Whispem!")
let content = read_file("output.txt")
print content
```

Script arguments:

```wsp
let script_args = args()
if length(script_args) == 0 {
    print "Usage: script.wsp <name>"
    exit(1)
}
print f"Hello, {script_args[0]}!"
```

---

## Introspection and Control

### `type_of(value)`

```wsp
fn safe_double(x) {
    if type_of(x) != "number" {
        return "error: expected number, got " + type_of(x)
    }
    return x * 2
}
print safe_double(5)      # 10
print safe_double("hi")   # error: expected number, got string
print type_of(fn(x){return x})  # function
```

### `assert(condition, message?)`

```wsp
fn process(items) {
    assert(type_of(items) == "array", "process() expects an array")
    assert(length(items) > 0, "items must not be empty")
    for n in items {
        assert(type_of(n) == "number", "all items must be numbers")
    }
}
```

### `exit(code?)`

```wsp
if length(args()) == 0 {
    print "Usage: script.wsp <name>"
    exit(1)
}
```

---

## Complete Examples

### FizzBuzz with `else if`

```wsp
for n in range(1, 101) {
    if n % 15 == 0 { print "FizzBuzz" }
    else if n % 3 == 0 { print "Fizz" }
    else if n % 5 == 0 { print "Buzz" }
    else { print n }
}
```

### Phonebook

```wsp
fn add(book, name, number) {
    book[name] = number
    return book
}
fn lookup(book, name) {
    if has_key(book, name) { return book[name] }
    return "not found"
}
let phonebook = {}
let phonebook = add(phonebook, "Alice", "06 12 34 56 78")
print lookup(phonebook, "Alice")    # 06 12 34 56 78
print lookup(phonebook, "Charlie")  # not found
```

### Closures as event handlers

```wsp
fn make_logger(prefix) {
    return fn(msg) {
        print f"[{prefix}] {msg}"
    }
}
let info  = make_logger("INFO")
let error = make_logger("ERROR")
info("server started")    # [INFO] server started
error("connection lost")  # [ERROR] connection lost
```

### Higher-order functions

```wsp
fn map_array(arr, f) {
    let result = []
    for item in arr {
        let result = push(result, f(item))
    }
    return result
}

fn filter_array(arr, pred) {
    let result = []
    for item in arr {
        if pred(item) {
            let result = push(result, item)
        }
    }
    return result
}

let nums    = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
let evens   = filter_array(nums,   fn(n) { return n % 2 == 0 })
let doubled = map_array(evens, fn(n) { return n * 2 })
print doubled   # [4, 8, 12, 16, 20]
```

---

## Under the Hood

Since v2.0.0, Whispem compiles source code to bytecode before executing it:

```
Source → Lexer → Parser → AST → Compiler → Bytecode → VM
```

Since v3.0.0, bytecode can be saved to `.whbc` files and loaded directly.

Since v5.0.0, the compiler performs **upvalue analysis** — it walks enclosing function scopes to detect captured variables and emits `MAKE_CLOSURE` instructions with inline variable-name descriptors.

You can inspect compiled bytecode with `--dump`:

```bash
cargo run -- --dump examples/fizzbuzz_proper.wsp
```

The VM has **38 opcodes** and two implementations: `src/vm.rs` (Rust reference) and `vm/wvm.c` (C standalone). Both produce identical output.

See [`docs/vm.md`](vm.md) for the complete VM specification.

---

**Whispem v5.0.0**
*You've seen the whole language. Everything Whispem has is in this document.*