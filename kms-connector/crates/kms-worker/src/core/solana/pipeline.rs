//! The authorization pipeline: one explicit sequence, one observation point, no second
//! thoughts.
//!
//! ```text
//! strict decode (typed form, access-proof form, non-empty handle list)
//!   → signature over the reconstructed envelope
//!   → validity window
//!   → deployment identity and chain-id agreement
//!   → KMS pair servability
//!   → READ host state
//!   → if any entry is delegated: resolve its encrypted value account to learn its
//!     authority, then READ again with the delegation records added — that read is the
//!     deciding observation and the first read's values are discarded
//!   → invalidation watermark
//!   → per entry: encrypted value account → its authority → handle binding → scope
//!   → per delegated entry: delegation freshness
//!   → accepted
//! ```
//!
//! The state-free rules come first because a request that fails them costs no RPC. The
//! watermark sits after the read because its value is snapshot state, even though it is a
//! permit-level rule — which is the one place the rule numbering and the execution order
//! disagree, and the numbering is not normative.
//!
//! Every rule below the reads takes the *deciding* snapshot: the last read, whole and on its
//! own. A delegated request reads twice because a delegation address is not computable before
//! an encrypted value account has been read, and the earlier read is a discovery step whose values decide
//! nothing (see [`super::snapshot`]). The two are held to their order and nothing else: a
//! deciding read older than the discovery read is refused as the lagging node it is, transiently.
//!
//! Once a request is accepted nothing re-reads state for it. A later handle update, subject
//! rotation or delegation revocation do not affect it: the normalized request, its linker and
//! its response bind exactly the handles resolved at the observation point. There is also no
//! cache in the other direction — a permit is reusable, but every request under it is
//! authorized from scratch against its own observation, so a revoked delegation stops the
//! next request immediately.

