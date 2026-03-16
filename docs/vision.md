# Whispem — Vision

**Version 4.0.0 — March 2026**

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

### v2.5.0 — Short-circuit logic and correctness

`and` and `or` were updated to short-circuit correctly using `JUMP_IF_FALSE` / `JUMP_IF_TRUE` with `POP` on the non-taken branch. 72 automated tests established the correctness contract.

### v3.0.0 — Self-hosting

Three things arrived together.

**The `.whbc` format.** Compiled bytecode can be written to a binary file and loaded back without recompilation. The format is versioned, magic-checked, and self-describing. Line numbers survive the round-trip.

**`LOAD_GLOBAL`.** Functions now contain explicit `LOAD_GLOBAL` instructions for global names. The bytecode is self-describing: any reader can tell from the opcode alone whether a read is local or global.

**`compiler/wsc.wsp`.** A Whispem compiler written in Whispem. It reads a `.wsp` source file, lexes, parses, compiles, and writes a `.whbc` bytecode file. The bootstrap is verified: gen1 and gen2 produce bit-identical bytecode (SHA-1 fixed point).

**`vm/wvm.c`.** A standalone C runtime that executes `.whbc` bytecode without Rust. The bootstrap fixed point holds on both VMs.

### v4.0.0 — Polish

Four additions, each small and earned.

**`else if`.** The most visible daily friction in writing Whispem was the nesting required for multiple branches. `else if` eliminates it without touching the VM, the compiler, or the AST — it is purely a lexer and parser transformation. The bytecode emitted is identical to the nested form.

**`assert`.** Writing correct programs means catching wrong assumptions early. `assert(condition, message)` gives that without adding a new statement form — it is a builtin that raises a runtime error on failure. No new opcodes, no new AST nodes.

**`type_of`.** Defensive code in Whispem was previously blind to types. `type_of(v)` returns a string; `if type_of(x) != "number"` is now a real pattern. Simple, transparent, no reflection machinery.

**`exit`.** Scripts sometimes need to terminate with a specific exit code. `exit(code)` propagates through the call stack as a special error kind, caught by the CLI and passed to the OS. The REPL handles it by terminating cleanly.

---

## What comes next

### v5.0.0

**Closures** are the largest remaining architectural change. They require a third variable kind (`upvalue`) alongside locals and globals, two new opcodes (`LOAD_UPVALUE`, `STORE_UPVALUE`), a `Value::Function` type that carries a captured environment, and an analysis pass in the compiler. The self-hosted compiler would also need updating.

The potential is real — closures enable callbacks, iterators, and higher-order patterns that currently require working around. The cost is a significant increase in VM and compiler complexity.

**String interpolation** (`"Hello, {name}!"`) is mostly a lexer and parser change. The value is real — string concatenation chains are verbose. The implementation is straightforward compared to closures.

---

## Features that will not arrive

- **Classes or objects** — dictionaries are enough; adding inheritance would obscure more than it reveals.
- **Modules or imports** — changes the entire architecture; outside the scope.
- **Try/catch** — interesting, but a significant VM change with unclear payoff for a language this size.
- **Type annotations** — optional annotations would add noise without runtime benefit.
- **A standard library** — the built-in functions are the library.

---

## Principles that will not change

- The language fits in one file.
- `--dump` always tells the truth.
- Error messages include line numbers.
- The test suite is the contract.
- Simplicity is earned, not assumed.

---

**Whispem v4.0.0**
*Self-hosted. Standalone. Bootstrappable.*