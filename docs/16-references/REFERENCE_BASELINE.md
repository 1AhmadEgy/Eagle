# Eagle Reference Baseline

المراجع التالية مصادر رسمية للاستفادة في التصميم والمراجعة. إدراج المرجع لا يعني اعتماد خوارزمية أو مكتبة تلقائيًا.

## Secure Development

- NIST SSDF SP 800-218: https://csrc.nist.gov/pubs/sp/800/218/final
- NIST SSDF project: https://csrc.nist.gov/projects/ssdf
- NIST CSF 2.0: https://www.nist.gov/publications/nist-cybersecurity-framework-csf-20

## Supply Chain

- OWASP SCVS: https://scvs.owasp.org/
- SCVS usage: https://scvs.owasp.org/scvs/using-scvs/

## Application Security Verification

- OWASP ASVS: https://owasp.org/projects/asvs
- ASVS developer guide: https://devguide.owasp.org/en/03-requirements/05-asvs/

## Messaging / Protocol

- IETF RFC 9420 — MLS: https://www.rfc-editor.org/info/rfc9420
- IETF RFC 9846 — TLS 1.3: https://www.rfc-editor.org/info/rfc9846
- RFC 8446 remains a historical TLS 1.3 reference and is superseded by RFC 9846.

> لا تفترض اعتماد MLS/TLS أو أي protocol بعينه قبل ADR يثبت الملاءمة مع topology ومتطلبات Eagle.

## Platform Security

- Android architecture recommendations: https://developer.android.com/topic/architecture/recommendations
- Apple Platform Security: https://support.apple.com/guide/security/welcome/web
- Apple Keychain data protection: https://support.apple.com/en-ae/guide/security/secb0694df1a/web

## كيفية استخدام المراجع

Research → Due Diligence → ADR → Threat Model → Implementation → Verification

المصدر المرجعي لا يصبح dependency بمجرد ظهوره في هذه القائمة.
