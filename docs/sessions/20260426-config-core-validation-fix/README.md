# Config Core Validation Fix

## Goal

Unblock focused `phenotype-config-core` validation after workspace formatter and
compiler runs failed on `crates/phenotype-config-core/src/lib.rs`.

## Changes

- Repaired the malformed `merge_configs` return expression.
- Made `ConfigLoader` object-safe by moving dynamic dispatch to `load_value()`.
- Preserved typed loading for concrete loaders through `load<T>()`.
- Declared the crate dependencies it already used: `serde_json`, `serde_yaml`,
  `thiserror`, and `toml`.
- Fixed `ConfigSource::default_source` to construct the `Priority` newtype.

## Validation

```bash
cargo fmt --check -p phenotype-config-core
cargo check -p phenotype-config-core
cargo test -p phenotype-config-core
cargo clippy -p phenotype-config-core -- -D warnings
```
