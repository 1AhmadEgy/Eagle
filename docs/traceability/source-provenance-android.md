# The Eagle — Android Source Provenance and Canonicalization Record

- Review date: 2026-10-06
- Primary source: `10. فريق Android_4085322700343523373.md`
- Scope: Android architecture/security/performance/transport/storage/calls/release material
- Canonicalization state: reconciled for current security-first baseline

## 1. Source characteristics

The reviewed file is a large accumulated engineering document containing multiple generations of design decisions, examples, implementation sketches, test plans, and historical alternatives.

Presence of text, code, or a "final" heading inside the source is not treated as proof of implementation.

The project baseline is derived only after:
- identifying repeated decisions;
- classifying conflicts;
- separating historical material from accepted decisions;
- attaching acceptance evidence requirements.

## 2. Accepted source-derived decisions

The source supports:

- Rust as the canonical security core.
- Android as platform/UI/lifecycle integration.
- typed FFI boundary.
- Room for metadata/indexes.
- encrypted/private application storage for ciphertext.
- Android Keystore as the platform key-protection boundary.
- one-to-one messaging/calls in the stated product scope.
- P2P direct connectivity when possible.
- TURN fallback where needed for connectivity.
- internal playback and restricted external output.
- explicit security/performance/release gates.

## 3. Superseded or conflicting material

The source contains older or conflicting material for:

### FCM/Firebase/Google runtime
The document contains both FCM-based paths and a later no-FCM/no-Firebase/no-Play-Services direction.

Current canonical decision: no FCM, no Firebase, no Google Play Services, no Play Integrity, no external runtime messaging dependency.

### Android minSdk
The document contains both minSdk 31 and an older minSdk 29 variant.

Current canonical decision: minSdk 31.

### Server-mediated application content
The document contains server/API/PostgreSQL/encrypted-envelope pipelines.

Current canonical decision: these are historical/deferred for the current P2P-only application-data baseline.

### Server Outbox
The document contains a complete Outbox implementation proposal.

Current canonical decision: not part of the application baseline. GitHub Issue #86 is the architecture gate.

## 4. Why canonicalization is necessary

Combining all source variants would create contradictory runtime behavior and security assumptions.

Examples:

- a no-FCM build cannot simultaneously depend on FCM wake semantics;
- a P2P-only application-data plane cannot silently adopt a server-content Outbox;
- minSdk 29 and 31 cannot both be the centrally enforced product baseline.

Therefore historical variants remain valuable as provenance but cannot remain mixed into implementation authority.

## 5. Repository records created from this review

- `docs/architecture/ADR-0001-security-first-runtime-boundary.md`
- `docs/security/security-first-baseline.md`
- `docs/traceability/canonical-requirements-matrix.md`
- `docs/security/threat-model-p2p.md`
- `docs/security/dependency-policy.md`
- `docs/performance/performance-gates.md`
- `docs/releases/security-first-release-gate.md`
- `docs/architecture/SECURITY-FIRST-INDEX.md`

## 6. Evidence state

The following remain UNKNOWN until executed and recorded:

- production code completeness
- physical-device benchmark results
- full FFI fault testing
- full fuzzing duration/results
- full network/MITM validation
- independent penetration testing
- final release AAB audit
- full device/version compatibility results

No UNKNOWN item is to be promoted to implemented without evidence.
