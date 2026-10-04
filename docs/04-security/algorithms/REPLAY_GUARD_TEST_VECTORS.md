# Eagle — Replay Guard Reference Vectors

These vectors define the expected deterministic decisions for ReplayGuard.

They are not cryptographic proof and do not replace authenticated protocol tests. The caller must bind the sequence number to the authenticated session before invoking the guard.

Implementation under test:
app/src/main/java/com/eagle/app/security/ReplayGuard.kt

The current production-design window is 1–64 positions. The guard accepts bounded out-of-order delivery, rejects duplicates, and rejects values outside the remembered window.
