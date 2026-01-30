# Whispem Examples

This directory contains example Whispem programs.

All examples can be run using:

```bash
cargo run examples/<file>.wsp
```

---

## hello.wsp

```wsp
let x = 10
print x
```

Output:

```text
10
```

---

## condition.wsp

```wsp
let x = 5

if x > 3 {
    print x
} else {
    print 0
}
```

Output:

```text
5
```
