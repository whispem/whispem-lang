# Whispem Syntax Reference

**Version 6.0.0**

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

| Type       | Examples                        | `type_of`   |
|------------|---------------------------------|-------------|
| `number`   | `42`, `3.14`, `-7`              | `"number"`  |
| `string`   | `"hello"`, `""`                 | `"string"`  |
| `bool`     | `true`, `false`                 | `"bool"`    |
| `array`    | `[1, "two", true]`              | `"array"`   |
| `dict`     | `{"key": "value"}`              | `"dict"`    |
| `function` | `fn(x) { return x }`, closures  | `"function"`|
| `none`     | returned by void functions      | `"none"`    |

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

### Access and assignment

```wsp
print person["name"]       # Em
person["city"] = "Paris"   # update or add key
```

Accessing a key that does not exist raises: `key "foo" not found in dict`.

---

## Expressions

### Arithmetic

```wsp
10 + 5    # 15
10 - 5    # 5
10 * 5    # 50
10 / 3    # 3.333...
10 % 3    # 1
```

### String concatenation

```wsp
"Hello" + ", " + "world!"
```

Numbers and booleans are coerced to string automatically when concatenated with a string.

### Chained calls

Calls chain left-to-right on any expression value:

```wsp
make_adder(5)(3)              # 8
fns[0](10)                    # calls closure at index 0
fn(x) { return x*2 }(7)      # 14 — immediate lambda call
outer(1)(2)(3)                # nested closure chain
```

---

## Comparisons

| Operator | Meaning          |
|----------|-----------------|
| `==`     | equal            |
| `!=`     | not equal        |
| `<`      | less than        |
| `<=`     | less than or equal |
| `>`      | greater than     |
| `>=`     | greater than or equal |

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

Both `else if` and `else` are optional. `else if` is native syntax.

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

Functions can be called before they are defined (forward calls work). Arity is checked at call time. A function with no explicit `return` returns `none`.

---

## Lambdas

`fn(params) { body }` is a first-class expression:

```wsp
let double = fn(x) { return x * 2 }
print double(7)   # 14

fn apply(f, x) { return f(x) }
print apply(fn(n) { return n * n }, 5)   # 25

let fns = [fn(x) { return x + 1 }, fn(x) { return x * 2 }]
print fns[0](10)   # 11

print fn(x) { return x * 2 }(7)   # 14 — immediate call
```

`type_of(fn(x){return x})` returns `"function"`.

---

## Closures

A function defined inside another function automatically captures variables from the enclosing scope:

```wsp
fn make_adder(n) {
    return fn(x) { return x + n }
}
let add5 = make_adder(5)
print add5(3)    # 8
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

Two closures in the same scope share the same cell:

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

Closures nest to arbitrary depth:

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

`f"..."` strings with `{expr}` interpolation. Any expression is valid inside braces:

```wsp
let name  = "Em"
let score = 42
print f"Hello, {name}!"
print f"Score: {score}, doubled: {score * 2}"
print f"{length([1, 2, 3])} items"
```

F-strings compile to `+` concatenation chains — identical performance to hand-written concatenation.

---

## Strings

### Escape sequences

| Sequence | Character   |
|----------|-------------|
| `\n`     | newline     |
| `\t`     | tab         |
| `\r`     | carriage return |
| `\\`     | backslash   |
| `\"`     | double quote |
| `\{`     | literal `{` (in f-strings) |
| `\}`     | literal `}` (in f-strings) |

---

## Built-in Functions

### Arrays

| Function  | Signature                       | Description                    |
|-----------|---------------------------------|--------------------------------|
| `length`  | `(array\|string\|dict) → number`| Number of elements             |
| `push`    | `(array, value) → array`        | New array with value appended  |
| `pop`     | `(array) → value`               | Last element (error if empty)  |
| `reverse` | `(array) → array`               | New reversed array             |
| `slice`   | `(array, start, end) → array`   | Sub-array `[start, end)`       |
| `range`   | `(start, end) → array`          | Integer range `[start, end)`   |

