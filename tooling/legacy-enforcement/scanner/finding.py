"""Finding model for legacy tooling scans."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any


@dataclass(frozen=True)
class Finding:
    """A single policy violation finding."""

    rule_id: str
    severity: str
    message: str
    file: str
    line: int
    command: str
    suggested_fix: str
    column: int = 0

    def to_dict(self) -> dict[str, Any]:
        return {
            "rule_id": self.rule_id,
            "severity": self.severity,
            "message": self.message,
            "file": self.file,
            "line": self.line,
            "column": self.column,
            "command": self.command,
            "suggested_fix": self.suggested_fix,
        }


