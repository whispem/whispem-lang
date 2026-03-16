# My Journey: From Literature to Programming Languages

*Hi, I'm Emilie — everyone calls me Em'.*
*This is the story of how I went from studying literature and linguistics to building a self-hosting programming language with a bytecode VM in Rust.*

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

**March 1, 2026** — Whispem 2.5.0. Correctness verified by 72 automated tests. `Span`, arity checking, short-circuit fix.

---

## March 2026 — Self-hosting and polish

**March 2, 2026** — Whispem 3.0.0. The self-hosting release.

Three things happened at once: bytecode serialisation (`.whbc` format), `LOAD_GLOBAL` opcode, and `compiler/wsc.wsp` — a Whispem compiler written in Whispem. The bootstrap is verified: the compiler compiles itself, producing bit-identical output. `vm/wvm.c` completes the chain: a standalone C runtime that runs `.whbc` without Rust.

**March 16, 2026** — Whispem 4.0.0. The polish release.

Four additions chosen for their daily impact: `else if` (no more nesting `if` inside `else`), `assert` (correctness checks), `type_of` (defensive code), `exit` (script control). Zero VM changes — purely lexer, parser, and builtins. 147 tests. Zero warnings.

Writing `else if` in Whispem for the first time after having worked around it since v1.0.0 was a small but satisfying moment. Some features are worth waiting until the language is ready for them.

---

## What building Whispem taught me

**v1.x — the interpreter:**
- How lexers tokenize source code
- How parsers build an AST
- How a tree-walking interpreter executes that AST
- How to make error messages useful

**v2.0.0 — the VM:**
- How bytecode compilers translate AST nodes into instructions
- How stack machines execute bytecode using call frames
- How constants pools, jump patching, and parameter binding work

**v2.5.0 — correctness:**
- How to write an in-process test harness without platform-specific code
- How short-circuit evaluation works at the bytecode level

**v3.0.0 — self-hosting:**
- How binary formats are structured
- What it means for a language to describe its own compilation
- That the gap between "working interpreter" and "self-hosting" is mostly conceptual
- That a self-hosted compiler can have subtle bugs invisible until you write an autonomous test suite

**v4.0.0 — polish:**
- That syntax sugar done right is invisible — `else if` emits identical bytecode to the nested form
- That `type_of` and `assert` cost almost nothing to implement but change how you write programs
- That zero warnings is a discipline worth keeping even when adding small features
- That sometimes the right time to add a feature is after you've used the workaround long enough to know exactly what you want

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
| March 2, 2026 | Whispem 3.0.0 — `.whbc`, `LOAD_GLOBAL`, `--compile`, self-hosting |
| March 2, 2026 | Standalone C VM, REPL, `--dump`, autonomous test suite — 125 tests, zero Rust dependency |
| March 16, 2026 | Whispem 4.0.0 — `else if`, `assert`, `type_of`, `exit`, 147 tests |

---

## Find me

- GitHub: [@whispem](https://github.com/whispem)
- RAM Community: [Discord](https://discord.gg/zgGWvVFJQg) · [LinkedIn](https://www.linkedin.com/company/rust-aix-marseille-ram)

---

*Em' — Marseille, 2026*