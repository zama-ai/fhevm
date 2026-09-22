//! Solana account authorization, using the shared permit and ACL crates.

/// Delegation-record freshness.
pub mod delegation;
/// Deployment identity: which program, which cluster.
pub mod deployment;
/// Encrypted store resolution: presence, ownership, type, address binding, and the
/// authority and application the account carries.
pub mod encrypted_store;
/// Authorization failures.
pub mod failure;
/// Handle binding: a sealed leaf proven against the account's own peaks.
pub mod handle_binding;
/// The host pause switch.
pub mod pause;
/// The authorization pipeline.
pub mod pipeline;
/// The leaf-proof reader — the only reader of the coprocessors' record.
pub mod proof;
/// The signed application scope.
pub mod scope;
/// The atomic host-state snapshot — the only reader of chain state.
pub mod snapshot;
/// Permit-invalidation watermark.
pub mod watermark;
