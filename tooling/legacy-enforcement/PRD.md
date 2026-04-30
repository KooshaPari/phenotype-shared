# PRD.md — tooling/legacy-enforcement

## Overview

This PRD describes **legacy-enforcement**, a policy scanner that enforces the Phenotype Technology Adoption Philosophy across all Phenotype repositories. It detects usage of legacy tooling (pip, npm, Jest, tsc, Make, etc.) and provides structured guidance toward modern alternatives (uv, bun, tsgo, Taskfile).

---

## 1. Users & Use Cases

### Primary Users

| User | Role | Goals |
|------|------|-------|
| Platform Engineers | Repo governance | Ensure consistent tooling across repos |
| DevOps Engineers | CI/CD maintenance | Integrate policy gates into pipelines |
| Developers | Daily development | Understand what tooling is approved |
| Architects | Technology strategy | Track adoption metrics |

### Use Cases

**UC-1: CI/CD Policy Gate**
- A GitHub Actions workflow runs on every PR
- The scanner detects legacy tooling usage (e.g., `npm ci`)
- If violations meet `--fail-on-severity`, the workflow fails
- A SARIF report is uploaded as a build artifact
- PR receives a comment with finding summary

**UC-2: Pre-commit Hook**
- A developer runs `git commit`
- Pre-commit hook triggers scanner
- Report is generated without blocking commit
- Developer reviews output and fixes violations before push

**UC-3: Exception Management**
- A team has a legitimate reason for legacy tool usage
- They file an exception ticket (TICKET-123)
- Platform engineer approves exception in `exceptions.yaml`
- Scanner skips that rule/path combination until expiry

**UC-4: Reporting and Metrics**
- Platform team runs scanner across all repos
- JSON report aggregates findings
- Dashboards show adoption progress over time
- Quarterly review of exception trends

---

## 2. Problem Statement

### Pain Points

1. **Tooling Inconsistency**: Each Phenotype repo uses different package managers (poetry, pipenv, uv, npm, yarn), causing cognitive overhead
2. **CI Pipeline Drift**: Legacy tools in CI cause slower builds, inconsistent environments
3. **Migration Unknowns**: No clear picture of which repos need migration work
4. **Exception Chaos**: Ad-hoc exceptions without tracking or expiry
5. **Agent Confusion**: AI agents (per AGENTS.md) don't know which tools are approved

### Jobs to Be Done

| Job | Priority | Current State | Desired State |
|-----|----------|---------------|---------------|
| Detect legacy tooling | P0 | Manual review | Automated regex scanning |
| Enforce in CI | P0 | None | Policy gate on PRs |
| Track exceptions | P1 | None | Versioned exception registry |
| Generate compliance reports | P1 | Manual | Automated JSON/MD/SARIF |
| Provide migration guidance | P0 | None | Suggested fix per violation |

---

## 3. User Stories

### Story 1: CI Gate Enforcement
> As a DevOps engineer, I want the scanner to fail CI when legacy tooling is detected so that violations don't reach main.

**Acceptance Criteria**:
- [ ] Scanner runs in GitHub Actions via reusable workflow
- [ ] Exit code 2 when violations match `--fail-on-severity`
- [ ] SARIF report uploaded as artifact
- [ ] PR comment posted with finding summary

### Story 2: Tiered Enforcement
> As a platform engineer, I want Tier 0 repos blocked and Tier 1 repos warned so that we migrate safely without breaking active development.

**Acceptance Criteria**:
- [ ] `policy/rules.yaml` defines rule severity and mode (block/warn/allow)
- [ ] Tier 0 repos (AgilePlus, heliosCLI, etc.) get block mode
- [ ] Tier 1 repos get warn mode
- [ ] Tier 2 repos get allow mode (report only)

### Story 3: Exception Management
> As a team lead, I want to request an exception with a ticket and expiry date so that legitimate legacy usage is temporarily allowed.

**Acceptance Criteria**:
- [ ] Exception added to `policy/exceptions.yaml`
- [ ] Required fields: id, rule_id, repo, path, owner, ticket, rationale, expires_at
- [ ] Max duration 90 days enforced
- [ ] Expired exceptions are skipped automatically

### Story 4: Developer Self-Service
> As a developer, I want to run `docgen legacy-tooling-scan` locally so that I can fix violations before they reach CI.

**Acceptance Criteria**:
- [ ] `python3 scanner/legacy_tooling_scanner.py --repo-root .` works
- [ ] Markdown report generated locally
- [ ] Suggested fixes printed for each finding
- [ ] Exit code 0 in report-only mode

