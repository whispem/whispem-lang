# My Journey: From Literature to Programming Languages

*Hi, I'm Emilie — everyone calls me Em'.*
*This is the story of how I went from studying literature and linguistics to building a self-hosting programming language with closures, lambdas, and a bytecode VM in Rust.*

---

## Where I started

Before 2025, my world was literature, linguistics, and foreign languages. Not computers.

I wasn't avoiding tech — it just wasn't part of my life. When I looked at code, it seemed like a foreign language I'd never been taught. The idea of *creating* a programming language felt like something other people did.

Curiosity won anyway.

---

## January 2025 — First contact with code

My first real encounter with programming happened through the **Apple Foundation Program** (AFP via Simplon) — a four-week intensive covering Swift, UI/UX design, and iOS prototyping.

Everything was visual and surface-level: assembling interfaces, wiring up buttons, seeing immediate results. It wasn't deep programming. But something clicked. I realized that code is just another way of expressing structure — and structure was something I already understood from years of reading and analyzing texts.

That was the spark.

---

## Spring–Summer 2025 — Solo exploration

After the AFP I kept building Swift projects on my own. Small things, just for fun. Each one made the underlying logic a little more familiar.

The question that kept growing: *what actually happens underneath?*

---

## October 2025 — Rust

**October 27, 2025 at 00:27** — first Rust program. A simple Hello World.

Everyone said Rust was too hard for beginners. The borrow checker would make me cry. I needed years of experience first.

I started anyway.

What I found: the compiler is strict, but it explains everything. Error messages in Rust don't just say "wrong" — they say *why*, and often *how to fix it*. For someone coming from close reading and textual analysis, that felt familiar. Read carefully, understand the structure, fix it.

Ownership was the hardest concept. It took time. But once it clicked, it changed how I think about programs entirely.

---

## Late 2025 — Building in public

While learning Rust I built **minikv**, a distributed key-value store with Raft consensus. It was way above my level at the time — which was exactly the point.

**December 16, 2025** — gave a public talk at Epitech Marseille about my journey from literature to distributed systems.

**January 19, 2026** — featured in *Programmez!* magazine for minikv.

In January 2026 I also founded **Rust Aix-Marseille (RAM)**, an inclusive Rust community in Provence.

---

## February 2026 — Whispem

After several months of Rust, I wanted to build something different: a programming language you could *fully understand* — including its own implementation.

Not a toy. Not a clone. Something with a clear philosophy:

> *Code should whisper intent, not shout complexity.*

**February 1, 2026** — Whispem 1.0.0.

**February 19, 2026** — Whispem 1.5.0. Dictionaries, modulo, interactive REPL.

**February 25, 2026** — Whispem 2.0.0. The tree-walking interpreter replaced by a bytecode compiler and a stack-based virtual machine.

**March 1, 2026** — Whispem 2.5.0. Correctness verified by 72 automated tests.

---

## March 2026 — Self-hosting, polish, and closures

**March 2, 2026** — Whispem 3.0.0. The self-hosting release. Bytecode serialisation, `LOAD_GLOBAL`, `compiler/wsc.wsp`, verified bootstrap, standalone C VM.

**March 16, 2026** — Whispem 4.0.0. The polish release. `else if`, `assert`, `type_of`, `exit`. Zero VM changes — purely lexer, parser, and builtins. 147 tests.

**March 18, 2026** — Whispem 5.0.0. The closure release.

Three additions that change what programs can express: **lambdas** (first-class anonymous functions), **closures** (capturing variables from enclosing scopes with shared mutable state), and **f-strings** (string interpolation with arbitrary expressions).

The upvalue machinery was the most interesting engineering challenge so far. The problem: how do you share a mutable variable between a function and the closures it creates, so that writes from either side are visible to the other? The answer in Whispem uses `Rc<RefCell<Upvalue>>` — a reference-counted heap cell — shared between the enclosing frame's `open_upvalues` table and the closure's upvalue list. When the enclosing scope stores a new value, it writes through the cell. When the closure reads it via `LOAD_UPVALUE`, it reads from the same cell.

The whole mechanism is ~150 lines of Rust and ~100 lines of C. Readable enough that you can understand it in one sitting — which was the point.

Writing the first closure test that actually worked — `make_counter()` returning `1, 2, 3` across three calls — was quietly satisfying. Small moment, big milestone.

---

## What building Whispem taught me

**v1.x — the interpreter:**
How lexers tokenize. How parsers build ASTs. How tree-walking interpreters execute. How to make error messages useful.

**v2.0.0 — the VM:**
How bytecode compilers translate AST nodes into instructions. How stack machines execute. How constants pools, jump patching, and parameter binding work.

**v2.5.0 — correctness:**
How to write an in-process test harness. How short-circuit evaluation works at the bytecode level.

**v3.0.0 — self-hosting:**
How binary formats are structured. What it means for a language to describe its own compilation. That the gap between "working interpreter" and "self-hosting" is mostly conceptual.

**v4.0.0 — polish:**
That syntax sugar done right is invisible. That `else if` emits identical bytecode to the nested form. That zero warnings is a discipline worth keeping.

**v5.0.0 — closures:**
How upvalue analysis works in a scope-stack compiler. How to share mutable state between frames using reference-counted heap cells. That the inline-name encoding for `MAKE_CLOSURE` — writing variable names as raw bytes in the bytecode stream instead of using constant-pool indices — is the cleanest solution because it makes descriptors self-contained. That `Rc<RefCell<T>>` is the right tool when you need shared mutable state with deterministic cleanup in Rust.

---

## On coming from a non-technical background

People sometimes ask if my background was a disadvantage.

Honestly, no. Close reading transfers directly to reading compiler errors. Attention to structure transfers to understanding grammars. Patience with ambiguity is just patience with debugging.

The skills that matter most in programming — careful observation, systematic thinking, knowing when to ask for help — aren't technical skills. They're habits of mind.

---

## Timeline

| Date | Milestone |
|------|-----------|
| January 2025 | Apple Foundation Program — first contact with code |
| October 27, 2025 | First Rust program (00:27) |
| December 16, 2025 | Public talk at Epitech Marseille |
| January 14, 2026 | First RAM online meetup |
| January 19, 2026 | Featured in *Programmez!* magazine |
| February 1, 2026 | Whispem 1.0.0 |
| February 19, 2026 | Whispem 1.5.0 |
| February 25, 2026 | Whispem 2.0.0 — bytecode VM |
| March 1, 2026 | Whispem 2.5.0 — error spans, arity, short-circuit, 72 tests |
| March 2, 2026 | Whispem 3.0.0 — self-hosting, C VM, 125 tests |
| March 16, 2026 | Whispem 4.0.0 — `else if`, `assert`, `type_of`, `exit`, 147 tests |
| March 19, 2026 | Whispem 5.0.0 — closures, lambdas, f-strings, 130 Rust + 37 autonomous tests |

---

## Find me

- GitHub: [@whispem](https://github.com/whispem)
- RAM Community: [Discord](https://discord.gg/zgGWvVFJQg) · [LinkedIn](https://www.linkedin.com/company/rust-aix-marseille-ram)

---

*Em' — Marseille, March 2026*