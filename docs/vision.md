# Whispem Vision

**Version 2.0.0**

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

Whispem v2.0.0 compiles source code to bytecode and runs it on a stack-based virtual machine — instead of walking the AST directly.

This is how Python, Lua, and Ruby work under the hood. The benefits:

- **Separation of concerns** — the compiler and the runtime are independent
- **Inspectability** — `--dump` shows the compiled bytecode in human-readable form
- **Bootstrappability** — the VM is simple enough that a Whispem program could eventually target it

The VM has 31 opcodes. The entire instruction set fits on one page. See [`docs/vm.md`](vm.md) for the complete specification.

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
- **31 VM opcodes**

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
| **2.0.0** | **Bytecode compiler, stack VM, `--dump`, `docs/vm.md`** |

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
- Error messages with line and column numbers

### Teachable by design

Someone new to programming can:
- Write their first program in 5 minutes
- Understand the full language in a weekend
- Read the entire implementation in an afternoon
- Inspect compiled bytecode with `--dump`

---

## Roadmap

### [x] 2.0.0 — Bytecode VM

Compile to bytecode, run on a stack machine. Proper call frames, constants pool, disassembler.

```
Source → Lexer → Parser → AST → Compiler → Bytecode → VM
```

### 2.5.0 — Bytecode serialisation + test suite

Write compiled bytecode to disk (`.whbc` files). Re-execute without recompiling. Richer error spans. Automated test suite.

### 3.0.0 — Self-hosting

The Whispem compiler written in Whispem, targeting the WVM. The symbolic milestone that defines a "real" language.

---

**Whispem v2.0.0 — Simple. Explicit. Bootstrappable.**