# Changelog

## v1.5.0 — Dictionaries, Modulo & REPL

### New Features

#### Dictionaries
- Added dictionary type with `{"key": value}` literal syntax
- Added dictionary indexing with `dict["key"]`
- Added dictionary assignment with `dict["key"] = value`
- Added `keys(dict)` — returns sorted array of keys
- Added `values(dict)` — returns array of values sorted by key
- Added `has_key(dict, key)` — returns boolean
- `length()` now works on dictionaries
- Dictionary keys can be strings or numbers

#### Modulo Operator
- Added `%` modulo operator
- Proper precedence (same level as `*` and `/`)
- Division by zero error on `% 0`

#### Interactive REPL
- `cargo run` (no arguments) launches an interactive REPL
- Persistent state between lines
- Type `exit` or `quit` to leave
- Ctrl-D to exit

### Error Handling — Complete Overhaul
- All `panic!` replaced by a proper `Result<T, WhispemError>` system
- Every error now shows line number and column
- New `src/error.rs` module with typed `ErrorKind` enum
- Errors propagate cleanly through lexer → parser → interpreter
- Process exits with code 1 on error (no more crashes)

### Architecture Changes
- New `src/error.rs` — central error type
- New `src/repl.rs` — REPL implementation
- `Token::String` renamed to `Token::Str` (avoids ambiguity with Rust's `String`)
- `Expr::String` renamed to `Expr::Str`
- AST operators now use typed enums (`BinaryOp`, `UnaryOp`, `LogicalOp`) instead of strings
- All statements carry source line number for runtime error reporting
- `Lexer::tokenize()` replaces the old `next_token()` loop — returns `Vec<Spanned>` or error

### Breaking Changes
- None for `.wsp` programs — fully backwards compatible with v1.0.0

---

## v1.0.0 — Production Release 🎉

### New Language Features
- Added `for` loops for easier iteration over arrays
- Added `break` statement to exit loops early
- Added `continue` statement to skip to next iteration
- Added `range(start, end)` built-in for generating number sequences

### Enhanced Built-in Functions
- Added `pop(array)` — removes and returns last element
- Added `reverse(array)` — returns reversed array
- Added `slice(array, start, end)` — returns sub-array
- Added `input(prompt)` — read user input from console
- Added `read_file(filename)` — read text file contents
- Added `write_file(filename, content)` — write to text file

### Error Handling Improvements
- Line and column tracking in lexer
- Better error messages with context
- Array index out of bounds shows array length
- Function argument count mismatch shows expected vs actual
- Division by zero error

### Breaking Changes
- None — fully backwards compatible with v0.9.0

---

## v0.9.0 — Arrays

- Added array literals with `[...]` syntax
- Added array indexing with `array[index]`
- Added array assignment with `array[index] = value`
- Added built-in `length()` function for arrays and strings
- Added built-in `push()` function to append elements to arrays
- Arrays can contain mixed types
- Arrays can be passed to and returned from functions

---

## v0.8.0 — Functions

- Added function declarations with `fn`
- Added `return` statement
- Added function calls with arguments
- Added call stack and local scopes
- Added string concatenation with `+`
- Functions support recursion

---

## v0.7.0 — Loops & Logic

- Added `while` loops
- Added logical operators: `and`, `or`, `not`
- Added unary operators: `-`, `!`
- Added escape sequences in strings: `\n`, `\t`, `\r`, `\\`, `\"`
- Short-circuit evaluation for `and`/`or`

---

## v0.6.0 — Control Flow

- Added boolean values
- Added comparison operators
- Added `if / else`
- Introduced block syntax

---

## v0.5.0 — Expressions

- Operator precedence
- Parentheses support

---

## v0.4.0 — CLI

- File-based execution
- Cleaner CLI output

---

## v0.3.0 — Variables

- `let` bindings
- Basic interpreter execution

---

## v0.2.0 — Lexer & Tokens

- Tokenizer implementation

---

## v0.1.0 — Initial release

- Project initialization
- First executable prototype