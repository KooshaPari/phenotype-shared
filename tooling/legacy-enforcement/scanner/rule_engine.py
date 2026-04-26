"""Rule loading and repository scanning for legacy tooling policy."""

from __future__ import annotations

import fnmatch
import json
import os
import re
from datetime import datetime, timezone
from pathlib import Path

try:
    from .finding import Finding
except ImportError:  # pragma: no cover - direct script execution fallback
    from finding import Finding


class RuleEngine:
    """Loads and evaluates policy rules."""

    SEVERITY_ORDER = {"low": 0, "medium": 1, "high": 2, "critical": 3}

    def __init__(self, rules_path: Path, exceptions_path: Path | None = None):
        self.rules = self._load_yaml(rules_path)
        self.exceptions = self._load_yaml(exceptions_path) if exceptions_path else {}
        self._compiled_rules: list[tuple[dict, re.Pattern]] = []
        self._compile_rules()

    def _load_yaml(self, path: Path) -> dict:
        """Load YAML file safely."""
        try:
            import yaml

            with open(path, "r", encoding="utf-8") as f:
                return yaml.safe_load(f) or {}
        except ImportError:
            # Fallback to JSON if yaml not available
            if path.suffix == ".json":
                with open(path, "r", encoding="utf-8") as f:
                    return json.load(f)
            raise
        except FileNotFoundError:
            return {}

    def _compile_rules(self) -> None:
        """Pre-compile regex patterns for performance."""
        for rule in self.rules.get("rules", []):
            patterns = rule.get("patterns", [])
            for p in patterns:
                regex = p.get("regex", "")
                if regex:
                    try:
                        compiled = re.compile(regex, re.IGNORECASE)
                        self._compiled_rules.append((rule, compiled))
                    except re.error as e:
                        print(f"Warning: Invalid regex in rule {rule.get('id')}: {e}", file=sys.stderr)

    def detect_repo_stacks(self, repo_root: Path) -> set[str]:
        """Detect which technology stacks are present in the repo."""
        stacks = set()

        # Python detection
        if any(
            (repo_root / f).exists()
            for f in ["pyproject.toml", "setup.py", "requirements.txt", "uv.lock"]
        ):
            stacks.add("python")

        # JavaScript/TypeScript detection
        if (repo_root / "package.json").exists():
            stacks.add("javascript")
            if any((repo_root / f).exists() for f in ["tsconfig.json", "bun.lockb"]):
                stacks.add("typescript")

        # Go detection
        if (repo_root / "go.mod").exists():
            stacks.add("go")

        # Rust detection
        if (repo_root / "Cargo.toml").exists():
            stacks.add("rust")

        stacks.add("general")
        return stacks

    def get_file_threshold(self, rule: dict, ext: str) -> int | None:
        """Get line threshold for file size rules."""
        thresholds = rule.get("thresholds", {})
        # Map extensions to language names
        ext_map = {
            ".py": "python",
            ".js": "javascript",
            ".ts": "typescript",
            ".tsx": "typescript",
            ".rs": "rust",
            ".go": "go",
        }
        lang = ext_map.get(ext)
        if lang and lang in thresholds:
            return thresholds[lang]
        return None

    def check_file_size(
        self, file_path: Path, repo_root: Path, rule: dict
    ) -> list[Finding]:
        """Check if file exceeds line threshold."""
        findings = []
        ext = file_path.suffix
        threshold = self.get_file_threshold(rule, ext)

        if threshold is None:
            return findings

        try:
            with open(file_path, "r", encoding="utf-8", errors="replace") as f:
                lines = f.readlines()
                line_count = len(lines)

            if line_count > threshold:
                rel_path = str(file_path.relative_to(repo_root))
                findings.append(
                    Finding(
                        rule_id=rule.get("id", "LT-UNKNOWN"),
                        severity=rule.get("severity", "medium"),
                        message=f"{rule.get('name', 'Large file')}: {line_count} lines exceeds threshold of {threshold}",
                        file=rel_path,
                        line=1,
                        column=0,
                        command=f"File has {line_count} lines",
                        suggested_fix=rule.get("suggested_fix", "Refactor into smaller modules"),
                    )
                )
        except (IOError, OSError):
            pass

        return findings

    def scan_file_content(
        self, file_path: Path, repo_root: Path, stacks: set[str]
    ) -> list[Finding]:
        """Scan a single file for rule violations."""
        findings = []
        rel_path = str(file_path.relative_to(repo_root))

        try:
            with open(file_path, "r", encoding="utf-8", errors="replace") as f:
                content = f.read()
                lines = content.splitlines()
        except (IOError, OSError):
            return findings

        for rule, pattern in self._compiled_rules:
            # Check if rule applies to this repo's stacks
            applies_to = set(rule.get("applies_to", []))
            if not applies_to.intersection(stacks):
                continue

            # Check file patterns
            files_patterns = rule.get("files", ["*"])
            if not any(fnmatch.fnmatch(rel_path, fp) for fp in files_patterns):
                continue

            # Check for regex matches
            for i, line in enumerate(lines, start=1):
                if pattern.search(line):
                    # Check exceptions
                    if self._is_excepted(rule.get("id"), rel_path, line):
                        continue

                    findings.append(
                        Finding(
                            rule_id=rule.get("id", "LT-UNKNOWN"),
                            severity=rule.get("severity", "medium"),
                            message=f"{rule.get('id')}: {rule.get('description', 'Unknown rule')}",
                            file=rel_path,
                            line=i,
                            column=line.find(pattern.search(line).group()) if pattern.search(line) else 0,
                            command=line.strip(),
                            suggested_fix=rule.get("suggested_fix", "Review and update"),
                        )
                    )

        return findings

    def _is_excepted(self, rule_id: str, file_path: str, command: str) -> bool:
        """Check if a finding matches an active exception."""
        now = datetime.now(timezone.utc)

        exceptions_list = (self.exceptions or {}).get("exceptions") or []
        for exc in exceptions_list:
            if exc.get("rule_id") != rule_id:
                continue

            # Check if exception is expired
            expires = exc.get("expires_at", "")
            if expires:
                try:
                    expiry_dt = datetime.fromisoformat(expires.replace("Z", "+00:00"))
                    # Ensure timezone-aware comparison
                    if expiry_dt.tzinfo is None:
                        expiry_dt = expiry_dt.replace(tzinfo=timezone.utc)
                    if now > expiry_dt:
                        continue  # Exception expired
                except ValueError:
                    continue

            # Check path glob
            path_glob = exc.get("path_glob", "")
            if path_glob and not fnmatch.fnmatch(file_path, path_glob):
                continue

            # Check command regex
            cmd_regex = exc.get("command_regex", "")
            if cmd_regex:
                try:
                    if not re.search(cmd_regex, command, re.IGNORECASE):
                        continue
                except re.error:
                    continue

            return True

        return False

    def scan_directory(self, repo_root: Path, stacks: set[str]) -> list[Finding]:
        """Scan entire repository directory."""
        findings = []

        # Directories to ignore
        ignore_dirs = {
            ".git",
            ".venv",
            ".env",
            ".tox",
            "node_modules",
            "target",
            "dist",
            "build",
            "__pycache__",
            ".pytest_cache",
            ".mypy_cache",
            ".ruff_cache",
        }

        for root, dirs, files in os.walk(repo_root):
            # Filter out ignored directories
            dirs[:] = [d for d in dirs if d not in ignore_dirs]

            for filename in files:
                file_path = Path(root) / filename

                # Check file size rules
                for rule in self.rules.get("rules", []):
                    if not rule.get("patterns"):  # Size rules have empty patterns
                        findings.extend(self.check_file_size(file_path, repo_root, rule))

                # Content scanning for relevant files
                if filename.endswith(
                    (".yml", ".yaml", ".sh", ".py", ".js", ".ts", ".rs", ".go", ".toml", ".json")
                ):
                    findings.extend(self.scan_file_content(file_path, repo_root, stacks))

        return findings

