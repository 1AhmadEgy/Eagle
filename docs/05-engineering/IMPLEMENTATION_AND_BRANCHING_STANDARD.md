# Implementation, Branching, PR & CI Standard

## 1. One change, one purpose

الفرع يمثل تغييرًا واضحًا قابلًا للمراجعة والاختبار.

الأنماط:
- feat/*
- fix/*
- security/*
- docs/*
- test/*
- infra/*

## 2. Base branch

الافتراضي هو main. إذا بُني branch على branch آخر يجب توضيح السبب وmerge graph.

## 3. Before coding

تثبيت:
- Requirement/Issue.
- ADR/Spec.
- impacted components.
- security impact.
- acceptance criteria.
- tests.

## 4. PR minimum

كل PR يذكر:
1. لماذا.
2. ماذا تغير.
3. المخاطر.
4. الاختبارات.
5. الوثائق المتأثرة.
6. خطة التراجع.
7. REQ/ADR/Issue.
8. حالة الأمن.

## 5. Required checks

format/lint، unit/integration، static/security analysis، provenance/dependency checks، secret scanning، platform checks، documentation consistency.

## 6. Sensitive changes

لا self-approval لتغييرات crypto/protocol/identity/storage/update/signing.

## 7. Merge rule

لا merge عند فشل بوابة حرجة أو غياب evidence.

## 8. Historical branches

تُحفظ كProvenance. قبل الاستفادة منها:
compare → inspect → reconcile → test → security review

## 9. Reproducibility

كل build رسمي يسجل source commit، toolchain، dependencies، build environment، artifact hash وprovenance.
