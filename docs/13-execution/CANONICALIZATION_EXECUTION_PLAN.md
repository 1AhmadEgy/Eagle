# Canonicalization Execution Plan

## المرحلة A — Freeze claims

لا نستخدم كلمات final أو production-ready إلا مع evidence.

## المرحلة B — Inventory

تحديث source register من:
- chat corpus.
- main.
- branches.
- PRs.
- historical archive.

## المرحلة C — Reconcile

بناء:
- duplicate map.
- conflict register.
- requirement traceability.
- implementation matrix.
- dependency/provenance map.

## المرحلة D — Promote

يتم اختيار مرجع canonical لكل مجال:
- product.
- architecture.
- security.
- protocol.
- storage.
- platform.
- testing.
- release.

كل promotion يجب أن يحمل سبب القرار ومرجع البديل.

## المرحلة E — Remediate

الفجوات P0 ثم P1 ثم P2/P3.

## المرحلة F — Implement

تنفيذ المكونات من العقود المعتمدة، وليس من archive مباشر.

## المرحلة G — Verify

إعادة الاختبارات والأمن والـconformance والتكامل والأداء.

## المرحلة H — Release

بناء release candidate ثم Gate ثم إصدار.

## المرحلة I — Continuous

أي تغيير جديد يعيد التحقق من المرجع المتأثر ويمنع drift.
