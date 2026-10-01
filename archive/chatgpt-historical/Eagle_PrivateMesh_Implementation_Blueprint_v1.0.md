# Eagle / PrivateMesh — Implementation Blueprint
## الإصدار 1.0

**الغرض:** تحويل قرارات ADR-001..ADR-008 إلى بنية تنفيذ برمجية قابلة للتقسيم إلى مهام، واختبارات، وبوابات اعتماد.

> هذا المستند هو مخطط تنفيذ. لا يعني أن المكونات المذكورة موجودة أو مكتملة في المستودع الحالي. أي بند يجب وسمه لاحقًا بـ `Implemented` أو `Partial` أو `Missing` بعد فحص الكود الفعلي.

---

# 1. نطاق V1

## 1.1 ما يدخل V1

- حساب مستخدم وهوية منطقية للحساب.
- جهاز واحد على الأقل لكل حساب.
- إدارة عدة أجهزة للحساب.
- محادثات فردية.
- جلسات تشفير طرفي.
- إرسال واستقبال مع انقطاع الشبكة.
- طابور إرسال وإعادة محاولة.
- تخزين محلي مشفر.
- WebSocket/TLS للنقل الأساسي.
- صندوق تسليم خادم محدود المدة.
- QR لربط جهاز جديد.
- إبطال جهاز وإدارة حالة الثقة.
- استرداد حساب منفصل عن استرداد البيانات المشفرة.
- حذف متعدد الطبقات.
- سجلات تشغيلية لا تحتوي على محتوى الرسائل أو الأسرار.

## 1.2 خارج V1

- بروتوكول تشفير مصمم داخليًا.
- P2P كشرط للإطلاق.
- مجموعات محادثة قبل اعتماد وتنفيذ بروتوكول مجموعات مناسب.
- استعادة سرية تلقائية من الخادم.
- قدرة المشغل على فك محتوى المستخدم.
- ادعاء محو فوري مطلق من جميع النسخ الاحتياطية.
- تحليلات محتوى الرسائل.

---

# 2. البنية العامة

```text
┌─────────────────────────────────────────────────────────────┐
│                     Client Applications                     │
│  Mobile / Desktop / Web                                    │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                  Platform Adapter Layer                    │
│ UI / OS Key Store / Secure Storage / Notifications         │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                     Shared Core (Rust)                    │
│ Identity / Device / Crypto / Sessions / Messages / Sync    │
└──────────────┬───────────────────┬──────────────────────────┘
               │                   │
               ▼                   ▼
      Local Encrypted DB       Transport Adapter
                                   │
                                   ▼
                              TLS / WebSocket
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────┐
│                        Server                              │
│ Auth / Device Registry / Delivery Queue / TTL / Presence  │
└─────────────────────────────────────────────────────────────┘
```

قاعدة معمارية: **الـ Core يملك منطق الحالة والأمان؛ المنصات تملك التكامل مع النظام؛ الخادم ينفذ النقل والتسليم ولا يحتاج إلى أسرار فك التشفير.**

---

# 3. بنية المستودع المقترحة

```text
eagle/
├── apps/
│   ├── mobile/
│   ├── desktop/
│   └── web/
│
├── crates/
│   ├── core/
│   ├── crypto/
│   ├── identity/
│   ├── devices/
│   ├── sessions/
│   ├── messaging/
│   ├── sync/
│   ├── transport/
│   ├── recovery/
│   └── protocol/
│
├── server/
│   ├── api/
│   ├── auth/
│   ├── devices/
│   ├── delivery/
│   ├── presence/
│   ├── storage/
│   └── jobs/
│
├── db/
│   ├── migrations/
│   └── fixtures/
│
├── tests/
│   ├── protocol/
│   ├── crypto/
│   ├── integration/
│   ├── resilience/
│   ├── security/
│   └── e2e/
│
├── docs/
│   ├── adr/
│   ├── threat-model/
│   ├── protocols/
│   └── runbooks/
│
└── tooling/
    ├── fuzz/
    ├── test-fixtures/
    └── release/
```

الأسماء يمكن تغييرها بحسب المستودع الموجود؛ المهم الحفاظ على فصل المسؤوليات.

---

# 4. مسؤوليات الوحدات

## 4.1 `core`

واجهة عالية المستوى للتطبيق.

مسؤوليات:
- تنسيق الهوية والأجهزة.
- دورة حياة جلسة التطبيق.
- حالات المزامنة.
- تنسيق الرسائل.
- فرض invariants الأمنية.

لا يحتوي:
- كود UI.
- SQL خام منتشر في كل مكان.
- تفاصيل منصة واحدة.

