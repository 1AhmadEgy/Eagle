# Corpus & Git Reconciliation Baseline — 2026-10-06

## النطاق

مصادر Eagle الأساسية:
1. ملفات ومخرجات محادثات Eagle.
2. تاريخ Git.
3. main.
4. الفروع وPRs وIssues.

## Snapshot تم التحقق منه

- Repository: 1AhmadEgy/Eagle
- Visibility: public
- Default branch: main
- Main HEAD: 46aa86b6d71d34396b86b35124392cb3ba49c2e9
- Main tree: 201 عنصرًا
- docs/: 82 عنصرًا
- لا توجد GitHub Releases منشورة وقت الفحص.
- توجد فروع تنفيذية وأمنية ووثائقية كثيرة.
- توجد PRs مفتوحة، وبعضها يعتمد على فروع غير main.

## قاعدة المطابقة

Branch/PR → changed files → requirements/ADR → tests → security review → evidence → target base

لا يتم اعتماد فرع لمجرد أنه أحدث.

## مستويات الثقة

- A: main + code/test evidence.
- B: branch/PR + evidence جزئي.
- C: docs/archive فقط.
- D: claim محادثة بلا artifact.
- E: غير متاح أو غير مكتمل.

## قاعدة التكرار

لا نحذف duplicate حتى نثبت هل هو same-content أو superseded أو forked-content أو unknown.

## قاعدة الملفات الخارجية

أي archive/package خارجي من المحادثة يظل untrusted input حتى يفحص، ويحسب hash، وتراجع الاعتماديات والترخيص والسبب والاستخدام.

## توزيع الفريق

D1–D10 لهم حق قراءة كامل corpus المرجعي من المحادثات دون استثناء. التخصص يحدد ملكية المهمة، وليس حق قراءة المرجع.

## تحديث السجل

عند كل دورة كبيرة يسجل timestamp وmain HEAD والفروع/PRs المهمة والتغييرات canonical والفجوات.
