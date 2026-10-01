# مشروع النسر — MASTER REFERENCE INDEX

## وظيفة الملف

هذا الملف هو فهرس المصادر، وليس مواصفة المشروع.

القاعدة:

`Source → Research → ADR → Specification → Implementation → Test Evidence`

## التصنيف

### 01 — Standards / RFCs
- IETF RFCs
- NIST FIPS
- W3C/ISO/ETSI عند الحاجة

### 02 — Cryptography
- Signal protocol ecosystem
- PQC standards
- AEAD / KDF / signature references

### 03 — Group Messaging
- MLS / RFC 9420
- OpenMLS

### 04 — Network
- QUIC / RFC 9000
- ICE / RFC 8445
- TURN / RFC 8656
- WebRTC
- Tor

### 05 — Platform Security
- Android Keystore
- StrongBox
- Key Attestation
- Secure Enclave
- App Attest
- TPM 2.0

### 06 — Storage
- SQLite
- SQLCipher
- OS secure storage facilities

### 07 — Security Engineering
- OWASP MASVS
- OWASP MASTG
- threat modeling references
- secure coding guidance

### 08 — Supply Chain
- SLSA
- SPDX
- CycloneDX
- OpenSSF

### 09 — Observability
- OpenTelemetry
- Wazuh
- Suricata
- Snort
- MISP

### 10 — Incident Response / Automation
- TheHive
- Cortex
- Tracecat
- Shuffle

### 11 — Research
- GHOSTS
- DeepSeek / cyber-range material
- academic papers
- security studies
- benchmark reports

### 12 — Libraries / Projects
لكل مكتبة سجل مستقل يحتوي:
- name
- repository
- official documentation
- license
- version
- maintenance state
- security advisories
- supported platforms
- integration status
- test status
- adoption status

## حالات المصدر

`REFERENCE | CANDIDATE | POC | ADOPTED | REQUIRED | OPTIONAL | DEFERRED | REJECTED | REPLACED`

## قاعدة مهمة

وجود المصدر في هذا الفهرس لا يعني اعتماده في PrivateMesh.
