//! User-decrypt authorization. Each worker attempt reads and validates a fresh host snapshot.

use super::delegation::{
    AuthorizedRow, check_delegation, delegation_address, wildcard_delegation_address,
};
use super::deployment::{DeploymentIdentity, check_deployment};
use super::encrypted_store::{ResolvedEncryptedStore, resolve_encrypted_store};
use super::failure::AuthorizationFailure;
use super::handle_binding::{check_handle_binding, verify_proofs_with_one_retry};
use super::pause::check_not_paused;
use super::proof::{HostProofReader, LeafKind, LeafQuery, ProofBatch};
use super::scope::check_scope;
use super::snapshot::{HostSnapshot, HostStateReader, plan_first_read, plan_second_read};
use super::watermark::{check_not_invalidated, check_window, read_watermark};
use crate::core::event_processor::{ContextManager, RequestCheckError};
use crate::core::solana_acl::{HandleBytes, SolanaPubkeyBytes};
use alloy::primitives::U256;
use connector_utils::types::extra_data::ExtraData;
use connector_utils::types::solana_request::SolanaUserDecryptRequest;
use tracing::info;
use zama_solana_permit::{KmsRouting, verify_signature};

/// Everything authorization needs that is neither the request nor chain state.
#[derive(Clone, Copy, Debug)]
pub struct AuthorizationContext<'a> {
    /// This Connector's own deployment identity.
    pub deployment: &'a DeploymentIdentity,
    /// The wall-clock second the validity window is evaluated at. A parameter rather than a
    /// call to the clock, so that every window test states its own time.
    pub now_unix_seconds: u64,
}

/// The audit record of one authorized delegated entry, for the per-request log event: whose
/// access the signer used, under which authority, for which handle, and which row carried the
/// grant.
struct DelegatedEntryAudit {
    index: usize,
    delegator: SolanaPubkeyBytes,
    authority: SolanaPubkeyBytes,
    handle: HandleBytes,
    authorizing_row: AuthorizedRow,
}

impl std::fmt::Debug for DelegatedEntryAudit {
    // Hand-written for the identities: the derived form prints a `[u8; 32]` as thirty-two
    // decimal numbers, unreadable in a log line.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("entry")
            .field("index", &self.index)
            .field(
                "delegator",
                &format_args!("{}", alloy::hex::encode(self.delegator)),
            )
            .field(
                "authority",
                &format_args!("{}", alloy::hex::encode(self.authority)),
            )
            .field(
                "handle",
                &format_args!("{}", alloy::hex::encode(self.handle)),
            )
            .field("authorizing_row", &self.authorizing_row)
            .finish()
    }
}

/// The signature rule on its own: the permit's signature over the envelope this Connector
/// reconstructs from the strictly decoded typed fields.
///
/// A named step rather than an inline call, because it is the one rule whose behaviour has to be
/// identical in five implementations, and the way that is checked is by running the normative
/// permit vectors through *this* function. Reaching for the permit crate directly in a test
/// would prove the crate correct and say nothing about the Connector.
pub fn check_signature(request: &SolanaUserDecryptRequest) -> Result<(), AuthorizationFailure> {
    verify_signature(request.permit(), request.signature()).map_err(AuthorizationFailure::Signature)
}

/// Checks the permit, its KMS context, and the host authorization state for this attempt.
pub async fn authorize_request<R, C, P>(
    reader: &R,
    context_manager: &C,
    proofs: &P,
    context: AuthorizationContext<'_>,
    request: &SolanaUserDecryptRequest,
) -> Result<(), RequestCheckError>
where
    R: HostStateReader,
    C: ContextManager,
    P: HostProofReader,
{
    let permit = request.permit();
    check_signature(request)?;
    check_window(
        permit.start_timestamp(),
        permit.duration_seconds(),
        context.now_unix_seconds,
    )
    .map_err(AuthorizationFailure::from)?;
    check_deployment(request, context.deployment).map_err(AuthorizationFailure::from)?;
    let KmsRouting::ContextAndEpoch {
        kms_context_id,
        kms_epoch_id,
    } = permit.extra_data();
    context_manager
        .validate_context(&ExtraData {
            context_id: Some(U256::from_be_bytes(*kms_context_id.as_bytes())),
            epoch_id: Some(U256::from_be_bytes(*kms_epoch_id.as_bytes())),
        })
        .await?;
    authorize_accounts(reader, proofs, context, request)
        .await
        .map_err(Into::into)
}

