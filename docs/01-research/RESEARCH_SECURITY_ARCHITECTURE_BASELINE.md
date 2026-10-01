# البحث التفصيلي — الأمن والمعايير وسلسلة التوريد والذكاء الاصطناعي

## 1. نطاق البحث
هذه الجولة بحثية تأسيسية وليست اعتمادًا نهائيًا للـStack؛ لأن فحص المستودع الحالي لم يثبت بعد وجود ملفات التطبيق الأساسية أو الأرشيفات التاريخية المشار إليها.

## 2. المصادر المرجعية عالية الإشارة
| المجال | المصدر | ما يقدمه | حالة الاستخدام في Eagle |
|---|---|---|---|
| Web AppSec | OWASP ASVS 5.0.0 | متطلبات قابلة للاختبار للتحقق من ضوابط أمن تطبيقات الويب | مرجع متطلبات واختبارات الأمن |
| Secure SDLC | NIST SP 800-218 SSDF 1.1 | ممارسات تطوير آمن قابلة للإدماج في دورة التطوير | مرجع لسير التطوير وسلسلة التوريد |
| Cybersecurity governance | NIST CSF 2.0 | إطار Govern/Identify/Protect/Detect/Respond/Recover | مرجع لإدارة المخاطر والعمليات |
| AI security | OWASP Top 10 for LLM Applications 2025 | مخاطر خاصة بتطبيقات LLM | يستخدم إذا ثبت وجود AI/LLM |
| AI risk | NIST AI RMF + GenAI Profile | إدارة مخاطر الذكاء الاصطناعي عبر دورة الحياة | يستخدم لمسار AI |
| Build provenance | SLSA v1.x | مستويات سلامة provenance للبناء | مرجع CI/CD |
| GitHub supply chain | GitHub Artifact Attestations | إثبات مصدر وكيفية بناء artifacts والتحقق منها | مرشح قوي مع GitHub Actions |

## 3. النتائج الرئيسية

### 3.1 OWASP ASVS
OWASP يعرّف ASVS كأساس لاختبار ضوابط الأمن التقنية لتطبيقات الويب وكقائمة متطلبات للتطوير الآمن. صفحة المشروع تشير إلى الإصدار 5.0.0 وتوفر المتطلبات بصيغ قابلة للاستخدام البرمجي مثل JSON/CSV.

الإجراء المقترح: بعد معرفة الـStack، أنشئ مصفوفة ASVS requirement → implementation → automated/manual test → evidence.

### 3.2 NIST SSDF
NIST SP 800-218 يحدد ممارسات تطوير آمن يمكن دمجها في أي SDLC، بهدف تقليل الثغرات وتقليل أثر الاستغلال ومعالجة الأسباب الجذرية. صفحة NIST الحالية تسجل أيضًا مسودة SP 800-218 Rev.1 / SSDF 1.2 بتاريخ 2025-12-17، بينما الإصدار 1.1 هو الإصدار النهائي المنشور.

الإجراء المقترح: ثبّت نسخة المرجع المستخدمة في كل وثيقة ولا تخلط بين الإصدار النهائي 1.1 والمسودة الأحدث دون بيان الحالة.

### 3.3 NIST CSF 2.0
CSF 2.0 ينظم النتائج الأمنية ضمن Govern وIdentify وProtect وDetect وRespond وRecover. يمكن استخدامه على مستوى الإدارة والمخاطر، بينما ASVS وSSDF يحملان تفاصيل التطوير والتحقق.

الإجراء المقترح: استخدم CSF كطبقة حوكمة، وSSDF كسير تطوير آمن، وASVS كمعيار تحقق لتطبيق الويب؛ لا تجعلها بدائل متنافسة.

### 3.4 سلسلة توريد البرمجيات
SLSA يعرّف Build Track بمستويات متزايدة من الثقة في provenance؛ وتوضح GitHub أن Artifact Attestations تنشئ provenance مرتبطة بالبناء، وأن استخدام reusable workflows مع attestations يمكن أن يصل إلى SLSA Build Level 3 وفق شروط الدليل الرسمي.

الإجراء المقترح: عند تثبيت CI/CD، أضف provenance وSBOM والتحقق من artifacts إلى Definition of Done للإصدارات، مع أقل صلاحيات ممكنة لـGitHub Actions.

