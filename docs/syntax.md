# Whispem Syntax Reference

**Version 4.0.0**

Complete reference for the Whispem programming language syntax.

Whispem is line-oriented. No semicolons. Blocks delimited by `{` and `}`.

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

Variables are declared using `let`:

```wsp
let x = 10
let name = "Whispem"
let ready = true
```

To update a variable, use `let` again:

```wsp
let counter = 0
let counter = counter + 1
```

There is no bare assignment — only `let x = expr` and `x[i] = expr`.

---

## Types

| Type | Examples | Notes |
|------|----------|-------|
| `number` | `42`, `3.14`, `-7` | All numbers are `f64` |
| `string` | `"hello"`, `""` | UTF-8 |
| `bool` | `true`, `false` | |
| `array` | `[1, "two", true]` | Ordered, mixed types |
| `dict` | `{"key": "value"}` | Keys are always strings |
| `none` | — | Returned by void functions |

Types are inferred automatically.

---

## Arrays

### Literals

```wsp
let numbers = [1, 2, 3, 4, 5]
let names   = ["Alice", "Bob"]
let mixed   = [1, "hello", true]
let empty   = []
```

### Indexing (0-based)

```wsp
let fruits = ["apple", "banana", "cherry"]
print fruits[0]   # apple
print fruits[2]   # cherry
```

### Index assignment

```wsp
let scores = [10, 20, 30]
scores[1] = 99
print scores   # [10, 99, 30]
```

---

## Dictionaries

### Literals

```wsp
let person = {"name": "Em", "age": 26, "city": "Marseille"}
let empty  = {}
```

Keys must be strings. Values can be any type.

### Access

```wsp
print person["name"]   # Em
print person["age"]    # 26
```

Accessing a key that does not exist raises a clear runtime error:
`key "foo" not found in dict`

### Assignment

```wsp
person["city"] = "Paris"       # update existing key
person["job"]  = "developer"   # add new key
```

---

## Expressions

### Arithmetic

```wsp
10 + 5    # 15
10 - 5    # 5
10 * 5    # 50
10 / 3    # 3.333...
10 % 3    # 1  ← modulo
```

### Unary

```wsp
let n = -42
let b = not true
```

### String concatenation

```wsp
"Hello" + ", " + "world!"
```

Numbers and booleans are converted to string automatically when concatenated with a string:

```wsp
print "Count: " + 42   # Count: 42
```

---

## Comparisons

| Operator | Meaning |
|----------|---------|
| `==` | equal |
| `!=` | not equal |
| `<` | less than |
| `<=` | less than or equal |
| `>` | greater than |
| `>=` | greater than or equal |

Works on numbers, strings (lexicographic), and booleans.

---

## Logical Operators

```wsp
a and b   # true if both truthy
a or b    # true if at least one truthy
not a     # negates
```

`and` and `or` short-circuit: the left value becomes the result when the right side would not change the outcome.

**Truthiness:** a value is falsy if it is `false`, `0`, `""`, `[]`, `{}`, or `none`. Everything else is truthy.

---

## Conditionals

```wsp
if condition {
    ...
} else if other_condition {
    ...
} else {
    ...
}
```

Both `else if` and `else` are optional. `else if` chains are supported natively — no need to nest `if` inside `else`:

```wsp
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

`else if` can appear on the same line as `}` or on the next line — both are valid:

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
```

### For

```wsp
for item in [1, 2, 3] {
    print item
}

for i in range(0, 10) {
    print i
}
```

`for` desugars to a counter-based while loop at compile time. The iterable must be an array.

### Break and continue

```wsp
for n in range(1, 100) {
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
```

### Return values

A function with no explicit `return` returns `none`. A bare `return` also returns `none`.

### Arity checking

Calling a function with the wrong number of arguments raises a runtime error:
`Function 'add' expected 2 arguments, got 3`

### Scope

- Variables declared at the top level are globals.
- Variables declared inside a function are locals.
- Functions can read globals (via `LOAD_GLOBAL` in the bytecode).
- Functions cannot mutate globals — `STORE` inside a function writes to the frame's locals.

### Forward calls

