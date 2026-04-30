# phenotype-shared

**Status:** stable

Rust infrastructure toolkit extracted from the Phenotype ecosystem. The workspace now provides shared domain, application, port, and infrastructure crates that support hexagonal and clean architecture across the broader polyrepo.

## Crates

| Crate | Description |
|-------|-------------|
| [`ffi_utils`](crates/ffi_utils) | FFI helper utilities (regex caching, synchronization primitives) for cross-language bindings |
| [`phenotype-domain`](crates/phenotype-domain) | DDD value objects, entities, aggregates, events, and domain errors |
| [`phenotype-application`](crates/phenotype-application) | CQRS commands, queries, DTOs, and application handlers |
| [`phenotype-port-interfaces`](crates/phenotype-port-interfaces) | Hexagonal inbound/outbound port traits and shared contracts |
| [`phenotype-contracts`](crates/phenotype-contracts) | Async trait contracts and shared interface definitions across the ecosystem |
| [`phenotype-event-sourcing`](crates/phenotype-event-sourcing) | Append-only event store with hash-chain verification and snapshot management |
| [`phenotype-cache-adapter`](crates/phenotype-cache-adapter) | Two-tier cache with TTL expiration and observability hooks |
| [`phenotype-policy-engine`](crates/phenotype-policy-engine) | Rule-based policy evaluation engine with TOML config loading |
| [`phenotype-state-machine`](crates/phenotype-state-machine) | Generic finite state machine with transition guards and history tracking |
| [`phenotype-config-core`](crates/phenotype-config-core) | Core configuration types and traits for Phenotype crates |
| [`phenotype-error-core`](crates/phenotype-error-core) | Canonical error types for the Phenotype ecosystem (API, domain, repository, config, storage) |
| [`phenotype-health`](crates/phenotype-health) | Shared health check abstraction for Phenotype services |
| [`phenotype-postgres-adapter`](crates/phenotype-postgres-adapter) | PostgreSQL persistence adapter |
| [`phenotype-redis-adapter`](crates/phenotype-redis-adapter) | Redis persistence / cache adapter |
| [`phenotype-http-adapter`](crates/phenotype-http-adapter) | HTTP adapter and transport utilities |
| [`phenotype-nanovms-client`](crates/phenotype-nanovms-client) | Rust client library for NanoVMs unikernel orchestration (wraps the `ops` CLI) |

## Quick Start

Add any crate as a git dependency:

```toml
[dependencies]
phenotype-domain = { git = "https://github.com/KooshaPari/phenotype-shared" }
phenotype-application = { git = "https://github.com/KooshaPari/phenotype-shared" }
phenotype-port-interfaces = { git = "https://github.com/KooshaPari/phenotype-shared" }
phenotype-event-sourcing = { git = "https://github.com/KooshaPari/phenotype-shared" }
phenotype-cache-adapter = { git = "https://github.com/KooshaPari/phenotype-shared" }
phenotype-policy-engine = { git = "https://github.com/KooshaPari/phenotype-shared" }
phenotype-state-machine = { git = "https://github.com/KooshaPari/phenotype-shared" }
phenotype-config-core = { git = "https://github.com/KooshaPari/phenotype-shared" }
phenotype-error-core = { git = "https://github.com/KooshaPari/phenotype-shared" }
phenotype-health = { git = "https://github.com/KooshaPari/phenotype-shared" }
phenotype-postgres-adapter = { git = "https://github.com/KooshaPari/phenotype-shared" }
phenotype-redis-adapter = { git = "https://github.com/KooshaPari/phenotype-shared" }
phenotype-http-adapter = { git = "https://github.com/KooshaPari/phenotype-shared" }
```

## Usage Examples

### Event Sourcing

```rust
use phenotype_event_sourcing::{EventEnvelope, EventStore};
use phenotype_event_sourcing::memory::InMemoryEventStore;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct UserCreated { user_id: String, email: String }

let store = InMemoryEventStore::new();
let event = EventEnvelope::new(
    UserCreated { user_id: "u-1".into(), email: "a@b.com".into() },
    "system",
);
let seq = store.append(&event, "UserCreated", "u-1").unwrap();
assert_eq!(seq, 1);

// Retrieve events
let events = store.get_events::<UserCreated>("UserCreated", "u-1").unwrap();
assert_eq!(events.len(), 1);
```

