# Master Requirements Baseline

> هذه الوثيقة تفصل متطلبات المنتج والسياسات عن الحالة الفعلية للتنفيذ.

## Product

| ID | المتطلب | Priority |
|---|---|---:|
| REQ-PROD-001 | Android | P0 |
| REQ-PROD-002 | iOS | P0 |
| REQ-PROD-003 | Desktop | P0 |
| REQ-PROD-004 | مراسلة بين أجهزة النظير | P0 |
| REQ-PROD-005 | لا يعتمد منطق الرسائل وقت التشغيل على خدمة طرف ثالث | P0 |
| REQ-PROD-006 | الأمن والخصوصية جزء من التصميم | P0 |
| REQ-PROD-007 | الأداء والاستقرار والكفاءة والحجم أهداف هندسية أساسية | P1 |

## Security

| ID | المتطلب |
|---|---|
| REQ-SEC-001 | هوية تشفيرية للجهاز قابلة للتحقق. |
| REQ-SEC-002 | حدود ثقة واضحة بين التطبيق والنواة والشبكة والتخزين. |
| REQ-SEC-003 | حماية المفاتيح لكل منصة عبر الآلية الآمنة المتاحة. |
| REQ-SEC-004 | لا plaintext أو secrets في logs/telemetry. |
| REQ-SEC-005 | fail-closed في الفشل الأمني. |
| REQ-SEC-006 | replay/freshness protections. |
| REQ-SEC-007 | bounds/validation لكل parser ومدخل شبكة. |
| REQ-SEC-008 | مراجعة مستقلة للتغييرات الأمنية الحرجة. |
| REQ-SEC-009 | provenance وSBOM/Dependency evidence لكل إصدار. |

## Data

- أقل قدر ممكن من البيانات.
- تخزين محلي محمي.
- سياسة حذف واستعادة قابلة للإثبات.
- لا message store مركزي في المسار الأساسي.
- recovery لا يتحول إلى تسريب للهوية أو المفاتيح.

## UX

- أخطاء واضحة لا تكشف أسرارًا.
- حالات offline/reconnect مفهومة.
- حالة تحقق الهوية واضحة.
- دعم الوصولية.

## Performance

لا توضع أرقام p95 أو throughput أو memory قبل وجود benchmark baseline قابل لإعادة التكرار.

## Traceability

كل P0:
Requirement → ADR/Spec → Component → Test → Evidence → Release

## Rejection rule

أي تصميم يتعارض مع P2P/no-third-party-runtime أو حدود الثقة أو security baseline يحتاج ADR صريح وrisk acceptance؛ لا يُقبل ضمنيًا.
