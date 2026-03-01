# Whispem Examples

**Version 2.5.0**

A collection of example programs covering all Whispem language features.

```bash
# Run an example
cargo run examples/<file>.wsp

# Inspect compiled bytecode
cargo run -- --dump examples/<file>.wsp
```

---

## Basics

| File | Description |
|------|-------------|
| `hello.wsp` | Hello World — first Whispem program |
| `variables.wsp` | Variable declaration with `let` |
| `arithmetic.wsp` | Arithmetic operators and precedence |
| `strings.wsp` | String literals, escapes, and concatenation |
| `boolean.wsp` | Boolean values `true` and `false` |
| `comments.wsp` | Comment syntax with `#` |
| `modulo.wsp` | Modulo operator `%` and practical uses |

---

## Control Flow

| File | Description |
|------|-------------|
| `comparison.wsp` | Comparison operators |
| `condition.wsp` | `if / else` |
| `logical_operators.wsp` | `and`, `or`, `not` |
| `short_circuit.wsp` | Short-circuit evaluation of `and` / `or` |

---

## Loops

| File | Description |
|------|-------------|
| `while_loop.wsp` | Basic `while` loop |
| `for_loop.wsp` | `for` loop with arrays and `range()` |
| `countdown.wsp` | Countdown with `while` |
| `break_continue.wsp` | Loop control with `break` and `continue` |

---

## Functions

| File | Description |
|------|-------------|
| `function_basic.wsp` | Function declaration and calling |
| `function_return.wsp` | Return values |
| `function_recursive.wsp` | Recursion — factorial |
| `function_no_params.wsp` | Functions without parameters |

---

## Arrays

| File | Description |
|------|-------------|
| `array_basic.wsp` | Creation, indexing, assignment |
| `array_iteration.wsp` | Iterating over arrays |
| `array_functions.wsp` | `length()`, `push()` |
| `array_advanced.wsp` | `pop()`, `reverse()`, `slice()`, `range()` |
| `array_with_functions.wsp` | Passing arrays to functions |
| `array_mixed_types.wsp` | Mixed types and nested arrays |
| `array_build_dynamic.wsp` | Building arrays dynamically |

---

## Dictionaries

| File | Description |
|------|-------------|
| `dict_basic.wsp` | Creation, access, assignment, `has_key`, `keys`, `values` |
| `dict_nested.wsp` | Nested dicts and dict-aware functions |
| `dict_phonebook.wsp` | Practical example — phonebook |
| `dict_word_count.wsp` | Word frequency counter |

---

## I/O

| File | Description |
|------|-------------|
| `user_input.wsp` | Reading user input with `input()` |
| `file_io.wsp` | `read_file()` and `write_file()` |

---

## Complete Programs

| File | Description |
|------|-------------|
| `fizzbuzz.wsp` | FizzBuzz with `while` (v1.0 style) |
| `fizzbuzz_proper.wsp` | FizzBuzz with modulo — the right way |
| `prime_numbers.wsp` | Prime number generator |
| `data_processing.wsp` | Filtering and aggregation |
| `task_manager.wsp` | Simple task manager |
| `interactive_game.wsp` | Number guessing game |

---

## Learning Path

Recommended order for beginners:

1. `hello.wsp`
2. `variables.wsp`
3. `arithmetic.wsp`
4. `modulo.wsp`
5. `strings.wsp`
6. `boolean.wsp`
7. `comparison.wsp`
8. `condition.wsp`
9. `logical_operators.wsp`
10. `short_circuit.wsp`
11. `while_loop.wsp`
12. `for_loop.wsp`
13. `break_continue.wsp`
14. `function_basic.wsp`
15. `function_return.wsp`
16. `function_recursive.wsp`
17. `array_basic.wsp`
18. `array_functions.wsp`
19. `dict_basic.wsp`
20. `dict_word_count.wsp`
21. `fizzbuzz_proper.wsp`

---

## Notes

- All examples are self-contained and runnable
- Functions can be called before they are defined — forward calls work since v2.0.0
- Calling a function with the wrong number of arguments produces a clear runtime error
- Arrays use 0-based indexing
- `push()` returns a new array — the original is unchanged
- Dictionary keys are always strings internally
- `and`/`or` short-circuit correctly: the short-circuited value is the result of the expression
- Use `--dump` to inspect the bytecode of any example

---

**Whispem v2.5.0**