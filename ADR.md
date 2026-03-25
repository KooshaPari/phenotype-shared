# Architecture Decision Records - phenotype-shared

## ADR-001: Workspace with Independent Crates
**Status**: Accepted
**Context**: Shared infrastructure must be consumable per-crate without pulling the full workspace.
**Decision**: Cargo workspace with independent crates sharing only workspace-level dep versions.
**Consequences**: Clean dependency graphs; individual crate versioning possible.

## ADR-002: phenotype-dev Organization Scope
**Status**: Accepted
**Context**: Shared crates serve the broader Phenotype dev organization, not just one project.
**Decision**: Publish under `phenotype-dev` org; separate from project-specific repos.
**Consequences**: Clear ownership; reuse across all Phenotype projects.
