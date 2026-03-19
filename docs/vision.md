# Whispem — Vision

**Version 5.0.0 — March 2026**

---

## What Whispem is trying to be

Whispem is a small, honest language. It does not try to be fast, feature-rich, or production-ready. It tries to be *understandable* — a language whose entire implementation can be read in an afternoon, whose every design decision can be explained in a sentence, and whose evolution from interpreter to compiler to self-hosting system teaches more than any textbook.

The name comes from "whisper" — a language that speaks quietly, that does not shout.

---

## Core values

### Simplicity over power

Whispem has seven types, one control flow construct per category, and a single statement form for variable binding. There are no classes, no modules, no pattern matching, no type annotations. What fits in one file stays in one file.

Every feature that was considered and rejected — classes, modules, a standard library, algebraic types — was rejected because it would make the language harder to *understand*, not harder to *use*.

### Transparency over magic

The VM is a simple stack machine. The bytecode format is documented byte by byte. The compiler is a three-pass tree-walk with upvalue analysis. There is no hidden garbage collector — values are reference-counted, cleaned up deterministically, and never by surprise.

When you run `--dump` on a program, everything the machine will do is visible in front of you. That is not an accident.

### Learning over productivity

Whispem is not trying to replace Python or JavaScript. It is trying to answer: *what is the minimum a language needs to be genuinely interesting and useful?*

The answer, as of v5.0.0: variables, arithmetic, strings, booleans, arrays, dicts, conditionals, loops, functions, recursion, first-class functions, closures, and string interpolation. That is enough to write a word counter, a phonebook, a FizzBuzz, a factorial, a higher-order `map`, and — as of v3.0.0 — its own compiler.

---

## The road so far

### v1.0.0 — Tree-walk interpreter

Whispem began as the simplest thing that could work: an AST interpreter. No bytecode, no compilation step. Every node was evaluated by recursively visiting the tree.

It worked. It was slow. It was instructive.

### v2.0.0 — Bytecode compiler

The tree-walk interpreter was replaced by a compiler and a stack-based VM. The AST became a representation to be compiled away, not a thing to be executed.

This was the first real architectural decision: separate concerns. The compiler's job is to understand structure. The VM's job is to execute instructions. They do not mix.

### v2.5.0 — Short-circuit logic and correctness

`and` and `or` were updated to short-circuit correctly. 72 automated tests established the correctness contract.

### v3.0.0 — Self-hosting

Three things arrived together: the `.whbc` binary format, `LOAD_GLOBAL` opcode, and `compiler/wsc.wsp` — a Whispem compiler written in Whispem. The bootstrap was verified: gen1 and gen2 produce bit-identical bytecode. The standalone C VM completed the chain.

### v4.0.0 — Polish

Four additions, each small and earned: `else if`, `assert`, `type_of`, `exit`. Pure syntax sugar and builtins — zero VM changes.

### v5.0.0 — First-class functions

Three additions that change what programs can express:

**Lambdas.** `fn(params) { body }` as an expression. A language without first-class functions is half a language. The implementation cost was low — lambdas compile to `MAKE_CLOSURE` with zero upvalues, same path as any closure.

**Closures.** Functions that capture variables from their enclosing scope, with shared mutable state across closures created in the same scope. The upvalue machinery — inline name descriptors in `MAKE_CLOSURE`, `Rc<RefCell<Upvalue>>` cells in the VM — is ~150 lines of Rust and ~100 lines of C. Readable, auditable, understandable.

**F-strings.** `f"Hello, {name}!"` with arbitrary expression interpolation. Implementation cost: a few dozen lines in the lexer and parser. Runtime cost: zero — f-strings are desugared to `+` chains before the compiler sees them.

The combination enables real functional programming patterns in Whispem — `map`, `filter`, event handlers, factories — without adding any heavyweight machinery.

---

## What comes next

### v6.0.0

**Self-hosted compiler and C VM catch up.** The `compiler/wsc.wsp` self-hosted compiler and `vm/wvm.c` need f-string lexing, lambda parsing, and upvalue analysis. The self-hosted compiler is the most interesting engineering challenge remaining: implementing upvalue resolution in Whispem itself.

**`map` / `filter` / `reduce` builtins.** With closures available, these are trivial to implement and make programs significantly shorter.

---

## Features that will not arrive

- **Classes or objects** — dicts + closures are enough; adding inheritance would obscure more than it reveals.
- **Modules or imports** — changes the entire architecture; outside the scope.
- **Try/catch** — interesting, but a significant VM change with unclear payoff for a language this size.
- **Type annotations** — optional annotations would add noise without runtime benefit.
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

**Whispem v5.0.0**
*Closures. Lambdas. F-strings. Self-hosted. Standalone.*