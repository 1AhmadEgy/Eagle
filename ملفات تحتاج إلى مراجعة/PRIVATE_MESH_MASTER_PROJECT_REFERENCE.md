# مشروع النسر — PrivateMesh
## MASTER PROJECT REFERENCE

**Document:** Master Project Reference  
**Project:** Eagle / النسر  
**Technical name:** PrivateMesh  
**Status:** Living reference — not a final protocol specification  
**Version:** 0.1  
**Date:** 2026-09-28

---

## 0. Purpose

هذا الملف هو المرجع الرئيسي الذي يربط:

`المصادر → البحث → القرارات → المواصفات → التنفيذ → الاختبارات → الأدلة`

لا يُفترض أن يكون هذا الملف بديلاً عن المواصفات التفصيلية. وظيفته تثبيت الصورة الموحدة للمشروع ومنع التناقض بين المحادثات والوثائق.

### قاعدة أساسية

لا تُعتمد تقنية أو مكتبة أو مشروع خارجي لمجرد أنه مشهور أو ذُكر في نقاش سابق. كل اعتماد يجب أن يمر عبر:

1. تحديد المشكلة التي يحلها.
2. تحديد الطبقة التي ينتمي إليها.
3. مراجعة المصدر الرسمي/المعياري.
4. مراجعة الأمن والصيانة والترخيص والتوافق.
5. اختبار PoC عند الحاجة.
6. تسجيل القرار في ADR.
7. إدخاله في المواصفة فقط بعد اعتماد القرار.

---

# 1. هوية المشروع

## 1.1 الاسم

- الاسم: **النسر**
- الاسم التقني: **PrivateMesh**
- الشعار/الهوية البصرية: نسر مستوحى من علم مصر، وفق هوية المشروع.

## 1.2 طبيعة المشروع

PrivateMesh منصة اتصال خاصة وآمنة، متعددة المنصات، مبنية حول:

- تشفير طرفي حقيقي.
- تقليل الثقة المطلوبة في الخادم.
- تخزين محلي للبيانات الحساسة.
- هوية وأجهزة وجلسات منفصلة منطقيًا.
- سياسات أمنية حتمية Fail-Closed.
- قابلية تدقيق واختبار.
- عدم جعل الذكاء الاصطناعي صاحب القرار الأمني النهائي.

## 1.3 المنصات

النظام مصمم ليكون Platform-Agnostic على مستوى البروتوكول والنواة، مع تطبيقات منفصلة حسب المنصة.

النطاق المستهدف يشمل، حسب مراحل التنفيذ:

- Android
- iOS
- Desktop
- Web عند اعتماد نموذج الأمان المناسب

---

# 2. المبادئ غير القابلة للتفاوض

1. **Security First**
2. **Privacy by Design**
3. **Protocol First**
4. **Fail Closed**
5. **Least Privilege**
6. **Minimal Server Trust**
7. **Local Ownership of Sensitive State**
8. **Deterministic Security Policy**
9. **Auditable Decisions**
10. **No Security by Obscurity**
11. **No Reinvention of Standard Cryptography**
12. **Test Before Release**
13. **External Security Audit Before Production Release**
14. **No V2 before V1 is stable and security-tested**
15. **AI cannot override security policy**
16. **Claims must not exceed what the technology actually guarantees**

---

# 3. المرجعية والسلطة

ترتيب السلطة المقترح:

1. Standards / RFC / NIST / official platform security documentation
2. Official documentation of adopted projects/libraries
3. Peer-reviewed research / high-quality technical research
4. Security advisories and independent audits
5. Project ADRs
6. Project specifications
7. Implementation
8. Discussion notes / chat history

إذا تعارضت ملاحظة محادثة مع معيار أو دليل رسمي موثوق، لا تُعتمد الملاحظة تلقائيًا؛ تُفتح ADR لمراجعة القرار.

---

# 4. النموذج المعماري

```text
                         PRIVATE MESH
                              |
             +----------------+----------------+
             |                                 |
        Client Side                       Minimal Services
             |                                 |
    +--------+---------+               +-------+-------+
    |                  |               |               |
Security Kernel     App Layer       Auth/Identity   Rendezvous
    |                  |               |               |
    +--------+---------+               +-------+-------+
             |
       Protocol / Crypto
             |
      Network / Transport
             |
     Local Secure Storage
```

## 4.1 المكونات الرئيسية

### PrivateMesh Core
النواة المرجعية المقترحة بلغة Rust، وتحتوي على:

- protocol state machines
- identity
- session state
- cryptographic interfaces
- trust decisions
- recovery policy
- deletion policy
- deterministic security policy

