# Eagle Integrated Security Lab

> **Status:** Architecture and verification baseline
> **Date:** 2026-10-06
> **Scope:** Android + iOS + Desktop applications
> **Web:** Explicitly out of scope

## 1. Purpose

The Eagle Integrated Security Lab is the project's unified security verification architecture.

It is not a single external product. It is a controlled composition of:

- repository/security hygiene checks;
- static analysis;
- dependency and supply-chain checks;
- cryptography tests;
- protocol/state-machine tests;
- identity and trust tests;
- storage/key-custody tests;
- platform security tests;
- integration and interoperability tests;
- regression testing;
- fuzz/property testing;
- adversarial scenarios;
- evidence collection and release gating.

The laboratory must distinguish **framework presence** from **security evidence**. A tool, script, or category that exists but has not produced category-specific evidence is not a PASS.

## 2. Architecture

```text
                         Eagle Security Lab
                                │
               ┌────────────────┼────────────────┐
               │                │                │
        Shared/Core Lab   Platform Security   Supply Chain
               │                │                │
        Rust + KMP tests     Android/iOS/       SAST/DAST/
        protocol/crypto      Desktop adapters   dependency/SBOM
               │                │                │
               └────────────────┼────────────────┘
                                ▼
                         Evidence Engine
                                │
                    ┌───────────┴───────────┐
                    ▼                       ▼
              Test Lab Record          Security Gate
                    │                       │
                    └───────────┬───────────┘
                                ▼
                         Release decision
```

## 3. Laboratory domains

| Domain | Required objective | Evidence state |
|---|---|---|
| Repository hygiene | Secrets, unsafe files, policy boundaries | Baseline implemented |
| Build | Reproducible/verified application builds | Required; product depth pending |
| Unit | Deterministic component tests | Required |
| Integration | Cross-layer and cross-adapter behavior | Required |
| Cryptography | Primitive usage, key lifecycle, negative tests | Required |
| Protocol | State machines, transcript, replay/downgrade resistance | Required |
| Identity/Trust | Pairing, authorization, revocation, recovery boundaries | Required |
| Platform security | Native storage, lifecycle, permissions, signing | Required |
| Static analysis | SAST, lint, dangerous API patterns | Baseline present; depth pending |
| Dependency/supply chain | Known vulnerabilities, lockfile/provenance, policy | Required |
| Regression | Previously fixed security properties remain enforced | Required |
| Fuzz/property | Parser, serialization, protocol and state-machine robustness | Required |
| Adversarial scenarios | Abuse cases and failure-mode validation | Required |
| Evidence | Exact SHA, test output, artifact lineage | Baseline implemented |

## 4. Tooling strategy

Tools are selected by capability and evidence quality, not by brand.

Candidate/expected families include:

- OWASP MASVS / MASTG for mobile application security requirements and testing methodology;
- MobSF for mobile static/dynamic assessment where applicable;
- Frida for authorized dynamic instrumentation during security testing;
- Gitleaks or equivalent secret scanning;
- Semgrep or equivalent SAST rules;
- Trivy or equivalent dependency/container/artifact scanning;
- Rust security tooling such as cargo-audit, cargo-deny, and cargo-fuzz where applicable;
- Gradle/Android lint and platform-native signing/package verification;
- platform-native security/debugging tools for Android, iOS, Windows, macOS, and Linux.

These are laboratory components, not claims that every tool is already installed or integrated. Integration status must be recorded by executable evidence.

## 5. Platform-specific security laboratories

### 5.1 Android

Required security checks include:

- Android Keystore integration;
- hardware-backed custody capability detection where applicable;
- key purpose/alias/domain separation;
- backup/export restrictions appropriate to the key class;
- app permission minimization;
- lifecycle/background behavior;
- secure logging;
- debug/release boundary;
- APK/AAB integrity and signing verification;
- exported component review;
- local storage protection;
- screen/data exposure controls where required;
- runtime tamper/resilience checks appropriate to the threat model.

