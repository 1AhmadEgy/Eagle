# Eagle Security Release Gate — 2026-10-05

## Gate policy

A release can pass only when all applicable security requirements have executable evidence.

## Blocking gates

### G0 Corpus
- all retrievable artifacts classified;
- unretrieved ChatGPT-only artifacts marked Pending;
- provenance links preserve original source and Git identity.

### G1 Architecture
- approved requirements;
- approved trust boundaries;
- approved platform scope;
- approved transport policy;
- no unresolved critical architecture conflict.

### G2 Cryptography and keys
- approved message protocol;
- approved key lifecycle;
- secure platform key custody;
- canonical serialization;
- conformance vectors;
- negative/misuse tests.

### G3 Implementation
- Rust Security Core boundary enforced;
- no duplicated cryptographic logic in platform/UI layers;
- direct P2P data path enforced;
- storage contains no private keys.

### G4 Adversarial verification
- MITM;
- replay;
- downgrade;
- identity substitution;
- pairing;
- revocation race;
- compromised-device scenarios;
- parser/resource exhaustion;
- recovery abuse;
- deletion/rollback;
- cross-platform parity;
- fuzz/property testing.

### G5 Supply chain
- full SHA-pinned external GitHub Actions;
- secret scan;
- dependency/SCA scan;
- SBOM;
- provenance/attestation;
- license review;
- controlled dependency upgrades.

### G6 Independent review
- cryptographic review;
- security architecture review;
- platform/FFI review;
- remediation evidence.

### G7 Release
- no open P0/P1 security blockers;
- all supported targets verified;
- rollback/recovery tested;
- release artifact identity and provenance verified;
- human approval recorded.

## Immediate no-go conditions

- protocol or key-management ADR remains Proposed/Pending;
- relay/content forwarding is enabled;
- trust can self-promote;
- revoked/stale membership can regain authority;
- secrets are present in code/artifacts/logs;
- required security test category is missing;
- CI third-party actions are not fully pinned;
- independent review is absent.

## Current gate

**BLOCKED — foundational security slice only.**
