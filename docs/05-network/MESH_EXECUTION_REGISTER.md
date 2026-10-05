# PrivateMesh — MESH-001..MESH-015 Execution Register

Status vocabulary:
- SPECIFIED: contract/acceptance defined
- IMPLEMENTATION-PENDING: requires source implementation after stack/ADR gate
- VERIFIED: implementation + tests + evidence + review
- BLOCKED: prerequisite not satisfied

Current repository state: the P2P planning, contract, failure, provenance, gap, resource, interoperability, and security-review layers are specified. Concrete transport implementation remains blocked by ADR-0012, verified Rust/core integration evidence, and numeric resource limits.

| ID | Stage | Entry condition | Deliverable | Verification |
|---|---|---|---|---|
| MESH-001 | Peer Identity Reference | core identity contract | opaque PeerId | identity mapping + isolation tests |
| MESH-002 | Endpoint/Candidate Model | PeerId | Endpoint/Candidate schema | validation/property tests |
| MESH-003 | Discovery | Candidate model | discovery interface + expiry | stale/duplicate/poisoning tests |
| MESH-004 | Connectivity/NAT Boundary | candidates | connectivity-check state machine | NAT/candidate failure tests |
| MESH-005 | Connection Lifecycle | connectivity abstraction | explicit lifecycle FSM | transition/timeouts/cancellation tests |
| MESH-006 | Opaque Transport | protocol envelope contract | MeshTransport interface | opaque-frame contract tests |
| MESH-007 | Direct P2P | transport contract + approved implementation | direct path selection | direct-path success/failure tests |
| MESH-008 | Relay Fallback | relay contract | relay fallback/recovery | outage/downgrade tests |
| MESH-009 | Routing/Forwarding | peer/path model | bounded routes | loop/hop-limit/duplication tests |
| MESH-010 | Retry/Backoff | failure taxonomy | bounded retry policy | deterministic backoff tests |
| MESH-011 | Network Recovery | lifecycle | interface/sleep/wake recovery | network-change tests |
| MESH-012 | Mesh Security | all mesh contracts | security invariant suite | negative/security review |
| MESH-013 | Conformance | protocol + transport | protocol/transport conformance | vectors/version/downgrade tests |
| MESH-014 | Failure Matrix | all critical paths | repeatable fault scenarios | loss/reorder/replay/NAT/relay suite |
| MESH-015 | Observability/Release | verified test suite | sanitized telemetry + release evidence | evidence review |

## Supporting evidence

- Canonical authority: docs/05-network/PRIVATEMESH_CANONICAL_AUTHORITY.md
- Gap matrix: docs/05-network/PRIVATEMESH_GAP_MATRIX.md
- Interoperability: docs/05-network/PRIVATEMESH_INTEROPERABILITY_MATRIX.md
- Resource limits: docs/05-network/PRIVATEMESH_RESOURCE_LIMITS.md
- Security review: docs/05-network/PRIVATEMESH_SECURITY_REVIEW_CHECKLIST.md
- Evidence index: docs/05-network/PRIVATEMESH_EVIDENCE_INDEX.md
- Execution audit: docs/05-network/PRIVATEMESH_EXECUTION_AUDIT.md
- Release gate: docs/05-network/PRIVATEMESH_RELEASE_GATE.md
- Network scenarios: tests/network/privatemesh_network_scenarios.yaml

## Dependency chain

MESH-001 → MESH-002 → MESH-003 → MESH-004 → MESH-005 → MESH-006 → MESH-007 → MESH-008 → MESH-009 → MESH-010 → MESH-011 → MESH-012 → MESH-013 → MESH-014 → MESH-015

## Non-negotiable mesh invariants

- No application plaintext in Mesh.
- No private-key access.
- No Crypto↔Mesh circular dependency.
- No storage-internal access.
- Relay is fallback transport, not a trust authority.
- No silent protocol downgrade.
- All network-derived inputs are untrusted until validated.
- Resource usage is bounded.
- Security-sensitive changes require targeted negative tests and appropriate review.

## Completion rule

MESH-* may be marked VERIFIED only when observable repository evidence exists. A document or historical reference alone cannot mark an implementation complete.
