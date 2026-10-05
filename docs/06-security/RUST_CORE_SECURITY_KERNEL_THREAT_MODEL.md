# Rust Core / Security Kernel — Threat Model

**Date:** 2026-10-05  
**Scope:** Rust Core only  
**Posture:** fail-closed, least privilege, no unsafe Rust

## Assets

| Asset | Required property |
|---|---|
| Trust state | integrity, monotonic security posture |
| Session state | integrity, no unauthorized transition |
| Negotiated protocol | authenticity of bounds, no downgrade |
| Capability decision | least privilege, fail closed |
| Envelope structure | parser safety and bounded resource use |
| Security API boundary | no privilege escalation through representation |

## Trust boundaries

```
Platform Adapter / KMP
        |
        v
   Rust Security Core
        |
        +---- authenticated peer (future protocol boundary)
        |
        +---- secure storage (future key boundary)
```

The peer, storage, and platform layers are hostile or untrusted inputs from the kernel's perspective. The kernel accepts only validated domain values and explicit state transitions.

## Threat controls

| Threat | Control | Current status |
|---|---|---|
| Untrusted caller obtains authority | guarded authorization | Implemented |
| Public trust self-promotion | trust promotion kept internal | Implemented |
| Session establishment from unauthenticated state | state precondition | Implemented |
| Protocol downgrade | monotonic negotiation + bounds | Implemented |
| State mutation on rejected input | validate before mutation | Implemented |
| Oversized identifiers | constructor bounds | Implemented |
| Oversized ciphertext | envelope validation | Implemented |
| Frame length confusion | exact length check | Implemented |
| Revoked device reuse | terminal device state | Implemented |
| Replaced device reuse | terminal device state | Implemented |
| Unsafe memory operation | `forbid(unsafe_code)` | Implemented |
| Crypto misuse | no primitive implementation in current slice | Controlled / deferred |
| Key leakage | no key representation in current public API | Controlled |
| FFI privilege escalation | bindings prohibited from exposing policy bypass | Controlled by contract |

## Residual risks

The kernel cannot independently provide E2E security until the project accepts and implements:

- authenticated key exchange and session protocol;
- key lifecycle and secure storage;
- canonical wire serialization;
- message cryptographic processing;
- replay and freshness semantics;
- cross-platform binding verification.

These are not failures of the current deterministic kernel slice; they are explicit gated dependencies.

## Security decision

The current slice is acceptable as a **non-cryptographic Security Kernel foundation** and must not be represented as E2E or production cryptographic security.
