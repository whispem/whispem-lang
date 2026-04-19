# Changelog

All notable changes to Whispem are documented here.
Format: [Semantic Versioning](https://semver.org). Dates are in YYYY-MM-DD format.

---

## [6.0.0] — 2026-04-19

**The higher-order release.** `map`, `filter`, `reduce` as first-class builtins. F-strings and closures land in the self-hosted compiler. Lambda naming bug fixed. 153 Rust tests · 51 autonomous tests · zero warnings.

### Added

- **`map(array, f) → array`** — applies `f` to every element and returns a new array.

  ```wsp
  print map([1, 2, 3, 4], fn(x) { return x * 2 })   # [2, 4, 6, 8]
  ```

- **`filter(array, pred) → array`** — keeps elements for which `pred` returns truthy.

  ```wsp
  print filter([1, 2, 3, 4, 5, 6], fn(n) { return n % 2 == 0 })   # [2, 4, 6]
  ```

- **`reduce(array, f, initial) → value`** — folds `f(accumulator, element)` left-to-right starting from `initial`.

  ```wsp
  print reduce([1, 2, 3, 4, 5], fn(acc, n) { return acc + n }, 0)   # 15
  ```

  All three compose naturally with closures and each other:

  ```wsp
  let total = reduce(
      map(filter(range(1, 11), fn(n) { return n % 2 == 0 }),
          fn(n) { return n * n }),
      fn(acc, n) { return acc + n },
      0)
  print total   # 220  (sum of squares of evens 2..10)
  ```

- **`map`, `filter`, `reduce` as reserved keywords** — lexed and parsed identically to other builtins; callable in both expression and statement position.

- **`compiler/wsc.wsp` updated to v6.0** — the self-hosted compiler now recognises `map`, `filter`, `reduce` as keywords and emits correct `CALL` instructions for them. F-string lexing and parsing were already present from v5; the self-hosted compiler now documents this explicitly.

### Fixed

- **Lambda naming collision** (`compiler.rs`) — nested lambdas on the same source line previously received identical names (`__lambda_{line}_0`) because the counter used `self.functions.len()`, which does not increment during recursive compilation of inner lambdas. The compiler now maintains a monotonically increasing `lambda_count` field, guaranteeing unique names across all nesting depths. This fixed `outer(1)(2)(3)` returning a closure instead of `6`.

- **`invoke_closure` argument order** (`vm.rs`) — `map`/`filter`/`reduce` call closures via `invoke_closure`. Arguments were incorrectly reversed before being pushed onto the stack, causing `reduce` to produce wrong results (e.g. `"dcba"` instead of `"abcd"` for string concatenation). Arguments are now pushed in the order they arrive.

- **`map`/`filter`/`reduce` in statement position** — calling these builtins as a statement (`map(arr, f)` on its own line) previously failed with an unexpected-token error because the parser only recognised `Identifier` tokens in statement position. The parser now handles `Token::Map`, `Token::Filter`, and `Token::Reduce` as statement forms.

### Changed

- **`src/token.rs`** — `Token::Map`, `Token::Filter`, `Token::Reduce` added.
- **`src/lexer.rs`** — `map`, `filter`, `reduce` keywords recognised.
- **`src/parser.rs`** — map/filter/reduce handled in `parse_stmt` and `parse_primary`.
- **`src/compiler.rs`** — `lambda_count: usize` field; lambda names now use this counter instead of `functions.len()`.
- **`src/vm.rs`** — `invoke_closure` for closure dispatch from builtins; `execute_until` helper for depth-bounded execution; `step` extracted from `execute` to avoid duplication; argument order fixed.
- **`compiler/wsc.wsp`** — version string updated to v6.0; `map`/`filter`/`reduce` keywords added to `keyword_kind`, `is_builtin_call_kind`, and `is_builtin_primary_kind`.
- **`src/repl.rs`** — version string updated to `6.0.0`.
- **`Cargo.toml`** — version `6.0.0`.

### Architecture notes

**`invoke_closure` and depth-bounded execution.** `map`, `filter`, and `reduce` need to call user-supplied closures synchronously and collect their return values. The implementation records `target_depth = frames.len()` before pushing the closure frame, then runs `execute_until(target_depth)` — a copy of the dispatch loop that exits when the frame stack drops back to that depth. This avoids any duplication of opcode logic: all opcodes except `Return`, `ReturnNone`, and `Halt` are handled by the shared `step()` method.

**`lambda_count`.** The fix is minimal: replace `self.functions.len()` with `self.lambda_count`, which is incremented immediately before each `compile_fn_body` call for a lambda. The counter is field-level (not scope-level), so it remains monotone across all nesting depths and any number of functions.

**No new opcodes.** `map`, `filter`, and `reduce` are pure builtins dispatched through the existing `CALL` mechanism. The `.whbc` format version stays at `0x04`.

---

## [5.0.0] — 2026-03-19

**The closure release.** First-class functions, closures with mutable shared state, lambdas, and f-string interpolation. 130 Rust tests · 37 autonomous tests. Zero warnings.

### Added

- **Closures** — functions that capture variables from their enclosing scope.
- **Lambdas** — `fn(params) { body }` as an expression.
- **F-strings** — `f"..."` with `{expr}` interpolation.
- `OpCode::MakeClosure` (0x53), `LoadUpvalue` (0x13), `StoreUpvalue` (0x14), `CloseUpvalue` (0x15).
- `Value::Closure { chunk, upvalues }`.
- `ErrorKind::UpvalueError(String)`.

---

## [4.0.0] — 2026-03-16

**The polish release.** `else if` syntax, `assert`, `type_of`, `exit`. 147 tests. Zero warnings.

---

## [3.0.0] — 2026-03-02

**The self-hosting release.** Bytecode serialisation, `LOAD_GLOBAL`, self-hosted compiler, C VM, bootstrap verification. 125 tests.

---

## [2.5.0] — 2026-03-01

**The quality release.** Zero warnings, richer error spans, 72 automated tests.

---

## [2.0.0] — 2026-02-25

**The bytecode VM release.**

---

## [1.5.0] — prior release

Tree-walking interpreter.

---

## Roadmap

| Version | Goal |
|---------|------|
| [x] 1.5.0 | Tree-walking interpreter, full language, REPL |
| [x] 2.0.0 | Bytecode VM, compiler, `--dump` |
| [x] 2.5.0 | Error spans, arity checking, short-circuit, 72 tests, 0 warnings |
| [x] 3.0.0 | `.whbc` serialisation, self-hosted compiler, C VM, 125 tests |
| [x] 4.0.0 | `else if`, `assert`, `type_of`, `exit`, 147 tests |
| [x] 5.0.0 | Closures, lambdas, f-strings, 130 Rust + 37 autonomous tests |
| [x] 6.0.0 | `map` / `filter` / `reduce`; f-strings + closures in self-hosted compiler; lambda naming fix |
| 7.0.0 | `map` / `filter` / `reduce` in C VM; string methods (`split`, `trim`, `starts_with`); `none` literal |