### Cache

```rust
use phenotype_cache_adapter::TieredCache;
use std::time::Duration;

let cache = TieredCache::new(100, Duration::from_secs(300), None);
cache.insert("key".to_string(), "value".to_string());
assert_eq!(cache.get(&"key".to_string()), Some("value".to_string()));
```

### Policy Engine

```rust
use phenotype_policy_engine::{PolicyEngine, PolicyContext};

let engine = PolicyEngine::new();
engine.load_toml_str(r#"
[[rules]]
name = "require-auth"
action = "require"
field = "auth_token"
"#).unwrap();

let mut ctx = PolicyContext::new();
ctx.set("auth_token", "abc123");
let result = engine.evaluate(&ctx).unwrap();
assert!(!result.passed());
```

### State Machine

```rust
use phenotype_state_machine::{State, StateMachine};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
enum Status { Draft, Review, Approved, Done }

impl State for Status {
    fn ordinal(&self) -> u32 {
        match self { Self::Draft => 0, Self::Review => 1, Self::Approved => 2, Self::Done => 3 }
    }
}

let mut sm = StateMachine::new(Status::Draft);
sm.transition(Status::Review).unwrap();
assert_eq!(sm.current(), &Status::Review);
```

## Development

```bash
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

## Architecture

The workspace is organized as a layered hexagonal stack:

- **Domain**: `phenotype-domain` owns value objects, entities, aggregates, events, and domain errors.
- **Application**: `phenotype-application` coordinates commands, queries, DTOs, and application handlers.
- **Ports**: `phenotype-port-interfaces` holds technology-agnostic inbound and outbound contracts.
- **Infrastructure**: event sourcing, cache, policy, state machine, database, Redis, and HTTP adapters live below the core layers.

```
phenotype-shared/
  Cargo.toml
  crates/
    phenotype-domain/
    phenotype-application/
    phenotype-port-interfaces/
    phenotype-event-sourcing/
    phenotype-cache-adapter/
    phenotype-policy-engine/
    phenotype-state-machine/
    phenotype-postgres-adapter/
    phenotype-redis-adapter/
    phenotype-http-adapter/
```

## Consolidation opportunities

### Governance baseline PR scope

The initial governance PR for `phenotype-shared` should stay intentionally small and canonical:

- add a repo-level quality gate workflow that runs format, lint, build, and tests
- add an ADR note for governance, branch protection, and canonical repo policy
- add lightweight policy-gate checks for top-level repo layout and required governance files
- add architecture lint rules for boundary and dependency drift
- keep repo-specific behavior local; only extract cross-cutting rules into shared code

### Ports and shared contracts

- `phenotype-shared/crates/phenotype-port-interfaces` is the canonical home for domain-agnostic traits and value objects.
- Repo-local port layers such as `phench/src/phench/application/ports.py` and `worktree-manager/src/worktree_manager/ports/mod.rs` should keep their names and responsibilities aligned with the shared port vocabulary.
- Prefer extracting only truly generic contracts into the shared ports crate; keep workflow- or repo-specific ports local.

### Extraction priorities

- **First**: pagination primitives and response wrappers shared across application/query boundaries.
- **Next**: error mapping helpers, keeping domain-specific errors local.
- **Then**: reusable infrastructure primitives that already repeat across repos.

### Repo structure and worktree placement

- Keep canonical top-level buckets explicit: `apps/`, `libs/`, `infrastructure/`, `governance/`, `tooling/`, `templates/`.
- Keep active worktrees under `repos/worktrees/<project>/<category>/<wtree>` so canonical repo roots stay clean.
- Prefer shallow, discoverable top-level directories over deeply nested ad hoc layouts.

### Errors and failure taxonomy

- Keep domain-specific exceptions local to each crate or package, such as `phench/src/phench/domain/exceptions.py` and `worktree-manager/src/worktree_manager/domain/errors.rs`.
- Extract only cross-cutting, technology-neutral error categories into shared layers, such as validation/storage/not-found/permission state errors.
- Avoid creating parallel generic error enums in multiple repositories unless they serve different bounded contexts.

MIT
