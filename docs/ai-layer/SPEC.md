# Eagle AI Layer Specification

Status: M2 IMPLEMENTATION BASELINE
Branch: ai/reverse-engineering-foundation
Revision baseline: 6fa710578558450ade86e3ed1c7d0fc6bf8060d7

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
- no app/src/androidTest tree is present at the M2 baseline
- no separate :core or :security Gradle module exists
- security primitives live under com.eagle.app.security
- the concrete session class is SessionStateMachine

Verified security primitives:
- ReplayGuard
- TokenBucket
- SecurityEvent
- SessionStateMachine
- DeterministicSecurityEngine
- FeatureVector / SecurityFeatureExtractor

## 3. Architectural boundaries

### 3.1 Proposer versus verifier

A proposer may inspect broader source context and produce:
- finding;
- recommended direction;
- reproduction-test proposal;
- optional patch proposal.

A verifier receives only the claim, declared evidence, and verification target required to evaluate it. The verifier must not receive the proposer's hidden reasoning or private chain-of-thought.

Eagle evidence storage must never persist model chain-of-thought. Only final model output required by the evidence policy may be retained.

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

Provider ID is metadata, not a routing rule.

## 5. Existing agents

The OpenCode agents under .opencode/agents/ are development-time instruction/configuration artifacts, not Kotlin model-runtime components.

- orchestrator.md: coordinator, no direct edits
- security-auditor.md: read-only security proposer
- code-reviewer.md: read-only correctness/regression proposer
- test-engineer.md: read-only verification/category evaluator
- debugger.md: read-only root-cause verifier
- gatekeeper.md: evidence-only gate
- repair-agent.md: restricted repair worker
- documentation-agent.md: strict read-only provenance/documentation worker

The Kotlin AI layer does not replace these definitions.

## 6. Evidence contract

Every EvidenceRecord binds:
- target/task identity;
- analyzed source snapshot hash;
- provider identity/version;
- prompt hash;
- final provider output subject to redaction policy;
- parsed finding if present;
- reproduction-test identity;
- SAST identity;
- regression results;
- build identity;
- human approval if required;
- previous record hash;
- current record hash;
- signature if produced.

recordHash is a derived value and is not included in its own hash preimage. signature is also excluded from the preimage and signs the canonical recordHash.

Evidence is logically append-only. Persistence technology is intentionally decoupled from the contract.

## 7. M2 canonical hash

EvidenceHasher uses SHA-256 over a deterministic binary representation with:
- domain/version marker;
- fixed field order;
- type-aware enum encoding;
- length-delimited UTF-8 strings;
- explicit null/presence markers;
- deterministic ordering for set-valued fields;
- explicit list cardinality and order;
- fixed-width Instant fields.

This framing prevents ambiguous concatenation/canonicalization. It is not claimed as a general remedy for SHA-256 length-extension attacks.

## 8. M2 chain verification

EvidenceChain.verifyChain:
1. requires genesis previousHash == null;
2. requires each later previousHash to equal the prior recordHash;
3. recomputes every record hash;
4. stops at the first mismatch;
5. returns isValid, firstBrokenIndex, and reason.

## 9. M2 signing

EvidenceSigner is intentionally independent of key storage.

Current executable signer exists only in JVM tests:
- InMemoryTestSigner
- Ed25519
- test-only fixed key material

Production adapters remain deferred:
- Android Keystore for on-device operation;
- CI-managed key source for controlled tooling.

No production private key belongs in repository source.

## 10. Storage decision

M2 intentionally does not choose JSON/JSONL or Room.

The chain is a logical verification primitive. A later storage adapter must preserve:
- order;
- recordHash;
- previousHash;
- signature;
- verification metadata;
- append-only semantics.

A persistence choice requires its own ADR after the evidence schema and operating boundary are known.

## 11. Verification boundary

The repository's scripts/ci/verify.sh does not currently detect Gradle/Android and therefore is not sufficient Android-build evidence.

The Test Lab workflow invokes:
- gradle :app:testDebugUnitTest --no-daemon --console=plain
- gradle :app:lint --no-daemon --console=plain
- gradle :app:assembleDebug --no-daemon --console=plain

The current CI failure occurs before these commands, while trying to install the unavailable Android SDK package platforms;android-37 on the GitHub runner.

## 12. Lifecycle

M0 - repository baseline: COMPLETE
M1 - provider/finding/evidence contracts: COMPLETE
M2 - canonical hash, chain verification, signer interface and JVM signer: IMPLEMENTED
M3 - orchestration + routing: NOT IMPLEMENTED
M4 - concrete provider adapters: NOT IMPLEMENTED
M5 - OpenCode integration: NOT IMPLEMENTED
M6 - policy enforcement: PARTIAL CONTRACTS / IMPLEMENTATION PENDING
M7 - final architecture/documentation review: NOT IMPLEMENTED

No future phase may be described as implemented until executable evidence exists.
