# Functional Requirements - phenotype-shared

## FR-EVT-001: Event Append
The system SHALL append events with monotonically increasing sequence numbers.

## FR-EVT-002: Hash Chain Integrity
Each event SHALL include a SHA-256 hash linking to the previous event.

## FR-CACHE-001: Two-Tier Lookup
Cache SHALL check L1 (LRU) first, then L2 (DashMap) on miss.

## FR-CACHE-002: TTL Expiration
Entries SHALL be evicted after TTL expires.

## FR-POL-001: TOML Rule Loading
Policy engine SHALL load rules from TOML format.

## FR-POL-002: Rule Evaluation
Engine SHALL evaluate allow/deny/require rules against a context map.

## FR-SM-001: Forward-Only Transitions
State machine SHALL reject backward transitions.

## FR-SM-002: Transition Guards
State machine SHALL support guard callbacks.
