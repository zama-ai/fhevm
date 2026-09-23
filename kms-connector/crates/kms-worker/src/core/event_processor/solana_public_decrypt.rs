//! Solana public decryption authorization: a handle is public when a public-decrypt leaf for it
//! is proven against its encrypted store, named by the version-4 `extraData`.

use crate::core::{
    event_processor::SolanaHost,
    solana::{
        HandleBytes,
        encrypted_store::{EncryptedStoreFailure, resolve_encrypted_store},
        handle_binding::{
            HandleBindingFailure, check_public_binding, verify_proofs_with_one_retry,
        },
        proof::{LeafKind, LeafQuery, ProofReadError},
        snapshot::{HostStateReader, SnapshotError, SnapshotKeys},
    },
};
use connector_utils::types::solana_extra_data::parse_solana_public_decrypt_extra_data;

pub async fn check_public_decrypt(
    host: &SolanaHost,
    handle: HandleBytes,
    extra_data: &[u8],
) -> Result<(), PublicDecryptFailure> {
    let extra = parse_solana_public_decrypt_extra_data(extra_data)
        .ok_or(PublicDecryptFailure::MalformedExtraData)?;
    let observation = host
        .reader
        .read_accounts(&SnapshotKeys::new([extra.encrypted_store]))
        .await?;
    let store = resolve_encrypted_store(&observation, host.program_id, extra.encrypted_store)?;
    let query = LeafQuery {
        encrypted_store: store.account_key(),
        handle,
        kind: LeafKind::Public,
    };
    verify_proofs_with_one_retry(&host.proofs, &[(query, ())], |_, outcome| {
        check_public_binding(&store, handle, outcome)
    })
    .await?
    .into_iter()
    .try_for_each(|binding| binding)?;
    Ok(())
}

/// Why a public decryption was not authorized.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum PublicDecryptFailure {
    #[error("Solana public decryption requires the version-4 extraData naming the store")]
    MalformedExtraData,
    #[error("host state: {0}")]
    Snapshot(#[from] SnapshotError),
    #[error("encrypted store: {0}")]
    EncryptedStore(#[from] EncryptedStoreFailure),
    #[error("leaf proofs: {0}")]
    ProofRead(#[from] ProofReadError),
    #[error("handle binding: {0}")]
    HandleBinding(#[from] HandleBindingFailure),
}

impl PublicDecryptFailure {
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::MalformedExtraData => false,
            Self::Snapshot(source) => source.is_recoverable(),
            Self::EncryptedStore(source) => source.is_recoverable(),
            Self::ProofRead(source) => source.is_recoverable(),
            Self::HandleBinding(source) => source.is_recoverable(),
        }
    }
}
