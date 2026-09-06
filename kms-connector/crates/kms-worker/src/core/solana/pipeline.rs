//! The authorization pipeline: one explicit sequence, one observation point, no second
//! thoughts.
//!
//! ```text
//! strict decode (typed form, non-empty handle list)
//!   → signature over the reconstructed envelope
//!   → validity window
//!   → deployment identity and chain-id agreement
//!   → KMS pair servability
//!   → READ host state
//!   → host pause switch, from that first read
//!   → if any entry is delegated: resolve its encrypted value account to learn its
//!     authority, then READ again with the delegation records added — that read is the
//!     deciding observation and the first read's values are discarded
//!   → invalidation watermark
//!   → per entry: encrypted value account → scope
//!   → READ the leaf proofs, one batch for the request, once more if the record is behind
//!   → per entry: handle binding against the account's own peaks
//!   → per delegated entry: delegation freshness
//!   → accepted
//! ```
//!
//! The state-free rules come first because a request that fails them costs no RPC. The
//! watermark sits after the read because its value is snapshot state, even though it is a
//! permit-level rule — which is the one place the rule numbering and the execution order
//! disagree, and the numbering is not normative.
//!
//! Every rule below the account reads takes the *deciding* snapshot: the last read, whole and on
//! its own. A delegated request reads twice because a delegation address is not computable before
//! an encrypted value account has been read, and the earlier read is a discovery step whose values
//! decide nothing (see [`super::snapshot`]). The two are held to their order and nothing else: a
//! deciding read older than the discovery read is refused as the lagging node it is, transiently.
//!
//! The proof read is not a third observation of the chain. It reads the coprocessors' record of
//! the chain's events, and nothing it returns is a decision: every proof is verified against the
//! deciding snapshot's own peaks (see [`super::proof`]). It comes after the accounts are resolved
//! because the batch is planned from them, and after scope because a request outside its signed
//! scope should not cost a round trip to the record.
//!
//! The pause switch is the one exception, and it is not an authorization rule: it says whether
//! this Connector serves user decryptions at all. Reading it on the first read stops a paused
//! host before the second round trip and keeps the singleton off the deciding read, whose worst
//! case is sized to the RPC's account limit exactly (see [`super::pause`]).
//!
//! Once a request is accepted nothing re-reads state for it. A later handle update or delegation
//! revocation does not affect it: the normalized request, its linker and its response bind
//! exactly the handles resolved at the observation point. There is also no cache in the other
//! direction — a permit is reusable, but every request under it is authorized from scratch
//! against its own observation, so a revoked delegation stops the next request immediately.

use super::delegation::{
    AuthorizedRow, check_delegation, delegation_address, wildcard_delegation_address,
};
use super::deployment::{DeploymentIdentity, check_deployment};
use super::encrypted_value_account::{
    ResolvedEncryptedValueAccount, resolve_encrypted_value_account,
};
use super::failure::AuthorizationFailure;
use super::handle_binding::check_handle_binding;
use super::kms_pair::KmsPairValidator;
use super::pause::check_not_paused;
use super::proof::{HostProofReader, LeafKind, LeafQuery, ProofBatch, read_proofs_with_one_retry};
use super::request::{RequestFormError, SolanaUserDecryptRequest};
use super::scope::check_scope;
use super::snapshot::{HostSnapshot, HostStateReader, plan_first_read, plan_second_read};
use super::watermark::{check_not_invalidated, check_window, read_watermark};
use crate::core::solana_acl::{HandleBytes, SolanaPubkeyBytes};
use tracing::info;
use zama_solana_permit::{KmsRouting, PermitError, verify_signature};

/// Everything authorization needs that is neither the request nor chain state.
#[derive(Clone, Copy, Debug)]
pub struct AuthorizationContext<'a> {
    /// This Connector's own deployment identity.
    pub deployment: &'a DeploymentIdentity,
    /// The wall-clock second the validity window is evaluated at. A parameter rather than a
    /// call to the clock, so that every window test states its own time.
    pub now_unix_seconds: u64,
}