---

## 4. Requirements

### Functional Requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-1 | Regex-based pattern detection across file types | P0 |
| FR-2 | YAML policy rule loading | P0 |
| FR-3 | Severity levels: critical, high, medium, low | P0 |
| FR-4 | Enforcement modes: block, warn, allow | P0 |
| FR-5 | Exception registry with expiry | P1 |
| FR-6 | Multi-format output: JSON, Markdown, SARIF | P0 |
| FR-7 | GitHub Actions reusable workflow | P0 |
| FR-8 | Pre-commit hook integration | P1 |
| FR-9 | File line/column reporting | P0 |
| FR-10 | Suggested fix per violation | P0 |
| FR-11 | CI comment integration | P2 |

### Non-Functional Requirements

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-1 | Scan speed | < 1 second per repo |
| NFR-2 | Rule count | 20+ policy rules |
| NFR-3 | Language coverage | Python, JS/TS, Rust, Go, General |
| NFR-4 | False positive rate | < 1% |
| NFR-5 | Report size | < 1MB for typical repo |

---

## 5. Out of Scope

- Auto-fixing violations (future phase)
- IDE plugin integration (future phase)
- Non-GitHub CI systems (Jenkins, GitLab CI)
- Language server protocol integration
- Repository cloning/fetching

---

## 6. Policy Scope

### Python (uv primary)

| Legacy Tool | Banned Pattern | Recommended Replacement |
|-------------|----------------|------------------------|
| `python -m pytest` | LT-PY-001 | `uv run pytest` |
| `pip install -e` | LT-PY-002 | `uv pip install` or pyproject.toml |
| `poetry` | LT-PY-003 | `uv sync`, `uv run` |
| `pipenv` | LT-PY-004 | `uv` with pyproject.toml |
| `ruff` direct | LT-PY-005 | `uv run ruff` |
| `setup.py` | LT-PY-006 | `uv build` with pyproject.toml |
| `requirements.txt` | LT-GEN-004 | `uv.lock` with pyproject.toml |

### JavaScript/TypeScript (Bun primary)

| Legacy Tool | Banned Pattern | Recommended Replacement |
|-------------|----------------|------------------------|
| `npm ci` | LT-JS-001 | `bun install` |
| `npm run` | LT-JS-002 | `bun run` |
| `yarn` | LT-JS-003 | `bun` |
| `pnpm` | LT-JS-004 | `bun` |
| `tsc` (TypeScript 6.x) | LT-JS-005 | `tsgo` (TypeScript 7 native) |
| `jest` | LT-JS-006 | `bun test` or Vitest |
| `node` runtime | LT-JS-007 | `bun run` |

### General

| Legacy Pattern | Rule | Replacement |
|---------------|------|-------------|
| `make test/build` | LT-GEN-001 | `Taskfile.yml`, `justfile` |
| `ci.sh/build.sh` | LT-GEN-002 | Task runner commands |
| Mixed package managers | LT-GEN-003 | Single manager alignment |
| Files > 350/400 lines | LT-GEN-005 | Module refactoring |

---

## 7. Success Metrics

| Metric | Baseline | Target | Measurement |
|--------|----------|--------|-------------|
| Tier 0 compliance | 0% | 100% | Scanner reports |
| Legacy tool usage in CI | Unknown | -50% YoY | SARIF artifact analysis |
| Exception count | 0 | < 10 active | exceptions.yaml count |
| Developer self-service rate | 0% | > 80% | Local vs CI-only runs |
| Rule coverage | 5 rules | 20+ rules | rules.yaml row count |

---

## 8. Release Plan

### Phase 1: Warn Mode (Current)
- All Tier 0 repos scan in `--report-only` mode
- Reports uploaded as artifacts
- PR comments with findings
- No blocking of builds

### Phase 2: Strict Enforcement (Target: Q3 2026)
- Enable `fail-on-severity: high` for Tier 0 repos
- All exceptions must have valid tickets and expiry dates
- Migration backlog tracked per repo

### Phase 3: Expand to Tier 1
- Enable for remaining active repos
- Warn mode for repos in migration
- Automated migration guidance generation

---

## 9. Open Questions

| Question | Owner | Status |
|----------|-------|--------|
| Should we auto-create exception tickets? | Unassigned | Open |
| How to handle forks of Phenotype repos? | Unassigned | Open |
| Should LT-RS-001/Rust nightly be block mode? | Unassigned | Open |
| How to measure migration velocity? | Unassigned | Open |
| Should we support `--fix` auto-correction? | Unassigned | Future |
