# Whispem Tutorial

**Version 1.0.0**

Welcome to Whispem! This tutorial will guide you from your very first program to building complete applications. By the end, you'll understand the entire language and be ready to create your own projects.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Your First Program](#your-first-program)
3. [Variables and Types](#variables-and-types)
4. [Expressions and Operators](#expressions-and-operators)
5. [Working with Strings](#working-with-strings)
6. [Control Flow](#control-flow)
7. [Loops](#loops)
8. [Functions](#functions)
9. [Arrays](#arrays)
10. [User Input and File I/O](#user-input-and-file-io)
11. [Building Complete Programs](#building-complete-programs)
12. [Next Steps](#next-steps)

---

## Getting Started

### Prerequisites

- Basic familiarity with using a terminal/command line
- A code editor (VS Code, Sublime Text, or any text editor)
- Rust installed (to build Whispem)

### Installation

Clone and build Whispem:

```bash
git clone https://github.com/whispem/whispem-lang.git
cd whispem-lang
cargo build --release
```

The binary will be at `target/release/whispem`.

### Running Your First Program

Create a file called `test.wsp`:

```wsp
print "Hello, Whispem!"
```

Run it:

```bash
cargo run examples/test.wsp
```

You should see:
```
Hello, Whispem!
```

**Congratulations!** You just ran your first Whispem program!

---

## Your First Program

Let's understand what happened:

```wsp
print "Hello, Whispem!"
```

- `print` is a built-in statement that outputs to the console
- `"Hello, Whispem!"` is a string (text in double quotes)
- No semicolons needed—Whispem is line-oriented

### Exercise 1: Modify Your Program

Try changing the message:

```wsp
print "I'm learning Whispem!"
print "This is my first program"
```

**Output:**
```
I'm learning Whispem!
This is my first program
```

---

## Variables and Types

### Declaring Variables

Variables store values. Use `let` to create them:

```wsp
let name = "Emilie"
let age = 26
let is_learning = true

print name
print age
print is_learning
```

**Output:**
```
Emilie
26
true
```

### Understanding Types

Whispem has four basic types:

1. **Numbers** (floating point): `42`, `3.14`, `-10`
2. **Strings**: `"hello"`, `"Whispem"`
3. **Booleans**: `true`, `false`
4. **Arrays**: `[1, 2, 3]`, `["a", "b"]`

Types are inferred automatically—you don't need to specify them!

### Reassigning Variables

You can change a variable's value:

```wsp
let counter = 0
print counter          # 0

let counter = 10
print counter          # 10

let counter = counter + 5
print counter          # 15
```

**Important:** Each reassignment uses `let` again—it creates a new binding with the same name.

### Exercise 2: Variables Practice

```wsp
let favorite_color = "blue"
let lucky_number = 7
let loves_rust = true

print favorite_color
print lucky_number
print loves_rust

# Now change them!
let favorite_color = "green"
let lucky_number = 13

print favorite_color
print lucky_number
```

---

## Expressions and Operators

### Arithmetic

Whispem supports basic math:

```wsp
let a = 10
let b = 3

print a + b    # 13
print a - b    # 7
print a * b    # 30
print a / b    # 3.333...
```

### Operator Precedence

Just like in math, multiplication and division happen before addition and subtraction:

```wsp
let result = 10 + 5 * 2
print result    # 20 (not 30!)

let result2 = (10 + 5) * 2
print result2   # 30
```

**Rule of thumb:** Use parentheses when in doubt!

### Negative Numbers

Use the `-` operator:

```wsp
let negative = -42
let opposite = -negative

print negative    # -42
print opposite    # 42
```

### Exercise 3: Calculator

Create a simple calculator:

```wsp
let num1 = 15
let num2 = 4

let sum = num1 + num2
let difference = num1 - num2
let product = num1 * num2
let quotient = num1 / num2

print "Sum:"
print sum
print "Difference:"
print difference
print "Product:"
print product
print "Quotient:"
print quotient
```

---

## Working with Strings

### String Basics

Strings are text in double quotes:

```wsp
let greeting = "Hello"
let name = "World"
```

### String Concatenation

Use `+` to join strings:

```wsp
let greeting = "Hello"
let name = "World"
let message = greeting + ", " + name + "!"

print message    # Hello, World!
```

### Escape Sequences

Special characters inside strings:

```wsp
let multiline = "Line 1\nLine 2\nLine 3"
print multiline

let quoted = "She said \"Hello!\""
print quoted

let tabbed = "Name:\tEmilie"
print tabbed
```

**Output:**
```
Line 1
Line 2
Line 3
She said "Hello!"
Name:	Emilie
```

**Available escape sequences:**
- `\n` - newline
- `\t` - tab
- `\r` - carriage return
- `\\` - backslash
- `\"` - double quote

### String Length

```wsp
let text = "Whispem"
let len = length(text)
print len    # 7
```

### Exercise 4: Personal Introduction

```wsp
let first_name = "Emilie"
let last_name = "Peretti"
let city = "Marseille"

let intro = "Hello! I'm " + first_name + " " + last_name
let location = "I live in " + city
let passion = "I love learning Rust and building languages!"

print intro
print location
print passion
```

---

## Control Flow

### If/Else Statements

Make decisions in your code:

```wsp
let temperature = 25

if temperature > 20 {
    print "It's warm!"
} else {
    print "It's cool!"
}
```

### Comparison Operators

- `<` less than
- `>` greater than
- `<=` less than or equal
- `>=` greater than or equal
- `==` equal to
- `!=` not equal to

```wsp
let x = 10
let y = 20

if x < y {
    print "x is smaller"
}

if x == 10 {
    print "x equals 10"
}

if x != y {
    print "x and y are different"
}
```

### Nested Conditions

```wsp
let score = 85

if score >= 90 {
    print "Grade: A"
} else {
    if score >= 80 {
        print "Grade: B"
    } else {
        if score >= 70 {
            print "Grade: C"
        } else {
            print "Grade: F"
        }
    }
}
```

### Logical Operators

Combine conditions with `and`, `or`, and `not`:

```wsp
let age = 26
let has_license = true

# AND - both conditions must be true
if age >= 18 and has_license {
    print "Can drive!"
}

# OR - at least one condition must be true
if age < 18 or not has_license {
    print "Cannot drive"
}

# NOT - negates a boolean
let is_raining = false
if not is_raining {
    print "No umbrella needed!"
}
```

### Exercise 5: Temperature Advisor

```wsp
let temp = 18
let is_sunny = true

if temp > 25 {
    print "It's hot! Stay hydrated."
} else {
    if temp > 15 and is_sunny {
        print "Perfect weather for a walk!"
    } else {
        if temp < 10 {
            print "It's cold! Wear a jacket."
        } else {
            print "It's mild."
        }
    }
}
```

---

## Loops

### While Loops

Repeat code while a condition is true:

```wsp
let counter = 0

while counter < 5 {
    print counter
    let counter = counter + 1
}

print "Done!"
```

**Output:**
```
0
1
2
3
4
Done!
```

### For Loops

Iterate over arrays:

```wsp
for number in [1, 2, 3, 4, 5] {
    print number
}
```

**Output:**
```
1
2
3
4
5
```

### Using range()

Generate number sequences:

```wsp
for i in range(0, 10) {
    print i
}
```

**Output:** 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

### Break and Continue

Control loop flow:

```wsp
# Break - exit the loop early
for num in range(1, 20) {
    if num > 10 {
        break
    }
    print num
}

# Continue - skip to next iteration
for num in range(1, 10) {
    if num == 5 {
        continue
    }
    print num
}
```

### Exercise 6: Multiplication Table

```wsp
let number = 7

for i in range(1, 11) {
    let result = number * i
    print result
}
```

---

## Functions

### Defining Functions

Group reusable code:

```wsp
fn greet(name) {
    print "Hello, " + name + "!"
}

greet("Emilie")
greet("World")
```

**Output:**
```
Hello, Emilie!
Hello, World!
```

### Return Values

Functions can return values:

```wsp
fn add(a, b) {
    return a + b
}

let sum = add(5, 3)
print sum    # 8
```

### Multiple Parameters

```wsp
fn calculate_area(width, height) {
    return width * height
}

let area = calculate_area(10, 5)
print area    # 50
```

### Functions Without Parameters

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

### Recursion

Functions can call themselves:

```wsp
fn factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

print factorial(5)    # 120
print factorial(6)    # 720
```

### Exercise 7: Temperature Converter

```wsp
fn celsius_to_fahrenheit(celsius) {
    return celsius * 9 / 5 + 32
}

fn fahrenheit_to_celsius(fahrenheit) {
    return (fahrenheit - 32) * 5 / 9
}

let temp_c = 25
let temp_f = celsius_to_fahrenheit(temp_c)
print "25°C in Fahrenheit:"
print temp_f

let back_to_c = fahrenheit_to_celsius(temp_f)
print "Back to Celsius:"
print back_to_c
```

---

## Arrays

### Creating Arrays

```wsp
let numbers = [1, 2, 3, 4, 5]
let names = ["Alice", "Bob", "Charlie"]
let mixed = [1, "hello", true, 3.14]
let empty = []
```

### Accessing Elements

Use `[index]` (0-based indexing):

```wsp
let fruits = ["apple", "banana", "cherry"]

print fruits[0]    # apple
print fruits[1]    # banana
print fruits[2]    # cherry
```

### Modifying Elements

```wsp
let numbers = [1, 2, 3]
numbers[1] = 10

print numbers    # [1, 10, 3]
```

### Array Functions

**length()** - get array size:

```wsp
let items = [1, 2, 3, 4, 5]
let size = length(items)
print size    # 5
```

**push()** - add element (returns new array):

```wsp
let numbers = [1, 2, 3]
let numbers = push(numbers, 4)
print numbers    # [1, 2, 3, 4]
```

**pop()** - remove last element:

```wsp
let items = [1, 2, 3, 4]
let last = pop(items)
print last    # 4
```

**reverse()** - reverse array:

```wsp
let nums = [1, 2, 3, 4, 5]
let reversed = reverse(nums)
print reversed    # [5, 4, 3, 2, 1]
```

**slice()** - get sub-array:

```wsp
let data = [10, 20, 30, 40, 50]
let middle = slice(data, 1, 4)
print middle    # [20, 30, 40]
```

**range()** - generate sequence:

```wsp
let nums = range(0, 5)
print nums    # [0, 1, 2, 3, 4]
```

### Iterating Over Arrays

```wsp
let colors = ["red", "green", "blue"]

for color in colors {
    print color
}
```

### Nested Arrays

```wsp
let matrix = [[1, 2], [3, 4], [5, 6]]
print matrix[0]       # [1, 2]
print matrix[1][0]    # 3
```

### Exercise 8: Array Processing

```wsp
fn sum_array(arr) {
    let total = 0
    for num in arr {
        let total = total + num
    }
    return total
}

fn find_max(arr) {
    let max = arr[0]
    for num in arr {
        if num > max {
            let max = num
        }
    }
    return max
}

let numbers = [5, 2, 9, 1, 7, 3]

let sum = sum_array(numbers)
print "Sum:"
print sum

let max = find_max(numbers)
print "Max:"
print max
```

---

## User Input and File I/O

### Reading User Input

```wsp
let name = input("What's your name? ")
print "Hello, " + name + "!"

let age = input("How old are you? ")
print "You are " + age + " years old."
```

### Writing to Files

```wsp
let message = "Hello from Whispem!"
write_file("output.txt", message)
print "File written!"
```

### Reading from Files

```wsp
let content = read_file("output.txt")
print "File contents:"
print content
```

### Exercise 9: Guest Book

```wsp
# Collect guest information
let name = input("Enter your name: ")
let message = input("Leave a message: ")

# Create entry
let entry = name + ": " + message + "\n"

# Write to file
write_file("guestbook.txt", entry)

print "Thank you! Your message has been saved."
```

---

## Building Complete Programs

### Project 1: Number Guessing Game

```wsp
let secret = 7
let attempts = 0
let found = false

print "=== Number Guessing Game ==="
print "I'm thinking of a number between 1 and 10"

while not found {
    let guess_str = input("Enter your guess: ")
    let attempts = attempts + 1
    
    # Simple number conversion
    let guess = 0
    if guess_str == "1" { let guess = 1 }
    if guess_str == "2" { let guess = 2 }
    if guess_str == "3" { let guess = 3 }
    if guess_str == "4" { let guess = 4 }
    if guess_str == "5" { let guess = 5 }
    if guess_str == "6" { let guess = 6 }
    if guess_str == "7" { let guess = 7 }
    if guess_str == "8" { let guess = 8 }
    if guess_str == "9" { let guess = 9 }
    if guess_str == "10" { let guess = 10 }
    
    if guess == secret {
        print "Correct! You found it!"
        let found = true
    } else {
        if guess < secret {
            print "Too low! Try again."
        } else {
            print "Too high! Try again."
        }
    }
}

print "Game over! Total attempts: " + guess_str
```

### Project 2: Todo List Manager

```wsp
fn show_menu() {
    print ""
    print "=== Todo List ==="
    print "1. Show tasks"
    print "2. Add task"
    print "3. Remove last task"
    print "4. Exit"
}

fn show_tasks(tasks) {
    if length(tasks) == 0 {
        print "No tasks yet!"
        return
    }
    
    print ""
    print "Your tasks:"
    let i = 1
    for task in tasks {
        print i
        print ". " + task
        let i = i + 1
    }
}

# Main program
let tasks = []
let running = true

print "Welcome to Whispem Todo!"

while running {
    show_menu()
    let choice = input("Choose an option: ")
    
    if choice == "1" {
        show_tasks(tasks)
    }
    
    if choice == "2" {
        let task = input("Enter task: ")
        let tasks = push(tasks, task)
        print "Task added!"
    }
    
    if choice == "3" {
        if length(tasks) > 0 {
            let removed = pop(tasks)
            print "Removed: " + removed
        } else {
            print "No tasks to remove!"
        }
    }
    
    if choice == "4" {
        let running = false
        print "Goodbye!"
    }
}
```

### Project 3: Data Analyzer

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

fn calculate_average(arr) {
    if length(arr) == 0 {
        return 0
    }
    
    let total = 0
    for num in arr {
        let total = total + num
    }
    
    return total / length(arr)
}

fn find_min_max(arr) {
    let min = arr[0]
    let max = arr[0]
    
    for num in arr {
        if num < min {
            let min = num
        }
        if num > max {
            let max = num
        }
    }
    
    print "Min: " + min
    print "Max: " + max
}

# Main program
let data = [-5, 3, -2, 8, 0, 12, -1, 7, 4]

print "Original data:"
print data

let positive = filter_positive(data)
print ""
print "Positive numbers:"
print positive

let avg = calculate_average(positive)
print ""
print "Average of positive numbers:"
print avg

print ""
find_min_max(positive)
```

---

## Next Steps

Congratulations! You've completed the Whispem tutorial. You now know:

- Variables and types  
- Expressions and operators  
- Strings and concatenation  
- Control flow (if/else)  
- Loops (while, for, break, continue)  
- Functions and recursion  
- Arrays and array operations  
- User input and file I/O  
- Building complete programs

### What to Do Next

1. **Build Your Own Projects**
   - Create a calculator
   - Build a text-based game
   - Make a file organizer
   - Write a data processor

2. **Explore the Examples**
   - Check out `examples/` directory
   - Study the code patterns
   - Modify and experiment

3. **Read the Documentation**
   - `docs/syntax.md` - Complete syntax reference
   - `docs/examples.md` - More code examples
   - `docs/vision.md` - Language philosophy

4. **Join the Community**
   - Report bugs on GitHub Issues
   - Share your projects
   - Ask questions in Discussions

5. **Challenge Yourself**
   - Implement a sorting algorithm
   - Create a simple database
   - Build a text adventure game
   - Write a Markdown parser

### Learning Resources

- **The Whispem Repository**: [github.com/whispem/whispem-lang](https://github.com/whispem/whispem-lang)
- **Syntax Reference**: `docs/syntax.md`
- **Example Programs**: `examples/` directory

### Tips for Success

- **Write code every day** - even just 10 minutes!
- **Read error messages carefully** - they tell you exactly what's wrong
- **Start small** - build simple things first
- **Experiment** - try things out, break stuff, learn
- **Have fun** - programming should be enjoyable!

---

## Common Patterns

### Input Validation

```wsp
let valid = false

while not valid {
    let input = input("Enter a number 1-10: ")
    
    # Check if valid
    let num = 0
    if input == "1" { let num = 1; let valid = true }
    if input == "2" { let num = 2; let valid = true }
    # ... etc
    
    if not valid {
        print "Invalid input! Try again."
    }
}
```

### Building Dynamic Arrays

```wsp
fn build_sequence(start, count) {
    let result = []
    let i = 0
    
    while i < count {
        let result = push(result, start + i)
        let i = i + 1
    }
    
    return result
}

let nums = build_sequence(10, 5)
print nums    # [10, 11, 12, 13, 14]
```

### Menu Systems

```wsp
let running = true

while running {
    print "1. Option A"
    print "2. Option B"
    print "3. Exit"
    
    let choice = input("Choose: ")
    
    if choice == "1" {
        # Do option A
    }
    if choice == "2" {
        # Do option B
    }
    if choice == "3" {
        let running = false
    }
}
```

---

## Final Thoughts

> *"Whispem whispers intent, not shout complexity. Code should be quiet, clear, and calm."*

You've learned a complete programming language! The skills you've gained here—logic, problem-solving, structured thinking—transfer to any language.

Remember:
- Every expert was once a beginner
- Making mistakes is how you learn
- The best way to learn is by building

Now go create something amazing! 

---

**Happy coding!**

*— Emilie Peretti*

**Version:** 1.0.0  
**Tutorial Status:** Complete  
**Your Status:** Ready to build!
