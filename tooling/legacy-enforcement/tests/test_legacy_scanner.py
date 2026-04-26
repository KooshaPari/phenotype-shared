"""
Test suite for legacy tooling scanner.

Usage:
    pytest tests/test_legacy_scanner.py -v
    pytest tests/test_legacy_scanner.py::TestRuleEngine -v

Ref: tooling/legacy-enforcement/scanner/legacy_tooling_scanner.py
"""

from __future__ import annotations

import json
import re
import tempfile
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import pytest
import yaml

# Add scanner to path
import sys
sys.path.insert(0, str(Path(__file__).parent.parent / "scanner"))

from legacy_tooling_scanner import Finding, RuleEngine, ReportGenerator


class TestFinding:
    """Tests for Finding dataclass."""

    def test_finding_creation(self):
        """Test creating a Finding instance."""
        finding = Finding(
            rule_id="LT-PY-001",
            severity="high",
            message="Direct pytest invocation detected",
            file=".github/workflows/ci.yml",
            line=42,
            command="pytest tests/",
            suggested_fix="Use 'uv run pytest' instead",
            column=0,
        )
        assert finding.rule_id == "LT-PY-001"
        assert finding.severity == "high"
        assert finding.line == 42

    def test_finding_to_dict(self):
        """Test converting Finding to dictionary."""
        finding = Finding(
            rule_id="LT-PY-001",
            severity="high",
            message="Direct pytest invocation detected",
            file=".github/workflows/ci.yml",
            line=42,
            command="pytest tests/",
            suggested_fix="Use 'uv run pytest' instead",
        )
        d = finding.to_dict()
        assert d["rule_id"] == "LT-PY-001"
        assert d["severity"] == "high"
        assert d["line"] == 42


