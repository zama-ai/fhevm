//! The normalized user-decryption request and its strict decoding.
//!
//! Transport carries whatever the sender chose; this module is the boundary where that
//! becomes typed. The pattern mirrors the permit crate: a wire type whose widths are
//! `Vec<u8>` because a wrong width has to be representable somewhere, and a validated type
//! with no public constructor, so "authorize a request nobody validated" is not expressible.
//!
//! Two absences are deliberate and are the whole of rule h6 on the request side: no
//! `encrypted_value_account_authority` field and no `acl_domain` field, in either the wire form or the validated
//! form. The app context of a handle is a property of its lineage account, and the only way
//! to learn it is to read and validate that account. A request cannot name it, so a
//! substituted app context is not a check that can be forgotten — it is a value that does
//! not exist.
//!
//! # The app context is not a request field
//!
//! Naming an app context in a request must stay a compile error, because the delegated branch
//! is looked up by app account: a request that could name one could name an app the signer
//! does hold a delegation for, against a lineage belonging to somebody else's application.
//!
//! ```compile_fail
//! use kms_worker::core::solana::request::SolanaHandleEntryWire;
//!
//! let entry = SolanaHandleEntryWire {
//!     handle: vec![0; 32],
//!     owner: vec![0; 32],
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
//! use kms_worker::core::solana::request::SolanaHandleEntryWire;
//!
//! let entry = SolanaHandleEntryWire {
//!     handle: vec![0; 32],
//!     owner: vec![0; 32],
//!     encrypted_value_id: vec![0; 32],
//!     proof_leaf_count: 0,
//!     access_proof: Vec::new(),
//! };
//! ```

use crate::core::solana_acl::{HandleBytes, SolanaPubkeyBytes};
use borsh::BorshDeserialize;
use zama_solana_acl::MmrProof;
use zama_solana_permit::{PermitFields, PermitWireFields, Signature};

/// Upper bound on `MmrProof::siblings` accepted from an untrusted request, matching the
/// MMR's `u64` height ceiling. Bounds the decode-time allocation.
pub const MAX_ACCESS_PROOF_SIBLINGS: usize = 64;

/// Upper bound on handle entries accepted from a request.
///
/// Every rule is evaluated against one atomic `getMultipleAccounts` snapshot, and a standard
/// Solana RPC node serves at most 100 accounts per call. The worst-case request needs three
/// accounts per entry — the lineage account plus the two delegation rows — and the signer's
/// invalidation record on top: `3 * N + 1 <= 100` gives 33. The Gateway entry point enforces
/// the same cap at admission, before the fee, so a request that exists as an event already
/// satisfies it; this bound keeps the snapshot's readability a property of this module rather
/// than an assumption about the contract version upstream.
pub const MAX_REQUEST_HANDLES: usize = 33;

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
    /// Claimed 32-byte ciphertext owner: the signer for a direct entry, the delegator for a
    /// delegated one.
    pub owner: Vec<u8>,
    /// Claimed 32-byte lineage identity.
    pub encrypted_value_id: Vec<u8>,
    /// The `leaf_count` the access proof was built against; 0 in current mode. Diagnostic
    /// only — it classifies an already-failed inclusion check and never decides one.
    pub proof_leaf_count: u64,
    /// Empty for current access; otherwise borsh `MmrProof`.
    pub access_proof: Vec<u8>,
}

/// How an entry claims access to its handle. The mode is per entry: one request freely
/// mixes both.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AccessEvidence {
    /// No proof: the handle must be the lineage's current handle and the subject a current
    /// member.
    Current,
    /// An inclusion proof for a replaced handle, verified against the snapshot's live
    /// peaks.
    Historical(MmrProof),
}

/// One validated handle entry.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SolanaHandleEntry {
    handle: HandleBytes,
    owner: SolanaPubkeyBytes,
    encrypted_value_id: [u8; 32],
    proof_leaf_count: u64,
    access: AccessEvidence,
}

impl SolanaHandleEntry {
    /// The exact handle this entry names. Never resolved to whatever is currently live.
    pub fn handle(&self) -> HandleBytes {
        self.handle
    }

