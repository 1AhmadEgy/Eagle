# Eagle — Identity & Trust Release Gate

## Gate state

**BLOCKED**

## Mandatory conditions

| Gate | Required evidence | State |
|---|---|---|
| Identity specification approved | ADR-005 decision | BLOCKED — Open |
| Pairing specification approved | ADR-006 decision | BLOCKED — Open |
| Recovery boundary approved | ADR-007 decision | BLOCKED — Open |
| Key identity hierarchy frozen | Key Management ADR | BLOCKED |
| Protocol transcript frozen | Protocol/Crypto ADR | BLOCKED |
| Rust Security Core tests | CI evidence | PENDING |
| Negative/adversarial tests | Test Lab evidence | PARTIAL |
| Cross-platform parity | Android/Desktop/iOS verification | PENDING |
| Revocation reconciliation | End-to-end evidence | PENDING |
| Security review | reviewer sign-off | PENDING |
| Release artifact/provenance | CI/release evidence | PENDING |

## Stop-the-line conditions

Immediate block on:
- identity-key substitution accepted without explicit reverification;
- unauthorized trust elevation;
- revoked device accepted for new authorization;
- stale trust state restoring authority;
- recovery bypass;
- plaintext/private-key exposure;
- pairing replay;
- protocol downgrade;
- unreviewed platform-specific reimplementation of trust policy.

## Required transition

```text
BLOCKED
  ↓
ADR approval
  ↓
Key/Protocol freeze
  ↓
Implementation
  ↓
Executable tests
  ↓
Independent verification
  ↓
Security review
  ↓
Release Gate PASS
```

No stage may be skipped by administrative declaration.


## Verified evidence snapshot

**Verified execution head before this documentation-only update:** `cdab10de4f3c2e374f12976230fb71c59ea7b0e2`

- Rust Security Core: 14/14 unit tests passed.
- Repository verification: PASS.
- Secret scan: PASS.
- Security policy verification: PASS.
- Android Unit Tests: PASS.
- Android Lint: PASS.
- Android Debug Build: PASS.
- CodeQL: PASS.
- Gradle dependency submission: PASS.

These results verify the implemented baseline only. They do not approve the unresolved protocol/key-management/recovery ADRs.
