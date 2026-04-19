# Whispem Tutorial

**Version 6.0.0**

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
10. [map, filter, reduce](#map-filter-reduce)
11. [Arrays](#arrays)
12. [Dictionaries](#dictionaries)
13. [User Input and File I/O](#user-input-and-file-io)
14. [Introspection and Control](#introspection-and-control)
15. [Complete Examples](#complete-examples)
16. [Under the Hood](#under-the-hood)

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

### Interactive REPL

```bash
cargo run
```

```
Whispem v6.0.0 — REPL
Type 'exit' or press Ctrl-D to quit.

>>> let x = 42
>>> print x
42
```

### Run the test suite

```bash
cargo test             # 153 Rust tests
./tests/run_tests.sh   # autonomous tests
```

---

## Variables and Types

```wsp
let name    = "Whispem"
let version = 6.0
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

```wsp
print 10 + 3    # 13
print 10 - 3    # 7
print 10 * 3    # 30
print 10 / 3    # 3.333...
print 10 % 3    # 1
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
print "Hello" + ", " + "Whispem" + "!"
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

Any expression works inside `{}`. F-strings compile to `+` concatenation — no runtime overhead.

---

## Conditionals

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
for fruit in ["apple", "banana", "cherry"] { print fruit }
for n in range(0, 5) { print n }
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

Forward calls work — you can call a function before it is defined. Arity is checked at call time.

---

## Lambdas

`fn(params) { body }` is a first-class expression:

```wsp
let double = fn(x) { return x * 2 }
print double(7)   # 14

fn apply(f, x) { return f(x) }
print apply(fn(n) { return n * n }, 5)   # 25

let ops = [fn(x) { return x + 1 }, fn(x) { return x * 2 }]
print ops[0](10)   # 11
print ops[1](10)   # 20

print fn(x) { return x * 2 }(7)   # 14 — immediate call
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

Each call to `make_counter()` creates an independent counter. Closures nest to arbitrary depth:

```wsp
fn outer(a) {
    return fn(b) {
        return fn(c) { return a + b + c }
    }
}
print outer(1)(2)(3)   # 6
```

---

## map, filter, reduce

Three higher-order builtins added in v6. They accept arrays and closures (including named functions and lambdas).

### map

`map(array, f)` — applies `f` to every element, returns a new array:

```wsp
print map([1, 2, 3, 4], fn(x) { return x * 2 })
# [2, 4, 6, 8]

fn square(n) { return n * n }
print map([1, 2, 3, 4, 5], square)
# [1, 4, 9, 16, 25]
```

### filter

`filter(array, pred)` — keeps elements for which `pred` returns truthy:

```wsp
print filter([1, 2, 3, 4, 5, 6], fn(n) { return n % 2 == 0 })
# [2, 4, 6]

fn make_gt(threshold) { return fn(n) { return n > threshold } }
print filter([1, 5, 3, 8, 2, 7], make_gt(4))
# [5, 8, 7]
```

### reduce

`reduce(array, f, initial)` — folds `f(accumulator, element)` left-to-right:

```wsp
print reduce([1, 2, 3, 4, 5], fn(acc, n) { return acc + n }, 0)
# 15

print reduce([1, 2, 3, 4], fn(acc, n) { return acc * n }, 1)
# 24

print reduce(["b", "c", "d"], fn(acc, s) { return acc + s }, "a")
# abcd
```

### Composing them

```wsp
# Sum of squares of even numbers from 1 to 10
let nums    = range(1, 11)
let evens   = filter(nums,   fn(n) { return n % 2 == 0 })
let squares = map(evens,     fn(n) { return n * n })
let total   = reduce(squares, fn(acc, n) { return acc + n }, 0)
print total   # 220
```

---

## Arrays

```wsp
let fruits = ["apple", "banana", "cherry"]
print fruits[0]        # apple
print length(fruits)   # 3

let scores = [10, 20, 30]
scores[1] = 99

let nums = push([1, 2, 3], 4)         # [1, 2, 3, 4]
let rev  = reverse([1, 2, 3])         # [3, 2, 1]
let mid  = slice([1,2,3,4,5], 1, 4)  # [2, 3, 4]
let seq  = range(0, 5)               # [0, 1, 2, 3, 4]
```

---

## Dictionaries

```wsp
let person = {"name": "Em", "city": "Marseille", "age": 26}
print person["name"]   # Em

person["city"] = "Paris"
person["job"]  = "developer"

print has_key(person, "name")   # true
print keys(person)              # [age, city, job, name] (sorted)
print length(person)            # 4
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

### FizzBuzz

```wsp
for n in range(1, 101) {
    if n % 15 == 0 { print "FizzBuzz" }
    else if n % 3 == 0 { print "Fizz" }
    else if n % 5 == 0 { print "Buzz" }
    else { print n }
}
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

### Higher-order data pipeline

```wsp
let data = [3, -1, 4, -1, 5, -9, 2, -6, 5, 3]

let positives = filter(data, fn(n) { return n > 0 })
let doubled   = map(positives, fn(n) { return n * 2 })
let sum       = reduce(doubled, fn(acc, n) { return acc + n }, 0)
print f"Sum of doubled positives: {sum}"
```

### Word frequency counter

```wsp
fn count_words(words) {
    let c = {}
    for w in words {
        if has_key(c, w) { c[w] = c[w] + 1 }
        else             { c[w] = 1 }
    }
    return c
}

let freq = count_words(["rust", "whispem", "rust", "language", "rust"])
print freq["rust"]       # 3
print freq["whispem"]    # 1
```

---

## Under the Hood

Since v2.0.0, Whispem compiles source code to bytecode before executing it:

```
Source → Lexer → Parser → AST → Compiler → Bytecode → VM
```

Since v6.0.0, the VM has three execution entry points:

- `execute()` — the main loop, runs until `HALT`
- `execute_until(depth)` — runs until the frame stack shrinks to `depth`, used by `map`/`filter`/`reduce` to call closures
- `step(op)` — executes a single non-terminal opcode; shared by both loops

You can inspect compiled bytecode with `--dump`:

```bash
cargo run -- --dump examples/fizzbuzz_proper.wsp
```

The VM has **38 opcodes**. `map`, `filter`, and `reduce` require no new opcodes — they are pure builtins dispatched through `CALL`, calling closures via `invoke_closure`.

See [`docs/vm.md`](vm.md) for the complete VM specification.

---

**Whispem v6.0.0**
*You've seen the whole language. Everything Whispem has is in this document.*