//! The normalized user-decryption request and its strict decoding.
//!
//! Transport carries whatever the sender chose; this module is the boundary where that
//! becomes typed. The wire form itself is not defined here — it is the shared canon in
//! `zama-solana-request`, which the relayer fills in and this connector reads, so the two
//! cannot hold different opinions about the layout. What is local is the validated type: it
//! has no public constructor, so "authorize a request nobody validated" is not expressible.
//!
//! Three absences are deliberate. No `encrypted_value_account_authority` field and no
//! `(program, scope)` field, in either the wire form or the validated form: both are properties
//! of the handle's encrypted value account, and the only way to learn them is to read and
//! validate that account. A request cannot name them, so a substituted authority is not a check
//! that can be forgotten — it is a value that does not exist. And no proof: the leaf proof that
//! binds a key to a handle is fetched from the coprocessor's leaf record by the pipeline and
//! verified against the account's own peaks. The wire half of these guarantees is pinned by the
//! compile-fail pair in `zama-solana-request`.

use crate::core::solana_acl::{HandleBytes, SolanaPubkeyBytes};
use zama_solana_permit::{PermitFields, Signature};

pub use zama_solana_request::{
    MAX_REQUEST_HANDLES, SolanaHandleEntryWire, SolanaUserDecryptRequestWire,
};

/// One validated handle entry.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SolanaHandleEntry {
    handle: HandleBytes,
    allowed_key: SolanaPubkeyBytes,
    encrypted_value_account: SolanaPubkeyBytes,
}

impl SolanaHandleEntry {
    /// The exact handle this entry names. Never resolved to whatever is currently live.
    pub fn handle(&self) -> HandleBytes {
        self.handle
    }

    /// The key whose allow leaf on the handle authorizes this entry. It selects the direct or
    /// delegated branch — equal to the requester in the first, the delegator in the second — and
    /// in both it is the key the leaf must name.
    pub fn allowed_key(&self) -> SolanaPubkeyBytes {
        self.allowed_key
    }

    /// The encrypted value account this entry qualifies under, as named by the request. Read
    /// and validated before anything is taken from it.
    pub fn encrypted_value_account(&self) -> SolanaPubkeyBytes {
        self.encrypted_value_account
    }
}

/// A request whose typed form has been validated: identity widths, the permit's own typed
/// rules, and a non-empty handle list.
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
    /// identities, and a non-empty handle list.
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

/// Validates one entry's identities.
fn decode_entry(
    index: usize,
    entry: &SolanaHandleEntryWire,
) -> Result<SolanaHandleEntry, RequestFormError> {
    Ok(SolanaHandleEntry {
        handle: entry_identity(index, EntryField::Handle, &entry.handle)?,
        allowed_key: entry_identity(index, EntryField::AllowedKey, &entry.allowed_key)?,
        encrypted_value_account: entry_identity(
            index,
            EntryField::EncryptedValueAccount,
            &entry.encrypted_value_account,
        )?,
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
    /// The key whose allow leaf authorizes the entry.
    AllowedKey,
    /// The encrypted value account address.
    EncryptedValueAccount,
}
