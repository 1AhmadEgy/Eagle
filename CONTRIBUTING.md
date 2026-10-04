# Contributing to Eagle

Eagle follows an evidence-first development workflow. Contributions should improve the repository without weakening its security, verification, or provenance controls.

## Before changing code

Read:

- [AGENTS.md](AGENTS.md)
- [SECURITY.md](SECURITY.md)
- [Architecture](docs/03-architecture/ARCHITECTURE.md)
- [Implementation Readiness](docs/12-readiness/IMPLEMENTATION_READINESS.md)

A change should have a clear reason, an identified scope, and a verification method.

## Branching

Do not push automated changes directly to `main`.

Use a focused branch for the change, for example:

```text
docs/<topic>
feature/<topic>
fix/<topic>
security/<topic>
test/<topic>
```

Keep the change as small as practical and avoid unrelated refactors.

## Verification

At minimum, use the checks relevant to the affected surface:

```bash
gradle :app:testDebugUnitTest --no-daemon --console=plain
gradle :app:lint --no-daemon --console=plain
gradle :app:assembleDebug --no-daemon --console=plain
```

For repository-level verification:

```bash
bash scripts/ci/verify.sh
```

A missing test category is **pending**, not passing.

## Security

Never commit:

- passwords or access tokens;
- API keys;
- private keys or certificates;
- production secrets;
- unnecessary personal data;
- exploit details intended for private disclosure.

Prefer established, maintained security libraries over custom cryptographic primitives.

## Documentation

When a change affects architecture, security assumptions, requirements, verification behavior, or project status, update the relevant documentation and preserve traceability.

## Pull requests

A pull request should state:

- what changed;
- why it changed;
- what was verified;
- known limitations or pending gates;
- affected documentation or architecture decisions.

Human review is required before merging generated or security-sensitive changes.
