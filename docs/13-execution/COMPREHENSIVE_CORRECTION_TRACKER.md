# Eagle Comprehensive Correction Tracker

**Status:** ACTIVE  
**Scope:** repository-wide research, correction, security, testing, and release readiness  
**Canonical repository:** `1AhmadEgy/Eagle`

## Operating rule

No unresolved architectural, cryptographic, privacy, ownership, or release decision is silently converted into an implementation assumption.

Each correction must be traceable to:
1. source/provenance;
2. requirement or finding;
3. decision/ADR where required;
4. implementation change;
5. test/security evidence;
6. verification status.

## Current correction lanes

| Lane | Current state | Gate |
|---|---|---|
| Repository description | Corrected in PR #18 | Review/merge |
| Source/provenance | Active | Evidence completeness |
| Architecture | Controlled | ADR decisions |
| Cryptographic protocol | Blocked by open decisions | OI-001/OI-002 |
| Transport | Blocked | OI-003 |
| Server retention/deletion | Blocked | OI-004/OI-008 |
| Identity/device trust | Controlled | OI-005/OI-006 |
| Recovery | Blocked | OI-007 |
| Android security | Active | MASVS/MASTG evidence |
| Test Lab | Active | Category-specific evidence |
| Supply chain | Active | Provenance/SBOM/attestation |
| Release | Blocked | Release Gate |

## Open decision controls

The historical decision pack records ADR-001 through ADR-008 as open and mapped to OI-001 through OI-008. These remain open until an explicit decision is recorded and independently verifiable.

### OI-001 — exact Signal implementation/version
Required evidence: selected implementation, exact version/commit, license, supported platforms, maintenance posture, security review, and PoC/interoperability evidence.

### OI-002 — PQXDH integration profile
Required evidence: exact profile, key/identity mapping, parameter set, compatibility requirements, test vectors, and interoperability evidence.

### OI-003 — V1 transport boundary
Required evidence: direct/relay model, protocol choices, NAT traversal, offline behavior, privacy implications, and explicit V1 scope.

### OI-004 — server ciphertext retention
Required evidence: whether ciphertext is retained, duration, encryption/access controls, deletion semantics, backup/replica behavior, and deletion evidence.

### OI-005/OI-006 — identity, trust, and device linking
Required evidence: state machines, authorization rules, revocation, pairing/linking protocol, recovery interaction, and negative/security tests.

### OI-007 — recovery
Required evidence: separation of account recovery from encrypted-data recovery, threat model, user/device flows, cryptographic consequences, and abuse resistance.

### OI-008 — deletion guarantees
Required evidence: local storage, caches, queues, replicas, backups, logs, and verifiable deletion boundaries.

## Security correction rule

Security claims are not accepted from documentation alone. A claim becomes VERIFIED only when implementation and appropriate test/evidence artifacts support it.

## Release rule

The project remains NOT RELEASE READY until critical decisions, implementation evidence, security testing, provenance/supply-chain checks, and independent verification satisfy the applicable release gate.

## Governance note

Repository visibility is currently public while project documentation records an intended private/all-rights-reserved posture. This is an owner-controlled governance issue and is not treated as resolved by documentation.
