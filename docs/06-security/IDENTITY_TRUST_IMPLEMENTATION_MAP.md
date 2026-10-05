# Eagle — Identity & Trust Implementation Map

**Scope:** Identity & Trust only  
**Baseline:** main  
**Working branch:** execution/identity-trust-clean-v1  
**Gate:** BLOCKED

| Area | Artifact | State | Blocking dependency |
|---|---|---|---|
| Identity model | IDENTITY_TRUST_SPECIFICATION.md | Implemented baseline | final identity/key authority |
| Threat model | IDENTITY_TRUST_THREAT_MODEL.md | Maintained | protocol/key review |
| Trust state machine | security-core/src/lib.rs | Implemented fail-closed | protocol integration |
| Membership binding | MembershipProofVerifier | Explicit proof boundary | approved identity proofs |
| P2P pairing | PairingContext | Lifecycle implemented | authenticated transcript |
| Approval | PairingApprovalVerifier | Explicit verifier required | final ceremony |
| Identity change | ContactIdentity | Quarantine implemented | successor/reverification proof |
| Revocation | TrustRecord / TrustEpochSet | Local baseline | distributed convergence |
| Recovery | AuthorizationAction | Historical recovery denied by default | recovery authority protocol |
| Platform assurance | PlatformAssurance | Advisory only | platform adapters |
| Security events | SecurityEvent | Secret-free/redacted | approved audit policy |

## Critical invariant

When an identity change is observed, the last verified identity remains the trusted identity. The replacement is held only as pending_identity while quarantined. Promotion requires an exact candidate match and an explicit ContactReverificationVerifier.

## Scope boundary

This specialty does not select cryptographic algorithms, key hierarchy, authenticated transcript construction, serialization, ratchets, transport, or recovery authority. Those remain external dependencies and release blockers.

## Verification

Local state/negative tests are maintained. Fresh repository CI and cross-platform/protocol evidence are mandatory before release.
