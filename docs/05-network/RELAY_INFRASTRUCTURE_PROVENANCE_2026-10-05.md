# Eagle — Relay / Infrastructure Provenance — 2026-10-05

## Scope

This record is limited to Relay / Infrastructure engineering. It does not promote transport, cryptography, storage, or product proposals outside their current authority.

## Evidence hierarchy

1. Current Git state on `main`.
2. Accepted architecture/security records.
3. Reviewed branch-local implementation evidence.
4. Historical archives and prior PRs.
5. Referenced but unretrieved artifacts.

## Sources examined

| Source | Role | Disposition |
|---|---|---|
| `docs/03-architecture/PLATFORMS.md` | platform authority | Current accepted reference |
| `docs/03-architecture/ARCHITECTURE.md` | architecture rules | Current reference framework |
| `docs/06-security/IDENTITY_TRUST_THREAT_MODEL.md` | trust boundary | Current security model |
| `docs/13-execution/CORPUS_CONFLICT_REGISTER_2026-10-05.md` | conflict authority | Current reconciliation record |
| `docs/13-execution/CORPUS_CANONICALIZATION_STATUS_2026-10-05.md` | canonicality rules | Current reconciliation record |
| `execution/protocol-implementation-baseline-2026-10-05` | transport/protocol candidate | Branch-local evidence only |
| `docs/06-security/P2P_TRANSPORT_THREAT_MODEL.md` on the protocol branch | P2P threat evidence | Candidate branch evidence |
| Historical PrivateMesh/ADR packages | prior alternatives | Historical evidence only |

## Canonical transport constraint

The current project constraint is **P2P-only for application data**. Historical server-mediated relay and WebSocket proposals are not current implementation authority.

The proposed ADR-0012 on the protocol branch identifies direct QUIC as a candidate and explicitly prohibits relay/TURN/content forwarding for application data. Because ADR-0012 is still Proposed, it is not treated here as authorization for runtime transport implementation.

## Infrastructure decision boundary

This work therefore implements only a repository-enforced safety boundary:

- application data must not acquire a relay path;
- relay/TURN/WebSocket server-mediated dependencies must not enter application/runtime source under the current P2P-only policy;
- discovery/rendezvous remains non-authoritative and is not an authorization source;
- no server-side decryption or message broker is introduced.

## Provenance rule

Every future transport or infrastructure implementation must record:

- baseline commit;
- governing accepted ADR/specification;
- dependency and exact version;
- threat model;
- tests and adversarial evidence;
- operational controls;
- release-gate state.

A branch name or version label is never sufficient to establish canonicality.
