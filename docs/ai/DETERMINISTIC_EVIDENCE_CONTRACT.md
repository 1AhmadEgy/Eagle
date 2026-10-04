# Deterministic Evidence Contract

## Purpose

The external EDA may reason, research, generate scenarios and propose repairs continuously. Repository changes are governed by a deterministic evidence contract that does not depend on model confidence.

## Default required evidence

Before an autonomous repository change can be treated as eligible for the next controlled stage, the system must be able to identify:

1. **Scenario** — what condition is being addressed.
2. **Validation** — what real or explicitly simulated experiment produced the observation.
3. **Security** — what security review was performed for the change.
4. **Verification** — what deterministic verification passed.
5. **Traceability** — repository, branch, commit, scenario and evidence are linked.
6. **Rollback** — a bounded rollback path exists.

Missing evidence means **BLOCKED**. An AI model cannot convert missing evidence to PASS.

## Separation of phases

A simulated experiment can create a training/evaluation case but cannot authorize a repository patch.

A real validation failure may authorize entry into the repair workflow, but the resulting patch remains untrusted until deterministic validation, security review, verification and traceability checks pass.

## Autonomy

Autonomy level may increase only from measured evidence. Security incidents or evidence gaps must demote/quarantine autonomy.

## Relationship to the Eagle lifecycle

This contract complements, and does not replace, Eagle's 20-stage lifecycle:

Inventory → Provenance → Classification → Triage → Analysis → Reconciliation → Conflicts → Gaps → Canonical Authority → Remediation Plan → Correction → Implementation → Testing → Security Review → Verification → Evidence → Release Gate → Release → Post-Release → Recycle.

The Release Gate remains the final deterministic release authority.
