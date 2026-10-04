# Eagle — Reuse-First Technology & Component Evaluation Register

**Date:** 2026-10-04  
**Status:** Evaluation framework established; individual components remain Pending until evidence is collected.

## Goal

Avoid unnecessary reinvention. Prefer mature, maintained, well-tested components when they satisfy Eagle's requirements without compromising the security model.

## Evaluation classes

### Cryptography
Evaluate mature implementations for:
- authenticated encryption;
- modern key agreement;
- signatures;
- secure randomness;
- key derivation;
- secure key storage integration.

**Policy:** no custom cryptographic primitive implementation unless explicitly justified and independently reviewed.

### Identity and key lifecycle
Evaluate:
- device identity formats;
- public-key identity;
- key rotation;
- revocation;
- recovery;
- secure hardware/Keystore integration.

### Protocol
Evaluate:
- binary serialization;
- canonical encoding;
- versioning;
- framing;
- replay protection patterns;
- authenticated envelopes.

### Networking / transport
Evaluate separately:
- local discovery;
- Bluetooth/Wi-Fi Direct/local network transport;
- Internet transport;
- relay;
- offline queueing;
- NAT traversal where required.

A transport library must not be mistaken for a complete Mesh protocol.

### Mesh
Evaluate:
- discovery;
- neighbor management;
- routing;
- relay;
- TTL/hop limits;
- duplicate suppression;
- congestion/backpressure;
- adversarial-node behavior;
- partition/reconnection behavior.

### Storage
Evaluate:
- encrypted local persistence;
- transactional storage;
- migration support;
- corruption recovery;
- secure deletion expectations;
- backup/export behavior.

### Testing
Prefer established tools for:
- unit tests;
- property-based testing;
- fuzzing;
- integration tests;
- Android instrumentation;
- protocol test vectors;
- static analysis;
- dependency analysis.

### Supply chain
Evaluate:
- lockfiles;
- SBOM generation;
- dependency review;
- artifact provenance;
- signature/attestation;
- reproducible builds where practical.

## Candidate decision table

| Component area | Candidate | Evidence | Decision | Reason |
|---|---|---|---|---|
| Crypto primitives | Pending survey | Pending | Pending | Must be selected from mature reviewed implementations |
| Identity | Pending survey | Pending | Pending | Requirements and trust model first |
| Protocol encoding | Pending survey | Pending | Pending | Must support canonical/versioned envelopes |
| Local transport | Pending survey | Pending | Pending | Platform-specific feasibility required |
| Mesh routing | Pending survey | Pending | Pending | No assumption that a generic mesh library satisfies security model |
| Storage | Pending survey | Pending | Pending | Data/security requirements not yet frozen |
| Property/fuzz testing | Pending survey | Pending | Pending | Select after protocol contracts exist |
| SBOM/provenance | Existing governance | Repository evidence | Foundation | Continue and integrate with actual build |

## Acceptance rule

No candidate becomes an Eagle dependency until its evaluation records:

1. exact version;
2. license;
3. maintenance status;
4. security/advisory history;
5. transitive dependencies;
6. supported platforms;
7. test evidence;
8. integration boundary;
9. failure behavior;
10. replacement/exit strategy;
11. human approval where security-sensitive.

## Anti-patterns prohibited

- adopting `latest`;
- copying an entire reference project without understanding its boundary;
- treating GitHub stars as security evidence;
- assuming a transport implementation is an E2EE protocol;
- assuming encryption at rest equals E2EE;
- using a dependency before its license and security posture are known;
- claiming an untested component is production-ready.
