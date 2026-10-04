# Experiment Engine and Adaptive Selection

## Experiment modes

### Simulation

Deterministic and isolated scenario modeling. It never claims production evidence.

### Validation

Runs only the repository's detected allowlisted verification commands on a clean `agent/*` branch. The selected scenario is a risk target; this mode validates the current repository baseline and does not inject faults.

## Selection

The base priority is:

```text
risk × uncertainty × coverage_gap × recurrence
----------------------------------------------
estimated_cost
```

Historical results modify scheduling:

- repeated failures increase priority;
- unseen scenarios receive an exploration bonus;
- repeated clean passes reduce priority without removing coverage obligations.

The selector is deterministic and has no authority over security or release policy.

## Evidence

A validation result records the repository branch and commit, command outcomes, output digests, bounded redacted previews, runtime and trace data.

## Continuous learning

A result may create an evaluation/training case, regression record, knowledge-graph evidence and future scenario seed. Dataset curation does not imply model fine-tuning.

Simulation and validation remain explicitly distinguishable in every experiment record.
