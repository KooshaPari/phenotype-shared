# Legacy Tooling Exception Request Template

## Exception Request

**Exception ID:** EXC-XXX (to be assigned)
**Rule ID:** LT-XX-XXX (e.g., LT-PY-001)
**Repository:** [repo-name]
**Submitted by:** [GitHub handle]
**Date:** YYYY-MM-DD

## Justification

### Business/Technical Rationale
[Explain why this exception is needed. Be specific about:
- Why the canonical tooling cannot be used
- Technical constraints or dependencies
- Migration timeline and blockers
]

### Impact Assessment
- [ ] This exception affects only CI/CD workflows
- [ ] This exception affects developer workflows
- [ ] This exception affects production deployments
- [ ] This exception creates security or compliance risks

## Proposed Exception Details

```yaml
id: EXC-XXX
rule_id: LT-XX-XXX
repo: "[repo-name]"
path_glob: "[path/to/file.yml]"
command_regex: "[regex matching the command]"
owner: "[team or individual]"
ticket: "[TICKET-123]"
rationale: "[Brief explanation]"
expires_at: "YYYY-MM-DD"
replacement_plan: "[Specific plan to migrate to canonical tooling]"
```

## Migration Plan

### Current State
[Describe current legacy tooling usage]

### Target State
[Describe desired canonical tooling state]

### Migration Steps
1. [Step 1 with timeline]
2. [Step 2 with timeline]
3. [Step 3 with timeline]

### Target Completion Date
YYYY-MM-DD (max 90 days from exception grant)

## Approval

- [ ] Reviewed by platform-engineering team
- [ ] Security implications assessed
- [ ] Exception duration approved (max 90 days)
- [ ] Replacement plan validated
- [ ] Tracking ticket created

**Approved by:** _______________  **Date:** _______________

---

## Example Completed Request

```yaml
id: EXC-001
rule_id: LT-PY-001
repo: "legacy-service"
path_glob: ".github/workflows/ci.yml"
command_regex: "python -m pytest"
owner: "team-backend"
ticket: "LEGACY-456"
rationale: "Service uses legacy pytest plugins incompatible with uv run"
expires_at: "2026-06-01"
replacement_plan: "Update pytest plugins to uv-compatible versions by Q2, migrate to uv run pytest"
```

## Process

1. **Submit:** Create PR adding exception to `tooling/legacy-enforcement/policy/exceptions.yaml`
2. **Review:** Platform engineering reviews within 2 business days
3. **Approve/Deny:** Decision communicated via PR review
4. **Track:** Exception monitored for expiry; renewal requires new request
5. **Close:** Upon migration completion, exception is removed

## Notes

- All exceptions expire automatically; no exceptions are permanent
- Expired exceptions become policy violations
- Repeated exceptions for same rule/repo require architectural review
- Contact: #platform-engineering on Slack
