# My Journey: From Literature to Programming Languages

*Hi, I'm Emilie (but everyone calls me Em').  
This is the story of how I went from studying literature and linguistics to building programming languages and distributed systems in Rust.*

---

## Background: Where I Started

Before 2025, my world was literature, linguistics, and foreign languages—not computers.

When I looked at code, it seemed like mysterious acronyms and endless semicolons. 
The idea of *creating* a programming language? That felt like science fiction.

But curiosity won. And here we are.

---

## Early 2025: First Steps with Programming

**January–February 2025: Apple Foundation Program (AFP)**

My first real encounter with code happened through Swift, UI/UX design, and iOS development. 
Everything was visual and interactive—building apps by assembling pieces, clicking buttons, seeing immediate results.

**What I learned:**
- Variables, loops, and functions started making sense
- Programming felt like learning a new language—but one where you *create* instead of just analyze
- **Most importantly:** I realized I could actually learn to code

This was the spark.

---

## Spring–Summer 2025: Solo Exploration

After the AFP, I kept building Swift projects on my own. Small apps, experiments, just for fun.

With each project:
- Logic became more natural
- Problem-solving became addictive
- The question grew stronger: *"What really happens behind the scenes?"*

I wanted to understand not just *how* to use programming languages, but *how they work*.

---

## Autumn 2025: The Rust Chapter Begins

