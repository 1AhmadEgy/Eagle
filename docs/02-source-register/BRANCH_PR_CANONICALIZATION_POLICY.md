# Branch & PR Canonicalization Policy

## الهدف

منع تعدد الفروع وPRs من خلق أكثر من حقيقة للمشروع.

## قواعد المقارنة

لكل PR أو branch مهم:
1. سجل base/head.
2. قارن commits/files.
3. حدد المتطلبات وADR المرتبطة.
4. افحص tests.
5. افحص security impact.
6. افحص provenance.
7. حدد هل يستحق الدمج أو الإلغاء أو الحفظ التاريخي.

## حالات العناصر

- MERGE-CANDIDATE
- REVIEW-REQUIRED
- SUPERSEDED
- CONFLICTED
- HISTORICAL
- REJECTED

## قاعدة main

main هو خط التكامل المرجعي. لا يسمح لفرع متقدم بتغيير تعريف "الحقيقة" قبل merge ناجح ومراجعة مناسبة.

## Base branch غير main

وجود PR مبني على فرع آخر لا يعني أنه صالح مباشرة لـmain. يجب تقييم السلسلة كاملة ثم إعادة توجيه أو دمجها بطريقة مقصودة.

## الإغلاق

عند إغلاق أو supersede فرع، لا نحذف provenance المرتبط إذا كان مفيدًا لفهم القرار أو الدليل.
