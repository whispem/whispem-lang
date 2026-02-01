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
- **Arrays** (ordered collections)

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

Access array elements with `[index]`:
```wsp
let numbers = [10, 20, 30]
let first = numbers[0]   # 10
let second = numbers[1]  # 20
```

Indices start at 0.

### Array Assignment

Modify array elements:
```wsp
let numbers = [1, 2, 3]
numbers[0] = 10
numbers[2] = 30
print numbers  # [10, 2, 30]
```

### Built-in Array Functions

**`length(array)`** - Get array length:
```wsp
let numbers = [1, 2, 3, 4, 5]
let len = length(numbers)  # 5
```

Also works with strings:
```wsp
let text = "hello"
let len = length(text)  # 5
```

**`push(array, item)`** - Add element to array:
```wsp
let numbers = [1, 2, 3]
let new_numbers = push(numbers, 4)
print new_numbers  # [1, 2, 3, 4]
```

Note: `push()` returns a new array; it doesn't modify the original.

### Iterating Over Arrays

Use a while loop with `length()`:
```wsp
let items = [10, 20, 30, 40]
let i = 0

while i < length(items) {
    print items[i]
    let i = i + 1
}
```

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

### Arrays and Functions

Pass arrays to functions:
```wsp
fn sum_array(arr) {
    let total = 0
    let i = 0
    while i < length(arr) {
        let total = total + arr[i]
        let i = i + 1
    }
    return total
}

let numbers = [1, 2, 3, 4, 5]
print sum_array(numbers)  # 15
```

Return arrays from functions:
```wsp
fn make_array(size) {
    let arr = []
    let i = 0
    while i < size {
        let arr = push(arr, i)
        let i = i + 1
    }
    return arr
}

let numbers = make_array(5)
print numbers  # [0, 1, 2, 3, 4]
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
print [1, 2, 3]
```

Each `print` statement outputs on a new line.

Arrays are printed in `[1, 2, 3]` format.

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
2. Array indexing `[index]`
3. Function calls `func(args)`
4. Unary operators `-`, `not`, `!`
5. Multiplication and division `*`, `/`
6. Addition and subtraction `+`, `-`
7. Comparisons `<`, `>`, `<=`, `>=`, `==`, `!=`
8. Logical AND `and`
9. Logical OR `or`

---

## Program Structure

A Whispem program is a sequence of statements executed from top to bottom.

Functions must be defined before they are called.

Example:
```wsp
# Define function
fn process_array(arr) {
    let i = 0
    while i < length(arr) {
        print arr[i]
        let i = i + 1
    }
}

# Create and use array
let numbers = [1, 2, 3, 4, 5]
process_array(numbers)
```

---

## Built-in Functions

Whispem provides the following built-in functions:

- **`length(value)`** - Returns the length of an array or string
- **`push(array, item)`** - Returns a new array with item appended

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
- `length`
- `push`
- `true`
- `false`
