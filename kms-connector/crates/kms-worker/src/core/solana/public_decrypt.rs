//! Solana public decryption authorization: a handle is public when a public-decrypt leaf for it
//! is proven against the encrypted store the request names for it.

use super::SolanaHost;
use super::encrypted_store::{EncryptedStoreFailure, resolve_encrypted_store};
use super::handle_binding::{HandleBindingFailure, check_public_binding, verify_proofs};
use super::proof::{LeafKind, LeafQuery, ProofReadError};
use super::snapshot::{SnapshotError, read_positional};
use connector_utils::types::solana_request::SolanaPublicDecryptionRequest;

/// Reads every named store in one snapshot, then proves each handle's leaf against its store. Every
/// store is judged before a coprocessor is asked, as for a user decryption.
pub async fn check_public_decrypt(
    host: &SolanaHost,
    request: &SolanaPublicDecryptionRequest,
) -> Result<(), PublicDecryptFailure> {
    let store_keys: Vec<_> = request
        .handles()
        .iter()
        .map(|entry| entry.encrypted_store)
        .collect();
    let read = read_positional(&host.reader, &store_keys, None).await?;
    let stores = store_keys
        .iter()
        .zip(&read.accounts)
        .enumerate()
        .map(|(index, (key, account))| {
            resolve_encrypted_store(account.as_ref(), host.program_id, *key)
                .map_err(|source| PublicDecryptFailure::EncryptedStore { index, source })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let batch: Vec<_> = request
        .handles()
        .iter()
        .zip(&stores)
        .map(|(entry, store)| {
            let query = LeafQuery {
                encrypted_store: store.account_key(),
                handle: entry.handle,
                kind: LeafKind::Public,
            };
            (query, (store, entry.handle))
        })
        .collect();
    let bindings = verify_proofs(&host.proofs, &batch, |(store, handle), outcome| {
        check_public_binding(store, *handle, outcome)
    })
    .await?;
    for (index, binding) in bindings.into_iter().enumerate() {
        binding.map_err(|source| PublicDecryptFailure::HandleBinding { index, source })?;
    }
    Ok(())
}

/// Why a public decryption was not authorized. Per-entry rules carry the entry index.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum PublicDecryptFailure {
    #[error("host state: {0}")]
    Snapshot(#[from] SnapshotError),
    #[error("entry {index}: encrypted store: {source}")]
    EncryptedStore {
        index: usize,
        source: EncryptedStoreFailure,
    },
    #[error("leaf proofs: {0}")]
    ProofRead(#[from] ProofReadError),
    #[error("entry {index}: handle binding: {source}")]
    HandleBinding {
        index: usize,
        source: HandleBindingFailure,
    },
}
