# Stage 1 Exit Gates — Layered Foundation Integration

The historical foundation branch is integrated through reviewable layers A through E. Historical results from PR #41 remain attributable to the original branch and are not reused as proof for a new layer.

## Required order

- [ ] A — documentation and ADR-0015
- [ ] B — CI and repository verification
- [ ] C — Rust Core and FFI
- [ ] D — KMP shared layer and SessionCoordinator
- [ ] E — Android application integration

A dependent layer is not accepted before its prerequisite is merged to `main`.

## Evidence rule

Each checked item requires independently inspectable repository or CI evidence tied to the tested commit.
