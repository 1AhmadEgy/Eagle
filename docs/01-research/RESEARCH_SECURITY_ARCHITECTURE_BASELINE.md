# البحث التفصيلي — الأمن والمعايير والمكونات الجاهزة

## 1. نطاق الجولة
هذه الجولة توسّع خط الأساس الأمني إلى **مصفوفة مكونات جاهزة قابلة للدراسة**. لا يوجد اعتماد نهائي لمكوّن بعينه قبل تثبيت متطلبات Eagle والـStack وبيئة التشغيل.

## 2. مبدأ الاختيار
أي مكوّن مرشح يجب أن يمر عبر:
- وظيفة واضحة مرتبطة بمتطلب.
- توثيق رسمي قابل للتحقق.
- إصدار/دورة صيانة قابلة للتحقق.
- سجل أمني وإفصاحات قابلة للمراجعة.
- ترخيص مناسب للمشروع.
- اختبارات/أتمتة مناسبة.
- قابلية الترقية والاستبدال.
- عدم إدخال أسرار أو إعدادات حساسة في المستودع.
- أقل صلاحيات ممكنة.
- دليل تشغيل آمن وليس مجرد نجاح محلي.

## 3. مصفوفة المكونات المرشحة

| الطبقة | مكونات للدراسة | الوظيفة | الضابط الأمني/الاختبار المطلوب | الحالة |
|---|---|---|---|---|
| Identity | Keycloak | OIDC/OAuth2/SAML، SSO، MFA، federation | hardening، token/session tests، admin isolation | مرشح للدراسة |
| Fine-grained AuthZ | OpenFGA | ReBAC/Zanzibar-style authorization | model tests، deny-by-default، tenant isolation | مرشح للدراسة |
| Policy-as-Code | OPA | سياسات قابلة للاختبار بلغة Rego | TLS/authN/authZ، policy unit tests | مرشح للدراسة |
| Secrets | HashiCorp Vault | secrets/policies/PKI/dynamic credentials | least privilege، audit، rotation، recovery | مرشح للدراسة |
| Observability | OpenTelemetry Collector | traces/metrics/logs pipeline | TLS/authentication، redaction، least privilege | مرشح للدراسة |
| Database | PostgreSQL | relational durable data | authz، encryption، backup/restore، migration tests | مرشح للدراسة |
| Secret scanning | Gitleaks | كشف API keys/tokens/passwords داخل Git/files | PR + history scan، redaction | مرشح للدراسة |
| Vulnerability/config scan | Trivy | images/filesystems/repos/K8s، CVEs/IaC/secrets/SBOM | CI gates، severity policy، pinned action/image | مرشح للدراسة |
| SBOM | Syft | توليد SBOM للصور/filesystems/archives | SBOM artifact + provenance | مرشح للدراسة |
| DAST | OWASP ZAP | فحص أمني ديناميكي للتطبيق/API | baseline ثم authenticated testing | مرشح للدراسة |
| Browser E2E | Playwright | اختبارات end-to-end للواجهة والتدفقات | auth/session/access-control regression | مرشح للدراسة |
| Supply chain | Sigstore/Cosign + SLSA + GitHub Attestations | signing/provenance/verification | verify identity/provenance قبل الترقية | مرشح للدراسة |

## 4. نتائج البحث الموثقة

### 4.1 Keycloak
توثيق Keycloak الرسمي يوضح دعمه لـOpenID Connect وOAuth 2.0 وSAML، ويدعم federation مع مزودي هوية خارجيين، كما يوفر Authorization Services لسياسات fine-grained وRBAC/ABAC/context-based access. هذا يجعله مكوّنًا مناسبًا للدراسة عندما يكون المطلوب IdP مركزيًا بدل بناء نظام هوية من الصفر.

**ضوابط Eagle المقترحة:** Authorization Code + PKCE للتطبيقات المناسبة، MFA/step-up حيث يلزم، فصل إدارة الهوية عن business authorization، تدوير/إبطال الجلسات، اختبارات issuer/audience/expiry/signature، وحماية لوحة الإدارة.

### 4.2 OpenFGA
التوثيق الرسمي يعرّف نموذجًا للعلاقات بين المستخدمين والموارد، مع union/intersection/exclusion والعلاقات المتوارثة والـconditional relationships. لذلك يمكنه تمثيل authorization معقد دون نشر منطق الصلاحيات داخل كل API.

**ضوابط Eagle المقترحة:** model-as-code، اختبارات allow/deny، اختبارات tenant isolation، عدم قبول object/tenant من العميل كمصدر ثقة، ومراجعة كل تغيير في authorization model.

### 4.3 OPA
توثيق OPA الأمني يوضح أن API الخاص به يجب أن يُحمى بـTLS والمصادقة والتفويض، وأن الوضع الافتراضي للمصادقة والتفويض في API ليس حماية تلقائية. كما يدعم سياسات Rego وdeny-by-default.

**قرار هندسي مهم:** لا يجوز نشر OPA كخدمة شبكة مفتوحة ثم افتراض أنها آمنة. إذا استُخدم، يجب تحديد trust boundary، TLS، identity، authorization، وربط سياسات الوصول باختبارات.

### 4.4 HashiCorp Vault
توثيق Vault يوضح أن السياسات path-based وdeny-by-default، وأن الصلاحيات تُحدد حسب capabilities مثل read/create/update/delete/list. هذا مناسب لفصل أسرار الخدمات عن source control.

**ضوابط Eagle المقترحة:** service identity منفصل لكل مكوّن، policies ضيقة، short-lived credentials عندما يكون ذلك ممكنًا، audit logging، rotation، وخطة recovery للـsecret store.

