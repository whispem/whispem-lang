# Whispem Examples

**Version 2.0.0**

A curated collection of example programs covering all Whispem language features.

Run any example with:
```bash
cargo run examples/<file>.wsp
```

Inspect compiled bytecode with:
```bash
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
**File:** `examples/variables.wsp`

---

## Arithmetic
```wsp
let a = 10
let b = 3

print a + b   # 13
print a - b   # 7
print a * b   # 30
print a / b   # 3.333...
print a % b   # 1  ← modulo
```
**File:** `examples/arithmetic.wsp`

---

## Modulo
```wsp
# Even numbers between 1 and 10
for n in range(1, 11) {
    if n % 2 == 0 {
        print n
    }
}
```
**File:** `examples/modulo.wsp`

---

## FizzBuzz (with modulo)
```wsp
for n in range(1, 101) {
    if n % 15 == 0 {
        print "FizzBuzz"
    } else {
        if n % 3 == 0 { print "Fizz" }
        else {
            if n % 5 == 0 { print "Buzz" }
            else { print n }
        }
    }
}
```
**File:** `examples/fizzbuzz_proper.wsp`

---

## Strings
```wsp
let greeting = "Hello"
let name = "World"

let multiline = "Line one\nLine two"
print multiline

let quoted = "She said \"Hello\""
print quoted

let full = greeting + ", " + name + "!"
print full
```
**File:** `examples/strings.wsp`

---

## Conditionals
```wsp
let temperature = 18

if temperature > 20 {
    print "It's warm"
} else {
    print "It's cool"
}
```
**File:** `examples/condition.wsp`

---

## While Loops
```wsp
let counter = 0

while counter < 5 {
    print counter
    let counter = counter + 1
}
```
**File:** `examples/while_loop.wsp`

---

## For Loops
```wsp
for num in [1, 2, 3, 4, 5] {
    print num
}

for i in range(0, 10) {
    print i
}
```
**File:** `examples/for_loop.wsp`

---

## Break and Continue
```wsp
for num in range(1, 20) {
    if num > 10 { break }
    if num % 2 == 0 { continue }
    print num
}
```
**File:** `examples/break_continue.wsp`

---

## Functions
```wsp
fn greet(name) {
    print "Hello, " + name + "!"
}

greet("World")
greet("Whispem")
```
**File:** `examples/function_basic.wsp`

---

## Recursion
```wsp
fn factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

print factorial(5)   # 120
print factorial(10)  # 3628800
```
**File:** `examples/function_recursive.wsp`

---

## Arrays — Basic
```wsp
let numbers = [1, 2, 3, 4, 5]
print numbers

print numbers[0]   # 1
numbers[2] = 10
print numbers      # [1, 2, 10, 4, 5]
```
**File:** `examples/array_basic.wsp`

---

## Arrays — Built-in Functions
```wsp
let items = [1, 2, 3]

print length(items)                      # 3
print push(items, 4)                     # [1, 2, 3, 4]
print pop([1, 2, 3, 4])                  # 4
print reverse([1, 2, 3])                 # [3, 2, 1]
print slice([1, 2, 3, 4, 5], 1, 4)      # [2, 3, 4]
print range(0, 5)                        # [0, 1, 2, 3, 4]
```
**File:** `examples/array_functions.wsp`

---

## Dictionaries — Basic
```wsp
let person = {"name": "Em", "age": 26, "city": "Marseille"}

print person["name"]           # Em
person["city"] = "Paris"
print person["city"]           # Paris

print has_key(person, "name")  # true
print has_key(person, "phone") # false
print keys(person)
print length(person)           # 3
```
**File:** `examples/dict_basic.wsp`

---

## Dictionaries — Phonebook
```wsp
fn add_contact(book, name, phone) {
    book[name] = phone
    return book
}

fn lookup(book, name) {
    if has_key(book, name) {
        return book[name]
    }
    return "Contact not found"
}

let phonebook = {}
let phonebook = add_contact(phonebook, "Alice", "06 12 34 56 78")
let phonebook = add_contact(phonebook, "Bob",   "07 98 76 54 32")

print lookup(phonebook, "Alice")
print lookup(phonebook, "Charlie")
```
**File:** `examples/dict_phonebook.wsp`

---

## Dictionaries — Word Counter
```wsp
fn count_words(words) {
    let counts = {}
    for word in words {
        if has_key(counts, word) {
            counts[word] = counts[word] + 1
        } else {
            counts[word] = 1
        }
    }
    return counts
}

let words  = ["rust", "whispem", "rust", "language", "whispem", "rust"]
let counts = count_words(words)

for word in keys(counts) {
    print word + ": " + counts[word]
}
```
**File:** `examples/dict_word_count.wsp`

---

## File I/O
```wsp
write_file("hello.txt", "Hello from Whispem 2.0!")

let content = read_file("hello.txt")
print content
```
**File:** `examples/file_io.wsp`

---

## User Input
```wsp
let name = input("What's your name? ")
print "Hello, " + name + "!"
```
**File:** `examples/user_input.wsp`

---

## Data Processing
```wsp
fn filter_positive(numbers) {
    let result = []
    for num in numbers {
        if num > 0 {
            let result = push(result, num)
        }
    }
    return result
}

fn sum_array(arr) {
    let total = 0
    for num in arr {
        let total = total + num
    }
    return total
}

let data     = [-5, 3, -2, 8, 0, 12, -1, 7]
let positive = filter_positive(data)

print "Positive:"
print positive
print "Sum:"
print sum_array(positive)
```
**File:** `examples/data_processing.wsp`

---

## Prime Numbers
```wsp
fn is_prime(n) {
    if n < 2 { return false }
    if n == 2 { return true }

    for i in range(2, n) {
        let quotient = n / i
        let product  = i * quotient
        if product == n { return false }
    }
    return true
}

print "Primes up to 30:"
for num in range(2, 31) {
    if is_prime(num) {
        print num
    }
}
```
**File:** `examples/prime_numbers.wsp`

---

## Notes

- Examples are self-contained and runnable
- Functions can be called before they are defined (forward calls work since v2.0.0)
- Arrays use 0-based indexing
- `push()` returns a new array — the original is unchanged
- Dictionary keys are always strings internally
- Use `--dump` to inspect the bytecode of any example

---

**Whispem v2.0.0**