# Eagle — Protocol & Security Component Selection Matrix v0.1

**Audit date:** 2026-10-04  
**Status:** Evaluation record — no production protocol is approved by this document.  
**Scope:** 1:1 asynchronous E2E messaging first; group messaging and mesh are later layers.

## 1. Decision rule

Eagle must not implement a home-grown cryptographic messaging protocol. A candidate becomes an Eagle dependency only after:

`Requirement → Architecture Fit → Security History → Exact Version/Commit → License → Transitive Dependencies → Test Evidence → Operational Fit → Exit Strategy → Approval`

A mature project can still be rejected when its licensing, support model, API stability, platform constraints, or protocol semantics do not fit Eagle.

## 2. Current candidates

| Candidate | Intended role | Current evidence | License / support signal | Eagle decision |
|---|---|---|---|---|
| Signal Protocol specifications + libsignal | 1:1 asynchronous E2E | Signal publishes X3DH and Double Ratchet specifications; libsignal is the Rust implementation used by official Signal clients | libsignal is AGPL-3.0 and its README says use outside Signal is unsupported; APIs may change | **STUDY; do not integrate yet** |
| vodozemac | 1:1/group ratchets in Matrix ecosystem | Rust implementation of Olm (Double Ratchet) and Megolm; current README reports one Least Authority audit with no significant findings | Apache-2.0; active Matrix ecosystem code | **STUDY / ADAPT candidate** |
| OpenMLS | Group E2E / MLS | Rust implementation of RFC 9420; CI builds and tests several native targets | MIT; explicit security policy | **DEFER for group phase; study now** |
| Noise | Authenticated handshake/session foundation | Formal protocol framework with handshake patterns and encrypted transport messages | Framework, not a complete asynchronous messaging protocol | **STUDY; not sufficient alone** |
| Custom protocol | Eagle-owned end-to-end protocol | No independent implementation or security review | Project-owned risk surface | **REJECT** |

## 3. Findings that affect Eagle

### Signal family

Signal's published specifications describe X3DH for asynchronous initial key agreement and the Double Ratchet for per-message key evolution, including forward security and break-in recovery properties.

However, the current libsignal repository explicitly says use outside Signal is unsupported. The repository is currently AGPLv3, and its public README states that the Java/Swift/TypeScript and non-bridge Rust APIs can change without notice. Eagle must therefore not silently add libsignal as a dependency.

**Current gate:** protocol study is allowed; production integration remains blocked on legal/support, exact-version, build, API, and security-review evidence.

### vodozemac

vodozemac is a pure-Rust implementation of Olm and Megolm, used for end-to-end encryption in Matrix. Its public repository reports an external security audit by Least Authority with no significant findings and uses Apache-2.0.

It is technically interesting because it is Rust-native and aligns with Eagle's planned Layer C. It is not a drop-in Signal Protocol replacement: Eagle would still need to evaluate its protocol semantics, authentication model, multi-device lifecycle, interoperability needs, and exact threat model.

**Current gate:** strong alternative for study/adaptation, not approved for production.

### OpenMLS

OpenMLS implements MLS (RFC 9420) and has explicit security governance. Its documentation shows tested targets and CI builds for Android and iOS targets. MLS is particularly relevant for future group messaging.

For the initial 1:1 slice, adopting MLS would add group-oriented state and complexity before Eagle has proven identity, storage, session lifecycle, and transport-neutral message handling.

**Current gate:** defer integration; keep as the preferred group-protocol evaluation track.

### Noise

Noise provides a well-defined handshake framework and encrypted transport messages, but it is not by itself Eagle's complete asynchronous messaging protocol. Eagle would still have to specify identity binding, asynchronous prekeys, session lifecycle, message sequencing, replay/duplicate handling, persistence, multi-device behavior, and recovery.

**Current gate:** use only as a building block if the selected session architecture explicitly calls for it.

## 4. Security boundary

The intended separation remains:

`Android Keystore / KeyMint → platform key boundary`

`Rust Security Core → protocol/session state + security-sensitive serialization`

`KMP Shared Layer → domain/message state/synchronization; no second crypto implementation`

`Transport / Mesh → opaque encrypted envelopes only`

The network layer must never require plaintext application messages.

## 5. Next selection gate

Before the first real cryptographic integration, Eagle must produce a separate decision record covering:

1. exact 1:1 protocol target;
2. long-lived identity-key model;
3. asynchronous prekey model;
4. session establishment;
5. forward secrecy and compromise recovery expectations;
6. replay/duplicate semantics;
7. multi-device roadmap;
8. group-protocol boundary;
9. licensing/support decision for the chosen implementation;
10. exact dependency version/commit and reproducible build evidence.

Until that record is accepted, the repository should continue with infrastructure, consistency gates, test scaffolding, and non-cryptographic contracts only.

## 6. Primary references

- Signal Double Ratchet specification: https://signal.org/docs/specifications/doubleratchet/
- Signal X3DH specification: https://signal.org/docs/specifications/x3dh/
- libsignal: https://github.com/signalapp/libsignal
- vodozemac: https://github.com/matrix-org/vodozemac
- OpenMLS: https://github.com/openmls/openmls
- Noise framework: https://noiseprotocol.org/noise.html
- RFC 9420 (MLS): https://www.rfc-editor.org/rfc/rfc9420

**External research captured:** 2026-10-04.
