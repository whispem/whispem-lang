# Whispem Examples

This document presents example programs written in Whispem.

Each example is meant to illustrate a specific language feature.
All examples can be executed using the Whispem interpreter.

Run an example with:
```bash
cargo run examples/<file>.wsp
```

---

## Hello World
```wsp
let message = "Hello, Whispem!"
print message
```

**File:** `examples/hello.wsp`

Output:
```text
Hello, Whispem!
```

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

Output:
```text
10
20
Whispem
```

---

## Arithmetic Expressions
```wsp
let a = 10
let b = 5

let sum = a + b
let diff = a - b
let prod = a * b
let quot = a / b

print sum
print diff
print prod
print quot

# Operator precedence
let result = 10 + 5 * 2
print result

# Parentheses
let result2 = (10 + 5) * 2
print result2
```

**File:** `examples/arithmetic.wsp`

Output:
```text
15
5
50
2
20
30
```

---

## Strings
```wsp
let greeting = "Hello"
let name = "World"
let message = "Whispem is minimalist"

print greeting
print name
print message

# Strings with escape sequences
let multiline = "Line one\nLine two"
print multiline

let quoted = "She said \"Hello\""
print quoted

# String concatenation
let full_greeting = greeting + ", " + name + "!"
print full_greeting
```

**File:** `examples/strings.wsp`

Output:
```text
Hello
World
Whispem is minimalist
Line one
Line two
She said "Hello"
Hello, World!
```

---

## Booleans
```wsp
let is_true = true
let is_false = false

print is_true
print is_false

let valid = true
print valid
```

**File:** `examples/boolean.wsp`

Output:
```text
true
false
true
```

---

## Comparisons
```wsp
let x = 10
let y = 20

# Less than
if x < y {
    print "10 is less than 20"
}

# Greater than
if y > x {
    print "20 is greater than 10"
}

# Equal
let a = 5
if a == 5 {
    print "a equals 5"
}

# Not equal
if x != y {
    print "x and y are different"
}
```

**File:** `examples/comparison.wsp`

Output:
```text
10 is less than 20
20 is greater than 10
a equals 5
x and y are different
```

---

## Conditional Execution
```wsp
let temperature = 18

if temperature > 20 {
    print "It's warm"
} else {
    print "It's cool"
}

# Nested conditions
let score = 85

if score >= 90 {
    print "Grade: A"
} else {
    if score >= 80 {
        print "Grade: B"
    } else {
        print "Grade: C"
    }
}
```

**File:** `examples/condition.wsp`

Output:
```text
It's cool
Grade: B
```

---

## While Loops
```wsp
let counter = 0

while counter < 5 {
    print counter
    let counter = counter + 1
}

print "Done!"
```

**File:** `examples/while_loop.wsp`

Output:
```text
0
1
2
3
4
Done!
```

---

## Countdown
```wsp
let countdown = 5

print "Countdown starts:"

while countdown > 0 {
    print countdown
    let countdown = countdown - 1
}

print "Liftoff!"
```

**File:** `examples/countdown.wsp`

Output:
```text
Countdown starts:
5
4
3
2
1
Liftoff!
```

---

## Logical Operators
```wsp
let x = 10
let y = 20

# Using 'and'
if x > 5 and y > 15 {
    print "Both conditions are true"
}

# Using 'or'
if x > 100 or y > 15 {
    print "At least one condition is true"
}

# Using 'not'
let is_false = false
if not is_false {
    print "Negation works!"
}

# Complex logical expression
let a = true
let b = false

if a and not b {
    print "a is true AND b is false"
}

# Combining with comparisons
let temperature = 25

if temperature > 20 and temperature < 30 {
    print "Perfect temperature!"
}
```

**File:** `examples/logical_operators.wsp`

Output:
```text
Both conditions are true
At least one condition is true
Negation works!
a is true AND b is false
Perfect temperature!
```

---

## FizzBuzz
```wsp
let n = 1

while n <= 15 {
    # Check divisibility by 3 and 5
    if n == 3 or n == 6 or n == 9 or n == 12 {
        print "Fizz"
    } else {
        if n == 5 or n == 10 {
            print "Buzz"
        } else {
            if n == 15 {
                print "FizzBuzz"
            } else {
                print n
            }
        }
    }
    
    let n = n + 1
}
```

**File:** `examples/fizzbuzz.wsp`

Output:
```text
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
```

---

## Functions - Basic
```wsp
fn greet(name) {
    print "Hello, " + name + "!"
}

greet("World")
greet("Whispem")
```

**File:** `examples/function_basic.wsp`

Output:
```text
Hello, World!
Hello, Whispem!
```

---

## Functions - Return Values
```wsp
fn add(a, b) {
    return a + b
}

fn multiply(x, y) {
    return x * y
}

let sum = add(5, 3)
let product = multiply(4, 7)

print sum
print product

let result = add(10, multiply(2, 5))
print result
```

**File:** `examples/function_return.wsp`

Output:
```text
8
28
20
```

---

## Functions - Recursion
```wsp
fn factorial(n) {
    if n <= 1 {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}

print factorial(5)
print factorial(6)
print factorial(10)
```

**File:** `examples/function_recursive.wsp`

Output:
```text
120
720
3628800
```

---

## Functions - No Parameters
```wsp
fn say_hello() {
    print "Hello from a function!"
}

fn get_pi() {
    return 3.14159
}

say_hello()
let pi = get_pi()
print pi
```

**File:** `examples/function_no_params.wsp`

Output:
```text
Hello from a function!
3.14159
```

---

## Comments
```wsp
# This is a comment
# Comments start with # and continue to the end of the line

let x = 42  # You can also put comments after code

# Comments are ignored by the interpreter
# They're useful for documentation and explanations

print x  # Output: 42
```

**File:** `examples/comments.wsp`

Output:
```text
42
```

---

## Notes

- Examples are intentionally small and readable
- There is no hidden control flow
- Execution always happens top-to-bottom
- Each example focuses on a single concept
- All examples are runnable and tested
- Functions must be defined before they are called
