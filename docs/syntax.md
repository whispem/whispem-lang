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
```

---

## Types

Whispem currently supports:

- Numbers (floating point)
- Strings
- Booleans (`true`, `false`)

Types are inferred automatically.

---

## Expressions

Whispem supports arithmetic expressions with operator precedence.

```wsp
let x = 10 + 5 * 2
let y = (10 + 5) * 2
```

Supported operators:

- `+` addition
- `-` subtraction
- `*` multiplication
- `/` division

---

## Comparisons

Expressions can be compared using:

- `<`
- `>`
- `<=`
- `>=`
- `==`
- `!=`

```wsp
if x > 5 {
    print x
}
```

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

Blocks are delimited with `{ }`.

---

## Print

The `print` keyword outputs a value to standard output.

```wsp
print x
print "Hello"
```

---

## Comments

Comments start with `#` and continue until the end of the line.

```wsp
# This is a comment
let x = 10
```

---

## Program structure

A Whispem program is a sequence of statements executed from top to bottom.

There is no explicit `main` function.
