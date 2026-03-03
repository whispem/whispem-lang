# Whispem — Vision

**Version 3.0.0 — March 2026**

---

## What Whispem is trying to be

Whispem is a small, honest language. It does not try to be fast, feature-rich, or production-ready. It tries to be *understandable* — a language whose entire implementation can be read in an afternoon, whose every design decision can be explained in a sentence, and whose evolution from interpreter to compiler to self-hosting system teaches more than any textbook.

The name comes from "whisper" — a language that speaks quietly, that does not shout.

---

## Core values

### Simplicity over power

Whispem has five types, one control flow construct per category, and a single statement form for variable binding. There are no classes, no modules, no closures. What fits in one file stays in one file.

Every feature that was considered and rejected — closures, first-class functions, type annotations, a standard library — was rejected because it would make the language harder to *understand*, not harder to *use*. There is a difference.

### Transparency over magic

The VM is a simple stack machine. The bytecode format is documented byte by byte. The compiler is a single-pass tree-walk with a second pass for globals. There is no hidden garbage collector — values are reference-counted, cleaned up deterministically, and never by surprise.

When you run `--dump` on a program, everything the machine will do is visible in front of you. That is not an accident.

### Learning over productivity

Whispem is not trying to replace Python or Lua. It is trying to answer the question: *what is the minimum a language needs to be genuinely interesting?*

The answer, so far: variables, arithmetic, strings, booleans, arrays, dicts, conditionals, loops, functions, and recursion. That is everything Whispem has. That is also enough to write a word counter, a phonebook, a FizzBuzz, a factorial, and — as of v3.0.0 — its own compiler.

---

## The road so far

### v1.0.0 — Tree-walk interpreter

Whispem began as the simplest thing that could work: an AST interpreter. No bytecode, no compilation step. Every node was evaluated by recursively visiting the tree.

It worked. It was slow. It was instructive.

### v2.0.0 — Bytecode compiler

The tree-walk interpreter was replaced by a compiler and a stack-based VM. The AST became a representation to be compiled away, not a thing to be executed.

This was the first real architectural decision: separate concerns. The compiler's job is to understand structure. The VM's job is to execute instructions. They do not mix.

### v2.5.0 — Short-circuit logic

A small release, but a meaningful one. `and` and `or` were updated to short-circuit correctly, using `JUMP_IF_FALSE` and `JUMP_IF_TRUE` with `POP` on the non-taken branch.

The change required understanding jump semantics at the bytecode level. It was worth doing slowly.

### v3.0.0 — Bytecode serialization and self-hosting

Two things arrived together:

**The `.whbc` format.** Compiled bytecode can now be written to a binary file and loaded back without recompilation. The format is versioned, magic-checked, and self-describing. Line numbers survive the round-trip. Constants are deduplicated. The format is documented in `docs/vm.md`.

**The `LOAD_GLOBAL` opcode.** Previously, global variables were copied into each function's call frame at `CALL` time. Now the compiler tracks which names are globals and emits `LOAD_GLOBAL` for them. Functions read `vm.globals` directly. The bytecode is now explicit about scope.

**`compiler/wsc.wsp`.** A Whispem compiler, written in Whispem. It reads a `.wsp` source file, lexes, parses, compiles, and writes a `.whbc` bytecode file — byte-for-byte identical to the Rust compiler’s output. The language can now describe its own compilation and produce executable output.

---

## What comes next

### v3.0.0 — Bootstrap (done)

The self-hosted compiler reads `.wsp` files, writes `.whbc` files, and compiles itself. The bootstrap is verified: gen1 and gen2 produce bit-identical bytecode (SHA-1 fixed point). Rc-based copy-on-write on arrays and dicts made it feasible despite pass-by-value semantics.

**`vm/wvm.c`.** A standalone C runtime that executes `.whbc` bytecode without Rust. Single-file, ~2000 lines, refcounted copy-on-write, same 34 opcodes and 20 builtins. Includes a `--dump` disassembler (byte-identical output to the Rust one) and an interactive REPL. The bootstrap fixed point holds identically on both the Rust VM and the C VM.

**`tests/run_tests.sh`.** An autonomous test suite that uses only `wvm` and `wsc.whbc` — no Rust needed. 32 tests covering every example program plus bootstrap verification. With this, Whispem's correctness can be checked without ever installing `cargo`. With `wsc.whbc` + `wvm`, Whispem runs without any Rust dependency.

### v4.0.0 — Polish

Some features that might arrive, in order of likelihood:

- **`else if`** — syntax sugar for nested `if`/`else`, since the current nesting is verbose
- **Closures** — functions that close over their lexical environment; requires a significant VM change
- **Multiple return values** — currently simulated with arrays; proper support would be cleaner
- **Column numbers in errors** — line numbers are precise; columns are not yet
- **Type annotations** — optional, for documentation only; no runtime enforcement

Some features that will probably never arrive:

- **Classes or objects** — dictionaries are enough; adding inheritance would obscure more than it reveals
- **A standard library** — the built-in functions are the library; adding more would make the language harder to teach
- **Concurrency** — interesting, but outside the scope of a language about clarity

---

## The self-hosting milestone, in perspective

Self-hosting is often treated as a milestone of completeness — the moment when a language is "real." That is not why it matters here.

What matters is that writing `compiler/wsc.wsp` required using every feature Whispem has: arrays to represent instruction streams, dicts to represent chunks, loops to iterate over tokens, functions to separate concerns, recursion to handle nested expressions. If any of those had been missing or broken, the compiler would not have worked.

Self-hosting is a test. It passed.

---

## Principles that will not change

- The language fits in one file (this one, eventually)
- `--dump` always tells the truth
- Error messages include line numbers
- The test suite is the contract
- Simplicity is earned, not assumed

---

**Whispem v3.0.0**  
*Self-hosted. Standalone. Bootstrappable.*