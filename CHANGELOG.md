# Changelog

All notable changes to Whispem are documented here.
Format: [Semantic Versioning](https://semver.org). Dates are in YYYY-MM-DD format.

---

## [5.0.0] — 2026-03-19

**The closure release.** First-class functions, closures with mutable shared state, lambdas, and f-string interpolation. 130 Rust tests · 37 autonomous tests. Zero warnings.

### Added

- **Closures** — functions that capture variables from their enclosing scope. Captured variables are shared: multiple closures created in the same scope see each other's mutations.

  ```wsp
  fn make_counter() {
      let count = 0
      return fn() {
          let count = count + 1
          return count
      }
  }
  let c = make_counter()
  print c()   # 1
  print c()   # 2
  print c()   # 3
  ```

- **Lambdas** — `fn(params) { body }` as an expression. Can be stored in variables, passed as arguments, returned from functions, and stored in arrays or dicts.

  ```wsp
  fn apply(f, x) { return f(x) }
  print apply(fn(n) { return n * n }, 5)   # 25

  let fns = [fn(x) { return x + 1 }, fn(x) { return x * 2 }]
  print fns[0](10)   # 11
  print fns[1](10)   # 20
  ```

- **F-strings** — `f"..."` with `{expr}` interpolation. Any expression is valid inside braces. Compiles to a chain of `ADD` operations — zero VM impact.

  ```wsp
  let name = "Em"
  let score = 42
  print f"Hello, {name}! Your score is {score}."
  print f"Double: {score * 2}"
  print f"{length([1, 2, 3])} items"
  ```

- **`CallExpr` AST node** — call on an arbitrary expression value (e.g. `make_adder(1)(2)`, `fns[0](x)`). The parser now handles chained calls and indexed calls in statement position.

- **`Expr::Lambda`** in the AST — anonymous function expression.

- **`FStrPart` / `Token::FStr`** — f-string token and AST support. F-strings are desugared to `Binary::Add` chains by the parser; the compiler and VM are unaffected.

- **`OpCode::MakeClosure` (0x53)** — creates a `Value::Closure` from a named chunk and a list of captured upvalues. Upvalue descriptors are encoded inline in the bytecode as `(is_local: u8, name_len: u8, name_bytes...)` — no dependency on any chunk's constant pool.

- **`OpCode::LoadUpvalue` (0x13)** — push the value of an upvalue from the current frame's upvalue list.

- **`OpCode::StoreUpvalue` (0x14)** — write a value back through a shared upvalue cell.

- **`OpCode::CloseUpvalue` (0x15)** — reserved for future use; emitted as a no-op marker.

- **`Value::Closure { chunk, upvalues }`** — new value variant. `type_of` returns `"function"`.

- **`Upvalue`** — heap-allocated `Rc<RefCell<Upvalue>>` cell shared between the enclosing frame and any closures that capture the same variable. Mutations via `StoreUpvalue` are visible to all sharers.

- **`CallFrame::open_upvalues`** — `HashMap<String, Rc<RefCell<Upvalue>>>` that tracks which locals in the current frame have been wrapped in shared upvalue cells.

- **`ErrorKind::UpvalueError(String)`** — new error kind for internal upvalue invariant violations.

- **130 Rust tests + 37 autonomous tests** — 110 Rust carried over from v4, 20 new for v5 features. All pass. Zero warnings.

### Changed

- **`src/token.rs`** — `Token::FStr(Vec<FStrPart>)` added. `FStrPart` is `Literal(String)` or `Expr(String)` (raw source of the interpolated expression).

- **`src/opcode.rs`** — `LoadUpvalue` (0x13), `StoreUpvalue` (0x14), `CloseUpvalue` (0x15), `MakeClosure` (0x53) added. Total: **38 opcodes**.

- **`src/value.rs`** — `Value::Closure` added. `Upvalue` type simplified to a single `Closed(Box<Value>)` state (eager capture with shared mutation via `Rc<RefCell<>>`). `type_name()` returns `"function"` for closures. `is_truthy()` returns `true` for closures.

- **`src/ast.rs`** — `Expr::Lambda`, `Expr::CallExpr`, `Expr::FStr`, `FStrPart` added.

- **`src/error.rs`** — `ErrorKind::UpvalueError(String)` added.

- **`src/lexer.rs`** — f-string lexing: `f"..."` prefix, `{...}` holes with nested brace tracking, `\{` and `\}` escape sequences.

- **`src/parser.rs`** — f-string desugaring to `Binary::Add` chains; lambda parsing (`fn(params) { body }` as expression); `parse_postfix` handles `CallExpr`; `parse_ident_stmt` handles `ident[idx]()` and chained calls in statement position.

- **`src/compiler.rs`** — upvalue analysis: `FnScope` tracks locals and upvalue captures by name; `resolve_upvalue` walks the scope stack recursively; `emit_make_closure` emits inline name descriptors; `Stmt::Let` emits `StoreUpvalue` when assigning to a captured variable; `Expr::Lambda` compiles to `compile_fn_body` + `emit_make_closure`; `Expr::CallExpr` compiles to `CALL __callee__ argc`.

- **`src/chunk.rs`** — `Chunk` gains `upvalue_count` field; `.whbc` format version bumped to `4`; serialisation writes `upvalue_count`; disassembler handles variable-length `MAKE_CLOSURE` descriptors.

- **`src/vm.rs`** — `CallFrame` gains `open_upvalues: HashMap<String, Rc<RefCell<Upvalue>>>` for shared cell tracking; `MAKE_CLOSURE` reads inline name descriptors and creates/reuses shared cells; `CALL` checks locals for `Value::Closure` before falling through to the functions table; `call_value` dispatches `Value::Closure`; `store()` writes through open upvalue cells; `LoadUpvalue` / `StoreUpvalue` / `CloseUpvalue` implemented.

- **`src/repl.rs`** — version string updated to `5.0.0`.

- **`Cargo.toml`** — version `5.0.0`.

### Architecture notes

**Upvalue encoding.** Variable names are written verbatim in the `MAKE_CLOSURE` bytecode stream (`is_local: u8`, `name_len: u8`, `name_bytes...`). This avoids any dependency on a specific chunk's constant pool and makes upvalue descriptors self-contained — the `.whbc` format remains straightforwardly readable.

**Eager capture.** Upvalues are captured by value at closure-creation time, then shared via `Rc<RefCell<Upvalue>>`. The enclosing frame registers a cell in `open_upvalues` on first capture; subsequent closures and `store()` calls reuse the same `Rc`. This gives correct mutable shared state (two closures created in the same scope share the same cell) without requiring a stack-scanning phase.

**Lambda naming.** Each lambda gets a unique internal name `__lambda_{line}_{count}` and is stored in `self.functions` like any named function. `MAKE_CLOSURE` always uses this name to locate the prototype.

**`__callee__` sentinel.** `Expr::CallExpr` compiles to `CALL __callee__ argc` with the callee value on the stack below the arguments. The VM detects the sentinel name and pops the callee from the stack instead of looking it up in the functions table.

**Format version.** `.whbc` files from v5 use version byte `0x04`. Files produced by v4 (version `0x03`) will not load in v5 and vice-versa. Recompile from source.

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

- **`type_of(value)` builtin** — returns the runtime type of any value as a string: `"number"`, `"string"`, `"bool"`, `"array"`, `"dict"`, or `"none"`.

- **`exit(code?)` builtin** — terminates the program with the given exit code (default `0`).

- **`ErrorKind::AssertionFailed(String)`** and **`ErrorKind::Exit(i64)`**.

- **17 new tests**, total 147 (110 Rust + 37 autonomous). All pass. Zero warnings.

### Changed

- `src/lexer.rs` — `collapse_else_if` post-processing pass.
- `src/parser.rs` — `parse_else_branch` handles `ElseIf` recursively.
- `src/error.rs` — `AssertionFailed`, `Exit` added.
- `src/vm.rs` — `assert`, `type_of`, `exit` builtins; dict key-not-found error message improved.
- `src/main.rs` — `handle_vm_error` separates `Exit` from real errors.
- `compiler/wsc.wsp` — updated to v4.

### Fixed

- Dict key-not-found error was `undefined variable 'dict key "foo"'`; now `key "foo" not found in dict`.

---

## [3.0.0] — 2026-03-02

**The self-hosting release.** Bytecode serialisation, explicit global reads, a Whispem compiler written in Whispem, and verified bootstrap. Standalone C VM. 125 tests.

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
| [x] 2.0.0 | Bytecode VM, compiler, `--dump`, `docs/vm.md` |
| [x] 2.5.0 | Error spans, arity checking, short-circuit fix, 72 tests, 0 warnings |
| [x] 3.0.0 | `.whbc` serialisation, `LOAD_GLOBAL`, `--compile`, self-hosted compiler, verified bootstrap, Rc COW, standalone C VM, 125 tests |
| [x] 4.0.0 | `else if`, `assert`, `type_of`, `exit`, dict error messages, 147 tests |
| [x] 5.0.0 | Closures, lambdas, f-strings, 130 Rust + 37 autonomous tests |
| 6.0.0 | String interpolation in self-hosted compiler + C VM; `map` / `filter` / `reduce` builtins |