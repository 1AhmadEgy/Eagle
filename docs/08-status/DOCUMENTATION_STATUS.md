# حالة التوثيق

## الحالة الحالية

**Foundation / Evidence Collection / Stack Discovery — توثيق مركزي محدث**

تم الانتقال من هيكل توثيق أولي إلى سجل مترابط يربط:

المصدر → المتطلب → القرار → المعمارية → الفجوة → التنفيذ → الاختبار → الدليل → حالة الجاهزية.

لم يتم الادعاء باكتمال التوثيق التاريخي أو اكتمال متطلبات المنتج.

## ما تم تثبيته

- سجل قرارات مركزي ومؤرخ.
- سجل متطلبات قابل للتتبع.
- سجل تحقق يميز بين Configured وVerified وPending وFailed.
- سجل فجوات محدث.
- سجل جاهزية وتنفيذ محدث.
- تدقيق حقائق المستودع.
- سجل إعادة استخدام المكونات.
- أول due-diligence للمكونات الخارجية.
- فهرس مركزي يربط السجلات الجديدة.
- قاعدة intake للأرشيفات التاريخية.
- قواعد Definition of Ready / Done.

## التغطية الحالية

| المجال | الحالة |
|---|---|
| Governance | Verified |
| Provenance | Verified |
| Android bootstrap | Verified |
| CI configuration | Verified |
| Live CI evidence | Pending |
| Requirements V1 | Pending |
| Architecture finalization | Pending |
| Threat model | Pending |
| Identity/key lifecycle | Pending |
| Protocol contract | Pending |
| E2EE implementation | Pending |
| Secure persistence | Pending |
| Transport | Pending |
| PrivateMesh | Pending |
| Production QA | Pending |
| Supply-chain evidence | Pending |
| Historical archive intake | Pending |

## قواعد الحالة

كل معلومة يجب أن تحمل حالة واحدة على الأقل:

Verified / Derived / Proposed / Pending / Rejected

ولا يجوز استخدام "Verified" لمجرد وجود وثيقة أو workflow؛ يجب أن تكون هناك أدلة مناسبة للادعاء.

## الأعمال التالية

1. استكمال intake للأرشيفات عند توفر bytes.
2. استخراج المتطلبات المتعارضة من المصادر التاريخية وتسجيل التعارضات بدل دمجها بصمت.
3. اعتماد V1 requirements baseline.
4. تثبيت trust boundaries.
5. إنشاء Threat Model.
6. تحويل المكونات المرشحة إلى contracts قبل إضافة dependencies.
7. استبدال SmokeTest الحالي باختبارات سلوكية تدريجيًا.
8. تنفيذ أول secure end-to-end slice.
9. حفظ evidence لكل CI/test run مرتبطًا بالـcommit SHA.
10. تحديث هذه الصفحة وسجلات القرار/المتطلبات/التحقق بعد كل milestone.

## قاعدة الجودة

لا تُغلق أي فجوة بمجرد اقتراح حل. الإغلاق يتطلب:

**Implementation + Test + Evidence + Documentation + Review**

## قاعدة التغيير

أي تغيير جوهري في المتطلبات أو المعمارية أو الأمن أو الاعتماديات أو الجاهزية يجب أن ينتج عنه تحديث للسجل المرجعي المناسب داخل المستودع.
