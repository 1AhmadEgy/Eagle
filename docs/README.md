# Eagle Documentation

هذه الصفحة هي المدخل العام لتوثيق Eagle. التوثيق مفتوح للقراءة ومصمم ليكون مفهومًا للمطور والمصمم والمدقق والمساهم الجديد دون الرجوع إلى المحادثات الخاصة.

## ما هو Eagle؟

Eagle منصة مراسلات متعددة المنصات تستهدف Android وiOS وDesktop، مع أولوية مطلقة للأمن والخصوصية والأداء والاستقرار، ومعمارية اتصال P2P أولًا وبدون اعتماد وقت التشغيل على خدمات طرف ثالث.

> **مهم:** الوثائق تميّز بين **الهدف/السياسة** وبين **ما تم التحقق من تنفيذه فعليًا**. وجود وثيقة أو فرع أو PR لا يساوي وجود ميزة منتجة.

## نقطة البداية

1. [ميثاق المشروع](00-reference/EAGLE_PROJECT_CHARTER.md)
2. [دورة حياة المشروع الكاملة 20 مرحلة](00-reference/EAGLE_MASTER_LIFECYCLE.md)
3. [سياسة التحكم بالوثائق](00-reference/EAGLE_DOCUMENT_CONTROL.md)
4. [لوحة الحالة المرجعية](00-reference/CANONICAL_STATUS_DASHBOARD.md)
5. [المتطلبات الرئيسية](09-requirements/MASTER_REQUIREMENTS_BASELINE.md)
6. [المعمارية المرجعية](03-architecture/CANONICAL_PRODUCT_ARCHITECTURE.md)
7. [المخططات](03-architecture/ARCHITECTURE_DIAGRAMS.md)
8. [خطة الأمن](04-security/SECURITY_ENGINEERING_PLAN.md)
9. [خطة التنفيذ والفروع وPR](05-engineering/IMPLEMENTATION_AND_BRANCHING_STANDARD.md)
10. [خطة الاختبارات والتحقق](07-verification/MASTER_TEST_AND_VERIFICATION_PLAN.md)
11. [بوابة الإصدار والإنتاج](12-readiness/RELEASE_GATE_AND_PRODUCTION_RUNBOOK.md)
12. [التشغيل بعد الإصدار](14-operations/POST_RELEASE_OPERATIONS.md)
13. [المراجع الخارجية](16-references/REFERENCE_BASELINE.md)

## هيكل التوثيق

- 00-reference — المرجع العام والميثاق والتحكم.
- 01-research — البحث والعناية الواجبة واختيار المكونات.
- 02-source-register — الجرد والمصادر وProvenance والمطابقة.
- 03-architecture — التصميم وADR والمخططات والعقود.
- 04-security و06-security — الأمن ونماذج التهديد وتنفيذ الضوابط.
- 05-engineering — سير التطوير والفروع وPR وCI.
- 07-verification و08-status — الاختبارات والأدلة والحالة.
- 09-requirements — المتطلبات والتتبع.
- 11-gaps و12-readiness — الفجوات والجاهزية وبوابات الإصدار.
- 13-execution — أعمال المصالحة والتنفيذ الجاري.
- 14-operations — التشغيل والاستجابة للحوادث والاستعادة.
- 15-public — مواد القراءة العامة.
- 16-references — المعايير والمراجع الخارجية.

## السلطة المرجعية

**main المدمج + ADR معتمد + المتطلبات المعتمدة + الكود المدمج + الاختبارات والأدلة = الحقيقة التشغيلية.**

الفروع وPRs والأرشيفات والمناقشات التاريخية مصادر Provenance حتى تُراجع وتُدمج وتُثبت بالأدلة.

## السرية

المستودع عام، لذلك لا يوضع فيه أي سر تشغيلي أو مفتاح خاص أو بيانات مستخدم حقيقية أو artifact سري.
