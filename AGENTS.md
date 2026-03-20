# AGENTS.md — Protorun Language Project

## Project Overview

Protorun is a new programming language combining functional programming, a strong type system, and algebraic effects. Influences: Scala, Haskell, Rust, Koka, Effekt. **This is currently a spec-only project** — no implementation code exists yet. The parser will be implemented in Rust using the Nom parser combinator library.

**Current phase**: MVP specification is complete. Next phase is parser/interpreter implementation.

## Repository Structure

```
docs/
  language-spec.md          # Table of contents
  language-spec/
    01-introduction.md      # Design philosophy, priorities
    02-lexical-structure.md # Keywords, operators, literals
    03-type-system.md       # Types (i8-i128, u8-u128, f32/f64, generics [T])
    04-declarations-and-statements.md  # let/let mut, type, trait, impl, effect, handler, return, while/for/loop
    05-expressions.md       # if/match, string interpolation, UFCS, patterns, literals
    06-algebraic-effects.md # Core feature: effect/handler/with, break/continue as effects
    07-modules.md           # module/import/export, entry point (top-level exprs)
    08-standard-library.md  # Minimal prelude: Option, Result, List, Map, Iterator, Console
    09-examples.md          # Sample programs
    10-grammar.md           # Canonical EBNF grammar (SOURCE OF TRUTH)
.clinerules                 # Project rules (see below)
.sisyphus/                  # Work session artifacts (plans, evidence, notepads)
mise.toml                   # Tool versions (rust = latest)
```

## Build / Test / Lint Commands

No implementation exists yet. When the Rust codebase is created:

```bash
# Build
cargo build

# Run all tests
cargo test

# Run a single test by name
cargo test test_name

# Run tests in a specific module
cargo test module_name::

# Check without building
cargo check

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt

# Format check (CI)
cargo fmt -- --check
```

## Language Spec Conventions (CRITICAL for all agents)

### 10-grammar.md is the Source of Truth

When prose in other chapters contradicts the EBNF in `10-grammar.md`, **the EBNF is correct**. Update the prose.

### Syntax Rules (enforced across ALL spec files)

| Element | Correct | WRONG |
|---------|---------|-------|
| Generics | `Option[T]`, `List[Int]` | `Option<T>`, `List<Int>` |
| Boolean literals | `True`, `False` | `true`, `false` |
| Function body | `(x: Int) -> Int = x + 1` | `(x: Int) -> Int => x + 1` |
| Return type | `(x: Int) -> Int` | `(x: Int): Int` |
| If expression | `if cond then { ... }` | `if cond { ... }` |
| Map literal | `{"key" : value}` | `{"key" -> value}` |
| String interpolation | `f"Hello {name}"` | `s"Hello ${name}"` |
| Match arm | `pattern => expr` | (this one IS `=>`) |
| Index access | `list.get(0)` | `list[0]` |
| Mutable field | `mut count: Int` | `let mutable count: Int` |

### Features NOT in MVP (do NOT add)

- Ownership system (`own`, `&`, `&mut`, lifetimes) — deferred to future phase
- `?` error propagation operator
- List patterns / spread (`[head, ...tail]`, `...`)
- User-defined operator overloading
- Concurrency / async
- FFI
- Annotations / attributes

## Code Style (for future Rust implementation)

### General

- Functional programming style preferred; minimize side effects
- Type safety is paramount — no `unwrap()` in production code
- Low coupling, high cohesion — continuously refactor toward this
- Delete unused code

### Rust-Specific

- Use Nom parser combinators for declarative parsing
- For comparison operators: parse 2-char operators (`<=`, `>=`) before 1-char (`<`, `>`)
- Error messages must include context information
- Follow `cargo clippy` recommendations strictly

### Naming

- Types/Traits: `PascalCase` (e.g., `Expression`, `TypeRef`)
- Functions/methods: `snake_case` (e.g., `parse_expression`, `eval_literal`)
- Constants: `SCREAMING_SNAKE_CASE`
- Match EBNF rule names in parser function names where possible

### Testing (TDD)

- Write tests BEFORE implementation
- Test individual functions bottom-up
- Small incremental changes — commit when tests pass
- On test failure: add debug output, identify root cause from facts, fix precisely
- Mock component functions when testing larger behaviors

## Git Conventions

- **Communication**: Japanese
- **Commit messages**: English with conventional prefixes
  - `feat:`, `fix:`, `refactor:`, `test:`, `docs:`, `chore:`
- **Commit granularity**: One logical change per commit (feature part, bug fix, refactor, test)
- **Branch**: Currently working on `master`/`main`; branch strategy TBD for multi-dev

## Key Design Decisions (MVP)

1. **Algebraic effects** are the core differentiator — `effect`/`handler`/`with` syntax
2. **GC-based memory** for MVP; ownership system planned for future phase
3. **`[T]` generics** (Scala-style) — `[]` reserved for generics, NOT for indexing
4. **Top-level expressions** as entry point — no `main()` function required
5. **`break`/`continue` as algebraic effects** — loops provide implicit handlers
6. **Explicit `with` required** at top-level for effect operations (no implicit handlers)
7. **`=` for function body** (not `=>`); `=>` only for match arms
8. **`f"...{expr}..."` string interpolation** (Python-style, not Scala `s"..."`)
9. **Sized numeric types**: `i8`–`i128`, `u8`–`u128`, `f32`/`f64`; `Int` = `i64`, `Float` = `f64`
10. **Iterator protocol**: `Iterator[T]`/`Iterable[T]` traits; `for` desugars to `.iter()` + `.next()`

## Spec Editing Workflow

1. Change EBNF in `10-grammar.md` first
2. Update prose in relevant chapters to match
3. Update examples in `09-examples.md`
4. Verify no contradictions: `grep` for old syntax across all `docs/language-spec/*.md`
5. Update changelog in `docs/language-spec.md`

## Known Challenges

- Balancing language feature complexity vs. simplicity
- Algebraic effects + ownership system integration (future phase)
- Type/effect/ownership inference interactions
- Parser performance optimization for large source files
- Improving error messages with specific, helpful context
