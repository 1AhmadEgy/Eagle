# Eagle Architecture Diagrams

## 1. System Context

~~~mermaid
flowchart LR
    U[User] --> A[Android]
    U --> I[iOS]
    U --> D[Desktop]
    A <--> P2P((Eagle Peer Network))
    I <--> P2P
    D <--> P2P
~~~

## 2. Trust Boundary

~~~mermaid
flowchart TB
    subgraph DEVICE[User Device]
      APP[Application/UI]
      CORE[Security & Protocol Core]
      STORE[Protected Local Storage]
    end
    APP --> CORE
    CORE --> STORE
    CORE <-->|Encrypted transport| NET[Untrusted Network]
    NET <-->|Ciphertext only| PEER[Untrusted Peer]
~~~

## 3. Project Lifecycle

~~~mermaid
flowchart LR
 I[Inventory]-->P[Provenance]-->C[Classification]-->T[Triage]
 T-->A[Analysis]-->R[Reconciliation]-->X[Conflicts]-->G[Gaps]
 G-->CA[Canonical]-->M[Remediation]-->CR[Correction]
 CR-->IM[Implementation]-->TE[Testing]-->SR[Security Review]
 SR-->V[Verification]-->E[Evidence]-->RG[Release Gate]-->RE[Release]
 RE-->PR[Post-Release]-->R2[Recycle]-->I
~~~

## 4. Message path

~~~mermaid
sequenceDiagram
    participant A as Peer A
    participant C as Secure Core
    participant T as P2P Transport
    participant B as Peer B
    A->>C: plaintext
    C->>C: policy + session + encrypt
    C->>T: ciphertext envelope
    T->>B: ciphertext
    B->>C: ciphertext envelope
    C->>C: verify + decrypt + policy
~~~

## 5. Release evidence

~~~mermaid
flowchart TB
    REQ[Requirement]-->CODE[Implementation]
    CODE-->TEST[Test]
    CODE-->SEC[Security Review]
    TEST-->VER[Verification]
    SEC-->VER
    VER-->EVID[Evidence]
    EVID-->GATE[Release Gate]
    GATE-->REL[Release]
~~~

الرسوم تصف التصميم المستهدف؛ الفروق مع التنفيذ تسجل كفجوة أو ADR.
