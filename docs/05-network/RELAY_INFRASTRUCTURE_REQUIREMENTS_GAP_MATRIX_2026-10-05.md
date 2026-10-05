# Eagle — Relay / Infrastructure Requirements & Gap Matrix — 2026-10-05

| ID | Requirement / security property | Evidence | Status | Required closure |
|---|---|---|---|---|
| RI-001 | Application data remains P2P-only | Current conflict register + P2P threat model | Verified policy | Keep fail-closed enforcement |
| RI-002 | Relay cannot become a trust root | Identity trust model | Verified policy | Revalidate on transport integration |
| RI-003 | Relay/TURN content path is absent from runtime | Current main source inventory | Verified on main | Enforce repository gate |
| RI-004 | Transport versioning fails closed | Proposed ADR-0012 + protocol branch evidence | Pending final ADR | Accept ADR + conformance |
| RI-005 | NAT traversal is direct-first | Proposed ADR-0012 | Pending | Implement only after transport approval |
| RI-006 | Reconnect reauthorizes the peer | Identity/P2P threat models | Pending integration | Cross-layer tests |
| RI-007 | Connection and byte exhaustion are bounded | P2P threat model | Pending implementation | Resource/quota tests |
| RI-008 | Discovery metadata is minimized | P2P threat model | Pending privacy review | Data inventory + privacy test |
| RI-009 | Observability excludes secrets/plaintext | Security baseline | Baseline policy | Add runtime telemetry tests |
| RI-010 | Infrastructure dependencies are pinned | Engineering/security baseline | Partial | Exact versions + SBOM |
| RI-011 | Container/IaC security is verified | Operations requirements | Pending | Container/IaC scanning when deployed |
| RI-012 | Independent transport security review | ADR-0012 | Pending | Independent review |
| RI-013 | Reproducible deployment and rollback | Release policy baseline | Pending | Artifact provenance + rollback drill |
| RI-014 | Post-release monitoring | Release gate requirements | Pending | SLO/SLI + alerting evidence |

## Gap status

```
Closed now:
  policy boundary, repository enforcement, provenance, architecture record

Open:
  actual direct P2P transport implementation
  approved transport ADR
  NAT traversal
  resource-exhaustion defenses in runtime
  privacy review
  deployment/IaC
  operational evidence
  independent security review
```

## Security rule

A missing implementation is never converted to PASS by documentation alone.
