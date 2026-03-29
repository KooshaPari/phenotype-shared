# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-03-28

### Added
- Event sourcing crate with event store abstractions and verification chain support
- Policy engine crate structure for domain policy management
- User journeys specification and workspace ergonomics requirements
- Hexagonal architecture adapter crates for clean architecture support
- @phenotype/docs shared VitePress theme integration

### Fixed
- EventSourcingError coercion in EventStore verify_chain method
- Dead code warnings on StoredEvent.event_type field (kept for future projection support)
- FFI utils unused imports causing cargo warnings
- TDD test failures in domain layer modules
- Cargo check, test, and doctest compatibility across shared crates

### Changed
- Migrated kitty-specs to docs/specs in AgilePlus format
- Refined hexagonal architecture specification to language-agnostic format
- Enhanced docs-site with VitePress 1.6 scaffold and verification harness

## [0.2.0] - 2026-02

### Added
- Language-agnostic hexagonal architecture specification
- Comparison matrix documentation (shared with phenotype-infrakit)
- Governance files (CODEOWNERS, CI workflow)
- VitePress docsite scaffolding with home page and sidebar configuration
- CLAUDE.md project guidelines

### Fixed
- CI workflows to skip billable runner configurations
- Workspace cargo check issues across all crates

## [0.1.0] - 2026-01

### Added
- Initial phenotype-shared crate with foundational shared types
- Domain layer with core entities and value objects
- Repository pattern abstractions
- FFI utilities for interop with C/C++ code
- Basic CI/CD pipeline with publishing configuration

[Unreleased]: https://github.com/KooshaPari/phenotype-shared/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/KooshaPari/phenotype-shared/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/KooshaPari/phenotype-shared/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/KooshaPari/phenotype-shared/releases/tag/v0.1.0
