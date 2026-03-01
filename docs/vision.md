# Whispem Vision

**Version 2.5.0**

Whispem is not designed to compete with large general-purpose languages.

It is designed to be **understandable in its entirety** — including its own implementation.

---

## Philosophy

> *Code should whisper intent, not shout complexity.*

Whispem values:

- Clarity over cleverness
- Explicitness over magic
- Small, intentional feature sets
- Calm readability

Every feature must justify its existence. Every design decision asks: *would a future Whispem program be able to do this too?*

---

## Why a bytecode VM?

Whispem v2.0.0 replaced the tree-walking interpreter with a bytecode compiler and a stack-based virtual machine — the same model used by Python, Lua, and Ruby under the hood.

The benefits:

- **Separation of concerns** — the compiler and the runtime are independent
- **Inspectability** — `--dump` shows compiled bytecode in human-readable form
- **Bootstrappability** — the VM is simple enough that a Whispem program could eventually target it

The VM has 33 opcodes. The entire instruction set fits on one page. See [`docs/vm.md`](vm.md) for the complete specification.

---

## Minimalism

Whispem avoids:

- Implicit behavior
- Hidden state
- Complex syntax
- Unnecessary abstractions
- Features that exist just because other languages have them

The entire language fits in your head:

- **14 keywords**
- **12 built-in functions**
- **5 data types**
- **33 VM opcodes**

---

## How Whispem grew

Each version added one layer. Each layer stayed stable before the next was added.

| Version | What was added |
|---------|----------------|
| 0.1–0.5 | Expressions, variables, control flow |
| 0.6–0.7 | Booleans, loops, logic |
| 0.8 | Functions and recursion |
| 0.9 | Arrays |
| 1.0.0 | `for`, `break`, `continue`, file I/O, complete error messages |
| 1.5.0 | Dictionaries, modulo, interactive REPL, error overhaul |
| 2.0.0 | Bytecode compiler, stack VM, `--dump`, `docs/vm.md` |
| **2.5.0** | **Error spans, arity checking, short-circuit fix, 72-test suite, zero warnings** |

---

## Design principles

### Readability first

Code should read like intent:

```wsp
fn count_words(words) {
    let counts = {}
    for word in words {
        if has_key(counts, word) {
            counts[word] = counts[word] + 1
        } else {
            counts[word] = 1
        }
    }
    return counts
}
```

### No surprises

What you see is what you get:

- No operator overloading (except `+` for string concatenation)
- No implicit type conversions
- No hidden mutations
- Arity errors are caught at runtime with a clear message
- Error messages include line numbers — column precision coming in v3.0.0

### Tested by design

From v2.5.0, every language feature is covered by an automated test. `cargo test` runs 72 tests in-process with zero subprocesses and zero platform-specific code. The test harness injects a `Vec<u8>` writer into the VM — the same technique a future embedding API would use.

### Teachable by design

Someone new to programming can:
- Write their first program in 5 minutes
- Understand the full language in a weekend
- Read the entire implementation in an afternoon
- Inspect compiled bytecode with `--dump`
- Run the full test suite with `cargo test`

---

## Roadmap

### [x] 2.0.0 — Bytecode VM

Compile to bytecode, run on a stack machine. Proper call frames, constants pool, disassembler.

```
Source → Lexer → Parser → AST → Compiler → Bytecode → VM
```

### [x] 2.5.0 — Correctness & testing

- `Span { line, column }` on every error
- Arity checking for user-defined functions
- Fixed short-circuit evaluation (`and`/`or` — `PEEK_JUMP_*` opcodes)
- 72 automated tests, zero warnings, zero unsafe in test infrastructure
- VM output injectable for testing and future embedding

### 3.0.0 — Self-hosting

The Whispem compiler written in Whispem, targeting the WVM. The symbolic milestone that defines a "real" language.

---

**Whispem v2.5.0 — Simple. Explicit. Bootstrappable.**