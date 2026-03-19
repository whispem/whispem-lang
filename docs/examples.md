# Whispem Examples

**Version 5.0.0**

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
**File:** `examples/hello.wsp`

---

## Variables

```wsp
let x = 10
let y = 20
let name = "Whispem"
print x
print y
print name
```

---

## Arithmetic

```wsp
let a = 10
let b = 3
print a + b   # 13
print a - b   # 7
print a * b   # 30
print a / b   # 3.333...
print a % b   # 1
```

---

## F-strings (v5.0.0)

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
**File:** `examples/fizzbuzz_proper.wsp`

---

## Conditionals with `else if`

```wsp
let score = 85

if score >= 90 { print "A" }
else if score >= 80 { print "B" }
else if score >= 70 { print "C" }
else { print "F" }
```

---

## While Loops

```wsp
let counter = 0
while counter < 5 {
    print counter
    let counter = counter + 1
}
```

---

## For Loops

```wsp
for num in [1, 2, 3, 4, 5] { print num }
for i in range(0, 10) { print i }
```

---

## Break and Continue

```wsp
for num in range(1, 20) {
    if num > 10 { break }
    if num % 2 == 0 { continue }
    print num
}
```

---

## Functions

```wsp
fn greet(name) {
    return "Hello, " + name + "!"
}
print greet("World")
```

---

## Recursion

```wsp
fn factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
print factorial(5)    # 120
print factorial(10)   # 3628800
```

---

## Lambdas (v5.0.0)

```wsp
# Store in a variable
let double = fn(x) { return x * 2 }
print double(7)   # 14

# Pass as argument
fn apply(f, x) { return f(x) }
print apply(fn(n) { return n * n }, 5)   # 25

# Store in an array
let ops = [fn(x) { return x + 1 }, fn(x) { return x * 2 }]
print ops[0](10)   # 11
print ops[1](10)   # 20

# Immediate call
print fn(x) { return x * 2 }(7)   # 14
```

---

## Closures (v5.0.0)

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

## Higher-order functions

```wsp
fn map_array(arr, f) {
    let result = []
    for item in arr {
        let result = push(result, f(item))
    }
    return result
}

fn filter_array(arr, pred) {
    let result = []
    for item in arr {
        if pred(item) {
            let result = push(result, item)
        }
    }
    return result
}

let nums    = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
let evens   = filter_array(nums, fn(n) { return n % 2 == 0 })
let doubled = map_array(evens, fn(n) { return n * 2 })
print doubled   # [4, 8, 12, 16, 20]
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

## `type_of` (v4.0.0+)

```wsp
print type_of(42)                 # number
print type_of("hello")            # string
print type_of(true)               # bool
print type_of([1, 2, 3])          # array
print type_of({"a": 1})           # dict
print type_of(fn(x){return x})    # function
```

---

## `assert` (v4.0.0+)

```wsp
fn process(items) {
    assert(type_of(items) == "array", "expected array")
    assert(length(items) > 0, "items must not be empty")
    for n in items {
        assert(type_of(n) == "number", "all items must be numbers")
    }
}
print process([3, 8, 12, 7])
```

---

## `exit` (v4.0.0+)

```wsp
let script_args = args()
if length(script_args) == 0 {
    print "Usage: script.wsp <name>"
    exit(1)
}
print f"Hello, {script_args[0]}!"
exit(0)
```

---

## Data Processing

```wsp
fn filter_positive(numbers) {
    let result = []
    for num in numbers {
        if num > 0 { let result = push(result, num) }
    }
    return result
}

fn sum_array(arr) {
    let total = 0
    for num in arr { let total = total + num }
    return total
}

let data     = [-5, 3, -2, 8, 0, 12, -1, 7]
let positive = filter_positive(data)
print positive
print sum_array(positive)
```

---

## Prime Numbers

```wsp
fn is_prime(n) {
    if n < 2 { return false }
    if n == 2 { return true }
    for i in range(2, n) {
        if n % i == 0 { return false }
    }
    return true
}

print "Primes up to 30:"
for num in range(2, 31) {
    if is_prime(num) { print num }
}
```

---

## File I/O

```wsp
write_file("hello.txt", "Hello from Whispem 5.0!")
let content = read_file("hello.txt")
print content
```

---

## Notes

- Examples are self-contained and runnable.
- Functions can be called before they are defined.
- Calling a function with the wrong number of arguments produces a clear runtime error.
- `push()` returns a new array — the original is unchanged.
- `and`/`or` short-circuit correctly.
- `else if` is supported natively.
- Lambdas are `fn(params) { body }` expressions.
- Closures capture variables by shared mutable reference.
- F-strings `f"..."` support `{expr}` interpolation.
- Use `--dump` to inspect bytecode.

---

**Whispem v5.0.0**