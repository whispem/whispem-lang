# Whispem Syntax Reference

**Version 5.0.0**

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
11. [Lambdas](#lambdas)
12. [Closures](#closures)
13. [F-strings](#f-strings)
14. [Strings](#strings)
15. [Built-in Functions](#built-in-functions)
16. [Comments](#comments)
17. [Operator Precedence](#operator-precedence)
18. [Reserved Keywords](#reserved-keywords)
19. [Error Messages](#error-messages)

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

| Type       | Examples                        | `type_of` |
|------------|---------------------------------|-----------|
| `number`   | `42`, `3.14`, `-7`              | `"number"` |
| `string`   | `"hello"`, `""`                 | `"string"` |
| `bool`     | `true`, `false`                 | `"bool"` |
| `array`    | `[1, "two", true]`              | `"array"` |
| `dict`     | `{"key": "value"}`              | `"dict"` |
| `function` | `fn(x) { return x }`, closures  | `"function"` |
| `none`     | returned by void functions      | `"none"` |

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

Keys must be strings. Values can be any type — including functions and closures.

### Access

```wsp
print person["name"]   # Em
print person["age"]    # 26
```

Accessing a key that does not exist raises: `key "foo" not found in dict`.

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

Numbers and booleans are coerced to string automatically:

```wsp
print "Count: " + 42   # Count: 42
```

### Chained calls

Calls can be chained on any expression value:

```wsp
make_adder(5)(3)        # 8
fns[0](10)              # calls closure stored at index 0
fn(x) { return x*2 }(7) # 14 — immediate lambda call
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

`and` and `or` short-circuit.

**Truthiness:** `false`, `0`, `""`, `[]`, `{}`, `none` are falsy. Everything else — including functions — is truthy.

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

Both `else if` and `else` are optional. `else if` is native syntax — no need to nest `if` inside `else`.

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
for item in [1, 2, 3] { print item }
for i in range(0, 10) { print i }
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

Functions can be called before they are defined (forward calls work). Arity is checked at call time.

### Scope

Top-level `let` bindings are globals. Variables inside a function are locals. Functions can read globals (via `LOAD_GLOBAL`); they cannot mutate globals.

### Return values

A function with no explicit `return` returns `none`. A bare `return` also returns `none`.

---

## Lambdas

`fn(params) { body }` is a first-class expression. It can be stored, passed, returned, and called immediately.

```wsp
# Store
let double = fn(x) { return x * 2 }
print double(7)   # 14

# Pass as argument
fn apply(f, x) { return f(x) }
print apply(fn(n) { return n * n }, 5)   # 25

# Return from function
fn make_double() { return fn(x) { return x * 2 } }
print make_double()(7)   # 14

# Store in array
let fns = [fn(x) { return x + 1 }, fn(x) { return x * 2 }]
print fns[0](10)   # 11
print fns[1](10)   # 20

# Immediate call
print fn(x) { return x * 2 }(7)   # 14
```

`type_of(fn(x){return x})` returns `"function"`.

---

## Closures

A function defined inside another function automatically captures variables from the enclosing scope.

```wsp
fn make_adder(n) {
    return fn(x) { return x + n }   # n is captured
}
let add5 = make_adder(5)
print add5(3)    # 8
print add5(10)   # 15
```

Captured variables are **shared and mutable** — all closures created in the same scope share the same cell:

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

Each call to `make_counter()` creates an independent counter:

```wsp
let c1 = make_counter()
let c2 = make_counter()
print c1()   # 1
print c1()   # 2
print c2()   # 1  ← independent
```

Closures can be nested to arbitrary depth:

```wsp
fn outer(a) {
    return fn(b) {
        return fn(c) { return a + b + c }
    }
}
print outer(1)(2)(3)   # 6
```

---

## F-strings

`f"..."` strings support `{expr}` interpolation. Any expression is valid inside braces.

```wsp
let name  = "Em"
let score = 42
print f"Hello, {name}!"
print f"Score: {score}, doubled: {score * 2}"
print f"{length([1, 2, 3])} items"
```

Escape sequences work normally inside f-strings. To include a literal `{` or `}`, use `\{` or `\}`.

F-strings compile to a chain of `+` concatenations — identical performance to hand-written concatenation. No runtime overhead.

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
| `\{` | literal `{` (in f-strings) |
| `\}` | literal `}` (in f-strings) |

### Concatenation

```wsp
let full = "Hello" + ", " + "world!"
```

---

## Built-in Functions

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
| `args` | `() → array` | Script arguments |
| `write_hex` | `(path, hex) → none` | Decode hex string to bytes, write to file |
| `num_to_hex` | `(n) → string` | IEEE-754 f64 as 16-char hex string |

### Introspection and control

| Function | Signature | Description |
|----------|-----------|-------------|
| `type_of` | `(value) → string` | Runtime type: `"number"`, `"string"`, `"bool"`, `"array"`, `"dict"`, `"function"`, `"none"` |
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
| 2 | `[ ]` indexing, `( )` call (postfix) |
| 3 | unary `-`, `not` |
| 4 | `*`, `/`, `%` |
| 5 | `+`, `-` |
| 6 | `<`, `>`, `<=`, `>=`, `==`, `!=` |
| 7 | `and` |
| 8 (lowest) | `or` |

Chained calls and index accesses associate left-to-right: `f(1)(2)`, `a[0][1]`, `f(1)[0](2)`.

---

## Reserved Keywords

```
let  print  if  else  while  for  in  fn  return  break  continue
and  or  not  true  false  assert  type_of  exit
```

Built-in function names:
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
[line 3, col 0]  Error: Undefined variable: 'x'
[line 5, col 0]  Error: key "foo" not found in dict
[line 7, col 0]  Error: Array index 10 out of bounds (length: 5)
[line 9, col 0]  Error: Function 'add' expected 2 arguments, got 3
[line 12, col 0] Error: Division by zero
[line 15, col 0] Error: Type error: expected number, found string
[line 18, col 0] Error: Assertion failed: array must not be empty
```

---

**Whispem v5.0.0 — Complete Syntax Reference**