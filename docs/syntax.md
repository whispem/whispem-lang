# Whispem v0.2.0 — Syntax

This document describes the syntax supported by Whispem version 0.2.0.

---

## Programs

A Whispem program is a sequence of statements.
Each statement is written on its own line.

---

## Variables

Variables are declared using the `let` keyword.

    let x = 10
    let name = "Whispem"

Variables are immutable in this version.

---

## Print

The `print` statement outputs the value of an expression.

    print x
    print name

---

## Types

Whispem currently supports two data types:

- Number (floating-point)
- String

---

## Expressions

In v0.2.0, expressions can be:

- numeric literals
- string literals
- variable references

Complex expressions are not supported yet.

---

## Comments

Comments start with `#` and continue until the end of the line.

    # This is a comment
    let version = "0.2.0"

---

## Limitations

Whispem v0.2.0 does not support:

- arithmetic operations
- conditionals
- loops
- functions

These limitations are intentional.
