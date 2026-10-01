# البحث التفصيلي — مصفوفة تدقيق المكونات المرجعية

## 1. الغرض
هذه المصفوفة تحول البحث العام إلى بوابة اعتماد قابلة للتدقيق. وجود المكون في الجدول لا يعني اعتماده في Eagle.

## 2. قواعد الاعتماد
أي مكون Production يجب أن يمر بالترتيب:
`Requirement → Architecture Fit → Security History → Exact Version → License → Dependency Review → Test Evidence → Operational Fit → Exit Strategy → Owner Approval`

ولا يسمح بـ:
- `latest` في الإنتاج أو CI.
- GitHub Actions غير مثبتة على commit SHA عندما يكون pinning متاحًا.
- صور حاويات غير مثبتة بوسم/هضم يمكن التحقق منه.
- أسرار داخل المستودع أو ملفات demo.
- إدخال مشروع كامل لمجرد أنه reference project.

## 3. المكونات التي تم التحقق منها في الجولة الحالية

| المكون | دليل الصيانة/الإصدار | دليل أمني | القرار الحالي |
|---|---|---|---|
| Keycloak | توثيق الإصدارات الرسمي يعرض سلسلة 26.x وتحديثات وترقيات واختبارات/توثيق للإصدارات | توجد إصدارات تتضمن إصلاحات أمنية متعددة؛ لذلك يجب تثبيت إصدار محدد ومراجعة release/security notes قبل الترقية | مرشح مشروط لـ IAM |
| OpenFGA | التوثيق يوضح متطلبات toolchain، APIs، PostgreSQL، OpenTelemetry، JWT/OIDC واختبارات/تشغيل | يجب فحص كل إصدار واعتماد نموذج authorization على متطلبات Eagle الفعلية | مرشح مشروط لـ Authorization |
| Trivy | أداة ناضجة للفحص، لكن حدثت حادثة supply-chain موثقة في 2026 | CVE-2026-33634 شملت إصدارًا ضارًا وGitHub Actions/صورًا متأثرة؛ pinning والتحقق من provenance أصبحا شرطًا صريحًا | أداة محتملة مع ضوابط مشددة |
| OWASP ZAP | مرجع DAST معروف ومستخدم للاختبار الديناميكي | لا يعتمد وحده لإثبات الأمان؛ يجب تشغيله في بيئة اختبار مع baseline واضح | مرشح لاختبارات DAST |
| Gitleaks | نمط secret scanning مناسب لبوابة CI | يجب التعامل معه كأداة كشف وليس بديلًا عن منع الأسرار وإدارة secrets | مرشح لـ CI |
| Syft/Grype | نمط SBOM + vulnerability scanning مفيد لسلسلة التوريد | يلزم تثبيت الإصدارات والتحقق من مصدر الأدوات ونتائجها | مرشح مشروط |
| OpenTelemetry | معيار/منظومة observability قابلة للاستخدام عبر الخدمات | يجب منع تسريب الأسرار والبيانات الحساسة في telemetry | مرشح لـ observability |

## 4. حادثة Trivy كاختبار لسياسة Eagle
تم التحقق من حادثة CVE-2026-33634 من مصادر أمنية عامة: في مارس 2026 استُخدمت بيانات اعتماد مخترقة لنشر Trivy v0.69.4 ضارًا، والتلاعب بعدة وسوم لإجراءات GitHub، إضافة إلى صور Docker ضارة v0.69.5 وv0.69.6. التوصيات المنشورة تضمنت استخدام النسخ الآمنة المحددة، فحص سجلات التنفيذ، وتدوير الأسرار إذا كان artifact المتأثر قد شُغّل.

**قاعدة Eagle الناتجة:**
1. لا يكفي اسم الأداة أو شهرتها.
2. كل أداة CI/security تعامل كـ supply-chain dependency.
3. GitHub Actions تثبت على SHA كامل.
4. artifacts والصور تتحقق من المصدر والتوقيع/provenance عندما يتوفر.
5. أي تعرض محتمل لأداة مخترقة يطلق incident-response وsecret rotation.
6. تسجل نسخة الأداة والـdigest/commit المستخدم ضمن Evidence.

## 5. ما يزال Pending
هذه العناصر تحتاج تثبيت Stack Eagle قبل اختيار إصدار نهائي:
- لغة Backend وframework.
- Frontend framework.
- نمط deployment: VM/container/Kubernetes/serverless.
- قاعدة البيانات النهائية.
- الحاجة الفعلية إلى AI/Agents/MCP.
- متطلبات الهوية متعددة المستأجرين إن وجدت.
- SLO/SLA والأحمال المتوقعة.
- متطلبات الامتثال ومكان تخزين البيانات.

## 6. بوابة اختيار الإصدار
عند معرفة الـstack، لكل dependency يجب إنشاء سجل:
`component → exact version → source → license → CVEs → release notes → integrity/provenance → transitive dependencies → tests → owner → approval date`

## 7. مخرجات التنفيذ
بعد اعتماد الـstack:
- إنشاء lockfiles أو ما يعادلها.
- SBOM لكل artifact قابل للنشر.
- dependency review في PR.
- secret scanning.
- SAST.
- container/IaC scanning عند الحاجة.
- DAST في بيئة الاختبار.
- artifact provenance/attestation.
- حفظ Evidence لكل بوابة.

## 8. الحالة
**Research / Component Due Diligence — Pending stack lock.**

هذه الوثيقة لا تمنح اعتمادًا تلقائيًا لأي مشروع أو إصدار؛ الاعتماد النهائي يجب أن يكون مرتبطًا بمتطلبات Eagle واختبارات قابلة لإعادة التنفيذ.
