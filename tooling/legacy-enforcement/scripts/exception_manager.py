#!/usr/bin/env python3
"""
Exception management automation for legacy tooling enforcement.

This script helps create, validate, and manage exceptions to the policy.

Usage:
    python3 exception_manager.py create --rule LT-PY-001 --repo myrepo --ticket TICKET-123
    python3 exception_manager.py validate
    python3 exception_manager.py list --expiring-soon
    python3 exception_manager.py expire-check

Ref: tooling/legacy-enforcement/policy/exceptions.yaml
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import asdict, dataclass
from datetime import date, datetime, timedelta
from pathlib import Path
from typing import Any

import yaml


DEFAULT_EXCEPTIONS_PATH = Path(__file__).parent.parent / "policy" / "exceptions.yaml"


@dataclass
class Exception:
    id: str
    rule_id: str
    repo: str
    path_glob: str
    command_regex: str
    owner: str
    ticket: str
    rationale: str
    expires_at: str
    replacement_plan: str

    def is_expired(self, today: date | None = None) -> bool:
        if today is None:
            today = date.today()
        try:
            expiry = datetime.fromisoformat(self.expires_at).date()
            return today > expiry
        except ValueError:
            return True

    def days_until_expiry(self, today: date | None = None) -> int | None:
        if today is None:
            today = date.today()
        try:
            expiry = datetime.fromisoformat(self.expires_at).date()
            return (expiry - today).days
        except ValueError:
            return None

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


class ExceptionManager:
    def __init__(self, exceptions_path: Path = DEFAULT_EXCEPTIONS_PATH):
        self.exceptions_path = exceptions_path
        self.exceptions: list[Exception] = []
        self._load()

    def _load(self) -> None:
        if not self.exceptions_path.exists():
            self.exceptions = []
            return
        try:
            with open(self.exceptions_path) as f:
                data = yaml.safe_load(f) or {}
            raw_exceptions = data.get("exceptions", [])
            self.exceptions = [Exception(**e) for e in raw_exceptions]
        except Exception as e:
            print(f"Error loading exceptions: {e}", file=sys.stderr)
            self.exceptions = []

    def save(self) -> None:
        data = {
            "exceptions": [e.to_dict() for e in self.exceptions],
            "metadata": {
                "last_updated": datetime.utcnow().isoformat() + "Z",
                "total_exceptions": len(self.exceptions),
                "expired": sum(1 for e in self.exceptions if e.is_expired()),
            },
        }
        with open(self.exceptions_path, "w") as f:
            yaml.dump(data, f, default_flow_style=False, sort_keys=False)

    def create(
        self,
        rule_id: str,
        repo: str,
        path_glob: str,
        command_regex: str,
        owner: str,
        ticket: str,
        rationale: str,
        days: int = 90,
        replacement_plan: str = "",
    ) -> Exception:
        # Generate exception ID
        existing_ids = {e.id for e in self.exceptions}
        counter = 1
        while f"EXC-{counter:03d}" in existing_ids:
            counter += 1
        exc_id = f"EXC-{counter:03d}"

        expires_at = (date.today() + timedelta(days=days)).isoformat()

        exc = Exception(
            id=exc_id,
            rule_id=rule_id,
            repo=repo,
            path_glob=path_glob,
            command_regex=command_regex,
            owner=owner,
            ticket=ticket,
            rationale=rationale,
            expires_at=expires_at,
            replacement_plan=replacement_plan or "TBD",
        )

        self.exceptions.append(exc)
        self.save()
        return exc

    def list_exceptions(
        self,
        repo: str | None = None,
        rule_id: str | None = None,
        expired_only: bool = False,
        expiring_soon: int | None = None,
    ) -> list[Exception]:
        result = self.exceptions

        if repo:
            result = [e for e in result if e.repo == repo]
        if rule_id:
            result = [e for e in result if e.rule_id == rule_id]
        if expired_only:
            result = [e for e in result if e.is_expired()]
        if expiring_soon:
            result = [
                e
                for e in result
                if (days := e.days_until_expiry()) is not None
                and 0 < days <= expiring_soon
            ]

        return result

    def validate(self) -> list[dict[str, Any]]:
        errors = []
        required_fields = [
            "id",
            "rule_id",
            "repo",
            "path_glob",
            "command_regex",
            "owner",
            "ticket",
            "rationale",
            "expires_at",
            "replacement_plan",
        ]

        for exc in self.exceptions:
            for field in required_fields:
                if not getattr(exc, field, None):
                    errors.append(
                        {
                            "exception_id": exc.id,
                            "error": f"Missing required field: {field}",
                        }
                    )

            # Validate regex
            try:
                re.compile(exc.command_regex)
            except re.error as e:
                errors.append(
                    {
                        "exception_id": exc.id,
                        "error": f"Invalid regex pattern: {e}",
                    }
                )

            # Validate date format
            try:
                datetime.fromisoformat(exc.expires_at)
            except ValueError:
                errors.append(
                    {
                        "exception_id": exc.id,
                        "error": f"Invalid date format: {exc.expires_at}",
                    }
                )

        return errors

    def expire_check(self) -> dict[str, Any]:
        today = date.today()
        expired = []
        expiring_30 = []
        expiring_7 = []

        for exc in self.exceptions:
            if exc.is_expired(today):
                expired.append(exc)
            elif (days := exc.days_until_expiry(today)) is not None:
                if days <= 7:
                    expiring_7.append((exc, days))
                elif days <= 30:
                    expiring_30.append((exc, days))

        return {
            "expired": [e.to_dict() for e in expired],
            "expiring_30_days": [{"exception": e.to_dict(), "days": d} for e, d in expiring_30],
            "expiring_7_days": [{"exception": e.to_dict(), "days": d} for e, d in expiring_7],
            "summary": {
                "total": len(self.exceptions),
                "expired": len(expired),
                "expiring_30_days": len(expiring_30),
                "expiring_7_days": len(expiring_7),
            },
        }


def cmd_create(args: argparse.Namespace) -> int:
    manager = ExceptionManager()
    exc = manager.create(
        rule_id=args.rule,
        repo=args.repo,
        path_glob=args.path_glob,
        command_regex=args.command_regex,
        owner=args.owner,
        ticket=args.ticket,
        rationale=args.rationale,
        days=args.days,
        replacement_plan=args.replacement_plan,
    )
    print(f"Created exception: {exc.id}")
    print(f"  Rule: {exc.rule_id}")
    print(f"  Repo: {exc.repo}")
    print(f"  Path: {exc.path_glob}")
    print(f"  Expires: {exc.expires_at}")
    print(f"  Owner: {exc.owner}")
    print(f"  Ticket: {exc.ticket}")
    return 0


def cmd_list(args: argparse.Namespace) -> int:
    manager = ExceptionManager()
    exceptions = manager.list_exceptions(
        repo=args.repo,
        rule_id=args.rule,
        expired_only=args.expired_only,
        expiring_soon=args.expiring_soon,
    )

    if not exceptions:
        print("No exceptions found.")
        return 0

    print(f"{'ID':<12} {'Rule':<12} {'Repo':<20} {'Expires':<12} {'Owner':<15}")
    print("-" * 80)
    for exc in exceptions:
        status = "❌ EXPIRED" if exc.is_expired() else ""
        if not status and (days := exc.days_until_expiry()) is not None:
            if days <= 7:
                status = f"⚠️  {days}d"
        print(
            f"{exc.id:<12} {exc.rule_id:<12} {exc.repo:<20} {exc.expires_at:<12} {exc.owner:<15} {status}"
        )

    print(f"\nTotal: {len(exceptions)} exceptions")
    return 0


def cmd_validate(args: argparse.Namespace) -> int:
    manager = ExceptionManager()
    errors = manager.validate()

    if not errors:
        print("✅ All exceptions are valid.")
        return 0

    print(f"❌ Found {len(errors)} validation errors:")
    for err in errors:
        print(f"  [{err['exception_id']}] {err['error']}")
    return 1


def cmd_expire_check(args: argparse.Namespace) -> int:
    manager = ExceptionManager()
    result = manager.expire_check()

    print("Exception Expiration Check")
    print("=" * 50)

    summary = result["summary"]
    print(f"\nSummary:")
    print(f"  Total exceptions: {summary['total']}")
    print(f"  Expired: {summary['expired']}")
    print(f"  Expiring in 30 days: {summary['expiring_30_days']}")
    print(f"  Expiring in 7 days: {summary['expiring_7_days']}")

    if result["expired"]:
        print(f"\n❌ Expired Exceptions ({len(result['expired'])}):")
        for e in result["expired"]:
            print(f"  - {e['id']}: {e['repo']} / {e['rule_id']} (expired {e['expires_at']})")

    if result["expiring_7_days"]:
        print(f"\n⚠️  Expiring in 7 Days ({len(result['expiring_7_days'])}):")
        for item in result["expiring_7_days"]:
            e = item["exception"]
            print(f"  - {e['id']}: {e['repo']} ({item['days']} days left)")

    if args.json:
        print("\n" + json.dumps(result, indent=2))

    return summary["expired"]


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Manage legacy tooling enforcement exceptions"
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    # Create command
    create_parser = subparsers.add_parser("create", help="Create a new exception")
    create_parser.add_argument("--rule", required=True, help="Rule ID (e.g., LT-PY-001)")
    create_parser.add_argument("--repo", required=True, help="Repository name")
    create_parser.add_argument("--path-glob", default="*", help="Path glob pattern")
    create_parser.add_argument("--command-regex", default=".*", help="Command regex")
    create_parser.add_argument("--owner", required=True, help="Owner username/team")
    create_parser.add_argument("--ticket", required=True, help="Ticket reference")
    create_parser.add_argument("--rationale", required=True, help="Exception rationale")
    create_parser.add_argument("--days", type=int, default=90, help="Exception duration (days)")
    create_parser.add_argument("--replacement-plan", default="", help="Replacement plan")

    # List command
    list_parser = subparsers.add_parser("list", help="List exceptions")
    list_parser.add_argument("--repo", help="Filter by repo")
    list_parser.add_argument("--rule", help="Filter by rule")
    list_parser.add_argument("--expired-only", action="store_true", help="Show only expired")
    list_parser.add_argument(
        "--expiring-soon", type=int, help="Show expiring within N days"
    )

    # Validate command
    subparsers.add_parser("validate", help="Validate all exceptions")

    # Expire check command
    expire_parser = subparsers.add_parser("expire-check", help="Check expiration status")
    expire_parser.add_argument("--json", action="store_true", help="Output as JSON")

    args = parser.parse_args()

    match args.command:
        case "create":
            return cmd_create(args)
        case "list":
            return cmd_list(args)
        case "validate":
            return cmd_validate(args)
        case "expire-check":
            return cmd_expire_check(args)
        case _:
            parser.print_help()
            return 1


if __name__ == "__main__":
    sys.exit(main())
