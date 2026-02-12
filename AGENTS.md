# Agent Guidelines

## Philosophy

- Keep code minimal; don't add features, flags, or abstractions unless explicitly asked
- Err on the side of creating a plan before implementing
- Avoid defensive code; don't add conditionals or checks "just in case"
- Let errors panic and fail rather than handling hypothetical upstream issues
- Don't proactively create helper methods; prefer inline code unless a code path is used multiple times
- Only extract helpers when there is explicit, demonstrated reuse - not "just in case"

## Working Method

- Always default to planning; only write code after the user is happy and has explicitly signed off.

## Code Style

- `mod.rs` files should only contain module declarations and re-exports
- Do not put logic or implementations in `mod.rs`
- Use singular names for files (e.g., `macro.rs` not `macros.rs`)
- Prefer destructuring references vs. dereferencing them as-needed
- Prefer to use explicit `return` keywords wherever possible
- Tests must be in separate test modules, never inline
- Use `foo/test.rs` alongside `foo/bar.rs`, not `#[cfg(test)]` blocks within implementation files
- Only use functional combinators for pure transformations; if the body has side effects or mutates external state, use loops instead
- Prefer named constants at the top of the file over magic numbers inline
- No single-line `if` bodies; always use braces on the next line
- Prefer information hiding: keep fields private by default, use read-only accessors across module boundaries, and only expose mutation when explicitly needed.
- Use `cargo fmt` to ensure code is correctly formatted
- Use `cargo clippy` to ensure we don't introduce *unexpected* warnings
