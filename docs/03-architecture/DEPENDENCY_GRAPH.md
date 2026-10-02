# Eagle Dependency Graph

**Status:** Proposed planning artifact  
**Governing issue:** ARCH-001  
**Governing ADR:** ADR-0007

## Graph

```mermaid
graph TD
  CORE[CORE Contracts]
  ID[IDENTITY]
  CR[CRYPTO]
  ST[STORAGE]
  PR[PROTOCOL]
  ME[MESH]
  UI[UI]
  IN[INTEGRATION]
  SE[SECURITY]
  OB[OBSERVABILITY]
  CORE --> ID
  CORE --> CR
  CORE --> ST
  CORE --> PR
  CORE --> ME
  CORE --> UI
  ID --> CR
  CR --> PR
  PR --> ME
  ME --> UI
  CR --> UI
  IN --> ID
  IN --> CR
  IN --> ST
  IN --> PR
  IN --> ME
  IN --> UI
  SE -. audit .-> ID
  SE -. audit .-> CR
  SE -. audit .-> PR
  SE -. audit .-> ST
  SE -. audit .-> ME
  OB -. sanitized events .-> CORE
```

## Core constraints
- Mesh never handles application plaintext.
- Protocol never handles application plaintext.
- Storage never exposes private keys.
- UI never accesses private keys or crypto internals.
- Crypto and Mesh must not form a circular dependency.
- Storage does not define Protocol.

Solid edges are dependency direction; dashed edges are audit relationships.