# ADR-0011 → ADR-0016 Worklist

These are pending decision scopes, not accepted decisions.

- ADR-0011 — Local Storage: persistence model, encrypted-record strategy, migration, backup/retention constraints.
  - Review issue: #43
  - Current implementation gate: platform-neutral record/deletion/recovery contract only.
- ADR-0012 — Transport Architecture: transport abstraction, supported channels, reliability, lifecycle, framing, background behavior.
  - Review issue: #44
  - Current implementation gate: opaque-frame/lifecycle contract only.
- ADR-0013 — Architecture Enforcement: automated module-dependency enforcement and CI gate implementation.
- ADR-0014 — Observability: sanitized logging, diagnostics, metrics, crash reporting, and privacy boundaries.
- ADR-0016 — Storage / Recovery Contract: authoritative record boundary, explicit recovery states, fail-closed operation matrix, and deletion contract.
  - Current implementation gate: contract documentation/tests only; no production persistence technology is selected.

Each ADR requires its own evidence and approval. Do not infer a technical choice from an ADR title.