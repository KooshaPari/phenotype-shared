#!/usr/bin/env python3
"""Legacy Tooling Anti-Pattern Scanner CLI."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

try:
    from .finding import Finding
    from .report_generator import ReportGenerator
    from .rule_engine import RuleEngine
except ImportError:  # pragma: no cover - direct script execution fallback
    from finding import Finding
    from report_generator import ReportGenerator
    from rule_engine import RuleEngine

__all__ = ["Finding", "ReportGenerator", "RuleEngine", "main"]


def main() -> int:
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Legacy Tooling Anti-Pattern Scanner",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s --repo-root . --policy policy/rules.yaml
  %(prog)s --repo-root /path/to/repo --fail-on-severity high
  %(prog)s --repo-root . --report-only --output-json report.json
""",
    )

    parser.add_argument("--repo-root", default=".", help="Path to repository root (default: .)")
    parser.add_argument("--policy", default="", help="Path to policy rules YAML file")
    parser.add_argument("--exceptions", default="", help="Path to exceptions YAML file")
    parser.add_argument("--fail-on-severity", default="high", choices=["none", "low", "medium", "high", "critical"], help="Minimum severity to fail (default: high)")
    parser.add_argument("--report-only", action="store_true", help="Report only, don't fail")
    parser.add_argument("--output-json", default="legacy-tooling-report.json", help="JSON output path")
    parser.add_argument("--output-md", default="legacy-tooling-report.md", help="Markdown output path")
    parser.add_argument("--output-sarif", default="", help="SARIF output path (optional)")
    parser.add_argument("--verbose", "-v", action="store_true", help="Verbose output")

    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve()
    if not repo_root.exists():
        print(f"Error: Repository root does not exist: {repo_root}", file=sys.stderr)
        return 1

    # Determine policy path
    if args.policy:
        policy_path = Path(args.policy)
    else:
        # Try to find policy relative to script location or repo
        script_dir = Path(__file__).parent.resolve()
        policy_path = script_dir.parent / "policy" / "rules.yaml"
        if not policy_path.exists():
            policy_path = repo_root / "tooling" / "legacy-enforcement" / "policy" / "rules.yaml"

    if not policy_path.exists():
        print(f"Error: Policy file not found: {policy_path}", file=sys.stderr)
        return 1

    # Determine exceptions path
    exceptions_path = None
    if args.exceptions:
        exceptions_path = Path(args.exceptions)
    else:
        exc_path = policy_path.parent / "exceptions.yaml"
        if exc_path.exists():
            exceptions_path = exc_path

    if args.verbose:
        print(f"Scanning repository: {repo_root}")
        print(f"Policy: {policy_path}")
        if exceptions_path:
            print(f"Exceptions: {exceptions_path}")

    # Initialize rule engine
    try:
        engine = RuleEngine(policy_path, exceptions_path)
    except Exception as e:
        print(f"Error loading policy: {e}", file=sys.stderr)
        return 1

    # Detect stacks
    stacks = engine.detect_repo_stacks(repo_root)
    if args.verbose:
        print(f"Detected stacks: {', '.join(stacks)}")

    # Scan repository
    findings = engine.scan_directory(repo_root, stacks)

    # Calculate totals
    severity_order = {"low": 0, "medium": 1, "high": 2, "critical": 3}
    totals = {"critical": 0, "high": 0, "medium": 0, "low": 0}
    for finding in findings:
        totals[finding.severity] = totals.get(finding.severity, 0) + 1

    # Generate reports
    ReportGenerator.json_report(findings, totals, Path(args.output_json))
    ReportGenerator.markdown_report(findings, totals, Path(args.output_md))
    if args.output_sarif:
        ReportGenerator.sarif_report(findings, totals, Path(args.output_sarif))

    # Console output
    print(f"\nLegacy Tooling Anti-Pattern Scan Complete")
    print(f"Repository: {repo_root}")
    print(f"Stacks: {', '.join(stacks)}")
    print(f"\nTotals:")
    fail_threshold = severity_order.get(args.fail_on_severity, 999)
    for sev in ["critical", "high", "medium", "low"]:
        count = totals[sev]
        indicator = "❌" if count > 0 and severity_order[sev] >= fail_threshold else "✓"
        print(f"  {indicator} {sev.upper()}: {count}")

    if findings:
        print(f"\nTop findings by severity:")
        sorted_findings = sorted(
            findings,
            key=lambda f: (severity_order.get(f.severity, 0), f.file),
            reverse=True,
        )[:10]
        for finding in sorted_findings:
            print(f"  [{finding.severity.upper()}] {finding.file}:{finding.line} - {finding.rule_id}")

    print(f"\nReports generated:")
    print(f"  - JSON: {args.output_json}")
    print(f"  - Markdown: {args.output_md}")
    if args.output_sarif:
        print(f"  - SARIF: {args.output_sarif}")

    # Determine exit code
    if args.report_only:
        return 0

    # Check if any findings meet the fail threshold
    threshold_level = severity_order[args.fail_on_severity]
    for finding in findings:
        if severity_order.get(finding.severity, 0) >= threshold_level:
            print(f"\nFailed: Found {finding.severity.upper()} severity violation ({finding.rule_id})")
            return 2

    return 0


if __name__ == "__main__":
    sys.exit(main())
