# Whispem

![Logo Whispem](https://imgur.com/YDjrAKR.png)

[![Version](https://img.shields.io/badge/version-6.0.0-cyan.svg)](https://github.com/whispem/whispem-lang/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Tests](https://img.shields.io/badge/tests-204%20passing-brightgreen.svg)

> *Whisper your intent. The machine listens.*

Whispem is a small, self-hosted programming language. The compiler is written in Whispem, compiles itself, and runs on a standalone C VM — no external dependencies beyond a C compiler. Rust serves as the reference implementation.

**Current version: 6.0.0** — `map` · `filter` · `reduce` · closures · lambdas · f-strings · 153 Rust tests · 51 autonomous tests · zero warnings

---

## Quick start

```bash
cargo build --release
cargo run -- examples/hello.wsp
cargo run -- --compile examples/hello.wsp   # → examples/hello.whbc
cargo run -- --dump examples/hello.wsp
cargo run                                    # REPL
cargo test                                   # 153 Rust tests · 51 autonomous tests
```

---

## What's new in v6.0.0

### `map`, `filter`, `reduce`

Three higher-order builtins that take arrays and closures.

```wsp
# map(array, f) → [f(x) for x in array]
print map([1, 2, 3, 4], fn(x) { return x * 2 })
# [2, 4, 6, 8]

# filter(array, pred) → elements where pred is truthy
print filter([1, 2, 3, 4, 5, 6], fn(n) { return n % 2 == 0 })
# [2, 4, 6]

# reduce(array, f, initial) → fold left
print reduce([1, 2, 3, 4, 5], fn(acc, n) { return acc + n }, 0)
# 15
```

They compose cleanly with each other and with closures:

```wsp
let make_gt = fn(t) { return fn(n) { return n > t } }

let result = reduce(
    map(filter(range(1, 11), make_gt(4)),
        fn(n) { return n * n }),
    fn(acc, n) { return acc + n },
    0)
print result   # 285  (25+36+49+64+81+100 — squares of 5..10)
```

### Lambda naming fix

Nested closures on the same source line previously received duplicate internal names, causing `outer(1)(2)(3)`-style chains to return a closure instead of a value. Fixed by replacing the `functions.len()` counter with a monotone `lambda_count` field in the compiler.

---

## The language

```wsp
# Variables
let name = "Em"
let age  = 26

# F-strings
print f"Hello, {name}! Age: {age}"

# Conditionals
if age >= 18 {
    print "adult"
} else if age >= 13 {
    print "teen"
} else {
    print "child"
}

# For / while loops
for fruit in ["apple", "banana", "cherry"] { print fruit }

let i = 0
while i < 5 { print i\nlet i = i + 1 }

# Functions
fn greet(person) { return f"Hello, {person}!" }
print greet("world")

# Lambdas
let double = fn(x) { return x * 2 }
print double(7)   # 14

# Closures
fn make_adder(n) { return fn(x) { return x + n } }
print make_adder(5)(3)   # 8

# Higher-order functions (v6)
print map([1, 2, 3], fn(x) { return x * x })       # [1, 4, 9]
print filter([1..5], fn(n) { return n % 2 == 1 })   # [1, 3, 5]
print reduce([1,2,3,4,5], fn(a,n){return a+n}, 0)   # 15

# Arrays
let nums = push([1, 2, 3], 4)
print length(nums)   # 4

# Dicts
let person = {"name": "Em", "age": 26}
print person["name"]

# assert and type_of
assert(type_of(double) == "function")
print type_of(42)   # number
```

---

## Architecture

```
source code (.wsp)
    ↓  src/compiler.rs (Rust reference)
bytecode (.whbc)
    ↓  src/vm.rs (Rust reference)
output
```

The VM is a stack machine with **38 opcodes**. `map`, `filter`, and `reduce` are pure builtins — no new opcodes. The `.whbc` format stays at version `0x04`.

**`invoke_closure`** — the mechanism used by `map`/`filter`/`reduce` to call user-supplied closures. Records `target_depth = frames.len()`, pushes the closure frame, then runs `execute_until(target_depth)`. All opcodes are handled by the shared `step()` method, avoiding code duplication.

`--dump` disassembles all chunks:

```
== <main> ==
0000     1  PUSH_CONST           0    '1'
...
```

See [`docs/vm.md`](docs/vm.md) for the complete VM specification.

---

## CLI

```bash
whispem                          # interactive REPL
whispem file.wsp                 # run source file
whispem file.wsp arg1 arg2       # run with script arguments
whispem --dump file.wsp          # disassemble
whispem --compile file.wsp       # compile to file.whbc
whispem file.whbc                # run precompiled bytecode
```

---

## Testing

```bash
cargo test             # 153 Rust tests · 51 autonomous tests
./tests/run_tests.sh   # autonomous tests (requires wvm + compiler/wsc.whbc)
```

153 Rust tests · 51 autonomous tests cover: arithmetic, strings, booleans, comparisons, logic, control flow, functions, recursion, forward calls, arrays, dicts, truthiness, error spans, integration programs, bytecode round-trips, all v4/v5/v6 features — including `map`, `filter`, `reduce`, their composition, error cases, and bytecode round-trips.

---

## Project layout

```
whispem/
├── src/
│   ├── main.rs        entry point · CLI · 153 Rust tests · 51 autonomous tests
│   ├── repl.rs        interactive REPL
│   ├── lexer.rs       tokeniser — else-if collapse, f-string lexing, map/filter/reduce
│   ├── token.rs       token types — Map, Filter, Reduce, FStr, ElseIf, …
│   ├── parser.rs      recursive descent — lambdas, f-string desugaring, builtins
│   ├── ast.rs         AST — Lambda, CallExpr, FStr, FStrPart
│   ├── error.rs       WhispemError · ErrorKind · Span
│   ├── value.rs       runtime values — Closure, Upvalue
│   ├── opcode.rs      38 opcodes
│   ├── chunk.rs       Chunk · serialise · deserialise · disassembler
│   ├── compiler.rs    AST → bytecode — upvalue analysis, lambda_count
│   └── vm.rs          VM loop · builtins · invoke_closure · execute_until · step
├── compiler/
│   └── wsc.wsp        self-hosted compiler v6.0
├── examples/          30+ example programs
├── docs/              vm.md · syntax.md · tutorial.md · examples.md · vision.md
├── vm/
│   └── wvm.c          standalone C VM (v5 — map/filter/reduce not yet in C VM)
├── tests/
│   ├── expected/      baseline outputs for autonomous tests
│   ├── run_tests.sh   autonomous test runner
│   └── test_v5.0.0.wsp
├── Makefile           builds wvm from vm/wvm.c
├── CHANGELOG.md
└── README.md
```

---

## Roadmap

| Version | Goal |
|---------|------|
| [x] 1.5.0 | Tree-walking interpreter, full language, REPL |
| [x] 2.0.0 | Bytecode VM, compiler, `--dump` |
| [x] 2.5.0 | Error spans, arity checking, 72 tests, 0 warnings |
| [x] 3.0.0 | `.whbc` serialisation, self-hosted compiler, C VM, 125 tests |
| [x] 4.0.0 | `else if`, `assert`, `type_of`, `exit`, 147 tests |
| [x] 5.0.0 | Closures, lambdas, f-strings, 130 Rust + 37 autonomous tests |
| [x] 6.0.0 | `map` / `filter` / `reduce`; lambda naming fix; 153 Rust tests · 51 autonomous tests |
| 7.0.0 | `map`/`filter`/`reduce` in C VM; string methods; `none` literal |

---

## Philosophy

Whispem is intentionally small. The goal is a language whose entire implementation can be read and understood in a single sitting.

In v6, the language gained its three canonical higher-order functions. The implementation is clean: no new opcodes, no changes to the bytecode format. `map`, `filter`, and `reduce` are builtins that call closures through `invoke_closure`, a small helper that runs a bounded slice of the dispatch loop.

The lambda naming fix is a good example of the project's philosophy: the bug was one line (`functions.len()` → `lambda_count`), the fix is one field and one increment. Small, auditable, correct.

---

*Whispem v6.0.0 — map · filter · reduce · closures · lambdas · f-strings*