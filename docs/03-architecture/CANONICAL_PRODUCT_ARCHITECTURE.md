# Canonical Product Architecture — Target Baseline

## 1. High-level

~~~text
+-------------------------------------------------------------+
|                     Eagle Applications                      |
|       Android        |        iOS        |     Desktop       |
+----------------------+--------------------+------------------+
                       |
                       v
+-------------------------------------------------------------+
|                Shared Secure Application API                |
| identity | sessions | messaging | local policy | events      |
+-------------------------------------------------------------+
                       |
                       v
+-------------------------------------------------------------+
|                 Shared Security / Protocol Core              |
| trust | crypto boundary | protocol | replay | framing       |
+-------------------------------------------------------------+
             |                         |
             v                         v
+---------------------------+   +------------------------------+
| Secure Local Storage      |   | P2P Transport                |
| keys | metadata | queue   |   | discovery | NAT | streams     |
+---------------------------+   +------------------------------+
             |                         |
             +------------+------------+
                          v
                    Other Eagle Peers
~~~

## 2. Core rule

المنصات لا تعيد تنفيذ الأمن الحساس بصورة مستقلة. النواة المشتركة تفرض العقود، والمنصات تكيف واجهات النظام.

## 3. Runtime topology

المسار الطبيعي:
Device A → P2P transport → Device B

إن احتاجت الشبكة مساعدة:
- أي relay/network helper يجب أن يكون تحت boundary معتمد.
- لا يملك plaintext أو مفاتيح الجلسة.
- لا يصبح message database.
- لا يصبح مصدر ثقة.
- يبقى قابلًا للاستبدال.

## 4. Security boundaries

Platform boundary، Core security boundary، Storage boundary، Network boundary، Untrusted peer boundary، Untrusted media/file boundary، Build/supply-chain boundary.

## 5. Canonical interfaces

Identity، Key lifecycle، Session، Message envelope، Transport، Storage، Recovery، Verification state.

كل عقد يملك input constraints وoutput guarantees وerror model وversioning وsecurity invariants واختبارات.

## 6. Data ownership

الرسالة plaintext لا تغادر الجهاز قبل التشفير. تقلل metadata إلى الحد اللازم للاتصال والبروتوكول.

## 7. Build strategy

Shared secure core + thin platform adapters، ثم اختبار core وbindings وplatform adapter وfull application.

## 8. الحالة

هذا المستند يثبت الهدف المرجعي؛ لا يثبت أن كل طبقة منفذة على main حتى توجد أدلة مطابقة.