async fn authorize_accounts(
    reader: &impl HostStateReader,
    proofs: &impl HostProofReader,
    context: AuthorizationContext<'_>,
    request: &SolanaUserDecryptRequest,
) -> Result<(), AuthorizationFailure> {
    let permit = request.permit();
    let signer = *permit.user_pubkey().as_bytes();
    let program_id = context.deployment.program_id();
    // The reads. The first covers the deployment's config singleton, the signer's invalidation
    // record and one account per named encrypted store; a delegated entry then needs a
    // second, because its record's address is a function of an authority only its encrypted store
    // can supply.
    let first_keys = plan_first_read(request, context.deployment);
    let first = reader.read_accounts(&first_keys).await?;
    // The one rule decided on the first read. A paused host releases no plaintext at all, so
    // refusing here costs a delegated request its second round trip instead of spending it to
    // reach the same answer — and the config singleton then leaves the second read, which is
    // sized to the account budget without it.
    check_not_paused(&first, program_id)?;
    let delegation_keys = discover_delegation_keys(&first, program_id, signer, request)?;
    let observation = if delegation_keys.is_empty() {
        first
    } else {
        let second_keys = plan_second_read(&first_keys, context.deployment, delegation_keys);
        let second = reader.read_accounts(&second_keys).await?;
        // The one condition on the pair of reads, and it is ordering rather than agreement: a
        // deciding read behind the discovery read would report grants the discovery read saw as
        // absent, blaming the delegation for what the read did.
        second.deciding_after(&first)?
    };

    // Everything below is evaluated against that one observation, and nothing below reads chain
    // state.
    let watermark = read_watermark(&observation, program_id, signer)?;
    check_not_invalidated(permit.start_timestamp(), watermark)?;

    let mut accounts = Vec::with_capacity(request.handles().len());
    for (index, entry) in request.handles().iter().enumerate() {
        let encrypted_store =
            resolve_encrypted_store(&observation, program_id, entry.encrypted_store())
                .map_err(|source| AuthorizationFailure::EncryptedStore { index, source })?;
        check_scope(permit.allowed_scopes(), &encrypted_store)
            .map_err(|source| AuthorizationFailure::Scope { index, source })?;
        accounts.push(encrypted_store);
    }

    // The proof read: one batch for the request, planned from the resolved accounts. The key the
    // leaf must name is the entry's owner in both branches — the signer for a direct entry, the
    // delegator for a delegated one. Proving the signer's own leaf in the delegated branch would
    // authorize a delegate against handles the delegator was never allowed on.
    let batch = ProofBatch::new(
        request
            .handles()
            .iter()
            .zip(&accounts)
            .map(|(entry, account)| {
                (
                    LeafQuery {
                        encrypted_store: account.account_key(),
                        handle: entry.handle(),
                        kind: LeafKind::Allowed {
                            key: entry.allowed_key(),
                        },
                    },
                    (account, entry.handle(), entry.allowed_key()),
                )
            }),
    );
    let bindings =
        verify_proofs_with_one_retry(proofs, &batch, |(account, handle, key), outcome| {
            check_handle_binding(account, *handle, *key, outcome)
        })
        .await?;

    let mut delegated = Vec::new();
    for (index, (entry, encrypted_store)) in request.handles().iter().zip(&accounts).enumerate() {
        let query = LeafQuery {
            encrypted_store: encrypted_store.account_key(),
            handle: entry.handle(),
            kind: LeafKind::Allowed {
                key: entry.allowed_key(),
            },
        };
        batch
            .position(&query)
            .and_then(|position| bindings.get(position))
            .ok_or(AuthorizationFailure::MissingProofBinding { index })?
            .clone()
            .map_err(|source| AuthorizationFailure::HandleBinding { index, source })?;

        if entry.allowed_key() != signer {
            delegated.push((
                index,
                entry.allowed_key(),
                encrypted_store.authority(),
                entry.handle(),
            ));
        }
    }

    let mut audit = Vec::with_capacity(delegated.len());
    for (index, delegator, authority, handle) in delegated {
        let authorizing_row =
            check_delegation(&observation, program_id, delegator, signer, authority)
                .map_err(|source| AuthorizationFailure::Delegation { index, source })?;
        audit.push(DelegatedEntryAudit {
            index,
            delegator,
            authority,
            handle,
            authorizing_row,
        });
    }

    // Every delegated authorization leaves one structured log event: whose access was used, by
    // whom, under which authority — and which row carried each grant, because a wildcard grant
    // and an authority-scoped one are different facts to an auditor even though they authorize
    // identically. One event per request, not per entry.
    if !audit.is_empty() {
        info!(
            delegate = %alloy::hex::encode(signer),
            observed_slot = observation.observed_slot(),
            entries = ?audit,
            "Solana delegated user-decryption entries authorized"
        );
    }

    Ok(())
}

/// Derives the delegation-record addresses a delegated request needs, from the discovery read.
///
/// This is the only use the first read of a delegated request is put to, and it is why the read
/// happens at all. The encrypted store is resolved here to learn its authority and for no other
/// purpose: every rule, including the resolution of this same
/// encrypted store, is applied again against the deciding observation.
///
/// Two addresses per delegated entry, because two rows can carry the grant: the encrypted store's
/// authority and the delegator's wildcard row. Both are planned unconditionally rather
/// than the wildcard being fetched only when the authority-specific row is missing — that would
/// be a third read, and a rule that reads state after the deciding observation is the thing this
/// pipeline does not do. Repeats collapse in the key set, so a batch under one delegator costs
/// one wildcard key.
///
/// Empty for a direct-only request, which is what makes that request cost one read.
fn discover_delegation_keys(
    first: &HostSnapshot,
    program_id: SolanaPubkeyBytes,
    signer: SolanaPubkeyBytes,
    request: &SolanaUserDecryptRequest,
) -> Result<Vec<SolanaPubkeyBytes>, AuthorizationFailure> {
    let mut keys = Vec::new();
    for (index, entry) in request.handles().iter().enumerate() {
        let delegator = entry.allowed_key();
        if delegator == signer {
            continue;
        }
        let encrypted_store: ResolvedEncryptedStore =
            resolve_encrypted_store(first, program_id, entry.encrypted_store())
                .map_err(|source| AuthorizationFailure::EncryptedStore { index, source })?;
        let (account_key, _) =
            delegation_address(program_id, delegator, signer, encrypted_store.authority());
        keys.push(account_key);
        let (wildcard_key, _) = wildcard_delegation_address(program_id, delegator, signer);
        keys.push(wildcard_key);
    }
    Ok(keys)
}
