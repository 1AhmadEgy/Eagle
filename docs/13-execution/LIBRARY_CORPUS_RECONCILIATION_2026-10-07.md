# Eagle Library Corpus Reconciliation — 2026-10-07

## Authority

This is an evidence/provenance record only.

- Canonical execution reference remains `main @ 46aa86b6d71d34396b86b35124392cb3ba49c2e9`.
- Library archives, historical packages, conversation exports, branch names, and `FINAL`/version labels do not become implementation authority automatically.
- No cryptographic, protocol, storage, transport, AI, or release decision is approved by this record.

## Library evidence observed

Accessible Library folder:

`/Eagle`

Key current artifacts:

| Artifact | Size | SHA-256 |
|---|---:|---|
| `Download.zip` | 6,946,895 | `7ec3ea13406fa9497b2ffab5db39935710d2046e116127f8a3a346b2b823d8d8` |
| `Eagle_CONTEXT_LIBRARY_CONVERSATION_2026-10-07.zip` | 7,511,068 | `f08b536b74ad6d1a23768ad7fd77d7bdff9ab92a6cc3041cb8697b97990ef811` |
| `Eagle_MASTER_HANDOFF_v1.2.zip` | 62,716 | `eb8ffa1969ee316e09699b98b7a6b7077a1c7757f4581d31ec9b4f75a4c2d77e` |

The 2026-10-06 Library corpus manifest records:

- 2,107 leaf-file occurrences;
- 17,554,046 bytes across the recursive Download corpus;
- 888 unique SHA-256 values;
- 233 exact duplicate-hash groups;
- 1,452 occurrences belonging to duplicate groups.

Duplicate occurrences must not be treated as separate unique project sources.

## Requirement evidence recovered from historical corpus

The consolidated corpus contains a historical Software Requirements Specification with:

### Must / earliest secure-core scope

- device identity generation;
- local cryptographic key generation;
- authenticated secure sessions;
- session renewal after interruption/expiry;
- end-to-end message encryption;
- message integrity/authenticity verification;
- encrypted local sensitive-data storage;
- core security controls.

### Later / extended scope

- peer discovery;
- relay-based delivery;
- chunked file transfer;
- multi-device identity;
- multi-hop routing;
- limited AI assistance.

### Security requirements

- no plaintext private-key storage;
- rejection of unauthenticated/tampered messages where required;
- replay prevention/freshness;
- downgrade rejection unless explicitly approved;
- key rotation/revocation;
- safe logs;
- AI cannot control key management or final security decisions.

### Operational requirements

- deployment and rollback;
- monitoring/alerting;
- backup restore;
- incident response.

These historical requirements are useful source evidence, but they are not yet the final accepted Eagle MVP requirement baseline because the current repository deliberately keeps `REQUIREMENTS_TRACEABILITY.md` in Pending state.

## Important reconciliation findings

### RQ-001 — Historical relay wording vs P2P-only constraint

The historical SRS includes relay-based delivery as a later capability. The current project security baseline is P2P-first/P2P-only for application data.

Disposition:

- historical relay wording remains evidence;
- a relay may only be considered if its exact role is explicitly compatible with the current P2P trust boundary;
- server-mediated application-data authority must not be inferred from the historical SRS;
- transport/service decisions require explicit ADR and evidence.

### RQ-002 — Historical ADR references do not equal current ADR approval

Historical RTM entries refer to ADR-0001..0006 for earlier choices, while the current repository ADR index requires explicit compatibility verification before the newer proposed ADR sequence is accepted.

Disposition:

- historical ADR references remain provenance;
- no technical choice is promoted solely because the historical RTM names an ADR;
- current acceptance must be tied to an explicit repository ADR state and review record.

### RQ-003 — Historical AI requirement vs current AI boundary

The historical SRS describes limited AI assistance. Current Eagle governance explicitly keeps AI outside security authority and treats AI Labs material as proposed/documentation-only.

Disposition:

- AI assistance is a future/candidate requirement;
- no AI component may alter identity, trust, authorization, recovery, deletion, key management, or release decisions;
- AI implementation remains gated behind explicit requirements, architecture, tests, and human review.

### RQ-004 — Platform document path mismatch

The accepted platform baseline names `androidApp/`, while the actual current Android module on `main` is `app/`.

Disposition:

- documentation mismatch remains open;
- no directory rename is performed as an automatic correction because it could alter build/integration assumptions;
- correction requires repository evidence and targeted change review.

## Current exact-head CI evidence

PR #93 head:

`454afff3284b85a252d6244a7b466efef072572d`

Verified successful:

- CI run `37630478204`;
- Eagle Test Lab run `37630478217`;
- CodeQL run `37630473244`;
- Gradle dependency submission run `37630181754`.

The CI/Test Lab remediation changes only workflow pinning and Android SDK package selection. It does not change product security semantics.

## Gate state

`main @ 46aa86b6d71d34396b86b35124392cb3ba49c2e9` remains the canonical implementation reference.

Release remains:

**BLOCKED / NO-GO**

Remaining P0 classes include:

- final accepted product requirements;
- accepted crypto/key-management/serialization/storage/transport decisions;
- production Security Core/KMP/P2P implementation;
- cross-platform implementation evidence;
- protocol conformance/interoperability;
- adversarial/fuzz/property testing;
- main protection enforcement;
- SBOM/provenance/signing;
- independent verification;
- required external security audit;
- human approval.

## Next deterministic sequence

`PR #93 evidence -> human review/integration decision -> exact-main re-verification -> requirements decision/traceability closure -> ADR closure -> implementation -> security/negative/conformance testing -> provenance -> independent verification -> Release Gate`

No step may promote historical or branch-local content to canonical status without the required evidence and acceptance path.
