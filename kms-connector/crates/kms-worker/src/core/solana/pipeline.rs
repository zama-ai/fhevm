//! Solana user decryption authorization. Every worker attempt reads fresh host state.

use super::delegation::check_delegation;
use super::encrypted_store::resolve_encrypted_store;
use super::failure::AuthorizationFailure;
use super::handle_binding::{check_handle_binding, verify_proofs};
use super::proof::{HostProofReader, LeafKind, LeafQuery};
use super::scope::check_scope;
use super::snapshot::{DelegationRowKeys, HostObservation, HostStateReader, observe};
use super::watermark::{check_not_invalidated, check_window, read_watermark};
use super::{
    SolanaPubkeyBytes, delegation_address, permit_invalidation_address, wildcard_delegation_address,
};
use connector_utils::types::solana_request::SolanaUserDecryptionRequestV1;
use solana_pubkey::Pubkey;
use tracing::info;
use zama_solana_permit::verify_signature;

/// Everything authorization needs besides the request and chain state.
#[derive(Clone, Copy, Debug)]
pub struct AuthorizationContext {
    /// The host program this Connector serves on the request's chain.
    pub program_id: SolanaPubkeyBytes,
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
    let permit = request.permit();
    let program_id = context.program_id;
    let signer = *permit.user_address().as_bytes();
    verify_signature(permit, request.signature()).map_err(AuthorizationFailure::Signature)?;
    check_window(
        permit.start_timestamp(),
        permit.duration_seconds(),
        context.now_unix_seconds,
    )?;
    let signed_program = *permit.verifying_program_id().as_bytes();
    if signed_program != program_id {
        return Err(AuthorizationFailure::ProgramIdMismatch {
            signed: signed_program,
            own: program_id,
        });
    }

    // A delegation record's address depends on the application of the entry's encrypted store, so
    // a delegated request needs a second read, no older than the first. Every rule is evaluated
    // against the last read alone.
    let watermark_address = permit_invalidation_address(program_id, signer);
    let store_keys: Vec<_> = request
        .handles()
        .iter()
        .map(|entry| entry.encrypted_store)
        .collect();
    let first = observe(reader, watermark_address, &store_keys, &[], None).await?;
    let delegated = delegation_row_keys(&first, program_id, signer, request)?;
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

    let stores = request
        .handles()
        .iter()
        .zip(&observation.entries)
        .enumerate()
        .map(|(index, (entry, observed))| {
            let store =
                resolve_encrypted_store(observed.store.as_ref(), program_id, entry.encrypted_store)
                    .map_err(|source| AuthorizationFailure::EncryptedStore { index, source })?;
            check_scope(permit.allowed_scopes(), &store)
                .map_err(|source| AuthorizationFailure::Scope { index, source })?;
            Ok(store)
        })
        .collect::<Result<Vec<_>, AuthorizationFailure>>()?;

    // The leaf must name the entry's owner address: the signer for a direct entry, the delegator for
    // a delegated one. Proving the signer's leaf instead would let a delegate decrypt handles the
    // delegator was never allowed on.
    let batch: Vec<_> = request
        .handles()
        .iter()
        .zip(&stores)
        .map(|(entry, store)| {
            let query = LeafQuery {
                encrypted_store: store.account_key(),
                handle: entry.handle,
                kind: LeafKind::Allowed {
                    key: entry.owner_address,
                },
            };
            (query, (store, entry))
        })
        .collect();
    let bindings = verify_proofs(proofs, &batch, |(store, entry), outcome| {
        check_handle_binding(store, entry.handle, entry.owner_address, outcome)
    })
    .await?;

    let mut delegated = Vec::new();
    for (index, (((entry, observed), store), binding)) in request
        .handles()
        .iter()
        .zip(&observation.entries)
        .zip(&stores)
        .zip(bindings)
        .enumerate()
    {
        binding.map_err(|source| AuthorizationFailure::HandleBinding { index, source })?;
        if entry.owner_address == signer {
            continue;
        }
        let rows = observed
            .delegation
            .as_ref()
            .expect("the deciding read carries the rows of every delegated entry");
        let row = check_delegation(
            rows,
            program_id,
            entry.owner_address,
            signer,
            store.encrypted_store(),
        )
        .map_err(|source| AuthorizationFailure::Delegation { index, source })?;
        delegated.push(format!(
            "entry {index}: delegator {}, application ({}, {}), {row:?} row",
            Pubkey::new_from_array(entry.owner_address),
            Pubkey::new_from_array(store.program()),
            Pubkey::new_from_array(store.scope()),
        ));
    }

    // An auditor has to tell an application-scoped grant from a wildcard one, although both
    // authorize identically.
    if !delegated.is_empty() {
        info!(
            delegate = %Pubkey::new_from_array(signer),
            observed_slot = observation.slot,
            entries = ?delegated,
            "Solana delegated user decryption entries authorized"
        );
    }
    Ok(())
}

/// The exact and wildcard delegation rows of every delegated entry, derived from the store
/// applications of the first read.
fn delegation_row_keys(
    first: &HostObservation,
    program_id: SolanaPubkeyBytes,
    signer: SolanaPubkeyBytes,
    request: &SolanaUserDecryptionRequestV1,
) -> Result<Vec<DelegationRowKeys>, AuthorizationFailure> {
    let mut rows = Vec::new();
    for (entry, (claim, observed)) in request.handles().iter().zip(&first.entries).enumerate() {
        let delegator = claim.owner_address;
        if delegator == signer {
            continue;
        }
        let store =
            resolve_encrypted_store(observed.store.as_ref(), program_id, claim.encrypted_store)
                .map_err(|source| AuthorizationFailure::EncryptedStore {
                    index: entry,
                    source,
                })?;
        rows.push(DelegationRowKeys {
            entry,
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
