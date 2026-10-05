# Protocol Security Research — 2026-10-05

## 1. Signal family
Signal's current Double Ratchet specification (Revision 4, 2025-11-04) describes per-message key evolution, forward-security properties, and a Triple Ratchet construction combining classical Diffie-Hellman ratcheting with a post-quantum ratchet for hybrid security.
PQXDH remains the asynchronous key-establishment candidate for Eagle's 1:1 path.
Current libsignal repository documentation explicitly states that use outside Signal is unsupported and that APIs/implementations may change without notice. Therefore libsignal is not yet an unconditional Eagle dependency.

## 2. MLS implementation comparison
### OpenMLS
OpenMLS implements RFC 9420 and is maintained by Phoenix R&D and CE Labs. The project documents an external security audit sponsored by the Sovereign Tech Agency and conducted by SRLabs.
OpenMLS 0.9.0 (2026-08-25 release announcement; repository release tag dated 2026-08-03) is the current release found in the official repository evidence.
Two August 25, 2026 security advisories affected versions <0.9.0 and are patched in 0.9.0: pre-authentication extension decoding quadratic duplicate detection and a remotely triggerable panic in manual DeserializeBytes implementations.
The February 2026 high-severity improper tag-validation advisory was already patched before 0.9.0; OpenMLS release history records the related security patch in the 0.7.2 line.
Decision impact: OpenMLS 0.9.0+ is the strongest group-protocol implementation candidate by available evidence, but remains gated on exact revision, transitive dependency audit, configured input/resource limits, interoperability, fuzzing, and independent Eagle security review.

### mls-rs
mls-rs implements RFC 9420, reports full RFC conformance, and provides security/interoperability-focused test coverage. Its current security notice explicitly says it has not yet received a full third-party security audit.
Decision impact: strong secondary candidate and interoperability reference, but weaker than OpenMLS on external-audit evidence for Eagle's security-first gate.

## 3. Eagle recommendation
1. Do not implement a custom group-key protocol.
2. Keep MLS at the protocol-specification level until implementation selection is formally approved.
3. Use OpenMLS 0.9.0+ as the primary implementation candidate for evaluation, contingent on current advisory status and exact dependency closure.
4. Retain mls-rs as the independent conformance/interoperability reference candidate.
5. Before runtime adoption require: exact commit/version pin, SBOM/provenance, cargo-audit/RustSec review, fuzzing, MLS interoperability vectors, storage-state migration tests, parser/resource limits before unauthenticated decoding, independent security review, and rollback/revocation procedures.
6. Do not treat third-party audit existence as proof of current-version security; verify the audited scope against the exact revision selected.

## 4. Parser/resource rule derived from current OpenMLS advisories
Any Eagle P2P implementation must bound attacker-controlled frame/object size and request rate before invoking unauthenticated MLS decoding. The OpenMLS quadratic extension parsing advisory explicitly notes that the issue occurs before authentication and recommends input-size/rate limits as a temporary mitigation until patched versions are adopted.
Eagle therefore requires a protocol-level resource envelope independent of the selected MLS library.

## 5. Current Eagle implementation slice
The repository implementation now contains bounded replay/freshness delivery guards without cryptographic primitives or serialization commitments. This is intentionally compatible with the future approved Signal/MLS integration.

## 6. Release gate
PROTOCOL = NOT APPROVED.
Production integration remains blocked until protocol, key-management, serialization, and P2P transport decisions are accepted and the selected implementations pass exact-version security review and independent review.

## Sources
Signal Double Ratchet: https://signal.org/docs/specifications/doubleratchet/
Signal libsignal: https://github.com/signalapp/libsignal
OpenMLS implementation: https://github.com/openmls/openmls
OpenMLS security advisories: https://github.com/openmls/openmls/security/advisories
OpenMLS 0.9.0 release: https://github.com/openmls/openmls/releases
OpenMLS audit announcement: https://blog.openmls.tech/
mls-rs: https://github.com/awslabs/mls-rs
MLS implementation list: https://github.com/mlswg/mls-implementations
RFC 9420: https://www.rfc-editor.org/rfc/rfc9420