### PrivateMesh Clients

تطبيقات المنصات التي تستخدم النواة مع واجهة مستخدم وطبقة تكامل OS.

### PrivateMesh Network

طبقات النقل والاتصال، مثل:

- direct connectivity
- QUIC
- ICE
- TURN fallback
- خيارات خصوصية إضافية في مراحل لاحقة

### PrivateMesh Services

خدمات خفيفة لا تتولى محتوى الرسائل أو مفاتيح فك التشفير.

---

# 5. حدود الخادم Server Data Contract

الخادم ليس مخزن الرسائل.

## 5.1 ما يجب ألا يملكه الخادم

كقاعدة تصميمية:

- plaintext messages
- plaintext attachments
- message history
- private identity keys
- session encryption keys
- recovery secrets التي تكفي وحدها لاستعادة البيانات
- مفاتيح فك التشفير المحلية

## 5.2 ما قد يحتاجه الخادم

بحسب البروتوكول النهائي:

- account/authentication identifiers
- public identity information
- public/prekey material
- rendezvous/discovery metadata
- delivery/transport metadata اللازمة لتشغيل النظام
- abuse/rate-limit/security metadata وفق سياسة محددة

أي عنصر Server-side يجب تسجيله في **Server Data Contract** مع:

`purpose / sensitivity / retention / access / deletion / audit`

---

# 6. Security Kernel

النواة الأمنية الصغيرة هي نقطة التحكم في القرارات الحساسة:

```text
Security Kernel
├── Trust Engine
├── Recovery Engine
├── Deletion Engine
└── Policy Engine
```

## 6.1 خصائصها

- deterministic
- versioned
- auditable
- testable
- fail-closed
- independent from AI recommendations

## 6.2 AI boundary

الذكاء الاصطناعي يمكنه:

- اكتشاف أنماط شاذة.
- تحليل telemetry.
- اقتراح اختبارات.
- اقتراح remediation.
- تلخيص الأدلة.
- توليد سيناريوهات اختبار.

ولا يمكنه منفردًا:

- منح الثقة.
- قبول مفتاح هوية.
- تجاوز سياسة الحذف.
- استعادة أسرار.
- تغيير سياسة أمان حرجة.
- منح صلاحيات أعلى.
- تعطيل الضوابط.

---

# 7. نموذج الهوية

النموذج المفاهيمي:

```text
Username
   ↓
Public Identity
   ↓
Device Identity
   ↓
Session
   ↓
Ephemeral / Ratcheted Keys
```

يجب فصل:

- اسم المستخدم عن المفتاح.
- هوية الحساب عن هوية الجهاز.
- هوية الجهاز عن مفاتيح الجلسة.
- المصادقة عن استعادة البيانات.

---

# 8. التشفير والبروتوكول

## 8.1 المبدأ

لا يتم اختراع بروتوكول تشفير خاص بالمشروع.

## 8.2 1:1 messaging

مرشح التصميم الأساسي:

- Signal-style architecture
- PQXDH/X3DH-family key agreement بحسب الإصدار المعتمد
- Double Ratchet
- authenticated encryption

الاعتماد النهائي يجب أن يرتبط بنسخة مكتبة/بروتوكول محددة، وليس باسم عام فقط.

## 8.3 Groups

MLS / RFC 9420 وOpenMLS مرشحان للبحث في V2، وليس اعتمادًا تلقائيًا.

## 8.4 AEAD

AES-256-GCM وChaCha20-Poly1305 خيارات تقنية محتملة داخل البروتوكول؛ لا تُعامل كبديل عن تصميم بروتوكول كامل.

## 8.5 Post-Quantum

المراجع الأساسية:

- NIST FIPS 203 — ML-KEM
- NIST FIPS 204 — ML-DSA
- NIST FIPS 205 — SLH-DSA

PQC في V1 لا يُفترض تلقائيًا دون تحديد:

- threat horizon
- interoperability
- performance
- implementation maturity
- migration strategy
- downgrade protection

---

# 9. الشبكة

## 9.1 الفصل بين الطبقات

```text
Application
   ↓
E2E Protocol
   ↓
Session
   ↓
Transport
   ↓
Network
```

### QUIC

طبقة نقل مرشحة.

### ICE

آلية لاكتشاف مسار اتصال مناسب عبر NAT.

### TURN

Fallback relay عندما يتعذر الاتصال المباشر.

### WebRTC

مسار محتمل للصوت/الفيديو، وليس بديلًا عامًا لـ QUIC أو ICE أو TURN.

