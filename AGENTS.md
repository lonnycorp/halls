# Agent Guidelines

## Philosophy

- Keep code minimal; do not add features, flags, or abstractions unless asked.
- Avoid defensive code and "just in case" checks.
- Let errors bubble up unless there is a clear reason to handle them.

## Layout

- `mod.rs` files contain declarations and re-exports only.
- Do not put logic or implementations in `mod.rs`.
- Prefer module boundaries via `mod.rs`; inside module trees, use `pub` over `pub(super)`.
- For large split features, define one shared parent `state.rs` struct (`*State`).
- Keep shared cross-cutting data in that state; keep sub-module-specific data local.
- Sub-modules use shared state via `&FooState` / `&mut FooState`.
- Use singular file names.
- Tests live in separate modules (`foo/test.rs`), never inline `#[cfg(test)]`.
- Within a file, put dependencies before dependents.
- Define types/errors/helpers before public functions that use them.
- Only extract helper methods when reused.

## Naming

- Unless a module is purely organizational, namespace public APIs by parent module (`foo/bar` -> `FooBar`).
- Shared module state structs must be suffixed with `State`.
- If a function takes too many arguments, group them into one `Params`-suffixed struct.
- Avoid generic names like `State` for shared/public types.
- Prefer object-verb function names (`foo_get` over `get_foo`).
- For noun/adjective-only names, use broader-to-narrower order (`context_item`).
- In PascalCase, keep acronyms fully capitalized (`URLParse`, `GLTFMesh`).
- For concise read-only accessors, `_get` may be implied by context (`foo()` over `foo_get()`).
- For `Result` returns, define `{Type}{Method}Error`.

## Visibility

- Prefer information hiding at externally visible module boundaries.
- Default to private fields with read-only accessors.
- Use `pub` fields only for data structs where direct mutation is expected.
- If unsure, keep fields private and expose the smallest surface needed.

## Aesthetics

- Prefer explicit `return` where possible.
- Use functional combinators only for pure transformations.
- If code has side effects or external mutation, use explicit loops/match blocks.
- Avoid side effects in expression-style assignments.
- Prefer named constants over inline magic numbers.
- No single-line `if` bodies; always use braces on the next line.

## Validation

- Run `cargo fmt`.
- Run `cargo clippy`; avoid unexpected warnings.
