//! Solana user-decryption authorization: permit reconstruction, deployment identity, the
//! atomic state snapshot, and the per-handle rules.
//!
//! The shape of this module tree is load-bearing rather than organizational. Every check is
//! a pure function of `(typed request, snapshot, deployment, now)`, and the only place that
//! reads host state is [`snapshot`]. Two properties follow from that split, and neither
//! survives if it is blurred:
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
//! typed form, canonical text, envelope, signature) and the ACL model (the
//! `zama-solana-acl` crate — account layout, encrypted value IDs, leaf commitments, MMR). Neither
//! is reimplemented here; this module is the host policy that consumes both.

//! # What this module replaces
//!
//! The proof-of-concept Solana user-decryption path verified a wallet signature over an ad-hoc
//! binary preimage, carried its authorization material inside a versioned `extraData` blob, and
//! authorized exactly one handle per request. All three are replaced here: the signature is the
//! permit envelope, the material is typed request fields, and a request carries as many handles
//! as the bit budget allows.
//!
//! This module is additive while that path is still wired up, so both exist for the moment and
//! the old one keeps working. The examples that assert the replaced surface is *gone* — the old
//! signature verifier, the single-handle restriction, and the delegation verifier that pinned a
//! counter — belong with the change that deletes it: they would fail by construction until then,
//! which would make them noise rather than a pin.
//!
//! What can be stated now is what the replacements are, and this example is compiled:
//!
//! ```
//! use kms_worker::core::solana::{delegation::check_delegation, pipeline::check_signature};
//! let _ = check_signature;
//! let _ = check_delegation;
//! ```

/// Delegation-record freshness.
pub mod delegation;
/// Deployment identity: which program, which cluster.
pub mod deployment;
/// Encrypted value account resolution: presence, ownership, type, identity binding, app context.
pub mod encrypted_value_account;
/// Failure taxonomy and the terminal / transient / retryable classification.
pub mod failure;
/// Handle binding: current membership and historical inclusion proofs.
pub mod handle_binding;
/// KMS context/epoch servability.
pub mod kms_pair;
/// The authorization pipeline.
pub mod pipeline;
/// The normalized request and its strict decoding.
pub mod request;
/// The signed ACL-domain scope.
pub mod scope;
/// The atomic host-state snapshot — the only reader of chain state.
pub mod snapshot;
/// Permit-invalidation watermark.
pub mod watermark;
