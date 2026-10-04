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
