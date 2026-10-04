# سجل الفجوات والمخاطر المرجعية

## فجوات حالية

| ID | الفجوة | الأثر | الإجراء المطلوب | الحالة |
|---|---|---|---|---|
| GAP-001 | أرشيفات/مرفقات تاريخية محددة ما زالت غير قابلة للاستخراج كبيانات خام قابلة لإعادة الفحص | لا يمكن إثبات اكتمال كامل Corpus التاريخي | استرجاع ثنائي موثق + hash + manifest + security scan | Open |
| GAP-002 | متطلبات المنتج النهائية الكاملة ليست مثبتة بعد كوثيقة authoritative واحدة | يمنع اعتماد نطاق المنتج النهائي | توحيد المتطلبات من Corpus وربطها بقرارات | Open |
| GAP-003 | Stack الكامل للمنتج غير مثبت؛ المثبت حاليًا Android/Gradle + Rust Security Core | يمنع اعتماد جميع مكونات التشغيل والإصدارات | استكمال discovery قبل إضافة runtime dependencies | Partial |
| GAP-004 | Threat Models للأمن الأساسي أصبحت موجودة، لكن نماذج التخزين/الرسائل/المنصات/الاسترداد الشاملة لم تُغلق | مخاطر cross-boundary ما زالت مفتوحة | استكمال النماذج وربطها بالاختبارات | Partial |
| GAP-005 | CI/Test Lab موجود ويكشف الفشل فعليًا، لكن current corrected head لم يحصل بعد على PASS شامل | لا يوجد دليل release-grade قابل للتكرار حتى الآن | إنهاء fresh CI وإغلاق root causes | Partial |
| GAP-006 | صلاحيات أعضاء الفريق ليست جزءًا من مسار التحقق الحالي | لا يمكن إثبات least-privilege على مستوى GitHub team | مراجعة الصلاحيات عند دخول هذا النطاق | Open |
| GAP-007 | protocol/key-management/serialization decisions ليست merged/approved technical authorities | يمنع cryptographic production implementation | مراجعة واعتماد ADRs مع evidence | Open |
| GAP-008 | direct-only P2P transport implementation غير موجود في الكود الحالي | لا يمكن إثبات P2P-only runtime property | تنفيذ transport slice ثم adversarial tests | Open |
| GAP-009 | storage encryption/recovery/deletion proof غير منفذ | local compromise/recovery risk غير مغلق | تنفيذ storage boundary + tests | Open |
| GAP-010 | independent cryptographic/security review غير منجز | لا يمكن منح release authorization | مراجعة مستقلة قبل G6/G7 | Open |

## قاعدة الإغلاق

لا تُغلق الفجوة بمجرد اقتراح حل؛ يجب تسجيل الدليل الذي يثبت تنفيذ الحل والتحقق منه.

## أولوية الأمن

الفجوات التي تمس الأسرار أو الصلاحيات أو المصادقة أو سلامة سلسلة البناء أو التشفير تُعامل كحواجز قبل الإنتاج، وليس كتحسينات اختيارية.
