# 🦅 مشروع النسر --- Master Project Reference & Conversation Map

> **الغرض:** هذا الملف هو المرجع التشغيلي المركزي للمشروع بعد إعادة
> تنظيم المحادثات.
>
> **قاعدة أساسية:** لا تُعتبر أي معلومة أو قرار أو تقنية "معتمدة" لمجرد
> ورودها في نقاش. يجب أن تمر عبر مسار: مصدر → دليل/تحليل → قرار → مواصفة
> → تنفيذ → اختبار → تحقق → توثيق.

------------------------------------------------------------------------

# 1. هوية المشروع

## 1.1 الاسم

**مشروع النسر --- Eagle**

## 1.2 النطاق التقني

المشروع يتضمن PrivateMesh كنواة/بروتوكول اتصال آمن، مع تطبيقات ومنصات
وخدمات وأدوات مساندة.

## 1.3 الأولوية

-   الأمان القوي.
-   تقليل سطح الهجوم.
-   فصل الصلاحيات.
-   التحقق قبل الاعتماد.
-   الاعتماد على مشاريع ومكتبات وأدوات وأبحاث جاهزة ومختبرة عندما يكون
    ذلك مناسبًا.
-   عدم إضافة تقنية لمجرد الحداثة أو الشهرة.
-   قابلية التدقيق وإعادة الإنتاج.
-   عدم السماح لمكون واحد بامتلاك صلاحيات غير ضرورية.

------------------------------------------------------------------------

# 2. القاعدة الحاكمة

``` text
SOURCE
  ↓
EVIDENCE
  ↓
REVIEW
  ↓
DECISION / ADR
  ↓
SPECIFICATION
  ↓
IMPLEMENTATION
  ↓
TEST
  ↓
VERIFICATION
  ↓
DOCUMENTATION
  ↓
RELEASE
```

أي عنصر لا يستطيع المرور بهذا المسار يبقى: - مقترحًا، - أو تجريبيًا، - أو
غير معتمد، ولا يدخل إلى المسار الإنتاجي دون قرار واضح.

------------------------------------------------------------------------

# 3. خريطة المحادثات النهائية

  -----------------------------------------------------------------------
  ID                      الاسم                   الوظيفة الأساسية
  ----------------------- ----------------------- -----------------------
  00                      القيادة والمرجع الرئيسي الحالة، القرارات،
                                                  المخاطر، الربط

  01                      التأسيس وPrivateMesh    النواة والتنفيذ الأساسي

  02                      UX/UI                   تجربة المستخدم
                                                  والواجهات

  03                      Architecture            المعمارية وحدود
                                                  المكونات

  04                      Engineering             التنفيذ التقني والتكامل

  05                      Security Core &         التشفير، الهوية،
                          Cryptography            المفاتيح

  06                      PrivateMesh Security    Threat Model وأمن
                                                  البروتوكول

  07                      Security Testing        الاختبارات الهجومية
                                                  والأمنية

  08                      Release & CI/CD         البناء، التوقيع،
                                                  الإصدار

  09                      QA Test Lab             الاختبارات الوظيفية
                                                  والتوافقية

  10                      Technical               المواصفات الرسمية
                          Specifications          

  11                      Engineering             التوثيق أثناء التطوير
                          Documentation           

  12                      Final Documentation     التوثيق النهائي
                                                  والتسليم

  13                      Android                 أمن وتنفيذ Android

  14                      Apple                   أمن وتنفيذ Apple

  15                      Network / P2P /         QUIC/ICE/TURN/NAT
                          Transport               

  16                      Storage / Recovery /    البيانات المحلية
                          Deletion                والاسترداد والحذف

  17                      AI Governance           AI وحدود صلاحياته

  18                      MCP / Tools /           الأدوات والتكاملات
                          Integrations            والصلاحيات

  19                      Supply Chain &          المكتبات، SBOM،
                          Dependencies            provenance

  20                      Sources / Research      المصادر والأبحاث
                          Library                 والمشاريع

  21                      Future / V2 / V3        البحث والتجارب
                                                  والمستقبل
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 4. المحادثة 00 --- القيادة والمرجع الرئيسي

