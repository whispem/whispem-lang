# Whispem VM — Specification

**Version 3.0.0**

> *"A virtual machine should be as simple as the language it runs."*

This document is the complete specification of the Whispem Virtual Machine (WVM).  
It is intentionally written to be readable by a human — and by a Whispem program.

---

## Table of Contents

1. [Philosophy](#philosophy)
2. [Architecture Overview](#architecture-overview)
3. [Data Types](#data-types)
4. [Instruction Set](#instruction-set)
5. [Chunk Format](#chunk-format)
6. [`.whbc` Binary Format](#whbc-binary-format)
7. [Call Frames](#call-frames)
8. [Execution Model](#execution-model)
9. [Variable Scoping](#variable-scoping)
10. [Error Handling](#error-handling)
11. [Compilation: AST → Bytecode](#compilation-ast--bytecode)
12. [Example: Annotated Bytecode](#example-annotated-bytecode)
13. [Built-in Functions](#built-in-functions)
14. [Source Files](#source-files)

---

## Philosophy

The Whispem VM (WVM) follows the same principles as the language itself:

- **Small** — the entire instruction set fits on one page
- **Explicit** — every opcode does exactly one thing
- **Readable** — bytecode can be disassembled into human-readable form
- **Bootstrappable** — simple enough that a Whispem program can target it

The WVM is a **stack-based virtual machine**.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                      Whispem v3.0.0                     │
│                                                         │
│  source.wsp ──► Compiler ──► Chunks ──► VM ──► output  │
│                                │                        │
│                                ▼                        │
│                           serialise()                   │
│                                │                        │
│                                ▼                        │
│                          source.whbc                    │
│                                │                        │
│                          deserialise()                  │
│                                │                        │
│                                ▼                        │
│                             VM ──► output               │
└─────────────────────────────────────────────────────────┘
```

**Key components:**

- **Compiler** — AST → one `Chunk` per function + one for `<main>`
- **Chunk** — flat byte array + constants pool + per-byte line numbers + `param_count`
- **VM** — reads one opcode at a time, executes it
- **Stack** — shared `Vec<Value>` across all frames
- **Call frames** — one `CallFrame` per active call, each with its own `ip` and `locals`
- **Globals** — `HashMap<String, Value>` for top-level variables
- **Output** — `Box<dyn Write + Send>`; stdout by default, injectable for tests

---

## Data Types

| Type     | Rust representation         | Notes                          |
|----------|-----------------------------|--------------------------------|
| `number` | `f64`                       | All numbers are floating point |
| `string` | `String`                    | UTF-8                          |
| `bool`   | `bool`                      |                                |
| `array`  | `Vec<Value>`                | Ordered, mixed types           |
| `dict`   | `HashMap<String, Value>`    | Keys are always strings        |
| `none`   | `Value::None`               | Returned by void functions     |

---

## Instruction Set

**Total: 34 opcodes.** One byte per opcode, optional operand bytes follow.

### Notation

```
OPCODE              — no operands
OPCODE <u8>         — one-byte operand (0–255)
OPCODE <u16>        — two-byte operand, big-endian (0–65535)
OPCODE <u8> <u8>    — two separate one-byte operands
```

### Complete Opcode Table

| Code   | Name                  | Operands    | Stack effect               | Description                                              |
|--------|-----------------------|-------------|----------------------------|----------------------------------------------------------|
| `0x00` | `PUSH_CONST`          | `<u8>`      | `( -- value )`             | Push constant at pool index                              |
| `0x01` | `PUSH_TRUE`           | —           | `( -- true )`              |                                                          |
| `0x02` | `PUSH_FALSE`          | —           | `( -- false )`             |                                                          |
| `0x03` | `PUSH_NONE`           | —           | `( -- none )`              |                                                          |
| `0x10` | `LOAD`                | `<u8>`      | `( -- value )`             | Push value from current frame's locals                   |
| `0x11` | `STORE`               | `<u8>`      | `( value -- )`             | Pop and store into current scope (local or global)       |
| `0x12` | `LOAD_GLOBAL`         | `<u8>`      | `( -- value )`             | **v3.0.0** — push value from `vm.globals` directly       |
| `0x20` | `ADD`                 | —           | `( a b -- a+b )`           | Add numbers or concatenate strings                       |
| `0x21` | `SUB`                 | —           | `( a b -- a-b )`           |                                                          |
| `0x22` | `MUL`                 | —           | `( a b -- a*b )`           |                                                          |
| `0x23` | `DIV`                 | —           | `( a b -- a/b )`           | Error on zero divisor                                    |
| `0x24` | `MOD`                 | —           | `( a b -- a%b )`           |                                                          |
| `0x25` | `NEG`                 | —           | `( a -- -a )`              |                                                          |
| `0x30` | `EQ`                  | —           | `( a b -- bool )`          |                                                          |
| `0x31` | `NEQ`                 | —           | `( a b -- bool )`          |                                                          |
| `0x32` | `LT`                  | —           | `( a b -- bool )`          |                                                          |
| `0x33` | `LTE`                 | —           | `( a b -- bool )`          |                                                          |
| `0x34` | `GT`                  | —           | `( a b -- bool )`          |                                                          |
| `0x35` | `GTE`                 | —           | `( a b -- bool )`          |                                                          |
| `0x36` | `NOT`                 | —           | `( a -- bool )`            |                                                          |
| `0x40` | `JUMP`                | `<u16>`     | `( -- )`                   | Unconditional jump to absolute byte offset               |
| `0x41` | `JUMP_IF_FALSE`       | `<u16>`     | `( cond -- )`              | **Pop** condition; jump if falsy                         |
| `0x42` | `JUMP_IF_TRUE`        | `<u16>`     | `( cond -- )`              | **Pop** condition; jump if truthy                        |
| `0x43` | `PEEK_JUMP_IF_FALSE`  | `<u16>`     | `( cond -- cond )`         | **Peek** (no pop); jump if falsy — used by `and`         |
| `0x44` | `PEEK_JUMP_IF_TRUE`   | `<u16>`     | `( cond -- cond )`         | **Peek** (no pop); jump if truthy — used by `or`         |
| `0x50` | `CALL`                | `<u8> <u8>` | `( args.. -- retval )`     | const idx of name + argc                                 |
| `0x51` | `RETURN`              | —           | `( value -- )`             |                                                          |
| `0x52` | `RETURN_NONE`         | —           | `( -- )`                   |                                                          |
| `0x60` | `MAKE_ARRAY`          | `<u8>`      | `( n items -- array )`     |                                                          |
| `0x61` | `MAKE_DICT`           | `<u8>`      | `( n pairs -- dict )`      |                                                          |
| `0x62` | `GET_INDEX`           | —           | `( obj idx -- value )`     |                                                          |
| `0x63` | `SET_INDEX`           | —           | `( obj idx val -- obj' )`  | Mutate array/dict; push mutated copy                     |
| `0x70` | `PRINT`               | —           | `( value -- )`             | Write to the VM output sink                              |
| `0x71` | `POP`                 | —           | `( value -- )`             |                                                          |
| `0xFF` | `HALT`                | —           | `( -- )`                   | Stop; pop current frame                                  |

> **v3.0.0:** `LOAD_GLOBAL` (0x12) replaces the v2.0.0 approach of copying `globals` into each new call frame at `CALL` time. Function bodies now contain explicit `LOAD_GLOBAL` instructions for global names. This makes the bytecode self-describing: any reader (including `wsc.wsp`) can tell from the opcode alone whether a read is local or global.

---

## Chunk Format

```rust
pub struct Chunk {
    pub code:        Vec<u8>,      // raw bytecode
    pub constants:   Vec<Value>,   // pool (max 256 entries)
    pub lines:       Vec<usize>,   // one entry per bytecode byte
    pub name:        String,       // "<main>" or function name
    pub param_count: usize,        // 0 for <main>
}
```

The constants pool stores all literal values: numbers, strings, and variable/function names (as `Value::Str`). It is indexed by `u8` — max **256 constants per chunk**. Strings are deduplicated.

---

## `.whbc` Binary Format

```
Magic:        4 bytes   "WHBC"  (0x57 0x48 0x42 0x43)
Version:      1 byte    0x03 for v3.0.0

fn_count:     u16 big-endian   (number of chunks, ≥ 1)

For each chunk  (index 0 = <main>):
  name_len:     u16 big-endian
  name:         UTF-8 bytes (name_len bytes)
  param_count:  u8
  const_count:  u8              (0–255)
  For each constant:
    tag:        u8
      0 = Number  → 8 bytes IEEE-754 big-endian f64
      1 = Bool    → 1 byte (0 = false, 1 = true)
      2 = Str     → u16 length + UTF-8 bytes
      3 = None    → 0 bytes
  code_len:     u32 big-endian
  code:         code_len bytes
  lines_len:    u32 big-endian  (== code_len)
  lines:        lines_len × u32 big-endian  (one per bytecode byte)
```

**Notes:**
- `Array` and `Dict` values never appear in the constants pool.
- Line numbers are preserved in the serialised format so error messages remain accurate after deserialization.
- A future version may add a checksum field after the function table.

---

## Call Frames

```rust
struct CallFrame {
    chunk:  Chunk,
    ip:     usize,
    locals: HashMap<String, Value>,   // parameters + inner lets
}
```

**On `CALL name_idx argc`:**
1. Pop `argc` arguments from the stack (last arg on top); reverse them.
2. If `name` matches a built-in → call directly, push result, done.
3. Look up `name` in `vm.functions`.
4. Check arity: `argc != chunk.param_count` → `ArgumentCount` error.
5. Create a new `CallFrame` with **empty** locals.
6. Push arguments back for the preamble to consume.
7. Push the new frame.

Unlike v2.0.0, no globals are copied into the new frame. Global reads use `LOAD_GLOBAL` which reads `vm.globals` directly.

**Function preamble (compiler-generated):**
```
fn f(a, b, c):  STORE c, STORE b, STORE a   (reverse order)
```

**On `RETURN` / `RETURN_NONE`:**  pop frame, push return value.  
**On `HALT`:** pop frame, return `Ok(())`.

---

## Execution Model

```
loop:
  byte = frame.chunk.code[frame.ip]; frame.ip += 1
  match opcode(byte):
    ...
    LOAD        idx  → push( frame.locals[const_str(idx)] )
    LOAD_GLOBAL idx  → push( vm.globals[const_str(idx)] )
    STORE       idx  →
      if in function:  frame.locals[const_str(idx)] = pop()
      else:            vm.globals[const_str(idx)]   = pop()
    ...
```

**Truthiness:** `false`, `0`, `""`, `[]`, `{}`, `none` are falsy. Everything else is truthy.

---

## Variable Scoping

| Scope  | Storage                        | Lifetime          |
|--------|--------------------------------|-------------------|
| Global | `vm.globals: HashMap`          | Entire program    |
| Local  | `frame.locals: HashMap`        | One function call |

In v3.0.0, `LOAD` reads **only** `frame.locals`; `LOAD_GLOBAL` reads **only** `vm.globals`. The compiler ensures the right opcode is emitted for each variable reference.

Functions **read** globals via `LOAD_GLOBAL`.  
Functions **cannot mutate** globals — `STORE` inside a function writes to `frame.locals`.

---

## Error Handling

All errors are `WhispemError { kind: ErrorKind, span: Span }`.

```rust
pub struct Span { pub line: usize, pub column: usize }
```

### v3.0.0 new error kinds

| Kind | When |
|------|------|
| `InvalidBytecode(String)` | Bad magic, wrong version, truncated `.whbc` |
| `SerializationError(String)` | Name too long, unsupported constant type |

### Existing error kinds (unchanged)

| Kind | When |
|------|------|
| `UndefinedVariable` | `LOAD` / `LOAD_GLOBAL` of unknown name |
| `UndefinedFunction` | `CALL` of unknown name |
| `ArgumentCount` | Wrong arity |
| `TypeError` | Operation on wrong type |
| `IndexOutOfBounds` | Array index out of range |
| `DivisionByZero` | `DIV` or `MOD` with zero |
| `StackUnderflow` | Compiler bug |

---

## Compilation: AST → Bytecode

### Two-pass compilation

1. **First pass** — collect all top-level `let` names into `global_names`. Compile all `fn` declarations into separate chunks.
2. **Second pass** — compile the main body.

Forward calls and `LOAD_GLOBAL` emission both depend on this.

### `LOAD` vs `LOAD_GLOBAL`

Inside a function body, `Expr::Variable("x")` compiles to:
- `LOAD_GLOBAL x` if `x` is in `global_names`
- `LOAD x` otherwise (local variable or function parameter)

### Jump patching

Jumps are emitted with a `0xFFFF` placeholder, then patched once the target offset is known.

### `for` loop desugaring (unchanged from v2.0.0)

```
STORE __iter_N
PUSH_CONST 0; STORE __idx_N
loop_start:
  LOAD __idx_N; LOAD __iter_N; CALL length 1; LT
  JUMP_IF_FALSE [after]
  LOAD __iter_N; LOAD __idx_N; GET_INDEX; STORE x
  <body>
continue_target:
  LOAD __idx_N; PUSH_CONST 1; ADD; STORE __idx_N
  JUMP [loop_start]
after:
```

---

## Example: Annotated Bytecode

### Global read from function (v3.0.0)

```wsp
let greeting = "Hello"

fn say(name) {
    print greeting + ", " + name
}

say("Em")
```

**Chunk `<main>`**
```
0000  1  PUSH_CONST    0    'Hello'
0002  1  STORE         0    'greeting'
0004  7  PUSH_CONST    1    'Em'
0006  7  CALL          2    'say' (1 args)
0009  7  HALT
```

**Chunk `say`** — `param_count: 1`
```
0000  3  STORE         0    'name'       ← preamble
0002  4  LOAD_GLOBAL   1    'greeting'   ← reads vm.globals, not frame.locals
0004  4  PUSH_CONST    2    ', '
0006  4  ADD
0007  4  LOAD          0    'name'       ← reads frame.locals
0009  4  ADD
0010  4  PRINT
0011  4  RETURN_NONE
```

> Before v3.0.0, `say` would have started with `LOAD greeting` and `greeting` would have been copied from globals into `frame.locals` at call time. Now the frame starts empty and `LOAD_GLOBAL` reads globals directly.

---

## Built-in Functions

Built-ins are resolved at `CALL` time before checking user-defined functions.

| Name         | Signature                        | Description                    |
|--------------|----------------------------------|--------------------------------|
| `length`     | `(array\|string\|dict) → number` |                                |
| `push`       | `(array, value) → array`         | Returns new array              |
| `pop`        | `(array) → value`                | Returns last element           |
| `reverse`    | `(array) → array`                |                                |
| `slice`      | `(array, start, end) → array`    | `[start, end)`                 |
| `range`      | `(start, end) → array`           | Integer range                  |
| `input`      | `(prompt?) → string`             |                                |
| `read_file`  | `(path) → string`                |                                |
| `write_file` | `(path, content) → none`         |                                |
| `keys`       | `(dict) → array`                 | Sorted                         |
| `values`     | `(dict) → array`                 | Sorted by key                  |
| `has_key`    | `(dict, key) → bool`             |                                |

---

## Source Files

| File              | Role                                     |
|-------------------|------------------------------------------|
| `compiler/wsc.wsp`| Self-hosted compiler (1618 lines of Whispem) |
| `vm/wvm.c`        | Standalone C VM — `--dump`, REPL, ~2000 lines |
| `src/value.rs`    | `Value` enum                             |
| `src/opcode.rs`   | `OpCode` enum — 34 opcodes               |
| `src/chunk.rs`    | `Chunk` + `serialise` + `deserialise`    |
| `src/compiler.rs` | AST → bytecode, `LOAD_GLOBAL` emission   |
| `src/vm.rs`       | Rust VM loop, `LOAD_GLOBAL` execution    |
| `src/error.rs`    | `WhispemError`, `ErrorKind`, `Span`      |
| `src/main.rs`     | CLI: `--compile`, `.whbc` run, 93 Rust tests |

Unchanged: `src/lexer.rs`, `src/parser.rs`, `src/ast.rs`, `src/token.rs`, `src/repl.rs`.

---

**Whispem VM — v3.0.0**  
*Self-hosted. Standalone. Bootstrappable.*