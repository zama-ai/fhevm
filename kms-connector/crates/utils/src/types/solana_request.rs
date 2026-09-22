//! Typed Solana user-decrypt data shared by ingress, persistence, and authorization.

use crate::types::handle::extract_chain_id_from_handle;
use alloy::primitives::{B256, U256};
use fhevm_gateway_bindings::decryption::Decryption::UserDecryptionRequest_4;
use zama_solana_permit::PermitWireFields;
use zama_solana_permit::{PermitFields, Signature};

pub use zama_solana_request::{
    MAX_REQUEST_HANDLES, SolanaHandleEntryWire, SolanaUserDecryptRequestWire,
};

/// One validated handle entry.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SolanaHandleEntry {
    handle: [u8; 32],
    allowed_key: [u8; 32],
    encrypted_store: [u8; 32],
}

impl SolanaHandleEntry {
    /// The exact handle this entry names. Never resolved to whatever is currently live.
    pub fn handle(&self) -> [u8; 32] {
        self.handle
    }

    /// The key whose allow leaf on the handle authorizes this entry. It selects the direct or
    /// delegated branch — equal to the requester in the first, the delegator in the second — and
    /// in both it is the key the leaf must name.
    pub fn allowed_key(&self) -> [u8; 32] {
        self.allowed_key
    }

    /// The encrypted store this entry qualifies under, as named by the request. Read
    /// and validated before anything is taken from it.
    pub fn encrypted_store(&self) -> [u8; 32] {
        self.encrypted_store
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

        let first_chain = extract_chain_id_from_handle(&B256::from(handles[0].handle))
            .map_err(|e| RequestFormError::Handle(e.to_string()))?;
        for (index, entry) in handles.iter().enumerate().skip(1) {
            let found = extract_chain_id_from_handle(&B256::from(entry.handle))
                .map_err(|e| RequestFormError::Handle(e.to_string()))?;
            if found != first_chain {
                return Err(RequestFormError::MixedEmbeddedChainIds {
                    index,
                    found,
                    expected: first_chain,
                });
            }
        }
        if first_chain != permit.chain_id() {
            return Err(RequestFormError::ChainId {
                declared: permit.chain_id(),
                handle: first_chain,
            });
        }
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
        encrypted_store: entry_identity(index, EntryField::EncryptedStore, &entry.encrypted_store)?,
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
    #[error("invalid handle: {0}")]
    Handle(String),
    #[error("signed chain id {declared} does not match handle chain id {handle}")]
    ChainId { declared: u64, handle: u64 },
    #[error("handle {index} embeds chain id {found}, an earlier handle embeds {expected}")]
    MixedEmbeddedChainIds {
        index: usize,
        found: u64,
        expected: u64,
    },
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
    /// The encrypted store address.
    EncryptedStore,
}

/// The connector request ID and the validated Solana attestation body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaUserDecryptionRequestV1 {
    pub decryption_id: U256,
    pub request: SolanaUserDecryptRequest,
}

impl TryFrom<UserDecryptionRequest_4> for SolanaUserDecryptionRequestV1 {
    type Error = anyhow::Error;

    fn try_from(event: UserDecryptionRequest_4) -> anyhow::Result<Self> {
        let wire = zama_solana_request::decode_solana_request(&event.solanaRequest)?;
        let request = SolanaUserDecryptRequest::decode(&wire)?;
        let permit = request.permit();
        let handles: Vec<_> = event.ctHandles.iter().map(|h| h.0).collect();
        zama_solana_request::check_handle_list_parity(&handles, &wire)?;
        anyhow::ensure!(
            event.publicKey.as_ref() == permit.transport_key().as_bytes(),
            "event publicKey does not match the signed transport key"
        );
        anyhow::ensure!(
            event.requestValidity.startTimestamp == U256::from(permit.start_timestamp())
                && event.requestValidity.durationSeconds == U256::from(permit.duration_seconds()),
            "event requestValidity does not match the signed window"
        );
        anyhow::ensure!(
            event.extraData.as_ref() == permit.extra_data().to_extra_data(),
            "event extraData does not match the signed KMS routing"
        );
        Ok(Self {
            decryption_id: event.decryptionId,
            request,
        })
    }
}