### 4.5 OpenTelemetry
التوثيق الأمني لـOpenTelemetry Collector يوصي بالتشفير والمصادقة، وعدم تشغيل Collector بصلاحيات root، واستخدام أقل صلاحيات، وربط endpoints بشبكات/واجهات محددة، وإزالة البيانات الحساسة عبر redaction/filter/attribute processors.

**ضابط Eagle:** telemetry ليست مكانًا لنسخ الأسرار أو كل بيانات المستخدم. يجب تعريف allowlist للحقول الحساسة ومراجعة instrumentation.

### 4.6 Gitleaks
المشروع الرسمي يعرّف Gitleaks كأداة لاكتشاف الأسرار مثل كلمات المرور ومفاتيح API والـtokens في Git والملفات وstdin، ويمكن تشغيله كـpre-commit hook أو GitHub Action.

**ضابط Eagle:** تشغيله محليًا وداخل CI، مع فحص التاريخ عند الحاجة، وعدم تسجيل قيمة السر المكتشف في logs.

### 4.7 Trivy
المشروع الرسمي يذكر دعم فحص container images وfilesystems وGit repositories وVM images وKubernetes، وكشف CVEs ومشكلات IaC والمعلومات الحساسة وSBOM/licenses.

**ضابط Eagle:** لا يكفي فحص image فقط؛ عند معرفة الـstack يحدد نطاق الفحص (filesystem/dependencies/image/IaC/K8s) وسياسة severity وexception expiry.

### 4.8 Syft
Syft يولد SBOM من container images وfilesystems وarchives ويدعم صيغًا مثل CycloneDX وSPDX، ويمكن استخدام attestations مرتبطة بالـSBOM.

**ضابط Eagle:** كل release قابل للتتبع إلى SBOM محدد، مع الاحتفاظ بالدليل وربطه بالartifact.

### 4.9 Supply Chain
SLSA وGitHub Artifact Attestations يقدمان أساسًا لإثبات provenance البناء. التوثيق الرسمي يوضح أن attestation مفيد لإثبات المصدر/البناء والتحقق، لكنه ليس ضمانًا أمنيًا بذاته.

**ضابط Eagle:** release gate يجب أن يربط artifact → commit → workflow identity → provenance → SBOM → verification.

## 5. مكونات لم يتم اعتمادها بعد
لا ينبغي إضافة Keycloak/OpenFGA/OPA/Vault/PostgreSQL أو غيرها إلى runtime لمجرد وجودها في هذه الوثيقة. اختيار المكوّن يعتمد على:
1. المتطلبات النهائية.
2. اللغة/framework.
3. نمط deployment.
4. حجم النظام والـtenancy.
5. متطلبات availability/recovery.
6. القيود التشغيلية والفريق.
7. نتائج threat model.
8. الترخيص وسياسة الاعتماد على dependencies.

## 6. خط CI/CD أمني مبدئي
بعد تثبيت الـstack، المسار المقترح:
1. format/lint/unit tests.
2. secret scan — Gitleaks.
3. SAST مناسب للغة.
4. dependency/SCA scan.
5. IaC/config scan عند وجود IaC.
6. build reproducibly قدر الإمكان.
7. SBOM — Syft.
8. container/filesystem scan — Trivy عند وجود containers.
9. integration/E2E tests.
10. DAST — ZAP في بيئة اختبار.
11. artifact signing/provenance/attestation.
12. deploy only after required gates and human review للـchanges الحساسة.

## 7. مصفوفة تحقق أمنية مطلوبة
لكل مكوّن معتمد يجب إنشاء سجل:
Component → Version → License → Source → Threats → Configuration → Permissions → Tests → Vulnerability policy → Upgrade policy → Evidence → Owner

ولكل security requirement:
Requirement → Source → Implementation → Automated Test → Manual Review → Evidence → Status

## 8. مخاطر لا تزال مفتوحة
- الأرشيفات التاريخية المذكورة سابقًا لم تُسترجع في هذه الجولة.
- Stack Eagle غير مثبت من المستودع الحالي.
- لا يوجد بعد دليل كافٍ على CI/CD الفعلي للمشروع.
- Threat Model تفصيلي مرتبط بمكونات حقيقية لم يُغلق.
- لا يمكن اعتماد مكونات تشغيلية أو بنية deployment نهائية قبل إغلاق هذه الفجوات.

## 9. المصادر الرسمية الرئيسية
- Keycloak: https://www.keycloak.org/docs/latest/server_admin/
- Keycloak Authorization Services: https://www.keycloak.org/docs/latest/authorization_services/
- OpenFGA Configuration Language: https://openfga.dev/docs/configuration-language
- OPA Security: https://openpolicyagent.org/docs/security
- HashiCorp Vault Policies: https://docs.hashicorp.com/vault/docs/concepts/policies
- OpenTelemetry Collector security: https://opentelemetry.io/docs/security/config-best-practices/
- OpenTelemetry sensitive data: https://opentelemetry.io/docs/security/handling-sensitive-data/
- Gitleaks: https://github.com/gitleaks/gitleaks
- Trivy: https://github.com/aquasecurity/trivy
- Syft: https://github.com/anchore/syft
- OWASP ZAP: https://www.zaproxy.org/
- Playwright: https://playwright.dev/
- SLSA: https://slsa.dev/spec/v1.1/levels
- GitHub Artifact Attestations: https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations

## 10. الحالة
Research Expansion / Verified external documentation + Derived Eagle controls.
لا تُفهم هذه القائمة على أنها ترتيب أو فائز؛ هي قائمة مرشحين ووسائل تحقق قبل الاختيار.