# Whispem Examples

**Version 4.0.0**

A collection of Whispem programs demonstrating every language feature.

---

## Running examples

```bash
# Standalone (no Rust needed)
make                                         # build wvm (once)
./wvm compiler/wsc.whbc examples/hello.wsp   # compile + run
./wvm examples/hello.whbc                    # run precompiled
./wvm --dump examples/hello.whbc             # inspect bytecode

# Rust reference implementation
cargo run -- examples/hello.wsp
cargo run -- --compile examples/hello.wsp
cargo run -- --dump examples/hello.wsp
```

---

## File index

### Basics

| File | What it shows |
|------|---------------|
| `hello.wsp` | Hello world, `let`, `print` |
| `variables.wsp` | Variable declarations and types |
| `arithmetic.wsp` | `+`, `-`, `*`, `/`, `%` |
| `modulo.wsp` | Modulo operator with a loop |
| `strings.wsp` | String literals, escape sequences, concatenation |

### Control flow

| File | What it shows |
|------|---------------|
| `condition.wsp` | `if` / `else if` / `else` |
| `while_loop.wsp` | `while` loop |
| `for_loop.wsp` | `for … in`, `range()` |
| `break_continue.wsp` | `break`, `continue` |
| `short_circuit.wsp` | `and`/`or` short-circuit evaluation |

### Functions

| File | What it shows |
|------|---------------|
| `function_basic.wsp` | Defining and calling functions |
| `function_recursive.wsp` | Recursion — factorial |
| `prime_numbers.wsp` | Recursion + loops |
| `fizzbuzz_proper.wsp` | FizzBuzz using `else if`, modulo, and `range` |

### Arrays

| File | What it shows |
|------|---------------|
| `array_basic.wsp` | Literals, indexing, index assignment |
| `array_functions.wsp` | `push`, `pop`, `reverse`, `slice`, `range`, `length` |

### Dictionaries

| File | What it shows |
|------|---------------|
| `dict_basic.wsp` | Literals, access, update, `has_key`, `keys`, `length` |
| `dict_phonebook.wsp` | Dictionary as a data structure |
| `dict_word_count.wsp` | Building a frequency table |

### I/O

| File | What it shows |
|------|---------------|
| `user_input.wsp` | `input()` |
| `file_io.wsp` | `read_file()`, `write_file()` |

### Advanced

| File | What it shows |
|------|---------------|
| `data_processing.wsp` | Filter, sum, higher-order patterns with arrays |

---

## v4.0.0 features

### `else if`

`else if` is now native syntax — no more nesting `if` inside `else`:

```wsp
if score >= 90 { print "A" }
else if score >= 80 { print "B" }
else if score >= 70 { print "C" }
else { print "F" }
```

`examples/fizzbuzz_proper.wsp` has been updated to use `else if`.

### `assert(condition, message?)`

```wsp
assert(length(items) > 0, "items must not be empty")
assert(type_of(x) == "number")
```

### `type_of(value)`

Returns `"number"`, `"string"`, `"bool"`, `"array"`, `"dict"`, or `"none"`:

```wsp
fn safe_double(x) {
    if type_of(x) != "number" { return "error" }
    return x * 2
}
```

### `exit(code?)`

```wsp
if length(args()) == 0 {
    print "Usage: script.wsp <n>"
    exit(1)
}
```

---

## Self-hosted compiler (v3.0.0+)

```bash
./wvm compiler/wsc.whbc examples/hello.wsp
```

`compiler/wsc.wsp` is a Whispem compiler written in Whispem — 1724 lines implementing the full compilation pipeline: lexer, recursive-descent parser, bytecode compiler, and binary serialiser. Updated in v4.0.0 to support `else if`, `assert`, `type_of`, and `exit`.

## Autonomous test suite

```bash
./tests/run_tests.sh
```

Compiles each example via `wsc.whbc`, runs it, and compares output to expected baselines. No Rust needed.

---

## More

- Full syntax reference → [`docs/syntax.md`](../docs/syntax.md)
- Step-by-step tutorial → [`docs/tutorial.md`](../docs/tutorial.md)
- All examples with code inline → [`docs/examples.md`](../docs/examples.md)
- VM specification and `.whbc` format → [`docs/vm.md`](../docs/vm.md)