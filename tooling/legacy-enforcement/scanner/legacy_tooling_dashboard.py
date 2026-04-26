#!/usr/bin/env python3
"""
Legacy Tooling Enforcement Dashboard
Aggregates scan results across all Tier 0 repos for monitoring

Usage:
    python legacy_tooling_dashboard.py --repos AgilePlus Tracera thegent phenoSDK heliosCLI
    python legacy_tooling_dashboard.py --all-tier0 --output dashboard.md
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Optional


@dataclass
class RepoResult:
    name: str
    stacks: list[str]
    critical: int
    high: int
    medium: int
    low: int
    total: int
    findings: list[dict]


def run_scanner(repo_path: Path, policy_path: Path) -> Optional[RepoResult]:
    """Run scanner on a repo and parse results."""
    scanner_path = Path(__file__).parent / "scanner" / "legacy_tooling_scanner.py"
    report_path = f"/tmp/dashboard_{repo_path.name}.json"

    cmd = [
        sys.executable,
        str(scanner_path),
        "--repo-root", str(repo_path),
        "--policy", str(policy_path),
        "--fail-on-severity", "none",
        "--report-only",
        "--output-json", report_path,
    ]

    try:
        subprocess.run(cmd, capture_output=True, timeout=300)

        with open(report_path) as f:
            data = json.load(f)

        totals = data.get("totals", {})
        return RepoResult(
            name=repo_path.name,
            stacks=list(set(data.get("stacks", []))),  # Deduplicate
            critical=totals.get("critical", 0),
            high=totals.get("high", 0),
            medium=totals.get("medium", 0),
            low=totals.get("low", 0),
            total=sum(totals.values()),
            findings=data.get("findings", []),
        )
    except Exception as e:
        print(f"Error scanning {repo_path.name}: {e}", file=sys.stderr)
        return None


def generate_markdown(results: list[RepoResult]) -> str:
    """Generate Markdown dashboard report."""
    lines = [
        "# Legacy Tooling Enforcement Dashboard",
        "",
        f"**Generated:** {datetime.now(timezone.utc).isoformat()}Z",
        "",
        "## Tier 0 Repos Summary",
        "",
        "| Repo | Stacks | Critical | High | Medium | Low | Status |",
        "|------|--------|----------|------|--------|-----|--------|",
    ]

    for r in results:
        status = "✅ Strict" if r.critical == 0 and r.high == 0 else "⚠️ Violations"
        stacks_str = ", ".join(sorted(r.stacks)) if r.stacks else "-"
        lines.append(
            f"| {r.name} | {stacks_str} | {r.critical} | {r.high} | {r.medium} | {r.low} | {status} |"
        )

    # Totals
    total_crit = sum(r.critical for r in results)
    total_high = sum(r.high for r in results)
    total_med = sum(r.medium for r in results)
    total_low = sum(r.low for r in results)

    lines.extend([
        "",
        "## Totals",
        "",
        f"- **Critical:** {total_crit}",
        f"- **High:** {total_high}",
        f"- **Medium:** {total_med}",
        f"- **Low:** {total_low}",
        f"- **Overall:** {sum([total_crit, total_high, total_med, total_low])}",
        "",
    ])

    # Violations by type
    if any(r.findings for r in results):
        lines.extend([
            "## Top Violations by Rule",
            "",
        ])

        rule_counts: dict[str, int] = {}
        for r in results:
            for f in r.findings:
                rule_id = f.get("rule_id", "UNKNOWN")
                rule_counts[rule_id] = rule_counts.get(rule_id, 0) + 1

        sorted_rules = sorted(rule_counts.items(), key=lambda x: x[1], reverse=True)
        for rule_id, count in sorted_rules[:10]:
            lines.append(f"- {rule_id}: {count} occurrences")

    # Per-repo details
    lines.extend([
        "",
        "## Per-Repo Details",
        "",
    ])

    for r in results:
        if r.findings:
            lines.extend([
                f"### {r.name}",
                "",
            ])

            # Group by severity
            by_sev: dict[str, list[dict]] = {"critical": [], "high": [], "medium": [], "low": []}
            for f in r.findings:
                sev = f.get("severity", "low")
                by_sev.get(sev, []).append(f)

            for sev in ["critical", "high", "medium", "low"]:
                if by_sev[sev]:
                    lines.append(f"**{sev.upper()}:** {len(by_sev[sev])} violations")
                    for f in by_sev[sev][:5]:  # Show top 5 per severity
                        file = f.get("file", "unknown")
                        line = f.get("line", 0)
                        rule = f.get("rule_id", "UNKNOWN")
                        lines.append(f"- `{file}:{line}` - {rule}")
                    if len(by_sev[sev]) > 5:
                        lines.append(f"- ... and {len(by_sev[sev]) - 5} more")
                    lines.append("")

    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description="Legacy Tooling Enforcement Dashboard")
    parser.add_argument("--repos", nargs="+", help="Repo names to scan")
    parser.add_argument("--all-tier0", action="store_true", help="Scan all Tier 0 repos")
    parser.add_argument("--policy", default="tooling/legacy-enforcement/policy/rules.yaml")
    parser.add_argument("--output", "-o", default="-", help="Output file (- for stdout)")
    parser.add_argument("--repos-root", default="/Users/kooshapari/CodeProjects/Phenotype/repos")

    args = parser.parse_args()

    if args.all_tier0:
        repos = ["AgilePlus", "Tracera", "thegent", "phenoSDK", "heliosCLI"]
    elif args.repos:
        repos = args.repos
    else:
        print("Error: Specify --repos or --all-tier0", file=sys.stderr)
        return 1

    policy_path = Path(args.policy).resolve()
    if not policy_path.exists():
        print(f"Error: Policy not found: {policy_path}", file=sys.stderr)
        return 1

    repos_root = Path(args.repos_root)

    print("Scanning repos...", file=sys.stderr)
    results: list[RepoResult] = []
    for repo_name in repos:
        repo_path = repos_root / repo_name
        if not repo_path.exists():
            print(f"Warning: Repo not found: {repo_path}", file=sys.stderr)
            continue

        print(f"  - {repo_name}...", file=sys.stderr)
        result = run_scanner(repo_path, policy_path)
        if result:
            results.append(result)

    print("\nGenerating dashboard...", file=sys.stderr)
    dashboard = generate_markdown(results)

    if args.output == "-":
        print(dashboard)
    else:
        Path(args.output).write_text(dashboard)
        print(f"Dashboard written to: {args.output}", file=sys.stderr)

    return 0


if __name__ == "__main__":
    sys.exit(main())
