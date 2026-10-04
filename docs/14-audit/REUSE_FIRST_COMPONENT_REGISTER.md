# Eagle — Reuse-First Technology & Component Evaluation Register

Date: 2026-10-04  
Status: Framework established; first external due-diligence pass completed; no production dependency approved yet.

## Goal

Avoid unnecessary reinvention. Prefer mature, maintained, well-tested components when they satisfy Eagle's requirements without compromising the security model.

## Evidence policy

A component is not approved because it is popular, has many stars, or is used by another project.

Acceptance requires:

Requirement → Architecture Fit → Security History → Exact Version → License → Dependency Review → Platform Support → Test Evidence → Failure Behavior → Operational Fit → Exit Strategy → Human Approval

The concrete first-pass evaluations are recorded in:

- docs/14-audit/COMPONENT_DUE_DILIGENCE_2026-10-04.md

## Evaluation classes

### Cryptography

Evaluate mature implementations for:

- authenticated encryption;
- key agreement;
- signatures;
- secure randomness;
- key derivation;
- secure key storage integration.

Policy: no custom cryptographic primitive implementation unless explicitly justified and independently reviewed.

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

## First concrete candidate register

| Area | Candidate | Current status | Decision |
|---|---|---|---|
| Device key protection | Android Keystore / KeyMint | Platform capability verified | Baseline candidate |
| E2EE protocol | Signal libsignal | Protocol implementation exists; official project states use outside Signal is unsupported | Rejected for direct adoption at this stage |
| AEAD primitives | RustCrypto AEADs | Active collection; primitive-level scope | Pending |
| Rust crypto backend | aws-lc-rs | Active 1.x line; Apache-2.0/ISC; Rust binding to AWS-LC | Pending |
| Mesh/networking | rust-libp2p | Mature ecosystem; active security policy; 2026 advisories require continuous patching | Pending / heightened scrutiny |
| Encrypted local DB | sqlcipher-android 4.19.1 | Current Android line; API 23+; supports current Android packaging requirements | Pending |
| Shared Kotlin logic | Kotlin Multiplatform | Android/iOS stable | Architecture candidate |
| Rust bridge | Mozilla UniFFI | Extensively used by Mozilla for Kotlin/Swift bindings | Architecture candidate |
| Rust SCA | cargo-audit / RustSec | Established advisory workflow | Tool candidate |
| License/dependency policy | cargo-deny | Licenses/bans/advisories/sources checks | Tool candidate |
| Multi-ecosystem SCA | OSV-Scanner | Scans source, lockfiles, SBOMs and git directories | Tool candidate |
| Rust binary provenance | cargo-auditable | Embeds dependency metadata into binaries | Tool candidate |
| Protocol fuzzing | cargo-fuzz | libFuzzer integration and corpus/coverage workflow | Tool candidate |

## Decision boundaries

### Adoptable now

Only components that are platform primitives already required by the verified Android application baseline may be introduced without waiting for the entire future product architecture.

Android Keystore is therefore a baseline candidate, not yet an instruction to add code.

### Pending until requirements are frozen

Crypto, protocol, Mesh, encrypted database, KMP, and UniFFI choices remain pending until their exact Eagle contract is known.

### Rejected for direct dependency

The current libsignal evaluation is explicitly rejected for direct dependency adoption because the upstream project states that use outside Signal is unsupported. Reconsideration would require an explicit compatibility/support/licensing/security decision.

## Acceptance checklist

Before changing Eagle dependencies, record:

1. Requirement ID.
2. Exact version/tag/commit.
3. License and notice obligations.
4. Security/advisory review date.
5. Transitive dependency set.
6. Supported Android/iOS/Desktop/Rust targets as applicable.
7. Test and interoperability evidence.
8. Integration boundary.
9. Failure behavior and recovery semantics.
10. Performance/resource implications.
11. Replacement/exit strategy.
12. Human approval for security-sensitive choices.

## Anti-patterns prohibited

- adopting latest;
- floating + versions;
- unreviewed Git dependencies;
- copying an entire reference project;
- treating GitHub stars as security evidence;
- assuming transport equals E2EE;
- assuming encryption at rest equals E2EE;
- claiming a component is production-ready without evidence.
