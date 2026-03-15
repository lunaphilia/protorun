# AGENTS.md

## Project Overview

Protorun is a programming language combining functional programming, static typing, algebraic data types, traits, and algebraic effects. Currently in the **specification phase** — no compiler/interpreter source code exists yet. The implementation will be in Rust using the Nom parser combinator library.

## Repository Structure

```
docs/language-spec.md   # Consolidated language specification (Japanese)
mise.toml               # Toolchain: Rust (latest via mise)
.clinerules             # Project conventions (Japanese)
.gitignore              # Ignore rules
```

## Toolchain

- **Runtime management**: `mise` (not brew/apt). Run `mise install` after cloning.
- **Language**: Rust (`mise.toml` pins `rust = "latest"`)
- **Parser library**: Nom (parser combinators) — to be used when implementation begins
- **Repository management**: `ghq` (cloned under `~/ghq/`)

## Build / Test / Lint Commands

No implementation exists yet. When Rust source is added:

```bash
cargo build              # Build
cargo test               # Run all tests
cargo test test_name     # Run a single test by name
cargo test module::name  # Run tests in a specific module
cargo clippy             # Lint
cargo fmt                # Format
cargo fmt -- --check     # Check formatting without modifying
```

## Communication & Language Rules

- **All communication with the user**: Japanese
- **Commit messages**: English, with conventional commit prefixes (`feat:`, `fix:`, `refactor:`, `test:`, `docs:`, `chore:`)
- **Specification document**: Japanese
- **Code comments**: Follow Rust conventions (English preferred for code, Japanese acceptable for design notes)

## Git Workflow

- Branch: Currently single-branch development on `master`
- Remote: SSH (`git@github.com:lunaphilia/protorun.git`)
- Commits: Logical units — one feature/fix/refactor per commit
- TDD cycle: Small changes, test after each, commit when green
- Always keep spec and implementation in sync: changing one requires updating the other

## Development Philosophy

1. **Functional-first**: Prefer functional style, minimize side effects
2. **Type safety**: Design for strong static typing
3. **TDD**: Write tests bottom-up per function; mock dependencies for integration tests
4. **Spec-implementation consistency**: Any spec change must reflect in code and vice versa
5. **Present choices**: When multiple approaches exist, present options to the user rather than deciding alone
6. **Clean code**: Remove unused code; maintain low coupling, high cohesion
7. **Incremental**: Start with interpreter, gradually add compiler features

## Code Style (Rust — for future implementation)

### Naming
- Types, traits, effects: `PascalCase`
- Functions, variables, modules: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Generic parameters: single uppercase letters (`T`, `U`, `E`)

### Error Handling
- Use `Result<T, E>` — never panic in library code
- Error messages must include context (what was being parsed, where)
- Custom error types over string errors

### Parser Implementation (Nom)
- Declarative combinator style
- Multi-character operators (`<=`, `>=`) must be tried before single-character ones (`<`, `>`)
- Each parser function should be unit-testable in isolation
- Organize parsers into separate modules by grammar category (expressions, declarations, types, etc.)

### Imports
- Group: std → external crates → local modules
- Prefer explicit imports over glob imports

### Testing
- Test files: alongside source in same module or in `tests/` directory
- One test per parser rule / AST node behavior
- Test failure debugging: add debug output to identify root cause, then fix precisely

### Formatting
- `cargo fmt` (rustfmt defaults)
- No manual formatting overrides

## Language Design Reference

The full language spec is at `docs/language-spec.md`. Key design points:

- **`let`-unified declarations**: Values, functions, types, traits, effects, aliases all bound via `let`
- **Expression-based**: Type/trait/effect definitions are expressions
- **`impl` is separate**: `impl Trait for Type` / `impl Effect for Type`
- **Algebraic effects**: `effect` defines interface, functions declare `(effect alias: EffectType)` params, `with` provides handlers
- **No semicolons**: Newlines separate statements
- **No null**: Use `Option<T>` instead
- **Pattern matching**: `match` with exhaustiveness checking
- **Variant types**: `type<T> { A(T) | B | C{...} }` with `|` separator
- **Ownership modifiers**: `own`, `&`, `&mut` (semantics TBD)

## Known Challenges

- Balancing language feature complexity vs simplicity
- Integrating algebraic effects with the ownership system
- Interaction between type inference, effect inference, and ownership inference
- Parser performance optimization for large source files
- Producing helpful, context-rich error messages

## Anti-patterns to Avoid

- Do not suppress type errors (`as any`, `unwrap()` without justification, `#[allow(unused)]` broadly)
- Do not write large monolithic parser functions — keep them small and composable
- Do not commit broken tests — fix or remove before committing
- Do not refactor while fixing bugs — separate concerns into separate commits
- Do not add dependencies without discussing with the user first
