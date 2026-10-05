# Eagle — Identity & Trust Test Matrix

**Scope:** Identity & Trust only  
**Release state:** BLOCKED

| ID | Scenario | Expected |
|---|---|---|
| IT-001 | PENDING self-promotion | DENY |
| IT-002 | explicit pairing approval | TRUSTED |
| IT-003 | rejected pairing approval | DENY |
| IT-004 | cross-account device reuse | DENY |
| IT-005 | duplicate device binding | DENY |
| IT-006 | stale membership epoch | DENY |
| IT-007 | future membership epoch | DENY |
| IT-008 | malformed identity | DENY |
| IT-009 | malformed membership window | DENY |
| IT-010 | identity change observed | QUARANTINE; verified identity preserved |
| IT-011 | rejected reverification | DENY; remain QUARANTINED |
| IT-012 | candidate mismatch | DENY; preserve verified identity |
| IT-013 | exact-candidate successful reverification | VERIFIED |
| IT-014 | pairing device mismatch | DENY |
| IT-015 | pairing replay | DENY |
| IT-016 | pairing expiry/cancellation | DENY |
| IT-017 | stale trust epoch | DENY |
| IT-018 | revoked/replaced device | DENY |
| IT-019 | account recovery implies history recovery | DENY |
| IT-020 | trust epoch overflow/non-monotonicity | DENY |

## Deferred protocol-dependent tests

Transcript tampering, signature forgery, identity-key substitution, concurrent approval races, offline revocation convergence, downgrade/framing, persistent rollback, cross-platform interoperability, platform-assurance abuse, and recovery interoperability remain pending until their governing technical contracts are frozen.

Missing evidence is never treated as PASS.
