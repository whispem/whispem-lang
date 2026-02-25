# Changelog

All notable changes to Whispem are documented here.  
Format: [Semantic Versioning](https://semver.org). Dates are in YYYY-MM-DD format.

---

## [2.0.0] — 2026-02-25

**The bytecode VM release.** Whispem now compiles to bytecode and runs on a stack-based virtual machine instead of walking the AST directly. Programs run faster, error messages include more context, and the `--dump` flag lets you inspect compiled bytecode.

### Added

- **`src/value.rs`** — `Value` is now its own module, cleanly separated from the old interpreter
- **`src/opcode.rs`** — `OpCode` enum with 31 opcodes, each mapping to a `u8` via `#[repr(u8)]`
- **`src/chunk.rs`** — `Chunk` struct: bytecode array + constants pool + per-byte line numbers + disassembler
- **`src/compiler.rs`** — single-pass AST → bytecode compiler producing `(main_chunk, HashMap<String, Chunk>)`
- **`src/vm.rs`** — stack-based VM execution loop with call frame management and all built-in functions
- **`--dump` flag** — `whispem --dump file.wsp` prints a human-readable bytecode disassembly and exits
- **`POP` opcode (`0x71`)** — emitted after every bare expression statement to keep the stack clean
- **`HALT` now pops its frame** — prevents frame leaks in the REPL across multiple snippets
- **`docs/vm.md`** — complete VM specification: instruction set, chunk format, call frames, compilation rules, annotated examples

### Changed

- **`src/main.rs`** — updated to use the new `Compiler` + `Vm` pipeline; `interpreter.rs` is no longer invoked
- **`src/repl.rs`** — REPL now compiles and runs each snippet through the VM; function definitions persist across lines
- **`src/error.rs`** — three new `ErrorKind` variants: `StackUnderflow`, `InvalidOpcode(u8)`, `TooManyConstants`

### Fixed

- **`SET_INDEX` now actually mutates** — previously validated but did not persist changes; now performs real in-place mutation and pushes the mutated value back for `STORE` to write
- **Function parameter binding order** — arguments are now pushed and popped in the correct order relative to the preamble's `STORE` sequence
- **REPL frame leak** — `HALT` now pops the `<main>` frame, preventing dead frames from accumulating across REPL snippets
- **Lexer double-match** — `read_ident()` had a redundant first match that produced dead code; collapsed into a single clean match
- **Parser built-in token names** — `to_string().trim_matches('\'')` replaced with explicit per-token string literals

### Removed

- **`src/interpreter.rs`** as the execution backend — the tree-walking interpreter is no longer called at runtime. The file may be kept for reference but is not compiled into the binary.

### Architecture notes

The new pipeline is:

```
source
  → Lexer      (unchanged)
  → Parser     (unchanged)
  → Compiler   (new) → (Chunk, HashMap<String, Chunk>)
  → Vm         (new) → execution
```

Each `fn` declaration compiles to its own `Chunk` in a first pass. The second pass compiles top-level statements into `<main>`. This allows forward calls without any lookahead.

The constants pool is capped at **256 entries per chunk** by design. The compiler deduplicates string constants (variable names, function names) to maximise headroom.

The scoping model is intentionally simple: top-level `let` stores to `globals`; function-local `let` stores to `CallFrame.locals`. On function entry, globals are copied into locals for read access. Since Whispem has no bare assignment statement, functions cannot mutate globals — no closures needed.

---

## [1.5.0] — prior release

Tree-walking interpreter. Full language support including arrays, dicts, for loops, functions, break/continue, file I/O, and the interactive REPL.

---

## Roadmap

| Version | Goal                                                          |
|---------|---------------------------------------------------------------|
| 2.5.0   | Self-hosting preparation: bytecode serialisation, richer error spans, test suite |
| 3.0.0   | Whispem compiler written in Whispem, targeting the WVM        |