**Timeline:**
- **First Rust program:** October 27, 2025 at 00:27 UTC+1 - a simple "Hello World"
- **Built mini-kvstore-v2:** November 21, 2025
- **Released minikv (distributed storage):** December 2025 (v0.3.0 on Dec 22, v0.4.0 on Dec 31)
- **Public talk at Epitech Marseille:** December 16, 2025 - shared my journey with students
- **Featured in Programmez! magazine:** January 19, 2026 - web publication about minikv ([read it here](https://www.programmez.com/actualites/minikv-un-key-value-store-distribue-en-rust-construit-en-public-38861))

Everyone said:
- "Rust is way too hard for beginners"
- "The borrow checker will make you cry"
- "You need years of experience first"

But I wanted to *really* understand how systems work. 
I wanted to see memory, ownership, and low-level concepts with my own hands.

So I started anyway.

---

## What I Found in Rust

**The compiler became my teacher**

Error messages in Rust are *detailed*. 
They don't just say "wrong"—they explain *why* and often *how to fix it*.

**Ownership changed how I think**

I thought I understood "ownership" from literature analysis. 
Rust forced me to internalize it at a deeper level: who owns what, who can read or modify, and for how long.

**Everything is explicit**

No hidden behavior. No magic. You see exactly what the computer will do.

**The community is genuinely welcoming**

Despite Rust's reputation for being "hard," the community is incredibly supportive—even to total beginners.

---

## What Helped Me Learn

1. **The Rust Book** - Especially Chapter 4 on ownership
2. **Clippy** - My strict but helpful code reviewer
3. **Taking notes** - Writing down concepts, errors, and solutions
4. **Building real projects** - Not just tutorials, but actual things I wanted to create
5. **Accepting failure as progress** - Every bug taught me something

---

## My Non-Technical Background: An Advantage

Skills from literature and linguistics that transferred perfectly:

- **Close reading** → Careful attention to compiler messages
- **Structure and syntax** → Understanding language grammar (programming or natural)
- **Pattern recognition** → Seeing recurring structures in code
- **Patience with ambiguity** → Sitting with confusion until understanding emerges
- **Teaching and explaining** → Breaking down complex ideas into simple terms

Coming from outside tech wasn't a disadvantage—it gave me fresh perspectives.

---

## Building Whispem: A Language of My Own

After learning Rust and building distributed systems, I wanted to create something different: **a programming language you can fully understand**.

**Why Whispem?**

Most languages grow complex over time. Features pile up. The surface area expands.

I wanted the opposite:
- Small enough to learn in a weekend
- Simple enough to implement yourself
- Clear enough to read like prose
- Complete enough to build real programs

**Philosophy:**

*"Code should whisper intent, not shout complexity."*

Every feature in Whispem exists for a reason. Nothing is hidden. No surprises.

**What I learned building Whispem:**
- How lexers tokenize text
- How parsers build syntax trees
- How interpreters execute code
- How to design for clarity over cleverness
- How to make error messages actually helpful

---

## Major Milestones

**October 27, 2025** - First Rust program ("Hello World")  
**November 21, 2025** - Shipped mini-kvstore-v2  
**December 2025** - Released minikv with Raft consensus, distributed storage  
**December 16, 2025** - Public talk at Epitech Marseille about my journey  
**January 19, 2026** - Featured in Programmez! magazine  
**February 2026** - Whispem 1.0.0 released - production-ready programming language

From complete beginner to building production systems in **3.5 months**.

---

## What I Wish I'd Known Earlier

**You don't need to be "technical" to start**

Curiosity and persistence matter more than background.

**Error messages are your friends**

Read them carefully. They're trying to teach you.

**You learn by building**

Tutorials help, but real learning happens when you create your own projects.

**The "right time" doesn't exist**

Start before you feel ready. You get ready by doing.

**Community matters**

Don't hesitate to ask questions. Most developers *love* helping beginners.

---

## Tips for Beginners

1. **Start now** - Don't wait until you "know enough"
2. **Build things** - Make stuff you actually want to use
3. **Read error messages** - They contain all the clues
4. **Celebrate small wins** - Your first compiling program is a victory!
5. **Ask for help** - Discord, forums, Reddit—people want to help
6. **Have fun** - If it's not enjoyable, you won't stick with it

---

## About Rust Aix-Marseille (RAM)

Inspired by my journey, I founded **Rust Aix-Marseille (RAM)** in January 2026—an inclusive Rust community in Provence and beyond.

**Mission:** Make Rust accessible, welcoming, and fun for everyone—from total beginners to experts.

**Growth:**
- 71 members on Discord
- 109 followers on LinkedIn
- First online meetup: January 14, 2026
- IRL events coming in Aix-Marseille area

**Values:**
- Radical inclusion - everyone truly welcome
- No gatekeeping - all questions are valid
- Learning together - beginners and experts side by side

**Join us:**
- [Discord](https://discord.gg/zgGWvVFJQg)
- [LinkedIn](https://www.linkedin.com/company/rust-aix-marseille-ram)

---

## Current Projects

**Whispem** - A minimalist programming language (v1.0.0)
- Feature-complete
- Built entirely in Rust
- Designed for clarity and teachability
- [github.com/whispem/whispem-lang](https://github.com/whispem/whispem-lang)

**minikv** - Distributed key-value store (v0.6.0+)
- Multi-node Raft consensus
- Distributed storage with replication
- S3-compatible API
- Real-time notifications (WebSocket/SSE)
- Production-grade features: TLS, RBAC, audit logging, quotas
- Built in public, documented journey

**RAM Community** - Building the Rust ecosystem in Provence
- Online and IRL meetups
- Talks, workshops, project showcases
- Welcoming space for learners

---

## What's Next

**Short term:**
- Grow Whispem community and ecosystem
- Expand RAM meetups (online + IRL)
- Help more beginners start their Rust journey

**Long term:**
- Self-hosting Whispem (implement Whispem in Whispem)
- Standard library and module system
- VS Code extension and online playground
- Continue building in public, sharing everything I learn

---

## My Philosophy

> *"Structure determines meaning. You learn by writing—and by building."*

Whether it's literature, linguistics, or programming languages, the principle is the same: understand the structure, practice creating, and share what you learn.

**Core beliefs:**
- Anyone can learn to code
- Clarity beats cleverness
- Small and understandable beats large and complex
- Teaching is the best way to learn
- Communities grow when everyone feels welcome

---

## Final Thoughts

**From October 27, 2025 to now:**

- Learned Rust from scratch
- Built distributed systems
- Created a programming language
- Gave public talks
- Got featured in tech publications
- Founded a community
- Helped others start their journeys

**If I can do this starting from literature and linguistics, you can too.**

The skills that matter most aren't technical—they're curiosity, persistence, willingness to learn, and courage to start before you feel ready.

---

## Let's Connect

I love meeting people who are learning, building, or just curious about programming.

**Find me:**
- GitHub: [@whispem](https://github.com/whispem)
- Whispem Language: [whispem-lang](https://github.com/whispem/whispem-lang)
- RAM Community: [Discord](https://discord.gg/zgGWvVFJQg) | [LinkedIn](https://www.linkedin.com/company/rust-aix-marseille-ram)

**Want to:**
- Learn Rust? Join RAM!
- Try Whispem? Clone the repo and build something!
- Share your journey? I'd love to hear it!

---

## Acknowledgments

To everyone who:
- Answered my beginner questions
- Reviewed my code
- Encouraged me to keep going
- Believed a literature student could build systems
- Joined RAM and helped it grow

**Thank you.**

This journey wouldn't exist without the kindness and generosity of the Rust community.

---

*"If you can read and express an idea, you can code. Patience, curiosity, and a love of learning are everything."*

**— Em' (@whispem)**  
Rust learner, language builder, community founder  
Living proof that anyone can learn to code

**Started Rust:** October 27, 2025 at 00:27  
**Today:** Building production systems and teaching others

*Let's build something amazing together.* ✨
