# Eagle — Cryptographic Provider Selection

## Security rule

No provider is production-approved merely because it implements a relevant protocol. Approval requires exact version and revision, provenance, license compatibility, supported target platforms, conformance/interoperability evidence, and independent security review.

## Candidate A — Signal libsignal

Status: REFERENCE / CONFORMANCE ONLY

The upstream repository contains the Signal Protocol implementation, including PQXDH and Double Ratchet. The upstream project states that use outside Signal is unsupported, and the workspace is AGPL-3.0-only. Current upstream workspace version observed during review: 0.104.0.

Decision:
- keep as the protocol/reference corpus;
- do not vendor or copy implementation code;
- do not mark production approval without a separate license/support/platform/provenance/security decision.

Reference:
https://github.com/signalapp/libsignal

## Candidate B — Matrix vodozemac

Status: RESEARCH / COMPONENT CANDIDATE ONLY

vodozemac is a pure-Rust implementation of Olm and Megolm, licensed Apache-2.0. The upstream project reports one Least Authority security audit with no significant findings. It is not a drop-in PQXDH provider and must not be combined with an independently implemented handshake without a dedicated protocol security review.

Decision:
- may be used for research and comparative testing;
- may not be substituted for the Eagle Signal/PQXDH profile by composition;
- no production adoption until the complete protocol profile is approved.

Reference:
https://github.com/matrix-org/vodozemac

## Candidate C — Small independent Signal implementations

Status: REJECTED FOR PRODUCTION BY DEFAULT

Small or educational implementations are not promoted to production merely because they are MIT/Apache/GPL licensed or implement X3DH/Double Ratchet. The Eagle rule is evidence-first: implementation maturity, review, conformance, interoperability, provenance and lifecycle support all have to be demonstrated.

## Current decision

**NO PRODUCTION CRYPTO PROVIDER APPROVED YET.**

This is intentional. The safest path is to keep the protocol boundary fail-closed while selecting one complete, supportable implementation path rather than constructing Signal from unrelated cryptographic components.
