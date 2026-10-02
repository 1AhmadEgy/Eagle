# ADR-0011 → ADR-0014 Worklist

These records are intentionally work items, not accepted decisions.

- ADR-0011 — Local Storage
  - scope: persistence model, encrypted-record strategy, migration, backup/retention constraints
- ADR-0012 — Transport Architecture
  - scope: transport abstraction, supported channels, reliability, framing, lifecycle, background behavior
- ADR-0013 — Architecture Enforcement
  - scope: automated module-dependency enforcement, CI gate implementation, rule maintenance
- ADR-0014 — Observability
  - scope: sanitized logging, diagnostics, metrics, crash reporting, privacy boundaries

## Rule
Do not infer a technical choice from the title of an ADR. Each ADR becomes Accepted only after its own evidence and approval checklist are completed.

## Dependency
- ADR-0011 is informed by accepted key-management and protocol decisions.
- ADR-0012 is informed by accepted protocol and transport requirements.
- ADR-0013 enforces the accepted Architecture Baseline.
- ADR-0014 must preserve the boundary against plaintext and secrets in telemetry.