# Eagle AI Layer Specification

Status: M1 CONTRACT BASELINE
Branch: ai/reverse-engineering-foundation
Revision baseline: f8ce28911597f8460f6bf0ecd02d5bf191386871

## 1. Purpose

Build a model-agnostic AI security/engineering layer for Eagle in which models are replaceable providers, AI output is evidence rather than authority, and security enforcement remains deterministic.

The layer must:
- remain provider/model agnostic;
- separate proposal from verification;
- preserve reproducible evidence;
- require explicit human approval for cryptography/E2EE/identity changes;
- reuse the existing OpenCode agent surface rather than duplicating it;
- never place an AI model inside the identity/authentication/authorization authority path.

## 2. Current repository reality

The audited branch is a single Android application module:
- Gradle Kotlin DSL: build.gradle.kts and app/build.gradle.kts
- Android namespace: com.eagle.app
- compileSdk: 37
- minSdk: 29
- targetSdk: 37
- Kotlin source is under app/src/main/java
- unit tests are under app/src/test/java
- current test dependency is JUnit 4.13.2
- no app/src/androidTest tree is present on this branch
- no separate :core or :security Gradle module exists
- security primitives live under com.eagle.app.security
- the concrete session class is SessionStateMachine, not SessionState

Existing security primitives verified on the branch:
- ReplayGuard
- TokenBucket
- SecurityEvent
- SessionStateMachine
- DeterministicSecurityEngine
- FeatureVector / SecurityFeatureExtractor

The repository already contains development-time OpenCode agents under .opencode/agents/. They remain a separate development orchestration surface.

## 3. Architectural boundaries

### 3.1 Proposer versus verifier

A proposer may inspect broader source context and produce:
- finding;
- recommended direction;
- reproduction-test proposal;
- optional patch proposal.

A verifier receives only the claim, declared evidence, and verification target required to evaluate it. The verifier must not receive the proposer's hidden reasoning or private chain-of-thought.

Eagle evidence storage must never persist model chain-of-thought. Only final model output needed for the evidence record may be retained, subject to redaction and policy.

### 3.2 Deterministic authority

AI must not:
- authenticate an identity;
- authorize a peer;
- release or derive private keys;
- select cryptographic primitives at runtime;
- override protocol state validation;
- disable replay protection;
- disable rate limiting;
- approve a cryptographic patch for merge.

### 3.3 Crypto gate

For CRYPTO_REVIEW and any change touching E2EE, identity, key handling, nonce/freshness enforcement, or cryptographic protocol code:
- AI may produce finding, direction, and test proposal;
- AI-generated cryptographic patches are never auto-mergeable;
- human cryptographic review is mandatory;
- regression and independent verification are mandatory.

## 4. Provider abstraction

Routing is based on ProviderCapabilities + ProviderPolicy rather than hard-coded provider names.

Required capabilities:
- toolUse
- longContextTokens
- onPremise
- telemetryFree
- codeOnly
- reasoningDepth
- supportsCryptoReview

The provider ID is metadata, not a routing rule.

## 5. Existing agents and their role

The current OpenCode agents are not Kotlin runtime components and must not be conflated with the future Kotlin AI layer.

- orchestrator.md: development-time coordinator; no direct code edits
- security-auditor.md: read-only security proposer
- code-reviewer.md: read-only correctness/regression proposer
- test-engineer.md: read-only verification/category evaluator
- debugger.md: read-only root-cause verifier
- gatekeeper.md: evidence-only decision gate
- repair-agent.md: restricted repair worker
- documentation-agent.md: strict read-only provenance/documentation worker

The Kotlin AiOrchestrator may consume or integrate with these development-time roles later, but it does not replace their Markdown definitions.

## 6. Evidence model

Every evidence record must bind:
- target/task identity;
- analyzed source snapshot hash;
- provider identity/version;
- prompt hash (not the prompt itself);
- final model output;
- parsed finding, if any;
- reproduction-test identity/result;
- SAST identity/result;
- regression results;
- build identity/result;
- human approval, if required;
- previous record hash;
- current record hash;
- signature, once the signing implementation is available.

Evidence must be append-only at the logical layer.

The exact signing-key storage mechanism is deliberately deferred to M2. The private signing key must never be stored in the repository.

## 7. Verification pipeline

Canonical pipeline:

AI Finding
  -> Reproduction Test
  -> Static Analysis / SAST
  -> Patch Proposal
  -> Human Crypto Review when required
  -> Regression Test
  -> Android Build + Lint
  -> Independent Verification
  -> Gatekeeper

Important repository-specific verification commands are currently:
- gradle :app:testDebugUnitTest --no-daemon --console=plain
- gradle :app:lint --no-daemon --console=plain
- gradle :app:assembleDebug --no-daemon --console=plain

The current scripts/ci/verify.sh does not detect a Gradle/Android application. Therefore it must not be treated as proof of an Android build until that gap is closed.

## 8. Human approval policy

- CRYPTO_REVIEW: REQUIRED_FOR_CRYPTO
- security findings with HIGH or CRITICAL severity: REQUIRED_FOR_MERGE
- architecture changes that alter trust boundaries: REQUIRED_FOR_ARCHITECTURE
- documentation and non-sensitive analysis: NONE unless a downstream policy says otherwise
- no patch is mergeable solely because an AI provider reports success

## 9. Package layout for M1+

The initial contracts are placed under:

app/src/main/java/com/eagle/app/ai/
  provider/
  findings/
  evidence/

Future orchestration/policy/storage implementations may extend this namespace after contract validation.

## 10. Lifecycle

M0 - repository baseline: COMPLETE
M1 - contracts + unit tests: IN PROGRESS / first implementation committed with this specification
M2 - hash-chain + signing: NOT IMPLEMENTED
M3 - orchestration + routing: NOT IMPLEMENTED
M4 - concrete provider adapters: NOT IMPLEMENTED
M5 - OpenCode integration: NOT IMPLEMENTED
M6 - policy enforcement: NOT IMPLEMENTED
M7 - final architecture/documentation review: NOT IMPLEMENTED

No future phase may be described as implemented until its executable evidence exists.