## الهدف

المصدر التشغيلي الأعلى للمشروع.

## تحتوي على

-   Vision.
-   Scope.
-   Current Phase.
-   Current Version.
-   Approved Decisions.
-   Pending Decisions.
-   ADR Index.
-   Requirements Status.
-   Security Blockers.
-   Risks.
-   Dependencies.
-   Milestones.
-   Release Status.
-   Cross-conversation links.
-   Master source index.
-   Change log.

## لا تحتوي على

-   تفاصيل تنفيذية ضخمة.
-   كود طويل.
-   نقاشات تقنية متكررة.
-   تجارب غير موثقة.

## المخرجات

-   قرار.
-   تكليف.
-   حالة.
-   Blocker.
-   رابط إلى القسم المختص.

------------------------------------------------------------------------

# 5. المحادثة 01 --- التأسيس وPrivateMesh

## النطاق

-   Repository structure.
-   Core modules.
-   Runtime.
-   Service boundaries.
-   Core protocol implementation.
-   Initial project scaffolding.

## المخرجات

-   Project structure.
-   Buildable baseline.
-   Core interfaces.
-   Initial implementation plan.
-   Definition of Done.

## ممنوع

اعتماد أي crypto أو protocol choice بلا الرجوع إلى 05/06/10.

------------------------------------------------------------------------

# 6. المحادثة 02 --- UX/UI

## النطاق

-   Onboarding.
-   Identity UX.
-   Device enrollment.
-   Pairing.
-   Messaging UX.
-   Security indicators.
-   Recovery flows.
-   Error states.
-   Accessibility.
-   Localization.
-   Offline/online states.

## Security UX

يجب أن تكون حالات: - Verified. - Unverified. - Suspicious. - Blocked. -
Recovery. - Key change. واضحة للمستخدم دون كشف أسرار.

------------------------------------------------------------------------

# 7. المحادثة 03 --- Architecture

## النطاق

-   System architecture.
-   Module boundaries.
-   Trust boundaries.
-   Data flow.
-   Control flow.
-   Dependency direction.
-   Threat boundaries.
-   Platform abstraction.
-   Service isolation.

## قاعدة

كل مكون يجب أن يمتلك: - Responsibility. - Inputs. - Outputs. - Trust
level. - Required permissions. - Failure behavior. - Test boundary.

------------------------------------------------------------------------

# 8. المحادثة 04 --- Engineering

## النطاق

-   Implementation.
-   APIs.
-   Interfaces.
-   Error handling.
-   Concurrency.
-   Serialization.
-   Configuration.
-   Observability.
-   Performance.
-   Integration.

## قواعد

-   Fail closed حيث يلزم.
-   Validate inputs.
-   Avoid implicit trust.
-   Minimize privileges.
-   Avoid secrets in logs.
-   No security bypass for convenience.

------------------------------------------------------------------------

# 9. المحادثة 05 --- Security Core & Cryptography

## النطاق

-   Cryptographic primitives.
-   Key lifecycle.
-   Key generation.
-   Key storage.
-   Key rotation.
-   Session establishment.
-   Authentication.
-   Authorization primitives.
-   Secure randomness.
-   Secure memory considerations.
-   Crypto-agility.

## قاعدة

لا نبتكر cryptography خاصة بالمشروع عندما توجد primitives قياسية ومراجعة
ومختبرة.

## لكل primitive

-   Purpose.
-   Algorithm.
-   Parameters.
-   Threat assumptions.
-   Library.
-   Version.
-   Test vectors.
-   Failure behavior.
-   Migration plan.

------------------------------------------------------------------------

# 10. المحادثة 06 --- PrivateMesh Security

## Threat Model

يشمل: - Network attacker. - Malicious peer. - Compromised endpoint. -
Stolen device. - Malicious local process. - Malicious plugin/tool. -
Supply-chain compromise. - Metadata observer. - Replay attacker. -
Downgrade attacker. - Identity substitution. - Key compromise.

