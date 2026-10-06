# Master Testing & Verification Plan

## 1. Levels

### L0 Static
format، lint، build checks، policy checks، secret scan.

### L1 Unit
identity rules، crypto boundaries، protocol state، storage abstractions.

### L2 Integration
core + storage + transport + platform adapters.

### L3 End-to-End
جهازان أو أكثر، trust، session، send/receive، reconnect، network failure.

### L4 Adversarial
replay، invalid frames، truncation، duplicates، reorder، oversized input، malicious peer.

### L5 Fuzz/Property
كل parser/state machine حساس مع corpus ثابت وfuzzing مستمر.

### L6 Performance
startup، memory، CPU، battery، latency، throughput، recovery.

## 2. Independence

المطور يثبت التنفيذ، QA يتحقق من السلوك، Security يراجع invariants، Verifier مستقل يراجع evidence.

## 3. Acceptance

لكل requirement:
Given → When → Then → Evidence

## 4. Evidence retention

source commit، run/job id، result، test version، artifact، timestamp، environment.

## 5. Flaky tests

الاختبار غير المستقر ليس نجاحًا. يسجل كفجوة ويعالج أو يعزل رسميًا قبل release.

## 6. Failure policy

أي crash أو data loss أو security-invariant violation في P0 يمنع الإصدار حتى الإغلاق والتحقق.

## 7. Verification report

يجب أن يجيب: ماذا اختبرنا؟ ماذا لم نختبر؟ ما القيود؟ ما المخاطر؟ وهل الدليل قابل لإعادة التحقق؟
