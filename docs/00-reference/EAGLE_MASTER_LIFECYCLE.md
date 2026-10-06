# Eagle Master Lifecycle — من الصفر إلى الإنتاج والاستقرار

## الدورة الرسمية

Inventory → Provenance → Classification → Triage → Analysis → Reconciliation → Conflicts → Gaps → Canonical Authority → Remediation → Correction → Implementation → Testing → Security Review → Verification → Evidence → Release Gate → Release → Post-Release → Recycle

## 1. Inventory — الجرد

نجرد كل ما يتعلق بالمشروع: ملفات المحادثات، Git، الفروع، PRs، Issues، ADRs، الاختبارات، CI، الأرشيف، والمكونات الخارجية المرشحة. المخرج هو سجل مصدر ثابت لكل عنصر.

## 2. Provenance — المصدر والنسخ

نسجل الاسم والمسار والتاريخ والفرع أو PR والcommit أو Git blob SHA، ومع الملفات القابلة للتنزيل نضيف SHA-256 مستقلًا.

مهم: Git blob SHA ليس SHA-256 للملف.

## 3. Classification — التصنيف

Canonical candidate، Supporting evidence، Historical، Duplicate، Superseded، Untrusted external input، Pending review، Secret/sensitive.

## 4. Triage — الفحص الأولي

نفحص اكتمال الملف والتكرار والأسرار وقابلية التنفيذ وعلاقته بالمتطلبات وحاجة الملف لمراجعة أمنية. لا يتم تشغيل archive أو script غير موثوق أثناء الجرد.

## 5. Analysis — التحليل العميق

نربط كل claim بالمتطلبات والمعمارية والأمن والاختبارات والكود.

## 6. Reconciliation — المطابقة

نستخدم السلسلة:

Source → Requirement → ADR/Spec → Component → Code → Test → Evidence

أي عنصر بلا مسار كامل يُسجل كفجوة أو قرار مطلوب.

## 7. Conflicts — التعارضات

لا نخفي اختلاف نسخ أو ادعاء تنفيذ غير مثبت. نسجل التعارض ونحدد القرار.

## 8. Gaps — الفجوات

كل فجوة لها ID وSeverity وImpact وOwner وRemediation وEvidence وStatus. الفجوات الأمنية الحرجة وذات أثر سلامة البناء أو الأسرار أو الهوية تعتبر حواجز إصدار.

## 9. Canonical Authority — المرجع

1. الكود المدمج في main مع اختبارات وأدلة.
2. ADR معتمد.
3. المتطلبات المعتمدة.
4. العقود والمخططات.
5. الأدلة.
6. الفروع وPRs غير المدمجة.
7. الأرشيف والمناقشات التاريخية.

## 10. Remediation

تحدد السبب الجذري والتغيير والتبعيات والاختبارات والمالك ومعيار القبول ودليل الإغلاق.

## 11. Correction

يشمل الدمج وإعادة الهيكلة والنقل والتسمية وإزالة التكرار وتصحيح التناقضات، مع الحفاظ على provenance.

## 12. Implementation

Requirement → ADR/Spec → Task → Branch → Code → Tests → PR → Review → Merge

## 13. Testing

Unit، Integration، E2E، Protocol/Conformance، Property/Fuzz، Negative paths، Performance، Platform، Build/Provenance.

## 14. Security Review

Threat model، trust boundaries، crypto، identity، keys، storage، parsers، replay/freshness، network، supply chain، update/signing.

## 15. Verification

المطور يثبت التنفيذ، QA يتحقق من السلوك، Security يراجع invariants، والمتحقق المستقل يراجع evidence قبل الإصدار.

## 16. Evidence

source commit، build identifiers، test results، security review، SBOM/dependency snapshot، provenance، limitations وrollback evidence.

## 17. Release Gate

الحظر عند Security blocker، اختبار أساسي فاشل، متطلب حرج بلا evidence، artifact بلا provenance، dependency حرجة بلا مراجعة، secret في repo أو artifact، أو rollback/recovery غير معروف.

## 18. Release

نسخة موسومة وموقعة بحسب سياسة المشروع، مع artifacts وhashes وملاحظات وأدلة.

## 19. Post-Release

مراقبة الأعطال والأداء والاتصال والاعتماديات والثغرات والتحديثات، دون جمع plaintext كآلية مراقبة.

## 20. Recycle

أي حادث أو ثغرة أو تعديل protocol أو dependency يعيد الدورة من المرحلة المناسبة.