## Security properties

-   Confidentiality.
-   Integrity.
-   Authentication.
-   Forward secrecy where applicable.
-   Replay protection.
-   Downgrade resistance.
-   Key separation.
-   Least privilege.
-   Auditability.

------------------------------------------------------------------------

# 11. المحادثة 07 --- Security Testing

## أنواع الاختبارات

-   Fuzzing.
-   Mutation testing.
-   Abuse cases.
-   Authentication bypass.
-   Authorization bypass.
-   Parser attacks.
-   Serialization attacks.
-   Protocol state attacks.
-   Replay.
-   Downgrade.
-   Malformed packets.
-   Resource exhaustion.
-   Tool abuse.
-   Prompt injection.
-   Data exfiltration.
-   Supply-chain scenarios.

## قاعدة

أي Security Finding يجب أن يحتوي على: - ID. - Severity. - Affected
component. - Reproduction. - Expected. - Actual. - Impact. - Fix. -
Regression test. - Verification.

------------------------------------------------------------------------

# 12. المحادثة 08 --- Release / CI/CD

## Pipeline

``` text
Checkout
↓
Dependency Verification
↓
Lint / Format
↓
Static Analysis
↓
Unit Tests
↓
Integration Tests
↓
Protocol Tests
↓
Security Tests
↓
Fuzzing Gate
↓
Build
↓
SBOM
↓
Provenance
↓
Artifact Verification
↓
Signing
↓
Release
```

## Release Gate

لا يتم الإصدار عند وجود: - Critical security issue. - Failed required
test. - Unverified artifact. - Broken provenance. - Dependency policy
violation. - Missing release evidence.

------------------------------------------------------------------------

# 13. المحادثة 09 --- QA Test Lab

## طبقات الاختبار

### Unit

اختبار المكونات الصغيرة.

### Integration

اختبار تفاعل المكونات.

### System

اختبار النظام الكامل.

### Protocol

اختبار حالات البروتوكول.

### Interoperability

اختبار التوافق بين الإصدارات والمنصات.

### Regression

منع عودة الأخطاء.

### Performance

Latency / Throughput / Resource use.

### Recovery

الفشل، الانقطاع، الاستعادة.

### Conformance

مطابقة المواصفة.

## كل Test Case

-   ID.
-   Objective.
-   Preconditions.
-   Input.
-   Steps.
-   Expected.
-   Actual.
-   Environment.
-   Artifact.
-   Result.

------------------------------------------------------------------------

# 14. المحادثة 10 --- Technical Specifications

## الوثائق الرسمية

-   Protocol specification.
-   State machines.
-   Message formats.
-   Error codes.
-   Authentication flows.
-   Session lifecycle.
-   Key lifecycle.
-   Recovery protocol.
-   Device pairing.
-   Transport behavior.
-   Compatibility rules.

## القاعدة

أي شيء غير موجود في المواصفة الرسمية لا يُعامل كـprotocol requirement إلا
بعد اعتماده.

------------------------------------------------------------------------

# 15. المحادثة 11 --- Engineering Documentation

## تحتوي على

-   Architecture notes.
-   Implementation guides.
-   Developer setup.
-   Build instructions.
-   Testing instructions.
-   Debugging.
-   Operational notes.
-   Design rationale.

## الفرق عن 12

11 = توثيق أثناء التطوير. 12 = النسخة النهائية القابلة للتسليم.

------------------------------------------------------------------------

# 16. المحادثة 12 --- Final Documentation

## الحزمة النهائية

-   README.
-   Architecture.
-   Protocol.
-   Security model.
-   Threat model.
-   Deployment.
-   Build.
-   Testing.
-   Recovery.
-   Privacy.
-   Operations.
-   Release process.
-   Known limitations.
-   Version history.

------------------------------------------------------------------------

# 17. المحادثة 13 --- Android

## النطاق

