# Rust Core / Security Kernel — Version Comparison & Canonicalization

**Date:** 2026-10-05  
**Scope:** Rust Core / Security Kernel only

## Version set inspected

| Version | Reference | Classification | Disposition |
|---|---|---|---|
| V0 | `execution/phase-1-security-kernel` | historical execution branch | superseded |
| V1 | `execution/phase-1-security-kernel-verified-2026-10-04` | verified candidate | superseded by hardened branch |
| V2 | `execution/rust-core-security-kernel-complete-2026-10-05` | current specialization | canonical candidate for this specialization |

## Reconciliation

### V0
Contained a public `SecurityContext` representation and public authentication promotion. Protocol negotiation was not bounded by a maximum/current version.

Security disposition: **rejected for promotion** because public mutable security state and unconstrained negotiation increase the privilege/bypass surface.

### V1
Made core state private and retained fail-closed authorization, but still relied on a public `authenticate()` transition.

Security disposition: **rejected as final kernel boundary** because trust elevation must remain behind a verified internal/authentication seam until the cryptographic verifier exists.

### V2
Adds:
- explicit `Authenticating`, `Authenticated`, `Established`, and `Rekeying` states;
- private security-sensitive fields;
- internal-only verified-authentication seam;
- bounded minimum/maximum protocol handling;
- constructor validation for identifiers and envelopes;
- immutable public representations;
- terminal revocation/replacement behavior;
- expanded negative-path tests;
- dedicated security contract/threat model/release gate.

Security disposition: **selected as the current specialization baseline**.

## Canonical authority

The current canonical candidate for Rust Core implementation is:

`execution/rust-core-security-kernel-complete-2026-10-05`

This is **branch-local canonicality for the specialization**, not repository-wide canonicality. Repository-wide canonicality requires the normal review/merge process.

## Explicitly not promoted

No historical crypto choice, key-management scheme, serialization format, transport design, or FFI implementation is promoted by this comparison.

## Security preference

When two implementations satisfy the same functional requirement, prefer the one with:
- smaller trusted computing base;
- fewer public mutation paths;
- fail-closed behavior;
- explicit bounds;
- immutable representations;
- fewer dependencies;
- stronger negative-path evidence;
- simpler auditability.

## Verification status

The current dedicated Rust Security Kernel workflow for code head `02e1a85cafdbd3c0331ca0745d43d0b07dfb93a4` is still queued in the observed GitHub state. Therefore current-head verification is not marked PASS.
