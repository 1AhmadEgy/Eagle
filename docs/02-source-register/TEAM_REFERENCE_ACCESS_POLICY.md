# Team Reference Access Policy

## القاعدة

D1–D10 جميعهم يحصلون على كامل corpus المرجعي من محادثات Eagle دون استثناء.

التخصص يحدد المسؤولية التنفيذية، وليس نطاق الاطلاع على المواد المرجعية.

## يشمل

- جميع الملفات والمستندات من المحادثات.
- النسخ التاريخية.
- Provenance.
- الفهارس.
- المراجع.
- الأدلة غير السرية.
- أرشيف Git ذي الصلة.

## لا يشمل

- مفاتيح خاصة.
- Tokens.
- كلمات مرور.
- شهادات الإنتاج.
- بيانات مستخدمين حقيقية.
- أسرار البنية التحتية.

## طلب الوصول

الطلب الأساسي:
CORPUS-ACCESS
- Member ID
- Role
- Scope = ALL PROJECT REFERENCE CORPUS
- Purpose
- Read/Write need
- Related task/issue

طلب ملف محدد:
FILE-REQUEST
- File ID
- Path/name
- Version/commit
- Reason
- Task/ADR
- Required deadline

## قاعدة التسليم

يسلم الملف بواسطة File ID + version + provenance/hash عند توفره، وليس الاسم وحده.

## فصل الصلاحيات

Read ≠ Merge ≠ Security Approval ≠ Release Approval.
