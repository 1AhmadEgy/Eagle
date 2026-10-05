# MESH-002 — Endpoint and Candidate Model
Status: SPECIFIED / IMPLEMENTATION-GATED

## Model
Endpoint describes reachable transport metadata.
Candidate describes a possible connectivity path with bounded metadata.

Candidate classes:
- host
- server-reflexive
- relayed
- transport-specific extension

## Rules
- Candidate data is untrusted input.
- Candidate count and field sizes are bounded.
- Endpoint is not an authentication credential.
- Candidate priority is deterministic.

## Acceptance
Validation, deduplication, expiry, ordering, and malformed-input behavior are tested.
