# MESH-015 — Observability and Release Gate
Status: SPECIFIED / RELEASE-GATED

## Telemetry
Allowed:
- connection state
- path type
- latency
- retry counts
- candidate counts
- sanitized error codes

Forbidden:
- application plaintext
- private keys
- session secrets
- recovery secrets
- full sensitive payloads

## Release evidence
- network test matrix passing
- security invariants reviewed
- protocol/transport conformance passing
- failure injection artifacts retained
- dependency and license review completed for adopted transport stack
- reproducibility/provenance evidence available when required

## Rule
No PASS/COMPLETE without observable repository evidence.
