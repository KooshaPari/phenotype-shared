# Contributing to phenotype-shared

Thank you for your interest in contributing to phenotype-shared!

## Development Setup

1. **Prerequisites**
   - Rust 1.75 or later
   - Cargo

2. **Clone the repository**
   ```bash
   git clone https://github.com/KooshaPari/phenotype-shared
   cd phenotype-shared
   ```

3. **Run tests**
   ```bash
   cargo test --workspace
   ```

4. **Run linter**
   ```bash
   cargo clippy --workspace -- -D warnings
   ```

5. **Format code**
   ```bash
   cargo fmt
   ```

## Pull Request Process

1. Fork the repository and create a feature branch from `main`.
2. Follow the commit message format: `<type>(<scope>): <description>`
   - Types: `feat`, `fix`, `chore`, `docs`, `refactor`, `test`, `ci`
3. Ensure all tests pass, clippy is clean, and code is formatted.
4. Update documentation if adding new features.
5. Submit a pull request with a clear description of changes.

## Code Standards

- Full type annotations required
- Document all public APIs
- No `unsafe` code without justification
- Keep files under 350 lines (hard limit 500)

## Crate Structure

Each crate in `crates/` should be independently consumable with no inter-crate dependencies.
