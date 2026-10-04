# Eagle — Decision Log

## Decision status vocabulary

- Accepted — approved for current scope.
- Proposed — candidate requiring evidence/approval.
- Pending — insufficient evidence.
- Rejected — evaluated and explicitly not selected.
- Superseded — replaced by a later decision.

## Decisions

### DEC-001 — Evidence before implementation
Status: Accepted  
Date: 2026-10-04

Repository claims must be grounded in executable or documentary evidence. Planned architecture is not implementation evidence.

Consequence: production-readiness, E2EE, PrivateMesh, and security-audit claims remain blocked until evidence exists.

### DEC-002 — Reuse-first engineering
Status: Accepted  
Date: 2026-10-04

Eagle evaluates mature external/platform components before implementing equivalent functionality.

Consequence: every security-sensitive dependency requires requirement fit, security/advisory review, exact version, license, platform/test evidence, and exit strategy.

Record: docs/14-audit/REUSE_FIRST_COMPONENT_REGISTER.md

### DEC-003 — No novel cryptographic primitives
Status: Accepted  
Date: 2026-10-04

Eagle will not implement new cryptographic primitives when established reviewed primitives satisfy the requirement.

Consequence: Eagle owns protocol composition and key lifecycle semantics, not replacement implementations of basic primitives.

### DEC-004 — Android Keystore/KeyMint as baseline platform candidate
Status: Proposed / Baseline candidate  
Date: 2026-10-04

Use Android platform key protection as the first candidate for device-held key material once the identity/key lifecycle contract is frozen.

Not yet implemented: no Keystore dependency or identity implementation is claimed by this decision.

### DEC-005 — libsignal direct adoption rejected for current scope
Status: Rejected  
Date: 2026-10-04

Direct adoption is not approved because upstream states that use outside Signal is unsupported.

Reconsideration requires explicit support, licensing, interoperability, security, and maintenance review.

### DEC-006 — Mesh is layered after secure messaging slice
Status: Accepted  
Date: 2026-10-04

Mesh discovery/routing/relay must not become the first implementation milestone.

Required predecessor: Device A → Identity → Session/Key Establishment → Encrypt → Envelope → Transport → Device B → Verify → Decrypt → Persist → Test Evidence.

### DEC-007 — Historical archives remain untrusted until intake
Status: Accepted  
Date: 2026-10-04

Filebin and other historical artifacts are not promoted based on filenames or prior claims.

Required intake: retrieve → hash → manifest → secret scan → duplicate detection → provenance → classification → promotion decision.

### DEC-008 — CI configuration is not CI evidence
Status: Accepted  
Date: 2026-10-04

Presence of workflow YAML is evidence of configuration only. Passing/release-grade evidence must reference an actual run and commit SHA.

### DEC-009 — Current SmokeTest is insufficient for product verification
Status: Accepted  
Date: 2026-10-04

The current tautological smoke test cannot verify security or product behavior.

Next test milestone: behavior-level test around the first implemented contract.

### DEC-010 — Documentation is a controlled project artifact
Status: Accepted  
Date: 2026-10-04

Material decisions, requirement changes, dependency choices, security assumptions, and verification results must update the canonical project records.

## Open decisions

- V1 requirements freeze.
- Trust-boundary model.
- Threat model.
- Protocol/envelope format.
- Identity/key lifecycle.
- Transport architecture.
- Persistent storage.
- Rust/KMP/UniFFI boundaries.
- Mesh architecture and abuse controls.
- Production dependency versions.
- Release/provenance gates.
