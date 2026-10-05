# Eagle — Identity & Trust Implementation Map

**Scope:** Identity & Trust only  
**Canonical baseline:** `main @ abfc263e6ac28ff7b19a40a4d8e1da93c565a6e9`  
**Current implementation candidate:** `implementation/security-first-v1-2026-10-05` / PR #77  
**Current head under verification:** `136f595d2dfa0fdd6350bda1d758353837c0f299`  
**Gate:** BLOCKED / NO-GO

| Area | Current artifact | State | Security boundary |
|---|---|---|---|
| Account / device identity | `core/src/identity.rs` | Implemented baseline | final key semantics remain external |
| Membership binding | `MembershipRegistry` | Implemented fail-closed | identity proof verifier required |
| Trust state machine | `TrustRecord` | Implemented | protocol/key integration pending |
| P2P pairing | `PairingContext` | Lifecycle implemented | authenticated transcript pending |
| Explicit approval | `PairingApprovalVerifier` | Required | production verifier must be protocol-backed |
| Identity-change quarantine | `ContactIdentity` | Implemented | exact-candidate reverification required |
| Revocation / epochs | `TrustRecord`, `TrustEpochSet` | Implemented local guards | distributed convergence pending |
| Recovery boundary | `AuthorizationAction` | Fail-closed | recovery authority protocol pending |
| Platform assurance | `PlatformAssurance` | Advisory only | must never create trust |
| Security events | `SecurityEvent` | Secret-safe baseline | final audit policy pending |

## Critical invariants

1. Unknown/PENDING devices cannot self-promote to trusted.
2. Pairing is bound to account, target device, and exact current trust epoch.
3. Pairing is expiring, single-use, and cancellation-final.
4. A future membership epoch is rejected; trust cannot be advanced by an unconfirmed remote statement.
5. An unverified identity replacement is quarantined as `pending_identity`; the last verified identity remains authoritative until exact-candidate reverification succeeds.
6. Revoked/replaced devices cannot start protected sessions.
7. Account recovery cannot imply historical message-key recovery.
8. Platform attestation/assurance is additive evidence, never the trust root.
9. Private cryptographic choices are not invented in this specialty.

## Lifecycle evidence

Inventory, provenance, classification, triage, deep analysis, reconciliation, conflicts, gaps, canonical authority, remediation, correction, implementation, testing, and security review are maintained as engineering evidence. Verification remains evidence-driven: no local source result is promoted to CI PASS.

## Release blockers

- approved cryptographic protocol and key-management decisions;
- authenticated P2P pairing transcript and anti-phishing ceremony;
- identity rotation/successor proof;
- distributed revocation convergence;
- persistent rollback protection;
- recovery authority/protocol;
- cross-platform equivalence;
- adversarial integration/fuzz/property evidence;
- independent architecture/security review;
- fresh exact-head CI/Test Lab evidence;
- complete provenance and release evidence.

No Identity & Trust release is authorized while any required blocker remains unresolved.
