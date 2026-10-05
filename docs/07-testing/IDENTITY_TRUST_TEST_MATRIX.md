# Eagle — Identity & Trust Test Matrix

**Scope:** Identity & Trust only  
**Release state:** BLOCKED

| ID | Scenario | Expected |
|---|---|---|
| IT-001 | PENDING self-promotion | DENY |
| IT-002 | Explicit pairing approval | TRUSTED |
| IT-003 | Rejected pairing approval | DENY |
| IT-004 | Cross-account device reuse | DENY |
| IT-005 | Duplicate device binding | DENY |
| IT-006 | Stale membership epoch | DENY |
| IT-007 | Malformed identity | DENY |
| IT-008 | Invalid membership window | DENY |
| IT-009 | Identity change observed | QUARANTINE; verified identity preserved |
| IT-010 | Reverification rejected | DENY; remain QUARANTINED |
| IT-011 | Reverification candidate mismatch | DENY; preserve verified identity |
| IT-012 | Successful exact-candidate reverification | VERIFIED |
| IT-013 | Pairing device mismatch | DENY |
| IT-014 | Pairing replay | DENY |
| IT-015 | Pairing expiry/cancellation | DENY |
| IT-016 | Stale trust epoch | DENY |
| IT-017 | Revoked/replaced device | DENY |
| IT-018 | Account recovery implies history recovery | DENY |
| IT-019 | Epoch overflow/non-monotonicity | DENY |

Protocol-dependent tests remain pending: transcript tampering, signature forgery, identity-key substitution, concurrent approval race, offline revocation convergence, downgrade, persistent rollback, and cross-platform interoperability.