### 3.5 الذكاء الاصطناعي
NIST AI RMF مخصص لإدارة مخاطر الذكاء الاصطناعي، وNIST AI 600-1 يقدم Profile للـGenerative AI. OWASP Top 10 for LLM Applications 2025 يركز على المخاطر الخاصة بتطبيقات LLM.

الإجراء المقترح: لا تُفترض بنية AI أو Agent قبل إثبات الحاجة من المتطلبات؛ وإذا ثبتت، يجب أن تكون لكل أداة/Agent صلاحيات محدودة، مع فصل واضح بين التعليمات والبيانات والأدوات واختبارات للـprompt injection وتسريب البيانات وإساءة استخدام الأدوات.

## 4. الأمن كمعمارية
- هوية ومصادقة قوية.
- Authorization مركزي وقابل للتدقيق.
- Least Privilege.
- إدارة أسرار خارج Git.
- تحقق صارم من المدخلات والمخرجات.
- عزل حدود الثقة.
- Audit logging بدون أسرار أو بيانات غير لازمة.
- Dependency/SBOM scanning.
- SAST وsecret scanning وdependency review.
- Build provenance/attestation عند توفر CI المناسب.
- نسخ احتياطية واختبار recovery.
- مراقبة وتنبيهات قابلة للتدقيق.

## 5. مصفوفة بحث أولية
| المسار | المرجع | المخرج |
|---|---|---|
| 03 Architecture | ASVS V1 + threat modeling | Architecture + trust boundaries |
| 08 Security | ASVS 5 + CSF 2.0 | Security requirements/control matrix |
| 10 DevOps | SSDF + SLSA + GitHub security | Secure CI/CD |
| 09 QA | ASVS verification + application tests | Security/functional test matrix |
| 06 AI | OWASP LLM + NIST AI RMF | AI threat/control matrix إذا كان AI مطلوبًا |
| 04 Backend | ASVS API/security controls | API security specification |
| 07 Data | CSF/SSDF + privacy/security requirements | Data classification/retention/recovery |

## 6. الفجوات الحالية
1. لا يمكن تثبيت المعمارية النهائية قبل استرجاع الأرشيفات التاريخية.
2. لا يمكن اختيار مكتبات تنفيذية محددة قبل معرفة اللغة والframework والdeployment target.
3. لا يمكن بناء CI نهائي قبل معرفة الـStack.
4. لا يمكن اعتبار AI جزءًا من المنتج دون متطلب مثبت.
5. Threat Model تفصيلي ينتظر context/component/data-flow الفعلي.
6. يجب فحص الأرشيفات قبل رفعها إلى GitHub بحثًا عن الأسرار والبيانات الحساسة.

## 7. سياسة الاعتماد على المشاريع الجاهزة
- مشروع نشط وموثق رسميًا.
- إصدار مستقر واضح.
- سجل أمني وإفصاحات قابلة للتحقق.
- ترخيص مناسب.
- اختبارات وأتمتة.
- صيانة قابلة للاستمرار.
- قابلية الترقية والخروج من الحل.
- ربط كل dependency بمتطلب واضح.

## 8. مصادر البحث
- OWASP ASVS: https://owasp.org/projects/asvs
- OWASP Top 10 for LLM Applications 2025: https://genai.owasp.org/resource/owasp-top-10-for-llm-applications-2025/
- NIST SP 800-218 SSDF: https://csrc.nist.gov/pubs/sp/800/218/final
- NIST SSDF: https://csrc.nist.gov/projects/ssdf
- NIST CSF 2.0: https://nvlpubs.nist.gov/nistpubs/CSWP/NIST.CSWP.29.pdf
- NIST AI RMF: https://www.nist.gov/itl/ai-risk-management-framework
- NIST GenAI Profile: https://www.nist.gov/publications/artificial-intelligence-risk-management-framework-generative-artificial-intelligence
- SLSA levels: https://slsa.dev/spec/v1.1/levels
- GitHub Artifact Attestations: https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations

## 9. حالة الوثيقة
Research Baseline / Verified external sources + Derived project actions.
هذه الوثيقة لا تعني أن هذه الأطر مناسبة لكل جزء من Eagle تلقائيًا؛ المواءمة النهائية تعتمد على المتطلبات والـStack بعد استرجاع وفحص المواد التاريخية.