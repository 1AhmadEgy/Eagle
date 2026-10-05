# PrivateMesh — Canonical Authority & Provenance

Status: IN REVIEW / implementation-gated

## Purpose

Establish the single authority chain for P2P/Networking material and prevent historical drafts, copied artifacts, or technology notes from becoming accidental normative requirements.

## Authority order

1. Accepted repository ADRs and normative specifications.
2. Accepted core/protocol contracts referenced by the mesh boundary.
3. Verified implementation and deterministic test evidence.
4. Internal review records and failure matrices.
5. Historical repository documents and imported project material.
6. Conversation notes, external copies, and exploratory designs.

Lower layers cannot override a higher layer without an explicit reconciled change record.

## P2P canonical set

| Topic | Canonical artifact | State |
|---|---|---|
| Mesh boundary | docs/05-network/PRIVATEMESH_MESH_CONTRACTS.md | Proposed |
| Path selection | docs/05-network/PRIVATEMESH_PATH_SELECTION_POLICY.md | Proposed |
| Connection lifecycle | docs/05-network/PRIVATEMESH_CONNECTION_STATE_MACHINE.md | Proposed |
| Security invariants | docs/05-network/PRIVATEMESH_SECURITY_INVARIANTS.md | Proposed |
| Transport decision | docs/05-network/ADR-0012-TRANSPORT-DECISION-INPUT.md | Decision input |
| Failure matrix | docs/05-network/PRIVATEMESH_FAILURE_MATRIX.md | Proposed |
| Executable scenarios | tests/network/privatemesh_network_scenarios.yaml | Scenario definition |

No item above is release authority until the stated review/implementation gates are satisfied.

## Provenance labels

SOURCE → IMPORTED → RECONCILED → CANONICAL

SUPERSEDED and REJECTED records remain discoverable for auditability.

## Reconciliation rules

- Historical QUIC/ICE/STUN/TURN/WebRTC references are candidate inputs only.
- A transport becomes normative only through an accepted ADR with implementation and interoperability evidence.
- Conflicting network semantics must be resolved before implementation constants are introduced.
- A document-only statement never upgrades an implementation status.

## Security rule

The safest default is to reject ambiguity rather than infer compatibility. Unknown, conflicting, or unauthenticated network metadata MUST NOT be allowed to mutate trusted connection state.

## Exit criteria

Canonical authority is release-ready only after:
- all conflicting network requirements are reconciled;
- the approved transport stack is explicit;
- resource bounds are explicit;
- negative tests and failure injection exist;
- implementation evidence is traceable to contracts;
- independent verification signs off.
