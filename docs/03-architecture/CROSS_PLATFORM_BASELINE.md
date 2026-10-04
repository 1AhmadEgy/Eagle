# Eagle — Cross-Platform Architecture Baseline

**Status:** Accepted baseline  
**Date:** 2026-10-04  
**ADR:** ADR-0015

## Target

Eagle is a multi-platform platform:

- Android
- iOS
- Windows
- macOS
- Linux

The architecture is defined by boundaries, not by a preselected framework.

```
                         EAGLE PLATFORM
                              |
             +----------------+----------------+
             |            EAGLE CORE            |
             |                                  |
             | Identity / Session Contracts     |
             | Security Events                  |
             | Replay / Rate Policies           |
             | Crypto Contracts                 |
             | Protocol / Serialization         |
             | Networking Contracts             |
             | Storage Contracts                |
             | Audit / Telemetry                |
             +----------------+-----------------+
                              |
                   Platform Abstraction
                              |
          +-----------+-------+-------+-----------+
          |           |               |           |
       Android       iOS        Windows/macOS    Linux
          |           |               |           |
       Native       Native          Native       Native
       APIs         APIs            APIs         APIs
          |           |               |           |
          +-----------+-------+-------+-----------+
                              |
                       Application Layer
                         UI / State / Features
```

## Evidence rule

The diagram describes the target boundary.

It does not prove that any layer exists.

For every component, the repository must distinguish:

- **Verified** — current code + relevant test/build evidence;
- **Implemented / unverified** — code exists but verification is incomplete;
- **Target** — architecture intended for future implementation;
- **Proposed** — candidate technology or design awaiting decision;
- **Historical** — no longer authoritative.

## Current verified security slice

The current Android/JVM source contains:

`SessionStateMachine → ReplayGuard → TokenBucket → SecurityEvent → FeatureVector → StatisticalBaseline`

This is a deterministic security foundation. It is not yet a complete identity, cryptographic, transport, or messaging stack.

## Cross-platform extraction strategy

The first extraction target is **contracts and deterministic semantics**, not UI.

### Shared candidates

- session state semantics;
- replay decisions;
- token-bucket policy semantics;
- security-event schema;
- feature-vector calculations;
- deterministic statistical calculations;
- protocol/domain value objects after dependency audit.

### Native candidates

- secure key storage;
- OS lifecycle/background services;
- push notifications;
- permissions;
- filesystem integration;
- packaging/signing;
- OS networking hooks where required.

## Platform implementation policy

### Android

Current verified runtime. Continue using it as the first executable validation environment.

### iOS

Future target. Do not create a second security implementation merely to reach iOS. Reuse verified contracts and add native integration only where required.

### Windows/macOS/Linux

Future desktop targets. Prefer one shared application/domain contract with OS adapters rather than separate product logic per desktop OS.

## Technology selection

KMP, Compose Multiplatform, Rust, UniFFI, native Kotlin/Swift, or another approach may be selected later.

Selection must be based on:

- actual repository needs;
- security boundary;
- testability;
- build reproducibility;
- platform coverage;
- dependency maturity;
- maintenance;
- licensing;
- performance;
- operational cost.

## Validation invariant

`Same Input → Same Core Semantics → Same Security Decision`

Platform-specific differences must be explicit and tested.

## Next execution slice

1. Keep the existing security foundation green.
2. Build the component inventory.
3. Separate platform-neutral contracts from Android implementation details.
4. Add cross-platform test vectors where practical.
5. Evaluate candidate shared technologies.
6. Extract the first shared module only after the evidence review.
7. Add one second runtime and compare deterministic outputs.
