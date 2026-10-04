# Eagle — Identity & Trust Test Matrix

| ID | Requirement | Test | Expected | Status |
|---|---|---|---|---|
| IT-001 | PENDING cannot self-promote | authorize protected session from PENDING | DENY | Implemented |
| IT-002 | Explicit approval required | approve trusted device through valid pairing | TRUSTED | Implemented |
| IT-003 | Account binding | approve with wrong account | DENY | Implemented |
| IT-004 | Pairing single-use | verify consumed pairing | DENY | Implemented |
| IT-005 | Pairing expiry | verify zero-TTL pairing | DENY | Implemented |
| IT-006 | Epoch freshness | authorize with stale epoch | DENY | Implemented |
| IT-007 | Revocation | authorize revoked device | DENY | Implemented |
| IT-008 | Revoked terminality | reinstate revoked device | DENY | Implemented |
| IT-009 | Replacement terminality | reinstate replaced device | DENY | Implemented |
| IT-010 | Recovery separation | request historical data authorization from account state | DENY | Implemented |
| IT-011 | Revocation epoch | revoke device increments monotonic epoch | PASS | Implemented |
| IT-012 | Pairing cancellation | resume cancelled pairing | DENY | Implemented |
| IT-013 | Identity key structural validity | construct empty public identity key | DENY | Implemented |
| IT-014 | Membership time-window validity | malformed membership validity interval | DENY | Implemented |
| IT-015 | Cross-account device reuse | bind an already-bound device to another account | DENY | Implemented |
| IT-016 | Duplicate membership binding | bind the same device twice for one account | DENY | Implemented |
| IT-017 | Membership epoch freshness | bind membership below current account epoch | DENY | Implemented |
| IT-018 | Identity-change quarantine | observe changed contact identity without reverification | QUARANTINE | Implemented |
| IT-019 | Unchanged identity update | submit identical identity as a change | DENY | Implemented |
| IT-020 | Suspended authorization | start protected session while SUSPENDED | DENY | Implemented |
| IT-021 | Trust epoch overflow | advance epoch at maximum value | DENY | Implemented |
| IT-022 | Trust epoch monotonicity | revoke/replace without increasing epoch | DENY | Implemented |
| IT-023 | Pairing device binding | consume a valid pairing for a different device | DENY | Implemented |
| IT-024 | Pairing context validity | construct pairing with empty token/device identifiers | DENY | Implemented |
| IT-025 | Membership validity upper bound | issued-at later than not-after | DENY | Implemented |
| IT-026 | Pairing expiry overflow | construct expiry beyond representable UNIX time | DENY | Implemented |

## Evidence rule

A PASS requires executable test evidence. Specification prose alone does not promote a case to PASS.

## Deferred adversarial matrix

The following requires protocol/key-management implementation and remains PENDING:
- transcript tampering;
- signature forgery;
- identity-key substitution;
- replay across accounts;
- cross-device authorization race;
- offline revocation reconciliation;
- serialization downgrade;
- malformed protocol framing;
- cross-platform interoperability;
- platform-attestation policy abuse;
- persistent-storage rollback.

These cases are intentionally not marked PASS before their actual protocol boundaries exist.

## Release criteria

The workstream can only progress to release readiness when:
- executable tests pass in CI;
- protocol and key-management interfaces are frozen;
- cross-platform bindings are verified;
- recovery semantics are approved;
- revocation reconciliation is tested end-to-end;
- external security review has no open critical/high findings affecting identity trust.


## Latest hardening coverage

IT-023 and IT-024 close two pre-protocol boundary gaps: pairing authorization is now cryptographically/protocol-agnostic but explicitly bound to the intended device identity at the Security Core boundary, and malformed pairing contexts are rejected before they can enter the lifecycle.

These tests do not replace the deferred end-to-end transcript-integrity, replay, signature-forgery, downgrade, rollback, or cross-platform interoperability tests.


Latest hardening extends the executable boundary matrix to IT-026. These remain Security Core negative tests; protocol-cryptographic and cross-platform cases remain deferred until their authoritative interfaces are frozen.
