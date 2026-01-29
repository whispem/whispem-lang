# Changelog

All notable changes to **Whispem** will be documented in this file.

This project follows [Semantic Versioning](https://semver.org/).

---

## [0.3.0] - 2026-01-29

### Added
- Execution of `.wsp` source files via CLI
- File-based program loading
- Basic interpreter implementation
- Variable storage and resolution
- Support for `let` statements
- Support for `print` statements
- Comment support using `#`
- String and number literals
- Binary expressions in the AST
- Basic runtime environment

### Changed
- CLI behavior: running without arguments now prints `Whispem`
- Improved separation between lexer, parser, AST, and interpreter

### Fixed
- Proper handling of newlines and comments
- Lexer stability for strings and identifiers

### Known limitations
- No operator precedence yet
- Expressions are partially supported
- No runtime error handling
- Strings are not fully evaluated

---

## [0.2.0] - 2026-01-29

### Added
- Abstract Syntax Tree (AST) structure
- Initial parser implementation
- Support for variable declarations
- Support for numeric literals
- Support for basic expressions
- Example `.wsp` file

### Changed
- Internal architecture aligned around AST nodes
- Improved token handling

### Fixed
- Minor lexer inconsistencies

---

## [0.1.0] - 2026-01-28

### Added
- Initial project setup
- Core project structure
- Lexer implementation
- Token definitions
- Basic CLI entry point
- First executable binary

---
