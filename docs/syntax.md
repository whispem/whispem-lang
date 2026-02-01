# Whispem Syntax Reference

**Version 1.0.0**

This document provides a complete reference for the Whispem programming language syntax.

Whispem is line-oriented and whitespace-tolerant. There are no semicolons.

---

## Table of Contents

1. [Variables](#variables)
2. [Types](#types)
3. [Arrays](#arrays)
4. [Expressions](#expressions)
5. [Comparisons](#comparisons)
6. [Logical Operators](#logical-operators)
7. [Conditionals](#conditionals)
8. [Loops](#loops)
9. [Functions](#functions)
10. [Strings](#strings)
11. [Built-in Functions](#built-in-functions)
12. [Comments](#comments)
13. [Operator Precedence](#operator-precedence)
14. [Reserved Keywords](#reserved-keywords)

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

Types are inferred automatically.

---

## Arrays

### Array Literals

Create arrays with square brackets:
```wsp
let numbers = [1, 2, 3, 4, 5]
let names = ["Alice", "Bob", "Charlie"]
let mixed = [1, "hello", true, [1, 2, 3]]
let empty = []
```

Arrays can contain any type, including nested arrays.

### Array Indexing

Access array elements with `[index]` (0-based):
```wsp
let numbers = [10, 20, 30]
let first = numbers[0]   # 10
let second = numbers[1]  # 20
```

### Array Assignment

Modify array elements:
```wsp
let numbers = [1, 2, 3]
numbers[0] = 10
numbers[2] = 30
print numbers  # [10, 2, 30]
```

---

## Expressions

### Arithmetic Operators
```wsp
let x = 10 + 5 * 2      # 20
let y = (10 + 5) * 2    # 30
```

- `+` addition (also string concatenation)
- `-` subtraction
- `*` multiplication
- `/` division

### Unary Operators
```wsp
let negative = -42
let opposite = not true
```

- `-` negation
- `not` logical negation
- `!` logical negation (alias)

---

## Comparisons

Compare values using:

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

Combine boolean expressions with:

- `and` — both conditions must be true
- `or` — at least one condition must be true
- `not` — negates a boolean
```wsp
if x > 5 and x < 15 {
    print "x is in range"
}

if x < 0 or x > 100 {
    print "x is out of range"
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

Use `if / else` for conditional execution:
```wsp
if x < 10 {
    print x
} else {
    print 10
}
```

Nested conditions:
```wsp
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

Blocks use `{ }`.

---

## Loops

### While Loops

Repeat while a condition is true:
```wsp
let counter = 0

while counter < 5 {
    print counter
    let counter = counter + 1
}
```

### For Loops

Iterate over arrays:
```wsp
for item in [1, 2, 3, 4, 5] {
    print item
}

for name in ["Alice", "Bob", "Charlie"] {
    print "Hello, " + name
}
```

Use `range()` for number sequences:
```wsp
for i in range(0, 10) {
    print i
}
```

### Break and Continue

Exit early with `break`:
```wsp
for num in range(1, 100) {
    if num > 10 {
        break
    }
    print num
}
```

Skip to next iteration with `continue`:
```wsp
for num in range(1, 10) {
    if num == 5 {
        continue
    }
    print num
}
```

---

## Functions

### Function Declaration

Define functions with `fn`:
```wsp
fn greet(name) {
    print "Hello, " + name + "!"
}
```

### Parameters

Functions can have zero or more parameters:
```wsp
fn say_hello() {
    print "Hello!"
}

fn add(a, b) {
    return a + b
}
```

### Return Statement

Use `return` to return a value:
```wsp
fn multiply(x, y) {
    return x * y
}

let result = multiply(5, 3)
print result  # 15
```

### Recursion

Functions can call themselves:
```wsp
fn factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

print factorial(5)  # 120
```

### Variable Scope

- Variables inside functions are **local**
- Variables outside functions are **global**
- Function parameters are local
```wsp
let x = 10  # Global

fn test() {
    let y = 20  # Local
    print x     # Can access global
    print y     # Can access local
}

test()
# print y would fail - y doesn't exist here
```

---

## Strings

### String Literals

Strings use double quotes:
```wsp
let message = "Hello, World!"
```

### Escape Sequences

- `\n` — newline
- `\t` — tab
- `\r` — carriage return
- `\\` — backslash
- `\"` — double quote
```wsp
let message = "Hello\nWorld"
print message
```

Output:
```
Hello
World
```

### String Concatenation

Use `+` to concatenate strings:
```wsp
let greeting = "Hello, " + "World!"
print greeting  # Hello, World!
```

---

## Built-in Functions

### Array Functions

**`length(array/string)`** - Get length:
```wsp
let len = length([1, 2, 3])  # 3
let str_len = length("hello")  # 5
```

**`push(array, item)`** - Append element (returns new array):
```wsp
let arr = push([1, 2], 3)  # [1, 2, 3]
```

**`pop(array)`** - Remove and return last element:
```wsp
let last = pop([1, 2, 3])  # 3
```

**`reverse(array)`** - Reverse array:
```wsp
let rev = reverse([1, 2, 3])  # [3, 2, 1]
```

**`slice(array, start, end)`** - Get sub-array:
```wsp
let sub = slice([1, 2, 3, 4, 5], 1, 4)  # [2, 3, 4]
```

**`range(start, end)`** - Generate number sequence:
```wsp
let nums = range(0, 5)  # [0, 1, 2, 3, 4]
```

### I/O Functions

**`input(prompt)`** - Read user input:
```wsp
let name = input("Enter your name: ")
print "Hello, " + name
```

**`read_file(filename)`** - Read file contents:
```wsp
let content = read_file("data.txt")
print content
```

**`write_file(filename, content)`** - Write to file:
```wsp
write_file("output.txt", "Hello, World!")
```

### Output

**`print(value)`** - Print to console:
```wsp
print "Hello"
print 42
print [1, 2, 3]
```

---

## Comments

Comments start with `#` and continue to end of line:
```wsp
# This is a comment
let x = 10  # This is also a comment
```

---

## Operator Precedence

From highest to lowest:

1. Parentheses `( )`
2. Array indexing `[index]`
3. Function calls `func(args)`
4. Unary operators `-`, `not`, `!`
5. Multiplication and division `*`, `/`
6. Addition and subtraction `+`, `-`
7. Comparisons `<`, `>`, `<=`, `>=`, `==`, `!=`
8. Logical AND `and`
9. Logical OR `or`

---

## Reserved Keywords

These words cannot be used as variable or function names:

- `let`
- `print`
- `if`
- `else`
- `while`
- `for`
- `in`
- `and`
- `or`
- `not`
- `fn`
- `return`
- `break`
- `continue`
- `length`
- `push`
- `pop`
- `reverse`
- `slice`
- `range`
- `input`
- `read_file`
- `write_file`
- `true`
- `false`

---

## Error Messages

Whispem provides helpful error messages:
```
Error: Undefined variable: counter
Error: Array index 10 out of bounds (array length: 5)
Error: Function add expected 2 arguments, got 3
Error: Division by zero
Error: Failed to read file 'data.txt': No such file or directory
```

---

**Whispem v1.0.0 - Complete Syntax Reference**
