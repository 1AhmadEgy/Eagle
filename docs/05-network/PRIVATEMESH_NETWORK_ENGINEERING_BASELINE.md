# Eagle / PrivateMesh — P2P Networking Engineering Baseline

Status: Engineering baseline for the P2P / Networking workstream. Transport implementation remains gated by ADR-0012.

## Scope
PrivateMesh only:
- peer discovery
- candidate gathering
- connectivity checks / NAT traversal
- connection lifecycle
- direct P2P path
- relay fallback
- routing / forwarding
- retry / backoff
- network-change recovery
- mesh-specific security invariants
- protocol/transport conformance
- failure injection and observability

Out of scope:
- application/UI implementation
- cryptographic primitive design
- private-key management
- application plaintext processing
- platform-specific product behavior except adapter boundary requirements

## Security boundary
Mesh transports opaque/encrypted frames only.
Mesh never handles application plaintext.
Mesh never owns or accesses private keys.
Relay is transport fallback and does not replace end-to-end encryption.
Crypto and Mesh must not form a circular dependency.

## Required flow
Discovery -> Candidate Gathering -> Connectivity Checks -> Connection -> Peer Authentication boundary -> Secure Session boundary -> Direct P2P -> Relay Fallback -> Direct Recovery.

## Contract families
PeerId
Endpoint
Candidate
DiscoveryRecord
Connection
ConnectionState
MeshEnvelope
MeshTransport
RelayTransport
Route
RouteTable
MeshSession
NetworkEvent
RetryPolicy
Backoff

## MESH-001..015 sequence
1. Peer Identity Reference
2. Endpoint/Candidate Model
3. Discovery Contract
4. Connectivity/NAT Traversal Boundary
5. Connection Lifecycle
6. Opaque Transport Contract
7. Direct P2P Path
8. Relay Fallback
9. Routing/Forwarding
10. Retry/Backoff
11. Network Change/Recovery
12. Mesh Security Invariants
13. Protocol/Transport Conformance
14. Failure Injection/Network Test Matrix
15. Observability/Release Evidence

## Transport decision gate
QUIC, WebRTC, ICE, STUN and TURN are references/candidates until ADR-0012 is explicitly approved. Do not infer adoption from discussion or historical material.

## Acceptance
Every task requires implementation evidence, deterministic tests, security review where applicable, and repository evidence before PASS/COMPLETE.
