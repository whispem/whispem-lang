# Whispem Examples

This document presents example programs written in Whispem.

Each example is meant to illustrate a specific language feature.
All examples can be executed using the Whispem interpreter.

Run an example with:

```bash
cargo run examples/<file>.wsp
```

---

## Basic variable

```wsp
let x = 10
print x
```

Output:

```text
10
```

---

## Arithmetic expressions

```wsp
let a = 5
let b = a + 3 * 2
print b
```

Output:

```text
11
```

---

## Strings

```wsp
let name = "Whispem"
print name
```

Output:

```text
Whispem
```

---

## Conditional execution

```wsp
let temperature = 18

if temperature > 20 {
    print "Warm"
} else {
    print "Cool"
}
```

Output:

```text
Cool
```

---

## Boolean logic

```wsp
let x = 10

if x == 10 {
    print true
} else {
    print false
}
```

Output:

```text
true
```

---

## Comments

```wsp
# This is a comment
let x = 42
print x
```

Output:

```text
42
```

---

## Notes

- Examples are intentionally small and readable
- There is no hidden control flow
- Execution always happens top-to-bottom
- Errors are currently minimal and will be improved in future versions