## 4.2 `crypto`

مسؤول فقط عن تكامل بروتوكول التشفير المعتمد.

مسؤوليات:
- إنشاء/تحميل المفاتيح.
- إنشاء الجلسة.
- تشفير/فك الرسالة.
- تدوير المواد المطلوبة حسب البروتوكول.
- التعامل مع أخطاء البروتوكول.

ممنوع:
- ابتكار primitive جديد.
- تخزين أسرار في logs.
- تسليم مفاتيح خاصة إلى طبقة النقل.

## 4.3 `identity`

مسؤول عن:
- هوية الحساب.
- هوية الجهاز.
- الهوية طويلة الأجل.
- بصمات التحقق.
- تغير حالة الهوية.

## 4.4 `devices`

مسؤول عن:
- تسجيل الجهاز.
- ثقة الجهاز.
- QR pairing.
- الإبطال والاستبدال.
- قائمة الأجهزة.

الحالات المبدئية:

```text
Pending -> Trusted -> Revoked
              \
               -> Replaced
```

## 4.5 `sessions`

مسؤول عن:
- إنشاء الجلسة.
- حفظ حالتها.
- إعادة البناء بعد restart.
- فشل الجلسة وإعادة التفاوض وفق البروتوكول.

## 4.6 `messaging`

مسؤول عن النموذج المنطقي للرسالة:

```text
MessageEnvelope
├── message_id
├── conversation_id
├── sender_device_id
├── recipient_device_id / route
├── ciphertext
├── protocol_version
├── created_at
└── delivery_state
```

لا يحتوي الـ envelope على النص الصريح.

## 4.7 `sync`

مسؤول عن:
- مزامنة state.
- إعادة محاولة الرسائل.
- معالجة التكرار.
- ترتيب الأحداث.
- reconciliation بعد الانقطاع.

## 4.8 `transport`

واجهة مجردة:

```text
Transport
├── connect()
├── send()
├── receive()
├── ack()
├── disconnect()
└── state()
```

النسخة الأولى:
`WebSocket + TLS`

مع إمكانية إضافة transport آخر لاحقًا دون تغيير نموذج الرسالة المشفرة.

## 4.9 `recovery`

مسؤول عن:
- استعادة الحساب.
- استرداد البيانات إذا أُعد مسبقًا.
- إدارة recovery material.
- فصل account recovery عن data recovery.

---

# 5. نموذج بيانات العميل

## 5.1 `account`

```text
account_id
status
created_at
updated_at
```

## 5.2 `device`

```text
device_id
account_id
device_name
platform
trust_state
created_at
last_seen_at
revoked_at
replaced_by
```

## 5.3 `identity_key`

```text
device_id
key_version
public_identity_key
fingerprint
created_at
status
```

المفتاح الخاص لا يخزن كنص صريح في قاعدة بيانات عادية؛ يستخدم مخزن مفاتيح النظام أو طبقة حماية مناسبة للمنصة.

## 5.4 `conversation`

```text
conversation_id
type
created_at
updated_at
```

## 5.5 `message`

```text
message_id
conversation_id
sender_device_id
ciphertext
protocol_version
created_at
local_state
server_delivery_state
expires_at
```

## 5.6 `sync_cursor`

```text
account_id
device_id
cursor
updated_at
```

---

# 6. نموذج بيانات الخادم

## 6.1 `accounts`

```text
id
status
created_at
updated_at
```

## 6.2 `devices`

```text
id
account_id
public_identity
status
registered_at
revoked_at
last_seen_at
```

## 6.3 `delivery_messages`

```text
id
recipient_device_id
ciphertext
protocol_version
created_at
expires_at
delivery_status
delivery_attempts
```

قيد أساسي: لا يوجد `plaintext`.

## 6.4 `pairing_requests`

```text
id
source_device_id
target_device_nonce
expires_at
used_at
status
```

## 6.5 `audit_events`

يخزن أحداثًا تشغيلية وأمنية غير حساسة:

```text
event_id
account_id
device_id
event_type
created_at
metadata_safe
```

يمنع:
- ciphertext الكامل.
- tokens.
- private keys.
- recovery secrets.
- نص الرسائل.

---

# 7. واجهات داخلية أساسية

## Identity

```text
create_account_identity()
load_account_identity()
get_identity_fingerprint()
rotate_identity_if_required()
```

## Device

```text
register_device()
request_pairing()
approve_pairing()
revoke_device()
list_devices()
```

## Session

```text
start_session()
encrypt_message()
decrypt_message()
process_session_event()
```

## Messaging

```text
create_message()
queue_outgoing()
mark_sent()
mark_delivered()
mark_failed()
```