/// One entry as authorization resolved it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AuthorizedEntry {
    /// The handle, exactly as named by the request.
    pub handle: HandleBytes,
    /// The key whose allow leaf was proven: the signer for a direct entry, the delegator for a
    /// delegated one.
    pub allowed_key: SolanaPubkeyBytes,
    /// The authority, read from the validated encrypted value account.
    pub encrypted_value_account_authority: SolanaPubkeyBytes,
    /// The application program, read from the validated encrypted value account.
    pub program: SolanaPubkeyBytes,
    /// The program-declared scope, read from the validated encrypted value account.
    pub scope: SolanaPubkeyBytes,
}

/// An authorized request: the handle set is frozen here and nowhere later.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AuthorizedRequest {
    observed_slot: u64,
    request: SolanaUserDecryptRequest,
    entries: Vec<AuthorizedEntry>,
}

impl AuthorizedRequest {
    /// The observation point this authorization was decided at. Recorded rather than
    /// recomputed: it is what makes "accepted at slot N" a statement anyone downstream can
    /// read instead of infer.
    pub fn observed_slot(&self) -> u64 {
        self.observed_slot
    }

    /// The validated request, for the KMS normalization that follows.
    pub fn request(&self) -> &SolanaUserDecryptRequest {
        &self.request
    }

    /// The resolved entries, in request order — same order, same count, duplicates included.
    pub fn entries(&self) -> &[AuthorizedEntry] {
        &self.entries
    }
}

/// The audit record of one authorized delegated entry, for the per-request log event: whose
/// access the signer used, under which authority, for which handle, and which row carried the
/// grant.
struct DelegatedEntryAudit {
    index: usize,
    delegator: SolanaPubkeyBytes,
    encrypted_value_account_authority: SolanaPubkeyBytes,
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
                "encrypted_value_account_authority",
                &format_args!(
                    "{}",
                    alloy::hex::encode(self.encrypted_value_account_authority)
                ),
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
    verify_signature(request.permit(), request.signature()).map_err(|error| match error {
        PermitError::SignatureMismatch => AuthorizationFailure::SignatureMismatch,
        PermitError::UnusableUserPubkey => AuthorizationFailure::UnusableUserPubkey,
        // The typed-form violations cannot arrive here: these fields came out of strict
        // decoding, which rejects every one of them. Carried through as themselves rather than
        // renamed into a signature failure, so that if the crate ever does produce one, it
        // surfaces as what it is instead of as a wrong diagnosis.
        typed_form @ (PermitError::IdentityWidth { .. }
        | PermitError::ScopeWidth { .. }
        | PermitError::TooManyScopes { .. }
        | PermitError::ScopesNotAscending { .. }
        | PermitError::DuplicateScope { .. }
        | PermitError::DurationOutOfRange { .. }
        | PermitError::StartTimestampOutOfRange { .. }
        | PermitError::TransportKeyLength { .. }
        | PermitError::UnknownKmsRoutingVersion { .. }
        | PermitError::KmsRoutingLength { .. }) => {
            AuthorizationFailure::Form(RequestFormError::Permit(typed_form))
        }
    })
}

