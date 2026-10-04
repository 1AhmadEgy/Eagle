# Protocol Engineering Execution Ledger — 2026-10-05

Scope: Protocol specialization only.
Canonical product constraint: P2P-only messaging; no server-mediated message relay is acceptable as a product fallback.

## Stage 1 — Inventory
- Main planning map declares PROTO-001..005.
- Main repository does not contain an accepted production protocol module.
- Structural Rust protocol implementation exists on execution/security branches, not as an accepted canonical production protocol.
- Historical protocol material is retained as non-canonical evidence.

## Stage 2 — Provenance
- Main HEAD verified as commit abfc263e6ac28ff7b19a40a4d8e1da93c565a6e9.
- Branch-local implementation examined on security/reconciled-foundation-2026-10-05.
- New protocol security profile recorded on execution/protocol-security-baseline-2026-10-05.

## Stage 3 — Classification
- Accepted decision: none for protocol/serialization/crypto/transport.
- Proposed decisions: ADR-0008, ADR-0009, ADR-0010.
- Transport remains decision input under ADR-0012.
- Structural Rust contract is implementation scaffolding, not a wire protocol implementation.

## Stage 4 — Version comparison
- Rust structural contract was compared against required protocol properties.
- Historical/private-mesh specifications were treated as evidence only.
- No historical document was promoted to canonical protocol authority.

## Stage 5 — Canonical reference
Canonical protocol authority is intentionally NOT assigned yet.
Reason: authenticated session profile, serialization, key management, and transport decisions are not all approved.

## Stage 6 — Requirements / gap matrix
Closed by this execution profile:
- no custom cryptographic protocol;
- P2P-only boundary;
- fail-closed version handling;
- bounded parser/frame resource rules;
- explicit downgrade prohibition;
- protocol/crypto/mesh separation.

Open critical gaps:
- authenticated version/capability negotiation;
- Signal-family integration decision and exact revision;
- group MLS integration decision and exact revision;
- replay/idempotency/order/expiry semantics;
- deterministic serialization profile;
- transcript and epoch binding;
- fragmentation/reassembly profile;
- cross-platform wire/crypto vectors;
- independent protocol security review.

## Stage 7 — Correction
- Recorded the strongest security profile without silently approving dependencies.
- Explicitly excluded relay fallback from the canonical P2P architecture.
- Explicitly prohibited custom cryptographic protocol construction.
- Added release-gate conditions tied to ADR acceptance and evidence.

## Stage 8 — Implementation
- structural Rust envelope/frame validation: present on security branch;
- bounded IDs and ciphertext: present;
- exact frame payload length checking: present;
- local version downgrade/unsupported-version checks: present;
- cryptographic session implementation: not accepted/not implemented;
- serialization implementation: not accepted/not implemented;
- authenticated negotiation: not implemented;
- replay semantics: not implemented.

## Stage 9 — Testing
Required before protocol approval:
- positive and negative protocol vectors;
- fuzzing of unauthenticated parser inputs;
- replay/ordering/duplicate tests;
- downgrade/capability tests;
- cross-platform conformance;
- deterministic byte vectors;
- upstream crypto test vectors where available;
- direct-P2P connectivity failure matrix.
Current status: gate not closed.

## Stage 10 — Security audit
Current external evidence confirms that Double Ratchet provides per-message key evolution plus forward security and break-in recovery properties, and Signal's current specification describes a Triple Ratchet hybrid composition.
PQXDH is designed for asynchronous key agreement.
OpenMLS currently lists 2026 advisories including a high-severity improper tag validation issue and moderate parser/DoS issues; exact revision selection therefore remains a security-gated decision.

## Stage 11 — Verification
- New security profile commit: 6eb73d3eab0ab098192c22fc1806f53517a3fe39.
- Profile file SHA: cd9e581a3206a67de085b73f18bff400c09c3ffb.
- GitHub issues #30, #32, and #60 were updated with the current protocol security position.
- No protocol approval was asserted.

## Stage 12 — Release Gate
PROTOCOL GATE = NOT APPROVED / PRODUCTION BLOCKED.

Approval requires all of:
1. ADR-0008 accepted with exact 1:1 protocol profile.
2. ADR-0009 accepted with exact key-management architecture.
3. ADR-0010 accepted with exact deterministic serialization profile.
4. ADR-0012 accepted with explicit P2P-only transport composition.
5. Exact dependency revisions/provenance recorded.
6. Full conformance, negative, replay, fuzz, and interoperability evidence.
7. Independent protocol/security review recorded.
8. No unresolved critical/high vulnerability applicable to selected revisions.

## Security conclusion
The safest available path is reuse of mature, independently specified protocols with strict composition boundaries. Eagle must not invent a new messaging cryptosystem or a custom group-key protocol. Direct P2P remains mandatory; relay fallback is excluded.