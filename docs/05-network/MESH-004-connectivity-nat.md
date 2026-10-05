# MESH-004 — Connectivity Checks and NAT Traversal
Status: SPECIFIED / IMPLEMENTATION-GATED

## Boundary
Connectivity checking is an abstraction. ICE/STUN/TURN may be selected only after ADR-0012 approval.

## Required behavior
- Gather candidates.
- Check candidate pairs.
- Prefer a healthy direct path.
- Fall back to relay when policy permits.
- Fail closed on unauthorized capability changes.

## Acceptance
Connectivity logic is independently testable from transport implementation.