    /// The ciphertext owner, which selects the direct or delegated branch and, in both,
    /// names the subject whose access is then proven.
    pub fn owner(&self) -> SolanaPubkeyBytes {
        self.owner
    }

    /// The lineage this entry qualifies under.
    pub fn encrypted_value_id(&self) -> [u8; 32] {
        self.encrypted_value_id
    }

    /// The leaf count the proof was built against, for failure classification only.
    pub fn proof_leaf_count(&self) -> u64 {
        self.proof_leaf_count
    }

    /// Current or historical access.
    pub fn access(&self) -> &AccessEvidence {
        &self.access
    }
}

/// A request whose typed form has been validated: identity widths, the permit's own typed
/// rules, the access-proof form, and a non-empty handle list.
///
/// No public constructor: [`SolanaUserDecryptRequest::decode`] is the only way in.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SolanaUserDecryptRequest {
    permit: PermitFields,
    signature: Signature,
    handles: Vec<SolanaHandleEntry>,
}

impl SolanaUserDecryptRequest {
    /// Strictly decodes the transport form.
    ///
    /// Covers the permit's own typed rules (delegated to the permit crate, so the Connector
    /// cannot be softer or stricter than any other verifier), the widths of the entry
    /// identities, the access-proof form (strict borsh, no trailing bytes, bounded sibling
    /// count), and a non-empty handle list.
    ///
    /// What it deliberately does not do: verify the signature, or look at any clock or
    /// account. Those are the authorization layer.
    ///
    /// It also does not re-check the request's bit budget or the FHE types of its handles. The
    /// Gateway entry point this request came through sums the per-handle bit widths on chain and
    /// reverts past the budget or on a type with no width, so a request that exists has already
    /// passed both — and the EVM path likewise does not re-adjudicate them here. Re-checking
    /// would put a second copy of that table in this Connector and let it reject, terminally and
    /// after the fee was paid, a request the Gateway accepted.
    ///
    /// Two size-shaped rules stay, and neither is a mirror of the Gateway's budget — both are
    /// preconditions of this module's own operation. The empty list is rejected because a
    /// request with no entries would authorize nothing and still be accepted. The handle count
    /// is capped at [`MAX_REQUEST_HANDLES`] because the snapshot every rule reads is one
    /// `getMultipleAccounts` call; the Gateway enforces the same cap at admission, so this arm
    /// is unreachable through it and exists to keep the invariant local.
    pub fn decode(wire: &SolanaUserDecryptRequestWire) -> Result<Self, RequestFormError> {
        let permit = PermitFields::decode(&wire.permit)?;
        let signature: [u8; zama_solana_permit::SIGNATURE_LEN] = wire
            .signature
            .as_slice()
            .try_into()
            .map_err(|_| RequestFormError::SignatureWidth {
                len: wire.signature.len(),
            })?;
        if wire.handles.is_empty() {
            return Err(RequestFormError::EmptyHandles);
        }
        if wire.handles.len() > MAX_REQUEST_HANDLES {
            return Err(RequestFormError::TooManyHandles {
                handles: wire.handles.len(),
            });
        }
        let handles = wire
            .handles
            .iter()
            .enumerate()
            .map(|(index, entry)| decode_entry(index, entry))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            permit,
            signature: Signature::new(signature),
            handles,
        })
    }

    /// The validated permit.
    pub fn permit(&self) -> &PermitFields {
        &self.permit
    }

    /// The signature over the reconstructed envelope.
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    /// The handle entries, in request order. Order and count are preserved verbatim:
    /// duplicates are legal and each occurrence is authorized independently.
    pub fn handles(&self) -> &[SolanaHandleEntry] {
        &self.handles
    }
}

