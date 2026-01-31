# Whispem Syntax

This document describes the syntax of the Whispem programming language.

Whispem is line-oriented and whitespace-tolerant.
There are no semicolons.

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

Whispem currently supports:

- **Numbers** (floating point)
- **Strings** (with escape sequences)
- **Booleans** (`true`, `false`)

Types are inferred automatically.

---

## Expressions

Whispem supports arithmetic expressions with operator precedence.
```wsp
let x = 10 + 5 * 2      # 20
let y = (10 + 5) * 2    # 30
```

### Arithmetic Operators

- `+` addition (also string concatenation)
- `-` subtraction
- `*` multiplication
- `/` division

### Unary Operators

- `-` negation
- `not` logical negation
- `!` logical negation (alias)
```wsp
let negative = -42
let opposite = not true
```

---

## Comparisons

Expressions can be compared using:

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

**Note:** Logical operators use short-circuit evaluation:
- `and` stops if the left side is false
- `or` stops if the left side is true

---

## Conditionals

Conditional execution uses `if / else`.
```wsp
if x < 10 {
    print x
} else {
    print 10
}
```

Conditions can be nested:
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

Blocks are delimited with `{ }`.

---

## Loops

### While Loops

Repeat a block while a condition is true:
```wsp
let counter = 0

while counter < 5 {
    print counter
    let counter = counter + 1
}
```

**Warning:** Make sure your condition eventually becomes false,
or you'll create an infinite loop!

---

## Functions

### Function Declaration

Define a function with `fn`:
```wsp
fn greet(name) {
    print "Hello, " + name + "!"
}
```

### Function Parameters

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

Functions without an explicit `return` return nothing.

### Function Calls

Call a function by its name with parentheses:
```wsp
greet("World")
let sum = add(10, 20)
```

### Recursion

Functions can call themselves:
```wsp
fn factorial(n) {
    if n <= 1 {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}

print factorial(5)  # 120
```

### Variable Scope

- Variables declared inside functions are **local** to that function
- Variables declared outside functions are **global**
- Function parameters are local to the function
```wsp
let x = 10  # Global

fn test() {
    let y = 20  # Local to test()
    print x     # Can access global
    print y     # Can access local
}

test()
# print y would fail - y doesn't exist here
```

---

## Strings

String literals support escape sequences:

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
let first = "Hello, "
let second = "World!"
let greeting = first + second
print greeting  # Hello, World!
```

---

## Print

The `print` keyword outputs a value to standard output.
```wsp
print x
print "Hello"
print true
```

Each `print` statement outputs on a new line.

---

## Comments

Comments start with `#` and continue until the end of the line.
```wsp
# This is a comment
let x = 10  # This is also a comment
```

---

## Operator Precedence

From highest to lowest:

1. Parentheses `( )`
2. Function calls `func(args)`
3. Unary operators `-`, `not`, `!`
4. Multiplication and division `*`, `/`
5. Addition and subtraction `+`, `-`
6. Comparisons `<`, `>`, `<=`, `>=`, `==`, `!=`
7. Logical AND `and`
8. Logical OR `or`

---

## Program Structure

A Whispem program is a sequence of statements executed from top to bottom.

Functions must be defined before they are called.

Example:
```wsp
# Define function
fn add(a, b) {
    return a + b
}

# Use function
let result = add(5, 10)
print result
```

---

## Reserved Keywords

The following words are reserved and cannot be used as variable or function names:

- `let`
- `print`
- `if`
- `else`
- `while`
- `and`
- `or`
- `not`
- `fn`
- `return`
- `true`
- `false`
