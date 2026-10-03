# 06 — Ten-Developer Architecture Ownership

| Developer | Primary ownership | Required second-party review |
|---|---|---|
| D1 | Tech Lead / Integration | D10 |
| D2 | Security Kernel / Trust / Policy | D9 |
| D3 | Cryptography boundary | D2 + D9 |
| D4 | Protocol | D2 + D9 |
| D5 | Network / P2P / Relay | D4 + D9 |
| D6 | Storage / Recovery / Deletion | D2 + D9 |
| D7 | Android | D8 + D9 |
| D8 | iOS / Platform | D7 + D9 |
| D9 | QA / Fuzz / Conformance | Independent verifier |
| D10 | DevSecOps / Provenance / Release | D1 + D9 |

Security-critical implementation must not be self-certified as its only verification.
