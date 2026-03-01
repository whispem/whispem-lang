# Whispem VM — Specification

**Version 2.5.0**

> *"A virtual machine should be as simple as the language it runs."*

This document is the complete specification of the Whispem Virtual Machine (WVM).  
It is intentionally written to be readable by a human — and eventually, by a Whispem program.

---

## Table of Contents

1. [Philosophy](#philosophy)
2. [Architecture Overview](#architecture-overview)
3. [Data Types](#data-types)
4. [Instruction Set](#instruction-set)
5. [Chunk Format](#chunk-format)
6. [Call Frames](#call-frames)
7. [Execution Model](#execution-model)
8. [Variable Scoping](#variable-scoping)
9. [Error Handling](#error-handling)
10. [Compilation: AST → Bytecode](#compilation-ast--bytecode)
11. [Example: Annotated Bytecode](#example-annotated-bytecode)
12. [Built-in Functions](#built-in-functions)
13. [Source Files](#source-files)

---

## Philosophy

The Whispem VM (WVM) follows the same principles as the language itself:

- **Small** — the entire instruction set fits on one page
- **Explicit** — every opcode does exactly one thing
- **Readable** — bytecode can be disassembled into human-readable form
- **Bootstrappable** — simple enough that a future Whispem compiler can target it

The WVM is a **stack-based virtual machine**.  
All operations read from and write to an operand stack. There is no register file.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────┐
│                    Whispem VM                       │
│                                                     │
│  ┌───────────────┐     ┌──────────────────────────┐ │
│  │   Compiler    │────▶│  Chunk (bytecode +        │ │
│  │  (AST→bytes)  │     │    constants pool)         │ │
│  └───────────────┘     └────────────┬──────────────┘ │
│                                     │               │
│                                     ▼               │
│                          ┌──────────────────────────┐│
│                          │       VM Core            ││
│                          │                          ││
│                          │  ip         (per frame)  ││
│                          │  stack      Vec<Value>   ││
│                          │  frames     Vec<Frame>   ││
│                          │  globals    HashMap      ││
│                          │  functions  HashMap      ││
│                          │  output     Box<Write>   ││
│                          └──────────────────────────┘│
└─────────────────────────────────────────────────────┘
```

**Key components:**

- **Compiler** — walks the AST and emits one `Chunk` per function + one for `<main>`
- **Chunk** — flat array of bytes + constants pool + per-byte line numbers + `param_count`
- **VM Core** — a loop that reads one opcode at a time and executes it
- **Stack** — shared across all frames; all intermediate values live here
- **Call frames** — one `CallFrame` per active function call, each with its own `ip` and `locals`
- **Globals** — `HashMap<String, Value>` for top-level variables, copied into functions on call
- **Output** — `Box<dyn Write + Send>`; defaults to stdout, substituted in tests

---

## Data Types

The VM works with the same types as the language:

| Type     | Rust representation         | Notes                              |
|----------|-----------------------------|------------------------------------|
| `number` | `f64`                       | All numbers are floating point     |
| `string` | `String`                    | UTF-8                              |
| `bool`   | `bool`                      | `true` / `false`                   |
| `array`  | `Vec<Value>`                | Ordered, mixed types               |
| `dict`   | `HashMap<String, Value>`    | Keys are always strings            |
| `none`   | —                           | Returned by void functions         |

Defined in `src/value.rs`.

---

## Instruction Set

Each instruction is one byte (the **opcode**), optionally followed by operand bytes.

### Notation

```
OPCODE              — no operands
OPCODE <u8>         — one-byte operand (0–255)
OPCODE <u16>        — two-byte operand, big-endian (0–65535)
OPCODE <u8> <u8>    — two separate one-byte operands
```

### Complete Opcode Table

| Code   | Name                  | Operands    | Stack effect               | Description                                             |
|--------|-----------------------|-------------|----------------------------|---------------------------------------------------------|
| `0x00` | `PUSH_CONST`          | `<u8>`      | `( -- value )`             | Push constant at pool index                             |
| `0x01` | `PUSH_TRUE`           | —           | `( -- true )`              | Push boolean `true`                                     |
| `0x02` | `PUSH_FALSE`          | —           | `( -- false )`             | Push boolean `false`                                    |
| `0x03` | `PUSH_NONE`           | —           | `( -- none )`              | Push `none`                                             |
| `0x10` | `LOAD`                | `<u8>`      | `( -- value )`             | Push value of variable (name at const index)            |
| `0x11` | `STORE`               | `<u8>`      | `( value -- )`             | Pop and store into variable (name at const index)       |
| `0x20` | `ADD`                 | —           | `( a b -- a+b )`           | Add numbers or concatenate strings                      |
| `0x21` | `SUB`                 | —           | `( a b -- a-b )`           | Subtract                                                |
| `0x22` | `MUL`                 | —           | `( a b -- a*b )`           | Multiply                                                |
| `0x23` | `DIV`                 | —           | `( a b -- a/b )`           | Divide (error on zero)                                  |
| `0x24` | `MOD`                 | —           | `( a b -- a%b )`           | Modulo                                                  |
| `0x25` | `NEG`                 | —           | `( a -- -a )`              | Negate number                                           |
| `0x30` | `EQ`                  | —           | `( a b -- bool )`          | Equal                                                   |
| `0x31` | `NEQ`                 | —           | `( a b -- bool )`          | Not equal                                               |
| `0x32` | `LT`                  | —           | `( a b -- bool )`          | Less than                                               |
| `0x33` | `LTE`                 | —           | `( a b -- bool )`          | Less than or equal                                      |
| `0x34` | `GT`                  | —           | `( a b -- bool )`          | Greater than                                            |
| `0x35` | `GTE`                 | —           | `( a b -- bool )`          | Greater than or equal                                   |
| `0x36` | `NOT`                 | —           | `( a -- bool )`            | Logical not                                             |
| `0x40` | `JUMP`                | `<u16>`     | `( -- )`                   | Unconditional jump to absolute byte offset              |
| `0x41` | `JUMP_IF_FALSE`       | `<u16>`     | `( cond -- )`              | **Pop** condition; jump if falsy                        |
| `0x42` | `JUMP_IF_TRUE`        | `<u16>`     | `( cond -- )`              | **Pop** condition; jump if truthy                       |
| `0x43` | `PEEK_JUMP_IF_FALSE`  | `<u16>`     | `( cond -- cond )`         | **Peek** (no pop); jump if falsy — used by `and`        |
| `0x44` | `PEEK_JUMP_IF_TRUE`   | `<u16>`     | `( cond -- cond )`         | **Peek** (no pop); jump if truthy — used by `or`        |
| `0x50` | `CALL`                | `<u8> <u8>` | `( args.. -- retval )`     | Call function: const idx of name + argc                 |
| `0x51` | `RETURN`              | —           | `( value -- )`             | Return value from current function                      |
| `0x52` | `RETURN_NONE`         | —           | `( -- )`                   | Return `none` (implicit at function end)                |
| `0x60` | `MAKE_ARRAY`          | `<u8>`      | `( n items -- array )`     | Pop n items; build array in source order                |
| `0x61` | `MAKE_DICT`           | `<u8>`      | `( n pairs -- dict )`      | Pop n key-value pairs; build dict                       |
| `0x62` | `GET_INDEX`           | —           | `( obj idx -- value )`     | Index into array or dict                                |
| `0x63` | `SET_INDEX`           | —           | `( obj idx val -- obj' )`  | Mutate array/dict; push mutated copy                    |
| `0x70` | `PRINT`               | —           | `( value -- )`             | Write top of stack to the VM output sink                |
| `0x71` | `POP`                 | —           | `( value -- )`             | Discard top of stack                                    |
| `0xFF` | `HALT`                | —           | `( -- )`                   | Stop execution; pop the current frame                   |

**Total: 33 opcodes.**

> **v2.5.0 additions:** `PEEK_JUMP_IF_FALSE` (0x43) and `PEEK_JUMP_IF_TRUE` (0x44).  
> These replace the v2.0.0 pattern of `JUMP_IF_FALSE`/`JUMP_IF_TRUE` for short-circuit `and`/`or`, which incorrectly popped the left-hand value off the stack even when it was the result of the expression.

---

## Chunk Format

```rust
pub struct Chunk {
    pub code:        Vec<u8>,      // raw bytecode
    pub constants:   Vec<Value>,   // constants pool (max 256 entries)
    pub lines:       Vec<usize>,   // one line number per byte in code
    pub name:        String,       // "<main>" or the function name
    pub param_count: usize,        // number of parameters (0 for <main>)
}
```

> **v2.5.0 addition:** `param_count` — the compiler sets this when emitting a function chunk. The VM uses it to verify arity at call time, producing a clear error instead of a silent stack mismatch.

### Constants Pool

The pool stores all values that appear literally in source code: numbers, strings, and variable/function names (as `Value::Str`).

```
constants[0] = "counter"    ← variable name
constants[1] = 0.0          ← number literal
constants[2] = "Hello"      ← string literal
constants[3] = "greet"      ← function name
```

- `PUSH_CONST 2` pushes `"Hello"` onto the stack
- `LOAD 0` looks up the variable named `"counter"`
- `CALL 3 1` calls the function named `"greet"` with 1 argument

The pool is indexed by `u8` → max **256 constants per chunk**.  
String constants are **deduplicated**: the same string reuses the same slot.

---

## Call Frames

```rust
struct CallFrame {
    chunk:  Chunk,
    ip:     usize,
    locals: HashMap<String, Value>,
}
```

The VM keeps a `Vec<CallFrame>`; the top frame is always executing.

**On `CALL name_idx argc`:**
1. Pop `argc` arguments from the stack (last arg on top)
2. If `name` is a built-in → call directly, push result, done
3. Clone the function's `Chunk` from the `functions` map
4. **Check arity:** `argc != chunk.param_count` → `ArgumentCount` error
5. Create a new `CallFrame`; copy globals into its locals (read access)
6. Push arguments back in forward order for the preamble to consume
7. Push the new frame — execution continues inside the function

**Function preamble:**  
The compiler emits `STORE` instructions in **reverse parameter order**. Arguments must be on the stack such that the last param is on top:

```
fn f(a, b, c):  preamble = STORE c, STORE b, STORE a
caller pushes:  a, b, c  → c on top
pops in order:  c ✓, b ✓, a ✓
```

**On `RETURN` / `RETURN_NONE`:**
1. Pop the current `CallFrame`
2. Push the return value for the caller

**On `HALT`:**
1. Pop the current frame (the `<main>` frame)
2. Return `Ok(())`

---

## Execution Model

```
loop:
    byte = frame.chunk.code[frame.ip]
    frame.ip += 1
    match opcode(byte):
        PUSH_CONST  idx  → stack.push(frame.chunk.constants[idx].clone())
        PUSH_TRUE        → stack.push(Value::Bool(true))
        PUSH_FALSE       → stack.push(Value::Bool(false))
        PUSH_NONE        → stack.push(Value::None)
        LOAD        idx  → stack.push(frame.locals[const_str(idx)])
        STORE       idx  → frame.locals[const_str(idx)] = stack.pop()
        ADD              → b=pop(); a=pop(); push(a+b)
        ...              (arithmetic, comparison, logic — all pop 1 or 2, push 1)
        JUMP        target     → frame.ip = target
        JUMP_IF_FALSE  target  → cond=pop(); if !cond.truthy() { frame.ip=target }
        JUMP_IF_TRUE   target  → cond=pop(); if  cond.truthy() { frame.ip=target }
        PEEK_JUMP_IF_FALSE  t  → cond=stack.last(); if !cond.truthy() { frame.ip=t }
        PEEK_JUMP_IF_TRUE   t  → cond=stack.last(); if  cond.truthy() { frame.ip=t }
        CALL  name_idx argc    → (see Call Frames above)
        RETURN                 → frames.pop(); stack.push(retval)
        RETURN_NONE            → frames.pop(); stack.push(None)
        PRINT                  → val=pop(); writeln!(vm.output, "{}", val.format())
        POP                    → stack.pop()
        HALT                   → frames.pop(); break
```

**Truthiness:** a value is falsy if it is `false`, `0`, `""`, `[]`, `{}`, or `none`. Everything else is truthy.

---

## Variable Scoping

Whispem has two scopes: **global** and **local**.

| Scope | Storage | Lifetime |
|-------|---------|----------|
| Global | `vm.globals: HashMap<String, Value>` | Entire program |
| Local | `frame.locals: HashMap<String, Value>` | One function call |

**Rule:** `let x = expr` at the top level → `globals`. Inside a function → `frame.locals`.

Functions have **read access to globals**: on `CALL`, the globals map is cloned into the new frame's locals. This means a function can read global variables but cannot mutate them — there is no bare assignment statement, only `let` (which reassigns a local) and `x[i] = v` (index assignment).

---

## Error Handling

All errors are represented as `WhispemError { kind: ErrorKind, span: Span }`.

### Span

```rust
pub struct Span {
    pub line:   usize,   // 1-based source line
    pub column: usize,   // 1-based column (0 = unknown)
}
```

Constructors:
- `Span::new(line, col)` — known location
- `Span::unknown()` — sentinel for compiler-internal errors without source position

### Error display

```
[line 3, col 0] Error: Undefined variable: 'counter'
[line 7, col 0] Error: Array index 10 out of bounds (length: 5)
[line 12, col 0] Error: Function 'add' expected 2 arguments, got 3
[line 15, col 0] Error: Division by zero
```

Column tracking is recorded by the lexer but currently reported as `0` in most error sites. Column precision is planned for v3.0.0.

### Error kinds

| Kind | When |
|------|------|
| `UndefinedVariable` | `LOAD` of an unknown name |
| `UndefinedFunction` | `CALL` of an unknown name |
| `ArgumentCount` | `CALL` with wrong number of args (user functions only) |
| `TypeError` | Operation on wrong type |
| `IndexOutOfBounds` | Array index out of range |
| `DivisionByZero` | `DIV` or `MOD` with zero divisor |
| `StackUnderflow` | `POP` on an empty stack (compiler bug) |
| `TooManyConstants` | More than 256 constants in one chunk |

---

## Compilation: AST → Bytecode

### Two-pass compilation

Functions are compiled in a **first pass** before the main body. This enables forward calls: you can call a function defined later in the file.

### Jump patching

All control-flow jumps are emitted in two steps:
1. Emit the jump opcode with placeholder `0xFFFF`
2. After compiling the body, **patch** the placeholder with the real offset

`break` and `continue` collect their patch sites on a `LoopContext` stack and are all patched when the enclosing loop finishes compiling.

### Compilation rules

**`let x = expr`**
```
<compile expr>
STORE  <idx("x")>
```

**`print expr`**
```
<compile expr>
PRINT
```

**`expr` (bare statement)**
```
<compile expr>
POP              ← always emitted; keeps the stack clean
```

**`if cond { then } else { else }`**
```
<compile cond>
JUMP_IF_FALSE  [else_start]
<compile then>
JUMP           [after_else]
else_start:
<compile else>             ← omitted if no else branch
after_else:
```

**`while cond { body }`**
```
loop_start:
<compile cond>
JUMP_IF_FALSE  [after_loop]
<compile body>
JUMP           [loop_start]
after_loop:
```

**`for x in iterable { body }`** — desugars to a counter while-loop:
```
<compile iterable>
STORE  __iter_N
PUSH_CONST  0
STORE  __idx_N
loop_start:
  LOAD __idx_N  /  LOAD __iter_N  /  CALL length 1  /  LT
  JUMP_IF_FALSE [after_loop]
  LOAD __iter_N  /  LOAD __idx_N  /  GET_INDEX
  STORE x
  <compile body>
continue_target:
  LOAD __idx_N  /  PUSH_CONST 1  /  ADD  /  STORE __idx_N
  JUMP [loop_start]
after_loop:
```

`break` patches to `after_loop`; `continue` patches to `continue_target`.

**`fn name(params) { body }`** — compiled to a separate chunk, `param_count` set:
```
STORE  <last param>          ← preamble: bind params in reverse
...
STORE  <first param>
<compile body>
RETURN_NONE                  ← implicit fallback
```

**`obj[idx] = value`**
```
LOAD       <idx("obj")>
<compile idx>
<compile value>
SET_INDEX               ← pops val/idx/obj, pushes mutated obj
STORE      <idx("obj")> ← write back
```

**`a and b`** (short-circuit, corrected in v2.5.0)
```
<compile a>
PEEK_JUMP_IF_FALSE [done]  ← a stays on stack if falsy (result = a)
POP                        ← discard a if truthy
<compile b>                ← result = b
done:
```

**`a or b`** (short-circuit, corrected in v2.5.0)
```
<compile a>
PEEK_JUMP_IF_TRUE [done]   ← a stays on stack if truthy (result = a)
POP                        ← discard a if falsy
<compile b>                ← result = b
done:
```

> **Why peek?** The v2.0.0 `JUMP_IF_FALSE`/`JUMP_IF_TRUE` unconditionally popped the top-of-stack. For `and`/`or`, the short-circuited value *is* the result — popping it caused a stack underflow in the caller. `PEEK_JUMP_*` leaves the value in place when jumping, so the result is always on the stack after the expression, regardless of which branch was taken.

---

## Example: Annotated Bytecode

### Simple program

```wsp
let x = 10
let y = x + 5
print y
```

Constants pool: `["x", 10.0, "y", 5.0]`

```
0000  1    PUSH_CONST    1    '10'
0002  1    STORE         0    'x'
0004  2    LOAD          0    'x'
0006  2    PUSH_CONST    3    '5'
0008  2    ADD
0009  2    STORE         2    'y'
0011  3    LOAD          2    'y'
0013  3    PRINT
0014  3    HALT
```

---

### Function call

```wsp
fn double(n) {
    return n * 2
}
print double(7)
```

**Chunk `<main>`** — constants: `["double", 7.0]`
```
0000  4    PUSH_CONST    1    '7'
0002  4    CALL          0    'double' (1 args)
0005  4    PRINT
0006  4    HALT
```

**Chunk `double`** — constants: `["n", 2.0]` — `param_count: 1`
```
0000  1    STORE         0    'n'       ← preamble
0002  2    LOAD          0    'n'
0004  2    PUSH_CONST    1    '2'
0006  2    MUL
0007  2    RETURN
0008  2    RETURN_NONE                  ← implicit fallback
```

---

### Short-circuit `and`

```wsp
let r = false and (1 == 1)
print r
```

```
0000  1    PUSH_FALSE                   ← compile `false`
0001  1    PEEK_JUMP_IF_FALSE  0008     ← false → jump, `false` stays on stack
0004  1    POP                          ← (not reached) discard left
0005  1    PUSH_CONST  ?    '1'
0007  1    PUSH_CONST  ?    '1'
...       EQ
0008  1    STORE  'r'                   ← result = false
0010  2    LOAD   'r'
0012  2    PRINT
0013  2    HALT
```

---

### Index assignment

```wsp
let arr = [1, 2, 3]
arr[1] = 99
print arr
```

**Chunk `<main>`** (relevant fragment)
```
...  MAKE_ARRAY 3
...  STORE 'arr'
     LOAD  'arr'     ← load current array
     PUSH_CONST '1'  ← index
     PUSH_CONST '99' ← new value
     SET_INDEX        ← mutates; pushes result
     STORE 'arr'     ← write back
     LOAD  'arr'
     PRINT
     HALT
```

---

## Built-in Functions

Built-ins are resolved at `CALL` time before checking user-defined functions. They bypass the arity-check path (each built-in validates its own arguments).

| Name         | Signature                        | Description                          |
|--------------|----------------------------------|--------------------------------------|
| `length`     | `(array\|string\|dict) → number` | Number of elements                   |
| `push`       | `(array, value) → array`         | New array with value appended        |
| `pop`        | `(array) → value`                | Last element (error if empty)        |
| `reverse`    | `(array) → array`                | New reversed array                   |
| `slice`      | `(array, start, end) → array`    | Sub-array `[start, end)`             |
| `range`      | `(start, end) → array`           | Integer range `[start, end)`         |
| `input`      | `(prompt?) → string`             | Read line from stdin                 |
| `read_file`  | `(path) → string`                | Read file contents                   |
| `write_file` | `(path, content) → none`         | Write string to file                 |
| `keys`       | `(dict) → array`                 | Sorted list of keys                  |
| `values`     | `(dict) → array`                 | Values in key-sorted order           |
| `has_key`    | `(dict, key) → bool`             | Check if key exists                  |

---

## Source Files

| File              | Role                                        |
|-------------------|---------------------------------------------|
| `src/value.rs`    | `Value` enum — all runtime types            |
| `src/opcode.rs`   | `OpCode` enum and byte values               |
| `src/chunk.rs`    | `Chunk` struct and disassembler             |
| `src/compiler.rs` | AST → bytecode compiler                     |
| `src/vm.rs`       | VM execution loop and built-in functions    |
| `src/error.rs`    | `WhispemError`, `ErrorKind`, `Span`         |

Unchanged from v1.5.0: `src/lexer.rs`, `src/parser.rs`, `src/ast.rs`, `src/token.rs`.  
Updated for v2.5.0: `src/main.rs`, `src/repl.rs`, `src/compiler.rs`, `src/vm.rs`, `src/chunk.rs`, `src/opcode.rs`, `src/error.rs`.

---

**Whispem VM — v2.5.0**  
*Simple. Explicit. Bootstrappable.*