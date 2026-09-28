# Eagle — Pre-Push Test Gate

## Rule

Code must not be uploaded to GitHub when a mandatory gate fails.

## Required checks

1. Unit tests
2. Android lint
3. Debug build

The gate is intentionally small at bootstrap. Crypto, protocol, security, dependency, fuzz, emulator, performance, and regression suites will become mandatory as their corresponding modules are implemented.

## Local execution

Run:

`./scripts/pre-push-gate.sh`

The same checks are repeated by GitHub Actions as an independent second layer.

## Security principle

A green test run is evidence of tested behavior, not proof that the system is secure. Security claims require threat-model-driven tests, code review, dependency verification, and adversarial testing.