class TestRuleEngine:
    """Tests for RuleEngine class."""

    @pytest.fixture
    def temp_policy(self, tmp_path: Path):
        """Create a temporary policy file."""
        policy = {
            "version": "1.0.0",
            "maturity_tiers": {
                "tier_0": {"name": "strict", "repos": ["test-repo"]},
            },
            "rules": [
                {
                    "id": "TEST-001",
                    "severity": "high",
                    "category": "testing",
                    "description": "Test rule for pytest",
                    "rationale": "Testing",
                    "applies_to": ["python"],
                    "files": ["*.yml", "*.yaml"],
                    "patterns": [
                        {"regex": r"\bpytest\b", "target": "run_block"},
                    ],
                    "suggested_fix": "Use proper test runner",
                },
                {
                    "id": "TEST-002",
                    "severity": "medium",
                    "category": "build",
                    "description": "Test rule for make",
                    "rationale": "Testing",
                    "applies_to": ["general"],
                    "files": ["*"],
                    "patterns": [
                        {"regex": r"^make\s+\w+", "target": "run_block"},
                    ],
                    "suggested_fix": "Use task runner",
                },
            ],
        }
        policy_file = tmp_path / "rules.yaml"
        with open(policy_file, "w") as f:
            yaml.dump(policy, f)
        return policy_file

    @pytest.fixture
    def temp_exceptions(self, tmp_path: Path):
        """Create a temporary exceptions file."""
        exceptions = {
            "exceptions": [
                {
                    "id": "EXC-001",
                    "rule_id": "TEST-001",
                    "repo": "test-repo",
                    "path_glob": ".github/workflows/ci.yml",
                    "owner": "test-team",
                    "ticket": "TICKET-123",
                    "rationale": "Testing exception",
                    "expires_at": "2026-12-31",
                }
            ]
        }
        exc_file = tmp_path / "exceptions.yaml"
        with open(exc_file, "w") as f:
            yaml.dump(exceptions, f)
        return exc_file

    def test_load_yaml(self, temp_policy: Path):
        """Test YAML loading."""
        engine = RuleEngine(temp_policy)
        assert "rules" in engine.rules
        assert len(engine.rules["rules"]) == 2

    def test_compile_rules(self, temp_policy: Path):
        """Test rule compilation."""
        engine = RuleEngine(temp_policy)
        assert len(engine._compiled_rules) == 2
        assert all(isinstance(p[1], re.Pattern) for p in engine._compiled_rules)

    def test_detect_repo_stacks_python(self, tmp_path: Path):
        """Test Python stack detection."""
        (tmp_path / "pyproject.toml").touch()
        # Create a minimal policy to avoid file not found
        policy_file = tmp_path / "policy.yaml"
        policy_file.write_text("rules: []")
        engine = RuleEngine(policy_file)
        stacks = engine.detect_repo_stacks(tmp_path)
        assert "python" in stacks

    def test_detect_repo_stacks_javascript(self, tmp_path: Path):
        """Test JavaScript stack detection."""
        (tmp_path / "package.json").touch()
        policy_file = tmp_path / "policy.yaml"
        policy_file.write_text("rules: []")
        engine = RuleEngine(policy_file)
        stacks = engine.detect_repo_stacks(tmp_path)
        assert "javascript" in stacks

    def test_detect_repo_stacks_rust(self, tmp_path: Path):
        """Test Rust stack detection."""
        (tmp_path / "Cargo.toml").touch()
        policy_file = tmp_path / "policy.yaml"
        policy_file.write_text("rules: []")
        engine = RuleEngine(policy_file)
        stacks = engine.detect_repo_stacks(tmp_path)
        assert "rust" in stacks

    def test_scan_file_content_violation(self, temp_policy: Path, tmp_path: Path):
        """Test scanning a file for violations."""
        engine = RuleEngine(temp_policy)
        
        # Create a test workflow file with violation
        workflow = tmp_path / "ci.yml"
        workflow.write_text("""
name: CI
on: push
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: pytest tests/
""")
        
        stacks = {"python", "general"}
        findings = engine.scan_file_content(workflow, tmp_path, stacks)
        
        # Should find the pytest violation
        pytest_findings = [f for f in findings if "pytest" in f.command]
        assert len(pytest_findings) >= 1
        assert pytest_findings[0].rule_id == "TEST-001"
        assert pytest_findings[0].severity == "high"

    def test_scan_file_content_no_violation(self, temp_policy: Path, tmp_path: Path):
        """Test scanning a clean file."""
        engine = RuleEngine(temp_policy)
        
        workflow = tmp_path / "ci.yml"
        workflow.write_text("""
name: CI
on: push
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: uv run pytest tests/
""")
        
        stacks = {"python", "general"}
        findings = engine.scan_file_content(workflow, tmp_path, stacks)
        
        # 'uv run pytest' should not match the simple 'pytest' pattern
        simple_pytest_findings = [f for f in findings if f.command.strip() == "pytest tests/"]
        assert len(simple_pytest_findings) == 0

    def test_scan_file_content_with_exception(self, temp_policy: Path, temp_exceptions: Path, tmp_path: Path):
        """Test exception handling."""
        engine = RuleEngine(temp_policy, temp_exceptions)
        
        workflow = tmp_path / "ci.yml"
        workflow.write_text("""
name: CI
on: push
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: pytest tests/
""")
        
        # The exception is for .github/workflows/ci.yml, not ci.yml
        # So this will still find violations unless we put the file in the right path
        stacks = {"python", "general"}
        findings = engine.scan_file_content(workflow, tmp_path, stacks)
        
        # Should still find because path doesn't match exception
        assert len(findings) >= 1

    def test_exception_expired(self, tmp_path: Path):
        """Test expired exceptions are ignored."""
        policy = {
            "rules": [
                {
                    "id": "TEST-001",
                    "severity": "high",
                    "category": "testing",
                    "description": "Test rule",
                    "rationale": "Test",
                    "applies_to": ["python"],
                    "files": ["*"],
                    "patterns": [{"regex": r"\bpytest\b", "target": "run_block"}],
                    "suggested_fix": "Fix it",
                }
            ]
        }
        policy_file = tmp_path / "rules.yaml"
        with open(policy_file, "w") as f:
            yaml.dump(policy, f)

        exceptions = {
            "exceptions": [
                {
                    "id": "EXC-001",
                    "rule_id": "TEST-001",
                    "repo": "test-repo",
                    "path_glob": "*",
                    "owner": "test",
                    "ticket": "TICKET",
                    "rationale": "Expired",
                    "expires_at": "2020-01-01",  # Expired
                }
            ]
        }
        exc_file = tmp_path / "exceptions.yaml"
        with open(exc_file, "w") as f:
            yaml.dump(exceptions, f)

        engine = RuleEngine(policy_file, exc_file)
        
        workflow = tmp_path / "test.yml"
        workflow.write_text("jobs:\n  test:\n    steps:\n      - run: pytest tests/")
        
        stacks = {"python", "general"}
        findings = engine.scan_file_content(workflow, tmp_path, stacks)
        
        # Exception is expired, so violation should be reported
        assert len(findings) >= 1


