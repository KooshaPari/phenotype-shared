"""Report generation for legacy tooling scan results."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path

try:
    from .finding import Finding
except ImportError:  # pragma: no cover - direct script execution fallback
    from finding import Finding


class ReportGenerator:
    """Generate reports in various formats."""

    @staticmethod
    def json_report(findings: list[Finding], totals: dict[str, int], output_path: Path) -> None:
        """Generate JSON report."""
        report = {
            "generated_at": datetime.now(timezone.utc).isoformat(),
            "totals": totals,
            "findings": [f.to_dict() for f in findings],
        }
        with open(output_path, "w", encoding="utf-8") as f:
            json.dump(report, f, indent=2)

    @staticmethod
    def markdown_report(findings: list[Finding], totals: dict[str, int], output_path: Path) -> None:
        """Generate Markdown report."""
        lines = [
            "# Legacy Tooling Anti-Pattern Report",
            "",
            f"Generated: {datetime.now(timezone.utc).isoformat()}Z",
            "",
            "## Summary",
            "",
            f"- **Critical**: {totals.get('critical', 0)}",
            f"- **High**: {totals.get('high', 0)}",
            f"- **Medium**: {totals.get('medium', 0)}",
            f"- **Low**: {totals.get('low', 0)}",
            f"- **Total**: {sum(totals.values())}",
            "",
            "## Findings",
            "",
        ]

        if not findings:
            lines.append("No legacy tooling anti-patterns detected.")
        else:
            # Sort by severity (high to low) then by file
            severity_order = {"critical": 0, "high": 1, "medium": 2, "low": 3}
            sorted_findings = sorted(
                findings,
                key=lambda f: (severity_order.get(f.severity, 99), f.file, f.line),
            )

            for finding in sorted_findings:
                lines.extend([
                    f"### {finding.rule_id} ({finding.severity.upper()})",
                    "",
                    f"- **File**: `{finding.file}:{finding.line}`",
                    f"- **Message**: {finding.message}",
                    f"- **Command**: `{finding.command}`",
                    f"- **Suggested Fix**: {finding.suggested_fix}",
                    "",
                ])

        with open(output_path, "w", encoding="utf-8") as f:
            f.write("\n".join(lines))

    @staticmethod
    def sarif_report(findings: list[Finding], totals: dict[str, int], output_path: Path) -> None:
        """Generate SARIF report for GitHub Advanced Security integration."""
        sarif = {
            "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
            "version": "2.1.0",
            "runs": [
                {
                    "tool": {
                        "driver": {
                            "name": "legacy-tooling-scanner",
                            "informationUri": "https://github.com/phenotype/tooling",
                            "rules": [],
                        }
                    },
                    "results": [],
                }
            ],
        }

        # Add rules and results
        rule_ids = {f.rule_id for f in findings}
        for rule_id in sorted(rule_ids):
            sarif["runs"][0]["tool"]["driver"]["rules"].append(
                {
                    "id": rule_id,
                    "name": rule_id,
                    "shortDescription": {"text": f"Legacy tooling rule {rule_id}"},
                }
            )

        for finding in findings:
            sarif["runs"][0]["results"].append(
                {
                    "ruleId": finding.rule_id,
                    "level": finding.severity if finding.severity in ["error", "warning", "note"] else "warning",
                    "message": {"text": finding.message},
                    "locations": [
                        {
                            "physicalLocation": {
                                "artifactLocation": {"uri": finding.file},
                                "region": {
                                    "startLine": finding.line,
                                    "startColumn": finding.column,
                                },
                            }
                        }
                    ],
                }
            )

        with open(output_path, "w", encoding="utf-8") as f:
            json.dump(sarif, f, indent=2)