### Tor

خيار خصوصية/رouting محتمل في مرحلة لاحقة. لا يُعتبر بديلًا عن E2E ولا ضمانًا لإخفاء كل metadata.

---

# 10. الثقة في الجهاز

مرشحون حسب المنصة:

- Android Keystore / StrongBox / Key Attestation
- Apple Secure Enclave / App Attest
- Desktop TPM 2.0

لكن:

> Attestation لا يثبت أن الجهاز آمن بالكامل أو غير مخترق.

لذلك يجب أن يكون القرار الأمني مبنيًا على دليل محدود ومحدد، مع تعريف واضح لحالات الثقة.

---

# 11. التخزين المحلي

المبدأ:

- البيانات الحساسة محليًا.
- قاعدة بيانات محلية مشفرة.
- أقل قدر ممكن من البيانات المشتقة.
- عدم تسجيل الأسرار في logs.
- حماية ملفات temporary/cache/WAL/journal وفق نموذج التخزين الفعلي.

SQLite/SQLCipher خيارات بحث/تنفيذ محتملة، ويجب اعتماد النسخة والتهيئة والسياسة بعد التحقق.

---

# 12. الحذف

الحذف ليس ادعاءً بـ "تدمير جنائي مطلق".

يجب الفصل بين:

1. Logical deletion
2. Application-level deletion
3. Cryptographic deletion
4. OS/filesystem behavior
5. Backups
6. Journals/WAL/temp/cache
7. Forensic residuals

## 12.1 النطاق

عند تطبيق سياسة الحذف، يجب دراسة:

- messages
- attachments
- thumbnails
- local indexes
- cache
- temporary files
- derived state
- relevant key material

## 12.2 مدد الحذف

خيارات سابقة للمناقشة:

- 5 seconds
- 1 minute
- 10 minutes
- 1 hour
- 24 hours
- 7 days
- custom

المدة النهائية جزء من مواصفة الحذف وليست قرارًا أمنيًا مستقلًا.

---

# 13. الاستعادة Recovery

يجب الفصل بين:

### Identity Recovery
استعادة القدرة على إثبات/استعادة هوية الحساب.

### Data Recovery
استعادة الرسائل/البيانات المحلية.

لا يعني استرداد الهوية تلقائيًا استرداد البيانات القديمة.

الترتيب المقترح للدراسة:

1. Device-to-device transfer
2. Recovery code
3. Trusted contacts
4. Repeated identity proofing

كل آلية تحتاج threat model مستقلًا.

---

# 14. Test Lab

مختبر الاختبار داخل ChatGPT يجب أن يكون مساعدًا للتطوير قبل GitHub، وليس بديلًا عن الاختبارات المحلية والمستقلة.

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

## 14.1 أنواع الاختبار

- Build tests
- Unit tests
- Integration tests
- Protocol tests
- Interoperability tests
- Security tests
- Fuzzing
- Property-based testing
- Negative testing
- Fault injection
- Privacy tests
- Deletion tests
- Recovery tests
- Supply-chain checks
- Release checks
- External penetration testing

## 14.2 سيناريو الاختبار

```yaml
id:
title:
version:
objective:
scope:
category:
layer:
risk:
preconditions:
stimulus:
injection:
expected_behavior:
observables:
detection:
classification:
evidence:
verification:
recovery:
rollback:
regression:
status:
```

---

# 15. نموذج الأحداث والـTelemetry

لا يجوز أن تتحول المراقبة إلى تسريب أسرار.

ممنوع افتراضيًا تسجيل:

- plaintext messages
- private keys
- session secrets
- recovery codes
- unnecessary PII

يجب أن يكون كل event مصممًا حول:

`event_type / timestamp / component / correlation_id / severity / safe_metadata`

مع سياسة retention واضحة.

---

# 16. أمن الذكاء الاصطناعي وMCP

في حال استخدام Agents/MCP/tools:

التهديدات التي يجب اختبارها تشمل:

- prompt injection
- tool poisoning
- argument injection
- confused deputy
- excessive agency
- privilege escalation
- data exfiltration
- cross-tool contamination
- unauthorized side effects

المبدأ:

```text
AI Recommendation
       ↓
Deterministic Policy
       ↓
Authorization
       ↓
Execution
```

وليس:

```text
AI → Execute
```

---

# 17. سلسلة الإمداد

يجب إنشاء مسار مستقل لـ:

- dependency inventory
- SBOM
- provenance
- package integrity
- signing
- reproducible builds حيثما أمكن
- vulnerability scanning
- license review
- dependency update policy