impl SolanaUserDecryptRequest {
    pub fn from_row(row: &sqlx::postgres::PgRow) -> anyhow::Result<Self> {
        use sqlx::Row;
        let handles: Vec<Vec<u8>> = row.try_get("ct_handles")?;
        let allowed_keys: Vec<Vec<u8>> = row.try_get("allowed_keys")?;
        let encrypted_stores: Vec<Vec<u8>> = row.try_get("encrypted_stores")?;
        anyhow::ensure!(
            handles.len() == allowed_keys.len() && handles.len() == encrypted_stores.len(),
            "Solana handle/key/store array length mismatch"
        );
        let first = handles
            .first()
            .ok_or_else(|| anyhow::anyhow!("request names no handles"))?;
        let chain_id = extract_chain_id_from_handle(&B256::try_from(first.as_slice())?)?;
        let wire = SolanaUserDecryptRequestWire {
            permit: PermitWireFields {
                user_pubkey: row.try_get("user_pubkey")?,
                transport_key: row.try_get("public_key")?,
                allowed_scopes: row.try_get("allowed_scopes")?,
                start_timestamp: u64::try_from(row.try_get::<i64, _>("start_timestamp")?)?,
                duration_seconds: u64::try_from(row.try_get::<i64, _>("duration_seconds")?)?,
                verifying_program_id: row.try_get("host_program_id")?,
                chain_id,
                extra_data: row.try_get("extra_data")?,
            },
            signature: row.try_get("signature")?,
            handles: handles
                .into_iter()
                .zip(allowed_keys)
                .zip(encrypted_stores)
                .map(
                    |((handle, allowed_key), encrypted_store)| SolanaHandleEntryWire {
                        handle,
                        allowed_key,
                        encrypted_store,
                    },
                )
                .collect(),
        };
        Ok(Self::decode(&wire)?)
    }
}

/// Both ingress paths store the same authorization columns. Only HTTP retries reset failed work.
pub async fn insert_solana_user_decryption<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    request: &SolanaUserDecryptionRequestV1,
    tx_hash: Option<B256>,
    created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    otlp_context: &crate::monitoring::otlp::PropagationContext,
    source: super::db::RequestSource,
) -> anyhow::Result<sqlx::postgres::PgQueryResult> {
    let body = &request.request;
    let permit = body.permit();
    let handles: Vec<Vec<u8>> = body.handles().iter().map(|e| e.handle().to_vec()).collect();
    let keys: Vec<Vec<u8>> = body
        .handles()
        .iter()
        .map(|e| e.allowed_key().to_vec())
        .collect();
    let stores: Vec<Vec<u8>> = body
        .handles()
        .iter()
        .map(|e| e.encrypted_store().to_vec())
        .collect();
    let scopes: Vec<Vec<u8>> = permit
        .allowed_scopes()
        .as_slice()
        .iter()
        .map(|s| s.as_bytes().to_vec())
        .collect();
    Ok(sqlx::query!(
        "INSERT INTO user_decryption_requests AS existing (
            decryption_id, attestation_type, ct_handles, public_key, extra_data, signature,
            start_timestamp, duration_seconds, user_pubkey, allowed_keys, encrypted_stores,
            allowed_scopes, host_program_id, tx_hash, created_at, otlp_context, source
        ) VALUES ($1, 'solana-srfc38-user-decrypt-v1', $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
        ON CONFLICT (decryption_id) DO UPDATE SET
            status = 'pending', created_at = EXCLUDED.created_at, otlp_context = EXCLUDED.otlp_context
        WHERE existing.status = 'failed' AND existing.source = 'http' AND EXCLUDED.source = 'http'",
        request.decryption_id.as_le_slice(), &handles, permit.transport_key().as_bytes().as_slice(),
        permit.extra_data().to_extra_data(), body.signature().as_bytes().as_slice(),
        i64::try_from(permit.start_timestamp())?, i64::try_from(permit.duration_seconds())?,
        permit.user_pubkey().as_bytes().as_slice(), &keys, &stores, &scopes,
        permit.verifying_program_id().as_bytes().as_slice(), tx_hash.map(|h| h.to_vec()),
        created_at, bc2wrap::serialize(otlp_context)?, source as super::db::RequestSource,
    ).execute(executor).await?)
}