/// Authorizes one request, or says why not.
///
/// Generic over the three seams that are not pure functions — the state reader, the KMS pair
/// validator and the proof reader — and takes nothing else it could read the world through. That
/// is what makes a scenario in a test a value rather than a moment in time.
pub async fn authorize_request<R, V, P>(
    reader: &R,
    pair_validator: &V,
    proofs: &P,
    context: AuthorizationContext<'_>,
    request: &SolanaUserDecryptRequest,
) -> Result<AuthorizedRequest, AuthorizationFailure>
where
    R: HostStateReader,
    V: KmsPairValidator,
    P: HostProofReader,
{
    let permit = request.permit();
    let signer = *permit.user_pubkey().as_bytes();
    let program_id = context.deployment.program_id();

    // Everything that needs no state, first: a request that fails any of these costs no RPC.
    check_signature(request)?;
    check_window(
        permit.start_timestamp(),
        permit.duration_seconds(),
        context.now_unix_seconds,
    )?;
    check_deployment(request, context.deployment)?;
    let (kms_context_id, kms_epoch_id) = match permit.extra_data() {
        KmsRouting::ContextAndEpoch {
            kms_context_id,
            kms_epoch_id,
        } => (kms_context_id, kms_epoch_id),
    };
    pair_validator
        .validate_pair(kms_context_id.as_bytes(), kms_epoch_id.as_bytes())
        .await?;

    // The reads. The first covers the deployment's config singleton, the signer's invalidation
    // record and one account per named encrypted value account; a delegated entry then needs a
    // second, because its record's address is a function of an authority only its encrypted value
    // account can supply.
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
        let encrypted_value_account = resolve_encrypted_value_account(
            &observation,
            program_id,
            entry.encrypted_value_account(),
        )
        .map_err(|source| AuthorizationFailure::EncryptedValueAccount { index, source })?;
        check_scope(permit.allowed_scopes(), &encrypted_value_account)
            .map_err(|source| AuthorizationFailure::Scope { index, source })?;
        accounts.push(encrypted_value_account);
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
            .map(|(entry, account)| LeafQuery {
                encrypted_value_account: account.account_key(),
                handle: entry.handle(),
                kind: LeafKind::Allowed {
                    key: entry.allowed_key(),
                },
            }),
    );
    let live_leaf_count = |position: usize| {
        let query = &batch.queries()[position];
        accounts
            .iter()
            .find(|account| account.account_key() == query.encrypted_value_account)
            .map(|account| account.encrypted_value().leaf_count)
            .unwrap_or(0)
    };
    let outcomes = read_proofs_with_one_retry(proofs, &batch, live_leaf_count).await?;

    let mut entries = Vec::with_capacity(request.handles().len());
    let mut delegated = Vec::new();
    for (index, (entry, encrypted_value_account)) in
        request.handles().iter().zip(&accounts).enumerate()
    {
        let query = LeafQuery {
            encrypted_value_account: encrypted_value_account.account_key(),
            handle: entry.handle(),
            kind: LeafKind::Allowed {
                key: entry.allowed_key(),
            },
        };
        let outcome = &outcomes[batch
            .position(&query)
            .expect("every entry's query was planned into the batch")];
        check_handle_binding(
            encrypted_value_account,
            entry.handle(),
            entry.allowed_key(),
            outcome,
        )
        .map_err(|source| AuthorizationFailure::HandleBinding { index, source })?;

        if entry.allowed_key() != signer {
            delegated.push((
                index,
                entry.allowed_key(),
                encrypted_value_account.encrypted_value_account_authority(),
                entry.handle(),
            ));
        }
        entries.push(AuthorizedEntry {
            handle: entry.handle(),
            allowed_key: entry.allowed_key(),
            encrypted_value_account_authority: encrypted_value_account
                .encrypted_value_account_authority(),
            program: encrypted_value_account.program(),
            scope: encrypted_value_account.scope(),
        });
    }

    let mut audit = Vec::with_capacity(delegated.len());
    for (index, delegator, encrypted_value_account_authority, handle) in delegated {
        let authorizing_row = check_delegation(
            &observation,
            program_id,
            delegator,
            signer,
            encrypted_value_account_authority,
        )
        .map_err(|source| AuthorizationFailure::Delegation { index, source })?;
        audit.push(DelegatedEntryAudit {
            index,
            delegator,
            encrypted_value_account_authority,
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

    Ok(AuthorizedRequest {
        observed_slot: observation.observed_slot(),
        request: request.clone(),
        entries,
    })
}

/// Derives the delegation-record addresses a delegated request needs, from the discovery read.
///
/// This is the only use the first read of a delegated request is put to, and it is why the read
/// happens at all. The encrypted value account is resolved here to learn its encrypted value
/// account authority and for no other purpose: every rule, including the resolution of this same
/// encrypted value account, is applied again against the deciding observation.
///
/// Two addresses per delegated entry, because two rows can carry the grant: the encrypted value
/// account's authority and the delegator's wildcard row. Both are planned unconditionally rather
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
        let encrypted_value_account: ResolvedEncryptedValueAccount =
            resolve_encrypted_value_account(first, program_id, entry.encrypted_value_account())
                .map_err(|source| AuthorizationFailure::EncryptedValueAccount { index, source })?;
        let (account_key, _) = delegation_address(
            program_id,
            delegator,
            signer,
            encrypted_value_account.encrypted_value_account_authority(),
        );
        keys.push(account_key);
        let (wildcard_key, _) = wildcard_delegation_address(program_id, delegator, signer);
        keys.push(wildcard_key);
    }
    Ok(keys)
}
