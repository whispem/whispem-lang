# Changelog

All notable changes to Whispem are documented here.
Format: [Semantic Versioning](https://semver.org). Dates are in YYYY-MM-DD format.

---

## [4.0.0] — 2026-03-16

**The polish release.** `else if` syntax, three new builtins (`assert`, `type_of`, `exit`), clearer dict error messages, and a self-hosted compiler updated to match. 17 new Rust tests + `tests/test_v4.0.0.wsp`. Zero warnings.

### Added

- **`else if` syntax** — chains of `if / else if / else` are now first-class syntax instead of nested `if` inside `else`. The lexer collapses `else (newline*) if` into a single `ELSE_IF` token; the parser handles it recursively, producing the same nested `If` AST nodes as before — zero VM or compiler impact.

  ```wsp
  if score >= 90 { print "A" }
  else if score >= 80 { print "B" }
  else if score >= 70 { print "C" }
  else { print "F" }
  ```

- **`assert(condition, message?)` builtin** — raises `AssertionFailed` with the provided message if the condition is falsy. Message is optional; defaults to `"assertion failed"`. Accepts any falsy value as failure (`false`, `0`, `""`, `[]`, `{}`, `none`).

  ```wsp
  assert(length(arr) > 0, "array must not be empty")
  assert(type_of(n) == "number", "expected a number")
  ```

- **`type_of(value)` builtin** — returns the runtime type of any value as a string: `"number"`, `"string"`, `"bool"`, `"array"`, `"dict"`, or `"none"`.

  ```wsp
  fn safe_add(a, b) {
      if type_of(a) != "number" or type_of(b) != "number" {
          return "type error"
      }
      return a + b
  }
  ```

- **`exit(code?)` builtin** — terminates the program with the given exit code (default `0`). Propagated as `ErrorKind::Exit(i64)` internally; the CLI catches it and calls `process::exit` rather than printing an error.

  ```wsp
  if length(args()) == 0 {
      print "Usage: script.wsp <file>"
      exit(1)
  }
  ```

- **`ErrorKind::AssertionFailed(String)`** — new error kind displayed as `Assertion failed: <message>`.

- **`ErrorKind::Exit(i64)`** — new error kind used to propagate `exit()` through the call stack without printing anything.

- **`Token::ElseIf`** — new token produced by the lexer's post-processing pass; never appears in source.

- **`Token::Assert`, `Token::TypeOf`, `Token::Exit`** — reserved keywords.

- **17 new tests** covering:
  - `else if` basic chain, last branch, fallthrough to `else`, no `else`, and FizzBuzz rewritten with `else if`.
  - `assert` passes with and without message, fails with message, fails with default message, fails on all falsy values.
  - `type_of` on all six value types, in a conditional guard.
  - `exit` stops execution, `exit(code)` propagates the correct code.

- **Total: 147 tests** (110 Rust + 37 autonomous). All pass. Zero warnings.

### Changed

- **`src/lexer.rs`** — after tokenising, a `collapse_else_if` pass rewrites `Else (Newline*) If` sequences into `ElseIf`. This keeps the grammar change entirely in the lexer; no other file needs to know about `else if` as a two-word sequence.

- **`src/parser.rs`** — `parse_if` delegates to a new `parse_else_branch` method that handles `ElseIf` (recursive) and `Else` (terminal). Also skips newlines before looking for `else if` / `else` so both inline and newline-separated styles work.

- **`src/error.rs`** — `ErrorKind` extended with `AssertionFailed(String)` and `Exit(i64)`.

- **`src/vm.rs`** — three new builtins: `assert`, `type_of`, `exit`. Dict key-not-found error message changed from `undefined variable 'dict key "foo"'` to `key "foo" not found in dict`. `call_builtin` signature changed to `&mut self` to allow future builtins that need mutable access.

- **`src/main.rs`** — new `handle_vm_error` helper separates `Exit` (clean exit with code) from genuine runtime errors (print and exit 1). Version string updated.

- **`src/repl.rs`** — version string updated to `4.0.0`. `exit()` in the REPL now terminates the process cleanly instead of printing an error.

- **`compiler/wsc.wsp`** — updated to v4.0: `keyword_kind` recognises `assert`, `type_of`, `exit`; the lexer's collapse loop produces `ELSE_IF` tokens; `parse_stmt` handles `ASSERT` and `EXIT`; `parse_else_branch` is a new recursive function handling `ELSE_IF` and `ELSE`; `parse_primary` recognises `TYPE_OF`, `ASSERT`, `EXIT` as variable-like primaries for use in expressions.

- **`Cargo.toml`** — version bumped to `4.0.0`.

### Fixed

- Dict key-not-found error message was `undefined variable 'dict key "foo"'` — now reads `key "foo" not found in dict`.
- `else if` on the next line after `}` now works correctly (newlines are skipped before checking for `else if` / `else`).

### Architecture notes

`else if` is pure syntax sugar with no representation in the AST, bytecode, or VM. The `ElseIf` token exists only inside the lexer's output; by the time the parser emits `Stmt::If` nodes, `else if` has become a nested `If` in the `else_branch` — the same structure produced by v3 when writing `else { if ... }` by hand.

`exit()` follows the same propagation path as any runtime error (`WhispemResult`), which means it correctly unwinds through nested function calls. The CLI distinguishes it from real errors by matching on `ErrorKind::Exit`.

---

## [3.0.0] — 2026-03-02

**The self-hosting release.** Bytecode serialisation, explicit global reads, a Whispem compiler written in Whispem, and **verified bootstrap** — the compiler compiles itself, producing bit-identical output across generations. Standalone C VM with REPL and disassembler. Autonomous test suite that runs without Rust.

### Added

- **`.whbc` bytecode format** — binary serialisation of compiled chunks.
  `cargo run -- --compile file.wsp` produces a `.whbc` file.
  `cargo run -- file.whbc` executes it directly — no recompilation needed.
  Format: magic `WHBC` + version byte + function table. See `docs/vm.md` for the complete spec.

- **`chunk::serialise(main, fns) → Vec<u8>`** — serialise a compiled program to the `.whbc` binary format.

- **`chunk::deserialise(data) → (Chunk, HashMap)`** — deserialise a `.whbc` buffer back to executable chunks.

- **`OpCode::LoadGlobal` (0x12)** — explicit global variable reads inside function bodies.

- **`--compile` CLI flag** — `whispem --compile file.wsp` compiles to `file.whbc`.

- **`compiler/wsc.wsp`** — a Whispem compiler written in Whispem, targeting the WVM.

- **`vm/wvm.c`** — standalone C runtime (~2000 lines). Build: `make`.

- **Bootstrap verified** — SHA-1 `f090aa0f650a3b00e4286b332f82c0ba5c3b71d5`, stable fixed point.

- **`tests/run_tests.sh`** — autonomous test suite using only the C VM. 32 tests.

- **Rc-based copy-on-write** for `Value::Array` and `Value::Dict`.

- **`args()`, `num_to_hex()`, `write_hex()`** builtins.

- **`ErrorKind::InvalidBytecode(String)`** and **`ErrorKind::SerializationError(String)`**.

- **Total: 125 tests** (93 Rust + 32 autonomous). All pass. Zero warnings.

### Changed

- `OpCode`, `Compiler`, `Value`, `Vm`, `Chunk`, `main.rs` — see v3.0.0 full notes.
- `Cargo.toml` — version `3.0.0`.

---

## [2.5.0] — 2026-03-01

**The quality release.** Zero warnings, richer error spans, full automated test suite.

### Added

- **`error::Span`** — `Span { line, column }` replacing bare `(usize, usize)` pairs.
- **Compiler propagates source line to every opcode.**
- **72 automated tests** in `src/main.rs`.
- **`vm::CaptureVm`** — VM variant for in-process test assertions.

### Changed

- `error.rs`, `compiler.rs`, `ast.rs`, `Cargo.toml` — version `2.5.0`.

### Fixed

- `POP` line number was always `0`.
- Zero warnings on `cargo build` and `cargo test`.

---

## [2.0.0] — 2026-02-25

**The bytecode VM release.**

### Added
- `src/value.rs`, `src/opcode.rs`, `src/chunk.rs`, `src/compiler.rs`, `src/vm.rs`
- `--dump` flag
- `POP` opcode (`0x71`)
- `docs/vm.md`

### Fixed
- `SET_INDEX` now actually mutates.
- Function parameter binding order.
- REPL frame leak.

---

## [1.5.0] — prior release

Tree-walking interpreter. Full language support including arrays, dicts, for loops, functions, break/continue, file I/O, and the interactive REPL.

---

## Roadmap

| Version | Goal |
|---------|------|
| [x] 1.5.0 | Tree-walking interpreter, full language, REPL |
| [x] 2.0.0 | Bytecode VM, compiler, `--dump`, `docs/vm.md` |
| [x] 2.5.0 | Error spans, arity checking, short-circuit fix, 72 tests, 0 warnings |
| [x] 3.0.0 | `.whbc` serialisation, `LOAD_GLOBAL`, `--compile`, self-hosted compiler, verified bootstrap, Rc COW, standalone C VM, 125 tests |
| [x] 4.0.0 | `else if`, `assert`, `type_of`, `exit`, dict error messages, 147 tests (110 Rust + 37 autonomes) |
| 5.0.0 | Closures, string interpolation |