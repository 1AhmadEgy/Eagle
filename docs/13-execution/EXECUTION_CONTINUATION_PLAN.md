# خطة الاستكمال والتنفيذ — Eagle

## 1. الهدف

تحويل مشروع النسر من مرحلة تأسيس المرجع إلى مرحلة تنفيذ قابلة للتدقيق، مع عدم افتراض أي متطلب أو مكوّن غير مثبت.

## 2. حالة الأدلة الحالية

| المجال | الحالة | الدليل/الإجراء التالي |
|---|---|---|
| GitHub repository | Verified | 1AhmadEgy/Eagle + audit branch |
| Android application bootstrap | Verified | app/ وMainActivity.kt وapp/build.gradle.kts |
| Android build stack | Verified | AGP 9.4.0 + Gradle 9.6.0 + JDK 17 + API 37 compatibility |
| Product requirements | Pending | استرجاع المواد التاريخية واعتماد V1 |
| Production architecture | Pending | تثبيت contracts/trust boundaries بعد requirements |
| Threat model | Pending | بناء النموذج على architecture الفعلية |
| CI configuration | Verified | ci.yml + testlab.yml موجودان |
| CI live evidence | Pending | ربط runs بالـSHA وحفظ artifact evidence |
| Current QA | Baseline only | Smoke test حالي لاختبار التنفيذ وليس السلوك المنتجِي |
| External component due diligence | In Progress | المسح الأول منشور؛ exact adoption لم يعتمد |
| Dependency/SBOM baseline | Pending | بعد اعتماد production stack |
| Operations/recovery | Pending | عند تثبيت deployment/data requirements |
| Filebin archive intake | Pending | retrieval/extraction unavailable in current path |

## 3. ترتيب التنفيذ الإلزامي

1. جمع كل المواد التاريخية المتاحة.
2. فحص الأرشيفات قبل أي import إلى Git.
3. استخراج المتطلبات والقرارات والتعارضات.
4. تثبيت Android baseline الحالي كحقيقة قابلة لإعادة البناء.
5. اعتماد Stack الإنتاجي على أساس المتطلبات، لا على أساس الرغبة التقنية.
6. تثبيت Architecture وTrust Boundaries.
7. بناء Threat Model.
8. اعتماد المكونات الخارجية واحدًا واحدًا.
9. إنشاء استراتيجية الاختبارات والعقود.
10. إنشاء CI security gates وربطها بالأدلة.
11. إنشاء SBOM/provenance/release evidence.
12. تنفيذ أول end-to-end secure slice.
13. توسيع السلوك الوظيفي إلى التخزين والنقل ثم Mesh.
14. مراجعة بشرية ثم merge فقط بعد استيفاء البوابات.

## 4. قواعد عدم التخمين

- لا يتم اعتماد framework لمجرد أنه شائع.
- لا يتم نسخ مشروع مرجعي كامل.
- لا يتم استخدام latest أو floating versions.
- لا يتم إدخال أسرار أو بيانات demo الحساسة.
- لا يتم إغلاق فجوة دون دليل.
- لا يتم اعتبار ملف تاريخي أو محادثة مراجعًا إلا بعد الوصول إليها وفحصها.
- لا يتم اعتبار workflow configured مساويًا لـ workflow passed.
- لا يتم اعتبار encryption-at-rest مساويًا لـ E2EE.

## 5. بوابة المواد التاريخية

قبل إدخال أي archive/document:

- حساب SHA-256 عند الإمكان.
- تحديد المصدر والتاريخ والمالك/السياق.
- فحص secrets/API keys/tokens/passwords/private certificates.
- فحص PII غير اللازمة.
- استخراج manifest.
- duplicate detection.
- تسجيل provenance.
- تصنيف: Verified / Derived / Proposed / Pending / Rejected.
- عدم استبدال الملفات canonical دون سجل للتغييرات.

## 6. بوابة Stack

يجب تسجيل:

- language
- framework
- runtime
- package manager
- exact versions
- lockfiles
- database
- cache/queue إن وجدت
- deployment target
- authentication/authorization
- AI/agent components إن وجدت
- observability
- testing framework

كل dependency تمر عبر:

Requirement → Architecture Fit → Security History → Exact Version → License → Dependency Review → Tests → Operations → Exit Strategy → Human Approval

أول نتائج المسح الخارجي محفوظة في:

docs/14-audit/COMPONENT_DUE_DILIGENCE_2026-10-04.md

## 7. بوابة الأمن

الحد الأدنى المقترح، حسب الـStack الفعلي:

- Secret scanning
- SAST مناسب للـstack
- Dependency/SCA scanning
- SBOM
- Container/IaC scanning عند الحاجة
- DAST في بيئة اختبار عند وجود سطح ويب مناسب
- AuthN/AuthZ tests
- Input/output validation
- Audit logging بدون أسرار
- Backup/recovery verification
- Artifact provenance/attestation حيثما أمكن
- Least-privilege CI permissions

## 8. Definition of Ready للميزة

لا تبدأ ميزة تنفيذية قبل وجود:

- Requirement ID
- acceptance criteria
- architecture impact
- security impact
- owner
- test plan
- rollback consideration

## 9. Definition of Done

الميزة لا تعتبر مكتملة إلا إذا:

- الكود موجود.
- الاختبارات المطلوبة ناجحة.
- الفحوص الأمنية المناسبة ناجحة.
- dependency/license impact موثق.
- الوثائق محدثة.
- PR تمت مراجعته.
- evidence قابل لإعادة التنفيذ محفوظ.

## 10. القرار الحالي

المشروع في مرحلة Foundation → Evidence Collection → Stack Discovery.

لا يُسمح بتحويل الحالة إلى Production Ready قبل إغلاق البوابات أعلاه بالأدلة.

## 11. الخطوة التشغيلية التالية

الخطوة التالية ليست إضافة مكتبات عشوائية؛ بل تحويل المسح الخارجي إلى component contracts ثم اختيار dependency محددة فقط عندما يثبت Requirement استخدامها.

الهدف التنفيذي التالي:

Device A → Identity → Session/Key Establishment → Encrypt → Canonical Envelope → Transport → Device B → Verify → Decrypt → Persist → Test Evidence

ثم توسيع ذلك تدريجيًا إلى discovery/routing/Mesh بعد إثبات الأساس الآمن.
