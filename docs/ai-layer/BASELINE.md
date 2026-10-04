# Eagle AI Layer — Repository Baseline

Status: CONFIRMED
Branch: ai/reverse-engineering-foundation
Baseline note: superseded by subsequent implementation commits; see the verification log for current evidence.
Commit time: 2026-10-04T16:10:30Z

## Build structure

| Item | Evidence |
|---|---|
| Build DSL | Gradle Kotlin DSL |
| Root module | :app only |
| Android namespace | com.eagle.app |
| compileSdk | 37 |
| minSdk | 29 |
| targetSdk | 37 |
| Unit-test framework | JUnit 4.13.2 |
| Instrumentation tests | No app/src/androidTest tree on audited commit |
| Main language | Kotlin |
| Separate :core/:security module | Not present |

## Security implementation surface

Production sources:
- app/src/main/java/com/eagle/app/security/ReplayGuard.kt
- app/src/main/java/com/eagle/app/security/TokenBucket.kt
- app/src/main/java/com/eagle/app/security/SecurityEvent.kt
- app/src/main/java/com/eagle/app/security/SessionStateMachine.kt
- app/src/main/java/com/eagle/app/security/DeterministicSecurityEngine.kt
- app/src/main/java/com/eagle/app/security/FeatureVector.kt
- app/src/main/java/com/eagle/app/security/StatisticalBaseline.kt
- app/src/main/java/com/eagle/app/security/SecurityBoundaryContracts.kt

AI contracts:
- app/src/main/java/com/eagle/app/ai/provider/AIProvider.kt
- app/src/main/java/com/eagle/app/ai/provider/ProviderPolicy.kt
- app/src/main/java/com/eagle/app/ai/provider/ProviderCapabilities.kt
- app/src/main/java/com/eagle/app/ai/findings/SecurityFinding.kt
- app/src/main/java/com/eagle/app/ai/evidence/EvidenceRecord.kt

Tests:
- app/src/test/java/com/eagle/app/security/ReplayGuardTest.kt
- app/src/test/java/com/eagle/app/security/TokenBucketTest.kt
- app/src/test/java/com/eagle/app/security/SecurityEventTest.kt
- app/src/test/java/com/eagle/app/security/SessionStateMachineTest.kt
- app/src/test/java/com/eagle/app/security/DeterministicSecurityEngineTest.kt
- app/src/test/java/com/eagle/app/security/SecurityFeatureExtractorTest.kt
- app/src/test/java/com/eagle/app/SmokeTest.kt

## Existing AI/development-agent surface

The repository already contains eight OpenCode agent definitions under .opencode/agents/.

They are Markdown configuration/instruction artifacts, not Kotlin model-runtime implementations.

## Important verification gap

The repository's mandatory scripts/ci/verify.sh supports npm, Python, Go, and Rust detection, but not Gradle/Android.

The independent Test Lab workflow does execute:
- Gradle unit tests;
- Android lint;
- debug build.

Therefore a green scripts/ci/verify.sh run alone must not be interpreted as Android build evidence.

## Current product-AI status

No evidence was found on this branch of a product-integrated LLM/model runtime or concrete remote/local model provider adapter. The implemented AI surface is provider/evidence contracts and policy constraints.

The existing deterministic security layer is the authority surface. The new AI layer begins as contracts/evidence infrastructure and remains outside enforcement until later phases establish explicit controls.

## Provenance note

The branch tip commit is currently unsigned according to GitHub commit metadata. This does not prevent M1 contract work, but it reinforces the need for the future EvidenceSigner to use an explicitly managed key outside repository source.
