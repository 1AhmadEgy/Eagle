# Eagle — Security Core Contracts

**Status:** Implemented contract baseline  
**Date:** 2026-10-04  
**Branch:** `ai/reverse-engineering-foundation`

## Purpose

This document records the deterministic security orchestration and the first platform-neutral security boundary contracts.

It does **not** claim cross-platform runtime support, production authentication, or production E2E cryptography.

## Current deterministic primitives

- Session lifecycle: `SessionStateMachine`
- Replay protection: `ReplayGuard`
- Rate policy: `TokenBucket`
- Security event schema: `SecurityEvent`
- Feature extraction: `SecurityFeatureExtractor`
- Statistical baseline: `StatisticalBaseline`
- Security orchestration: `DeterministicSecurityEngine`

## Deterministic message path

```text
Authenticated Message
        |
        v
   ReplayGuard
        |
   reject? ---- yes ---> REPLAY_REJECTED
        |
       no
        |
        v
   TokenBucket
        |
   allow? ---- no ----> RATE_LIMITED
        |
       yes
        |
        v
   RATE_ALLOWED
```

The engine assumes cryptographic authentication and session binding have already succeeded before replay evaluation.

Session lifecycle remains an explicit state machine and is not silently coupled to authentication.

## Platform-neutral boundary contracts

The current contract extension is:

```text
Identity Contract
      |
      v
Authentication Contract
      |
      v
SessionStateMachine
      |
      +----> CryptoBoundary
      |
      +----> KeyManagementBoundary
      |
      +----> SecureStorageBoundary
```

Implemented contract types live in:

`app/src/main/java/com/eagle/app/security/SecurityBoundaryContracts.kt`

The contracts deliberately avoid selecting:

- a cryptographic algorithm suite;
- a ratchet protocol;
- a wire format;
- a Rust/KMP implementation;
- a specific desktop/iOS storage provider.

They also avoid JVM-only annotations so that the contracts can be migrated into a future shared/common source set without changing their intended security semantics.

## Identity boundary

`IdentityContract` exposes the current device identity as public metadata only.

`DeviceIdentity` contains:

- opaque identity identifier;
- key version;
- public-key descriptor;
- lifecycle status.

Private key bytes are absent by construction.

## Authentication boundary

`AuthenticationContract` accepts a peer identity, opaque proof, and explicit authentication context.

The contract returns a deterministic decision plus a typed failure reason.

The proof bytes are opaque to the contract. Their meaning belongs to the selected protocol implementation.

## Key-management boundary

`KeyManagementBoundary` returns opaque `KeyHandle` references and public-key metadata.

No method returns private-key bytes.

Identity-key creation, rotation, revocation, and retirement remain provider responsibilities behind the boundary.

## Crypto boundary

`CryptoBoundary` is intentionally an interface only.

No cryptographic primitive, ratchet, nonce schedule, handshake pattern, or key schedule has been implemented here.

The selected protocol candidate must define the concrete cryptographic requirements before a provider is chosen.

## Secure-storage boundary

`SecureStorageBoundary` stores opaque/encrypted records.

It is not a raw private-key repository.

Private-key lifecycle remains owned by `KeyManagementBoundary`; platform adapters are responsible for protected local storage.

## Determinism and platform policy

The deterministic decision path uses caller-supplied monotonic time and contains no Android UI, Context, filesystem, or platform wall-clock dependency.

The current implementation is still compiled as Android/JVM source because no shared build technology has been selected.

Important limitation: some existing primitives still use JVM library types such as `java.math.BigInteger`. Their **semantics** are shared-core candidates, but their current **implementation source** requires dependency audit/adaptation before a common module extraction.

## Mature-component selection gate

Before implementing E2E cryptography, Eagle evaluates maintained protocol/library candidates.

Current evidence record:

`docs/08-status/IDENTITY_AUTH_CRYPTO_EVALUATION.md`

Current posture:

- 1:1 protocol: unselected; vodozemac is the leading integration candidate for a controlled proof.
- Group protocol: OpenMLS is the leading future candidate.
- libsignal: reference material only for now; no direct dependency.
- Noise: handshake framework only, not complete asynchronous E2E protocol.
- Primitive provider: unselected until protocol proof defines exact requirements.
- Native secure storage: platform-native adapters selected at the boundary level; implementations pending.

## Verification status

Tests exist for the current contracts and deterministic primitives.

Runtime verification remains **unverified** until Gradle unit tests, lint, and debug build execute successfully in an authoritative environment.

The current branch has workflow definitions for the Android Test Lab, but no workflow run was returned for the inspected branch head at the time of this update.

Therefore:

**Source + tests present ≠ verification passed.**

## Explicit non-claims

The repository must not describe the current code as:

- complete identity;
- production authentication;
- complete E2E encryption;
- production messaging;
- Rust Security Core;
- KMP Shared Layer;
- complete cross-platform runtime.

Those remain target/unverified states.
