# Whispem Examples

**Version 3.0.0**

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
cargo run examples/hello.wsp
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
| `condition.wsp` | `if` / `else` |
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
| `fizzbuzz_proper.wsp` | FizzBuzz using modulo and `range` |

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

## Self-hosted compiler (v3.0.0)

```bash
# Standalone (no Rust needed)
./wvm compiler/wsc.whbc examples/hello.wsp

# Rust reference implementation
cargo run -- compiler/wsc.wsp examples/hello.wsp
```

`compiler/wsc.wsp` lives in the `compiler/` directory at the project root. It is a Whispem compiler written in Whispem — 1618 lines implementing the full compilation pipeline: lexer, recursive-descent parser, bytecode compiler, and binary serialiser. Produces `.whbc` files byte-for-byte identical to the Rust compiler’s output.

## Autonomous test suite

```bash
./tests/run_tests.sh
```

32 tests using only `wvm` + `wsc.whbc` — compiles each example, runs it, and compares output to expected baselines. Includes bootstrap verification. No Rust needed.

---

## More

- Full syntax reference → [`docs/syntax.md`](../docs/syntax.md)
- Step-by-step tutorial → [`docs/tutorial.md`](../docs/tutorial.md)
- All examples with code inline → [`docs/examples.md`](../docs/examples.md)
- VM specification and `.whbc` format → [`docs/vm.md`](../docs/vm.md)