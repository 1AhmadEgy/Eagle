# Continuous Repair, Validation and Learning Operations

## End-to-end repair path

The external EDA supports this optional autonomous path after a real validation failure:

```text
validation failure
  ↓
preserve evidence
  ↓
scenario + repository knowledge context
  ↓
primary model proposes smallest unified diff
  ↓
secondary model(s) review the exact proposal
  ↓
structural patch policy
  ↓
secret/path/scope checks
  ↓
agent/* branch only
  ↓
deterministic repository verification
  ↓
security review
  ↓
path/staging verification
  ↓
commit only after all required checks pass
  ↓
independent revalidation
  ↓
regression + knowledge + training/evaluation updates
```

## Review semantics

Secondary model outputs are **review analyses**, not replacement patches and not independent release authority. The deterministic gate remains the source of release truth.

## Continuous mode

`continuous-run --lab validation --auto-repair` enables the above path. The command does not require a GitHub token unless a GitHub operation such as publishing is explicitly requested.

## Failure handling

A failed test, failed security check, unexpected path, patch error, staging mismatch or commit failure causes rollback before publication. Failed repairs are recorded as evidence and become future learning context.

## Data lifecycle

Each experiment or repair should leave repository-local records under `.eagle-agent/`. These records are redacted, append-oriented operational history and may seed future evaluation/regression work.

## Training boundary

Training/evaluation cases are curated from verified observations. This does not mean that model weights are automatically modified. External fine-tuning or model training is a separate provider/data-controlled operation.