### Higher-order (v6.0.0)

| Function | Signature                        | Description                                      |
|----------|----------------------------------|--------------------------------------------------|
| `map`    | `(array, f) → array`             | `[f(x) for x in array]`                          |
| `filter` | `(array, pred) → array`          | `[x for x in array if pred(x)]`                  |
| `reduce` | `(array, f, initial) → value`    | `f(…f(f(initial, a[0]), a[1])…, a[n-1])`         |

```wsp
print map([1, 2, 3], fn(x) { return x * 2 })              # [2, 4, 6]
print filter([1,2,3,4,5], fn(n) { return n % 2 == 0 })    # [2, 4]
print reduce([1,2,3,4,5], fn(acc,n) { return acc+n }, 0)  # 15
```

### Dictionaries

| Function  | Signature                    | Description                |
|-----------|------------------------------|----------------------------|
| `keys`    | `(dict) → array`             | Sorted list of keys        |
| `values`  | `(dict) → array`             | Values in key-sorted order |
| `has_key` | `(dict, key) → bool`         | Check if key exists        |

### Strings

| Function     | Signature                        | Description                          |
|--------------|----------------------------------|--------------------------------------|
| `length`     | `(string) → number`              | Character count (UTF-8 aware)        |
| `char_at`    | `(string, index) → string`       | Single character at index            |
| `substr`     | `(string, start, len) → string`  | Substring                            |
| `ord`        | `(string) → number`              | Unicode codepoint of first character |
| `num_to_str` | `(number) → string`              | Number to string                     |
| `str_to_num` | `(string) → number`              | String to number                     |

### I/O

| Function     | Signature                    | Description                             |
|--------------|------------------------------|-----------------------------------------|
| `input`      | `(prompt?) → string`         | Read line from stdin                    |
| `read_file`  | `(path) → string`            | Read file contents                      |
| `write_file` | `(path, content) → none`     | Write string to file                    |
| `args`       | `() → array`                 | Script arguments                        |
| `write_hex`  | `(path, hex) → none`         | Decode hex string to bytes, write file  |
| `num_to_hex` | `(n) → string`               | IEEE-754 f64 as 16-char hex string      |

### Introspection and control

| Function  | Signature                 | Description                                                    |
|-----------|---------------------------|----------------------------------------------------------------|
| `type_of` | `(value) → string`        | `"number"`, `"string"`, `"bool"`, `"array"`, `"dict"`, `"function"`, `"none"` |
| `assert`  | `(condition, message?) → none` | Raises `AssertionFailed` if condition is falsy            |
| `exit`    | `(code?) → none`          | Terminates with exit code (default `0`)                        |

---

## Comments

```wsp
# This is a comment
let x = 10   # inline comment
```

---

## Operator Precedence

From highest to lowest:

| Level       | Operators                                    |
|-------------|----------------------------------------------|
| 1 (highest) | `( )` parentheses                            |
| 2           | `[ ]` indexing, `( )` call (postfix)         |
| 3           | unary `-`, `not`                             |
| 4           | `*`, `/`, `%`                                |
| 5           | `+`, `-`                                     |
| 6           | `<`, `>`, `<=`, `>=`, `==`, `!=`             |
| 7           | `and`                                        |
| 8 (lowest)  | `or`                                         |

Chained calls and index accesses associate left-to-right: `f(1)(2)`, `a[0][1]`, `f(1)[0](2)`, `outer(1)(2)(3)`.

---

## Reserved Keywords

```
let  print  if  else  while  for  in  fn  return  break  continue
and  or  not  true  false  assert  type_of  exit
```

Built-in function names (also reserved):

```
length  push  pop  reverse  slice  range  map  filter  reduce
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

**Whispem v6.0.0 — Complete Syntax Reference**