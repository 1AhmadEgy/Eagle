# Eagle — Relay / Infrastructure Release Gate — 2026-10-05

## Gate rule

```
Any mandatory security or architecture blocker
        =>
NO RELEASE
```

## Gate matrix

| Gate | Evidence required | Current |
|---|---|---|
| Current canonical authority | exact main commit + accepted references | PASS |
| Provenance | source/version classification | PASS |
| P2P application-data policy | explicit invariant | PASS |
| Relay-content runtime absence | automated repository gate | PASS on branch after verification |
| Requirements | approved transport requirements | BLOCKED |
| Transport ADR | accepted direct P2P decision | BLOCKED — Proposed |
| Protocol interoperability | conformance vectors + interop | BLOCKED |
| Identity binding | cross-layer verification | BLOCKED |
| NAT traversal | direct-connect / failure tests | BLOCKED |
| Resource exhaustion | adversarial tests | BLOCKED |
| Deployment/IaC | hardened immutable deployment | NOT STARTED |
| Supply chain | exact dependencies + SBOM + provenance | PARTIAL |
| Observability | sanitized runtime telemetry | BLOCKED |
| Independent security review | external review | BLOCKED |
| Human review | architecture/security approval | BLOCKED |

## Release disposition

**NO-GO**

This gate is intentionally conservative. The implemented repository guard is not a substitute for a real transport implementation, protocol approval, or independent security review.

## Release prerequisites

A later release may proceed only after:

1. transport ADR acceptance;
2. approved runtime dependency set with exact versions;
3. direct P2P and NAT evidence;
4. resource-exhaustion and adversarial network tests;
5. identity/authorization integration evidence;
6. deployment hardening and rollback proof;
7. SBOM and artifact provenance;
8. independent security review;
9. human release approval.

## Post-release controls

Once released, retain immutable commit/artifact linkage, vulnerability status, rollback evidence, configuration-change audit, security incident workflow, and sanitized SLO/SLI monitoring.
