# Eagle Security Test Matrix — 2026-10-05

| Domain | Required tests | Current implementation evidence | Status |
|---|---|---|---|
| Trust state | unknown/pending/revoked/replaced authorization denial | Rust unit/integration tests | PASS for kernel slice |
| Session state | invalid transition denial | Rust unit/integration tests | PASS for kernel slice |
| Versioning | downgrade/unsupported rejection and no mutation | Rust unit/integration tests | PASS for kernel slice |
| Bounds | ID/payload/frame-length abuse | Rust unit/integration tests | PASS for kernel slice |
| Identity | key substitution/device mismatch | Identity threat scenarios | PENDING crypto integration |
| Pairing | MITM/replay/phishing/expiry | scenario corpus | PENDING |
| Revocation | offline convergence/stale epoch/rollback | scenario corpus | PENDING |
| Recovery | account-vs-history separation/abuse | recovery design | PENDING |
| P2P | direct-only, NAT, relay denial, peer binding | P2P scenario corpus | PENDING |
| Crypto | PQXDH vectors/Double Ratchet vectors/interoperability | protocol gate; no production crypto integrated | PENDING |
| Storage | encryption/deletion/rollback/power loss | Android Keystore AES-GCM adapter unit contract tests; device/instrumentation and full storage lifecycle evidence absent | PARTIAL |
| Cross-platform | Android/Desktop/iOS policy parity | platform strategy | PENDING |
| Fuzz/property | malformed framing and state machine fuzzing | test design | PENDING |
| Supply chain | secrets/dependency/provenance/SBOM | CI baseline | PARTIAL |
| Independent review | cryptographic + security architecture | release gate | PENDING |

## Rule

PENDING remains the status until an executable test and its evidence are present. A scenario definition alone never becomes PASS.
