# Eagle — Component Inventory

**Date:** 2026-10-04  
**Branch:** ai/reverse-engineering-foundation  
**Authority:** current source tree + repository evidence; architecture documents are target evidence only

## Current inventory

| Component | Classification | Current evidence | Tests | Platforms | Decision |
|---|---|---|---|---|---|
| SessionStateMachine | Security/Core candidate | Implemented Kotlin source | Present; execution unverified | Android/JVM | Keep → extract shared contract |
| ReplayGuard | Security/Core candidate | Implemented Kotlin source | Present; execution unverified | Android/JVM | Keep → extract shared contract |
| TokenBucket | Security/Core candidate | Implemented Kotlin source | Present; execution unverified | Android/JVM | Keep → audit JVM dependencies → extract shared contract |
| SecurityEvent | Security/Core candidate | Implemented Kotlin source | Present; execution unverified | Android/JVM | Keep → extract shared contract |
| FeatureVector / SecurityFeatureExtractor | Security/Core candidate | Implemented Kotlin source | Present; execution unverified | Android/JVM | Keep → audit JVM dependencies → extract shared contract |
| StatisticalBaseline | Security/Analytics candidate | Implemented Kotlin source | Present; execution unverified | Android/JVM | Keep → benchmark → extract/adapt JVM math dependency |
| DeterministicSecurityEngine | Security/Core orchestration | Implemented Kotlin source | Present; execution unverified | Android/JVM; platform-neutral API | Keep → integrate SecurityEvent emission and shared-core extraction |
| Identity / Authentication contracts | Core boundary | Implemented Kotlin contracts | Present; execution unverified | Android/JVM; shared-compatible API | Keep → protocol adapter pending |
| Crypto boundary | Security/Core boundary | Implemented interface only | Contract tests present; execution unverified | Android/JVM; shared-compatible API | Keep → bind after protocol decision |
| Key Management boundary | Security/Core boundary | Implemented interface only | Contract tests present; execution unverified | Android/JVM; shared-compatible API | Keep → native adapter pending |
| Secure Storage boundary | Platform/Core abstraction | Implemented interface only | Contract tests present; execution unverified | Android/JVM; shared-compatible API | Keep → native adapters pending |
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
| Product ML/LLM runtime | AI | Not verified | Pending | None | Deferred until real data/evaluation exists |

## Current code locations

Security source:
app/src/main/java/com/eagle/app/security/

Security tests:
app/src/test/java/com/eagle/app/security/

Current security contract file:
app/src/main/java/com/eagle/app/security/SecurityBoundaryContracts.kt

Current security contract test:
app/src/test/java/com/eagle/app/security/SecurityBoundaryContractsTest.kt

## Shared-core readiness notes

The new Identity/Authentication/Crypto/Key/Storage contracts avoid JVM-only annotations.

However, the existing deterministic primitives are not yet directly movable to a Kotlin common source set because:

- TokenBucket uses java.math.BigInteger;
- FeatureVector uses java.math.BigInteger;
- StatisticalBaseline uses java.math.BigInteger;
- StatisticalBaseline also imports kotlin.math APIs that require a common-source compatibility review.

The semantics remain deterministic and suitable for shared test vectors; implementation portability must be proven separately.

## Verification limitations

The inspected branch head after this increment is tracked by repository history; GitHub workflow execution evidence was not available for the latest contract changes at the time of review.

The branch contains an Android Test Lab workflow that runs:

- app unit tests;
- Android lint;
- debug build.

The repository verification script scripts/ci/verify.sh itself does not detect Gradle/Android.

Therefore:

**Source inspected + tests present ≠ runtime verification passed.**

## Architectural rule

Do not describe the current implementation as:

- complete E2E encryption;
- complete identity;
- production authentication;
- production messaging;
- complete cross-platform core;
- Rust Security Core;
- KMP Shared Layer.

Those remain target/unverified states.

## AI contract surface

Current AI contracts are implemented under `app/src/main/java/com/eagle/app/ai/`:
- `AIProvider`
- `ProviderCapabilities`
- `ProviderPolicy`
- `TaskType`
- `ReasoningDepth`
- `SecurityFinding`
- `EvidenceRecord`

They are contract/evidence infrastructure only. No concrete LLM/model runtime, adapter, signer, or persistence backend is verified.

## Latest increment — 2026-10-04

Added platform-neutral Identity, Authentication, Crypto, Key Management, and Secure Storage boundaries and contract tests.

Recorded mature-component evaluation before selecting a concrete E2E protocol.

Corrected the new contract types to avoid JVM-only inline-class annotations.

Recorded the remaining JVM-specific dependencies in the existing deterministic primitives as an extraction/adaptation task rather than silently claiming they are already common code.
