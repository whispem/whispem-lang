# Whispem Examples

**Version 6.0.0**

Example programs covering all Whispem language features.

Run any example:
```bash
cargo run -- examples/<file>.wsp
cargo run -- --compile examples/<file>.wsp
cargo run -- --dump examples/<file>.wsp
```

---

## Hello World

```wsp
let message = "Hello, Whispem!"
print message
```

---

## F-strings

```wsp
let name  = "Em"
let score = 42
print f"Hello, {name}!"
print f"Score: {score}, doubled: {score * 2}"
print f"{length([1, 2, 3])} items in the list"
```

---

## FizzBuzz — with `else if`

```wsp
for n in range(1, 101) {
    if n % 15 == 0 { print "FizzBuzz" }
    else if n % 3 == 0 { print "Fizz" }
    else if n % 5 == 0 { print "Buzz" }
    else { print n }
}
```

---

## Lambdas

```wsp
let double = fn(x) { return x * 2 }
print double(7)   # 14

fn apply(f, x) { return f(x) }
print apply(fn(n) { return n * n }, 5)   # 25

let fns = [fn(x) { return x + 1 }, fn(x) { return x * 2 }]
print fns[0](10)   # 11
print fns[1](10)   # 20

print fn(x) { return x * 2 }(7)   # 14
```

---

## Closures

### Adder factory

```wsp
fn make_adder(n) {
    return fn(x) { return x + n }
}
let add5  = make_adder(5)
let add10 = make_adder(10)
print add5(3)    # 8
print add10(3)   # 13
```

### Counter with mutable state

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

### Two closures sharing state

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

### Nested closures

```wsp
fn outer(a) {
    return fn(b) {
        return fn(c) { return a + b + c }
    }
}
print outer(1)(2)(3)   # 6
```

---

## map, filter, reduce (v6.0.0)

### map

```wsp
print map([1, 2, 3, 4], fn(x) { return x * 2 })
# [2, 4, 6, 8]

fn make_multiplier(n) { return fn(x) { return x * n } }
print map([1, 2, 3], make_multiplier(3))
# [3, 6, 9]
```

### filter

```wsp
print filter([1, 2, 3, 4, 5, 6], fn(n) { return n % 2 == 0 })
# [2, 4, 6]

fn make_gt(t) { return fn(n) { return n > t } }
print filter([1, 5, 3, 8, 2, 7], make_gt(4))
# [5, 8, 7]
```

### reduce

```wsp
print reduce([1, 2, 3, 4, 5], fn(acc, n) { return acc + n }, 0)
# 15

print reduce([1, 2, 3, 4], fn(acc, n) { return acc * n }, 1)
# 24

print reduce(["b","c","d"], fn(acc, s) { return acc + s }, "a")
# abcd
```

### Pipeline

```wsp
# Sum of squares of even numbers from 1 to 10
let total = reduce(
    map(
        filter(range(1, 11), fn(n) { return n % 2 == 0 }),
        fn(n) { return n * n }
    ),
    fn(acc, n) { return acc + n },
    0)
print total   # 220
```

---

## Arrays

```wsp
let numbers = [1, 2, 3, 4, 5]
print numbers[0]
numbers[2] = 10
print numbers

print push([1,2,3], 4)
print pop([1,2,3,4])
print reverse([1,2,3])
print slice([1,2,3,4,5], 1, 4)
print range(0, 5)
```

---

## Dictionaries

```wsp
let person = {"name": "Em", "age": 26, "city": "Marseille"}
print person["name"]
person["city"] = "Paris"
print has_key(person, "name")
print keys(person)
print length(person)
```

---

## `type_of`

```wsp
print type_of(42)                     # number
print type_of("hello")                # string
print type_of(true)                   # bool
print type_of([1, 2, 3])              # array
print type_of({"a": 1})               # dict
print type_of(fn(x) { return x })     # function
```

---

## `assert`

```wsp
fn process(items) {
    assert(type_of(items) == "array", "expected array")
    assert(length(items) > 0, "items must not be empty")
}
```

---

## Recursion

```wsp
fn factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
print factorial(10)   # 3628800

fn fib(n) {
    if n <= 1 { return n }
    return fib(n-1) + fib(n-2)
}
print fib(10)   # 55
```

---

## Data Processing

```wsp
fn filter_positive(numbers) {
    return filter(numbers, fn(n) { return n > 0 })
}

fn sum_array(arr) {
    return reduce(arr, fn(acc, n) { return acc + n }, 0)
}

let data     = [-5, 3, -2, 8, 0, 12, -1, 7]
let positive = filter_positive(data)
print positive            # [3, 8, 12, 7]
print sum_array(positive) # 30
```

---

## File I/O

```wsp
write_file("hello.txt", "Hello from Whispem 6.0!")
let content = read_file("hello.txt")
print content
```

---

## Notes

- Examples are self-contained and runnable.
- Functions can be called before they are defined.
- `push()` returns a new array — the original is unchanged.
- `and`/`or` short-circuit correctly.
- `else if` is supported natively.
- Lambdas are `fn(params) { body }` expressions.
- Closures capture variables by shared mutable reference.
- F-strings `f"..."` support `{expr}` interpolation.
- `map`, `filter`, `reduce` accept any callable: named functions, lambdas, or closures.
- Use `--dump` to inspect bytecode.

---

**Whispem v6.0.0**