Functions are compiled before the main body. You can call a function defined later in the file.

---

## Strings

### Escape sequences

| Sequence | Character |
|----------|-----------|
| `\n` | newline |
| `\t` | tab |
| `\r` | carriage return |
| `\\` | backslash |
| `\"` | double quote |

### Concatenation

```wsp
let full = "Hello" + ", " + "world!"
```

---

## Built-in Functions

Built-ins are resolved at call time by the VM before checking user-defined functions.

### Arrays

| Function | Signature | Description |
|----------|-----------|-------------|
| `length` | `(array\|string\|dict) → number` | Number of elements |
| `push` | `(array, value) → array` | New array with value appended |
| `pop` | `(array) → value` | Last element (error if empty) |
| `reverse` | `(array) → array` | New reversed array |
| `slice` | `(array, start, end) → array` | Sub-array `[start, end)` |
| `range` | `(start, end) → array` | Integer range `[start, end)` |

### Dictionaries

| Function | Signature | Description |
|----------|-----------|-------------|
| `keys` | `(dict) → array` | Sorted list of keys |
| `values` | `(dict) → array` | Values in key-sorted order |
| `has_key` | `(dict, key) → bool` | Check if key exists |

### Strings

| Function | Signature | Description |
|----------|-----------|-------------|
| `length` | `(string) → number` | Character count (UTF-8 aware) |
| `char_at` | `(string, index) → string` | Single character at index |
| `substr` | `(string, start, len) → string` | Substring |
| `ord` | `(string) → number` | Unicode codepoint of first character |
| `num_to_str` | `(number) → string` | Number to string |
| `str_to_num` | `(string) → number` | String to number |

### I/O

| Function | Signature | Description |
|----------|-----------|-------------|
| `input` | `(prompt?) → string` | Read line from stdin |
| `read_file` | `(path) → string` | Read file contents |
| `write_file` | `(path, content) → none` | Write string to file |
| `args` | `() → array` | Script arguments (after `.wsp` filename) |
| `write_hex` | `(path, hex) → none` | Decode hex string to bytes, write to file |
| `num_to_hex` | `(n) → string` | IEEE-754 f64 as 16-char hex string |

### Introspection and control (v4.0.0)

| Function | Signature | Description |
|----------|-----------|-------------|
| `type_of` | `(value) → string` | Runtime type: `"number"`, `"string"`, `"bool"`, `"array"`, `"dict"`, `"none"` |
| `assert` | `(condition, message?) → none` | Raises `AssertionFailed` if condition is falsy |
| `exit` | `(code?) → none` | Terminates the program with the given exit code (default `0`) |

---

## Comments

```wsp
# This is a comment
let x = 10   # inline comment
```

---

## Operator Precedence

From highest to lowest:

| Level | Operators |
|-------|-----------|
| 1 (highest) | `( )` parentheses |
| 2 | `[ ]` indexing |
| 3 | function calls |
| 4 | `-` (unary), `not` |
| 5 | `*`, `/`, `%` |
| 6 | `+`, `-` |
| 7 | `<`, `>`, `<=`, `>=`, `==`, `!=` |
| 8 | `and` |
| 9 (lowest) | `or` |

---

## Reserved Keywords

```
let  print  if  else  while  for  in  fn  return  break  continue
and  or  not  true  false  assert  type_of  exit
```

Built-in function names are also reserved:
```
length  push  pop  reverse  slice  range
input  read_file  write_file  args  write_hex
keys  values  has_key
char_at  substr  ord  num_to_str  str_to_num  num_to_hex
```

---

## Error Messages

Errors include source location as `[line N, col M]`:

```
[line 3, col 0]  Error: Undefined variable: 'counter'
[line 5, col 0]  Error: key "foo" not found in dict
[line 7, col 0]  Error: Array index 10 out of bounds (length: 5)
[line 9, col 0]  Error: Function 'add' expected 2 arguments, got 3
[line 12, col 0] Error: Division by zero
[line 15, col 0] Error: Type error: expected number, found string
[line 18, col 0] Error: Assertion failed: array must not be empty
```

---

**Whispem v4.0.0 — Complete Syntax Reference**