مراجع بحثية/معيارية مرشحة:

- SLSA
- SPDX
- CycloneDX
- OpenSSF

---

# 18. خارطة الإصدارات

## V1 — Foundation

النطاق الأساسي:

- 1:1 messaging
- identity
- session security
- Signal-family protocol design
- local encrypted storage
- device trust integration
- direct connectivity
- QUIC/ICE/TURN بحسب التصميم النهائي
- deletion
- recovery
- minimal server
- Android + iOS

لا يتم إدخال كل ميزات V2/V3 إلى V1 لمجرد وجودها في الخطة.

## V2 — Hardening & Scale

مرشح للنطاق:

- MLS/OpenMLS
- hybrid PQC
- privacy routing
- privacy firewall
- security modes
- multipath connectivity
- enhanced deletion
- desktop
- web حيث يكون النموذج الأمني مناسبًا
- SDK
- advanced security monitoring

## V3 — AI & Governance

مرشح للنطاق:

- AI anomaly detection
- MCP gateway/security
- Wazuh
- Tracecat/Shuffle
- TheHive/Cortex
- controlled automated response
- advanced cryptographic migration
- enterprise governance

كل انتقال يحتاج:

`implementation → tests → security review → audit gate`

---

# 19. بوابات الإيقاف Stop-the-Line

يجب إيقاف الإصدار عند وجود أي من التالي:

- private key leakage
- plaintext E2E bypass
- authentication bypass
- unauthorized trust elevation
- recovery bypass
- deletion-policy bypass
- critical dependency vulnerability بدون معالجة مقبولة
- reproducibility/provenance failure إذا كانت مطلوبة للإصدار
- failing security regression
- protocol interoperability failure في مسار أساسي
- unexplained security-critical behavior
- test evidence غير كافٍ للادعاء الأمني

---

# 20. حالة الاعتماد Adoption Status

كل تقنية يجب أن تحمل حالة:

| الحالة | المعنى |
|---|---|
| REFERENCE | مصدر للبحث فقط |
| CANDIDATE | مرشح للدراسة |
| POC | تم اختبار فكرة/تكامل أولي |
| ADOPTED | قرار اعتماد مسجل |
| REQUIRED | جزء إلزامي من المواصفة |
| OPTIONAL | ميزة اختيارية |
| DEFERRED | مؤجلة لمرحلة لاحقة |
| REJECTED | مرفوضة مع سبب |
| REPLACED | استُبدلت بقرار أحدث |

لا يُسمح بتحويل `CANDIDATE` إلى `ADOPTED` ضمنيًا.

---

# 21. ADRs

كل قرار معماري مهم يجب أن يسجل:

```text
ADR-ID
Title
Status
Context
Problem
Options
Security impact
Privacy impact
Operational impact
Compatibility
Performance
Supply-chain impact
Decision
Rejected alternatives
Migration
Test evidence
References
```

قرارات أساسية مبدئية:

- ADR-001: Rust Core
- ADR-002: Signal-family 1:1 protocol
- ADR-003: Minimal Server Trust
- ADR-004: Local-first sensitive state
- ADR-005: Deterministic Security Kernel
- ADR-006: Direct connectivity + relay fallback
- ADR-007: AI cannot override security policy
- ADR-008: V1/V2/V3 separation

---

# 22. المصادر المرجعية الرئيسية

يجب أن يحتفظ المشروع بسجل مصادر مستقل، مع رابط وإصدار وتاريخ تحقق.

## Standards / Specifications

- IETF RFC 8446 — TLS 1.3
- IETF RFC 9000 — QUIC
- IETF RFC 8445 — ICE
- IETF RFC 8656 — TURN
- IETF RFC 9420 — MLS
- NIST FIPS 203 — ML-KEM
- NIST FIPS 204 — ML-DSA
- NIST FIPS 205 — SLH-DSA

## Security / Testing

- OWASP MASVS
- OWASP MASTG
- SLSA
- SPDX
- CycloneDX
- OpenSSF

## Platform Security

- Android Keystore / StrongBox / Key Attestation
- Apple Secure Enclave / App Attest
- TPM 2.0 ecosystem

## Projects / Libraries to evaluate

- Signal/libsignal ecosystem
- OpenMLS
- Rust ecosystem
- SQLite / SQLCipher
- OpenTelemetry
- Wazuh
- Suricata / Snort
- MISP
- TheHive / Cortex
- Tracecat / Shuffle

هذه القائمة لا تعني أن كل عنصر فيها معتمد.

---

# 23. أبحاث المشروع

