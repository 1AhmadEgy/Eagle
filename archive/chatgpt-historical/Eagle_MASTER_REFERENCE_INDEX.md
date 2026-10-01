# 🦅 Eagle / PrivateMesh — Master Reference Index

**Document:** Master Reference & Knowledge Index  
**Project:** Eagle / PrivateMesh  
**Purpose:** المرجع المركزي للمصادر والقرارات والمعرفة المستخدمة في المشروع  
**Status:** Baseline / Living Document  
**Rule:** لا يتحول أي مصدر إلى قرار معماري أو أمني إلا بعد مراجعته واعتماده في وثيقة قرار/مواصفة.

---

## 0. How to use this document

هذا الملف هو **فهرس مرجعي** وليس مواصفة البروتوكول نفسها.

ترتيب المعرفة:

```text
Source
  ↓
Evidence / Finding
  ↓
Review
  ↓
Decision
  ↓
Specification
  ↓
Implementation
  ↓
Test
  ↓
Verification
```

### قواعد المرجع

1. المصادر الرسمية والمعايير هي المرجع الأول عند تعارض المصادر.
2. الأبحاث والدراسات تستخدم لفهم المخاطر والمنهجيات والبدائل.
3. المكتبات والمشاريع الجاهزة تستخدم بعد فحص النضج والأمان والترخيص والصيانة.
4. لا نعيد اختراع التشفير.
5. لا نعتمد مكتبة أو أداة لمجرد شهرتها.
6. لا يصبح الذكاء الاصطناعي مرجعًا أمنيًا أو صاحب قرار أمني نهائي.
7. كل قرار معماري مهم يجب أن يكون له سجل قرار (ADR).
8. كل اعتماد مهم يجب أن يكون له اختبار أو دليل تحقق.
9. كل مصدر له تخصص واضح.
10. المصادر القديمة لا تُعامل تلقائيًا كمصادر حالية؛ يجب التحقق من الحالة الحالية للمشروع/المعيار.

---

# 1. Project Foundation

## 1.1 Project identity

- **Project name:** Eagle / النسر
- **Technical name:** PrivateMesh
- **Scope:** منصة/بروتوكول اتصالات خاصة وآمنة متعددة المنصات.
- **Primary implementation direction:** Rust core with platform-specific clients/integration.
- **Security philosophy:** security-first, privacy-first, fail-closed, no custom cryptography.
- **Initial model:** one trusted device per account.
- **Server role:** identity/login/discovery/rendezvous and minimal supporting services; no central message history.
- **Sensitive state:** primarily local to the trusted client/device.
- **Default privacy:** automatic deletion according to explicit protocol/storage semantics.
- **AI role:** intelligence/analysis/recommendation only; deterministic security policy remains authoritative.

> هذه البنود تمثل قرارات ومبادئ ظهرت في نقاشات المشروع، وليست ادعاءات مستخرجة من المصادر الخارجية.

---

# 2. Architecture Decision References

## 2.1 Security Kernel

### Reference role
طبقة صغيرة، deterministic، versioned، auditable، fail-closed.

### Must not depend on
- LLM judgment
- opaque AI classification
- uncontrolled plugins
- arbitrary runtime policy changes

### Security decisions

```text
Input
  ↓
Validation
  ↓
Policy
  ↓
Decision
  ├── ALLOW
  ├── DENY
  ├── REQUIRE_APPROVAL
  └── QUARANTINE
```

---

# 3. Cryptography

## 3.1 Primary protocol family

### Signal Protocol specifications

Official Signal documentation:

- PQXDH
- Double Ratchet
- XEdDSA / VXEdDSA
- X3DH (historical/reference)
- ML-KEM Braid (future/reference)

Signal states that PQXDH is designed for asynchronous session establishment and that the server can hold public prekey material while the initiating party obtains a prekey bundle.  
Source: Signal PQXDH specification.

Signal's Double Ratchet specification describes deriving new message keys and mixing new Diffie-Hellman values into the ratchet state.

