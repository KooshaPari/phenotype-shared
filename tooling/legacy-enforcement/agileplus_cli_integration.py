#!/usr/bin/env python3
"""
AgilePlus CLI integration for legacy tooling enforcement.

This module provides a `legacy-scan` subcommand for the AgilePlus CLI,
enabling local enforcement checks before CI/CD.

Usage:
    agileplus legacy-scan [--repo-root PATH] [--severity LEVEL]

Ref: tooling/legacy-enforcement/policy/rules.yaml
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path
from typing import Any


def find_repos_root() -> Path:
    """Find the phenotype/repos root from AgilePlus location."""
    # AgilePlus is at repos/AgilePlus
    return Path(__file__).resolve().parents[2]


def find_scanner() -> Path | None:
    """Locate the legacy tooling scanner."""
    repos_root = find_repos_root()
    scanner = repos_root / "tooling" / "legacy-enforcement" / "scanner" / "legacy_tooling_scanner.py"
    if scanner.exists():
        return scanner
    return None


def find_policy() -> Path | None:
    """Locate the default policy file."""
    repos_root = find_repos_root()
    policy = repos_root / "tooling" / "legacy-enforcement" / "policy" / "rules.yaml"
    if policy.exists():
        return policy
    return None


def run_scan(
    repo_root: Path,
    policy: Path | None = None,
    fail_on_severity: str = "high",
    report_only: bool = False,
    output_json: Path | None = None,
    output_md: Path | None = None,
) -> dict[str, Any]:
    """Execute the legacy tooling scanner."""
    scanner = find_scanner()
    if not scanner:
        return {"error": "Scanner not found", "exit_code": 1}

    if not policy:
        policy = find_policy()

    cmd = [
        sys.executable,
        str(scanner),
        "--repo-root", str(repo_root),
        "--fail-on-severity", fail_on_severity,
    ]

    if policy:
        cmd.extend(["--policy", str(policy)])

    if report_only:
        cmd.append("--report-only")

    if output_json:
        cmd.extend(["--output-json", str(output_json)])

    if output_md:
        cmd.extend(["--output-md", str(output_md)])

    result = subprocess.run(cmd, capture_output=True, text=True)

    # Parse JSON output if available
    findings: dict[str, Any] = {
        "exit_code": result.returncode,
        "stdout": result.stdout,
        "stderr": result.stderr,
    }

    if output_json and output_json.exists():
        try:
            with open(output_json) as f:
                findings["data"] = json.load(f)
        except json.JSONDecodeError:
            pass

    return findings


def format_summary(findings: dict[str, Any]) -> str:
    """Format scan results for CLI output."""
    lines = ["╔══════════════════════════════════════════════════════════════════╗"]
    lines.append("║  Legacy Tooling Enforcement - Scan Results                        ║")
    lines.append("╚══════════════════════════════════════════════════════════════════╝")

    data = findings.get("data", {})
    totals = data.get("totals", {})

    lines.append(f"\n  Totals:")
    for sev in ["critical", "high", "medium", "low"]:
        count = totals.get(sev, 0)
        icon = "X" if sev in ("critical", "high") and count > 0 else "!" if count > 0 else "ok"
        lines.append(f"    [{icon}] {sev.upper():10} : {count}")

    # Exit code interpretation
    exit_code = findings.get("exit_code", 0)
    if exit_code == 0:
        lines.append("\n  [ok] No blocking issues found")
    elif exit_code == 2:
        lines.append("\n  [X] Blocking violations detected - fix required")
    else:
        lines.append(f"\n  [!] Scanner error (exit {exit_code})")

    # Show sample violations
    violations = data.get("findings", [])
    if violations:
        lines.append(f"\n  Top violations:")
        for v in violations[:5]:
            sev = v.get("severity", "unknown")
            rule = v.get("rule_id", "UNKNOWN")
            file = v.get("file", "?")
            line = v.get("line", 0)
            lines.append(f"    [{sev.upper()}] {rule} at {file}:{line}")
        if len(violations) > 5:
            lines.append(f"    ... and {len(violations) - 5} more")

    return "\n".join(lines)


def main(args: list[str] | None = None) -> int:
    """CLI entry point for legacy-scan command."""
    parser = argparse.ArgumentParser(
        prog="agileplus legacy-scan",
        description="Scan repository for legacy tooling violations",
    )
    parser.add_argument(
        "--repo-root",
        type=Path,
        default=Path.cwd(),
        help="Repository root to scan (default: cwd)",
    )
    parser.add_argument(
        "--severity",
        choices=["critical", "high", "medium", "low", "none"],
        default="high",
        help="Minimum severity to fail on",
    )
    parser.add_argument(
        "--report-only",
        action="store_true",
        help="Report only, do not fail",
    )
    parser.add_argument(
        "--output",
        type=Path,
        help="Output JSON report path",
    )
    parser.add_argument(
        "--summary-only",
        action="store_true",
        help="Print summary, skip full output",
    )

    parsed = parser.parse_args(args)

    output_json = parsed.output or Path("/tmp/agileplus_legacy_scan.json")

    print(f"Scanning: {parsed.repo_root.resolve()}")
    print(f"Policy: {find_policy() or 'builtin'}")
    print(f"Threshold: {parsed.severity}")
    print("-" * 70)

    findings = run_scan(
        repo_root=parsed.repo_root,
        fail_on_severity=parsed.severity,
        report_only=parsed.report_only,
        output_json=output_json,
    )

    print(format_summary(findings))

    return findings.get("exit_code", 1)


if __name__ == "__main__":
    sys.exit(main())
