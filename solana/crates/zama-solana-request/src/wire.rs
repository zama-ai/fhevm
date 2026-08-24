//! The request as it arrives, before anything about it has been established.
//!
//! Widths are `Vec<u8>` because a wrong width has to be representable somewhere: this is the
//! form a sender controls, and the consumer that authorizes turns it into its own validated
//! type. Nothing here is trusted and nothing here is checked.
//!
//! Two absences are deliberate: no `encrypted_value_account_authority` field and no
//! `acl_domain` field. Both are properties of the handle's encrypted value account, and the
//! only way to learn either is to read and validate that account. A request cannot name them,
//! so a substituted authority is not a check that can be forgotten — it is a value that does
//! not exist.
//!
//! # The authority is not a request field
//!
//! Naming the authority in a request must stay a compile error, because the delegated branch
//! is looked up by it: a request that could name one could name an authority the signer does
//! hold a delegation for, against an encrypted value account belonging to somebody else.
//!
//! ```compile_fail
//! use zama_solana_request::SolanaHandleEntryWire;
//!
//! let entry = SolanaHandleEntryWire {
//!     handle: vec![0; 32],
//!     subject: vec![0; 32],
//!     encrypted_value_id: vec![0; 32],
//!     proof_leaf_count: 0,
//!     access_proof: Vec::new(),
//!     encrypted_value_account_authority: vec![0; 32],
//! };
//! ```
//!
//! The same literal without that field compiles. The pair matters: a `compile_fail` example
//! passes when compilation fails for *any* reason, so on its own it would also pass on a typo.
//!
//! ```
//! use zama_solana_request::SolanaHandleEntryWire;
//!
//! let entry = SolanaHandleEntryWire {
//!     handle: vec![0; 32],
//!     subject: vec![0; 32],
//!     encrypted_value_id: vec![0; 32],
//!     proof_leaf_count: 0,
//!     access_proof: Vec::new(),
//! };
//! ```

use zama_solana_permit::PermitWireFields;

/// Upper bound on handle entries accepted from a request.
///
/// Every rule is evaluated against one atomic `getMultipleAccounts` snapshot, and a standard
/// Solana RPC node serves at most 100 accounts per call. The worst-case request needs three
/// accounts per entry — the encrypted value account plus the two delegation rows — and the
/// signer's invalidation record on top: `3 * N + 1 <= 100` gives 33.
///
/// It lives here, next to the wire form, because both ends need the same number: the relayer
/// refuses an oversized request before it submits one, and the connector refuses one that
/// reached it anyway. Two copies of this constant would be two different caps the day one of
/// them moved.
pub const MAX_REQUEST_HANDLES: usize = 33;

/// Upper bound on the sibling list of an access proof accepted from an untrusted request,
/// matching the MMR's `u64` height ceiling. Bounds the decode-time allocation.
///
/// Here for the same reason as [`MAX_REQUEST_HANDLES`]: the relayer refuses an overlong
/// proof before submitting, the connector refuses one that arrived anyway, and the two must
/// refuse the same proofs.
pub const MAX_ACCESS_PROOF_SIBLINGS: usize = 64;

/// The request as it arrives: permit fields, the signature over their envelope, and the
/// handle entries.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct SolanaUserDecryptRequestWire {
    /// The eight signed permit fields, in transport form.
    pub permit: PermitWireFields,
    /// Claimed Ed25519 signature over the reconstructed envelope.
    pub signature: Vec<u8>,
    /// Handle entries, in request order.
    pub handles: Vec<SolanaHandleEntryWire>,
}

/// One handle entry as it arrives. None of these fields are signed: they are evidence,
/// self-authenticating against host state, and a substituted value can fail the request but
/// never widen access.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct SolanaHandleEntryWire {
    /// Claimed 32-byte ciphertext handle.
    pub handle: Vec<u8>,
    /// Claimed 32-byte subject: the pubkey whose encrypted value this entry asks to decrypt —
    /// the requester itself for a direct entry, the delegator for a delegated one.
    pub subject: Vec<u8>,
    /// Claimed 32-byte encrypted value account identity.
    pub encrypted_value_id: Vec<u8>,
    /// The `leaf_count` the access proof was built against; 0 in current mode. Diagnostic
    /// only — it classifies an already-failed inclusion check and never decides one.
    pub proof_leaf_count: u64,
    /// Empty for current access; otherwise borsh `MmrProof`.
    pub access_proof: Vec<u8>,
}