### Project use

```text
V1
  └── PQXDH + Double Ratchet

V2+
  └── further post-quantum evolution / migration as validated
```

### Official references

- https://signal.org/docs/
- https://signal.org/docs/specifications/pqxdh/
- https://signal.org/docs/specifications/doubleratchet/

---

## 3.2 Post-Quantum Cryptography

### NIST

Primary standards:

- FIPS 203 — ML-KEM
- FIPS 204 — ML-DSA
- FIPS 205 — SLH-DSA

NIST finalized the three primary PQC standards in August 2024.

### Project rule

Do not select a PQ algorithm solely because it is new or fashionable.

Selection must consider:

- standardized status
- implementation maturity
- interoperability
- side-channel considerations
- performance
- key/ciphertext/signature sizes
- migration/crypto-agility
- implementation availability
- independent review

### References

- https://www.nist.gov/pqc
- https://csrc.nist.gov/projects/post-quantum-cryptography
- https://csrc.nist.gov/pubs/fips/203/final
- https://csrc.nist.gov/pubs/fips/204/final
- https://csrc.nist.gov/pubs/fips/205/final
- https://csrc.nist.gov/pubs/sp/800-227/final

---

# 4. Group Messaging

## 4.1 MLS

### Reference

- IETF RFC 9420 — The Messaging Layer Security (MLS) Protocol
- OpenMLS — implementation/reference ecosystem

### Project position

MLS is a V2/group-messaging concern unless a later decision explicitly promotes it.

### References

- https://www.rfc-editor.org/rfc/rfc9420
- https://openmls.tech/

---

# 5. Network / Transport

## 5.1 QUIC

Reference:

- RFC 9000 — QUIC

Project use:

```text
P2P transport
  ↓
QUIC
```

QUIC itself is the transport layer; it does not replace the application-level end-to-end cryptographic protocol.

Reference:
- https://www.rfc-editor.org/rfc/rfc9000

---

## 5.2 ICE

Reference:

- RFC 8445 — Interactive Connectivity Establishment (ICE)

Project role:

```text
ICE = connectivity / NAT traversal
QUIC = transport
```

Reference:
- https://www.rfc-editor.org/rfc/rfc8445

---

## 5.3 TURN

Reference:

- RFC 8656 — Traversal Using Relays around NAT (current TURN specification)
- RFC 5766 — historical/obsolete TURN specification

Project model:

```text
Direct P2P
    ↓
if unavailable
    ↓
TURN relay
```

TURN is a connectivity fallback, not a permission to store plaintext application messages.

References:

- https://www.rfc-editor.org/rfc/rfc8656
- https://www.rfc-editor.org/info/rfc5766

---

## 5.4 TLS

TLS 1.3 remains relevant for client/server control-plane communications where applicable.

Reference:

- RFC 8446 — The Transport Layer Security (TLS) Protocol Version 1.3
- https://www.rfc-editor.org/rfc/rfc8446

---

# 6. Platform Security

## 6.1 Android

### Reference areas

- Android Keystore
- hardware-backed keys where available
- StrongBox where available
- Key Attestation
- platform integrity controls

### Project rule

Do not describe every Android key as automatically hardware-bound.

Separate:

```text
Protocol Identity
Platform Attestation
Local Key Protection
```

References:

- https://developer.android.com/privacy-and-security/keystore
- https://developer.android.com/privacy-and-security/security-key-attestation

---

## 6.2 Apple

### Reference areas

- Secure Enclave
- Keychain
- App Attest
- DeviceCheck

Apple documents App Attest as a way for a server to verify that requests originate from legitimate instances of an app. The App Attest key is stored in the Secure Enclave and the service uses attestation/assertions.

References:

- https://developer.apple.com/documentation/devicecheck
- https://developer.apple.com/documentation/devicecheck/establishing-your-app-s-integrity
- https://developer.apple.com/documentation/devicecheck/validating-apps-that-connect-to-your-server

