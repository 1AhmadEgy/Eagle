# Eagle — Security Core Contracts

**Status:** Implemented baseline  
**Date:** 2026-10-04  
**Branch:** `ai/reverse-engineering-foundation`

## Purpose

This document records the first executable orchestration contract extracted from the verified Android/JVM security primitives.

It does **not** claim cross-platform support yet.

## Current verified primitives

- Session lifecycle: `SessionStateMachine`
- Replay protection: `ReplayGuard`
- Rate policy: `TokenBucket`
- Security event schema: `SecurityEvent`
- Feature extraction: `SecurityFeatureExtractor`
- Statistical baseline: `StatisticalBaseline`

## New orchestration boundary

`DeterministicSecurityEngine` composes the first three primitives:

```
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

Session lifecycle remains an explicit state machine and is not silently coupled to authentication.

## Security boundary

`ReplayGuard` must only receive messages after cryptographic authentication and session binding. The engine therefore deliberately does not claim to authenticate peers.

`TokenBucket` receives caller-supplied monotonic time. No wall-clock time is introduced into the deterministic decision path.

## Cross-platform interpretation

The engine currently lives inside the Android/JVM application module because the repository has not yet selected a shared build technology.

Its API is intentionally platform-neutral:

- no Android imports;
- no Android Context;
- no UI dependency;
- no filesystem dependency;
- no platform clock;
- no crypto implementation.

This makes it a candidate for the first shared module after technology evaluation.

## Not yet implemented

The following remain outside this contract:

- identity;
- peer authentication;
- key management;
- cryptographic protocol;
- E2E encryption;
- transport;
- secure persistence;
- messaging;
- push/background integration.

## Verification requirement

The new orchestration test suite must pass together with the existing security tests before this contract is promoted from implemented baseline to verified runtime evidence.

Until CI/local execution evidence exists, the repository must report **tests present, execution unverified**.
