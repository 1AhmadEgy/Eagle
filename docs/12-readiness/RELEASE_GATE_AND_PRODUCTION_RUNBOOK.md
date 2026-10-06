# Release Gate & Production Runbook

## Gate A — Requirements
- P0 requirements معتمدة.
- لا requirement حرجة مجهولة المصدر.

## Gate B — Architecture
- المخططات والعقود محدثة.
- ADRs الحرجة معتمدة.
- لا trust-boundary mismatch غير مقبولة.

## Gate C — Security
- crypto/key lifecycle reviewed.
- threat model محدث.
- security tests ناجحة.
- no secrets.
- supply chain reviewed.

## Gate D — Quality
- unit/integration/e2e.
- fuzz/property بحسب المكون.
- platform matrix.
- performance baseline.

## Gate E — Verification
- evidence pack مكتمل.
- verifier مستقل.
- known limitations مسجلة.

## Gate F — Release Engineering
- artifact provenance.
- hashes.
- signing process محمي.
- rollback artifact/path جاهز.

## قرار الإصدار

PASS = جميع البوابات الحرجة PASS.

CONDITIONAL = مسموح بقرار مخاطرة موثق ولا يمس السرية أو السلامة أو الهوية أو artifact integrity.

BLOCK = أي blocker أمني أو سلامة بيانات أو reproducibility حرج.

## Release package

version، source commit، artifacts، hashes، notes، security notes، known issues، test/evidence summary، rollback.

## Store distribution

Google Play / App Store / Desktop stores إن استُخدمت هي قنوات توزيع فقط، وليست message infrastructure أو trust authority.

## Stop conditions

critical vulnerability، corrupted artifact، unexpected signing state، severe interoperability break، message confidentiality/integrity concern، unacceptable data loss.
