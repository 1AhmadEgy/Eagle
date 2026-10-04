# ADR-0015 Implementation Notes

This record captures implementation constraints for the proposed Rust FFI + KMP boundary.

- core/ffi uses UniFFI proc-macros and uniffi::setup_scaffolding!.
- No build.rs is required for the proc-macro scaffolding path; binding generation is a separate tooling step.
- Generated Kotlin bindings are written to shared/build/generated/uniffi/kotlin and are not checked into source control.
- commonMain exposes EagleCore, SessionHandle, and domain state only.
- Platform adapters are intentionally deferred until the Rust native library packaging target is defined for Android, desktop, and iOS.
- The existing Android application remains on its current Activity shell; adding shared is dependency-only in this phase.
