# Whispem Examples

**Version 1.0.0**

A complete collection of example programs demonstrating all Whispem language features.

```bash
cargo run examples/<file>.wsp
```

---

## 📚 Examples by Category

### Basics

| File | Description |
|------|-------------|
| **hello.wsp** | Hello World - your first Whispem program |
| **variables.wsp** | Variable declaration with `let` |
| **arithmetic.wsp** | Arithmetic operators and precedence |
| **strings.wsp** | String literals, escapes, and concatenation |
| **boolean.wsp** | Boolean values `true` and `false` |
| **comments.wsp** | Comment syntax with `#` |

---

### Control Flow

| File | Description |
|------|-------------|
| **comparison.wsp** | Comparison operators: `<`, `>`, `<=`, `>=`, `==`, `!=` |
| **condition.wsp** | Conditional execution with `if/else` |
| **logical_operators.wsp** | Logical operators: `and`, `or`, `not` |

---

### Loops

| File | Description |
|------|-------------|
| **while_loop.wsp** | Basic `while` loop iteration |
| **for_loop.wsp** | `for` loop with arrays and `range()` |
| **countdown.wsp** | Countdown example using `while` |
| **break_continue.wsp** | Loop control with `break` and `continue` |

---

### Functions

| File | Description |
|------|-------------|
| **function_basic.wsp** | Function declaration and calling |
| **function_return.wsp** | Functions with return values |
| **function_recursive.wsp** | Recursive functions (factorial) |
| **function_no_params.wsp** | Functions without parameters |

---

### Arrays

| File | Description |
|------|-------------|
| **array_basic.wsp** | Array creation, indexing, and assignment |
| **array_iteration.wsp** | Iterating over arrays |
| **array_functions.wsp** | Built-in functions: `length()`, `push()` |
| **array_with_functions.wsp** | Passing arrays to functions |
| **array_mixed_types.wsp** | Mixed types and nested arrays |
| **array_build_dynamic.wsp** | Building arrays dynamically |
| **array_advanced.wsp** | Advanced: `pop()`, `reverse()`, `slice()`, `range()` |

---

### I/O

| File | Description |
|------|-------------|
| **user_input.wsp** | Reading user input with `input()` |
| **file_io.wsp** | File operations: `read_file()`, `write_file()` |

---

### Complete Programs

| File | Description |
|------|-------------|
| **fizzbuzz.wsp** | Classic FizzBuzz implementation |
| **prime_numbers.wsp** | Prime number generator |
| **data_processing.wsp** | Data filtering and aggregation |
| **task_manager.wsp** | Simple task manager application |
| **interactive_game.wsp** | Number guessing game with user input |

---

### Test Files

| File | Location | Description |
|------|----------|-------------|
| **test_control_flow.wsp** | `examples/` | Tests if/else, comparisons, booleans |
| **test_v1.0.0.wsp** | `tests/` | Comprehensive v1.0.0 test suite |

---

## 📖 Learning Path

Recommended order for beginners:

1. **hello.wsp** - Start here
2. **variables.wsp** - Variable basics
3. **arithmetic.wsp** - Expressions
4. **strings.wsp** - String handling
5. **boolean.wsp** - Boolean values
6. **comparison.wsp** - Comparisons
7. **condition.wsp** - If/else
8. **logical_operators.wsp** - and/or/not
9. **while_loop.wsp** - Basic loops
10. **for_loop.wsp** - For loops
11. **function_basic.wsp** - Functions intro
12. **function_return.wsp** - Return values
13. **function_recursive.wsp** - Recursion
14. **array_basic.wsp** - Arrays intro
15. **array_iteration.wsp** - Loop through arrays
16. **array_functions.wsp** - Built-in functions
17. **fizzbuzz.wsp** - Putting it all together

---

## 🎯 Quick Test

Run all examples:
```bash
for file in examples/*.wsp; do
    echo "=== $file ==="
    cargo run "$file"
done
```

---

## 💡 Tips

- Each example is self-contained and runnable
- Examples include comments explaining the code
- Functions must be defined before they are called
- Arrays use 0-based indexing
- `push()` returns a new array (original unchanged)
- Interactive examples (`user_input.wsp`, `interactive_game.wsp`) require terminal input

---

**Version:** 1.0.0  
**Examples:** 31 files  
**Status:** All language features covered
