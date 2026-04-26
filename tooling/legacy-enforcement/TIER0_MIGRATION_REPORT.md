# Tier 0 Strict Repos - Legacy Tooling Migration Report
**Generated:** 2026-04-04  
**Policy Version:** 1.0.0  
**Enforcement Mode:** WARN (ready for strict)

## Executive Summary

| Repo | Stacks | Critical | High | Medium | Status |
|------|--------|----------|------|--------|--------|
| AgilePlus | Rust, General | 0 | 0 | 558 | ✅ Ready for strict |
| Tracera | Python, JS, General | 0 | 0 | 3109 | ✅ Ready for strict |
| thegent | TypeScript, Python, General | 0 | 0 | 854 | ✅ Ready for strict |
| phenoSDK | Python, General | 0 | 0 | 468 | ✅ Ready for strict |
| heliosCLI | Python, Rust, JS, General | 0 | 0 | 1015 | ✅ Ready for strict |

**All Tier 0 repos have ZERO critical or high severity tooling violations.**

All violations are **LT-GEN-005 (file size)** - files exceeding 350-400 line recommendations.
No legacy tooling (make, npm, poetry, pip install -e, etc.) detected.

## Detailed Findings

### LT-GEN-005: File Size Violations (Medium Severity)

**Python files (>350 lines):**
- `scripts/backfill_live_ledger.py`
- `python/src/agileplus_mcp/grpc_client.py`
- Various test files

**TypeScript/JavaScript files (>350 lines):**
- `sdk/typescript/tests/run.test.ts`
- `sdk/typescript/src/exec.ts`

**Rust files (>400 lines):**
- `crates/agileplus-subcmds/src/events.rs`
- `crates/agileplus-sqlite/src/lib.rs`

## Enforcement Rollout Plan

### Phase 1: WARN Mode (Current - 2026-04-04)
- ✅ All repos scanning successfully
- ✅ Reports uploaded as artifacts
- ✅ PR comments enabled
- ✅ No build blocking

### Phase 2: Strict Enforcement (Target: 2026-04-11)
Enable blocking mode for all Tier 0 repos since they have zero high/critical violations.

To enable, change in each repo's `.github/workflows/legacy-tooling-gate.yml`:
```yaml
- name: Fail on critical/high (after migration period)
  if: false  # Change to: if: always()
```

### Phase 3: Expand to Tier 1 (Target: 2026-Q2)
- Identify repos for Tier 1 (standard enforcement)
- Apply warn-mode enforcement
- Track migration progress

## CI Workflow Status

All repos have been configured with:
- ✅ GitHub Actions workflow file
- ✅ Policy download from central registry
- ✅ Scanner download and execution
- ✅ Artifact upload (JSON, Markdown, SARIF)
- ✅ PR comment integration
- ✅ WARN mode (non-blocking)

## Migration Checklist

### For Repository Owners

- [ ] Review file size violations in your repo
- [ ] Refactor files >350/400 lines into smaller modules
- [ ] Confirm no actual legacy tooling violations present
- [ ] Test CI workflow in a PR
- [ ] Enable strict mode (remove `if: false` in workflow)

### For Platform Engineering

- [ ] Monitor scan reports across all Tier 0 repos
- [ ] Approve any exception requests
- [ ] Track migration progress dashboard
- [ ] Prepare Tier 1 repo list for Q2 rollout

## Exception Policy

If a repo needs temporary exemption from a rule:

1. Create ticket in tracking system (e.g., TICKET-123)
2. Add exception to `tooling/legacy-enforcement/policy/exceptions.yaml`:
```yaml
exceptions:
  - id: EXC-001
    rule_id: LT-PY-001
    repo: "repo-name"
    path_glob: ".github/workflows/ci.yml"
    command_regex: "pytest"
    owner: "team-platform"
    ticket: "TICKET-123"
    rationale: "Migration in progress"
    expires_at: "2026-06-01"
    replacement_plan: "Update to 'uv run pytest' by end of Q2"
```

**Exception limits:**
- Max duration: 90 days
- Requires owner + ticket + replacement plan
- Review by: platform-engineering team

## Related Documentation

- Technology Adoption Philosophy: [CLAUDE.md](../../CLAUDE.md)
- Policy Rules: [tooling/legacy-enforcement/policy/rules.yaml](../../tooling/legacy-enforcement/policy/rules.yaml)
- Scanner Documentation: [tooling/legacy-enforcement/README.md](../../tooling/legacy-enforcement/README.md)
- Full Scanner: [tooling/legacy-enforcement/scanner/legacy_tooling_scanner.py](../../tooling/legacy-enforcement/scanner/legacy_tooling_scanner.py)
