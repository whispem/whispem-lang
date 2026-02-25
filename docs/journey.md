# My Journey: From Literature to Programming Languages

*Hi, I'm Emilie — everyone calls me Em'.*  
*This is the story of how I went from studying literature and linguistics to building a programming language with a bytecode VM in Rust.*

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

**February 1, 2026** — Whispem 1.0.0. Variables, functions, loops, arrays, recursion, file I/O, error messages with line numbers.

**February 19, 2026** — Whispem 1.5.0. Dictionaries, modulo operator, interactive REPL, complete error overhaul.

**February 25, 2026** — Whispem 2.0.0. The tree-walking interpreter replaced by a bytecode compiler and a stack-based virtual machine. 31 opcodes, proper call frames, constants pool, disassembler. The pipeline is now:

```
Source → Lexer → Parser → AST → Compiler → Bytecode → VM
```

The v1.x series was built in about five days of actual work. The v2.0.0 VM took another week on top of that.

---

## What building Whispem taught me

**v1.x — the interpreter:**
- How lexers tokenize source code into tokens
- How parsers build an AST from tokens
- How a tree-walking interpreter executes that AST
- How to design a language that stays small on purpose
- How to make error messages useful instead of cryptic

**v2.0.0 — the VM:**
- How bytecode compilers translate AST nodes into instructions
- How stack machines execute bytecode using call frames
- How constants pools, jump patching, and parameter binding actually work
- Why separating compilation from execution matters
- What it means to inspect your own program with `--dump`

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

---

## Find me

- GitHub: [@whispem](https://github.com/whispem)
- RAM Community: [Discord](https://discord.gg/zgGWvVFJQg) · [LinkedIn](https://www.linkedin.com/company/rust-aix-marseille-ram)

---

*Em' — Marseille, 2026*