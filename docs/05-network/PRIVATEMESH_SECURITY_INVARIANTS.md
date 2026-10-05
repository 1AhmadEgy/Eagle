# PrivateMesh — Security Invariants

Status: Proposed security boundary.

## I-001 Opaque Payload
Mesh APIs accept only opaque/encrypted frames. No application plaintext type crosses the Mesh boundary.

## I-002 Key Isolation
Mesh has no API for obtaining or storing private keys.

## I-003 Relay Non-Trust
Relay is a transport fallback only. It cannot terminate E2E encryption or reinterpret application content.

## I-004 No Silent Downgrade
Unsupported versions, weaker paths, and unauthorized capability changes fail closed.

## I-005 Resource Bounds
Candidate counts, frame sizes, route lengths, concurrent connections, retries, and timers are bounded.

## I-006 Untrusted Network Input
Discovery records, candidates, frames, and peer metadata are treated as untrusted until validated.

## I-007 State Integrity
Connection and route state changes occur through explicit state-machine transitions.

## I-008 Replay/duplication boundary
Mesh provides transport-level duplicate control only where required; application/protocol replay semantics remain owned by Protocol.

## I-009 Metadata minimization
Logs and telemetry expose only safe identifiers, error codes, timing, and bounded connectivity metadata.

## I-010 Failure isolation
A transport or relay failure cannot mutate identity trust, cryptographic state, or application plaintext.

## Review gate
Any change affecting these invariants requires security-aware review and targeted negative tests.
