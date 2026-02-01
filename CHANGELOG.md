# Changelog

## v1.0.0 — Production Release 🎉

**Major milestone: Whispem is now production-ready!**

### New Language Features
- Added `for` loops for easier iteration over arrays
- Added `break` statement to exit loops early
- Added `continue` statement to skip to next iteration
- Added `range(start, end)` built-in for generating number sequences

### Enhanced Built-in Functions
- Added `pop(array)` - removes and returns last element
- Added `reverse(array)` - returns reversed array
- Added `slice(array, start, end)` - returns sub-array
- Added `input(prompt)` - read user input from console
- Added `read_file(filename)` - read text file contents
- Added `write_file(filename, content)` - write to text file

### Error Handling Improvements
- Line and column tracking in lexer
- Better error messages with context
- Array index out of bounds shows array length
- Function argument count mismatch shows expected vs actual
- Division by zero error
- Proper error messages for all built-in functions

### Quality of Life
- Improved error messages for undefined variables
- Improved error messages for type mismatches
- Better function call error reporting
- File I/O error handling with descriptive messages

### Breaking Changes
- None! Fully backwards compatible with v0.9.0

## v0.9.0 — Arrays

- Added array literals with `[...]` syntax
- Added array indexing with `array[index]`
- Added array assignment with `array[index] = value`
- Added built-in `length()` function for arrays and strings
- Added built-in `push()` function to append elements to arrays
- Arrays can contain mixed types (numbers, strings, booleans, nested arrays)
- Arrays can be passed to functions and returned from functions
- Arrays display properly when printed

## v0.8.0 — Functions

- Added function declarations with `fn`
- Added `return` statement for returning values
- Added function calls with arguments
- Added call stack and local scopes
- Added string concatenation with `+` operator
- Functions support recursion
- Functions can have zero or more parameters
- Proper variable scoping (local vs global)
- Return values can be used in expressions

## v0.7.0 — Loops & Logic

- Added `while` loops for iteration
- Added logical operators: `and`, `or`, `not`
- Added unary operators: `-` (negation), `!` (not)
- Fixed lexer: now properly tokenizes `{`, `}`, `<`, `>`, `<=`, `>=`, `==`, `!=`
- Fixed lexer: now recognizes `if`, `else`, `true`, `false` keywords
- Added escape sequences in strings: `\n`, `\t`, `\r`, `\\`, `\"`
- Improved number parsing (prevents invalid floats)
- Added short-circuit evaluation for `and`/`or`

## v0.6.0 — Control Flow

- Added boolean values
- Added comparison operators
- Added `if / else` control flow
- Introduced block syntax
- Updated documentation
- Stabilized interpreter core

## v0.5.0 — Expressions

- Operator precedence
- Parentheses support
- Improved expression parsing

## v0.4.0 — CLI Improvements

- File-based execution
- Cleaner CLI output

## v0.3.0 — Variables

- `let` bindings
- Basic interpreter execution

## v0.2.0 — Lexer & Tokens

- Tokenizer implementation
- Basic language structure

## v0.1.0 — Initial release

- Project initialization
- First executable prototype
