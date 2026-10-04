//! Reference implementations for Eagle core contracts.
//!
//! These are minimal deterministic implementations used for contract and
//! integration testing. They are not production persistence or transport
//! engines and must not appear in a production dependency graph.

#![forbid(unsafe_code)]

pub mod record_store;
pub mod transport;
