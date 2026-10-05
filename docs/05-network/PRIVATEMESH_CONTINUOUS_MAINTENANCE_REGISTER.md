# PrivateMesh — Continuous P2P Maintenance Register

Status: ACTIVE / GATED

Scope: P2P / Networking / PrivateMesh only.

## Operating rule

Every material transport/network change must pass:

Research → provenance → classification → compatibility → security impact → implementation → tests → review → verification → evidence.

No background or undocumented change is treated as release work. An item becomes complete only when repository evidence exists.

## Recurring controls

| Control | Trigger | Required action |
|---|---|---|
| Security advisories | new dependency release/advisory | re-resolve lockfile, run advisory scan, assess exploitability |
| Standards drift | RFC update/obsoletes/errata | refresh normative references and applicability |
| Transport changes | new QUIC/ICE/relay version | run interoperability and failure matrix |
| Resource behavior | limits/allocator/state changes | rerun abuse/resource tests |
| Network lifecycle | Android/network stack changes | rerun interface/sleep/wake/reconnect tests |
| Path selection | selector/hysteresis changes | rerun direct/relay convergence tests |
| Routing | forwarding changes | rerun loop/hop/resource tests |
| Telemetry | event/schema changes | rerun forbidden-data and metadata-minimization checks |
| Incident | production/network security incident | preserve evidence, patch, regression test, re-review |

## Dependency hygiene

- Exact resolved versions MUST be reproducible.
- Floating transport dependencies are prohibited for release.
- Security advisories are evaluated for the actual feature set, not merely crate names.
- Unused protocol features MUST remain disabled to reduce attack surface.
- A new dependency requires an exit/replacement strategy.

## Release regression minimum

NET-001..020 remain the minimum P2P regression set. Security-critical additions must add targeted tests rather than weakening existing scenarios.

## Current watchlist

- Quinn/quinn-proto security advisories and new releases.
- rust-libp2p advisories, especially transport/resource-control components.
- ICE/STUN/TURN specification updates and security errata.
- Android network/interface lifecycle behavior affecting the Rust/core adapter.
- Newly discovered resource-amplification or parser-panic classes.

## Current disposition

The register is active, but release remains blocked until the implementation and verification gates in PRIVATEMESH_RELEASE_GATE.md are satisfied.
