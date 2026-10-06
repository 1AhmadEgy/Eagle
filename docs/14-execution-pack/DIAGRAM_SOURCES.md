# Eagle Execution Pack - Diagram Sources

These diagrams are implementation handoff views. They complement, and do not override, accepted ADRs.

## 1. System context

```mermaid
flowchart LR
  A["Device A<br/>Eagle"] -->|"encrypted + authenticated"| P["P2P Network<br/>untrusted peers"]
  P -->|"ciphertext forwarding"| B["Device B<br/>Eagle"]
  P -. "optional user-controlled relay" .-> P
  C["No central runtime service"] -.-> P
```

## 2. Layered architecture

```mermaid
flowchart TB
  UI["Platform UI"] --> KMP["KMP Shared Application Layer"]
  KMP --> R["Rust Security Core"]
  KMP --> A["Platform Adapters"]
  R --> S["OS Secure Storage / Key Services"]
  R --> D["Local Encrypted Data"]
  A --> N["Network / OS APIs"]
```

## 3. Trust boundaries

```mermaid
flowchart LR
  U["User Input"] --> APP["Application Domain"]
  APP --> R["Rust Security Core"]
  R --> OS["OS Secure Facilities"]
  R --> T["P2P Transport"]
  T --> PEER["Remote Peer"]
  PEER --> UN["Untrusted Network"]
```

## 4. Message lifecycle

```mermaid
flowchart LR
  C["Compose"] --> E["Encrypt"] --> Q["Local Queue"] --> P["P2P Transfer"]
  P --> V["Verify + Decrypt"] --> ST["Store"]
  P -. "disconnect / timeout" .-> F["Fail / Retry"]
  F --> Q
```

## 5. Engineering gate

```mermaid
flowchart TB
  RQ["Requirement"] --> ADR["ADR / Architecture"]
  ADR --> IMPL["Implementation"]
  IMPL --> TEST["Tests"]
  TEST --> SEC["Security Gates"]
  SEC --> REV["Peer Review"]
  REV --> EV["Evidence"]
  EV --> REL["Merge / Release"]
  TEST -. "changes" .-> IMPL
  EV -. "decision update" .-> ADR
```

## 6. Platform boundary

```mermaid
flowchart TB
  RUST["Rust Security Core"] --> KMP["KMP Shared Layer"]
  KMP --> A["Android Adapter"]
  KMP --> D["Desktop Adapter"]
  KMP --> I["iOS Adapter"]
  KMP -.-> W["Web - Deferred"]
```