-   Android app.
-   Keystore.
-   StrongBox حيث يتوفر.
-   Hardware-backed keys.
-   Attestation.
-   Secure storage.
-   Biometric integration.
-   App lifecycle.
-   Backup behavior.
-   Screenshot/sensitive UI controls.
-   Root/compromise considerations.
-   Network security configuration.

## الاختبارات

-   Device matrix.
-   OS versions.
-   Hardware-backed vs software-backed.
-   Backup/restore.
-   Key invalidation.
-   Lock/unlock.
-   Offline behavior.

------------------------------------------------------------------------

# 18. المحادثة 14 --- Apple

## النطاق

-   Keychain.
-   Secure Enclave.
-   App Attest.
-   DeviceCheck عند الحاجة.
-   Biometrics.
-   Backup semantics.
-   Key lifecycle.
-   App lifecycle.
-   Secure UI.

## الاختبارات

-   iOS versions.
-   Device families.
-   Secure Enclave availability.
-   Restore.
-   Key invalidation.
-   Lock state.
-   Offline behavior.

------------------------------------------------------------------------

# 19. المحادثة 15 --- Network / P2P / Transport

## النطاق

-   QUIC.
-   ICE.
-   TURN.
-   NAT traversal.
-   Connectivity fallback.
-   Connection lifecycle.
-   Timeout.
-   Retry.
-   Congestion behavior.
-   Network change.
-   IPv4/IPv6.
-   Offline/online transitions.

## قاعدة

Transport security لا تساوي E2E message security.

## المطلوب

فصل:

``` text
Transport
Session
Message Encryption
Identity
Metadata
```

------------------------------------------------------------------------

# 20. المحادثة 16 --- Storage / Recovery / Deletion

## Storage

-   SQLite/database.
-   WAL.
-   Journal.
-   Cache.
-   Attachments.
-   Temporary files.
-   Metadata.
-   Indexes.
-   Derived state.

## Recovery

-   Backup.
-   Restore.
-   Device transfer.
-   Key recovery.
-   Failure recovery.

## Deletion

يتم اختبار: - Logical deletion. - Cryptographic deletion. - Application
deletion. - Cache deletion. - Temporary data. - Backup behavior. - Key
destruction. - Residual data.

## قاعدة

لا يتم الادعاء بـ"حذف آمن" دون تحديد حدود الضمان.

------------------------------------------------------------------------

# 21. المحادثة 17 --- AI Governance

## AI يستطيع

-   Analyze.
-   Classify.
-   Recommend.
-   Generate test candidates.
-   Detect anomalies.
-   Assist developers.

## AI لا يمتلك تلقائيًا

-   Root authority.
-   Crypto authority.
-   Key access.
-   Unrestricted tool execution.
-   Security policy override.
-   Protected-data deletion.
-   Trust-state modification.

## Architecture

``` text
AI
 ↓
Policy Layer
 ↓
Authorization
 ↓
Tool Sandbox
 ↓
Validation
 ↓
Execution
```

## AI Security

-   Prompt injection.
-   Context poisoning.
-   Tool poisoning.
-   Data leakage.
-   Excessive agency.
-   Privilege escalation.
-   Cross-tool contamination.

------------------------------------------------------------------------

# 22. المحادثة 18 --- MCP / Tools / Integrations

## لكل Tool

-   Identity.
-   Owner.
-   Purpose.
-   Input schema.
-   Output schema.
-   Permissions.
-   Data access.
-   Network access.
-   Rate limits.
-   Audit events.
-   Failure policy.
-   Sandbox.

## Tool execution

``` text
Request
↓
Authentication
↓
Authorization
↓
Allowlist
↓
Schema validation
↓
Argument validation
↓
Sandbox
↓
Execution
↓
Audit
```

------------------------------------------------------------------------

# 23. المحادثة 19 --- Supply Chain & Dependencies

## النطاق

-   Dependencies.
-   Libraries.
-   Packages.
-   SDKs.
-   Build tools.
-   CI actions.
-   Container images.
-   Third-party services.

## لكل dependency