### 5.2 iOS

Required security checks include:

- Keychain integration;
- Secure Enclave usage only where the key type/operation actually supports it;
- key/access-control policy;
- application entitlements;
- code-signing verification;
- data protection classes where applicable;
- backup/data migration behavior;
- pasteboard/share-extension exposure where applicable;
- debug/release boundary;
- logging and crash-data hygiene.

### 5.3 Desktop

Required security checks include:

- Windows protected credential/cryptographic facilities;
- macOS Keychain and supported Secure Enclave operations;
- Linux protected credential-store integration where supported;
- permission and file-system boundary checks;
- executable/signing/package verification;
- update/replacement integrity;
- local IPC boundary review;
- crash/logging hygiene;
- migration/import/destruction behavior.

The desktop storage contract already requires native protected storage, truthful custody reporting, key-domain separation, and fail-closed behavior for unsupported production profiles.

## 6. Shared/core security laboratory

Platform-independent testing must cover:

- approved cryptographic primitives and configurations;
- key generation and rotation;
- revocation and epoch transitions;
- session establishment;
- forward-secrecy/PCS requirements once their controlling ADRs are finalized;
- transcript integrity;
- replay resistance;
- downgrade resistance;
- identity binding;
- authorization state machines;
- serialization canonicalization;
- parser robustness;
- error/failure behavior;
- recovery boundaries;
- deletion/destruction invariants;
- deterministic negative tests.

No custom cryptography may be introduced merely to satisfy a test case. Where a standard reviewed primitive is required, the test must verify correct use and policy enforcement around it.

## 7. Adversarial scenario classes

The lab must maintain executable or formally traceable scenarios for at least:

- stolen/lost device;
- compromised peer;
- revoked device;
- stale authorization;
- replayed pairing material;
- expired pairing material;
- transcript modification;
- downgrade attempt;
- malformed/ambiguous serialized input;
- key migration/import misuse;
- backup/restore misuse;
- unauthorized local access;
- logging/crash-report leakage;
- dependency substitution or integrity failure;
- release-package tampering.

## 8. Evidence model

Every material security test should record:

- exact commit SHA;
- target platform/runtime;
- test category;
- test/scenario identifier;
- command or controlled execution path;
- result;
- relevant artifact/log hash where practical;
- environment/toolchain identity;
- known limitations;
- reviewer/gate decision when required.

Evidence must never contain secrets, private keys, tokens, or unnecessary personal data.

## 9. Gate states

Allowed laboratory states:

- **PASS** — category-specific evidence demonstrates the required property;
- **FAIL** — evidence demonstrates a failure;
- **PENDING** — required evidence does not yet exist;
- **NOT_APPLICABLE** — formally justified for the specific target;
- **UNCONFIRMED** — evidence exists but cannot yet be established as authoritative.

A missing implementation or missing test is **not** a PASS.

## 10. Security release gate

A supported release target is blocked when any mandatory security category is:

- FAIL;
- PENDING where the category is required for that release profile;
- UNCONFIRMED where authoritative evidence is required;
- contradicted by an unresolved critical security finding.

A PASS in repository policy checks must not be interpreted as a PASS for product security.

## 11. Relationship to the existing Test Lab

Eagle already contains a Test Lab execution framework and category resolver in `scripts/ci/`, with evidence records under `.ci/testlab/`.

The Integrated Security Lab extends that model rather than replacing it.

The existing framework currently provides the verification/evidence skeleton; category-specific product security depth remains an implementation responsibility and must be advanced through executable tests and artifacts.

## 12. Governance

Security-critical changes require:

1. evidence-based diagnosis;
2. minimal implementation change;
3. relevant Test Lab execution;
4. security review;
5. documentation/evidence update;
6. release-gate evaluation.

Agents must not convert absent tests into passing status and must not weaken security controls to make CI green.
