# PrivateMesh — Mesh Contract Specification

Status: Proposed implementation contract; transport choice gated by ADR-0012.

## PeerId
Opaque, stable reference to a peer identity. Mesh may compare/store it but never owns private identity material.

## Endpoint
Validated transport endpoint reference. An endpoint contains only connectivity metadata and is not an authentication credential.

## Candidate
A potential path to a peer.

Types:
- Host
- ServerReflexive
- Relayed
- FutureTransportSpecific

Candidate priority is policy-driven and deterministic. Direct paths have precedence over relay paths when equivalent and healthy.

## DiscoveryRecord
Contains peer reference, candidate set, expiry, source, and safe capability metadata. Discovery records are untrusted input.

## ConnectionState
DISCONNECTED
DISCOVERING
GATHERING
CHECKING
CONNECTING
CONNECTED
DEGRADED
RECONNECTING
FAILED

Invalid transitions MUST be rejected.

## MeshEnvelope
Opaque bytes plus bounded transport metadata. It MUST NOT expose application plaintext to Mesh.

Conceptual fields:
- protocol_version
- message_reference
- sender_peer
- recipient_peer
- opaque_payload
- route_hint
- expiry
- delivery_reference

The exact wire format remains a Protocol decision.

## MeshTransport
Required operations:
- discover(peer)
- gather_candidates(peer)
- connect(peer, candidates)
- send(peer, opaque_frame)
- receive()
- close(peer)
- state(peer)

Operations return typed errors, never decrypted content.

## RelayTransport
Implements the same opaque-frame contract as direct transport. Relay selection and failure semantics are explicit. Relay cannot become a decryption or trust service.

## Route
A bounded sequence of peer/transport hops. Route computation uses routing metadata only.

## RetryPolicy
Separates transient, recoverable, and terminal failures. Retries are bounded.

## NetworkEvent
Events are sanitized and must not contain plaintext, private keys, session secrets, or full payloads.

## Contract invariants
1. Mesh never processes application plaintext.
2. Mesh never accesses private keys.
3. Mesh has no dependency on storage internals.
4. Crypto↔Mesh circular dependency is prohibited.
5. Relay does not alter E2E semantics.
6. Unknown protocol versions fail safely.
7. Malformed network input is rejected before state mutation.
8. Resource limits are enforced before allocation.
