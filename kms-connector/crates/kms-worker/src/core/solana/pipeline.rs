//! Solana user decryption authorization. Every worker attempt reads fresh host state.

use super::delegation::check_delegation;
use super::encrypted_store::resolve_encrypted_store;
use super::failure::AuthorizationFailure;
use super::handle_binding::{check_handle_binding, verify_proofs_with_one_retry};
use super::proof::{HostProofReader, LeafKind, LeafQuery};
use super::scope::check_scope;
use super::snapshot::{HostSnapshot, HostStateReader, plan_first_read, plan_second_read};
use super::watermark::{check_not_invalidated, check_window, read_watermark};
use super::{SolanaPubkeyBytes, delegation_address, wildcard_delegation_address};
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
    // a delegated request needs a second read. Every rule is evaluated against the last read alone.
    let first_keys = plan_first_read(request, program_id);
    let first = reader.read_accounts(&first_keys).await?;
    let delegation_keys = discover_delegation_keys(&first, program_id, signer, request)?;
    // `now` is the deciding read's Clock, which only a delegated request reads.
    let (observation, now) = if delegation_keys.is_empty() {
        (first, None)
    } else {
        let second_keys = plan_second_read(&first_keys, delegation_keys);
        let second = reader
            .read_accounts(&second_keys)
            .await?
            .deciding_after(&first)?;
        let now = second.unix_timestamp()?;
        (second, Some(now))
    };

    let watermark = read_watermark(&observation, program_id, signer)?;
    check_not_invalidated(permit.start_timestamp(), watermark)?;

    let stores = request
        .handles()
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let store = resolve_encrypted_store(&observation, program_id, entry.encrypted_store)
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
    let bindings = verify_proofs_with_one_retry(proofs, &batch, |(store, entry), outcome| {
        check_handle_binding(store, entry.handle, entry.owner_address, outcome)
    })
    .await?;

    let mut delegated = Vec::new();
    for (index, ((entry, store), binding)) in request
        .handles()
        .iter()
        .zip(&stores)
        .zip(bindings)
        .enumerate()
    {
        binding.map_err(|source| AuthorizationFailure::HandleBinding { index, source })?;
        if entry.owner_address == signer {
            continue;
        }
        let row = check_delegation(
            &observation,
            program_id,
            now.expect("a delegated entry forces the second read"),
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
            observed_slot = observation.observed_slot(),
            entries = ?delegated,
            "Solana delegated user decryption entries authorized"
        );
    }
    Ok(())
}

/// The exact and wildcard delegation-record addresses of every delegated entry, derived from the
/// store applications of the first read.
fn discover_delegation_keys(
    first: &HostSnapshot,
    program_id: SolanaPubkeyBytes,
    signer: SolanaPubkeyBytes,
    request: &SolanaUserDecryptionRequestV1,
) -> Result<Vec<SolanaPubkeyBytes>, AuthorizationFailure> {
    let mut keys = Vec::new();
    for (index, entry) in request.handles().iter().enumerate() {
        let delegator = entry.owner_address;
        if delegator == signer {
            continue;
        }
        let store = resolve_encrypted_store(first, program_id, entry.encrypted_store)
            .map_err(|source| AuthorizationFailure::EncryptedStore { index, source })?;
        keys.push(
            delegation_address(
                program_id,
                delegator,
                signer,
                store.program(),
                store.scope(),
            )
            .0,
        );
        keys.push(wildcard_delegation_address(program_id, delegator, signer).0);
    }
    Ok(keys)
}
