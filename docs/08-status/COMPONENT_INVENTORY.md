# Eagle — Component Inventory

**Date:** 2026-10-04  
**Branch:** `ai/reverse-engineering-foundation`  
**Authority:** current source tree + tests; architecture documents are target evidence only

## Current inventory

| Component | Classification | Current evidence | Tests | Platforms | Decision |
|---|---|---|---|---|---|
| SessionStateMachine | Security/Core candidate | Verified Kotlin source | Verified | Android/JVM | Keep → extract shared contract |
| ReplayGuard | Security/Core candidate | Verified Kotlin source | Verified | Android/JVM | Keep → extract shared contract |
| TokenBucket | Security/Core candidate | Verified Kotlin source | Verified | Android/JVM | Keep → extract shared contract |
| SecurityEvent | Security/Core candidate | Verified Kotlin source | Verified | Android/JVM | Keep → extract shared contract |
| FeatureVector / SecurityFeatureExtractor | Security/Core candidate | Verified Kotlin source | Verified | Android/JVM | Keep → extract shared contract |
| StatisticalBaseline | Security/Analytics candidate | Verified Kotlin source | Verified | Android/JVM | Keep → audit algorithms before expansion |
| Identity | Core | Not verified | Pending | None | Implement after requirements/threat model |
| Authentication | Core | Not verified | Pending | None | Implement after identity contract |
| Crypto / E2E | Security Core | Not verified | Pending | None | Evaluate mature implementations first |
| Protocol / Serialization | Core | Not verified | Pending | None | Define contract from actual requirements |
| Networking | Abstraction | Not verified | Pending | None | Evaluate mature libraries after protocol |
| Storage abstraction | Abstraction | Not verified | Pending | None | Define after data/key requirements |
| Android secure storage | Native | Not verified | Pending | Android | Implement/evaluate |
| iOS Keychain integration | Native | Not verified | Pending | iOS | Future |
| Desktop secure storage | Native | Not verified | Pending | Desktop | Future |
| Messaging runtime | Application/Core | Not verified | Pending | None | Future |
| Mesh transport | Networking | Not verified | Pending | None | Do not assume |
| Product ML runtime | AI | Not verified | Pending | None | Deferred until real data/evaluation exists |

## Current code locations

Security source:

`app/src/main/java/com/eagle/app/security/`

Security tests:

`app/src/test/java/com/eagle/app/security/`

Verified source files include:

- `FeatureVector.kt`
- `ReplayGuard.kt`
- `SecurityEvent.kt`
- `SessionStateMachine.kt`
- `StatisticalBaseline.kt`
- `TokenBucket.kt`

Verified tests include:

- `ReplayGuardTest.kt`
- `SecurityEventTest.kt`
- `SecurityFeatureExtractorTest.kt`
- `SessionStateMachineTest.kt`
- `StatisticalBaselineTest.kt`
- `TokenBucketTest.kt`

## Verification limitations

The GitHub branch currently has no reported commit status/workflow result for the inspected head commit. A local verification attempt from this environment could not clone GitHub because external DNS/network access was unavailable.

Therefore:

**Tests present ≠ tests executed successfully in this review.**

The repository's own verification gate remains authoritative:

`bash scripts/ci/verify.sh`

## Architectural interpretation

The current security slice is the first executable foundation for the future shared core.

It must not yet be described as:

- complete E2E encryption;
- complete identity;
- production messaging;
- complete cross-platform core;
- Rust Security Core;
- KMP Shared Layer.

Those remain unverified/target states.
