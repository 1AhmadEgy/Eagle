# 05 — Security Invariants

| ID | Invariant | Verification target |
|---|---|---|
| INV-T01 | Untrusted cannot authorize | Negative test |
| INV-T02 | Pending cannot authorize | Negative test |
| INV-T03 | Revoked cannot authorize | Negative test |
| INV-T04 | Authentication requires Pending | State transition test |
| INV-T05 | Revocation closes session | State transition test |
| INV-P01 | Below-minimum protocol is rejected without mutation | Unit test |
| INV-P02 | Protocol downgrade after a higher negotiation is rejected | Regression test |
| INV-A01 | Administrative capability denied by default | Policy test |
| INV-C01 | No custom cryptographic primitives | Dependency/code review |
| INV-C02 | E2E security is independent of transport | Integration/conformance |
| INV-D01 | Deletion covers every approved persistence path | Storage conformance |
| INV-R01 | Recovery cannot bypass trust | Recovery security test |

A test passing a primitive is not evidence that the complete product security property is satisfied.
