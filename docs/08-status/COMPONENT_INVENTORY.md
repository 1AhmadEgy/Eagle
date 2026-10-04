# Eagle — Component Inventory

**Date:** 2026-10-04  
**Branch:** ai/reverse-engineering-foundation  
**Authority:** current source tree + repository evidence; architecture documents are target evidence only

## Current inventory

| Component | Classification | Current evidence | Tests | Platforms | Decision |
|---|---|---|---|---|---|
| SessionStateMachine | Security/Core candidate | Implemented Kotlin source | Present; execution unverified | Android/JVM | Keep → extract shared contract |
| ReplayGuard | Security/Core candidate | Implemented Kotlin source | Present; execution unverified | Android/JVM | Keep → extract shared contract |
| TokenBucket | Security/Core candidate | Implemented Kotlin source | Present; execution unverified | Android/JVM | Keep → extract shared contract |
| SecurityEvent | Security/Core candidate | Implemented Kotlin source | Present; execution unverified | Android/JVM | Keep → extract shared contract |
| FeatureVector / SecurityFeatureExtractor | Security/Core candidate | Implemented Kotlin source | Present; execution unverified | Android/JVM | Keep → extract shared contract |
| StatisticalBaseline | Security/Analytics candidate | Implemented Kotlin source | Present; execution unverified | Android/JVM | Keep → audit algorithms before expansion |
| DeterministicSecurityEngine | Security/Core orchestration | Implemented Kotlin source | Present; execution unverified | Android/JVM; platform-neutral API | Keep → extract into first shared module |
| Identity / Authentication contracts | Core boundary | Implemented Kotlin contracts | Present; execution unverified | Android/JVM; platform-neutral API | Keep → protocol adapter pending |
| Crypto boundary | Security/Core boundary | Implemented interface only | Contract tests present; execution unverified | Android/JVM; platform-neutral API | Keep → bind after protocol decision |
| Key Management boundary | Security/Core boundary | Implemented interface only | Contract tests present; execution unverified | Android/JVM; platform-neutral API | Keep → native adapter pending |
| Secure Storage boundary | Platform/Core abstraction | Implemented interface only | Contract tests present; execution unverified | Android/JVM; platform-neutral API | Keep → native adapters pending |
| Identity implementation | Core | Not verified | Pending | None | Implement after threat model + protocol selection |
| Authentication implementation | Core | Not verified | Pending | None | Implement after protocol selection |
| Crypto / E2E implementation | Security Core | Not verified | Pending | None | Integrate mature candidate first |
| Protocol / Serialization | Core | Not verified | Pending | None | Define after protocol proof |
| Networking | Abstraction | Not verified | Pending | None | Evaluate after protocol/framing contracts |
| Android secure storage adapter | Native | Not verified | Pending | Android | Adopt Android Keystore boundary; implementation pending |
| iOS Keychain integration | Native | Not verified | Pending | iOS | Adopt Apple Keychain boundary; future implementation |
| Desktop secure storage | Native | Not verified | Pending | Windows/macOS/Linux | Use OS-native adapters; exact implementation pending |
| Messaging runtime | Application/Core | Not verified | Pending | None | Future |
| Mesh transport | Networking | Not verified | Pending | None | Do not assume |
| Product ML runtime | AI | Not verified | Pending | None | Deferred until real data/evaluation exists |

## Current code locations

Security source:
app/src/main/java/com/eagle/app/security/

Security tests:
app/src/test/java/com/eagle/app/security/

Current contract file:
app/src/main/java/com/eagle/app/security/SecurityBoundaryContracts.kt

## Verification limitations

The inspected branch head is f8ce28911597f8460f6bf0ecd02d5bf191386871.

No successful GitHub Actions status was established for that head in this review, and the environment previously lacked external network/DNS access for a local Gradle verification run.

Therefore:

**Source inspected + tests present ≠ runtime verification passed.**

The repository verification gate remains authoritative:
bash scripts/ci/verify.sh

## Architectural rule

Do not describe the current implementation as complete E2E encryption, complete identity, production authentication, production messaging, complete cross-platform core, Rust Security Core, or KMP Shared Layer.

Those remain target/unverified states.

## Latest increment — 2026-10-04

Added platform-neutral Identity, Authentication, Crypto, Key Management, and Secure Storage boundaries plus contract tests. Added evidence-based third-party evaluation before selecting a concrete E2E protocol.
