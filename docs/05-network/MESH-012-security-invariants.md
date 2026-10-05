# MESH-012 — Mesh Security Invariants
Status: SPECIFIED / SECURITY-GATED

## Invariants
I-001 opaque payload only
I-002 no private-key access
I-003 relay is non-trust fallback
I-004 no silent downgrade
I-005 bounded resources
I-006 all network input untrusted
I-007 explicit state transitions
I-008 replay semantics remain owned by Protocol
I-009 sanitized telemetry
I-010 transport failure cannot mutate trust/crypto/application state

## Acceptance
Every invariant has a negative test and security-aware review before merge.
