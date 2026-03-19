# Whispem VM — Specification

**Version 5.0.0**

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
10. [Closures and Upvalues](#closures-and-upvalues)
11. [Error Handling](#error-handling)
12. [Compilation: AST → Bytecode](#compilation-ast--bytecode)
13. [Example: Annotated Bytecode](#example-annotated-bytecode)
14. [Built-in Functions](#built-in-functions)
15. [Source Files](#source-files)

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
│                      Whispem v5.0.0                     │
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

- **Compiler** — AST → one `Chunk` per function/lambda + one for `<main>`
- **Chunk** — flat byte array + constants pool + per-byte line numbers + `param_count` + `upvalue_count`
- **VM** — reads one opcode at a time, executes it
- **Stack** — shared `Vec<Value>` across all frames
- **Call frames** — one `CallFrame` per active call, each with its own `ip`, `locals`, `upvalues`, and `open_upvalues`
- **Globals** — `HashMap<String, Value>` for top-level variables
- **Upvalue cells** — `Rc<RefCell<Upvalue>>` heap-allocated cells shared between frames for mutable closure state

---

## Data Types

| Type       | Rust representation              | Notes                          |
|------------|----------------------------------|--------------------------------|
| `number`   | `f64`                            | All numbers are floating point |
| `string`   | `String`                         | UTF-8                          |
| `bool`     | `bool`                           |                                |
| `array`    | `Vec<Value>`                     | Ordered, mixed types           |
| `dict`     | `HashMap<String, Value>`         | Keys are always strings        |
| `function` | `Closure { chunk, upvalues }`    | First-class function value     |
| `none`     | `Value::None`                    | Returned by void functions     |

`type_of` returns `"function"` for both named functions called as closures and anonymous lambdas.

---

## Instruction Set

**Total: 38 opcodes.** One byte per opcode, optional operand bytes follow.

### Notation

```
OPCODE              — no operands
OPCODE <u8>         — one-byte operand
OPCODE <u16>        — two-byte operand, big-endian
OPCODE <u8> <u8>    — two separate one-byte operands
```

### Complete Opcode Table

| Code   | Name                  | Operands           | Stack effect               | Description                                              |
|--------|-----------------------|--------------------|----------------------------|----------------------------------------------------------|
| `0x00` | `PUSH_CONST`          | `<u8>`             | `( -- value )`             | Push constant at pool index                              |
| `0x01` | `PUSH_TRUE`           | —                  | `( -- true )`              |                                                          |
| `0x02` | `PUSH_FALSE`          | —                  | `( -- false )`             |                                                          |
| `0x03` | `PUSH_NONE`           | —                  | `( -- none )`              |                                                          |
| `0x10` | `LOAD`                | `<u8>`             | `( -- value )`             | Push value from current frame's locals (or globals)      |
| `0x11` | `STORE`               | `<u8>`             | `( value -- )`             | Pop and store into current scope                         |
| `0x12` | `LOAD_GLOBAL`         | `<u8>`             | `( -- value )`             | Push value from `vm.globals` directly                    |
| `0x13` | `LOAD_UPVALUE`        | `<u8>`             | `( -- value )`             | Push value from the current frame's upvalue list         |
| `0x14` | `STORE_UPVALUE`       | `<u8>`             | `( value -- )`             | Write through shared upvalue cell                        |
| `0x15` | `CLOSE_UPVALUE`       | `<u8>`             | `( -- )`                   | Reserved; no-op in current implementation                |
| `0x20` | `ADD`                 | —                  | `( a b -- a+b )`           | Add numbers or concatenate strings                       |
| `0x21` | `SUB`                 | —                  | `( a b -- a-b )`           |                                                          |
| `0x22` | `MUL`                 | —                  | `( a b -- a*b )`           |                                                          |
| `0x23` | `DIV`                 | —                  | `( a b -- a/b )`           | Error on zero divisor                                    |
| `0x24` | `MOD`                 | —                  | `( a b -- a%b )`           |                                                          |
| `0x25` | `NEG`                 | —                  | `( a -- -a )`              |                                                          |
| `0x30` | `EQ`                  | —                  | `( a b -- bool )`          |                                                          |
| `0x31` | `NEQ`                 | —                  | `( a b -- bool )`          |                                                          |
| `0x32` | `LT`                  | —                  | `( a b -- bool )`          |                                                          |
| `0x33` | `LTE`                 | —                  | `( a b -- bool )`          |                                                          |
| `0x34` | `GT`                  | —                  | `( a b -- bool )`          |                                                          |
| `0x35` | `GTE`                 | —                  | `( a b -- bool )`          |                                                          |
| `0x36` | `NOT`                 | —                  | `( a -- bool )`            |                                                          |
| `0x40` | `JUMP`                | `<u16>`            | `( -- )`                   | Unconditional jump to absolute byte offset               |
| `0x41` | `JUMP_IF_FALSE`       | `<u16>`            | `( cond -- )`              | Pop condition; jump if falsy                             |
| `0x42` | `JUMP_IF_TRUE`        | `<u16>`            | `( cond -- )`              | Pop condition; jump if truthy                            |
| `0x43` | `PEEK_JUMP_IF_FALSE`  | `<u16>`            | `( cond -- cond )`         | Peek; jump if falsy — used by `and`                      |
| `0x44` | `PEEK_JUMP_IF_TRUE`   | `<u16>`            | `( cond -- cond )`         | Peek; jump if truthy — used by `or`                      |
| `0x50` | `CALL`                | `<u8> <u8>`        | `( args.. -- retval )`     | const idx of name + argc                                 |
| `0x51` | `RETURN`              | —                  | `( value -- )`             |                                                          |
| `0x52` | `RETURN_NONE`         | —                  | `( -- )`                   |                                                          |
| `0x53` | `MAKE_CLOSURE`        | variable           | `( -- closure )`           | Create `Value::Closure`; see encoding below              |
| `0x60` | `MAKE_ARRAY`          | `<u8>`             | `( n items -- array )`     |                                                          |
| `0x61` | `MAKE_DICT`           | `<u8>`             | `( n pairs -- dict )`      |                                                          |
| `0x62` | `GET_INDEX`           | —                  | `( obj idx -- value )`     |                                                          |
| `0x63` | `SET_INDEX`           | —                  | `( obj idx val -- obj' )`  | Mutate array/dict; push mutated copy                     |
| `0x70` | `PRINT`               | —                  | `( value -- )`             | Write to the VM output sink                              |
| `0x71` | `POP`                 | —                  | `( value -- )`             |                                                          |
| `0xFF` | `HALT`                | —                  | `( -- )`                   | Stop; pop current frame                                  |

### `MAKE_CLOSURE` encoding

`MAKE_CLOSURE` has a variable-length encoding because each upvalue descriptor
embeds the variable name string inline:

```
0x53                      — MAKE_CLOSURE opcode
<u8>                      — constant-pool index of the chunk name string
<u8>                      — upvalue count (N)
for each of N upvalues:
  <u8>                    — is_local: 1 = local in enclosing frame, 0 = upvalue of enclosing frame
  <u8>                    — name_len
  <name_len bytes>        — UTF-8 variable name (if is_local) or decimal slot index (if not)
```

This encoding is self-contained — it does not depend on any particular chunk's constant pool.

---

## Chunk Format

```rust
pub struct Chunk {
    pub code:          Vec<u8>,
    pub constants:     Vec<Value>,
    pub lines:         Vec<usize>,
    pub name:          String,
    pub param_count:   usize,
    pub upvalue_count: usize,   // v5: number of upvalues this function closes over
}
```

---

## `.whbc` Binary Format

```
Magic:          4 bytes   "WHBC"  (0x57 0x48 0x42 0x43)
Version:        1 byte    0x04 for v5.0.0

fn_count:       u16 big-endian   (number of chunks, ≥ 1)

For each chunk  (index 0 = <main>):
  name_len:     u16 big-endian
  name:         UTF-8 bytes (name_len bytes)
  param_count:  u8
  upvalue_count:u8                          ← NEW in v5
  const_count:  u8                          (0–255)
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

**Version history:**
- `0x03` — v3.0.0 / v4.0.0 (no `upvalue_count` field)
- `0x04` — v5.0.0 (adds `upvalue_count` field per chunk)

Files from different versions are not interchangeable. Recompile from source when upgrading.

---

## Call Frames

```rust
struct CallFrame {
    chunk:         Rc<Chunk>,
    ip:            usize,
    locals:        HashMap<String, Value>,
    upvalues:      Vec<Rc<RefCell<Upvalue>>>,  // captured from enclosing scope
    open_upvalues: HashMap<String, Rc<RefCell<Upvalue>>>, // locals this frame has shared
}
```

**On `CALL name_idx argc`:**
1. Pop `argc` arguments from the stack; reverse them.
2. If `name == "__callee__"` → pop the callee value from the stack and dispatch as closure.
3. If `name` matches a built-in → call directly, push result.
4. If a local/global with this name holds a `Value::Closure` → dispatch as closure.
5. Look up `name` in `vm.functions` (named function table).
6. Check arity: `argc != chunk.param_count` → `ArgumentCount` error.
7. Push a new `CallFrame`.

**On `MAKE_CLOSURE name_idx uv_count [descriptors...]`:**
1. Look up the chunk prototype by name.
2. For each upvalue descriptor:
   - `is_local = true` → find or create a shared `Rc<RefCell<Upvalue>>` cell for the named local in the current frame, recording it in `open_upvalues`.
   - `is_local = false` → re-use the parent upvalue cell at the given slot index.
3. Push `Value::Closure { chunk, upvalues }`.

**On `STORE` when a local is captured:**
`store()` also writes through any `open_upvalue` cell for that name, so all closures that share the cell see the new value immediately.

---

## Execution Model

```
loop:
  byte = frame.chunk.code[frame.ip]; frame.ip += 1
  match opcode(byte):
    LOAD        idx  → push( frame.locals[const_str(idx)] ) or globals fallback
    LOAD_GLOBAL idx  → push( vm.globals[const_str(idx)] )
    LOAD_UPVALUE slot → push( frame.upvalues[slot].borrow().get().clone() )
    STORE       idx  →
      write through open_upvalue cell if one exists for this name
      if in function:  frame.locals[const_str(idx)] = pop()
      else:            vm.globals[const_str(idx)]   = pop()
    STORE_UPVALUE slot → frame.upvalues[slot].borrow_mut().set(pop())
    ...
```

**Truthiness:** `false`, `0`, `""`, `[]`, `{}`, `none` are falsy. Everything else (including closures) is truthy.

---

## Variable Scoping

| Scope    | Storage                                       | Lifetime          |
|----------|-----------------------------------------------|-------------------|
| Global   | `vm.globals: HashMap`                         | Entire program    |
| Local    | `frame.locals: HashMap`                       | One function call |
| Upvalue  | `Rc<RefCell<Upvalue>>` shared heap cell       | Until all closures that reference it are dropped |

`LOAD` reads `frame.locals` first, then `vm.globals` as fallback.
`LOAD_GLOBAL` reads only `vm.globals`.
`LOAD_UPVALUE` reads the upvalue cell at the given slot index.

---

## Closures and Upvalues

### Capture model

Whispem uses **eager capture with shared mutation**:

1. When `MAKE_CLOSURE` executes, each captured local is looked up in the enclosing frame's locals and wrapped in an `Rc<RefCell<Upvalue>>` cell.
2. The cell is stored in the enclosing frame's `open_upvalues` map (keyed by variable name).
3. The same `Rc` is shared with the new closure's upvalue list.
4. When the enclosing frame later `STORE`s to the variable, it also writes through the cell — so the closure sees the new value.
5. When multiple closures capture the same variable, they all share the same `Rc`. Mutations via `STORE_UPVALUE` are immediately visible to all sharers.

### Example

```wsp
fn make_counter() {
    let count = 0
    return fn() {
        let count = count + 1   # LOAD_UPVALUE 0, ADD, STORE_UPVALUE 0
        return count             # LOAD_UPVALUE 0
    }
}
```

The inner lambda captures `count` as upvalue slot 0. Each call reads and writes through the shared cell. Across calls, the cell retains the updated value.

---

## Error Handling

All errors are `WhispemError { kind: ErrorKind, span: Span }`.

```rust
pub struct Span { pub line: usize, pub column: usize }
```

### v5.0.0 new error kinds

| Kind | When |
|------|------|
| `UpvalueError(String)` | Internal upvalue invariant violated (compiler bug) |

### Full error kind table

| Kind | When |
|------|------|
| `InvalidBytecode(String)` | Bad magic, wrong version, truncated `.whbc` |
| `SerializationError(String)` | Constant type not serialisable |
| `UndefinedVariable` | `LOAD` / `LOAD_GLOBAL` of unknown name |
| `UndefinedFunction` | `CALL` of unknown name (not a builtin, not a closure) |
| `ArgumentCount` | Wrong arity |
| `TypeError` | Operation on wrong type |
| `IndexOutOfBounds` | Array index out of range |
| `DivisionByZero` | `DIV` or `MOD` with zero |
| `StackUnderflow` | Compiler bug |
| `AssertionFailed(String)` | `assert()` called with falsy condition |
| `Exit(i64)` | `exit(code)` — propagates to CLI, not printed |
| `UpvalueError(String)` | v5.0.0 — upvalue in invalid state |

`Exit` is caught by the CLI and passed to `process::exit` without printing.

---

## Compilation: AST → Bytecode

### Three-pass compilation

1. **First pass** — collect all top-level `let` names into `global_names`.
2. **Second pass** — compile all named `fn` declarations (enables forward calls).
3. **Third pass** — compile the main body.

### f-strings — zero VM impact

`f"Hello, {name}!"` is desugared by the **parser** into a chain of `Binary::Add` nodes before the compiler sees it. The compiler emits the same `ADD` sequences as hand-written `"Hello, " + name + "!"`. No new opcodes, no new AST nodes visible to the compiler.

### Lambdas — `MAKE_CLOSURE` with empty upvalue list (or captured vars)

```
fn(x) { return x * 2 }
```
compiles to:
```
MAKE_CLOSURE '__lambda_1_0' (0 upvalues)
```
The lambda chunk is stored in `functions` under its generated name.

### `else if` — zero bytecode impact

`else if` is collapsed to `ElseIf` by the lexer; the parser builds nested `Stmt::If` nodes — identical AST to `else { if ... }`. The compiler sees no difference.

### Jump patching

Jumps are emitted with a `0xFFFF` placeholder, then patched once the target offset is known.

### `for` loop desugaring

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

### Closure with shared mutable counter

```wsp
fn make_counter() {
    let count = 0
    return fn() {
        let count = count + 1
        return count
    }
}
```

```
== make_counter ==
0000     2  STORE                0    'count'        ← param_count=0, stores 0
0002     3  PUSH_CONST           0    '0'
0004     3  STORE                0    'count'
0006     4  MAKE_CLOSURE         1    '__lambda_4_0' (1 upvalues)
              [is_local=1 name='count']
0014     4  RETURN
0015     4  RETURN_NONE

== __lambda_4_0 ==
0000     5  LOAD_UPVALUE         0               ← read count from shared cell
0002     5  PUSH_CONST           0    '1'
0004     5  ADD
0005     5  STORE_UPVALUE        0               ← write back through shared cell
0007     6  LOAD_UPVALUE         0
0009     6  RETURN
0010     6  RETURN_NONE
```

### f-string desugaring

```wsp
let name = "Em"
print f"Hello, {name}!"
```
Compiles identically to:
```wsp
let name = "Em"
print "Hello, " + name + "!"
```

---

## Built-in Functions

Built-ins are resolved at `CALL` time before checking user-defined functions or closures.

| Name         | Signature                              | Description                    |
|--------------|----------------------------------------|--------------------------------|
| `length`     | `(array\|string\|dict) → number`       |                                |
| `push`       | `(array, value) → array`               | Returns new array              |
| `pop`        | `(array) → value`                      | Returns last element           |
| `reverse`    | `(array) → array`                      |                                |
| `slice`      | `(array, start, end) → array`          | `[start, end)`                 |
| `range`      | `(start, end) → array`                 | Integer range                  |
| `input`      | `(prompt?) → string`                   |                                |
| `read_file`  | `(path) → string`                      |                                |
| `write_file` | `(path, content) → none`               |                                |
| `keys`       | `(dict) → array`                       | Sorted                         |
| `values`     | `(dict) → array`                       | Sorted by key                  |
| `has_key`    | `(dict, key) → bool`                   |                                |
| `char_at`    | `(string, index) → string`             |                                |
| `substr`     | `(string, start, len) → string`        |                                |
| `ord`        | `(string) → number`                    | Unicode codepoint              |
| `num_to_str` | `(number) → string`                    |                                |
| `str_to_num` | `(string) → number`                    |                                |
| `args`       | `() → array`                           | Script arguments               |
| `num_to_hex` | `(number) → string`                    | IEEE-754 f64 as 16-char hex    |
| `write_hex`  | `(path, hex) → none`                   | Hex string → binary file       |
| `type_of`    | `(value) → string`                     | Runtime type name              |
| `assert`     | `(cond, msg?) → none`                  | Raises on falsy                |
| `exit`       | `(code?) → none`                       | Terminates program             |

---

## Source Files

| File              | Role                                              |
|-------------------|---------------------------------------------------|
| `src/value.rs`    | `Value` enum — includes `Closure`, `Upvalue`      |
| `src/opcode.rs`   | `OpCode` enum — 38 opcodes                        |
| `src/chunk.rs`    | `Chunk` + `serialise` + `deserialise`             |
| `src/compiler.rs` | AST → bytecode — upvalue analysis, closure emit   |
| `src/vm.rs`       | Rust VM loop — closure dispatch, upvalue cells    |
| `src/error.rs`    | `WhispemError`, `ErrorKind`, `Span`               |
| `src/lexer.rs`    | Tokeniser — `else if` collapse, f-string lexing   |
| `src/parser.rs`   | Parser — lambdas, f-string desugaring, `CallExpr` |
| `src/token.rs`    | Token types — `FStr`, `ElseIf`, `Assert`, `TypeOf`, `Exit` |
| `src/ast.rs`      | AST — `Lambda`, `CallExpr`, `FStr`, `FStrPart`    |
| `src/main.rs`     | CLI — `handle_vm_error`, 130 Rust tests · 37 autonomous tests           |
| `vm/wvm.c`        | Standalone C VM — full v5 support, ~1000 lines    |

---

**Whispem VM — v5.0.0**
*Closures. Lambdas. F-strings. Self-hosted. Standalone.*