## Sync

```text
sync()
resume_after_disconnect()
process_incoming_batch()
deduplicate()
```

## Recovery

```text
enable_recovery()
export_recovery_material()
restore_account_access()
restore_encrypted_data()
```

---

# 8. تدفق إرسال رسالة

```text
User
  │
  ▼
UI creates send request
  │
  ▼
core::messaging
  │
  ▼
session resolves recipient session
  │
  ▼
crypto.encrypt()
  │
  ▼
message stored locally as queued
  │
  ▼
sync queue
  │
  ▼
transport.send(ciphertext)
  │
  ▼
server delivery queue
  │
  ├── recipient online ──> deliver
  │
  └── offline ──> retain until ACK or TTL
```

قواعد:
- التخزين المحلي قبل الإرسال يمنع فقد الرسالة عند انقطاع الاتصال أثناء الإرسال.
- إعادة الإرسال تستخدم `message_id` ثابتًا.
- الخادم لا يحتاج النص الصريح.

---

# 9. تدفق الاستلام

```text
Server
  │
  ▼
ciphertext
  │
  ▼
transport
  │
  ▼
sync
  │
  ▼
deduplicate(message_id)
  │
  ▼
session lookup
  │
  ▼
crypto.decrypt()
  │
  ▼
local encrypted DB
  │
  ▼
UI
```

في حالة فشل فك التشفير:
- لا يتم إظهار plaintext مخمّن.
- يسجل الخطأ كرمز تشخيصي.
- يحاول core إصلاح حالة الجلسة وفق البروتوكول.
- يحتفظ الرسالة وفق سياسة المنتج إلى أن تنجح المعالجة أو تنتهي.

---

# 10. العمل دون اتصال

يجب دعم الحالات التالية:

### Offline قبل الإرسال
الرسالة تبقى `Queued`.

### Offline بعد التشفير وقبل ACK
الرسالة تبقى `Pending` وتُعاد بأمان عند reconnect.

### إعادة نفس الرسالة
يستخدم `message_id` لمنع duplicate processing.

### Restart للتطبيق
يُستعاد queue وsession state من التخزين المحلي المحمي.

### انقطاع متكرر
استخدام retry مع backoff وحد أعلى للمحاولات المتزامنة.

---

# 11. ربط جهاز جديد

```text
Trusted Device
      │
      ├── create pairing request
      │
      ▼
 One-time QR
      │
      ▼
New Device scans
      │
      ▼
Temporary pairing channel
      │
      ▼
Mutual confirmation
      │
      ▼
Create/register new device identity
      │
      ▼
Trusted Device approves
      │
      ▼
New Device = Trusted
```

اختبارات إلزامية:
- QR منتهي.
- QR مستخدم سابقًا.
- QR معدل.
- محاولة pairing دون موافقة.
- جهاز جديد يحاول تجاوز حالة `Pending`.
- إلغاء pairing.
- إبطال الجهاز بعد إضافته.

---

# 12. إدارة الثقة

## القاعدة

الثقة لا تساوي مجرد تسجيل الدخول.

عند:
- مفتاح جديد.
- جهاز جديد.
- إعادة تثبيت.
- استبدال جهاز.

يجب أن يظهر event أمني مناسب.

مثال:

```text
Trusted
   │
   ├── key-change detected
   ▼
Review Required
   │
   ├── verified
   ▼
Trusted
   │
   └── rejected
       ▼
Blocked / Revoked
```

يجب أن تكون هذه الانتقالات محددة في state machine وليس في UI فقط.

---

# 13. التخزين المحلي

## مبدأ

كل البيانات الحساسة محليًا تعامل على أنها غير موثوقة بالنسبة لأي مكون خارج secure storage boundary.

طبقات الحماية:

```text
App
 ↓
Encrypted Database
 ↓
Database Key
 ↓
OS Secure Key Store
```

اختبارات:
- صلاحيات الملفات.
- نسخ قاعدة البيانات.
- export/import.
- backup behavior.
- crash أثناء commit.
- rollback/corruption.
- عدم ظهور secrets في dump أو logs.

---

# 14. الخادم

## الخدمات الدنيا

### Auth
مصادقة الحساب/الجلسة.

### Device Registry
تسجيل الأجهزة وإبطالها.

### Delivery
استلام وتسليم ciphertext.

### Presence
إشارات online/offline التي يحتاجها المنتج.

### TTL Worker
حذف عناصر delivery المنتهية.

### Observability
metrics وhealth checks دون محتوى.

---

# 15. حدود الخادم الأمنية

الخادم **لا يفترض أن يمتلك**:

