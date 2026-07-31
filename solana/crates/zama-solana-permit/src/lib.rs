//! Canonical form of the Zama fhevm Solana user-decrypt permit.
//!
//! One implementation of the permit canon for every Rust consumer: typed fields,
//! strict decoding of the transport form, the canonical text, the offchain-message
//! envelope, Ed25519 verification, and the transport-key fingerprint.
//!
//! The public surface is deliberately narrow, so that unverifiable paths are not
//! expressible: verification accepts only strictly decoded typed fields plus a
//! signature. There is no entry point taking a caller-supplied canonical text, a
//! caller-supplied envelope, or a caller-supplied transport-key fingerprint —
//! "verify the signature against the text the client sent me" cannot be written
//! against this API. The text and the envelope exist only as outputs, reconstructed
//! locally from the typed fields.
//!
//! Nothing here reads chain state or a clock. Validity-window, invalidation,
//! deployment-identity, KMS-pair and scope checks are the authorization layer and
//! live in the consumer that owns live state.
//!
//! # The supported path
//!
//! Decode the transport form, then verify. There is one way in and it validates:
//!
//! ```
//! use zama_solana_permit::{PermitFields, PermitWireFields, Signature, verify_signature};
//!
//! # fn permit_from_the_wire() -> PermitWireFields { PermitWireFields::default() }
//! # fn signature_from_the_wire() -> Signature { Signature::new([0u8; 64]) }
//! let wire: PermitWireFields = permit_from_the_wire();
//! let signature: Signature = signature_from_the_wire();
//!
//! if let Ok(fields) = PermitFields::decode(&wire) {
//!     let _ = verify_signature(&fields, &signature);
//! }
//! ```
//!
//! # Paths that must not compile
//!
//! The three checks below are the enforcement of "verification is reconstruction". Each
//! is a shape that would let a caller decide what was signed; each must stay a
//! compile error. They are the mirror image of the signature pins in the test suite —
//! those assert what the API *is*, these assert what it is not.
//!
//! Assembling validated fields without decoding them:
//!
//! ```compile_fail
//! use zama_solana_permit::{Identity, PermitFields};
//!
//! // `PermitFields` has no public constructor and no public fields: skipping strict
//! // decoding is not expressible.
//! let fields = PermitFields {
//!     user_pubkey: Identity::new([0u8; 32]),
//! };
//! ```
//!
//! Verifying against a caller-supplied text:
//!
//! ```compile_fail
//! use zama_solana_permit::{PermitError, PermitFields, Signature, verify_signature};
//!
//! // No entry point takes the signed text. If this coercion ever compiles, a caller
//! // can have their own bytes verified instead of the reconstructed ones.
//! let verify_against_text: fn(&PermitFields, &str, &Signature) -> Result<(), PermitError> =
//!     verify_signature;
//! ```
//!
//! Rendering from a caller-supplied fingerprint:
//!
//! ```compile_fail
//! use zama_solana_permit::{render_canonical_text, PermitFields};
//!
//! // The fingerprint is derived from the full key, never accepted as an input.
//! let render_with_fingerprint: fn(&PermitFields, [u8; 32]) -> String = render_canonical_text;
//! ```

/// Envelope construction and Ed25519 verification.
pub mod envelope;
/// Rejection reasons produced by strict decoding and verification.
pub mod error;
/// Transport-key fingerprint committed by the canonical text.
pub mod fingerprint;
/// Canonical text rendering.
pub mod render;
/// Typed permit fields and their transport form.
pub mod types;
/// Strict decoding of the transport form into typed fields.
pub mod validate;

pub use envelope::{build_envelope, verify_signature};
pub use error::{IdentityField, PermitError};
pub use fingerprint::transport_key_fingerprint;
pub use render::render_canonical_text;
pub use types::{
    AclDomainKeys, Identity, KmsRouting, PermitFields, PermitWireFields, Signature, TransportKey,
    IDENTITY_LEN, KMS_ROUTING_EXTRA_DATA_LEN, KMS_ROUTING_VERSION_BYTE, MAX_ACL_DOMAIN_KEYS,
    MAX_DURATION_SECONDS, MAX_START_TIMESTAMP, MIN_DURATION_SECONDS, SIGNATURE_LEN,
    TRANSPORT_KEY_LEN,
};
