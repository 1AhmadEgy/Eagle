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

- Current branch: `execution/crypto-key-lifecycle-canonical-v4-2026-10-05`.
- - Current reconciled head: `9a4c4cb4452b1eb961caa9489896bb2aa425b01a`.
- Base: `security/reconciled-foundation-2026-10-05` at `b49bbbb1724f1dbcd2e498dfcf2d12c49f2c0a35`.
- PR state is open and GitHub currently reports it as mergeable; human review is still required.
- The prior CI cycle reached execution and exposed two verified issues: Rust 1.99 rejected the array comparison inside `const fn`, and rustfmt reported workspace formatting drift. Both causes were corrected without changing cryptographic behavior. An additional security review then closed a `u64::MAX` epoch-allocation overflow edge case with a regression test. The specialty was then rebuilt on the current canonical foundation; no PASS is claimed until CI for this reconciled head is observed.
- A local execution attempt was blocked because the execution environment has no `cargo`, `rustc`, or `rustfmt` toolchain installed. No local test PASS is claimed.

## Release disposition

Design and implementation boundary: COMPLETE.

Executable production crypto: NOT VERIFIED.

CI verification: PENDING.

Independent cryptographic review: REQUIRED.

Security review finding closed in this cycle: lifecycle epoch reservation now rejects any multi-epoch allocation that would exceed the representable range before mutating state.

Production release: BLOCKED.

## Re-entry rule

Any provider, protocol, platform-keystore, serialization, migration, recovery, or key-lifecycle change restarts the specialty lifecycle at Inventory.