- private identity keys.
- session secrets.
- plaintext message content.
- recovery keys القابلة للاستخدام.
- كلمات مرور خام.

الخادم **قد يمتلك** حسب التصميم:

- account identifiers.
- device identifiers.
- routing metadata.
- timestamps.
- message ciphertext.
- delivery status.
- limited operational telemetry.

يجب توثيق هذه البيانات صراحة في Threat Model.

---

# 16. واجهات API

صيغة تقريبية وليست عقدًا نهائيًا.

```text
POST   /v1/auth/session
GET    /v1/devices
POST   /v1/devices
POST   /v1/devices/{id}/revoke

POST   /v1/pairing
POST   /v1/pairing/{id}/approve
DELETE /v1/pairing/{id}

POST   /v1/messages
GET    /v1/messages/queue
POST   /v1/messages/{id}/ack

GET    /v1/sync?cursor=...
GET    /v1/health
GET    /v1/version
```

القواعد:
- كل endpoint يملك authentication وauthorization واضحين.
- حجم payload محدود.
- rate limiting.
- idempotency حيث يلزم.
- structured errors دون secrets.

---

# 17. إصدار البروتوكول

كل رسالة مشفرة تحمل نسخة منطقية:

```text
protocol_version = 1
```

سياسة الترقية:

```text
Current Version
      │
      ├── compatible ──> accept
      │
      └── incompatible
              │
              ├── supported migration
              │
              └── reject safely
```

يمنع:
- downgrade صامت.
- قبول format غير معروف باعتباره plaintext.
- migration غير قابلة للاختبار.

---

# 18. الاختبارات

## 18.1 Unit

- state transitions.
- message IDs.
- TTL calculations.
- retry/backoff.
- validation.
- serialization/deserialization.

## 18.2 Crypto/Protocol

- session establishment.
- encrypt/decrypt.
- key changes.
- duplicate messages.
- out-of-order messages.
- restart.
- device replacement.
- downgrade resistance.

## 18.3 Integration

- client ↔ server.
- multiple devices.
- offline/online.
- pairing.
- revocation.
- recovery.

## 18.4 Security

- authentication bypass.
- authorization bypass.
- replay.
- malformed messages.
- oversized payloads.
- rate-limit bypass.
- log leakage.
- secret exposure.

## 18.5 Fuzzing

الأولوية:
- protocol parsing.
- serialized envelopes.
- sync payloads.
- pairing payloads.
- state-machine transitions.

## 18.6 End-to-End

السيناريو الأدنى:

```text
Account A
 ├── Device A1
 └── Device A2

Account B
 └── Device B1

A1 -> B1 send/receive
A1 offline
B1 sends
A1 reconnects
A1 decrypts

A2 pairs
A2 receives permitted future traffic
A1 revokes A2
A2 stops receiving new traffic
```

---

# 19. اختبارات القبول الأمنية

لا يعتبر V1 جاهزًا إذا فشل أي مما يلي:

- plaintext يصل إلى الخادم.
- private key يصل إلى logs.
- جهاز `Revoked` يستقبل رسائل جديدة بعد الإبطال بسبب خلل في authorization.
- QR pairing قابل لإعادة الاستخدام.
- message replay يسبب duplicate processing غير مسيطر عليه.
- حذف الرسائل يتركها بلا TTL بسبب job فاشل وغير مكتشف.
- recovery account يُفهم منه خطأ أنه recovery للرسائل.
- downgrade غير مصرح به.
- تغيير مفتاح أمني لا يولد الحدث/التنبيه المحدد.
- crash يؤدي إلى فقد queue بطريقة تكسر guarantees المعلنة.

---

# 20. Observability

المسموح:

```text
connected_clients
messages_queued
messages_delivered
messages_expired
sync_failures
pairing_failures
auth_failures
crypto_error_codes
latency
```

الممنوع:

```text
plaintext_message
private_key
session_secret
recovery_secret
authorization_token
full_sensitive_payload
```

يفضل استخدام identifiers مشتقة/مجهلة حيث يكون ذلك كافيًا.

---

# 21. الإطلاق التدريجي

```text
Phase 0
Repo + dependency lock + threat model

Phase 1
Identity + device + local storage

Phase 2
Crypto/session integration

Phase 3
Messaging + local queue

Phase 4
WebSocket/TLS + server delivery

Phase 5
Offline/sync/retry

Phase 6
Pairing + device trust

Phase 7
Recovery + deletion

Phase 8
Security testing + external review

Phase 9
Controlled rollout
```

---

# 22. Definition of Done

## Feature

الميزة لا تعتبر منجزة إلا إذا توفر:

