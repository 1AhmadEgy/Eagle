# Eagle — Identity & Trust Release Gate

**Gate:** BLOCKED  
**Scope:** Identity & Trust only  
**Fail-closed rule:** local tests or platform assurance cannot override a missing cryptographic/protocol/security gate.

## Required before release

- approved canonical identity and trust specification;
- approved cryptographic identity binding and account/device key hierarchy;
- authenticated P2P pairing transcript and human-verifiable anti-phishing ceremony;
- identity rotation/successor proof;
- distributed revocation convergence;
- persistent rollback protection;
- recovery authority/protocol;
- cross-platform behavioral equivalence;
- adversarial protocol/integration evidence;
- independent security review;
- complete release provenance.

## Current local evidence

The Security Core implements fail-closed state transitions, explicit verifier boundaries, pairing binding/expiry/single-use/cancellation, trust epochs, revocation/replacement guards, recovery separation, and quarantine that preserves the last verified identity while holding an untrusted replacement candidate separately.

## External blockers

Repository-wide CI/security-policy failures and platform test-lab infrastructure failures remain release blockers and are not reclassified as Identity & Trust passes.

**Terminal state: IDENTITY_TRUST = BLOCKED.**