مواد مثل:

- GHOSTS
- DeepSeek / cyber-range material
- أوراق وبحوث توليد السيناريوهات
- event-driven telemetry
- fault injection
- detection/classification/RCA/remediation

تُستخدم كمصادر بحثية.

لا تُنسخ معماريتها إلى PrivateMesh دون ADR ومقارنة مع المتطلبات الأمنية للمشروع.

---

# 24. بنية التوثيق القياسية

```text
docs/
├── 00-project/
├── 01-specification/
├── 02-architecture/
├── 03-cryptography/
├── 04-platform-security/
├── 05-network/
├── 06-security/
├── 07-testing/
├── 08-test-lab/
├── 09-ai/
├── 10-operations/
├── 11-supply-chain/
├── 12-research/
├── 13-decisions/
└── references/
```

المرجع الحالي يوضع في:

```text
docs/references/MASTER_PROJECT_REFERENCE.md
```

ومؤشر المصادر في:

```text
docs/references/MASTER_REFERENCE_INDEX.md
```

---

# 25. مصفوفة الفصل بين المرجع والقرار

```text
External Source
      ↓
Source Record
      ↓
Research Finding
      ↓
Security Review
      ↓
ADR
      ↓
Specification
      ↓
Implementation
      ↓
Test Evidence
      ↓
Release
```

أي قفزة تتجاوز هذه السلسلة تحتاج مبررًا مسجلًا.

---

# 26. ما تم حسمه وما لم يُحسم

## محسوم مبدئيًا

- security-first
- privacy-first
- local-first sensitive state
- minimal server trust
- protocol-first
- deterministic security policy
- AI cannot override critical security decisions
- V1 قبل V2
- external security audit قبل الإنتاج

## غير محسوم نهائيًا

- نسخة/تنفيذ Signal المحدد
- شكل PQXDH النهائي المستخدم
- مكتبة التشفير النهائية
- PQC timing/migration
- MLS adoption
- WebRTC scope
- Tor scope
- desktop/web security model
- exact recovery protocol
- exact deletion guarantees
- exact server retention
- exact device trust states
- final database/encryption configuration
- final observability stack

---

# 27. قواعد ضد التناقض

عند ظهور قرار جديد:

1. لا يُعدل أكثر من ملف مباشرة.
2. حدد القرار المتعارض.
3. افتح ADR.
4. حدد التأثير على protocol/architecture/security/testing.
5. حدث المرجع الرئيسي.
6. حدث المواصفات المتأثرة.
7. أضف regression test إن أمكن.
8. أغلق القرار القديم أو علّمه REPLACED.

---

# 28. تعريف "جاهز"

لا يعني "يعمل على جهازي".

Feature Ready يعني على الأقل:

```text
Specification
+ Implementation
+ Unit Tests
+ Integration Tests
+ Negative Tests
+ Security Tests
+ Regression Tests
+ Evidence
+ Review
```

أما Release Ready فيضيف:

```text
Supply-chain checks
+ Release verification
+ Security review
+ Required external audit
```

---

# 29. قاعدة المشروع الذهبية

> **لا نضيف تقنية لأنها قوية؛ نضيفها لأنها تحل مشكلة محددة، ويمكن تبريرها، واختبارها، وصيانتها، وتأمينها.**

والقاعدة المقابلة:

> **لا نحذف تقنية لأنها معقدة؛ نحذفها أو نؤجلها عندما لا تقدم قيمة مثبتة تتناسب مع تكلفتها ومخاطرها في المرحلة الحالية.**

---

# 30. الخطوة التالية

بعد اعتماد هذا المرجع، يتم تفكيكه إلى الوثائق التنفيذية التالية بالترتيب:

1. `PROJECT_PRINCIPLES.md`
2. `SYSTEM_ARCHITECTURE.md`
3. `SECURITY_KERNEL.md`
4. `PROTOCOL_SPECIFICATION.md`
5. `IDENTITY_SPECIFICATION.md`
6. `SESSION_SPECIFICATION.md`
7. `SERVER_DATA_CONTRACT.md`
8. `DELETION_SPECIFICATION.md`
9. `RECOVERY_SPECIFICATION.md`
10. `THREAT_MODEL.md`
11. `TEST_STRATEGY.md`
12. `TEST_MATRIX.md`
13. `TEST_LAB/`
14. ADRs
15. Source Registry

**ملاحظة:** هذا الملف مرجع تنسيقي/معماري. لا يمثل بحد ذاته اعتمادًا نهائيًا لأي مكتبة أو بروتوكول أو تنفيذ.
