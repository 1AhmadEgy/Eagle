# Eagle Master Task Map — Architecture V2

**Status:** Proposed execution map  
**Baseline:** Current repository + Canonical Planning Baseline v1 + platform strategy  
**Purpose:** convert the architecture into reviewable Issues/PRs without prematurely creating a large module tree.

> **Important:** The repository currently has only `:app` as an actual Gradle module. All target modules in this map are planned work.

## 1. Execution principles

1. Architecture boundaries are established before large-scale extraction.
2. Every migration step must leave the Android app buildable.
3. A conceptual domain does not automatically become a Gradle module.
4. Security-sensitive code moves only after its API boundary and tests are defined.
5. AI remains optional and replaceable.
6. Desktop and iOS consume the shared architecture; they do not redefine it.
7. Web remains deferred until the shared/security boundaries are stable.
8. ADRs record decisions; Issues/PRs implement, verify, migrate, or remove temporary compatibility code.

## 2. Workstream sequence

| Order | Workstream | Primary outcome | Gate |
|---|---|---|---|
| 0 | Architecture baseline | V2 boundaries accepted | Architecture review |
| 1 | Dependency enforcement | Rules can be checked automatically | Static/dependency gate |
| 2 | Security boundary | Stable security API + FFI contract | Security review |
| 3 | KMP foundation | Minimal shared module exists | Shared build + tests |
| 4 | Android migration | `:app` consumes shared layer | Android regression gate |
| 5 | Core vertical slices | Identity, messaging, storage, sync moved incrementally | Integration gate |
| 6 | Desktop foundation | Desktop adapter/app can consume shared layer | Desktop build gate |
| 7 | AI boundary | Optional AI layer with explicit capabilities | Security/privacy review |
| 8 | iOS foundation | iOS adapter consumes stabilized shared APIs | iOS build/test gate |
| 9 | Web | Only after explicit readiness review | Web architecture ADR |

## 3. Proposed Issues / PRs

### ARCH-V2 — Architecture and enforcement

**ARCH-V2-001 — Approve Architecture V2 boundaries**
- Review `DEPENDENCY_GRAPH.md`.
- Review platform strategy.
- Reconcile with ADR-0007 and existing ADRs.
- Record approval evidence.
- **Done when:** architecture review explicitly accepts or rejects each boundary.

**ARCH-V2-002 — Define dependency enforcement rules**
- Translate forbidden edges into machine-checkable rules.
- Start with the smallest useful rule set.
- Do not weaken tests to accommodate violations.
- **Done when:** CI/Test Lab can fail on a forbidden dependency.

**ARCH-V2-003 — Define migration compatibility policy**
- Identify temporary compatibility packages/adapters.
- Give every compatibility layer an owner and removal condition.
- **Done when:** no migration shim can become permanent by accident.

### SEC-V2 — Security boundary

**SEC-V2-001 — Define Security API**
- Narrow operations for identity/key lifecycle and cryptographic services.
- No platform UI types.
- No database types.
- No AI/provider types.
- **Done when:** API can be reviewed independently from its implementation.

**SEC-V2-002 — Define Rust FFI boundary**
- Specify UniFFI-facing types and ownership/lifetime rules.
- Define supported Android/desktop targets.
- Keep implementation details behind the boundary.
- **Done when:** FFI contract is documented and testable.

**SEC-V2-003 — Security boundary test suite**
- Add tests for denied access patterns and sensitive-data handling.
- Track Cryptography and Security categories separately.
- **Done when:** required security tests are PASS or explicitly PENDING with evidence.

### KMP-V2 — Shared foundation

**KMP-V2-001 — Introduce minimal shared module**
- Add only the module required for stable shared contracts.
- Do not create all planned domains yet.
- **Done when:** shared code builds without depending on Android UI.

**KMP-V2-002 — Extract core/domain contracts**
- Move stable models, errors, events, and use-case contracts.
- Preserve behavior.
- **Done when:** Android behavior is unchanged and tests cover moved contracts.

**KMP-V2-003 — Extract authentication/identity contracts**
- Move identity-facing interfaces, not platform key implementations.
- **Done when:** Android consumes the contract through an adapter.

**KMP-V2-004 — Extract messaging/protocol contracts**
- Move protocol-facing contracts after ADR decisions are approved.
- **Done when:** no UI code owns protocol serialization details.

**KMP-V2-005 — Extract storage contracts**
- Define repository interfaces before moving persistence implementations.
- **Done when:** storage implementation can change without domain changes.

**KMP-V2-006 — Extract sync contracts**
- Define synchronization/use-case boundaries independently from transport.
- **Done when:** sync does not directly depend on platform transport APIs.

### ANDROID-V2 — Android migration

**ANDROID-V2-001 — Convert `:app` into composition root**
- Keep Android-specific startup, resources, permissions, and UI wiring here.
- **Done when:** `:app` contains platform composition rather than core business logic.