/// Validates one entry's identities and its access evidence.
fn decode_entry(
    index: usize,
    entry: &SolanaHandleEntryWire,
) -> Result<SolanaHandleEntry, RequestFormError> {
    Ok(SolanaHandleEntry {
        handle: entry_identity(index, EntryField::Handle, &entry.handle)?,
        owner: entry_identity(index, EntryField::Owner, &entry.owner)?,
        encrypted_value_id: entry_identity(index, EntryField::EncryptedValueId, &entry.encrypted_value_id)?,
        proof_leaf_count: entry.proof_leaf_count,
        access: decode_access_evidence(index, &entry.access_proof)?,
    })
}

/// One 32-byte identity, named by its field so a wrong width says which one.
fn entry_identity(
    index: usize,
    field: EntryField,
    bytes: &[u8],
) -> Result<[u8; 32], RequestFormError> {
    bytes
        .try_into()
        .map_err(|_| RequestFormError::EntryIdentityWidth {
            index,
            field,
            len: bytes.len(),
        })
}

/// Chooses the access mode from the proof bytes: absent is the current-handle claim, present is
/// a strictly decoded inclusion proof.
///
/// Strict means all three of: it decodes, nothing follows it, and its sibling list is within
/// what the tree can produce. The tail is a rejection here, unlike an account's tail, because
/// two byte strings for one proof would give two implementations two different answers about
/// whether they hold the same request.
fn decode_access_evidence(
    index: usize,
    access_proof: &[u8],
) -> Result<AccessEvidence, RequestFormError> {
    if access_proof.is_empty() {
        return Ok(AccessEvidence::Current);
    }
    let mut remaining = access_proof;
    let proof = MmrProof::deserialize(&mut remaining)
        .map_err(|_| RequestFormError::AccessProofMalformed { index })?;
    if !remaining.is_empty() {
        return Err(RequestFormError::AccessProofTrailingBytes {
            index,
            trailing: remaining.len(),
        });
    }
    if proof.siblings.len() > MAX_ACCESS_PROOF_SIBLINGS {
        return Err(RequestFormError::AccessProofTooManySiblings {
            index,
            siblings: proof.siblings.len(),
        });
    }
    Ok(AccessEvidence::Historical(proof))
}

/// Why a request's typed form was rejected.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum RequestFormError {
    /// A permit field violated its own typed rule.
    #[error("permit field: {0}")]
    Permit(#[from] zama_solana_permit::PermitError),
    /// The signature was not 64 bytes.
    #[error("signature is {len} bytes, expected 64")]
    SignatureWidth {
        /// The width that arrived.
        len: usize,
    },
    /// An entry identity was not 32 bytes.
    #[error("entry {index} field {field:?} is {len} bytes, expected 32")]
    EntryIdentityWidth {
        /// Which entry.
        index: usize,
        /// Which field of it.
        field: EntryField,
        /// The width that arrived.
        len: usize,
    },
    /// An access proof did not strictly borsh-decode.
    #[error("entry {index} access proof does not decode as an MMR proof")]
    AccessProofMalformed {
        /// Which entry.
        index: usize,
    },
    /// An access proof decoded, but bytes remained. Two byte strings for one proof would
    /// split deduplication and diagnostics between implementations, so the tail is a
    /// rejection here — unlike an account's tail, which is legal.
    #[error("entry {index} access proof carries {trailing} trailing byte(s)")]
    AccessProofTrailingBytes {
        /// Which entry.
        index: usize,
        /// How many bytes remained.
        trailing: usize,
    },
    /// An access proof carried more siblings than the MMR can produce.
    #[error("entry {index} access proof carries {siblings} siblings, exceeding the cap")]
    AccessProofTooManySiblings {
        /// Which entry.
        index: usize,
        /// The count that arrived.
        siblings: usize,
    },
    /// The handle list was empty.
    #[error("request names no handles")]
    EmptyHandles,
    /// The handle list exceeds what one atomic account snapshot can cover.
    #[error("request names {handles} handles, exceeding the {MAX_REQUEST_HANDLES}-handle cap")]
    TooManyHandles {
        /// The count that arrived.
        handles: usize,
    },
}

/// Which field of a handle entry carried a wrong width.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntryField {
    /// The ciphertext handle.
    Handle,
    /// The ciphertext owner.
    Owner,
    /// The lineage identity.
    EncryptedValueId,
}
