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

- `+` addition
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
2. Unary operators `-`, `not`, `!`
3. Multiplication and division `*`, `/`
4. Addition and subtraction `+`, `-`
5. Comparisons `<`, `>`, `<=`, `>=`, `==`, `!=`
6. Logical AND `and`
7. Logical OR `or`

---

## Program Structure

A Whispem program is a sequence of statements executed from top to bottom.

There is no explicit `main` function.

Example:

```wsp
# Initialize
let x = 10

# Check condition
if x > 5 {
    print "x is big"
}

# Loop
let i = 0
while i < 3 {
    print i
    let i = i + 1
}
```

---

## Reserved Keywords

The following words are reserved and cannot be used as variable names:

- `let`
- `print`
- `if`
- `else`
- `while`
- `and`
- `or`
- `not`
- `true`
- `false`