-   Name.
-   Version.
-   Source.
-   License.
-   Maintainer.
-   Security history.
-   Known vulnerabilities.
-   Transitive dependencies.
-   Hash/provenance where applicable.
-   Update policy.
-   Removal plan.

## Security controls

-   Pinning.
-   Lockfiles.
-   Audit.
-   SBOM.
-   Provenance.
-   Reproducible builds.
-   Signed artifacts.
-   Dependency review.
-   Vulnerability monitoring.

------------------------------------------------------------------------

# 24. المحادثة 20 --- Sources / Research Library

## التصنيف

``` text
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
UNVERIFIED
ARCHIVED
```

## لكل مصدر

-   Source ID.
-   Title.
-   Publisher.
-   URL/repository.
-   Version/date.
-   Category.
-   Relevant component.
-   Claim supported.
-   Evidence extracted.
-   Confidence.
-   Review status.
-   Related decision.
-   Related specification.
-   Related test.

## حالات المصدر

``` text
PROPOSED
UNDER_REVIEW
ACCEPTED
IMPLEMENTED
VERIFIED
DEPRECATED
REJECTED
ARCHIVED
```

------------------------------------------------------------------------

# 25. المحادثة 21 --- Future / V2 / V3

## تستخدم لـ

-   Research.
-   Experimental ideas.
-   Alternative architectures.
-   Future protocols.
-   New libraries.
-   New hardware capabilities.
-   Performance experiments.
-   Long-term roadmap.

## ممنوع

إدخال فكرة تجريبية إلى V1 بدون نقلها رسميًا إلى مسار القرار والمواصفة.

------------------------------------------------------------------------

# 26. نظام المتطلبات

كل Requirement له:

``` text
REQ-ID
Title
Description
Rationale
Priority
Source
Security impact
Affected components
Acceptance criteria
Implementation status
Test IDs
Verification evidence
```

الحالات:

``` text
PROPOSED
APPROVED
IN_PROGRESS
IMPLEMENTED
TESTED
VERIFIED
DEPRECATED
REJECTED
```

------------------------------------------------------------------------

# 27. نظام ADR

كل قرار معماري مهم:

``` text
ADR-ID
Title
Context
Problem
Options
Decision
Consequences
Security impact
Alternatives rejected
References
Implementation
Tests
Status
```

الحالات:

``` text
PROPOSED
ACCEPTED
SUPERSEDED
DEPRECATED
REJECTED
```

------------------------------------------------------------------------

# 28. نظام Security Finding

``` text
SEC-ID
Title
Component
Threat
Severity
Exploitability
Impact
Evidence
Reproduction
Fix
Regression Test
Verification
Status
```

الحالات:

``` text
OPEN
TRIAGED
IN_PROGRESS
FIXED
VERIFIED
ACCEPTED_RISK
CLOSED
```

------------------------------------------------------------------------

# 29. نظام الاختبارات

``` text
TEST-ID
Requirement
Component
Type
Environment
Preconditions
Input
Expected
Actual
Evidence
Result
Regression
```

النتيجة:

``` text
PASS
FAIL
BLOCKED
NOT_RUN
NOT_APPLICABLE
```

------------------------------------------------------------------------

# 30. Traceability Matrix

العلاقة الرسمية:

``` text
Source
  ↓
Requirement
  ↓
ADR
  ↓
Specification
  ↓
Implementation
  ↓
Test
  ↓
Security Test
  ↓
Verification
  ↓
Release
```

أي Requirement حرج يجب أن يمكن تتبعه حتى اختبار وEvidence.

------------------------------------------------------------------------

# 31. Definition of Done

لا تعتبر المهمة مكتملة إلا إذا:

-   تم تنفيذها.
-   تم اختبارها.
-   تم اختبار حالات الفشل.
-   تم فحص الأمن عند الحاجة.
-   تم تحديث التوثيق.
-   تم تحديث المواصفة عند الحاجة.
-   تم تسجيل dependencies.
-   تم توفير evidence.
-   تمت مراجعة التغيير.
-   لا يوجد blocker مفتوح متعلق بها.

------------------------------------------------------------------------

# 32. Stop-the-Line

