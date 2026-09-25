//! Solana user decryption authorization. Every worker attempt reads fresh host state.

use super::delegation::check_delegation;
use super::encrypted_store::resolve_encrypted_store;
use super::failure::AuthorizationFailure;
use super::handle_binding::{check_handle_binding, verify_proofs};
use super::proof::{HostProofReader, LeafKind, LeafQuery};
use super::snapshot::{DelegationRowKeys, HostObservation, HostStateReader, observe};
use super::watermark::{check_not_invalidated, check_window, read_watermark};
use super::{delegation_address, permit_invalidation_address, wildcard_delegation_address};
use alloy::primitives::B256;
use connector_utils::types::solana_request::SolanaUserDecryptionRequestV1;
use solana_pubkey::Pubkey;
use tracing::info;
use zama_solana_permit::verify_signature;
use zama_solana_request::HandleEntry;

/// Everything authorization needs besides the request and chain state.
#[derive(Clone, Copy, Debug)]
pub struct AuthorizationContext {
    /// The host program this Connector serves on the request's chain.
    pub program_id: Pubkey,
    /// The second the validity window is evaluated at.
    pub now_unix_seconds: u64,
}

/// Checks the permit and the host authorization state for one attempt. The KMS context is checked
/// by the caller, as for every other decryption.
pub async fn authorize_request(
    reader: &impl HostStateReader,
    proofs: &impl HostProofReader,
    context: AuthorizationContext,
    request: &SolanaUserDecryptionRequestV1,
) -> Result<(), AuthorizationFailure> {
    let (permit, entries) = (request.request.permit(), request.request.entries());
    let program_id = context.program_id;
    let signer = Pubkey::new_from_array(*permit.user_address().as_bytes());
    verify_signature(permit, request.request.signature())
        .map_err(AuthorizationFailure::Signature)?;
    check_window(
        permit.start_timestamp(),
        permit.duration_seconds(),
        context.now_unix_seconds,
    )?;
    let signed_program = Pubkey::new_from_array(*permit.verifying_program_id().as_bytes());
    if signed_program != program_id {
        return Err(AuthorizationFailure::ProgramIdMismatch {
            signed: signed_program,
            own: program_id,
        });
    }

    // A delegation record's address depends on the application of the entry's encrypted store, so
    // a delegated request needs a second read, no older than the first. The first read only locates
    // the rows (a store it cannot resolve fails the request there, as the request named it); every
    // rule that authorizes is judged against the last read.
    let watermark_address = permit_invalidation_address(program_id, signer);
    let store_keys: Vec<_> = entries
        .iter()
        .map(|entry| Pubkey::new_from_array(entry.encrypted_store))
        .collect();
    let first = observe(reader, watermark_address, &store_keys, &[], None).await?;
    let delegated = delegation_row_keys(&first, program_id, signer, entries)?;
    let observation = if delegated.is_empty() {
        first
    } else {
        observe(
            reader,
            watermark_address,
            &store_keys,
            &delegated,
            Some(first.slot),
        )
        .await?
    };

    let watermark = read_watermark(&observation.watermark, program_id, signer)?;
    check_not_invalidated(permit.start_timestamp(), watermark)?;

    // Every host rule is judged before a coprocessor is asked, so a host record the host program
    // could not have written fails the request whatever the proof read returns.
    let mut stores = Vec::with_capacity(observation.entries.len());
    let mut audit = Vec::new();
    for (index, ((entry, observed), store_key)) in entries
        .iter()
        .zip(&observation.entries)
        .zip(&store_keys)
        .enumerate()
    {
        let store = resolve_encrypted_store(observed.store.as_ref(), program_id, *store_key)
            .map_err(|source| AuthorizationFailure::EncryptedStore { index, source })?;
        let owner = Pubkey::new_from_array(entry.owner_address);
        if !store.is_in(permit.allowed_scopes()) {
            return Err(AuthorizationFailure::ScopeNotAllowed {
                index,
                program: store.program(),
                scope: store.scope(),
            });
        }
        if owner != signer {
            let rows = observed
                .delegation
                .as_ref()
                .expect("the deciding read carries the rows of every delegated entry");
            let row = check_delegation(rows, program_id, owner, signer, store.encrypted_store())
                .map_err(|source| AuthorizationFailure::Delegation { index, source })?;
            audit.push(format!(
                "entry {index}: delegator {owner}, application ({}, {}), {row:?} row",
                store.program(),
                store.scope(),
            ));
        }
        stores.push(store);
    }

    // The leaf must name the entry's owner address: the signer for a direct entry, the delegator for
    // a delegated one. Proving the signer's leaf instead would let a delegate decrypt handles the
    // delegator was never allowed on.
    let batch: Vec<_> = entries
        .iter()
        .zip(&stores)
        .map(|(entry, store)| {
            let (handle, owner) = (
                B256::from(entry.handle),
                Pubkey::new_from_array(entry.owner_address),
            );
            let query = LeafQuery {
                encrypted_store: store.account_key(),
                handle,
                kind: LeafKind::Allowed { key: owner },
            };
            (query, (store, handle, owner))
        })
        .collect();
    let bindings = verify_proofs(proofs, &batch, |(store, handle, owner), outcome| {
        check_handle_binding(store, *handle, *owner, outcome)
    })
    .await?;

    for (index, binding) in bindings.into_iter().enumerate() {
        binding.map_err(|source| AuthorizationFailure::HandleBinding { index, source })?;
    }

    // An auditor has to tell an application-scoped grant from a wildcard one, although both
    // authorize identically.
    if !audit.is_empty() {
        info!(
            delegate = %signer,
            observed_slot = observation.slot,
            entries = ?audit,
            "Solana delegated user decryption entries authorized"
        );
    }
    Ok(())
}

/// The exact and wildcard delegation rows of every delegated entry, derived from the store
/// applications of the first read.
fn delegation_row_keys(
    first: &HostObservation,
    program_id: Pubkey,
    signer: Pubkey,
    entries: &[HandleEntry],
) -> Result<Vec<DelegationRowKeys>, AuthorizationFailure> {
    let mut rows = Vec::new();
    for (index, (entry, observed)) in entries.iter().zip(&first.entries).enumerate() {
        let delegator = Pubkey::new_from_array(entry.owner_address);
        if delegator == signer {
            continue;
        }
        let store_key = Pubkey::new_from_array(entry.encrypted_store);
        let store = resolve_encrypted_store(observed.store.as_ref(), program_id, store_key)
            .map_err(|source| AuthorizationFailure::EncryptedStore { index, source })?;
        rows.push(DelegationRowKeys {
            entry: index,
            exact: delegation_address(
                program_id,
                delegator,
                signer,
                store.program(),
                store.scope(),
            ),
            wildcard: wildcard_delegation_address(program_id, delegator, signer),
        });
    }
    Ok(rows)
}
