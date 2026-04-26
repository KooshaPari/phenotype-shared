# Legacy Tooling Anti-Pattern Enforcement

Enforces the [Phenotype Technology Adoption Philosophy](../../CLAUDE.md#technology-adoption-philosophy) across all repositories.

## Quick Start

```bash
# Run scanner on current repo
python3 tooling/legacy-enforcement/scanner/legacy_tooling_scanner.py \
  --repo-root . \
  --policy tooling/legacy-enforcement/policy/rules.yaml \
  --fail-on-severity high

# Report only (don't fail)
python3 tooling/legacy-enforcement/scanner/legacy_tooling_scanner.py \
  --repo-root . \
  --report-only
```

## What Gets Enforced

### Python (uv primary, pip/poetry/pipenv banned)
- ❌ `python -m pytest` → Use `uv run pytest`
- ❌ `pip install -e` → Use `uv pip install` or pyproject.toml
- ❌ `poetry install` → Use `uv sync`
- ❌ `poetry run` → Use `uv run`
- ❌ `pipenv` → Migrate to uv
- ❌ `setup.py` → Use `uv build` with pyproject.toml

### JavaScript/TypeScript (Bun primary, npm/yarn/pnpm banned)
- ❌ `npm ci` → Use `bun install`
- ❌ `npm run` → Use `bun run`
- ❌ `yarn` → Use `bun`
- ❌ `pnpm` → Use `bun`
- ❌ `tsc` → Use `tsgo` (TypeScript 7 native)
- ❌ `jest` → Use `bun test`

### General
- ❌ `make test/build/lint` → Use `Taskfile.yml` or `justfile`
- ❌ `ci.sh/build.sh` → Use standardized task runners
- ❌ Mixed package managers → Align to single canonical manager
- ❌ Files >350/400 lines → Refactor into modules

### Rust
- ⚠️ `channel = "stable"` → Consider nightly with edition 2024
- ⚠️ `edition = "2021"` → Upgrade to edition 2024
- ⚠️ `Pin<Box<dyn Future>>` → Use `async fn` in traits

### Go
- ⚠️ `go 1.23` or lower → Upgrade to go 1.24+

## Maturity Tiers

| Tier | Repos | Enforcement | Description |
|------|-------|-------------|-------------|
| **Tier 0** | AgilePlus, thegent, Tracera, phenoSDK, heliosCLI | `block` | Aggressive adoption - strict enforcement |
| **Tier 1** | (none yet) | `warn` | Standard repos - warnings allowed |
| **Tier 2** | (none yet) | `allow` | Legacy repos - report only |

## Policy Configuration

Rules are defined in `policy/rules.yaml`:

```yaml
rules:
  - id: LT-PY-001
    name: "Direct pytest invocation"
    severity: high
    mode: block  # or warn, allow
    applies_to: [python]
    patterns:
      - regex: '\\bpython\\s+-m\\s+pytest\\b'
    files:
      - ".github/workflows/*.yml"
    suggested_fix: "Use 'uv run pytest'"
```

## Exceptions

To request an exception, add to `policy/exceptions.yaml`:

```yaml
exceptions:
  - id: EXC-001
    rule_id: LT-PY-001
    repo: "my-repo"
    path_glob: ".github/workflows/ci.yml"
    command_regex: "pytest"
    owner: "team-platform"
    ticket: "TICKET-123"
    rationale: "Migration in progress to uv-based workflow"
    expires_at: "2026-06-01"
    replacement_plan: "Update to 'uv run pytest' by end of Q2"
```

**Exception Policy:**
- Max duration: 90 days
- Required fields: id, rule_id, repo, owner, ticket, rationale, expires_at, replacement_plan
- Review team: platform-engineering

## CI Integration

### GitHub Actions (Reusable Workflow)

```yaml
jobs:
  legacy-tooling:
    uses: phenotype/repos/.github/workflows/reusable-legacy-tooling-gate.yml@main
    with:
      fail-on-severity: high
      report-only: true  # Set false after migration
```

### Standalone Workflow

See `template-standalone-workflow.yml` in this directory.

### Pre-commit Hook

Add to `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: local
    hooks:
      - id: legacy-tooling-scan
        name: Legacy Tooling Scanner
        entry: python3 tooling/legacy-enforcement/scanner/legacy_tooling_scanner.py
        language: system
        pass_filenames: false
        always_run: true
```

## Rollout Strategy

### Phase 1: WARN Mode (Current)
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

## Report Output

The scanner generates three artifacts:

1. **JSON** (`legacy-tooling-report.json`) - Machine-readable for integrations
2. **Markdown** (`legacy-tooling-report.md`) - Human-readable for PR comments
3. **SARIF** (`legacy-tooling-report.sarif`) - GitHub Advanced Security integration

## Exit Codes

- `0` - No violations found (or --report-only)
- `1` - Scanner error
- `2` - Violations found meeting fail-on-severity threshold

## Related Documentation

- [CLAUDE.md - Technology Adoption Philosophy](../../CLAUDE.md)
- [AGENTS.md - Agent Rules and Constraints](../../AGENTS.md)
- [Architecture Decision Records](../../ADR.md)