**ANDROID-V2-002 — Add Android platform adapters**
- Key storage, lifecycle/background behavior, notifications, and platform transport capabilities.
- **Done when:** shared code accesses these through interfaces.

**ANDROID-V2-003 — Remove forbidden direct dependencies**
- UI → private keys: forbidden.
- UI → crypto internals: forbidden.
- UI → database internals: forbidden.
- **Done when:** dependency enforcement passes.

### DESKTOP-V2 — Desktop

**DESKTOP-V2-001 — Desktop target skeleton**
- Add desktop only after the shared foundation is stable.
- **Done when:** desktop composition root builds without copying domain logic.

**DESKTOP-V2-002 — Desktop security/storage adapters**
- Reuse shared contracts; implement platform-specific capabilities separately.
- **Done when:** no desktop adapter leaks into shared domain.

### AI-V2 — AI boundary

**AI-V2-001 — Define AI capability interface**
- Model-independent interface.
- Explicit input/output contracts.
- Permission-aware tool invocation.
- **Done when:** AI can be disabled without breaking core flows.

**AI-V2-002 — Add orchestration layer**
- Tool routing, context limits, cancellation, audit events.
- No direct security implementation access.
- **Done when:** all privileged operations go through approved application interfaces.

**AI-V2-003 — Provider adapter**
- Provider/model integration behind an interface.
- No provider-specific types in domain.
- **Done when:** provider can be replaced without changing domain contracts.

### IOS-V2 — iOS

**IOS-V2-001 — iOS readiness review**
- Confirm shared/security APIs are stable.
- Confirm CI/toolchain requirements.
- **Done when:** explicit go/no-go decision exists.

**IOS-V2-002 — iOS composition root + adapters**
- Consume shared layer; keep Swift/platform concerns at the boundary.
- **Done when:** no duplicated business logic is introduced.

### WEB-V2 — Deferred

**WEB-V2-001 — Web readiness review**
- Reassess Wasm/Compose Web maturity, security boundary fit, and operational requirements.
- **Done when:** explicit ADR/go decision exists.

## 4. PR sequence

Recommended PR granularity:

1. **PR-01:** architecture docs + dependency rules.
2. **PR-02:** security API contract.
3. **PR-03:** Rust/FFI contract and test harness.
4. **PR-04:** minimal KMP shared foundation.
5. **PR-05+:** one vertical slice per PR (identity → messaging → storage → sync).
6. **PR-A:** Android composition/adapters cleanup.
7. **PR-D:** desktop skeleton.
8. **PR-AI:** AI boundary and provider adapter.
9. **PR-I:** iOS skeleton after readiness gate.
10. **PR-W:** web only after explicit approval.

Avoid a single “big KMP migration” PR.

## 5. Definition of Done for each migration PR

- Scope is limited to one boundary or vertical slice.
- Existing behavior is preserved unless an approved ADR changes it.
- Relevant tests are added or updated.
- No required test category is silently skipped.
- Dependency direction is checked.
- Security-sensitive changes receive security review.
- Temporary shims are documented with removal criteria.
- `bash scripts/ci/verify.sh` is run when the change reaches the repository verification stage.
- Human review is required before merge.

## 6. Current status snapshot

### Implemented / present
- Android `:app` exists.
- Architecture documentation exists.
- Agent governance exists.
- Platform strategy document exists.

### Planned / not yet implemented
- KMP shared module.
- Rust security-core module.
- Platform adapter modules.
- Desktop app.
- iOS app.
- AI layer.
- Web target.

### Pending decisions
- ADR-0007 through ADR-0014 remain subject to the repository's stated approval/evidence process unless individually verified otherwise.
- No technical choice should be inferred merely from an ADR title.

## 7. Release gates

### Gate A — Foundation
Architecture boundaries accepted + dependency checks available.

### Gate B — Security
Security API/FFI reviewed + security/cryptography evidence available.

### Gate C — Android migration
Android app uses shared contracts without forbidden dependencies.

### Gate D — Multiplatform
Desktop/iOS consume the same shared architecture without business-logic forks.

### Gate E — AI
AI is isolated, optional, permissioned, and replaceable.

### Gate F — Release
Build, Unit, Integration, Cryptography, Protocol, Security, Static analysis, Dependency checks, Regression, and Fuzz/property categories are each explicitly reported as PASS or PENDING.

## 8. Anti-patterns

Do not:
- create `shared/`, `core/`, `platform/`, `ai/`, `desktopApp/`, and `iosApp/` all in one migration;
- move crypto code before defining its boundary;
- let UI call Rust directly;
- let AI call crypto/storage internals;
- introduce desktop/iOS code by copying Android business logic;
- mark missing test categories as passing;
- convert an architectural diagram into a claim about the current repository state.
