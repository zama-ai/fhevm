//! Solana user-decryption authorization: permit reconstruction, deployment identity, the
//! atomic state snapshot, the leaf-proof read, and the per-handle rules.
//!
//! The shape of this module tree is part of the contract, not organization. Every check is
//! a pure function of `(typed request, snapshot, proofs, deployment, now)`; the only place that
//! reads host state is [`snapshot`], and the only place that reads the coprocessors' leaf
//! record is [`proof`]. Two properties follow from that split, and neither survives if it is
//! blurred:
//!
//! * race behaviour is testable without a network — a scenario is two snapshot values, not
//!   two moments in time;
//! * "re-check the state just before handing the request to the KMS" cannot be written,
//!   because a check has no way to read anything. A request accepted at its observation
//!   point is accepted; nothing downstream can reopen it.
//!
//! Failure classification is an enumeration ([`failure::FailureClass`]) and every taxonomy
//! in this tree is matched exhaustively — a new variant breaks the build instead of landing
//! in a catch-all arm that silently picks someone else's retry policy.
//!
//! What lives above this module: the permit canon itself (the `zama-solana-permit` crate —
//! typed form, canonical text, envelope, signature), the request canon (the
//! `zama-solana-request` crate — the wire form and the one encoder/decoder the relayer and
//! this connector share), and the ACL model (the `zama-solana-acl` crate — account layout,
//! seeds, leaf commitments, MMR). None is reimplemented here; this module is the host policy
//! that consumes all three.

/// Delegation-record freshness.
pub mod delegation;
/// Deployment identity: which program, which cluster.
pub mod deployment;
/// Encrypted value account resolution: presence, ownership, type, address binding, and the
/// authority and application the account carries.
pub mod encrypted_value_account;
/// Parity between the gateway event's typed fields and the signed request they carry.
pub mod event_parity;
/// Failure taxonomy and the terminal / transient / retryable classification.
pub mod failure;
/// Handle binding: a sealed leaf proven against the account's own peaks.
pub mod handle_binding;
/// KMS context/epoch servability.
pub mod kms_pair;
/// The host pause switch.
pub mod pause;
/// The authorization pipeline.
pub mod pipeline;
/// The leaf-proof reader — the only reader of the coprocessors' record.
pub mod proof;
/// The normalized request and its strict decoding.
pub mod request;
/// The signed application scope.
pub mod scope;
/// The atomic host-state snapshot — the only reader of chain state.
pub mod snapshot;
/// Permit-invalidation watermark.
pub mod watermark;