### Important limitation

App Attest is not a universal guarantee that a device/OS is uncompromised. Apple explicitly describes it as part of a broader risk/security assessment.

---

# 7. Local Storage

## 7.1 SQLite

SQLite is a candidate local database engine.

Reference areas:

- transactions
- WAL
- integrity
- crash recovery
- testing
- storage behavior

Official documentation:

- https://www.sqlite.org/docs.html
- https://www.sqlite.org/testing.html
- https://www.sqlite.org/wal.html

### Project rule

Database encryption, key protection, deletion semantics, WAL/journal handling, backups, temporary files and OS-specific storage must be tested separately.

---

## 7.2 Database encryption

SQLCipher may be evaluated as an implementation option for encrypted SQLite storage.

Reference:

- https://www.zetetic.net/sqlcipher/

Decision status:

**Candidate — not automatically adopted.**

---

# 8. Rust

## 8.1 Core implementation

Rust is the preferred core implementation language from the project architecture.

### Security relevance

Rust's safe subset provides memory-safety guarantees, while `unsafe` operations require explicit handling and review.

Reference:

- https://www.rust-lang.org/
- https://doc.rust-lang.org/reference/unsafety.html

### Project rule

All `unsafe` code:

- must be minimized
- must have a documented justification
- must be reviewed
- must have targeted tests
- must not be introduced merely for convenience

---

# 9. Observability / Telemetry

## 9.1 OpenTelemetry

OpenTelemetry provides standardized concepts for traces, metrics, logs and semantic conventions.

Reference:

- https://opentelemetry.io/docs/specs/otel/
- https://opentelemetry.io/docs/specs/semconv/
- https://opentelemetry.io/docs/specs/otel/schemas/

### Project use

OpenTelemetry is a **reference/integration candidate**, not permission to export sensitive PrivateMesh content.

### Never export

- plaintext messages
- private keys
- recovery codes
- session secrets
- sensitive authentication material
- unnecessary PII

### Preferred event model

```text
event_id
timestamp
component
event_type
correlation_id
session_id
policy_version
decision
reason_code
severity
artifact_reference
```

Sensitive payloads should be referenced by safe identifiers/hashes rather than copied into telemetry.

---

# 10. Mobile Security Testing

## 10.1 OWASP MASVS

Use OWASP MASVS as a verification baseline for mobile clients.

Control areas include:

- storage
- cryptography
- authentication/authorization
- network
- platform
- code
- resilience
- privacy

Reference:

- https://mas.owasp.org/MASVS/
- https://mas.owasp.org/MASTG/
- https://mas.owasp.org/MASWE/

---

# 11. Test Laboratory

## 11.1 Research-derived references from uploaded material

### A Decade of GHOSTS in the Machine

Use as engineering/process reference for:

- observability
- traceability
- instrumentation
- maintainability
- documentation
- CI/release discipline
- dependency management
- project evolution
- complexity control

Classification:

```text
RESEARCH / ENGINEERING REFERENCE
```

Do not copy its architecture into PrivateMesh automatically.

---

## 11.2 DeepSeek / cyber-range research material

Use as a methodology/reference source for:

- scenario generation
- event-driven telemetry
- fault injection
- detection
- classification
- RCA
- remediation
- verification
- regression curation
- governance
- evidence chains
- composite scenarios

Classification:

```text
RESEARCH / TEST-LAB METHODOLOGY
```

Again: proposed tools/components in the research are not automatically adopted.

---

# 12. Test-Lab Architecture

```text
Governance
    ↓
Orchestrator
    ↓
Scenario Selection
    ↓
Scenario Generator
    ↓
Safety Preflight
    ↓
Injector
    ↓
System Under Test
    ↓
Telemetry
    ↓
Detector
    ↓
Classifier
    ↓
RCA
    ↓
Remediation
    ↓
Independent Verification
    ↓
Regression
    ↓
Evidence
    ↓
Report
```

