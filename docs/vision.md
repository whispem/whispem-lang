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
7. ✅ Collections (arrays)
8. 🔄 Standard library expansion
9. 🔄 Quality of life improvements

Each layer must remain stable before the next is added.

**Current status:** Layers 1-7 complete and stable. Ready for 1.0.0.

---

## Design Principles

### 1. Readability First

Code should read like intent:
```wsp
fn find_max(numbers) {
    let max = numbers[0]
    let i = 1
    
    while i < length(numbers) {
        if numbers[i] > max {
            let max = numbers[i]
        }
        let i = i + 1
    }
    
    return max
}

let values = [5, 2, 9, 1, 7]
print find_max(values)
```

Not:
```
def f(a){m=a[0];for(i=1;i<len(a);i++)if(a[i]>m)m=a[i];return m}
```

### 2. No Surprises

What you see is what you get.
No operator overloading (except `+` for strings).
No implicit conversions.
No hidden mutations (arrays are explicitly copied with `push()`).

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
- ✅ Functions with parameters
- ✅ Return values
- ✅ Recursion
- ✅ Local variable scopes
- ✅ String concatenation
- ✅ **Arrays with literals**
- ✅ **Array indexing and assignment**
- ✅ **Built-in functions (length, push)**

### Path to 1.0.0
1. Break and continue statements
2. Better error messages with line numbers
3. More array operations (pop, slice, etc.)
4. File I/O capabilities
5. Final polish and documentation

### Post 1.0.0
- Self-hosting (Whispem interpreter written in Whispem)
- Module system
- Complete language specification
- Tutorial and book

---

## v0.9.0 Milestone

With the addition of arrays in v0.9.0, Whispem has reached another critical milestone:

**Whispem is now feature-complete for general-purpose programming.**

You can now:
- Store and manipulate collections of data
- Build complex data structures
- Implement real algorithms (sorting, searching, etc.)
- Process lists of items
- Work with dynamic data

This is the **final major feature** before 1.0.0.

The remaining work is polish:
- Better error messages
- More built-in functions
- Quality-of-life improvements
- Documentation refinement

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

1. ✅ A beginner can learn the entire language in one day
2. ✅ The implementation can be read and understood in one sitting
3. ✅ Every feature has a clear justification
4. ✅ The documentation is complete and accessible
5. ✅ The language is useful for teaching programming concepts
6. ✅ The language is Turing-complete and practical
7. ✅ The language supports collections (arrays)
8. 🔄 The language has helpful error messages
9. 🔄 The language is self-hosted

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

**Version:** 0.9.0  
**Status:** Feature complete, ready for 1.0.0 polish  
**Philosophy:** Whisper, don't shout
