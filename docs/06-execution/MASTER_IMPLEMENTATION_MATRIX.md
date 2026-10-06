# Master Implementation Matrix

هذه المصفوفة تفصل بين **المكونات المطلوبة** و**الدليل الحالي**.

| Domain | Target | Required evidence |
|---|---|---|
| Identity | device identity + trust | code + vectors + adversarial tests |
| Crypto | approved primitives and lifecycle | ADR + implementation + review + vectors |
| Session | authenticated secure session | protocol tests + replay/freshness tests |
| Protocol | bounded framed messaging | parser tests + fuzz + conformance |
| P2P | direct peer transport | E2E multi-device evidence |
| Relay boundary | optional network assistance only | ciphertext-only proof + fail-closed tests |
| Storage | protected local storage | platform tests + theft/failure tests |
| Recovery | safe recovery without central message store | design + negative tests |
| Android | production adapter/app | device matrix + release artifact |
| iOS | production adapter/app | simulator/device matrix + release artifact |
| Desktop | production adapter/app | OS matrix + release artifact |
| UI/UX | usable secure flows | acceptance tests + accessibility evidence |
| CI/CD | repeatable gated builds | workflow evidence |
| Supply chain | dependency/provenance controls | SBOM + hashes + review |
| Release | signed distributable | gate record + artifact verification |

## Current interpretation

وجود path أو docs أو branch لا يغير حالة العنصر إلى IMPLEMENTED.

الحالة تتحول إلى IMPLEMENTED بعد وجود source code مناسب، وإلى VERIFIED بعد نجاح الاختبارات والمراجعة والأدلة، وإلى RELEASED بعد اجتياز Release Gate.