1. كود.
2. اختبار آلي.
3. معالجة أخطاء.
4. logging آمن.
5. توثيق.
6. اختبار integration عند الحاجة.
7. مراجعة أمنية عندما تمس الثقة/التشفير/الهوية.

## Security-sensitive feature

يضاف:

1. Threat scenario.
2. Negative tests.
3. State-machine tests.
4. Failure-mode analysis.
5. Evidence artifact.

---

# 23. ترتيب التنفيذ العملي

الأولوية المقترحة:

### P0
- repository structure.
- dependency lock.
- threat model.
- secure storage abstraction.
- identity/device model.
- crypto integration boundary.

### P1
- session management.
- encrypted local DB.
- message queue.
- server delivery.
- WebSocket/TLS.
- offline synchronization.

### P2
- pairing.
- trust state machine.
- device revocation.
- recovery.

### P3
- deletion controls.
- telemetry hardening.
- fuzzing.
- external security review.

---

# 24. مصفوفة التتبع ADR → تنفيذ

| ADR | مكونات التنفيذ | الاختبار الأساسي | بوابة |
|---|---|---|---|
| ADR-001 | `crypto`, `sessions`, `protocol` | protocol/session tests | G3/G4 |
| ADR-002 | `crypto`, `protocol` | PQ/interoperability/downgrade | G3/G4 |
| ADR-003 | `transport`, `sync`, `messaging` | offline/retry/duplicate | G3/G4 |
| ADR-004 | `server/delivery`, `server/jobs` | TTL/ACK/expiry | G4/G6 |
| ADR-005 | `identity`, `devices` | state transitions/revocation | G4 |
| ADR-006 | `devices`, `pairing` | QR replay/expiry/intercept | G4 |
| ADR-007 | `recovery` | recovery/loss scenarios | G4/G5 |
| ADR-008 | `messaging`, `server/jobs`, storage | deletion/retention | G4/G6 |

---

# 25. أول Sprint تقني

## Sprint Goal

إثبات أن النواة قادرة على إنشاء هوية جهاز، حفظها بأمان، إنشاء جلسة تشفير، وتخزين/إرسال رسالة مشفرة دون اعتماد UI.

## المخرجات

```text
[ ] workspace structure
[ ] dependency lock
[ ] core crate
[ ] crypto abstraction
[ ] identity model
[ ] device model
[ ] encrypted local storage abstraction
[ ] protocol test harness
[ ] test fixtures
[ ] threat model draft
[ ] secure logging baseline
```

## شرط نهاية Sprint

سيناريو آلي واحد على الأقل:

```text
Create Account
    ↓
Create Device
    ↓
Create Session
    ↓
Encrypt Message
    ↓
Persist Locally
    ↓
Serialize Envelope
    ↓
Decrypt in Test Peer
    ↓
Assert Original Payload
```

---

# 26. مؤشرات المخاطر التي يجب مراقبتها

| الخطر | إشارة مبكرة |
|---|---|
| تسرب مسؤوليات الأمن إلى UI | شروط الثقة مكتوبة داخل صفحات الواجهة |
| بروتوكول مخصص | إضافة primitives/handshake خاص |
| تسرب plaintext | logging أو analytics من طبقة transport |
| اعتماد زائد على الخادم | server يحتاج secret لفك البيانات |
| فقد offline state | queue غير مستمر بعد restart |
| pairing ضعيف | QR طويل العمر أو قابل لإعادة الاستخدام |
| recovery مضلل | account recovery يعيد الرسائل تلقائيًا |
| deletion مبالغ فيه | ادعاء wipe مطلق دون دليل تشغيلي |

---

# 27. قاعدة مهمة للتنفيذ

كلما ظهر تعارض بين:

- سهولة التنفيذ،
- أو سرعة الإنجاز،
- أو ضمان أمني موثق،

لا يتم حل التعارض بإضافة كود عشوائي. يجب:
1. تحديد الـ invariant الأمني.
2. تحديث ADR عند الحاجة.
3. كتابة اختبار يفشل قبل الإصلاح وينجح بعده.
4. تسجيل الدليل.
5. إعادة تقييم بوابة G المناسبة.

---

# 28. حالة المستند

هذا المخطط جاهز ليكون baseline هندسي، لكنه لا يحدد بعد:

- أسماء الملفات الحقيقية في المستودع الحالي.
- الخدمات الموجودة فعليًا.
- المكونات الناقصة في الكود الحالي.
- الاعتماديات والإصدارات الفعلية.
- نتائج الاختبارات الحالية.

هذه البنود لا يجوز اختلاقها؛ يجب استخراجها من المستودع/الأرشيف الفعلي في مرحلة **Implementation Audit**.
