# PrivateMesh — Resource & Abuse Limits

Status: REQUIRED BEFORE IMPLEMENTATION RELEASE

## Principle

Every attacker-controlled quantity MUST have an explicit upper bound before allocation or state mutation.

## Limit classes

| Class | Must be bounded | Authority |
|---|---|---|
| Frame | maximum encoded frame and metadata size | Protocol registry |
| Candidates | candidates per peer / discovery record | Mesh policy |
| Peers | concurrently tracked peers | Core/Mesh policy |
| Connections | handshakes and active paths per peer | Mesh policy |
| Routes | hop count and route width | Mesh policy |
| Queues | per-peer and global pending bytes/items | Protocol/registry |
| Retries | attempts, elapsed time, backoff | Mesh policy |
| Timers | discovery/check/connect/recovery deadlines | Mesh policy |
| Discovery | records, expiry window, refresh rate | Mesh policy |
| Relay | allocations, lifetime, retry rate | Relay policy |
| Telemetry | event size and event rate | Observability contract |

## Required enforcement order

1. Validate fixed headers and length fields.
2. Reject values above hard limits.
3. Reject unsupported versions/profiles.
4. Apply per-peer and global quotas.
5. Only then allocate or mutate connection state.

## Safety rules

- No attacker-controlled unbounded allocation.
- No infinite retry loops.
- No unlimited candidate accumulation.
- No unbounded route growth.
- No fallback that bypasses limits.
- Resource exhaustion MUST degrade or reject the affected network operation without corrupting trusted state.

## Numeric values

Numeric V1 limits are intentionally not invented in this document. They MUST be derived from the canonical protocol registry, measured resource envelope, and threat-model review before implementation is marked release-ready.