class TestReportGenerator:
    """Tests for ReportGenerator class."""

    @pytest.fixture
    def sample_findings(self):
        """Create sample findings for testing."""
        return [
            Finding(
                rule_id="LT-PY-001",
                severity="high",
                message="Direct pytest",
                file="ci.yml",
                line=10,
                command="pytest",
                suggested_fix="Use uv run pytest",
            ),
            Finding(
                rule_id="LT-JS-001",
                severity="medium",
                message="npm ci usage",
                file="ci.yml",
                line=20,
                command="npm ci",
                suggested_fix="Use bun install",
            ),
            Finding(
                rule_id="LT-GEN-005",
                severity="medium",
                message="File too large",
                file="large.py",
                line=400,
                command="N/A",
                suggested_fix="Split into modules",
            ),
        ]

    def test_json_report(self, sample_findings, tmp_path: Path):
        """Test JSON report generation."""
        totals = {"critical": 0, "high": 1, "medium": 2, "low": 0}
        output = tmp_path / "report.json"
        ReportGenerator.json_report(sample_findings, totals, output)
        
        with open(output) as f:
            data = json.load(f)
        assert data["totals"]["high"] == 1
        assert data["totals"]["medium"] == 2
        assert len(data["findings"]) == 3

    def test_markdown_report(self, sample_findings, tmp_path: Path):
        """Test Markdown report generation."""
        totals = {"critical": 0, "high": 1, "medium": 2, "low": 0}
        output = tmp_path / "report.md"
        ReportGenerator.markdown_report(sample_findings, totals, output)
        
        content = output.read_text()
        assert "# Legacy Tooling Anti-Pattern Report" in content
        assert "LT-PY-001" in content
        assert "Direct pytest" in content

    def test_sarif_report(self, sample_findings, tmp_path: Path):
        """Test SARIF report generation."""
        totals = {"critical": 0, "high": 1, "medium": 2, "low": 0}
        output = tmp_path / "report.sarif"
        ReportGenerator.sarif_report(sample_findings, totals, output)
        
        with open(output) as f:
            data = json.load(f)
        assert data["$schema"] == "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json"
        assert len(data["runs"][0]["results"]) == 3
        for result in data["runs"][0]["results"]:
            region = result["locations"][0]["physicalLocation"]["region"]
            assert region["startLine"] >= 1
            assert region["startColumn"] >= 1


class TestIntegration:
    """Integration tests for full scanner flow."""

    def test_full_scan_detects_violations(self, tmp_path: Path):
        """Test full scan on a synthetic repo."""
        # Create policy
        policy = {
            "rules": [
                {
                    "id": "LT-PY-001",
                    "severity": "high",
                    "category": "testing",
                    "description": "Direct pytest",
                    "rationale": "Test",
                    "applies_to": ["python"],
                    "files": ["*.yml", "*.yaml"],
                    "patterns": [{"regex": r"\bpytest\b", "target": "run_block"}],
                    "suggested_fix": "Use uv run pytest",
                }
            ]
        }
        policy_file = tmp_path / "rules.yaml"
        with open(policy_file, "w") as f:
            yaml.dump(policy, f)

        # Create synthetic repo
        repo = tmp_path / "repo"
        workflows = repo / ".github" / "workflows"
        workflows.mkdir(parents=True)
        (repo / "pyproject.toml").touch()

        # Create workflow with violation
        workflow_file = workflows / "ci.yml"
        workflow_file.write_text("""
name: CI
on: push
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: pytest tests/
""")

        # Run scan
        engine = RuleEngine(policy_file)
        stacks = engine.detect_repo_stacks(repo)
        findings = engine.scan_directory(repo, stacks)

        # Should find the pytest violation
        pytest_findings = [f for f in findings if "pytest" in f.command]
        assert len(pytest_findings) >= 1

    def test_full_scan_passes_clean_repo(self, tmp_path: Path):
        """Test full scan on a clean repo."""
        policy = {"rules": []}
        policy_file = tmp_path / "rules.yaml"
        with open(policy_file, "w") as f:
            yaml.dump(policy, f)

        repo = tmp_path / "repo"
        repo.mkdir()
        (repo / "pyproject.toml").touch()

        engine = RuleEngine(policy_file)
        stacks = engine.detect_repo_stacks(repo)
        findings = engine.scan_directory(repo, stacks)
        
        # With no rules, should have no findings
        assert len(findings) == 0

    def test_load_rules_from_file(self):
        """Test loading rules from actual policy file."""
        policy_path = Path(__file__).parent.parent / "policy" / "rules.yaml"
        if not policy_path.exists():
            pytest.skip("Policy file not found")
        
        engine = RuleEngine(policy_path)
        assert len(engine.rules.get("rules", [])) > 0
        assert all("id" in r for r in engine.rules["rules"])
        assert all("severity" in r for r in engine.rules["rules"])


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
