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

**Verified execution head for the latest code update:** `48445f40c6bdced9e94db93e215f6430e32d7156`

- Rust Security Core: 26/26 unit tests expected after the latest fail-closed hardening update; CI verification remains required.
- Repository verification: PASS.
- Secret scan: PASS.
- Security policy verification: PASS.
- Android Unit Tests: PASS.
- Android Lint: PASS.
- Android Debug Build: PASS.
- CodeQL: PASS.
- Gradle dependency submission: PASS.

These results verify the implemented baseline only. They do not approve the unresolved protocol/key-management/recovery ADRs.


## Identity & Trust hardening evidence

The pairing lifecycle is explicitly bound to the target device identity in the Security Core. A valid account/epoch pairing context cannot be consumed to promote a different pending device. Empty pairing identifiers are rejected at construction, membership validity windows are bounded, expiry arithmetic fails closed on overflow, and the test suite includes negative coverage for each case.

The release gate remains **BLOCKED** because these implementation controls do not substitute for the unresolved ADR-005/006/007 approvals, cryptographic/key-management freeze, protocol adversarial proof, cross-platform parity, revocation reconciliation, and independent security review.