يتم إيقاف المسار عند:

-   Crypto flaw.
-   Authentication bypass.
-   Authorization bypass.
-   Key leakage.
-   Plaintext leakage.
-   Identity ambiguity.
-   Trust bypass.
-   Downgrade vulnerability.
-   Critical dependency vulnerability.
-   Build compromise.
-   Supply-chain compromise.
-   Data deletion/recovery bypass.
-   Critical protocol ambiguity.

المسار:

``` text
STOP
↓
CONTAIN
↓
INVESTIGATE
↓
FIX
↓
REGRESSION TEST
↓
INDEPENDENT VERIFY
↓
DOCUMENT
↓
RESUME
```

------------------------------------------------------------------------

# 33. الملفات المركزية المقترحة

``` text
/docs
  /architecture
  /protocol
  /security
  /threat-model
  /testing
  /release
  /platforms
  /operations

/spec
  protocol.md
  state-machines.md
  message-format.md
  errors.md

/adr
  ADR-0001-...
  ADR-0002-...

/requirements
  requirements.md
  traceability.md

/tests
  /unit
  /integration
  /system
  /protocol
  /security
  /fuzz
  /interop

/security
  threat-model.md
  security-policy.md
  findings.md

/supply-chain
  SBOM
  dependencies
  provenance
  licenses

/research
  sources.md
  papers.md
  references.md

/release
  release-checklist.md
  release-evidence.md
```

------------------------------------------------------------------------

# 34. قاعدة المصادر والمكتبات

لا تتم إضافة مكتبة بسبب: - الشهرة فقط. - سهولة الاستخدام فقط. - عدد
النجوم فقط. - توصية غير موثقة.

يجب تقييم: - الأمان. - النضج. - الصيانة. - الترخيص. - تاريخ الثغرات. -
الاعتماديات. - جودة الاختبارات. - provenance. - قابلية التحديث. - ملاءمة
المشروع.

------------------------------------------------------------------------

# 35. ما يجب الاحتفاظ به قبل حذف المحادثات القديمة

قبل الحذف، يجب أن يكون ما يلي موجودًا في المرجع المركزي:

1.  كل القرارات النهائية.
2.  كل المواصفات.
3.  كل Threat Models.
4.  كل المتطلبات.
5.  كل Security Findings.
6.  كل نتائج الاختبارات.
7.  كل المصادر.
8.  كل المكتبات المقترحة والمستخدمة.
9.  كل ADRs.
10. كل مخططات المعمارية.
11. كل متطلبات Android.
12. كل متطلبات Apple.
13. كل تفاصيل PrivateMesh.
14. كل تفاصيل Network.
15. كل تفاصيل Storage/Recovery/Deletion.
16. كل تفاصيل AI.
17. كل تفاصيل MCP/Tools.
18. كل Supply Chain requirements.
19. كل Release gates.
20. كل Known limitations.
21. كل Future/V2/V3 items.

------------------------------------------------------------------------

# 36. ما لا يجب اعتباره محفوظًا بمجرد حذف المحادثات

لا يجب الاعتماد على: - نقاش غير موثق. - اقتراح لم يتحول إلى قرار. - كود
لم يتم حفظه في repository. - مصدر لم يسجل. - اختبار لم تحفظ نتيجته. -
قرار لم يتحول إلى ADR. - مواصفة موجودة فقط داخل رسالة. - Finding لم يسجل
في Security tracker. - مكتبة لم تسجل في dependency inventory.

------------------------------------------------------------------------

# 37. الأولويات

## P0 --- Critical

أمن النواة، الهوية، المفاتيح، التشفير، trust boundaries، authentication،
authorization.

## P1 --- High

Protocol، storage، network، recovery، platform security، testing.

## P2 --- Medium

UX، performance، tooling، observability، developer experience.

## P3 --- Future

V2/V3، تجارب، تحسينات غير ضرورية للإصدار الأساسي.

------------------------------------------------------------------------

# 38. ترتيب التنفيذ المقترح

