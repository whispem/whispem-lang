# Whispem VM — Specification

**Version 2.0.0**

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
│  │   Compiler    │────▶│  Chunk (bytecode +       │ │
│  │  (AST→bytes)  │     │    constants pool)        │ │
│  └───────────────┘     └────────────┬─────────────┘ │
│                                     │               │
│                                     ▼               │
│                          ┌──────────────────────────┐│
│                          │       VM Core            ││
│                          │                          ││
│                          │  ip  (per call frame)    ││
│                          │  stack    Vec<Value>      ││
│                          │  frames   Vec<CallFrame>  ││
│                          │  globals  HashMap         ││
│                          │  functions HashMap        ││
│                          └──────────────────────────┘│
└─────────────────────────────────────────────────────┘
```

**Key components:**

- **Compiler** — walks the AST and emits one `Chunk` per function + one for `<main>`
- **Chunk** — flat array of bytes + constants pool + per-byte line numbers
- **VM Core** — a loop that reads one opcode at a time and executes it
- **Stack** — shared across all frames; all intermediate values live here
- **Call frames** — one `CallFrame` per active function call, each with its own `ip` and `locals`
- **Globals** — `HashMap<String, Value>` for top-level variables, copied into functions on call

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

| Code   | Name            | Operands    | Stack effect               | Description                                      |
|--------|-----------------|-------------|----------------------------|--------------------------------------------------|
| `0x00` | `PUSH_CONST`    | `<u8>`      | `( -- value )`             | Push constant at pool index                      |
| `0x01` | `PUSH_TRUE`     | —           | `( -- true )`              | Push boolean `true`                              |
| `0x02` | `PUSH_FALSE`    | —           | `( -- false )`             | Push boolean `false`                             |
| `0x03` | `PUSH_NONE`     | —           | `( -- none )`              | Push `none`                                      |
| `0x10` | `LOAD`          | `<u8>`      | `( -- value )`             | Push value of variable (name at const index)     |
| `0x11` | `STORE`         | `<u8>`      | `( value -- )`             | Pop and store into variable (name at const index)|
| `0x20` | `ADD`           | —           | `( a b -- a+b )`           | Add numbers or concatenate strings               |
| `0x21` | `SUB`           | —           | `( a b -- a-b )`           | Subtract                                         |
| `0x22` | `MUL`           | —           | `( a b -- a*b )`           | Multiply                                         |
| `0x23` | `DIV`           | —           | `( a b -- a/b )`           | Divide                                           |
| `0x24` | `MOD`           | —           | `( a b -- a%b )`           | Modulo                                           |
| `0x25` | `NEG`           | —           | `( a -- -a )`              | Negate number                                    |
| `0x30` | `EQ`            | —           | `( a b -- bool )`          | Equal                                            |
| `0x31` | `NEQ`           | —           | `( a b -- bool )`          | Not equal                                        |
| `0x32` | `LT`            | —           | `( a b -- bool )`          | Less than                                        |
| `0x33` | `LTE`           | —           | `( a b -- bool )`          | Less than or equal                               |
| `0x34` | `GT`            | —           | `( a b -- bool )`          | Greater than                                     |
| `0x35` | `GTE`           | —           | `( a b -- bool )`          | Greater than or equal                            |
| `0x36` | `NOT`           | —           | `( a -- bool )`            | Logical not                                      |
| `0x40` | `JUMP`          | `<u16>`     | `( -- )`                   | Unconditional jump to absolute byte offset       |
| `0x41` | `JUMP_IF_FALSE` | `<u16>`     | `( cond -- )`              | Pop condition; jump if falsy                     |
| `0x42` | `JUMP_IF_TRUE`  | `<u16>`     | `( cond -- )`              | Pop condition; jump if truthy (used by `or`)     |
| `0x50` | `CALL`          | `<u8> <u8>` | `( args.. -- retval )`     | Call function: const idx of name + argc          |
| `0x51` | `RETURN`        | —           | `( value -- )`             | Return value from current function               |
| `0x52` | `RETURN_NONE`   | —           | `( -- )`                   | Return `none` (implicit at function end)         |
| `0x60` | `MAKE_ARRAY`    | `<u8>`      | `( n items -- array )`     | Pop n items; build array in source order         |
| `0x61` | `MAKE_DICT`     | `<u8>`      | `( n pairs -- dict )`      | Pop n key-value pairs; build dict                |
| `0x62` | `GET_INDEX`     | —           | `( obj idx -- value )`     | Index into array or dict                         |
| `0x63` | `SET_INDEX`     | —           | `( obj idx val -- obj' )`  | Mutate array/dict; push mutated copy             |
| `0x70` | `PRINT`         | —           | `( value -- )`             | Print top of stack to stdout                     |
| `0x71` | `POP`           | —           | `( value -- )`             | Discard top of stack                             |
| `0xFF` | `HALT`          | —           | `( -- )`                   | Stop execution; pop the current frame            |

**Total: 31 opcodes.**

---

## Chunk Format

```rust
pub struct Chunk {
    pub code:      Vec<u8>,        // raw bytecode
    pub constants: Vec<Value>,     // constants pool (max 256 entries)
    pub lines:     Vec<usize>,     // one line number per byte in code
    pub name:      String,         // "<main>" or the function name
}
```

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
4. Create a new `CallFrame`; copy globals into its locals (read access)
5. Push arguments back in forward order for the preamble to consume
6. Push the new frame — execution continues inside the function

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

    match byte → opcode:

        PUSH_CONST idx  →  push(constants[idx])
        PUSH_TRUE       →  push(true)
        PUSH_FALSE      →  push(false)
        PUSH_NONE       →  push(none)

        LOAD idx        →  push(lookup(constants[idx]))   // locals → globals
        STORE idx       →  store(constants[idx], pop())   // locals if fn, else globals

        ADD             →  b,a = pop2(); push(a+b)        // numbers or string concat
        SUB/MUL/DIV/MOD →  b,a = pop2(); push(a op b)
        NEG             →  push(-pop())
        NOT             →  push(!truthy(pop()))
        EQ/NEQ/LT/…     →  b,a = pop2(); push(a cmp b)

        JUMP target         →  ip = target
        JUMP_IF_FALSE target →  if !truthy(pop()): ip = target
        JUMP_IF_TRUE target  →  if  truthy(pop()): ip = target

        CALL name_idx argc  →  (see Call Frames above)
        RETURN              →  val = pop(); pop_frame(); push(val)
        RETURN_NONE         →  pop_frame(); push(none)

        MAKE_ARRAY n    →  collect n pops (reversed); push(array)
        MAKE_DICT n     →  collect n pairs (reversed); push(dict)
        GET_INDEX       →  idx,obj = pop2(); push(obj[idx])
        SET_INDEX       →  val,idx,obj = pop3(); push(mutate(obj,idx,val))

        PRINT           →  println(pop())
        POP             →  pop()    // discard (after bare expression statements)
        HALT            →  pop_frame(); return Ok(())
```

### Truthiness

A value is **falsy** if it is: `false`, `0`, `""`, `[]`, `{}`, or `none`.  
Everything else is truthy.

---

## Variable Scoping

Whispem uses a two-level scope model:

- **`<main>` frame** (`frames.len() == 1`) → `STORE` writes to `globals`
- **Function frame** (`frames.len() > 1`) → `STORE` writes to `CallFrame.locals`

On function entry, globals are **copied into** the new frame's locals, giving read access to top-level variables without requiring closures.

Because Whispem has no bare assignment statement (only `let x = expr` and `x[i] = expr`), a function cannot mutate a global. This keeps the scoping model simple and predictable.

**Lookup order inside a function:** locals → globals → `UndefinedVariable` error.

---

## Error Handling

The VM reuses the `WhispemError` / `WhispemResult<T>` system from `src/error.rs`.

Each byte in `chunk.code` has a corresponding entry in `chunk.lines`.  
When a runtime error occurs, the VM reads `chunk.lines[ip - 1]` for the source line.

VM-specific error kinds:

```rust
ErrorKind::StackUnderflow      // pop on empty stack (indicates a compiler bug)
ErrorKind::InvalidOpcode(u8)   // unrecognised opcode byte
ErrorKind::TooManyConstants    // constants pool exceeded 256 entries
```

---

## Compilation: AST → Bytecode

```rust
// compiler.rs
pub fn compile(self, program: Vec<Stmt>) -> WhispemResult<(Chunk, HashMap<String, Chunk>)>
```

The compiler does two passes over the program:

1. **First pass** — compile all `fn` declarations into their own `Chunk`s
2. **Second pass** — compile all other top-level statements into `<main>`

This allows forward calls (calling a function before its definition in the source).

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

**`fn name(params) { body }`** — compiled to a separate chunk:
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

**`a and b`** (short-circuit)
```
<compile a>
JUMP_IF_FALSE [done]    ← falsy a → result is a, skip b
<compile b>
done:
```

**`a or b`** (short-circuit)
```
<compile a>
JUMP_IF_TRUE [done]     ← truthy a → result is a, skip b
<compile b>
done:
```

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

**Chunk `double`** — constants: `["n", 2.0]`
```
0000  1    STORE         0    'n'       ← preamble
0002  2    LOAD          0    'n'
0004  2    PUSH_CONST    1    '2'
0006  2    MUL
0007  2    RETURN
0008  2    RETURN_NONE                  ← implicit fallback
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
     LOAD  'arr'     ← load current array clone
     PUSH_CONST '1'  ← index
     PUSH_CONST '99' ← new value
     SET_INDEX        ← mutates clone; pushes result
     STORE 'arr'     ← write mutated array back
     LOAD  'arr'
     PRINT
     HALT
```

---

## Built-in Functions

Built-ins are resolved at `CALL` time before checking user-defined functions.

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

Unchanged from v1.5.0: `src/lexer.rs`, `src/parser.rs`, `src/ast.rs`, `src/error.rs`, `src/token.rs`.  
Updated for v2.0.0: `src/main.rs`, `src/repl.rs`.

---

**Whispem VM — v2.0.0**  
*Simple. Explicit. Bootstrappable.*
