# Eagle — Relay / Infrastructure Security Architecture — 2026-10-05

## Canonical objective

Provide infrastructure that improves connectivity and operability without becoming a trust root or a path for application content.

## Allowed topology

```text
Eagle A
  |
  +--> security boundary
  |
  +--> protocol boundary
  |
  +--> direct P2P transport ==================> Eagle B

Optional future rendezvous/discovery service
  ^                         |
  |                         v
  +---- opaque discovery / endpoint metadata only

Relay/TURN/content forwarding
  X  prohibited for application data
```

## Trust boundaries

### Application / UI

No direct network administration, credentials, relay control, or private-key access.

### Security Core

Authoritative for trust, authorization, key use, and fail-closed security policy.

### Protocol

Consumes/produces opaque encrypted envelopes and bounded frames. Application plaintext is outside the transport boundary.

### Mesh / Transport

Performs discovery, connectivity, retries, lifecycle, and opaque-frame movement. It never grants authorization merely because a connection exists.

### Infrastructure / Discovery

Untrusted control-plane assistance only. It may publish or exchange minimal endpoint metadata when a future accepted transport ADR permits it. It must never establish identity trust.

## Security invariants

1. No application message is sent through a relay path.
2. Network reachability never grants authorization.
3. Discovery metadata never grants identity trust.
4. Reconnect must revalidate trust/session state.
5. Unsupported versions fail closed.
6. Resource limits are checked before expensive work.
7. No runtime dependency on TURN/WebSocket/server-mediated content delivery is allowed under the current policy.
8. Logs and telemetry contain no plaintext, credentials, private keys, or raw message content.
9. Infrastructure credentials are short-lived and least-privileged when infrastructure is eventually deployed.
10. A service outage cannot silently downgrade the security model.

## Operational shape for a future approved service

When a transport service becomes authorized by an accepted ADR, the deployment should use:

- immutable container/artifact references;
- TLS 1.3 where applicable;
- isolated service identity;
- deny-by-default ingress and egress;
- explicit connection and byte quotas;
- bounded queues and request deadlines;
- health/readiness separation;
- graceful draining;
- structured sanitized logs;
- metric cardinality limits;
- secret rotation without source-code changes;
- audit trail for configuration changes;
- documented rollback.

No such production service is introduced by this change.
