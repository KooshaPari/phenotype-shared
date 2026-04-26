# ADR-002: Tiered Maturity Enforcement Model

**Status**: Accepted

**Date**: 2026-04-04

**Context**: legacy-enforcement must enforce policy across repositories at different stages of the Phenotype ecosystem. Some repos (AgilePlus, heliosCLI) are actively maintained and ready for strict enforcement, while others may be in migration or legacy status. We need a tiered model that enables progressive enforcement without breaking active development.

---

## Decision Drivers

| Driver | Priority | Notes |
|--------|----------|-------|
| Safety | High | Don't break CI for active development repos |
| Migration enablement | High | Allow gradual migration with visibility |
| Consistency | High | Same rules apply everywhere |
| Operational simplicity | Medium | Easy to understand and configure |
| Measurability | Medium | Track adoption progress |

---

## Options Considered

### Option 1: Single-tier block mode for all repos

**Description**: All repos must pass all policy checks or CI fails.

**Pros**:
- Simplest model to understand
- Maximum enforcement consistency
- Clear pass/fail criteria

**Cons**:
- Breaks CI for repos mid-migration
- No grace period for legitimate exceptions
- Risk of developer frustration and workarounds

**Performance Data**:
| Metric | Value | Notes |
|--------|-------|-------|
| Configuration complexity | None | Single flag |
| Exception handling | None | Not supported |
| Migration support | None | All or nothing |

### Option 2: Tiered enforcement (block/warn/allow)

**Description**: Repos are assigned to tiers with different enforcement levels.

**Pros**:
- Progressive enforcement based on repo readiness
- Block mode for mature repos, warn mode for migrating
- Allows exceptions with tracking
- Clear migration path

**Cons**:
- More complex configuration
- Tiers must be maintained as repos evolve
- Some inconsistency in enforcement

**Performance Data**:
| Metric | Value | Notes |
|--------|-------|-------|
| Configuration complexity | Medium | Tier definitions per repo |
| Exception handling | Full | exceptions.yaml registry |
| Migration support | Full | Warn → Block progression |

### Option 3: Global warn mode with opt-in block

**Description**: Default is warn-only. Repos explicitly opt into block mode.

**Pros**:
- Conservative approach (no breaking changes by default)
- Easy to opt in when ready
- Gradual rollout possible

**Cons**:
- Violations can accumulate without consequences
- Less urgency to migrate
- Requires active opt-in management

**Performance Data**:
| Metric | Value | Notes |
|--------|-------|-------|
| Configuration complexity | Low | Default is warn |
| Exception handling | Implicit | None explicit |
| Migration support | Weak | No pressure |

---

## Decision

**Chosen Option**: Option 2 — Tiered enforcement (block/warn/allow) with explicit tier assignments.

**Rationale**: A tiered model provides the right balance between safety and enforcement. It allows strict enforcement in mature repos (Tier 0) while providing visibility and a migration path for repos still transitioning (Tier 1). The explicit `exceptions.yaml` registry ensures legitimate exceptions are tracked with accountability (owner, ticket, expiry).

This mirrors industry practices like SOC 2 compliance tiers and security maturity models, which recognize that not all systems are at the same level of readiness.

**Evidence**:
- Cloud provider shared responsibility models use tiered approaches
- Kubernetes uses graduated pod security standards
- PCI DSS uses compliance levels based on transaction volume

---

## Performance Benchmarks

```bash
# Example: Tier 0 repo (AgilePlus) enforcement
python3 tooling/legacy-enforcement/scanner/legacy_tooling_scanner.py \
  --repo-root /path/to/AgilePlus \
  --fail-on-severity high
# Expected: Exit code 2 if violations found (block mode)

# Example: Tier 1 repo (in migration) enforcement
python3 tooling/legacy-enforcement/scanner/legacy_tooling_scanner.py \
  --repo-root /path/to/migrating-repo \
  --fail-on-severity high \
  --treat-as-tier 1
# Expected: Exit code 0 (warn mode), violations reported
```

**Results**:

| Tier | Enforcement | CI Effect | Exception Support | Example Repos |
|------|-------------|-----------|-------------------|---------------|
| Tier 0 | block | Fail CI | Yes (90-day max) | AgilePlus, heliosCLI |
| Tier 1 | warn | CI passes | Yes (90-day max) | (none yet) |
| Tier 2 | allow | CI passes | Report only | (none yet) |

---

## Implementation Plan

- [ ] Phase 1: Define tier structure in rules.yaml — Target: 2026-04-04 (done)
- [ ] Phase 2: Implement `--treat-as-tier` CLI flag — Target: 2026-04-11
- [ ] Phase 3: Document tier migration path — Target: 2026-04-18
- [ ] Phase 4: Create dashboard for tier visibility — Target: Future

---

## Consequences

### Positive

- Safe enforcement rollout (Tier 0 first)
- Clear migration path for Tier 1 → Tier 0
- Tracked exceptions with accountability
- Measurable adoption metrics per tier

### Negative

- More complex configuration than single-tier
- Requires ongoing tier maintenance as repos evolve
- Potential for "tier inflation" if not managed

### Neutral

- Tier assignments are documented in rules.yaml
- Can be overridden via CLI flags for testing
- Tier progression is documented per repo

---

## References

- [NIST Cybersecurity Framework tiers](https://csf.tips/ tiers) - Industry tier model reference
- [Gartner application security maturity](https://www.gartner.com) - Maturity model concepts
- [Google Workspace shared drive tiering](https://support.google.com) - Tiered access model example
- [AWS Well-Architected pillars](https://aws.amazon.com/architecture/well-architected/) - Tiered best practices
- [Phenotype Technology Adoption Philosophy](../../CLAUDE.md) - Source policy for enforcement