``` text
00 Governance
↓
20 Sources
↓
Requirements
↓
03 Architecture
↓
05 Security Core
↓
06 Threat Model
↓
10 Specification
↓
01/04 Implementation
↓
13/14 Platform
↓
15 Network
↓
16 Storage
↓
17 AI
↓
18 MCP
↓
19 Supply Chain
↓
09 QA
↓
07 Security Testing
↓
08 Release
↓
11 Documentation
↓
12 Final Documentation
↓
21 Future
```

------------------------------------------------------------------------

# 39. قواعد التنسيق بين المحادثات

كل محادثة يجب أن تبدأ عند الحاجة بـ:

``` text
Project: Eagle
Section: <ID>
Status: <status>
Current Version: <version>
Dependencies: <IDs>
Inputs: <IDs>
Outputs: <IDs>
Open Issues: <IDs>
```

وعند إنهاء موضوع:

``` text
Decision:
Evidence:
Implementation:
Tests:
Verification:
Documentation:
Next Owner/Section:
```

------------------------------------------------------------------------

# 40. قاعدة عدم التكرار

إذا ظهر نفس الموضوع في أكثر من محادثة:

-   المكان الأصلي يملك التفاصيل.
-   المحادثات الأخرى تضع reference فقط.
-   لا تنسخ المواصفة في عدة أماكن.
-   لا تنشئ قرارين متعارضين.
-   عند التعارض يعود القرار إلى 00.
-   عند تعارض مصدرين يسجل الخلاف بدل إخفائه.

------------------------------------------------------------------------

# 41. حالات المشروع

``` text
IDEA
RESEARCH
DESIGN
SPECIFIED
IMPLEMENTATION
TESTING
SECURITY_REVIEW
RELEASE_CANDIDATE
RELEASED
MAINTENANCE
DEPRECATED
```

------------------------------------------------------------------------

# 42. الحالة الحالية بعد إعادة التنظيم

**حالة تنظيمية:** RESTRUCTURING / MASTERIZATION

الهدف التالي: 1. تثبيت المرجع الرئيسي. 2. نقل القرارات. 3. تثبيت
المواصفات. 4. تثبيت الاختبارات. 5. تثبيت المصادر. 6. تثبيت dependency
inventory. 7. تحديد ما هو V1 وما هو V2/V3. 8. بعد ذلك يمكن حذف المحادثات
القديمة بعد التأكد من عدم وجود معلومات وحيدة المصدر داخلها.

------------------------------------------------------------------------

# 43. قاعدة الحذف الآمن للمحادثات

لا يُنصح بحذف أي محادثة قديمة إلا بعد إكمال:

``` text
Conversation Audit
↓
Important Data Extraction
↓
Decision Extraction
↓
Source Extraction
↓
Specification Extraction
↓
Test Extraction
↓
Security Finding Extraction
↓
Master Reference Update
↓
Cross-check
↓
Archive/Export
↓
Delete
```

**الهدف:** حذف المحادثات يصبح حذفًا للواجهة التاريخية فقط، وليس حذفًا
لمعلومات المشروع.

------------------------------------------------------------------------

# 44. المرجع النهائي

المشروع يجب أن ينتهي إلى مجموعة قليلة من المصادر الأساسية:

``` text
MASTER_REFERENCE
     │
     ├── GOVERNANCE
     ├── REQUIREMENTS
     ├── ADR
     ├── ARCHITECTURE
     ├── SPECIFICATION
     ├── SECURITY
     ├── TESTING
     ├── SUPPLY_CHAIN
     ├── RELEASE
     ├── DOCUMENTATION
     └── RESEARCH/SOURCES
```

والمحادثات هي واجهة عمل لهذه المصادر وليست بديلًا عنها.

------------------------------------------------------------------------

# 45. المبدأ النهائي

> **النسر لا يعتمد على كثرة الأدوات أو كثرة المحادثات؛ يعتمد على وضوح
> الحدود، وقابلية التحقق، وقوة الأمن، وإمكانية تتبع كل قرار إلى مصدر
> ودليل واختبار.**
