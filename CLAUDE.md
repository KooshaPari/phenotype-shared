# phenotype-infrakit

## Project

Rust workspace containing generic infrastructure crates extracted from the Phenotype ecosystem.
Each crate is independent, domain-agnostic, and can be consumed individually.

## Stack

- **Language**: Rust (edition 2021)
- **Build**: cargo (workspace)
- **Test**: `cargo test --workspace`
- **Lint**: `cargo clippy --workspace`
- **Format**: `cargo fmt`

## Structure

```
crates/
  phenotype-event-sourcing/   # Append-only event store with SHA-256 hash chains
  phenotype-cache-adapter/    # Two-tier LRU + DashMap cache with TTL
  phenotype-policy-engine/    # Rule-based policy evaluation with TOML config
  phenotype-state-machine/    # Generic FSM with transition guards
```

## Conventions

- All public types implement `Debug`, `Clone` where possible
- Error types use `thiserror` with `#[from]` for conversions
- Serialization via `serde` with `Serialize`/`Deserialize` derives
- No inter-crate dependencies; each crate stands alone
- Workspace-level dependency versions in root `Cargo.toml`
- Tests are inline (`#[cfg(test)]` modules) within each source file

## Adding a New Crate

1. Create `crates/<name>/` with `Cargo.toml` and `src/lib.rs`
2. Add to `members` in root `Cargo.toml`
3. Use `workspace = true` for shared dependencies
4. Include inline tests with `#[cfg(test)]`
5. Update `README.md` crate table