use super::delegation::{
    AuthorizedRow, check_delegation, delegation_address, wildcard_delegation_address,
};
use super::deployment::{DeploymentIdentity, check_deployment};
use super::encrypted_value_account::{
    ResolvedEncryptedValueAccount, resolve_encrypted_value_account,
};
use super::failure::AuthorizationFailure;
use super::handle_binding::{
    HandleBindingFailure, check_handle_binding, classify_inclusion_failure,
};
use super::kms_pair::KmsPairValidator;
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
    /// The subject whose access was established: the signer for a direct entry, the
    /// delegator for a delegated one.
    pub subject: SolanaPubkeyBytes,
    /// The authority, read from the validated encrypted value account.
    pub encrypted_value_account_authority: SolanaPubkeyBytes,
    /// The ACL domain, read from the validated encrypted value account.
    pub domain: SolanaPubkeyBytes,
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
        | PermitError::TooManyAclDomainKeys { .. }
        | PermitError::AclDomainKeysNotAscending { .. }
        | PermitError::DuplicateAclDomainKey { .. }
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
/// Generic over the two seams that are not pure functions — the state reader and the KMS pair
/// validator — and takes nothing else it could read the world through. That is what makes a
/// scenario in a test a value rather than a moment in time.
pub async fn authorize_request<R, V>(
    reader: &R,
    pair_validator: &V,
    context: AuthorizationContext<'_>,
    request: &SolanaUserDecryptRequest,
) -> Result<AuthorizedRequest, AuthorizationFailure>
where
    R: HostStateReader,
    V: KmsPairValidator,
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

    // The reads. The first covers the signer's invalidation record and one account per named
    // encrypted value account; a delegated entry then needs a second, because its record's address
    // is a function of an authority only its encrypted value account can
    // supply.
    let first_keys = plan_first_read(request, context.deployment);
    let first = reader.read_accounts(&first_keys).await?;
    let delegation_keys = discover_delegation_keys(&first, program_id, signer, request)?;
    let observation = if delegation_keys.is_empty() {
        first
    } else {
        let second_keys = plan_second_read(&first_keys, delegation_keys);
        let second = reader.read_accounts(&second_keys).await?;
        // The one condition on the pair of reads, and it is ordering rather than agreement: a
        // deciding read behind the discovery read would report grants the discovery read saw as
        // absent, blaming the delegation for what the read did.
        second.deciding_after(&first)?
    };

    // Everything below is evaluated against that one observation, and nothing below reads state.
    let watermark = read_watermark(&observation, program_id, signer)?;
    check_not_invalidated(permit.start_timestamp(), watermark)?;

    let mut entries = Vec::with_capacity(request.handles().len());
    let mut delegated = Vec::new();
    for (index, entry) in request.handles().iter().enumerate() {
        let encrypted_value_account =
            resolve_encrypted_value_account(&observation, program_id, entry.encrypted_value_id())
                .map_err(|source| AuthorizationFailure::EncryptedValueAccount { index, source })?;

        // The subject is the entry's owner in both branches: the signer for a direct entry, the
        // delegator for a delegated one. Proving the signer's own standing in the delegated branch
        // would authorize a delegate against encrypted value accounts the delegator never had.
        let subject = entry.subject();
        check_handle_binding(
            &encrypted_value_account,
            entry.handle(),
            subject,
            entry.access(),
        )
        .map_err(|source| classify_binding_failure(index, entry.proof_leaf_count(), source))?;
        check_scope(permit.allowed_acl_domain_keys(), &encrypted_value_account)
            .map_err(|source| AuthorizationFailure::Scope { index, source })?;

        if subject != signer {
            delegated.push((
                index,
                subject,
                encrypted_value_account.encrypted_value_account_authority(),
                entry.handle(),
            ));
        }
        entries.push(AuthorizedEntry {
            handle: entry.handle(),
            subject,
            encrypted_value_account_authority: encrypted_value_account
                .encrypted_value_account_authority(),
            domain: encrypted_value_account.domain(),
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
/// account's authority and the delegator's wildcard row. Both are planned
/// unconditionally rather than the wildcard being fetched only when the authority-specific row is
/// missing
/// — that would be a third read, and a rule that reads state after the deciding observation is the
/// thing this pipeline does not do. Repeats collapse in the key set, so a batch under one delegator
/// costs one wildcard key.
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
        let delegator = entry.subject();
        if delegator == signer {
            continue;
        }
        let encrypted_value_account: ResolvedEncryptedValueAccount =
            resolve_encrypted_value_account(first, program_id, entry.encrypted_value_id())
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

/// Turns a failed binding rule into the failure a client sees.
///
/// Both ways an inclusion proof can fail against an observation become one outcome here, because
/// they call for the same thing from a client and are distinguished by the same number: a proof
/// that did not verify, and a proof naming a leaf position the observation does not have — which
/// is what "the proof service is ahead of this Connector" looks like from this side. The action
/// needs the count the request claimed, which the binding rule is deliberately not given; this is
/// the layer that holds both.
fn classify_binding_failure(
    index: usize,
    proof_leaf_count: u64,
    source: HandleBindingFailure,
) -> AuthorizationFailure {
    let inclusion = |live_leaf_count| AuthorizationFailure::InclusionFailed {
        index,
        action: classify_inclusion_failure(proof_leaf_count, live_leaf_count),
        proof_leaf_count,
        live_leaf_count,
    };
    match source {
        HandleBindingFailure::ProofDoesNotVerify { live_leaf_count } => inclusion(live_leaf_count),
        HandleBindingFailure::LeafIndexOutOfRange { leaf_count, .. } => inclusion(leaf_count),
        binding @ (HandleBindingFailure::NotCurrentHandle { .. }
        | HandleBindingFailure::NotAMember { .. }
        | HandleBindingFailure::MmrStateInconsistent) => AuthorizationFailure::HandleBinding {
            index,
            source: binding,
        },
    }
}
