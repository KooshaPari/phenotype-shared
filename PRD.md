# PRD - phenotype-shared

## E1: Shared Infrastructure Crates

### E1.1: Event Sourcing
Append-only event store with SHA-256 hash chain verification, snapshot management, and pluggable storage backends for the Phenotype ecosystem.

### E1.2: Cache Adapter
Two-tier cache (L1 LRU + L2 DashMap) with TTL expiration and pluggable metrics hooks.

### E1.3: Policy Engine
Rule-based policy evaluation engine with allow/deny/require rules, TOML config loading, and severity levels.

### E1.4: State Machine
Generic finite state machine with transition guards, forward-only enforcement, and history tracking.

**Acceptance**: All crates independently consumable, fully tested, zero inter-crate dependencies.
