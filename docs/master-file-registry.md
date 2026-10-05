# Master File Registry — PrivateMesh

Canonical documentation index for the PrivateMesh protocol work in Eagle.

## 1. Registry purpose

This registry records the authoritative location and status of protocol specifications, baselines, architecture records, security reviews, test vectors, and implementation evidence.

The registry does not establish that a document or implementation is correct. It establishes provenance, authority, version, and review state.

## 2. Authority rules

- One document/version MUST be designated canonical for each normative topic.
- A superseded document MUST remain discoverable but MUST NOT be treated as current authority.
- Duplicate specifications MUST be reconciled before release.
- Repository state is the authoritative source for files committed here.
- Conversation attachments and external copies are evidence/provenance inputs, not canonical repository state unless explicitly imported and recorded.
- A document marked DRAFT or IN REVIEW MUST NOT be represented as a released protocol contract.

## 3. Canonical registry

| ID | Document | Repository Path | Class | Status | Authority |
|---|---|---|---|---|---|
| PM-REG-001 | Master File Registry | docs/master-file-registry.md | Registry | IN REVIEW | CANONICAL |
| PM-SPEC-001 | Identity & Key Model | docs/01-normative/PM-SPEC-001-identity-key-model.md | Normative | PLANNED | TBD |
| PM-SPEC-002 | Device Lifecycle | docs/01-normative/PM-SPEC-002-device-lifecycle.md | Normative | PLANNED | TBD |
| PM-SPEC-003 | Trust State Machine | docs/01-normative/PM-SPEC-003-trust-state-machine.md | Normative | PLANNED | TBD |
| PM-SPEC-004 | PQXDH + Double Ratchet | docs/01-normative/PM-SPEC-004-pqx-dh-double-ratchet.md | Normative | PLANNED | TBD |
| PM-SPEC-005 | Wire Format | docs/01-normative/PM-SPEC-005-wire-format.md | Normative | PLANNED | TBD |
| PM-SPEC-006 | Offline Queue | docs/01-normative/PM-SPEC-006-offline-queue.md | Normative | PLANNED | TBD |
| PM-SPEC-007 | Transport | docs/01-normative/PM-SPEC-007-transport.md | Normative | PLANNED | TBD |
| PM-SPEC-008 | Recovery | docs/01-normative/PM-SPEC-008-recovery.md | Normative | PLANNED | TBD |
| PM-SPEC-009 | Deletion | docs/01-normative/PM-SPEC-009-deletion.md | Normative | PLANNED | TBD |
| PM-SPEC-010 | Security Invariants & Threat Model | docs/01-normative/PM-SPEC-010-security-invariants.md | Normative | PLANNED | TBD |
| PM-SPEC-011 | Protocol Registry & Versioning | docs/01-normative/PM-SPEC-011-registry-versioning.md | Normative | IN REVIEW | CANONICAL DRAFT |
| PM-BASELINE-001 | V1 Protocol Baseline | docs/02-baseline/PM-BASELINE-001.md | Baseline | PLANNED | TBD |
| PM-AUDIT-003 | Normalization & Hardening Baseline | docs/12-audits/PM-AUDIT-003-normalization-baseline.md | Internal Review | IN REVIEW | CANONICAL REVIEW |
| PM-NET-REG-001 | PrivateMesh Canonical Authority | docs/05-network/PRIVATEMESH_CANONICAL_AUTHORITY.md | Governance | IN REVIEW | P2P GATE |
| PM-NET-GAP-001 | PrivateMesh Gap Matrix | docs/05-network/PRIVATEMESH_GAP_MATRIX.md | Internal Review | IN REVIEW | P2P GATE |
| PM-NET-INT-001 | Transport Interoperability Matrix | docs/05-network/PRIVATEMESH_INTEROPERABILITY_MATRIX.md | Candidate Evaluation | CANDIDATE | P2P GATE |
| PM-NET-LIM-001 | Resource & Abuse Limits | docs/05-network/PRIVATEMESH_RESOURCE_LIMITS.md | Security Control | REQUIRED | P2P GATE |
| PM-NET-SEC-001 | Security Review Checklist | docs/05-network/PRIVATEMESH_SECURITY_REVIEW_CHECKLIST.md | Security Review | REQUIRED | P2P GATE |
| PM-NET-EVID-001 | PrivateMesh Evidence Index | docs/05-network/PRIVATEMESH_EVIDENCE_INDEX.md | Evidence | GATED | P2P GATE |

## 4. Provenance states

| State | Meaning |
|---|---|
| SOURCE | Original supplied material |
| IMPORTED | Material copied into repository |
| RECONCILED | Material whose conflicts were resolved |
| CANONICAL | Current repository authority |
| SUPERSEDED | Replaced by a newer canonical artifact |
| REJECTED | Not accepted as protocol authority |

## 5. Required evidence for promotion to canonical

A normative specification is not promoted to CANONICAL until:

- requirements are explicit;
- dependencies and registry references resolve;
- security-critical ambiguities are closed;
- wire and cryptographic encodings are exact;
- negative behavior is defined;
- test vectors exist where applicable;
- implementation and tests are traceable;
- review status is recorded.

## 6. Known blockers

1. Protocol registry remains IN REVIEW; the 0x6201–0x6205 conflict is documented but final promotion is pending.
2. Undefined PQXDH KDF/transcript construction.
3. Wire header length inconsistency.
4. Ambiguous exactly-once semantics for the offline queue.
5. Recovery authority and epoch semantics.
6. Deletion semantics versus physical storage guarantees.
7. Placeholder cryptographic and wire test vectors.
8. Standards/reference verification and current applicability.
9. PrivateMesh transport stack is not yet approved by ADR-0012.
10. PrivateMesh numeric resource limits and executable network-fault evidence are not yet established.

## 7. P2P authority rule

The PrivateMesh documents under docs/05-network define the implementation boundary, not the concrete transport technology. Historical or candidate transport references MUST remain non-normative until ADR-0012 and its evidence gate are accepted.