---

# 13. AI Governance

AI belongs outside the Security Kernel.

```text
AI
 ├── detect
 ├── classify
 ├── analyze
 ├── recommend
 └── generate test candidates

        ↓

Deterministic Policy Engine

        ↓

ALLOW / DENY / APPROVAL / QUARANTINE
```

AI must not independently:

- alter trust state
- bypass authentication
- recover secrets
- delete protected data
- modify cryptographic policy
- approve its own security finding
- disable security controls
- access arbitrary tools

---

# 14. MCP / Tool Security

If MCP/tooling is used:

```text
Tool Request
   ↓
Authentication
   ↓
Authorization
   ↓
Allowlist
   ↓
Schema Validation
   ↓
Argument Validation
   ↓
Sandbox
   ↓
Rate Limit
   ↓
Audit
   ↓
Execution
```

Threats to test:

- prompt injection
- tool poisoning
- argument injection
- confused deputy
- excessive agency
- data exfiltration
- privilege escalation
- cross-tool contamination

This is a project architecture requirement derived from previous project discussions.

---

# 15. Threat Modeling

Required threat-model domains:

```text
Identity
Authentication
Device Trust
Key Management
Session Establishment
Messaging
Transport
NAT Traversal
Server Compromise
Client Compromise
Storage
Deletion
Recovery
Supply Chain
Build System
Dependencies
Updates
Telemetry
AI
Tools / MCP
Social Engineering
Metadata
```

Recommended references:

- NIST Cybersecurity Framework
- NIST SP 800-series security guidance
- STRIDE
- OWASP threat modeling material
- OWASP MASVS/MASTG

---

# 16. Supply Chain Security

Required project controls:

```text
Dependency Pinning
+
Dependency Auditing
+
SBOM
+
Provenance
+
Reproducible Builds
+
Signed Releases
+
CI Verification
+
Vulnerability Monitoring
```

Reference areas:

- SLSA
- SPDX
- CycloneDX
- OpenSSF
- GitHub dependency/security tooling

Official references:

- https://slsa.dev/
- https://spdx.dev/
- https://cyclonedx.org/
- https://openssf.org/

---

# 17. Build / Release Security

Release pipeline:

```text
Source
 ↓
Review
 ↓
Static Analysis
 ↓
Dependency Audit
 ↓
Unit Tests
 ↓
Integration Tests
 ↓
Protocol Tests
 ↓
Fuzzing
 ↓
Security Tests
 ↓
Build
 ↓
SBOM
 ↓
Provenance
 ↓
Artifact Signing
 ↓
Release Verification
```

---

# 18. Required Testing Classes

```text
Unit
Integration
Protocol
Interoperability
Cryptographic
Property-Based
Fuzz
State-Machine
Network
NAT/ICE
Transport
Storage
Deletion
Recovery
Attestation
Platform
Performance
Concurrency
Failure Injection
Regression
Supply Chain
Static Analysis
Dependency Audit
Penetration Testing
External Security Audit
```

---

# 19. Recovery

Separate:

```text
Identity / Account Recovery
```

from:

```text
Data Recovery
```

If the architecture does not retain encrypted user data or backups, identity recovery cannot magically restore deleted/local-only message history.

Preferred early path:

```text
Trusted Device
    ↓
Device-to-Device Transfer
    ↓
New Trusted Device
```

Additional recovery mechanisms require explicit threat modeling.

---

# 20. Deletion

Deletion must cover:

```text
Database
WAL / Journals
Cache
Attachments
Thumbnails
Temporary Files
Metadata
Indexes
Key Material
Derived State
```

The specification must distinguish:

- logical deletion
- cryptographic deletion
- application-level deletion
- OS/filesystem guarantees
- backup/restore behavior
- forensic residuals

Never claim absolute physical destruction unless the underlying platform provides a defensible guarantee.

---

# 21. Protocol Specification Reference

The protocol specification should eventually contain:

