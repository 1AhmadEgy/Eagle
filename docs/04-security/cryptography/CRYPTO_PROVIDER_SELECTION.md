# Eagle — Cryptographic Provider Selection

## Security rule

No provider is production-approved merely because it implements a relevant protocol. Approval requires exact version and revision, provenance, license compatibility, supported target platforms, conformance/interoperability evidence, supply-chain review, and independent security review.

## Candidate A — Signal libsignal

Status: REFERENCE / CONFORMANCE ONLY

Current upstream workspace version: **0.104.0**. The upstream repository implements the Signal protocol family, including PQXDH and Double Ratchet, but explicitly states that use outside Signal is unsupported. The current repository is AGPL-3.0-only.

Decision:
- retain as the primary protocol/reference corpus;
- do not vendor or copy implementation code;
- do not claim production approval without a separate legal/license, support, provenance, platform, conformance, supply-chain, and security decision.

## Candidate B — Matrix vodozemac

Status: RESEARCH / COMPONENT CANDIDATE ONLY

Current release: **0.10.0**. It is a pure-Rust implementation of Olm and Megolm, Apache-2.0, and reports one Least Authority security audit with no significant findings. It is not a PQXDH implementation and cannot be composed with a separately written PQXDH handshake without a dedicated protocol-security review. The current changelog also shows active security/behavior changes, reinforcing exact-version pinning.

Decision:
- research/comparative-test use only;
- never substitute it for Eagle's approved 1:1 PQXDH path;
- exact revision must be frozen before any integration experiment.

## Candidate C — OpenMLS

Status: GROUP-CRYPTO REFERENCE / INTEGRATION CANDIDATE ONLY

Current release: **0.9.0**. OpenMLS is a Rust implementation of RFC 9420 and currently documents classical MLS ciphersuites; it does not provide Eagle's 1:1 PQXDH/Double-Ratchet stack. OpenMLS also separates protocol implementation from cryptographic providers, which is useful for provider isolation but does not remove the need for independent integration review.

Decision:
- group track only;
- no 1:1 substitution;
- no production adoption until group interoperability, storage, provider, and security evidence exist.

## Candidate D — Small independent Signal implementations

Status: REJECTED FOR PRODUCTION BY DEFAULT

Small or educational implementations are not promoted to production merely because they are MIT/Apache/GPL licensed or implement X3DH/Double Ratchet. Evidence is required across implementation maturity, review, conformance, interoperability, provenance, maintenance, and lifecycle support.

## Current decision

**NO PRODUCTION CRYPTO PROVIDER APPROVED YET.**

The safest path remains a complete, supportable implementation path with an explicit protocol profile, rather than assembling a Signal-equivalent stack from unrelated cryptographic components.

---
