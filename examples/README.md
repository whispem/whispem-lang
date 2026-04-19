# Whispem Examples

**Version 6.0.0**

A collection of Whispem programs demonstrating every language feature.

---

## Running examples

```bash
cargo run -- examples/<file>.wsp
cargo run -- --compile examples/<file>.wsp   # → <file>.whbc
cargo run -- --dump examples/<file>.wsp       # disassemble
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
| `comments.wsp` | Comment syntax |
| `boolean.wsp` | Boolean values |
| `comparison.wsp` | Comparison operators |

### Control flow

| File | What it shows |
|------|---------------|
| `condition.wsp` | `if` / `else if` / `else` |
| `while_loop.wsp` | `while` loop |
| `countdown.wsp` | While loop counting down |
| `for_loop.wsp` | `for … in`, `range()` |
| `break_continue.wsp` | `break`, `continue` |
| `logical_operators.wsp` | `and`, `or`, `not` |
| `short_circuit.wsp` | Short-circuit evaluation |

### Functions

| File | What it shows |
|------|---------------|
| `function_basic.wsp` | Defining and calling functions |
| `function_return.wsp` | Return values |
| `function_no_params.wsp` | Zero-parameter functions |
| `function_recursive.wsp` | Recursion — factorial |
| `prime_numbers.wsp` | Recursion + loops |
| `fizzbuzz_proper.wsp` | FizzBuzz with `else if` and `range` |

### Lambdas and closures

| File | What it shows |
|------|---------------|
| `lambda_basic.wsp` | Lambdas stored in variables, passed as arguments |
| `closure_adder.wsp` | Closure factory — `make_adder` |
| `closure_counter.wsp` | Mutable shared state across calls |
| `closure_pair.wsp` | Two closures sharing the same upvalue cell |
| `higher_order.wsp` | `map_array`, `filter_array` using lambdas |

### Higher-order builtins (v6.0.0)

| File | What it shows |
|------|---------------|
| `higher_order_v6.wsp` | `map`, `filter`, `reduce`, closures as args, pipeline |

### F-strings

| File | What it shows |
|------|---------------|
| `fstrings.wsp` | Basic interpolation, expressions in holes |

### Arrays

| File | What it shows |
|------|---------------|
| `array_basic.wsp` | Literals, indexing, index assignment |
| `array_functions.wsp` | `push`, `pop`, `reverse`, `slice`, `range`, `length` |
| `array_advanced.wsp` | Combining operations |
| `array_iteration.wsp` | While-loop iteration over array by index |
| `array_mixed_types.wsp` | Mixed types, nested arrays |
| `array_build_dynamic.wsp` | Building arrays dynamically |
| `array_with_functions.wsp` | `sum_array`, `find_max` |

### Dictionaries

| File | What it shows |
|------|---------------|
| `dict_basic.wsp` | Literals, access, update, `has_key`, `keys`, `length` |
| `dict_nested.wsp` | Dict as a record type, nested data |
| `dict_phonebook.wsp` | Dictionary as a data structure |
| `dict_word_count.wsp` | Building a frequency table |

### I/O

| File | What it shows |
|------|---------------|
| `user_input.wsp` | `input()` |
| `file_io.wsp` | `read_file()`, `write_file()` |
| `interactive_game.wsp` | Number guessing game |

### Advanced

| File | What it shows |
|------|---------------|
| `data_processing.wsp` | Filter, sum, max |
| `task_manager.wsp` | Simple task manager with arrays |

---

## v6.0.0 features

### map, filter, reduce

```wsp
# map(array, f) → [f(x) for x in array]
print map([1, 2, 3, 4], fn(x) { return x * 2 })
# [2, 4, 6, 8]

# filter(array, pred) → elements where pred(x) is truthy
print filter([1,2,3,4,5,6], fn(n) { return n % 2 == 0 })
# [2, 4, 6]

# reduce(array, f, initial) → left fold
print reduce([1,2,3,4,5], fn(acc,n) { return acc + n }, 0)
# 15
```

They accept any callable — named functions, lambdas, or closures:

```wsp
fn make_gt(t) { return fn(n) { return n > t } }
print filter([1, 5, 3, 8, 2, 7], make_gt(4))
# [5, 8, 7]
```

And compose naturally:

```wsp
let total = reduce(
    map(filter(range(1, 11), fn(n) { return n % 2 == 0 }),
        fn(n) { return n * n }),
    fn(acc, n) { return acc + n },
    0)
print total   # 220
```

---

## v5.0.0 features

### Lambdas

```wsp
let double = fn(x) { return x * 2 }
print double(7)   # 14

fn apply(f, x) { return f(x) }
print apply(fn(n) { return n * n }, 5)   # 25

print fn(x) { return x + 1 }(10)   # 11
```

### Closures

```wsp
fn make_adder(n) {
    return fn(x) { return x + n }
}
print make_adder(5)(3)   # 8
```

Captured variables are shared and mutable:

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

### F-strings

```wsp
let name = "Em"
let x = 42
print f"Hello, {name}! Value: {x * 2}"
```

---

## v4.0.0 features

### `else if`

```wsp
if score >= 90 { print "A" }
else if score >= 80 { print "B" }
else if score >= 70 { print "C" }
else { print "F" }
```

### `assert(condition, message?)`

```wsp
assert(length(items) > 0, "items must not be empty")
assert(type_of(x) == "number")
```

### `type_of(value)`

Returns `"number"`, `"string"`, `"bool"`, `"array"`, `"dict"`, `"function"`, or `"none"`.

### `exit(code?)`

```wsp
if length(args()) == 0 {
    print "Usage: script.wsp <n>"
    exit(1)
}
```

---

## More

- Full syntax reference → [`docs/syntax.md`](../docs/syntax.md)
- Step-by-step tutorial → [`docs/tutorial.md`](../docs/tutorial.md)
- All examples with code inline → [`docs/examples.md`](../docs/examples.md)
- VM specification and `.whbc` format → [`docs/vm.md`](../docs/vm.md)
- Em's journey → [`docs/journey.md`](../docs/journey.md)