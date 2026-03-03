# Changelog

All notable changes to Whispem are documented here.  
Format: [Semantic Versioning](https://semver.org). Dates are in YYYY-MM-DD format.

---

## [3.0.0] — 2026-03-02

**The self-hosting release.** Bytecode serialisation, explicit global reads, a Whispem compiler written in Whispem, and **verified bootstrap** — the compiler compiles itself, producing bit-identical output across generations. Standalone C VM with REPL and disassembler. Autonomous test suite that runs without Rust.

### Added

- **`.whbc` bytecode format** — binary serialisation of compiled chunks.  
  `cargo run -- --compile file.wsp` produces a `.whbc` file.  
  `cargo run -- file.whbc` executes it directly — no recompilation needed.  
  Format: magic `WHBC` + version byte + function table. See `docs/vm.md` for the complete spec.

- **`chunk::serialise(main, fns) → Vec<u8>`** — serialise a compiled program to the `.whbc` binary format.

- **`chunk::deserialise(data) → (Chunk, HashMap)`** — deserialise a `.whbc` buffer back to executable chunks. Validates magic bytes and format version; every malformed input returns a clear `InvalidBytecode` error.

- **`OpCode::LoadGlobal` (0x12)** — new opcode for explicit global variable reads inside function bodies. Replaces the v2.0.0 approach of copying the entire globals map into each new call frame at call time. Functions now emit `LOAD_GLOBAL` for global names and `LOAD` for locals — cleaner semantics and the correct foundation for the self-hosted compiler.

- **`--compile` CLI flag** — `whispem --compile file.wsp` compiles to `file.whbc`. Optionally accepts an explicit output path: `whispem --compile file.wsp out.whbc`.

- **`compiler/wsc.wsp`** — a Whispem compiler written in Whispem, targeting the WVM. Implements the full compilation pipeline: lexer, recursive-descent parser, bytecode compiler, and binary serialiser. Reads a `.wsp` source file and writes a `.whbc` bytecode file that the VM executes directly — output identical to the Rust compiler. Usage: `./wvm compiler/wsc.whbc <source.wsp>` (standalone) or `cargo run -- compiler/wsc.wsp <source.wsp>` (Rust).

- **`vm/wvm.c`** — standalone C runtime that executes `.whbc` bytecode without Rust. Single-file implementation with refcounted copy-on-write values, the same 34 opcodes and 20 builtins as the Rust VM. Produces byte-for-byte identical output, including bootstrapping the self-hosted compiler. Includes `--dump` disassembler and interactive REPL. Build: `make` (→ `wvm`). Usage: `./wvm file.whbc [args...]`, `./wvm --dump file.whbc`, or `./wvm` for the REPL.

- **`Makefile`** — builds the C VM: `gcc -O2 -Wall -Wextra -Wpedantic -o wvm vm/wvm.c -lm`.

- **Bootstrap verified** — `wsc.wsp` compiles itself to `wsc.whbc`, and the resulting bytecode compiler produces bit-identical output when compiling `wsc.wsp` again (SHA-1 `f090aa0f650a3b00e4286b332f82c0ba5c3b71d5`). This is the proof that the self-hosted compiler is a stable fixed point: `wsc(wsc) == wsc(wsc(wsc))`. The same fixed point holds on both the Rust VM and the C VM.

- **`tests/run_tests.sh`** — autonomous test suite using only the C VM and the bootstrapped compiler. Compiles each `.wsp` example via `wsc.whbc`, runs the result, and compares output to expected baselines. 32 tests including bootstrap verification. No Rust needed.

- **Rc-based copy-on-write for `Value::Array` and `Value::Dict`** — `Vec<Value>` and `HashMap<String, Value>` are now wrapped in `Rc`. Cloning a value just increments the reference count (O(1)); deep copies happen only on mutation via `Rc::make_mut`. This makes the bootstrap feasible — without it, the pass-by-value semantics caused the compiler to hang on its own 1618-line source.

- **`args()` builtin** — returns the script arguments passed after the `.wsp` filename as an array of strings. Enables `wsc.wsp` to accept input files from the command line.

- **`num_to_hex()` builtin** — encodes a number as a 16-character hex string (IEEE-754 big-endian f64). Used by the self-hosted serialiser to produce correct floating-point constants.

- **`write_hex()` builtin** — decodes a hex string to raw bytes and writes them to a file. Used by `wsc.wsp` to produce `.whbc` binary output.

- **`ErrorKind::InvalidBytecode(String)`** and **`ErrorKind::SerializationError(String)`** — two new error kinds for the bytecode pipeline.

- **`Compiler::global_names` and `Compiler::in_function`** — the compiler now tracks which names are global so it can emit `LOAD_GLOBAL` inside function bodies automatically, with no changes required to `.wsp` source files.

- **13 new tests** covering:
  - Bytecode round-trip for hello world, arithmetic, variables, functions, loops, arrays, dicts, FizzBuzz, and recursion.
  - Global variable access via `LOAD_GLOBAL` after round-trip.
  - Invalid magic bytes → `InvalidBytecode` error.
  - Wrong format version → `InvalidBytecode` error.
  - Truncated buffer → `InvalidBytecode` error.
  - `output_path` helper (`.wsp` → `.whbc` extension replacement).

- **Total: 125 tests** (93 Rust + 32 autonomous). All pass. Zero warnings.

### Changed

- **`src/opcode.rs`** — `LoadGlobal = 0x12` added. `operand_size()` updated to return `1` for it. `from_byte()` and `name()` updated accordingly.

- **`src/compiler.rs`** — `Compiler` gains `global_names: Vec<String>` and `in_function: bool`. First pass collects all top-level `let` names before compiling. `compile_expr` for `Expr::Variable` emits `LOAD_GLOBAL` when inside a function and the name is a known global, `LOAD` otherwise. Second-pass `Stmt::Let` in `<main>` appends to `global_names` as it executes.

- **`src/value.rs`** — `Value::Array` and `Value::Dict` now wrap their inner data in `Rc` for copy-on-write semantics. All pattern-matching code works unchanged thanks to `Deref`; only mutation sites use `Rc::make_mut`.

- **`src/vm.rs`** — `OpCode::Load` now calls `lookup_local` (locals only). `OpCode::LoadGlobal` reads `self.globals` directly. `OpCode::Call` no longer copies globals into the new frame's locals — `CallFrame` starts empty for user functions. `store()` logic unchanged: top-level stores go to `self.globals`, function stores go to the frame's `locals`. Three new builtins: `args()`, `num_to_hex()`, `write_hex()`. `length()` on strings now returns character count (not byte count) for correct UTF-8 handling. All Array/Dict construction sites wrap in `Rc::new`; all mutation sites use `Rc::make_mut`.

- **`src/main.rs`** — CLI extended with `--compile`, `.whbc` run path, and script argument passing. Arguments after the `.wsp` filename are forwarded to the script via `args()`. `output_path` function extracted and tested.

- **`src/token.rs`**, **`src/lexer.rs`**, **`src/parser.rs`** — `args` and `write_hex` added as reserved keywords with dedicated token variants.

- **`src/chunk.rs`** — `serialise` and `deserialise` functions added at module level. `MAGIC` and `FORMAT_VERSION` constants exported. Disassembler updated to print `LOAD_GLOBAL` with the same width as other `LOAD` instructions.

- **`src/error.rs`** — `ErrorKind` extended with `InvalidBytecode(String)` and `SerializationError(String)`. Both display cleanly.

- **`Cargo.toml`** — version bumped to `3.0.0`.

- **`src/repl.rs`** — version string updated to `3.0.0`.

### Architecture notes

The `LOAD_GLOBAL` opcode makes the compilation model explicit: a function chunk now contains two kinds of variable reads — `LOAD` for locals (parameters and inner `let`) and `LOAD_GLOBAL` for names that were declared at the top level of the source file. This is the semantics the self-hosted compiler needs to replicate.

The `.whbc` format stores line numbers alongside every bytecode byte (as `u32`), so error messages keep accurate source locations after a round-trip.

The self-hosted compiler (`compiler/wsc.wsp`) implements the full compilation pipeline in Whispem: lexer, recursive-descent parser, bytecode compiler, and binary serialiser. It reads a `.wsp` source file from disk and writes a `.whbc` bytecode file that the VM executes directly — output identical to the Rust compiler across all tested programs.

The bootstrap is verified: running `whispem compiler/wsc.wsp compiler/wsc.wsp` produces `compiler/wsc.whbc` (80 055 bytes), and running that bytecode to compile wsc.wsp again produces the same SHA-1 hash (`f090aa0f...`) — a stable fixed point. The standalone C VM (`vm/wvm.c`) reproduces the same fixed point with zero Rust dependency, completing the chain from source to binary to execution in two independent runtimes.

---

## [2.5.0] — 2026-03-01

**The quality release.** Zero warnings, richer error spans, and a full automated test suite.

### Added

- **`error::Span`** — `Span { line, column }` replacing bare `(usize, usize)` pairs. Every `WhispemError` now carries a `Span` with a human-readable `Display` impl (`[line N, col M]`).
- **`WhispemError::at(kind, line, col)`** — convenience constructor.
- **Compiler propagates source line to every opcode** — `compile_expr` receives `ctx_line`.
- **Automated test suite** — 72 tests in `src/main.rs` covering all language features.
- **`vm::CaptureVm`** — VM variant for in-process test assertions.

### Changed

- `error.rs` — `WhispemError` holds `pub span: Span`.
- `compiler.rs` — `compile_expr` gains `ctx_line: usize`.
- `ast.rs` — removed `#[allow(dead_code)]`.
- `Cargo.toml` — version `2.5.0`.

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
| 4.0.0   | `else if` syntax sugar, closures, column numbers in errors |