# Whispem Syntax Reference

**Version 1.5.0**

This document provides a complete reference for the Whispem programming language syntax.

Whispem is line-oriented and whitespace-tolerant. There are no semicolons.

---

## Table of Contents

1. [Variables](#variables)
2. [Types](#types)
3. [Arrays](#arrays)
4. [Dictionaries](#dictionaries)
5. [Expressions](#expressions)
6. [Comparisons](#comparisons)
7. [Logical Operators](#logical-operators)
8. [Conditionals](#conditionals)
9. [Loops](#loops)
10. [Functions](#functions)
11. [Strings](#strings)
12. [Built-in Functions](#built-in-functions)
13. [Comments](#comments)
14. [Operator Precedence](#operator-precedence)
15. [Reserved Keywords](#reserved-keywords)
16. [Error Messages](#error-messages)

---

## Variables

Variables are declared using `let`.
```wsp
let x = 10
let name = "Whispem"
let is_valid = true
```

Variables can be reassigned:
```wsp
let counter = 0
let counter = counter + 1
```

---

## Types

Whispem supports these types:

- **Numbers** (floating point): `42`, `3.14`, `-10`
- **Strings** (with escape sequences): `"hello"`, `"line\n"`
- **Booleans**: `true`, `false`
- **Arrays** (ordered collections): `[1, 2, 3]`, `["a", "b"]`
- **Dictionaries** (key-value maps): `{"key": value}`

Types are inferred automatically.

---

## Arrays

### Array Literals

```wsp
let numbers = [1, 2, 3, 4, 5]
let names   = ["Alice", "Bob", "Charlie"]
let mixed   = [1, "hello", true, [1, 2, 3]]
let empty   = []
```

### Array Indexing

Access elements with `[index]` (0-based):
```wsp
let numbers = [10, 20, 30]
let first   = numbers[0]   # 10
let second  = numbers[1]   # 20
```

### Array Assignment

```wsp
let numbers = [1, 2, 3]
numbers[0] = 10
print numbers  # [10, 2, 3]
```

---

## Dictionaries

### Dictionary Literals

```wsp
let person = {"name": "Em", "age": 26, "city": "Marseille"}
let empty  = {}
```

Keys must be strings or numbers. Values can be any type.

### Dictionary Access

```wsp
print person["name"]   # Em
print person["age"]    # 26
```

### Dictionary Assignment

```wsp
person["city"] = "Paris"   # update existing key
person["job"]  = "developer"  # add new key
```

### Multi-line Dictionary

```wsp
let config = {
    "host": "localhost",
    "port": 8080,
    "debug": true
}
```

---

## Expressions

### Arithmetic Operators

```wsp
let x = 10 + 5 * 2    # 20
let y = (10 + 5) * 2  # 30
let r = 17 % 5        # 2
```

- `+` addition (also string concatenation)
- `-` subtraction
- `*` multiplication
- `/` division
- `%` modulo

### Unary Operators

```wsp
let negative = -42
let opposite = not true
```

---

## Comparisons

- `<` less than
- `>` greater than
- `<=` less than or equal
- `>=` greater than or equal
- `==` equal to
- `!=` not equal to

```wsp
if x > 5 {
    print x
}

if name == "Whispem" {
    print "Correct!"
}
```

---

## Logical Operators

- `and` — both conditions must be true
- `or` — at least one condition must be true
- `not` — negates a boolean

```wsp
if x > 5 and x < 15 {
    print "x is in range"
}

if not is_error {
    print "No errors!"
}
```

**Short-circuit evaluation:**
- `and` stops if the left side is false
- `or` stops if the left side is true

---

## Conditionals

```wsp
if x < 10 {
    print x
} else {
    print 10
}
```

---

## Loops

### While Loops

```wsp
let counter = 0

while counter < 5 {
    print counter
    let counter = counter + 1
}
```

### For Loops

```wsp
for item in [1, 2, 3, 4, 5] {
    print item
}

for i in range(0, 10) {
    print i
}
```

### Break and Continue

```wsp
for num in range(1, 100) {
    if num > 10 { break }
    if num % 2 == 0 { continue }
    print num
}
```

---

## Functions

```wsp
fn greet(name) {
    return "Hello, " + name + "!"
}

fn factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

print greet("World")
print factorial(5)  # 120
```

### Variable Scope

```wsp
let x = 10  # global

fn test() {
    let y = 20  # local
    print x     # can access global
    print y
}
```

---

## Strings

### Escape Sequences

- `\n` — newline
- `\t` — tab
- `\r` — carriage return
- `\\` — backslash
- `\"` — double quote

### Concatenation

```wsp
let greeting = "Hello, " + "World!"
```

---

## Built-in Functions

### Array Functions

**`length(array/string/dict)`** — get length
```wsp
length([1, 2, 3])      # 3
length("hello")        # 5
length({"a": 1})       # 1
```

**`push(array, item)`** — append (returns new array)
```wsp
let arr = push([1, 2], 3)  # [1, 2, 3]
```

**`pop(array)`** — remove and return last element
```wsp
let last = pop([1, 2, 3])  # 3
```

**`reverse(array)`** — reverse array
```wsp
reverse([1, 2, 3])  # [3, 2, 1]
```

**`slice(array, start, end)`** — sub-array
```wsp
slice([1, 2, 3, 4, 5], 1, 4)  # [2, 3, 4]
```

**`range(start, end)`** — number sequence
```wsp
range(0, 5)  # [0, 1, 2, 3, 4]
```

### Dictionary Functions

**`keys(dict)`** — sorted array of keys
```wsp
keys({"b": 2, "a": 1})  # [a, b]
```

**`values(dict)`** — array of values sorted by key
```wsp
values({"b": 2, "a": 1})  # [1, 2]
```

**`has_key(dict, key)`** — check if key exists
```wsp
has_key({"name": "Em"}, "name")   # true
has_key({"name": "Em"}, "phone")  # false
```

### I/O Functions

**`input(prompt)`** — read user input
```wsp
let name = input("Enter your name: ")
```

**`read_file(filename)`** — read file contents
```wsp
let content = read_file("data.txt")
```

**`write_file(filename, content)`** — write to file
```wsp
write_file("output.txt", "Hello!")
```

---

## Comments

```wsp
# This is a comment
let x = 10  # inline comment
```

---

## Operator Precedence

From highest to lowest:

1. Parentheses `( )`
2. Array/dict indexing `[index]`
3. Function calls `func(args)`
4. Unary operators `-`, `not`, `!`
5. Multiplication, division, modulo `*`, `/`, `%`
6. Addition and subtraction `+`, `-`
7. Comparisons `<`, `>`, `<=`, `>=`, `==`, `!=`
8. Logical AND `and`
9. Logical OR `or`

---

## Reserved Keywords

`let` `print` `if` `else` `while` `for` `in` `and` `or` `not` `fn` `return` `break` `continue` `true` `false` `length` `push` `pop` `reverse` `slice` `range` `input` `read_file` `write_file` `keys` `values` `has_key`

---

## Error Messages

Whispem provides helpful error messages with line and column numbers:

```
[line 3, col 12] Error: Undefined variable: 'counter'
[line 7, col 5]  Error: Array index 10 out of bounds (array length: 5)
[line 12, col 1] Error: Function 'add' expected 2 arguments, got 3
[line 15, col 8] Error: Division by zero
[line 20, col 3] Error: Type error: expected number, found string
Error: Failed to read file 'data.txt': No such file or directory
```

---

**Whispem v1.5.0 — Complete Syntax Reference**