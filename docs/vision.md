# Whispem Vision

Whispem is not designed to compete with large general-purpose languages.

It is designed to be **understandable in its entirety**.

---

## Philosophy

Whispem values:

- Clarity over cleverness
- Explicitness over magic
- Calm readability
- Small, intentional feature sets

Every feature must earn its place.

---

## Why an interpreter?

Whispem is interpreted by design.

This allows:
- immediate feedback
- simple execution model
- easier experimentation
- full control over language semantics

Performance is not the primary goal.
Understanding is.

---

## Minimalism

Whispem avoids:

- implicit behavior
- hidden state
- complex syntax
- unnecessary abstractions
- syntactic sugar without purpose

If something can be explained simply, it should be.

---

## Growth model

Whispem grows in **layers**:

1. ✅ Expressions
2. ✅ Variables
3. ✅ Control flow (if/else)
4. ✅ Loops (while)
5. ✅ Logic (and/or/not)
6. ✅ Functions
7. 🔄 Standard library
8. 🔄 Collections

Each layer must remain stable before the next is added.

**Current status:** Layers 1-6 complete and stable. Ready for 1.0.0 consideration.

---

## Design Principles

### 1. Readability First

Code should read like intent:
```wsp
fn calculate_total(price, quantity) {
    return price * quantity
}

let total = calculate_total(10, 5)
print total
```

Not:
```
def calc(p,q){return p*q;}let t=calc(10,5);print(t);
```

### 2. No Surprises

What you see is what you get.
No operator overloading (except `+` for strings).
No implicit conversions.
No hidden mutations.

### 3. Small Surface Area

The entire language should fit in your head.
If you can't explain a feature in one sentence, it's too complex.

### 4. Teachable

Someone new to programming should be able to:
- Write their first program in 5 minutes
- Understand the full language in a weekend
- Read the entire implementation in an afternoon

---

## What Whispem Is Not

Whispem is **not**:

- A systems programming language
- Performance-focused
- Trying to replace Python, JavaScript, or Rust
- Aiming for maximum expressiveness
- Designed for production applications

Whispem **is**:

- A teaching tool
- An exploration of minimalism
- A language you can fully understand
- A demonstration that less can be more

---

## Audience

Whispem is for:

- People learning how languages work
- Developers who value simplicity
- Creators who enjoy small, elegant systems
- Anyone who wants to **understand** their tools

---

## Progress Tracker

### Completed Features
- ✅ Variables and reassignment
- ✅ Numbers, strings, booleans
- ✅ Arithmetic expressions
- ✅ Operator precedence
- ✅ Comparisons
- ✅ Logical operators (and, or, not)
- ✅ Conditional execution (if/else)
- ✅ While loops
- ✅ Comments
- ✅ String escape sequences
- ✅ Unary operators
- ✅ **Functions with parameters**
- ✅ **Return values**
- ✅ **Recursion**
- ✅ **Local variable scopes**
- ✅ **String concatenation**

### Next Features (Path to 1.0.0)
1. Break and continue
2. Better error messages
3. Arrays or lists
4. Standard library (math, string operations)
5. File I/O

### Post 1.0.0
- Self-hosting (Whispem interpreter written in Whispem)
- Module system
- Complete language specification
- Tutorial and book

---

## v0.8.0 Milestone

With the addition of functions in v0.8.0, Whispem has reached a critical milestone:

**Whispem is now a complete, Turing-complete programming language.**

You can now:
- Define reusable functions
- Build abstractions
- Write recursive algorithms
- Structure complex programs
- Express any computable function

This marks the transition from "toy language" to "real language."

The path to 1.0.0 is now clear:
- Polish the rough edges
- Add quality-of-life features
- Improve error messages
- Expand the standard library

---

## Long-term vision

Eventually, Whispem aims to be:

- **Self-hosted** — bootstrap the interpreter in Whispem itself
- **Distributable** — single binary, no dependencies
- **Fully specified** — formal grammar and semantics
- **Boring** — in the best possible way

Boring means:
- No breaking changes
- Stable and predictable
- Well-documented
- Completely understood

---

## Success Criteria

Whispem will be considered successful when:

1. A beginner can learn the entire language in one day
2. The implementation can be read and understood in one sitting
3. Every feature has a clear justification
4. The documentation is complete and accessible
5. The language is useful for teaching programming concepts
6. **The language is Turing-complete and practical** ✅ (achieved in v0.8.0)

---

## Why "Whispem"?

Because the language is meant to **whisper intent**,
not shout complexity.

Code should be quiet.
Clear.
Calm.

---

## Contributing Philosophy

Contributions should:

- Make the language simpler, not more complex
- Have a clear use case
- Be teachable in under 5 minutes
- Not break existing programs
- Align with the minimalist philosophy

Before adding a feature, ask:

- Can the user accomplish this another way?
- Does this make the language harder to understand?
- Would a beginner find this intuitive?
- Is this essential or just convenient?

If the answer to the last question is "just convenient," 
the feature probably doesn't belong in Whispem.

---

**Version:** 0.8.0  
**Status:** Feature complete for 1.0.0 consideration  
**Philosophy:** Whisper, don't shout