1. Scope
2. Terminology
3. Security Invariants
4. Identity Model
5. Key Model
6. Prekey Model
7. Session Establishment
8. PQXDH
9. Double Ratchet
10. Message Format
11. Serialization
12. Message State Machine
13. P2P State Machine
14. ICE / QUIC
15. TURN fallback
16. Offline behavior
17. Delivery semantics
18. Ordering
19. Replay protection
20. Deletion semantics
21. Recovery protocol
22. Device transfer
23. Server API contract
24. Error codes
25. Version negotiation
26. Downgrade protection
27. Capability negotiation
28. Test vectors
29. Interoperability
30. Conformance

---

# 22. Version Roadmap

## V1

- Identity
- Authentication
- 1:1 messaging
- PQXDH
- Double Ratchet
- local encrypted storage
- server rendezvous/prekey support
- direct transport foundation
- testing foundation
- deletion semantics

## V1.x

- P2P
- ICE
- QUIC
- TURN fallback
- recovery
- device transfer
- attestation hardening
- release hardening

## V2

Candidate areas:

- MLS/groups
- broader PQ migration
- Tor/privacy routing
- advanced privacy controls
- additional platform clients
- stronger privacy firewall

## V3

Candidate areas:

- Security Intelligence
- AI-assisted analysis
- advanced governance
- MCP security gateway
- advanced SOC/monitoring
- large-scale security automation

---

# 23. Stop-the-Line

Development stops for:

- cryptographic flaw
- trust bypass
- authentication bypass
- identity ambiguity
- key leakage
- message plaintext leakage
- recovery bypass
- deletion-policy bypass
- downgrade vulnerability
- critical dependency vulnerability
- build/release compromise
- serious supply-chain issue
- security-policy ambiguity

Fix → retest → verify → document → resume.

---

# 24. Source Classification

Every source must be tagged:

```text
STANDARD
OFFICIAL_DOCUMENTATION
REFERENCE_IMPLEMENTATION
LIBRARY
RESEARCH_PAPER
SECURITY_GUIDANCE
TESTING_STANDARD
TOOL
CASE_STUDY
PROJECT_DISCUSSION
ARCHIVED_REFERENCE
UNVERIFIED
```

---

# 25. Adoption Status

Use exactly one:

```text
PROPOSED
UNDER_REVIEW
ACCEPTED
IMPLEMENTED
VERIFIED
DEPRECATED
REJECTED
ARCHIVED
```

---

# 26. Source Priority

When sources disagree:

```text
1. Normative standard / RFC / official specification
2. Official vendor/platform documentation
3. Maintainer documentation of mature implementation
4. Peer-reviewed / institutional research
5. Independent security research
6. Community documentation
7. Blog / informal material
8. AI-generated material
```

AI-generated content is never sufficient as the sole authority for a security decision.

---

# 27. Research-to-Implementation Gate

Before adopting anything:

```text
Source
 ↓
Security Review
 ↓
Maturity Review
 ↓
License Review
 ↓
Maintenance Review
 ↓
Interoperability Review
 ↓
Threat Model
 ↓
Prototype
 ↓
Tests
 ↓
Decision
```

---

# 28. Current Reference Map

| Domain | Primary reference | Status |
|---|---|---|
| 1:1 cryptographic session | Signal PQXDH | Reference / V1 |
| Message ratchet | Signal Double Ratchet | Reference / V1 |
| Group messaging | MLS / RFC 9420 / OpenMLS | V2 candidate |
| PQC | NIST FIPS 203/204/205 | Reference |
| Transport | QUIC / RFC 9000 | V1/V1.x |
| NAT traversal | ICE / RFC 8445 | V1.x |
| Relay fallback | TURN / RFC 8656 | V1.x |
| Mobile security | OWASP MASVS/MASTG | Testing baseline |
| Android trust | Android Keystore/Attestation | Platform reference |
| Apple trust | Secure Enclave/App Attest | Platform reference |
| Local DB | SQLite | Candidate/reference |
| Telemetry | OpenTelemetry | Candidate/reference |
| Core language | Rust | Project direction |
| Engineering research | GHOSTS | Research reference |
| Test-lab methodology | DeepSeek cyber-range material | Research reference |
| Supply chain | SLSA/SPDX/CycloneDX/OpenSSF | Security engineering reference |

