# Cryptography / Key Management — Execution Record — 2026-10-05

## Scope

Specialty only. Evidence-first, fail-closed, no bespoke cryptography.

## Completed in this cycle

- Deterministic key lifecycle state machine.
- Purpose-separated key domains.
- Monotonic generation and lifecycle epoch rules.
- Atomic metadata rotation with pre-reserved epoch allocation.
- Explicit revoke/destroy transitions.
- Single-use One-Time PreKey consumption.
- Fail-closed lifecycle error mapping.
- Explicit production-provider approval policy.
- Provider selection register.
- Key lifecycle specification.
- Extended lifecycle/provider test matrix.
- Regression coverage for failed rotation atomicity and metadata-capacity exhaustion.
- Canonicalized Rust exports after reconciling the pre-existing `key_management` policy API with the new cryptography/key-lifecycle API; overlapping legacy types are now exposed under `Policy*` aliases instead of colliding with canonical crypto types.

## Provider status

No production cryptographic provider is approved.

libsignal remains reference/conformance material pending license/support/provenance/platform/security approval. vodozemac remains a research/component candidate only because it does not implement Eagle's full PQXDH profile.

## Verification evidence

- Current branch: `execution/crypto-key-lifecycle-canonical-2026-10-05`.
- Pull request: #73.
- Current PR head: `e7a4fdb1ba0c1a1039f99f24934060999cc8c020`.
- Base: `security/reconciled-foundation-2026-10-05` at `a5ed695466d61c59a7d7f27fc6035ff94dde6cac`.
- PR state is open and GitHub currently reports it as mergeable; human review is still required.
- GitHub Actions now has an actual Rust Security Kernel run for this exact head (run #136), but at the latest verification it remained queued with no conclusion. The independent CI gate is therefore still **PENDING**, not PASS.
- A local execution attempt was blocked because the execution environment has no `cargo`, `rustc`, or `rustfmt` toolchain installed. No local test PASS is claimed.

## Release disposition

Design and implementation boundary: COMPLETE.

Executable production crypto: NOT VERIFIED.

CI verification: PENDING.

Independent cryptographic review: REQUIRED.

Production release: BLOCKED.

## Re-entry rule

Any provider, protocol, platform-keystore, serialization, migration, recovery, or key-lifecycle change restarts the specialty lifecycle at Inventory.
