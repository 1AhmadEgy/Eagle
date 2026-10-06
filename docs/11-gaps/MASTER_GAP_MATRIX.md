# Master Gap Matrix

## الغرض

الفجوة هي الفرق بين الحالة المطلوبة والدليل الموجود فعليًا.

## Prefixes

| Prefix | المجال |
|---|---|
| GAP-REQ | Requirements |
| GAP-ARCH | Architecture |
| GAP-SEC | Security |
| GAP-CORE | Shared Core |
| GAP-P2P | Networking |
| GAP-PLAT | Platform |
| GAP-DATA | Storage/Recovery |
| GAP-QA | Testing |
| GAP-SUPPLY | Supply Chain |
| GAP-OPS | Operations |
| GAP-DOC | Documentation |

## Priority

P0 blocker، P1 high، P2 normal، P3 improvement.

## Baseline gaps

1. يجب أن يظهر الفرق بين product target والتنفيذ الفعلي.
2. تعدد الفروع وPRs يتطلب reconciliation قبل canonicalization.
3. لا توجد Releases منشورة في snapshot الحالي؛ لذلك لا يوجد دليل إنتاج منشور من هذه الحالة.
4. كل claim عن implementation يجب أن يثبت بالكود والاختبار والدليل.
5. iOS/Desktop تحتاج evidence تنفيذية مستقلة قبل إعلان الاكتمال.
6. P2P يحتاج E2E evidence وليس وثائق فقط.
7. كل إصدار يحتاج dependency/SBOM/provenance evidence.

## Closure rule

لا تغلق GAP بوصف "تم". يجب ربطه بـcommit + test evidence + reviewer/verifier + documentation update + release impact.