---

# 29. Important Non-Adoption Rules

The following are **not automatically part of PrivateMesh** merely because they appeared in research or discussions:

- Kafka
- Elasticsearch
- Logstash
- Suricata
- Zeek
- Sysmon
- Kubernetes
- Blockchain
- Federated Learning
- arbitrary AI agents
- arbitrary cloud storage
- arbitrary external telemetry
- arbitrary third-party cryptographic wrappers

Each requires a separate adoption decision.

---

# 30. Uploaded Research Material

## A Decade of GHOSTS in the Machine...

Classification:

`RESEARCH / SOFTWARE ENGINEERING / OBSERVABILITY / MAINTAINABILITY`

Use for:

- instrumentation
- observability
- traceability
- documentation
- CI/release discipline
- dependency management
- long-term maintainability
- complexity management

Do not treat it as a PrivateMesh protocol specification.

## DeepSeek — Into the Unknown / Cyber-range research material

Classification:

`RESEARCH / SECURITY TESTING / CYBER RANGE / AI GOVERNANCE`

Use for:

- scenario generation
- fault injection
- detection
- classification
- RCA
- evidence
- regression
- governance
- AI-assisted testing

Do not treat proposed tooling as mandatory architecture.

---

# 31. Reference Maintenance

This document must be reviewed whenever:

- a protocol version changes
- a cryptographic standard changes
- a platform security API changes
- a dependency is adopted/removed
- a vulnerability affects an adopted component
- a new implementation replaces an old one
- V1/V2/V3 scope changes

Each change should record:

```text
date
source
old decision
new decision
reason
security impact
compatibility impact
tests required
```

---

# 32. Final Project Rule

The project should prefer:

```text
Existing + Mature + Audited + Tested
```

over:

```text
New + Custom + Unreviewed
```

Especially for cryptography, identity, authentication, transport security and platform trust.

The goal is not to use the largest number of technologies.

The goal is:

```text
Small Core
+
Strong Standards
+
Mature Implementations
+
Explicit Contracts
+
Deterministic Security
+
Strong Testing
+
Evidence
+
Auditable Decisions
```

---

# 33. External Reference Links

## Signal
https://signal.org/docs/

## NIST PQC
https://www.nist.gov/pqc

## RFC Editor
https://www.rfc-editor.org/

## OpenMLS
https://openmls.tech/

## OWASP MASVS
https://mas.owasp.org/MASVS/

## OpenTelemetry
https://opentelemetry.io/docs/specs/otel/

## Rust
https://www.rust-lang.org/

## SQLite
https://www.sqlite.org/docs.html

## Android Security
https://developer.android.com/privacy-and-security/keystore

## Apple DeviceCheck / App Attest
https://developer.apple.com/documentation/devicecheck

## SLSA
https://slsa.dev/

## SPDX
https://spdx.dev/

## CycloneDX
https://cyclonedx.org/

## OpenSSF
https://openssf.org/

---

# 34. Important Limitation

هذا الملف يجمع ما تم تثبيته في سياق المشروع المتاح لي، وما ظهر من الملفات المرفوعة، والمراجع الخارجية التي تم التحقق منها.

لا يفترض هذا الملف أنه يستطيع استرجاع محادثات محذوفة أو ملفات لم تعد متاحة في سياق العمل.

لذلك يجب الاحتفاظ بهذا الملف داخل مستودع المشروع نفسه، ويفضل وضعه في:

`docs/references/MASTER_REFERENCE_INDEX.md`

ويمكن تقسيمه لاحقًا إلى ملفات متخصصة دون فقدان الفهرس الرئيسي.
