# البحث التفصيلي — المشاريع المرجعية والقوالب الجاهزة

## 1. الهدف
هذه الجولة تبحث عن مشاريع مفتوحة المصدر وقوالب مرجعية يمكن الاستفادة من كودها أو وثائقها أو اختباراتِها أو أنماطها، بدل إعادة اختراع مكونات معروفة. لا يعني إدراج مشروع هنا اعتماده أو نسخ كوده إلى Eagle.

## 2. مشاريع مرجعية ظهرت في البحث

| المشروع | ما يمكن الاستفادة منه | ملاحظات أمان/استخدام | حالة Eagle |
|---|---|---|---|
| Yoshikemolo/IAM-Keycloak | مرجع IAM شامل: Keycloak، OIDC/SAML، MFA، OPA، PostgreSQL، Terraform، Helm، CI/CD، observability، threat model ووثائق تشغيل | مفيد لدراسة البنية والتوثيق؛ يجب فحص الترخيص والكود والإصدارات قبل أي reuse | دراسة |
| fhurau/distributed-auth-platform | مثال Full-stack: Next.js + Spring Boot + Keycloak + PostgreSQL، PKCE، object-level authorization، audit، Gitleaks، Trivy، kind smoke tests | صاحب المشروع يصفه كـdemo؛ لا يُعامل كـproduction blueprint | دراسة |
| vishnu2kmohan/mcp-server-langgraph | مرجع Agent/MCP يتضمن JWT، OpenFGA، OpenTelemetry، secrets، اختبارات أمنية، Kubernetes وCI/CD | مناسب فقط إذا أثبتت المتطلبات الحاجة إلى AI/MCP؛ يحتاج مراجعة مستقلة للأسرار والإعدادات والاعتماديات | دراسة مشروطة |
| ricsanfre/spring-microservices-otel-k8s | مرجع microservices مع Keycloak، Gateway، PostgreSQL، Kafka، OpenTelemetry، GitOps/Kubernetes | مثال معماري لا يُنقل كاملًا؛ تعقيد microservices يجب تبريره بمتطلبات Eagle | دراسة مشروطة |
| MicroForge | قالب cloud-native يجمع Terraform/Kubernetes/CI/observability/auth | الترخيص المعلن CC BY-NC-ND؛ لا يفترض قابلية استخدامه تجاريًا في Eagle | مرجع أنماط فقط |
| generic_spring_service | مرجع لخدمة Spring Boot مع OpenAPI، PostgreSQL/Flyway، Keycloak، ArchUnit، Testcontainers، observability | يحتوي عمدًا على بيانات اعتماد محلية غير إنتاجية داخل المستودع؛ مثال مهم لقاعدة: لا ننسخ أسرار/إعدادات demo إلى Eagle | دراسة أمنية فقط |

## 3. تحديث أمني مهم: لا نثق بالأداة لمجرد أنها Security Tool
البحث الحالي كشف مثالًا عمليًا على مخاطر سلسلة التوريد: تم تسجيل حادثة CVE-2026-33634 الخاصة بمنظومة Trivy، حيث استُخدمت بيانات اعتماد مخترقة لنشر Trivy v0.69.4 ضارًا، وتعديل وسوم لإصدارات من trivy-action وsetup-trivy، كما نُشرت صور ضارة v0.69.5 وv0.69.6 على Docker Hub. الإرشادات المنشورة أوصت بالنسخ الآمنة المحددة وبمراجعة الـworkflow وسجلات التنفيذ وتدوير الأسرار إذا كان الإصدار المتأثر قد نُفّذ.

**استنتاج Eagle:** لا يكفي اختيار أدوات أمنية معروفة. يجب تطبيق:
- pinning إلى commit SHA immutable للـGitHub Actions قدر الإمكان.
- التحقق من provenance/signature عندما يكون متاحًا.
- عدم استخدام `latest` في CI أو production.
- تقييد صلاحيات workflow (`permissions: {}` ثم منح المطلوب فقط).
- مراجعة release artifacts وSBOM.
- الاحتفاظ بإمكانية استبدال أداة security tool بأداة أخرى.
- تدوير الأسرار فور الاشتباه بتشغيل artifact/Action مخترق.

## 4. ما الذي نأخذه من المشاريع المرجعية؟
بدل نسخ مشروع كامل، تُستخرج الأنماط القابلة لإعادة الاستخدام:
- ADRs وتوثيق القرارات.
- بنية test pyramid وintegration tests.
- threat-model templates.
- CI security gates.
- Docker/Compose للتطوير المحلي فقط عندما يكون ذلك مناسبًا.
- IaC modules بعد مراجعة security وlicense.
- OpenAPI contract checks.
- health/readiness/liveness checks.
- observability conventions.
- authorization model tests.
- smoke tests قابلة للتكرار.
- release/provenance evidence.

## 5. قواعد منع النسخ غير الآمن
1. لا نسخ credentials أو realm exports التي تحتوي أسرارًا أو بيانات demo دون تنظيف.
2. لا نسخ Docker images غير مثبتة الإصدارات إلى مسار إنتاجي.
3. لا نسخ GitHub Actions من مصدر خارجي دون مراجعة permissions وpinning وscripts.
4. لا نسخ Terraform/Kubernetes manifests دون مراجعة RBAC وNetworkPolicy وPod Security وsecret handling.
5. لا نسخ dependencies لمجرد وجودها في reference project.
6. كل كود من طرف ثالث يحتاج مصدرًا وترخيصًا وقرار reuse واضحًا.
7. أي جزء من مشروع مرجعي يجب أن يكون قابلًا للإزالة أو التحديث دون قفل Eagle على مشروع واحد.

## 6. نمط الاختبار المقترح المستخلص
لأي خدمة جديدة في Eagle:
1. Unit tests.
2. Integration tests باستخدام بيئة مؤقتة عند الحاجة.
3. Authorization positive/negative tests.
4. Contract/OpenAPI validation.
5. Secret scanning.
6. Dependency/SCA scanning.
7. Container/IaC scanning عند وجودها.
8. E2E/smoke test.
9. DAST في بيئة اختبار.
10. Evidence artifact لكل gate.

## 7. قاعدة اختيار القوالب
القالب المرجعي لا يدخل Eagle إلا إذا اجتاز:
`Requirement → Architecture fit → Security review → License review → Dependency review → Test evidence → Maintainability review → Owner approval`

## 8. فجوة مهمة
نتائج البحث الحالية تُظهر أمثلة قوية لدراسة الأنماط، لكنها لا تثبت أن أي مشروع خارجي متوافق بالكامل مع متطلبات Eagle أو أنه آمن تلقائيًا. كما أن بعض النتائج هي مشاريع شخصية/تعليمية أو demos، ولذلك يجب فصل **reference material** عن **production dependency**.

## 9. الخطوة التالية في البحث
بعد تثبيت اللغة/framework ونمط deployment، يجب تحويل هذه القائمة إلى مصفوفة تنفيذية أدق لكل طبقة:
`Candidate → exact version → license → maintenance → CVE/security history → test coverage → architecture fit → integration effort → operational burden → exit strategy → approval`

## 10. الحالة
Research / Reference Projects & Reuse Patterns + Supply-chain verification.
المعلومات الخارجية تم جمعها لأغراض الدراسة؛ لا يوجد اعتماد نهائي أو ترتيب للمشاريع.