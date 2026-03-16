# Whispem Tutorial

**Version 4.0.0**

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
11. [Introspection and Control](#introspection-and-control)
12. [Complete Examples](#complete-examples)
13. [Under the Hood](#under-the-hood)

---

## Getting Started

### Install

The only dependency is a C compiler.

```bash
git clone https://github.com/whispem/whispem-lang.git
cd whispem-lang
make                                         # build the VM from vm/wvm.c
```

### Run a file

```bash
./wvm compiler/wsc.whbc examples/hello.wsp   # compile + run
```

### Inspect compiled bytecode

```bash
./wvm --dump examples/hello.whbc
```

```
== <main> ==
0000     1  PUSH_CONST           0    'Hello, Whispem!'
0002     1  STORE                1    'message'
0004     2  LOAD                 1    'message'
0006     2  PRINT
0007     2  HALT
```

### Compile to bytecode

```bash
./wvm compiler/wsc.whbc examples/hello.wsp   # → examples/hello.whbc
./wvm examples/hello.whbc                    # run precompiled — no recompilation
```

### Interactive REPL

```bash
./wvm
```

```
Whispem v4.0.0 — REPL
Type 'exit' or press Ctrl-D to quit.

>>> let x = 42
>>> print x
42
>>> exit
Bye!
```

### Run the test suite

```bash
./tests/run_tests.sh             # autonomous tests (no Rust needed)
```

### With the Rust reference implementation

```bash
cargo build --release
cargo run -- examples/hello.wsp
cargo run -- --compile examples/hello.wsp
cargo run -- --dump examples/hello.wsp
cargo run
cargo test
```

---

## Variables and Types

```wsp
let name    = "Whispem"
let version = 4.0
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

`and` and `or` short-circuit:

```wsp
let r = false and expensive_call()   # false — call never runs
let r = true  or  expensive_call()   # true  — call never runs
```

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

Basic `if / else`:

```wsp
let temperature = 18

if temperature > 20 {
    print "warm"
} else {
    print "cool"
}
```

`else if` chains (v4.0.0):

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

`else if` can also appear on the same line or on the next — both work:

```wsp
if x == 1 { print "one" }
else if x == 2 { print "two" }
else { print "other" }
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

### Recursion

```wsp
fn factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

print factorial(5)    # 120
print factorial(10)   # 3628800
```

### Arity is checked

```wsp
fn add(a, b) { return a + b }
add(1, 2, 3)   # Error: Function 'add' expected 2 arguments, got 3
```

### Scope

Variables at the top level are globals. Variables inside a function are local. Functions can read globals but cannot mutate them.

```wsp
let greeting = "Hello"

fn say(name) {
    print greeting + ", " + name   # reads global
}

say("Em")   # Hello, Em
```

### Forward calls

```wsp
print triple(4)   # 12 — works even though triple is defined below

fn triple(n) {
    return n * 3
}
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

Accessing a missing key raises a clear error:

```wsp
print d["z"]   # Error: key "z" not found in dict
```

Use `has_key` to guard access:

```wsp
if has_key(d, "z") {
    print d["z"]
} else {
    print "not found"
}
```

---

## User Input and File I/O

```wsp
let name = input("What's your name? ")
print "Hello, " + name + "!"

write_file("output.txt", "Hello from Whispem!")
let content = read_file("output.txt")
print content
```

Script arguments via `args()`:

```wsp
let script_args = args()
if length(script_args) == 0 {
    print "Usage: script.wsp <name>"
    exit(1)
}
print "Hello, " + script_args[0]
```

---

## Introspection and Control

### `type_of(value)`

Returns the runtime type as a string. Useful for defensive functions:

```wsp
fn safe_double(x) {
    if type_of(x) != "number" {
        return "error: expected number, got " + type_of(x)
    }
    return x * 2
}

print safe_double(5)      # 10
print safe_double("hi")   # error: expected number, got string
```

The six possible return values: `"number"`, `"string"`, `"bool"`, `"array"`, `"dict"`, `"none"`.

### `assert(condition, message?)`

Raises `Assertion failed: <message>` if the condition is falsy. Useful for catching incorrect assumptions early:

```wsp
fn process(items) {
    assert(type_of(items) == "array", "process() expects an array")
    assert(length(items) > 0, "items must not be empty")
    # ...
}
```

The message is optional — `assert(condition)` uses `"assertion failed"` as the default.

`assert` fails on any falsy value: `false`, `0`, `""`, `[]`, `{}`, `none`.

### `exit(code?)`

Terminates the program immediately with the given exit code (default `0`). The exit code is passed to the OS, so it can be read by shell scripts:

```wsp
let script_args = args()
if length(script_args) < 2 {
    print "Usage: script.wsp <input> <output>"
    exit(1)
}
```

`exit()` in the REPL also terminates the session.

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
```

### Validated data processing

```wsp
fn process(items) {
    assert(type_of(items) == "array", "expected array")
    assert(length(items) > 0, "array must not be empty")

    let total = 0
    for n in items {
        assert(type_of(n) == "number", "all items must be numbers")
        let total = total + n
    }
    return total
}

print process([3, 8, 12, 7])   # 30
```

### Script with argument handling

```wsp
let script_args = args()
if length(script_args) == 0 {
    print "Usage: script.wsp <name>"
    exit(1)
}

let name = script_args[0]
print "Hello, " + name + "!"
exit(0)
```

---

## Under the Hood

Since v2.0.0, Whispem compiles source code to bytecode before executing it:

```
Source → Lexer → Parser → AST → Compiler → Bytecode → VM
```

Since v3.0.0, the bytecode can be serialised to a `.whbc` file:

```
Source → ... → Compiler → serialise() → .whbc file
                                              ↓
                                        deserialise()
                                              ↓
                                             VM
```

You can inspect the compiled bytecode with `--dump`:

```bash
./wvm --dump examples/fizzbuzz_proper.whbc
```

The VM is a stack machine with **34 opcodes** and two implementations: `vm/wvm.c` (C, standalone) and `src/vm.rs` (Rust, reference). Both produce identical output.

`else if` compiles to exactly the same bytecode as nested `if / else { if ... }` — it is purely a lexer and parser transformation with no VM impact.

`assert` and `type_of` and `exit` compile as regular function calls (`CALL assert`, `CALL type_of`, `CALL exit`) — they are builtins resolved at call time in the VM.

See [`docs/vm.md`](vm.md) for the complete VM specification and the `.whbc` binary format.

---

**Whispem v4.0.0**
*You've seen the whole language. Everything Whispem has is in this document.*