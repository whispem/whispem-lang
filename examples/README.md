# Whispem Examples

This directory contains example Whispem programs demonstrating all language features.

All examples can be run using:
```bash
cargo run examples/<file>.wsp
```

---

## 📚 Available Examples

### Basic Examples

#### **hello.wsp**
```wsp
let message = "Hello, Whispem!"
print message
```
Output: `Hello, Whispem!`

---

#### **variables.wsp**
Demonstrates variable declaration and usage.

Output:
```
10
20
Whispem
```

---

#### **arithmetic.wsp**
Shows arithmetic expressions with operator precedence and parentheses.

Output:
```
15
5
50
2
20
30
```

---

#### **strings.wsp**
String literals with escape sequences and concatenation (`\n`, `\t`, `\"`, `+`).

Output:
```
Hello
World
Whispem is minimalist
Line one
Line two
She said "Hello"
Hello, World!
```

---

#### **boolean.wsp**
Boolean values and their usage.

Output:
```
true
false
true
```

---

#### **comments.wsp**
Comment syntax demonstration.

Output:
```
42
```

---

### Control Flow

#### **comparison.wsp**
All comparison operators: `<`, `>`, `<=`, `>=`, `==`, `!=`

Output:
```
10 is less than 20
20 is greater than 10
a equals 5
x and y are different
b is less than or equal to 10
b is greater than or equal to 10
```

---

#### **condition.wsp**
Conditional execution with `if/else` including nested conditions.

Output:
```
It's cool
Grade: B
Enjoy the sunshine
```

---

### Loops (v0.7.0)

#### **while_loop.wsp**
Basic while loop demonstrating iteration.

Output:
```
0
1
2
3
4
Done!
```

---

#### **countdown.wsp**
Countdown example using while loop.

Output:
```
Countdown starts:
5
4
3
2
1
Liftoff!
```

---

### Logical Operators (v0.7.0)

#### **logical_operators.wsp**
Demonstrates `and`, `or`, and `not` operators with various combinations.

Output:
```
Both conditions are true
At least one condition is true
Negation works!
a is true AND b is false
Perfect temperature!
```

---

### Functions (v0.8.0)

#### **function_basic.wsp**
Basic function declaration and calling.
```wsp
fn greet(name) {
    print "Hello, " + name + "!"
}

greet("World")
greet("Whispem")
```

Output:
```
Hello, World!
Hello, Whispem!
```

---

#### **function_return.wsp**
Functions with return values and expressions.

Output:
```
8
28
20
```

---

#### **function_recursive.wsp**
Recursive function example (factorial).

Output:
```
120
720
3628800
```

---

#### **function_no_params.wsp**
Functions without parameters.

Output:
```
Hello from a function!
3.14159
```

---

### Advanced Examples

#### **fizzbuzz.wsp**
Classic FizzBuzz implementation using while loops and logical operators.

Output:
```
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

## 🧪 Test Files

#### **test_control_flow.wsp**
Tests if/else, comparisons, and booleans.

#### **test_v0.7.0.wsp**
Comprehensive test covering all v0.7.0 features.

#### **test_v0.8.0.wsp**
Comprehensive test covering all v0.8.0 features including:
- Function declarations
- Return values
- Recursion
- String concatenation
- Functions in expressions
- While loops with functions

---

## 📖 Learning Path

Recommended order for learning:

1. **hello.wsp** - Start here
2. **variables.wsp** - Variable basics
3. **arithmetic.wsp** - Expressions
4. **strings.wsp** - String handling
5. **boolean.wsp** - Boolean values
6. **comparison.wsp** - Comparisons
7. **condition.wsp** - If/else
8. **logical_operators.wsp** - And/or/not
9. **while_loop.wsp** - Loops
10. **countdown.wsp** - Loop example
11. **function_basic.wsp** - Functions intro
12. **function_return.wsp** - Return values
13. **function_recursive.wsp** - Recursion
14. **fizzbuzz.wsp** - Putting it all together

---

## 🎯 Quick Test

Run all examples at once:
```bash
for file in examples/*.wsp; do
    echo "Running $file..."
    cargo run "$file"
    echo "---"
done
```

---

## 💡 Tips

- Each example is self-contained and runnable
- Examples include comments explaining the code
- Start with simple examples and progress to complex ones
- All examples demonstrate best practices
- Use examples as templates for your own programs
- Functions must be defined before they are called

---

## 🆕 What's New in v0.8.0

- **function_basic.wsp** - Learn function syntax
- **function_return.wsp** - Master return values
- **function_recursive.wsp** - Explore recursion
- **function_no_params.wsp** - Functions without parameters
- **test_v0.8.0.wsp** - Complete v0.8.0 test suite

String concatenation is now available in all examples!

---

**Version:** 0.8.0  
**Examples:** 17 files  
**Features:** All language features covered including functions!
