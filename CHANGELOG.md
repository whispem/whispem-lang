# Changelog

All notable changes to Whispem are documented here.  
Format: [Semantic Versioning](https://semver.org). Dates are in YYYY-MM-DD format.

---

## [2.5.0] — 2026-03-01

**The quality release.** Zero warnings, richer error spans, and a full automated test suite.

### Added

- **`error::Span`** — new `Span { line, column }` type replacing bare `(usize, usize)` pairs throughout the codebase. Every `WhispemError` now carries a `Span` with a human-readable `Display` impl (`[line N, col M]`). The `WhispemError::at(kind, line, col)` convenience constructor replaces `WhispemError::new(kind, line, col)`.
- **`WhispemError::at(kind, line, col)`** — convenience constructor for the common case; `::new(kind, span)` remains for when a `Span` is already in hand.
- **`Compiler` now propagates source line to every opcode** — the `compile_expr` method receives a `ctx_line` argument, eliminating the `line: 0` that appeared on `POP` and expression-level opcodes in v2.0.0. `--dump` output is now accurate for every instruction.
- **Automated test suite (`cargo test`)** — 60+ tests in `src/main.rs` covering:
  - All arithmetic operators and precedence rules
  - All comparison and logical operators (including short-circuit)
  - String operations and escape sequences
  - All control flow: `if/else`, `while`, `for`, `break`, `continue`
  - Functions: basic calls, recursion, forward calls, global read access, arity errors
  - Arrays: all built-in functions, index assignment, out-of-bounds errors
  - Dictionaries: access, assignment, new keys, `has_key`, `keys`, `values`, `length`
  - Truthiness for `0`, `""`, `[]`, `{}`
  - Error message span verification (errors report the correct line number)
  - Integration tests: FizzBuzz, word counter, Fibonacci
- **`vm::CaptureVm`** (test-only) — a VM variant that captures `PRINT` output to `Vec<String>` for in-process test assertions. Uses `dup2`/pipe redirection on Linux; gated by `#[cfg(test)]`, zero cost in release builds.

### Changed

- **`error.rs`** — `WhispemError` now holds `pub span: Span` instead of separate `pub line` and `pub column` fields. All construction sites updated. Display format unchanged: `[line N, col M] Error: ...`.
- **`compiler.rs`** — `compile_expr` gains a `ctx_line: usize` parameter. All call sites pass the statement's source line. `POP` after bare expression statements now correctly carries the statement's line number instead of `0`.
- **`ast.rs`** — removed `#[allow(dead_code)]`; all variants are reachable via the compiler.
- **`Cargo.toml`** — version bumped to `2.5.0`.
- **`src/repl.rs`** — version string updated to `2.5.0`.

### Fixed

- **`POP` line number was always `0`** — expressions compiled as statements now emit `POP` with the correct source line. `--dump` output is now fully accurate.
- **No warnings on `cargo build` or `cargo test`** — all `#[allow(dead_code)]`, unused imports, and unreachable code removed or justified.

### Architecture notes

The `Span` type is intentionally minimal for v2.5.0: it stores `(line, column)` as reported by the lexer. Future versions may extend it to carry a byte offset or a range, which would enable underline-style error messages.

The test harness uses `CaptureVm` for in-process execution. This avoids spawning subprocesses (which would require the binary to be pre-built) and keeps test latency low.

---

## [2.0.0] — 2026-02-25

**The bytecode VM release.** Whispem now compiles to bytecode and runs on a stack-based virtual machine instead of walking the AST directly.

### Added
- `src/value.rs`, `src/opcode.rs`, `src/chunk.rs`, `src/compiler.rs`, `src/vm.rs`
- `--dump` flag — prints human-readable bytecode disassembly
- `POP` opcode (`0x71`) — emitted after every bare expression statement
- `HALT` now pops its frame
- `docs/vm.md` — complete VM specification

### Changed
- `src/main.rs`, `src/repl.rs` — use new Compiler + Vm pipeline

### Fixed
- `SET_INDEX` now actually mutates
- Function parameter binding order
- REPL frame leak
- Lexer double-match
- Parser built-in token names

---

## [1.5.0] — prior release

Tree-walking interpreter. Full language support including arrays, dicts, for loops, functions, break/continue, file I/O, and the interactive REPL.

---

## Roadmap

| Version | Goal |
|---------|------|
| [x] 1.5.0 | Tree-walking interpreter, full language, REPL |
| [x] 2.0.0 | Bytecode VM, compiler, `--dump`, `docs/vm.md` |
| [x] 2.5.0 | Richer error spans, `Span` type, automated test suite (60+ tests), 0 warnings |
| 3.0.0   | Self-hosting: Whispem compiler written in Whispem, targeting the WVM |