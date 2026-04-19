# Whispem — Vision

**Version 6.0.0 — April 2026**

---

## What Whispem is trying to be

Whispem is a small, honest language. It does not try to be fast, feature-rich, or production-ready. It tries to be *understandable* — a language whose entire implementation can be read in an afternoon, whose every design decision can be explained in a sentence, and whose evolution from interpreter to compiler to self-hosting system teaches more than any textbook.

The name comes from "whisper" — a language that speaks quietly, that does not shout.

---

## Core values

### Simplicity over power

Whispem has seven types, one control flow construct per category, and a single statement form for variable binding. There are no classes, no modules, no pattern matching, no type annotations. What fits in one file stays in one file.

### Transparency over magic

The VM is a simple stack machine. The bytecode format is documented byte by byte. The compiler is a three-pass tree-walk with upvalue analysis. There is no hidden garbage collector — values are reference-counted, cleaned up deterministically.

When you run `--dump` on a program, everything the machine will do is visible in front of you.

### Learning over productivity

Whispem is not trying to replace Python or JavaScript. It is trying to answer: *what is the minimum a language needs to be genuinely interesting and useful?*

The answer, as of v6.0.0: variables, arithmetic, strings, booleans, arrays, dicts, conditionals, loops, functions, recursion, first-class functions, closures, string interpolation, and three higher-order builtins. That is enough to write a word counter, a phonebook, a pipeline that sums squares of even numbers, and its own compiler.

---

## The road so far

### v1.x — Tree-walk interpreter

Whispem began as the simplest thing that could work: an AST interpreter. No bytecode, no compilation step. Every node was evaluated by recursively visiting the tree.

### v2.0.0 — Bytecode compiler

The tree-walk interpreter was replaced by a compiler and a stack-based VM. The AST became a representation to be compiled away, not a thing to be executed.

### v2.5.0 — Correctness

`and` and `or` were updated to short-circuit correctly. 72 automated tests established the correctness contract.

### v3.0.0 — Self-hosting

The `.whbc` binary format, `LOAD_GLOBAL`, and `compiler/wsc.wsp` — a Whispem compiler written in Whispem. The bootstrap was verified: gen1 and gen2 produce bit-identical bytecode.

### v4.0.0 — Polish

Four additions earned, not assumed: `else if`, `assert`, `type_of`, `exit`. Zero VM changes.

### v5.0.0 — First-class functions

Lambdas, closures, and f-strings. The upvalue machinery is ~150 lines of Rust. The inline-name encoding for `MAKE_CLOSURE` makes descriptors self-contained.

### v6.0.0 — Higher-order functions

Three canonical higher-order builtins: `map`, `filter`, `reduce`. No new opcodes. No format changes. The implementation rests on `invoke_closure`, a small helper that runs a bounded slice of the dispatch loop.

This release also fixed a silent bug inherited from v5: nested lambdas on the same source line received duplicate internal names, breaking chains like `outer(1)(2)(3)`. The fix was one field (`lambda_count`) and one increment.

---

## What comes next

### v7.0.0

**`map`/`filter`/`reduce` in the C VM.** The standalone `vm/wvm.c` needs `invoke_closure` equivalents to support these builtins. This requires a small refactor of the C dispatch loop.

**String methods.** `split(string, sep)`, `trim(string)`, `starts_with(string, prefix)` — enough to do useful text processing without verbose loops.

**`none` as a literal.** Currently `none` is the absence of a return value, not a writable literal. Making it writeable (`let x = none`) would simplify optional patterns.

---

## Features that will not arrive

- **Classes or objects** — dicts + closures are enough.
- **Modules or imports** — changes the entire architecture; outside scope.
- **Try/catch** — significant VM change with unclear payoff at this size.
- **Type annotations** — optional annotations add noise without runtime benefit.
- **Pattern matching** — `if / else if` chains cover the same ground with less syntax to learn.

---

## Principles that will not change

- The language fits in one file.
- `--dump` always tells the truth.
- Error messages include line numbers.
- The test suite is the contract.
- Simplicity is earned, not assumed.
- Every new feature asks: *does this make the language easier to understand, or harder?*

---

**Whispem v6.0.0**
*map · filter · reduce · closures · lambdas · f-strings · self-hosted · standalone*