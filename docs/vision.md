# Whispem Vision

**Version 1.5.0**

Whispem is not designed to compete with large general-purpose languages.

It is designed to be **understandable in its entirety**.

---

## Philosophy

Whispem values:

- Clarity over cleverness
- Explicitness over magic
- Calm readability
- Small, intentional feature sets

Every feature must justify its existence.

---

## Why an interpreter?

Whispem is interpreted by design.

This allows:

- Immediate feedback
- Simple execution model
- Easier experimentation
- Full control over language semantics

Performance is not the primary goal.
Understanding is.

---

## Minimalism

Whispem avoids:

- Implicit behavior
- Hidden state
- Complex syntax
- Unnecessary abstractions
- Syntactic sugar without purpose

If something can be explained simply, it should be.

---

## Growth Model

Whispem grew in layers:

1. Expressions
2. Variables
3. Control flow (if/else)
4. Loops (while)
5. Logic (and/or/not)
6. Functions
7. Collections (arrays)
8. Advanced loops (for, break, continue)
9. I/O (input, files)
10. Quality (error messages)
11. Dictionaries
12. Modulo operator
13. REPL
14. Proper error system (Result, line/column)

Each layer remained stable before the next was added.

---

## Design Principles

### 1. Readability First

Code should read like intent:

```wsp
fn count_words(words) {
    let counts = {}
    for word in words {
        if has_key(counts, word) {
            let current = counts[word]
            counts[word] = current + 1
        } else {
            counts[word] = 1
        }
    }
    return counts
}
```

### 2. No Surprises

What you see is what you get:

- No operator overloading (except `+` for strings)
- No implicit conversions
- No hidden mutations
- Explicit, located error messages

### 3. Small Surface Area

The entire language fits in your head:

- 14 keywords
- 12 built-in functions
- 12 operators
- 5 data types

### 4. Teachable

Someone new to programming can write their first program in 5 minutes, understand the full language in a weekend, and read the entire implementation in an afternoon.

---

## What Whispem Is Not

Whispem is **not**:

- A systems programming language
- Performance-focused
- Trying to replace Python, JavaScript, or Rust
- Aiming for maximum expressiveness

Whispem **is**:

- A teaching tool
- An exploration of minimalism
- A language you can fully understand
- Production-ready for scripting and learning

---

## Post-1.5.0 Roadmap

### v2.0.0 — Bytecode VM

The real architectural leap. Instead of walking the AST directly, compile to bytecode and run on a small VM. This is how Python, Lua, and Ruby work under the hood.

```
Source → Lexer → Parser → AST → Compiler → Bytecode → VM
```

Benefits: real performance, proper stack traces, debugger possible.

### v2.5.0 — Optional Typing

Not full static types — optional annotations like TypeScript:

```wsp
fn add(a: number, b: number) -> number {
    return a + b
}
```

Ignored if absent, checked if present. Opens the door to a real LSP and VS Code extension.

### v3.0.0 — Self-hosting

The lexer and parser of Whispem written in Whispem. The symbolic milestone that defines a "real" language.

**Version:** 1.5.0  
**Status:** Complete  
**Philosophy:** Whisper, don't shout.

**Whispem — Simple. Clear. Complete.**