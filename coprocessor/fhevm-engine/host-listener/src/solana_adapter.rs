use std::{collections::HashSet, fmt, ops::DerefMut};

use alloy_primitives::{Address, FixedBytes, Log};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use sha2::{Digest, Sha256};
use sqlx::{Error as SqlxError, Row};

use crate::generated::{
    decode_anchor_cpi_event as decode_zama_host_anchor_cpi_event,
    decode_anchor_event as decode_zama_host_anchor_event,
    decode_confidential_token_anchor_cpi_event,
    decode_confidential_token_anchor_event, ConfidentialTokenEvent,
    FheBinaryOpCode, FheBinaryOpEvent, FheRandEvent, FheTernaryOpCode,
    FheTernaryOpEvent, TrivialEncryptEvent, ZamaHostEvent,
    CONFIDENTIAL_TOKEN_EVENT_VERSION, EVENT_VERSION,
};

use crate::cmd::block_history::BlockSummary;
use crate::contracts::TfheContract;
use crate::contracts::TfheContract::TfheContractEvents;
use crate::database::dependence_chains::dependence_chains;
use crate::database::tfhe_event_propagate::{
    ClearConst, Database, Handle, LogTfhe, Transaction, TransactionHash,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SolanaFinalizedAccountFetchKind {
    EncryptedValueAccount,
    AclPermission,
    HandleMaterialCommitment,
    DisclosureRequest,
    BurnRedemptionRequest,
    TokenMint,
    TokenAccount,
    BurnRedemption,
    TransferCallbackSettlement,
}

impl SolanaFinalizedAccountFetchKind {
    fn as_i16(self) -> i16 {
        match self {
            SolanaFinalizedAccountFetchKind::EncryptedValueAccount => 1,
            SolanaFinalizedAccountFetchKind::AclPermission => 2,
            SolanaFinalizedAccountFetchKind::HandleMaterialCommitment => 3,
            SolanaFinalizedAccountFetchKind::DisclosureRequest => 4,
            SolanaFinalizedAccountFetchKind::BurnRedemptionRequest => 5,
            SolanaFinalizedAccountFetchKind::TokenMint => 6,
            SolanaFinalizedAccountFetchKind::TokenAccount => 7,
            SolanaFinalizedAccountFetchKind::BurnRedemption => 8,
            SolanaFinalizedAccountFetchKind::TransferCallbackSettlement => 9,
        }
    }

    fn from_i16(value: i16) -> Option<Self> {
        match value {
            1 => Some(SolanaFinalizedAccountFetchKind::EncryptedValueAccount),
            2 => Some(SolanaFinalizedAccountFetchKind::AclPermission),
            3 => {
                Some(SolanaFinalizedAccountFetchKind::HandleMaterialCommitment)
            }
            4 => Some(SolanaFinalizedAccountFetchKind::DisclosureRequest),
            5 => Some(SolanaFinalizedAccountFetchKind::BurnRedemptionRequest),
            6 => Some(SolanaFinalizedAccountFetchKind::TokenMint),
            7 => Some(SolanaFinalizedAccountFetchKind::TokenAccount),
            8 => Some(SolanaFinalizedAccountFetchKind::BurnRedemption),
            9 => Some(
                SolanaFinalizedAccountFetchKind::TransferCallbackSettlement,
            ),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaFinalizedAccountFetch {
    pub account_key: [u8; 32],
    pub kind: SolanaFinalizedAccountFetchKind,
    pub reason: &'static str,
    pub handle: Option<Handle>,
    pub related_account: Option<[u8; 32]>,
    pub subject: Option<[u8; 32]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaFinalizedAccountFetchJob {
    pub account_key: [u8; 32],
    pub kind: SolanaFinalizedAccountFetchKind,
    pub reason: String,
    pub handle: Option<Handle>,
    pub related_account: Option<[u8; 32]>,
    pub subject: Option<[u8; 32]>,
    pub transaction_id: TransactionHash,
    pub block_number: u64,
    pub attempts: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaFinalizedAccountWitness {
    pub account_key: [u8; 32],
    pub owner: [u8; 32],
    pub lamports: u64,
    pub executable: bool,
    pub data: Vec<u8>,
    pub observed_slot: u64,
}

#[derive(Clone, Debug)]
pub enum SolanaHostEvent {
    FheBinaryOp(FheBinaryOpEvent),
    FheTernaryOp(FheTernaryOpEvent),
    TrivialEncrypt(TrivialEncryptEvent),
    FheRand(FheRandEvent),
    FinalizedAccountFetch(SolanaFinalizedAccountFetch),
}

#[derive(Clone, Debug)]
pub enum SolanaMappedEvent {
    Tfhe(Log<TfheContractEvents>),
    FinalizedAccountFetch(SolanaFinalizedAccountFetch),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolanaBlockMeta {
    pub block_number: u64,
    pub block_timestamp: time::PrimitiveDateTime,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SolanaIngestStats {
    pub tfhe_events: usize,
    pub acl_events: usize,
    pub inserted_rows: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SolanaEventDecodeError {
    MixedHostEventTransport {
        cpi_events: usize,
        log_events: usize,
    },
}

impl fmt::Display for SolanaEventDecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MixedHostEventTransport {
                cpi_events,
                log_events,
            } => write!(
                formatter,
                "transaction mixed ZamaHost CPI events ({cpi_events}) and log events ({log_events})"
            ),
        }
    }
}

impl std::error::Error for SolanaEventDecodeError {}

pub fn solana_transaction_id(signature_bytes: &[u8]) -> TransactionHash {
    let digest: [u8; 32] = Sha256::digest(signature_bytes).into();
    TransactionHash::from(digest)
}

pub fn decode_anchor_cpi_event(data: &[u8]) -> Option<SolanaHostEvent> {
    decode_solana_host_events(decode_zama_host_anchor_cpi_event(data)?)
        .into_iter()
        .next()
}

pub fn decode_anchor_event(data: &[u8]) -> Option<SolanaHostEvent> {
    decode_solana_host_events(decode_zama_host_anchor_event(data)?)
        .into_iter()
        .next()
}

/// Decode all ZamaHost events from one Solana transaction.
///
/// The inputs are intentionally RPC-client-neutral: production pollers can map
/// JSON-RPC transaction metadata, LiteSVM metadata, or SDK metadata into the
/// `(program_id, instruction_data)` iterator without making this adapter depend
/// on a specific Solana client crate. A transaction must use one host event
/// transport. Mixed host CPI/log event transport needs a chronological merge
/// before DB log indexes can be assigned, so this API rejects it explicitly.
pub fn decode_solana_transaction_events<'a>(
    logs: &[String],
    inner_instructions: impl IntoIterator<Item = (&'a str, &'a [u8])>,
    program_id: &str,
) -> Result<Vec<SolanaHostEvent>, SolanaEventDecodeError> {
    let cpi_events = decode_anchor_cpi_events(inner_instructions, program_id);
    let log_events = decode_anchor_log_events(logs, program_id);
    merge_solana_transport_events(cpi_events, log_events)
}

pub fn decode_anchor_cpi_events<'a>(
    inner_instructions: impl IntoIterator<Item = (&'a str, &'a [u8])>,
    program_id: &str,
) -> Vec<SolanaHostEvent> {
    inner_instructions
        .into_iter()
        .filter(|(instruction_program_id, _)| {
            *instruction_program_id == program_id
        })
        .filter_map(|(_, data)| decode_zama_host_anchor_cpi_event(data))
        .flat_map(decode_solana_host_events)
        .collect()
}

pub fn decode_anchor_log_events(
    logs: &[String],
    program_id: &str,
) -> Vec<SolanaHostEvent> {
    let mut stack = Vec::<&str>::new();
    let mut events = Vec::new();

    for log in logs {
        if let Some(invoked_program) = parse_program_invoke(log) {
            stack.push(invoked_program);
            continue;
        }

        if let Some(exited_program) = parse_program_exit(log) {
            pop_program_stack(&mut stack, exited_program);
            continue;
        }

        if stack.last().copied() != Some(program_id) {
            continue;
        }

        let Some(encoded) = log.strip_prefix("Program data: ") else {
            continue;
        };
        let Ok(data) = BASE64_STANDARD.decode(encoded) else {
            continue;
        };
        if let Some(event) = decode_zama_host_anchor_event(&data) {
            events.extend(decode_solana_host_events(event));
        }
    }

    events
}

/// Event-source policy for ZamaHost events in one transaction. Neither
/// transport is "preferred": a single `fhe_eval` frame emits all of its events
/// through exactly one transport (the host picks CPI or log per frame), so a
/// real transaction never spreads order-sensitive compute events across both.
///
/// - only one transport present: return its events unchanged;
/// - both present and BOTH carry order-sensitive compute events
///   (see [`needs_ordered_db_log_index`]): reject, because their chronological
///   interleaving cannot be reconstructed for DB log indexing;
/// - both present but at most one side carries compute events (e.g. CPI compute
///   events alongside log-only account-fetch events): concatenate CPI events
///   first, then log events. Order is unambiguous because only one side needs
///   ordered indexing.
pub fn merge_solana_transport_events(
    cpi_events: Vec<SolanaHostEvent>,
    log_events: Vec<SolanaHostEvent>,
) -> Result<Vec<SolanaHostEvent>, SolanaEventDecodeError> {
    match (cpi_events.is_empty(), log_events.is_empty()) {
        (true, _) => Ok(log_events),
        (_, true) => Ok(cpi_events),
        (false, false)
            if cpi_events.iter().any(needs_ordered_db_log_index)
                && log_events.iter().any(needs_ordered_db_log_index) =>
        {
            Err(SolanaEventDecodeError::MixedHostEventTransport {
                cpi_events: cpi_events.len(),
                log_events: log_events.len(),
            })
        }
        (false, false) => {
            Ok(cpi_events.into_iter().chain(log_events).collect())
        }
    }
}

fn needs_ordered_db_log_index(event: &SolanaHostEvent) -> bool {
    matches!(
        event,
        SolanaHostEvent::FheBinaryOp(_)
            | SolanaHostEvent::FheTernaryOp(_)
            | SolanaHostEvent::TrivialEncrypt(_)
            | SolanaHostEvent::FheRand(_)
    )
}

pub fn decode_solana_transaction_account_fetches<'a>(
    logs: &[String],
    inner_instructions: impl IntoIterator<Item = (&'a str, &'a [u8])>,
    host_program_id: &str,
    confidential_token_program_id: &str,
) -> Result<Vec<SolanaFinalizedAccountFetch>, SolanaEventDecodeError> {
    let inner_instructions = inner_instructions.into_iter().collect::<Vec<_>>();
    let host_events = decode_solana_transaction_events(
        logs,
        inner_instructions.iter().copied(),
        host_program_id,
    )?;
    let mut fetches = account_fetches_from_solana_events(host_events);
    fetches.extend(decode_confidential_token_anchor_cpi_account_fetches(
        inner_instructions.iter().copied(),
        confidential_token_program_id,
    ));
    fetches.extend(decode_confidential_token_anchor_log_account_fetches(
        logs,
        confidential_token_program_id,
    ));
    dedup_account_fetches(&mut fetches);
    Ok(fetches)
}

pub fn decode_confidential_token_anchor_cpi_account_fetches<'a>(
    inner_instructions: impl IntoIterator<Item = (&'a str, &'a [u8])>,
    program_id: &str,
) -> Vec<SolanaFinalizedAccountFetch> {
    inner_instructions
        .into_iter()
        .filter(|(instruction_program_id, _)| {
            *instruction_program_id == program_id
        })
        .filter_map(|(_, data)| {
            decode_confidential_token_anchor_cpi_event(data)
        })
        .flat_map(decode_confidential_token_account_fetches)
        .collect()
}

pub fn decode_confidential_token_anchor_log_account_fetches(
    logs: &[String],
    program_id: &str,
) -> Vec<SolanaFinalizedAccountFetch> {
    let mut stack = Vec::<&str>::new();
    let mut fetches = Vec::new();

    for log in logs {
        if let Some(invoked_program) = parse_program_invoke(log) {
            stack.push(invoked_program);
            continue;
        }

        if let Some(exited_program) = parse_program_exit(log) {
            pop_program_stack(&mut stack, exited_program);
            continue;
        }

        if stack.last().copied() != Some(program_id) {
            continue;
        }

        let Some(encoded) = log.strip_prefix("Program data: ") else {
            continue;
        };
        let Ok(data) = BASE64_STANDARD.decode(encoded) else {
            continue;
        };
        if let Some(event) = decode_confidential_token_anchor_event(&data) {
            fetches.extend(decode_confidential_token_account_fetches(event));
        }
    }

    fetches
}

fn parse_program_invoke(log: &str) -> Option<&str> {
    let rest = log.strip_prefix("Program ")?;
    let (program_id, depth) = rest.split_once(" invoke [")?;
    depth.strip_suffix(']')?;
    Some(program_id)
}

fn parse_program_exit(log: &str) -> Option<&str> {
    let rest = log.strip_prefix("Program ")?;
    if let Some(program_id) = rest.strip_suffix(" success") {
        return Some(program_id);
    }

    rest.split_once(" failed: ")
        .map(|(program_id, _)| program_id)
}

fn pop_program_stack(stack: &mut Vec<&str>, program_id: &str) {
    if stack.last().copied() == Some(program_id) {
        stack.pop();
        return;
    }

    if let Some(index) = stack.iter().rposition(|entry| *entry == program_id) {
        stack.truncate(index);
    }
}

fn decode_solana_host_events(event: ZamaHostEvent) -> Vec<SolanaHostEvent> {
    if zama_host_event_version(&event) != EVENT_VERSION {
        return Vec::new();
    }
    match event {
        ZamaHostEvent::FheBinaryOp(event) => {
            vec![SolanaHostEvent::FheBinaryOp(event)]
        }
        ZamaHostEvent::FheTernaryOp(event) => {
            vec![SolanaHostEvent::FheTernaryOp(event)]
        }
        ZamaHostEvent::TrivialEncrypt(event) => {
            vec![SolanaHostEvent::TrivialEncrypt(event)]
        }
        ZamaHostEvent::FheRand(event) => vec![SolanaHostEvent::FheRand(event)],
        ZamaHostEvent::DenySubjectUpdated(_)
        | ZamaHostEvent::HostConfigInitialized(_)
        | ZamaHostEvent::HostConfigUpdated(_)
        | ZamaHostEvent::NewKmsContext(_)
        | ZamaHostEvent::KmsContextDestroyed(_)
        | ZamaHostEvent::UserDecryptionDelegationUpdated(_) => Vec::new(),
    }
}

fn decode_confidential_token_account_fetches(
    event: ConfidentialTokenEvent,
) -> Vec<SolanaFinalizedAccountFetch> {
    if confidential_token_event_version(&event)
        != CONFIDENTIAL_TOKEN_EVENT_VERSION
    {
        return Vec::new();
    }
    match event {
        ConfidentialTokenEvent::AmountDisclosureRequested(event) => {
            vec![request_witness_fetch(
                event.request,
                SolanaFinalizedAccountFetchKind::DisclosureRequest,
                "amount_disclosure_requested",
            )]
        }
        ConfidentialTokenEvent::BalanceDisclosureRequested(event) => {
            vec![request_witness_fetch(
                event.request,
                SolanaFinalizedAccountFetchKind::DisclosureRequest,
                "balance_disclosure_requested",
            )]
        }
        ConfidentialTokenEvent::BurnRedemptionRequested(event) => {
            vec![request_witness_fetch(
                event.request,
                SolanaFinalizedAccountFetchKind::BurnRedemptionRequest,
                "burn_redemption_requested",
            )]
        }
        ConfidentialTokenEvent::AmountDisclosed(_)
        | ConfidentialTokenEvent::BalanceDisclosed(_)
        | ConfidentialTokenEvent::BalanceHandleUpdated(_)
        | ConfidentialTokenEvent::BurnRedeemed(_)
        | ConfidentialTokenEvent::ConfidentialBurn(_)
        | ConfidentialTokenEvent::ConfidentialTransfer(_)
        | ConfidentialTokenEvent::RandomAmountCreated(_)
        | ConfidentialTokenEvent::TotalSupplyHandleUpdated(_) => Vec::new(),
    }
}

fn confidential_token_event_version(event: &ConfidentialTokenEvent) -> u8 {
    match event {
        ConfidentialTokenEvent::AmountDisclosed(event) => event.version,
        ConfidentialTokenEvent::AmountDisclosureRequested(event) => {
            event.version
        }
        ConfidentialTokenEvent::BalanceDisclosed(event) => event.version,
        ConfidentialTokenEvent::BalanceDisclosureRequested(event) => {
            event.version
        }
        ConfidentialTokenEvent::BalanceHandleUpdated(event) => event.version,
        ConfidentialTokenEvent::BurnRedeemed(event) => event.version,
        ConfidentialTokenEvent::BurnRedemptionRequested(event) => event.version,
        ConfidentialTokenEvent::ConfidentialBurn(event) => event.version,
        ConfidentialTokenEvent::ConfidentialTransfer(event) => event.version,
        ConfidentialTokenEvent::RandomAmountCreated(event) => event.version,
        ConfidentialTokenEvent::TotalSupplyHandleUpdated(event) => {
            event.version
        }
    }
}

fn account_fetches_from_solana_events(
    events: impl IntoIterator<Item = SolanaHostEvent>,
) -> Vec<SolanaFinalizedAccountFetch> {
    events
        .into_iter()
        .filter_map(|event| match event {
            SolanaHostEvent::FinalizedAccountFetch(fetch) => Some(fetch),
            _ => None,
        })
        .collect()
}

// Only referenced by `solana_reconstruct` (feature-gated) outside of tests.
#[cfg_attr(not(feature = "solana-reconstruct"), allow(dead_code))]
pub(crate) fn acl_record_fetch(
    acl_record: [u8; 32],
    handle: [u8; 32],
    reason: &'static str,
) -> SolanaFinalizedAccountFetch {
    SolanaFinalizedAccountFetch {
        account_key: acl_record,
        kind: SolanaFinalizedAccountFetchKind::EncryptedValueAccount,
        reason,
        handle: Some(Handle::from(handle)),
        related_account: None,
        subject: None,
    }
}

fn request_witness_fetch(
    request: [u8; 32],
    kind: SolanaFinalizedAccountFetchKind,
    reason: &'static str,
) -> SolanaFinalizedAccountFetch {
    SolanaFinalizedAccountFetch {
        account_key: request,
        kind,
        reason,
        handle: None,
        related_account: None,
        subject: None,
    }
}

fn dedup_account_fetches(fetches: &mut Vec<SolanaFinalizedAccountFetch>) {
    let mut seen = HashSet::new();
    fetches.retain(|fetch| {
        seen.insert((fetch.account_key, fetch.kind, fetch.reason, fetch.handle))
    });
}

fn zama_host_event_version(event: &ZamaHostEvent) -> u8 {
    match event {
        ZamaHostEvent::DenySubjectUpdated(event) => event.version,
        ZamaHostEvent::FheBinaryOp(event) => event.version,
        ZamaHostEvent::FheRand(event) => event.version,
        ZamaHostEvent::FheTernaryOp(event) => event.version,
        ZamaHostEvent::HostConfigInitialized(event) => event.version,
        ZamaHostEvent::HostConfigUpdated(event) => event.version,
        ZamaHostEvent::KmsContextDestroyed(event) => event.version,
        ZamaHostEvent::NewKmsContext(event) => event.version,
        ZamaHostEvent::TrivialEncrypt(event) => event.version,
        ZamaHostEvent::UserDecryptionDelegationUpdated(event) => event.version,
    }
}

pub fn map_solana_event(event: SolanaHostEvent) -> SolanaMappedEvent {
    match event {
        SolanaHostEvent::FheBinaryOp(event) => {
            SolanaMappedEvent::Tfhe(to_tfhe_event(event))
        }
        SolanaHostEvent::FheTernaryOp(event) => {
            SolanaMappedEvent::Tfhe(to_tfhe_ternary_event(event))
        }
        SolanaHostEvent::TrivialEncrypt(event) => {
            SolanaMappedEvent::Tfhe(to_trivial_encrypt_event(event))
        }
        SolanaHostEvent::FheRand(event) => {
            SolanaMappedEvent::Tfhe(to_fhe_rand_event(event))
        }
        SolanaHostEvent::FinalizedAccountFetch(fetch) => {
            SolanaMappedEvent::FinalizedAccountFetch(fetch)
        }
    }
}

// --- Eager compute scheduling (RFC-024 / Q11 option A) ---------------------
//
// Historically, a Solana compute's `is_allowed` bit was derived from allow
// events observed in the SAME transaction, and a since-deleted
// `mark_solana_computation_allowed` re-opened it later if the allow instead
// landed in a following slot. That machinery is now retired as a *scheduling*
// gate: every Solana
// computation is inserted eager (`is_allowed = TRUE` unconditionally), so the
// tfhe-worker schedules it immediately regardless of ACL/allow state. Allow
// signals (`create_encrypted_value`, `allow_subjects`, `update_encrypted_value`,
// `make_handle_public`) still matter for DECRYPT availability (SNS/ct128 prep,
// `allowed_handles`) — they are not needed to unblock computation.
//
// Reorg unwind for eagerly-scheduled computations remains unimplemented: a
// computation whose containing block loses a fork race is wasted work, not a
// correctness bug, because `solana_finalized_account_fetcher` is the sole gate
// on releasing a decrypt for a *finalized* handle+subject/public state. Eager
// compute can never cause a wrong decrypt; at worst it burns cycles on a
// minority fork.
pub fn normalize_solana_events_for_db(
    events: impl IntoIterator<Item = SolanaHostEvent>,
    transaction_id: TransactionHash,
    block: SolanaBlockMeta,
) -> (Vec<LogTfhe>, Vec<SolanaFinalizedAccountFetch>) {
    let events = events.into_iter().collect::<Vec<_>>();

    let mut tfhe_logs = Vec::new();
    let mut account_fetches = Vec::new();

    for (index, event) in events.into_iter().enumerate() {
        match map_solana_event(event) {
            SolanaMappedEvent::Tfhe(event) => {
                // Eager: schedulable the moment the compute itself confirms,
                // independent of any allow/ACL signal. See module note above.
                tfhe_logs.push(to_log_tfhe(
                    event,
                    transaction_id,
                    block,
                    true,
                    index as u64,
                ));
            }
            SolanaMappedEvent::FinalizedAccountFetch(fetch) => {
                account_fetches.push(fetch);
            }
        }
    }

    dedup_account_fetches(&mut account_fetches);
    (tfhe_logs, account_fetches)
}

/// Whether a finalized-account-fetch reason denotes an ACL allow signal that
/// feeds decrypt availability (vs. a material-commitment or request witness).
/// Used only by the finalized decrypt-release consumer now — no longer by any
/// compute-scheduling path (see the eager-compute note above).
///
/// `encrypted_value_created` / `handle_superseded` / `handle_made_public` /
/// `subject_allowed` name the RFC-024 `EncryptedValue` instruction-derived
/// signals (`create_encrypted_value`, `update_encrypted_value`,
/// `make_handle_public`, `allow_subjects`); the legacy `public_decrypt_allowed`
/// / `acl_subject_allowed` / `acl_record_bound` reasons are kept for the
/// still-IDL-driven decode path pending its RFC-024 instruction-decode rewrite
/// (tracked separately — see host-listener README/TODO).
///
/// TODO(RFC-024 instruction decode): currently unreferenced because nothing
/// yet emits `SolanaFinalizedAccountFetch::reason` from the new
/// `EncryptedValue` instructions (`create_encrypted_value`/`allow_subjects`/
/// `update_encrypted_value`/`make_handle_public`) — see the host-listener
/// section of the RFC-024 rollout notes. Once that decode lands, this becomes
/// the finalized-fetcher's filter again.
#[allow(dead_code)]
pub(crate) fn is_solana_allow_reason(reason: &str) -> bool {
    matches!(
        reason,
        "public_decrypt_allowed"
            | "acl_subject_allowed"
            | "acl_record_bound"
            | "encrypted_value_created"
            | "handle_superseded"
            | "handle_made_public"
            | "subject_allowed"
    )
}

pub async fn insert_solana_events(
    db: &Database,
    tx: &mut Transaction<'_>,
    events: impl IntoIterator<Item = SolanaHostEvent>,
    transaction_id: TransactionHash,
    block: SolanaBlockMeta,
) -> Result<SolanaIngestStats, SqlxError> {
    let (mut tfhe_logs, account_fetches) =
        normalize_solana_events_for_db(events, transaction_id, block);
    let mut inserted_rows = 0;

    let chains =
        dependence_chains(&mut tfhe_logs, &db.dependence_chain, false, true)
            .await;

    let mut inserted_compute = false;
    for log in &tfhe_logs {
        if db.insert_tfhe_event(tx, log).await? {
            inserted_rows += 1;
            inserted_compute = true;
        }
    }
    for fetch in &account_fetches {
        if insert_finalized_account_fetch(
            tx,
            fetch,
            transaction_id,
            block.block_number,
        )
        .await?
        {
            inserted_rows += 1;
        }
        // No re-open needed: computations are inserted eager (is_allowed=TRUE
        // unconditionally, see `normalize_solana_events_for_db`), so there is
        // nothing left to activate here. This fetch only feeds the finalized
        // decrypt-release gate (`solana_finalized_account_fetcher`).
    }

    // Populate the dependence_chain scheduling table the tfhe-worker locks against; without
    // it the inserted computations are never scheduled (the EVM ingest path likewise calls
    // update_dependence_chain after inserting tfhe events). Solana host slots carry no
    // EVM-style block hash, so derive a unique per-slot hash from the slot number — it is used
    // only for reorg bookkeeping, which a single local validator never exercises.
    if inserted_compute {
        let mut block_hash = [0u8; 32];
        block_hash[24..32].copy_from_slice(&block.block_number.to_be_bytes());
        let block_summary = BlockSummary {
            number: block.block_number,
            hash: FixedBytes::<32>::from(block_hash),
            parent_hash: FixedBytes::<32>::ZERO,
            timestamp: 0,
        };
        db.update_dependence_chain(
            tx,
            chains,
            block.block_timestamp,
            &block_summary,
            &HashSet::new(),
        )
        .await?;
    }

    Ok(SolanaIngestStats {
        tfhe_events: tfhe_logs.len(),
        acl_events: account_fetches.len(),
        inserted_rows,
    })
}

async fn insert_finalized_account_fetch(
    tx: &mut Transaction<'_>,
    fetch: &SolanaFinalizedAccountFetch,
    transaction_id: TransactionHash,
    block_number: u64,
) -> Result<bool, SqlxError> {
    let handle = fetch.handle.map(|handle| handle.to_vec());
    let handle_key = finalized_fetch_handle_key(fetch.handle);
    let related_account = fetch.related_account.map(|account| account.to_vec());
    let subject = fetch.subject.map(|subject| subject.to_vec());
    sqlx::query(
        r#"
        INSERT INTO solana_finalized_account_fetches (
            account_key,
            kind,
            reason,
            handle,
            handle_key,
            related_account,
            subject,
            transaction_id,
            block_number
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (account_key, kind, reason, handle_key) DO UPDATE SET
            related_account = COALESCE(
                EXCLUDED.related_account,
                solana_finalized_account_fetches.related_account
            ),
            subject = COALESCE(EXCLUDED.subject, solana_finalized_account_fetches.subject),
            transaction_id = EXCLUDED.transaction_id,
            block_number = EXCLUDED.block_number,
            status = 'pending',
            last_seen_at = NOW()
        "#,
    )
    .bind(fetch.account_key.to_vec())
    .bind(fetch.kind.as_i16())
    .bind(fetch.reason)
    .bind(handle)
    .bind(handle_key)
    .bind(related_account)
    .bind(subject)
    .bind(transaction_id.to_vec())
    .bind(block_number as i64)
    .execute(tx.deref_mut())
    .await
    .map(|result| result.rows_affected() > 0)
}

fn finalized_fetch_handle_key(handle: Option<Handle>) -> Vec<u8> {
    let mut key = Vec::with_capacity(33);
    match handle {
        Some(handle) => {
            key.push(1);
            key.extend_from_slice(handle.as_slice());
        }
        None => {
            key.push(0);
            key.extend_from_slice(&[0u8; 32]);
        }
    }
    key
}

pub async fn claim_pending_finalized_account_fetches(
    tx: &mut Transaction<'_>,
    limit: i64,
) -> Result<Vec<SolanaFinalizedAccountFetchJob>, SqlxError> {
    let rows = sqlx::query(
        r#"
        WITH candidate AS (
            SELECT account_key, kind, reason, handle_key
            FROM solana_finalized_account_fetches
            WHERE status = 'pending'
               OR (
                    status = 'processing'
                    AND last_seen_at < NOW() - INTERVAL '5 minutes'
               )
            ORDER BY block_number, last_seen_at
            LIMIT $1
            FOR UPDATE SKIP LOCKED
        )
        UPDATE solana_finalized_account_fetches AS claimed
        SET
            status = 'processing',
            attempts = claimed.attempts + 1,
            last_seen_at = NOW()
        FROM candidate
        WHERE claimed.account_key = candidate.account_key
          AND claimed.kind = candidate.kind
          AND claimed.reason = candidate.reason
          AND claimed.handle_key = candidate.handle_key
        RETURNING
            claimed.account_key,
            claimed.kind,
            claimed.reason,
            claimed.handle,
            claimed.related_account,
            claimed.subject,
            claimed.transaction_id,
            claimed.block_number,
            claimed.attempts
        "#,
    )
    .bind(limit)
    .fetch_all(tx.deref_mut())
    .await?;

    Ok(rows
        .into_iter()
        .map(finalized_account_fetch_job_from_row)
        .collect())
}

pub async fn store_finalized_account_witness(
    tx: &mut Transaction<'_>,
    witness: &SolanaFinalizedAccountWitness,
) -> Result<(), SqlxError> {
    let lamports = i64::try_from(witness.lamports).map_err(|err| {
        SqlxError::Decode(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "Solana lamports exceed host-listener BIGINT storage: {err}"
            ),
        )))
    })?;
    let observed_slot = i64::try_from(witness.observed_slot).map_err(|err| {
        SqlxError::Decode(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Solana observed slot exceeds host-listener BIGINT storage: {err}"),
        )))
    })?;
    sqlx::query(
        r#"
        INSERT INTO solana_finalized_account_witnesses (
            account_key,
            owner,
            lamports,
            executable,
            data,
            observed_slot
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (account_key) DO UPDATE SET
            owner = EXCLUDED.owner,
            lamports = EXCLUDED.lamports,
            executable = EXCLUDED.executable,
            data = EXCLUDED.data,
            observed_slot = EXCLUDED.observed_slot,
            fetched_at = NOW()
        "#,
    )
    .bind(witness.account_key.to_vec())
    .bind(witness.owner.to_vec())
    .bind(lamports)
    .bind(witness.executable)
    .bind(witness.data.as_slice())
    .bind(observed_slot)
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

pub async fn complete_finalized_account_fetch(
    tx: &mut Transaction<'_>,
    job: &SolanaFinalizedAccountFetchJob,
) -> Result<(), SqlxError> {
    let handle_key = finalized_fetch_handle_key(job.handle);
    sqlx::query(
        r#"
        UPDATE solana_finalized_account_fetches
        SET status = 'done', last_error = NULL, last_seen_at = NOW()
        WHERE account_key = $1
          AND kind = $2
          AND reason = $3
          AND handle_key = $4
        "#,
    )
    .bind(job.account_key.to_vec())
    .bind(job.kind.as_i16())
    .bind(&job.reason)
    .bind(handle_key)
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

pub async fn fail_finalized_account_fetch(
    tx: &mut Transaction<'_>,
    job: &SolanaFinalizedAccountFetchJob,
    error: &str,
) -> Result<(), SqlxError> {
    let handle_key = finalized_fetch_handle_key(job.handle);
    sqlx::query(
        r#"
        UPDATE solana_finalized_account_fetches
        SET status = 'error', last_error = $5, last_seen_at = NOW()
        WHERE account_key = $1
          AND kind = $2
          AND reason = $3
          AND handle_key = $4
        "#,
    )
    .bind(job.account_key.to_vec())
    .bind(job.kind.as_i16())
    .bind(&job.reason)
    .bind(handle_key)
    .bind(error)
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

pub async fn retry_finalized_account_fetch(
    tx: &mut Transaction<'_>,
    job: &SolanaFinalizedAccountFetchJob,
    error: &str,
) -> Result<(), SqlxError> {
    let handle_key = finalized_fetch_handle_key(job.handle);
    sqlx::query(
        r#"
        UPDATE solana_finalized_account_fetches
        SET status = 'pending', last_error = $5, last_seen_at = NOW()
        WHERE account_key = $1
          AND kind = $2
          AND reason = $3
          AND handle_key = $4
        "#,
    )
    .bind(job.account_key.to_vec())
    .bind(job.kind.as_i16())
    .bind(&job.reason)
    .bind(handle_key)
    .bind(error)
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

fn finalized_account_fetch_job_from_row(
    row: sqlx::postgres::PgRow,
) -> SolanaFinalizedAccountFetchJob {
    let account_key = bytes32_from_vec(row.get::<Vec<u8>, _>("account_key"));
    let kind_value = row.get::<i16, _>("kind");
    let kind = SolanaFinalizedAccountFetchKind::from_i16(kind_value).expect(
        "solana_finalized_account_fetches.kind is constrained to known values",
    );
    let handle = row
        .get::<Option<Vec<u8>>, _>("handle")
        .map(bytes32_from_vec)
        .map(Handle::from);
    let related_account = row
        .get::<Option<Vec<u8>>, _>("related_account")
        .map(bytes32_from_vec);
    let subject = row
        .get::<Option<Vec<u8>>, _>("subject")
        .map(bytes32_from_vec);
    let transaction_id =
        Handle::from(bytes32_from_vec(row.get::<Vec<u8>, _>("transaction_id")));
    let block_number = row.get::<i64, _>("block_number") as u64;
    SolanaFinalizedAccountFetchJob {
        account_key,
        kind,
        reason: row.get("reason"),
        handle,
        related_account,
        subject,
        transaction_id,
        block_number,
        attempts: row.get("attempts"),
    }
}

fn bytes32_from_vec(bytes: Vec<u8>) -> [u8; 32] {
    bytes
        .try_into()
        .expect("Solana finalized-account queue bytea field has length 32")
}

pub fn to_log_tfhe(
    event: Log<TfheContractEvents>,
    transaction_id: TransactionHash,
    block: SolanaBlockMeta,
    is_allowed: bool,
    log_index: u64,
) -> LogTfhe {
    LogTfhe {
        event,
        transaction_hash: Some(transaction_id),
        is_allowed,
        block_number: block.block_number,
        block_timestamp: block.block_timestamp,
        tx_depth_size: 0,
        dependence_chain: transaction_id,
        log_index: Some(log_index),
    }
}

/// Converts IDL-decoded Solana host events into the existing TFHE event model.
///
/// The current coprocessor worker consumes the database rows produced from
/// `TfheContractEvents`. Keeping this adapter at the typed-event boundary lets
/// the Solana listener use native Solana decoding while reusing the existing
/// computation scheduler and worker unchanged.
pub fn to_tfhe_event(event: FheBinaryOpEvent) -> Log<TfheContractEvents> {
    let caller = Address::ZERO;
    let scalar_byte = FixedBytes::<1>::from([u8::from(event.scalar)]);
    let data = match event.op {
        FheBinaryOpCode::Add => {
            TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller,
                lhs: Handle::from(event.lhs),
                rhs: Handle::from(event.rhs),
                scalarByte: scalar_byte,
                result: Handle::from(event.result),
            })
        }
        FheBinaryOpCode::Sub => {
            TfheContractEvents::FheSub(TfheContract::FheSub {
                caller,
                lhs: Handle::from(event.lhs),
                rhs: Handle::from(event.rhs),
                scalarByte: scalar_byte,
                result: Handle::from(event.result),
            })
        }
        FheBinaryOpCode::Ge => TfheContractEvents::FheGe(TfheContract::FheGe {
            caller,
            lhs: Handle::from(event.lhs),
            rhs: Handle::from(event.rhs),
            scalarByte: scalar_byte,
            result: Handle::from(event.result),
        }),
    };

    Log {
        address: caller,
        data,
    }
}

pub fn to_tfhe_ternary_event(
    event: FheTernaryOpEvent,
) -> Log<TfheContractEvents> {
    let caller = Address::ZERO;
    let data = match event.op {
        FheTernaryOpCode::IfThenElse => {
            TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                caller,
                control: Handle::from(event.control),
                ifTrue: Handle::from(event.if_true),
                ifFalse: Handle::from(event.if_false),
                result: Handle::from(event.result),
            })
        }
    };

    Log {
        address: caller,
        data,
    }
}

pub fn to_trivial_encrypt_event(
    event: TrivialEncryptEvent,
) -> Log<TfheContractEvents> {
    let caller = Address::ZERO;
    Log {
        address: caller,
        data: TfheContractEvents::TrivialEncrypt(
            TfheContract::TrivialEncrypt {
                caller,
                pt: ClearConst::from_be_slice(&event.plaintext),
                toType: event.fhe_type,
                result: Handle::from(event.result),
            },
        ),
    }
}

pub fn to_fhe_rand_event(event: FheRandEvent) -> Log<TfheContractEvents> {
    let caller = Address::ZERO;
    Log {
        address: caller,
        data: TfheContractEvents::FheRand(TfheContract::FheRand {
            caller,
            randType: event.fhe_type,
            seed: FixedBytes::<16>::from(event.seed),
            result: Handle::from(event.result),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::{
        anchor_event_discriminator, ANCHOR_EVENT_IX_TAG_LE, EVENT_VERSION,
    };
    use time::{Date, Month, PrimitiveDateTime, Time};

    fn handle(byte: u8) -> Handle {
        Handle::from([byte; 32])
    }

    #[test]
    fn decodes_anchor_event_cpi_binary_event_to_existing_tfhe_event() {
        let encoded = anchor_event_cpi(
            "FheBinaryOpEvent",
            binary_op_payload(1, [9; 32], [1; 32], [2; 32], false, [3; 32]),
        );

        let decoded = decode_anchor_cpi_event(&encoded)
            .expect("expected binary op event");
        let SolanaMappedEvent::Tfhe(mapped) = map_solana_event(decoded) else {
            panic!("expected mapped TFHE event");
        };

        assert!(matches!(
            mapped.data,
            TfheContractEvents::FheSub(TfheContract::FheSub {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) if lhs == handle(1)
                && rhs == handle(2)
                && scalarByte == FixedBytes::<1>::from([0])
                && result == handle(3)
        ));
    }

    #[test]
    fn decodes_anchor_log_event_binary_event_to_existing_tfhe_event() {
        let encoded = anchor_event(
            "FheBinaryOpEvent",
            binary_op_payload(1, [9; 32], [1; 32], [2; 32], false, [3; 32]),
        );

        let decoded =
            decode_anchor_event(&encoded).expect("expected binary op event");
        let SolanaMappedEvent::Tfhe(mapped) = map_solana_event(decoded) else {
            panic!("expected mapped TFHE event");
        };

        assert!(matches!(
            mapped.data,
            TfheContractEvents::FheSub(TfheContract::FheSub {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) if lhs == handle(1)
                && rhs == handle(2)
                && scalarByte == FixedBytes::<1>::from([0])
                && result == handle(3)
        ));
    }

    #[test]
    fn decodes_threshold_sized_host_program_data_logs_in_order() {
        let host_program = "ZamaHost111111111111111111111111111111111";
        let other_program = "Other111111111111111111111111111111111111";
        let ignored = BASE64_STANDARD.encode(anchor_event(
            "FheBinaryOpEvent",
            binary_op_payload(1, [9; 32], [1; 32], [2; 32], false, [99; 32]),
        ));
        let mut logs = vec![
            format!("Program {host_program} invoke [1]"),
            format!("Program {other_program} invoke [2]"),
            format!("Program data: {ignored}"),
            format!("Program {other_program} success"),
            "Program data: not base64".to_owned(),
        ];

        for value in 0_u8..12 {
            logs.push(format!(
                "Program data: {}",
                BASE64_STANDARD.encode(anchor_event(
                    "FheBinaryOpEvent",
                    binary_op_payload(
                        1,
                        [9; 32],
                        [1; 32],
                        [2; 32],
                        false,
                        [value; 32],
                    ),
                ))
            ));
        }
        logs.push(format!("Program {host_program} success"));
        logs.push(format!("Program data: {ignored}"));

        let events = decode_anchor_log_events(&logs, host_program);

        assert_eq!(events.len(), 12);
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(
                    event,
                    SolanaHostEvent::FheBinaryOp(_)
                ))
                .count(),
            12
        );
        assert!(matches!(
            &events[0],
            SolanaHostEvent::FheBinaryOp(event) if event.result == [0; 32]
        ));
        assert!(matches!(
            &events[5],
            SolanaHostEvent::FheBinaryOp(event) if event.result == [5; 32]
        ));
    }

    #[test]
    fn decodes_solana_transaction_log_transport_events() {
        let host_program = "ZamaHost111111111111111111111111111111111";
        let logs = vec![
            format!("Program {host_program} invoke [1]"),
            format!(
                "Program data: {}",
                BASE64_STANDARD.encode(anchor_event(
                    "FheBinaryOpEvent",
                    binary_op_payload(
                        0, [9; 32], [1; 32], [2; 32], false, [3; 32],
                    ),
                ))
            ),
            format!("Program {host_program} success"),
        ];

        let events = decode_solana_transaction_events(&logs, [], host_program)
            .expect("log transport should decode");

        assert!(matches!(
            events.as_slice(),
            [SolanaHostEvent::FheBinaryOp(event)] if event.result == [3; 32]
        ));
    }

    #[test]
    fn failed_inner_program_exit_restores_host_log_scope() {
        // A nested program that fails (rather than succeeds) must still pop the
        // stack so subsequent host `Program data:` logs are attributed to the
        // host program, not silently dropped.
        let host_program = "ZamaHost111111111111111111111111111111111";
        let inner_program = "Other111111111111111111111111111111111111";
        let host_event = BASE64_STANDARD.encode(anchor_event(
            "FheBinaryOpEvent",
            binary_op_payload(1, [9; 32], [1; 32], [2; 32], false, [7; 32]),
        ));
        let logs = vec![
            format!("Program {host_program} invoke [1]"),
            format!("Program {inner_program} invoke [2]"),
            format!("Program {inner_program} failed: custom error"),
            format!("Program data: {host_event}"),
            format!("Program {host_program} success"),
        ];

        let events = decode_anchor_log_events(&logs, host_program);

        assert!(matches!(
            events.as_slice(),
            [SolanaHostEvent::FheBinaryOp(event)] if event.result == [7; 32]
        ));
    }

    #[test]
    fn out_of_order_program_exit_truncates_log_scope_stack() {
        // If an exit names a program deeper in the stack, everything above it is
        // truncated. A host event logged afterward is then outside host scope
        // and must be ignored.
        let host_program = "ZamaHost111111111111111111111111111111111";
        let inner_program = "Other111111111111111111111111111111111111";
        let host_event = BASE64_STANDARD.encode(anchor_event(
            "FheBinaryOpEvent",
            binary_op_payload(1, [9; 32], [1; 32], [2; 32], false, [7; 32]),
        ));
        let logs = vec![
            format!("Program {host_program} invoke [1]"),
            format!("Program {inner_program} invoke [2]"),
            // Exit names the host (below `inner` on the stack): truncates both.
            format!("Program {host_program} success"),
            format!("Program data: {host_event}"),
        ];

        let events = decode_anchor_log_events(&logs, host_program);

        assert!(events.is_empty());
    }

    #[test]
    fn rejects_mixed_host_cpi_and_log_transport() {
        let host_program = "ZamaHost111111111111111111111111111111111";
        let cpi_event = anchor_event_cpi(
            "FheBinaryOpEvent",
            binary_op_payload(0, [9; 32], [1; 32], [2; 32], false, [3; 32]),
        );
        let logs = vec![
            format!("Program {host_program} invoke [1]"),
            format!(
                "Program data: {}",
                BASE64_STANDARD.encode(anchor_event(
                    "FheBinaryOpEvent",
                    binary_op_payload(
                        0, [9; 32], [1; 32], [2; 32], false, [4; 32],
                    ),
                ))
            ),
            format!("Program {host_program} success"),
        ];

        let error = decode_solana_transaction_events(
            &logs,
            [(host_program, cpi_event.as_slice())],
            host_program,
        )
        .expect_err("mixed host transport should be rejected");

        assert_eq!(
            error,
            SolanaEventDecodeError::MixedHostEventTransport {
                cpi_events: 1,
                log_events: 1,
            }
        );
    }

    fn compute_event(result: u8) -> SolanaHostEvent {
        SolanaHostEvent::FheBinaryOp(FheBinaryOpEvent {
            version: EVENT_VERSION,
            op: FheBinaryOpCode::Add,
            subject: [0; 32],
            lhs: [1; 32],
            rhs: [2; 32],
            scalar: false,
            result: [result; 32],
        })
    }

    fn fetch_event(account: u8) -> SolanaHostEvent {
        SolanaHostEvent::FinalizedAccountFetch(acl_record_fetch(
            [account; 32],
            [account; 32],
            "test",
        ))
    }

    #[test]
    fn merge_returns_single_transport_unchanged() {
        // Only CPI events present: returned as-is, no log side.
        let cpi_only =
            merge_solana_transport_events(vec![compute_event(1)], Vec::new())
                .expect("cpi-only transport is valid");
        assert!(matches!(
            cpi_only.as_slice(),
            [SolanaHostEvent::FheBinaryOp(_)]
        ));

        // Only log events present: returned as-is, no cpi side.
        let log_only =
            merge_solana_transport_events(Vec::new(), vec![compute_event(2)])
                .expect("log-only transport is valid");
        assert!(matches!(
            log_only.as_slice(),
            [SolanaHostEvent::FheBinaryOp(_)]
        ));
    }

    #[test]
    fn merge_orders_cpi_events_before_log_events_when_one_side_is_compute() {
        // CPI compute event alongside log-only account-fetch events: neither
        // transport is preferred, but CPI events are emitted first so the single
        // compute side keeps a deterministic DB log index.
        let merged = merge_solana_transport_events(
            vec![compute_event(1)],
            vec![fetch_event(7), fetch_event(8)],
        )
        .expect("only the cpi side carries compute events");

        assert!(matches!(merged[0], SolanaHostEvent::FheBinaryOp(_)));
        assert!(matches!(
            merged[1],
            SolanaHostEvent::FinalizedAccountFetch(_)
        ));
        assert!(matches!(
            merged[2],
            SolanaHostEvent::FinalizedAccountFetch(_)
        ));
    }

    #[test]
    fn merge_allows_non_compute_events_on_both_transports() {
        // Account-fetch events are not order-sensitive, so both sides carrying
        // only fetches is not a mixed-transport conflict.
        let merged = merge_solana_transport_events(
            vec![fetch_event(1)],
            vec![fetch_event(2)],
        )
        .expect("non-compute events on both transports are not a conflict");
        assert_eq!(merged.len(), 2);
    }

    #[test]
    fn raw_log_event_lines_are_bounded() {
        let events = [
            anchor_event(
                "FheBinaryOpEvent",
                binary_op_payload(1, [9; 32], [1; 32], [2; 32], false, [3; 32]),
            ),
            anchor_event("FheTernaryOpEvent", {
                let mut payload = vec![EVENT_VERSION, 0];
                payload.extend_from_slice(&[9; 32]);
                payload.extend_from_slice(&[1; 32]);
                payload.extend_from_slice(&[2; 32]);
                payload.extend_from_slice(&[3; 32]);
                payload.extend_from_slice(&[4; 32]);
                payload
            }),
            anchor_event("TrivialEncryptEvent", {
                let mut payload = vec![EVENT_VERSION];
                payload.extend_from_slice(&[9; 32]);
                payload.extend_from_slice(&[0; 32]);
                payload.push(5);
                payload.extend_from_slice(&[8; 32]);
                payload
            }),
            anchor_event("FheRandEvent", {
                let mut payload = vec![EVENT_VERSION];
                payload.extend_from_slice(&[9; 32]);
                payload.extend_from_slice(&[7; 16]);
                payload.push(5);
                payload.extend_from_slice(&[8; 32]);
                payload
            }),
            anchor_event("FheRandBoundedEvent", {
                let mut payload = vec![EVENT_VERSION];
                payload.extend_from_slice(&[9; 32]);
                payload.extend_from_slice(&[1; 32]);
                payload.extend_from_slice(&[7; 16]);
                payload.push(5);
                payload.extend_from_slice(&[8; 32]);
                payload
            }),
            anchor_event(
                "AclAllowedEvent",
                acl_allowed_payload([7; 32], [8; 32]),
            ),
            anchor_event("AclSubjectAllowedEvent", {
                let mut payload = vec![EVENT_VERSION];
                payload.extend_from_slice(&[1; 32]);
                payload.extend_from_slice(&[2; 32]);
                payload.extend_from_slice(&[3; 32]);
                payload.extend_from_slice(&[4; 32]);
                payload.push(5);
                payload.extend_from_slice(&[6; 32]);
                payload.push(u8::MAX);
                payload.extend_from_slice(&7_u64.to_le_bytes());
                payload
            }),
        ];
        let longest_log_line = events
            .iter()
            .map(|event| {
                "Program data: ".len() + BASE64_STANDARD.encode(event).len()
            })
            .max()
            .expect("events");

        assert!(longest_log_line < 384, "{longest_log_line}");
    }

    #[test]
    fn decodes_anchor_event_cpi_trivial_encrypt_event() {
        let encoded = anchor_event_cpi("TrivialEncryptEvent", {
            let mut payload = vec![EVENT_VERSION];
            payload.extend_from_slice(&[9; 32]);
            payload.extend_from_slice(&[0; 31]);
            payload.push(7);
            payload.push(5);
            payload.extend_from_slice(&[8; 32]);
            payload
        });

        let decoded =
            decode_anchor_cpi_event(&encoded).expect("expected trivial event");
        let SolanaMappedEvent::Tfhe(mapped) = map_solana_event(decoded) else {
            panic!("expected mapped TFHE event");
        };

        assert!(matches!(
            mapped.data,
            TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                pt,
                toType,
                result,
                ..
            }) if pt == ClearConst::from(7_u64)
                && toType == 5
                && result == handle(8)
        ));
    }

    #[test]
    fn ignores_anchor_event_cpi_with_unsupported_event_version() {
        let mut payload =
            binary_op_payload(0, [9; 32], [1; 32], [2; 32], false, [3; 32]);
        payload[0] = EVENT_VERSION.wrapping_add(1);
        let encoded = anchor_event_cpi("FheBinaryOpEvent", payload);

        assert!(decode_anchor_cpi_event(&encoded).is_none());
    }

    #[test]
    fn confidential_token_request_cpi_events_schedule_request_witness_fetches()
    {
        let token_program = "ConfToken111111111111111111111111111111111";
        let amount_event = anchor_event_cpi(
            "AmountDisclosureRequestedEvent",
            amount_disclosure_requested_payload(5),
        );
        let balance_event = anchor_event_cpi(
            "BalanceDisclosureRequestedEvent",
            balance_disclosure_requested_payload(15),
        );
        let redemption_event = anchor_event_cpi(
            "BurnRedemptionRequestedEvent",
            burn_redemption_requested_payload(25),
        );

        let fetches = decode_confidential_token_anchor_cpi_account_fetches(
            [
                (token_program, amount_event.as_slice()),
                (token_program, balance_event.as_slice()),
                (token_program, redemption_event.as_slice()),
            ],
            token_program,
        );

        assert_eq!(fetches.len(), 3);
        assert_request_fetch(
            &fetches[0],
            5,
            SolanaFinalizedAccountFetchKind::DisclosureRequest,
            "amount_disclosure_requested",
        );
        assert_request_fetch(
            &fetches[1],
            15,
            SolanaFinalizedAccountFetchKind::DisclosureRequest,
            "balance_disclosure_requested",
        );
        assert_request_fetch(
            &fetches[2],
            25,
            SolanaFinalizedAccountFetchKind::BurnRedemptionRequest,
            "burn_redemption_requested",
        );
    }

    #[test]
    fn confidential_token_request_logs_schedule_request_witness_fetches() {
        let token_program = "ConfToken111111111111111111111111111111111";
        let logs = vec![
            format!("Program {token_program} invoke [1]"),
            program_data_log(
                "BalanceDisclosureRequestedEvent",
                balance_disclosure_requested_payload(19),
            ),
            format!("Program {token_program} success"),
        ];

        let fetches = decode_solana_transaction_account_fetches(
            &logs,
            [],
            "ZamaHost111111111111111111111111111111111",
            token_program,
        )
        .expect("account fetch scheduling should decode");

        assert_eq!(fetches.len(), 1);
        assert_request_fetch(
            &fetches[0],
            19,
            SolanaFinalizedAccountFetchKind::DisclosureRequest,
            "balance_disclosure_requested",
        );
    }

    #[test]
    fn confidential_token_response_events_do_not_schedule_closed_request_fetches(
    ) {
        let token_program = "ConfToken111111111111111111111111111111111";
        let amount_event = anchor_event_cpi(
            "AmountDisclosedEvent",
            amount_disclosed_payload(31),
        );
        let balance_event = anchor_event_cpi(
            "BalanceDisclosedEvent",
            balance_disclosed_payload(32),
        );
        let redeemed_event =
            anchor_event_cpi("BurnRedeemedEvent", burn_redeemed_payload(33));

        let fetches = decode_confidential_token_anchor_cpi_account_fetches(
            [
                (token_program, amount_event.as_slice()),
                (token_program, balance_event.as_slice()),
                (token_program, redeemed_event.as_slice()),
            ],
            token_program,
        );

        assert!(fetches.is_empty());
    }

    #[test]
    fn maps_binary_add_to_existing_tfhe_event() {
        let mapped = to_tfhe_event(FheBinaryOpEvent {
            version: EVENT_VERSION,
            op: FheBinaryOpCode::Add,
            subject: [0; 32],
            lhs: [1; 32],
            rhs: [2; 32],
            scalar: false,
            result: [3; 32],
        });

        assert!(matches!(
            mapped.data,
            TfheContractEvents::FheAdd(TfheContract::FheAdd {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) if lhs == handle(1)
                && rhs == handle(2)
                && scalarByte == FixedBytes::<1>::from([0])
                && result == handle(3)
        ));
    }

    #[test]
    fn maps_binary_ge_to_existing_tfhe_event() {
        let mapped = to_tfhe_event(FheBinaryOpEvent {
            version: EVENT_VERSION,
            op: FheBinaryOpCode::Ge,
            subject: [0; 32],
            lhs: [1; 32],
            rhs: [2; 32],
            scalar: false,
            result: [3; 32],
        });

        assert!(matches!(
            mapped.data,
            TfheContractEvents::FheGe(TfheContract::FheGe {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) if lhs == handle(1)
                && rhs == handle(2)
                && scalarByte == FixedBytes::<1>::from([0])
                && result == handle(3)
        ));
    }

    #[test]
    fn maps_ternary_if_then_else_to_existing_tfhe_event() {
        let mapped = to_tfhe_ternary_event(FheTernaryOpEvent {
            version: EVENT_VERSION,
            op: FheTernaryOpCode::IfThenElse,
            subject: [0; 32],
            control: [1; 32],
            if_true: [2; 32],
            if_false: [3; 32],
            result: [4; 32],
        });

        assert!(matches!(
            mapped.data,
            TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                control,
                ifTrue,
                ifFalse,
                result,
                ..
            }) if control == handle(1)
                && ifTrue == handle(2)
                && ifFalse == handle(3)
                && result == handle(4)
        ));
    }

    #[test]
    fn maps_trivial_encrypt_to_existing_tfhe_event() {
        let mut plaintext = [0_u8; 32];
        plaintext[31] = 7;

        let mapped = to_trivial_encrypt_event(TrivialEncryptEvent {
            version: EVENT_VERSION,
            subject: [0; 32],
            plaintext,
            fhe_type: 5,
            result: [8; 32],
        });

        assert!(matches!(
            mapped.data,
            TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                pt,
                toType,
                result,
                ..
            }) if pt == ClearConst::from(7_u64)
                && toType == 5
                && result == handle(8)
        ));
    }

    #[test]
    fn maps_random_to_existing_tfhe_event() {
        let mapped = to_fhe_rand_event(FheRandEvent {
            version: EVENT_VERSION,
            subject: [0; 32],
            seed: [7; 16],
            fhe_type: 5,
            result: [8; 32],
        });

        assert!(matches!(
            mapped.data,
            TfheContractEvents::FheRand(TfheContract::FheRand {
                randType,
                seed,
                result,
                ..
            }) if randType == 5
                && seed == FixedBytes::<16>::from([7; 16])
                && result == handle(8)
        ));
    }

    #[test]
    fn normalizes_solana_signature_to_stable_transaction_id() {
        let signature = [7_u8; 64];

        assert_eq!(
            solana_transaction_id(&signature),
            TransactionHash::from([
                0x6c, 0xfe, 0xeb, 0x3a, 0xa2, 0x5d, 0x3f, 0x41, 0x1d, 0xae,
                0x5e, 0xec, 0x17, 0xd7, 0x36, 0x9c, 0xa7, 0x15, 0x3e, 0x72,
                0xdc, 0xf5, 0x4b, 0xcf, 0x4c, 0x3d, 0xae, 0xc0, 0xf5, 0xb2,
                0x1f, 0xc7,
            ])
        );
    }

    #[test]
    fn builds_existing_db_log_shape() {
        let tx_id = solana_transaction_id(&[1_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );
        let event = to_tfhe_event(FheBinaryOpEvent {
            version: EVENT_VERSION,
            op: FheBinaryOpCode::Sub,
            subject: [0; 32],
            lhs: [1; 32],
            rhs: [2; 32],
            scalar: true,
            result: [3; 32],
        });

        let log = to_log_tfhe(
            event,
            tx_id,
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
            },
            true,
            7,
        );

        assert_eq!(log.transaction_hash, Some(tx_id));
        assert_eq!(log.block_number, 42);
        assert_eq!(log.block_timestamp, block_timestamp);
        assert!(log.is_allowed);
        assert_eq!(log.log_index, Some(7));
    }

    #[test]
    fn compute_is_eager_regardless_of_same_tx_allow_signal() {
        // Historically, the eval frame's compute would only be marked
        // materializable when an allow for its result landed in the same tx.
        // Under eager compute (RFC-024 Q11), it is unconditionally allowed;
        // the finalized fetch below separately gates the decrypt release.
        let tx_id = solana_transaction_id(&[7_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );

        let (tfhe_logs, acl_events) = normalize_solana_events_for_db(
            [
                SolanaHostEvent::TrivialEncrypt(TrivialEncryptEvent {
                    version: EVENT_VERSION,
                    subject: [0; 32],
                    plaintext: [55; 32],
                    fhe_type: 5,
                    result: [3; 32],
                }),
                SolanaHostEvent::FinalizedAccountFetch(acl_record_fetch(
                    [9; 32],
                    [3; 32],
                    "public_decrypt_allowed",
                )),
            ],
            tx_id,
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
            },
        );

        assert_eq!(tfhe_logs.len(), 1);
        assert!(
            tfhe_logs[0].is_allowed,
            "eager compute: schedulable independent of the allow signal"
        );
        // The allow is still queued for the finalized decrypt-release check.
        assert_eq!(acl_events.len(), 1);
        assert_eq!(acl_events[0].reason, "public_decrypt_allowed");
    }

    #[test]
    fn same_account_update_fetches_keep_distinct_handles_in_one_batch() {
        let tx_id = solana_transaction_id(&[9_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );

        let (_, acl_events) = normalize_solana_events_for_db(
            [
                SolanaHostEvent::FinalizedAccountFetch(acl_record_fetch(
                    [9; 32],
                    [1; 32],
                    "handle_superseded",
                )),
                SolanaHostEvent::FinalizedAccountFetch(acl_record_fetch(
                    [9; 32],
                    [2; 32],
                    "handle_superseded",
                )),
            ],
            tx_id,
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
            },
        );

        assert_eq!(acl_events.len(), 2);
        assert!(acl_events
            .iter()
            .any(|fetch| fetch.handle == Some(handle(1))));
        assert!(acl_events
            .iter()
            .any(|fetch| fetch.handle == Some(handle(2))));
    }

    #[test]
    fn unrelated_allow_handle_does_not_affect_eager_compute_result() {
        // An allow for a DIFFERENT handle is irrelevant either way under eager
        // compute: this compute is schedulable regardless.
        let tx_id = solana_transaction_id(&[8_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );

        let (tfhe_logs, _) = normalize_solana_events_for_db(
            [
                SolanaHostEvent::TrivialEncrypt(TrivialEncryptEvent {
                    version: EVENT_VERSION,
                    subject: [0; 32],
                    plaintext: [55; 32],
                    fhe_type: 5,
                    result: [3; 32],
                }),
                SolanaHostEvent::FinalizedAccountFetch(acl_record_fetch(
                    [9; 32],
                    [4; 32],
                    "public_decrypt_allowed",
                )),
            ],
            tx_id,
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
            },
        );

        assert_eq!(tfhe_logs.len(), 1);
        assert!(tfhe_logs[0].is_allowed, "eager compute: always schedulable");
    }

    #[test]
    fn normalizes_interleaved_eval_frame_events_for_worker_replay() {
        let tx_id = solana_transaction_id(&[5_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );

        let (tfhe_logs, acl_events) = normalize_solana_events_for_db(
            [
                SolanaHostEvent::TrivialEncrypt(TrivialEncryptEvent {
                    version: EVENT_VERSION,
                    subject: [0; 32],
                    plaintext: {
                        let mut plaintext = [0_u8; 32];
                        plaintext[31] = 1;
                        plaintext
                    },
                    fhe_type: 0,
                    result: [1; 32],
                }),
                SolanaHostEvent::FheRand(FheRandEvent {
                    version: EVENT_VERSION,
                    subject: [0; 32],
                    seed: [2; 16],
                    fhe_type: 5,
                    result: [2; 32],
                }),
                SolanaHostEvent::FheTernaryOp(FheTernaryOpEvent {
                    version: EVENT_VERSION,
                    op: FheTernaryOpCode::IfThenElse,
                    subject: [0; 32],
                    control: [1; 32],
                    if_true: [2; 32],
                    if_false: [1; 32],
                    result: [3; 32],
                }),
            ],
            tx_id,
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
            },
        );

        assert!(acl_events.is_empty());
        assert_eq!(tfhe_logs.len(), 3);
        assert_eq!(
            tfhe_logs
                .iter()
                .map(|log| log.log_index)
                .collect::<Vec<_>>(),
            vec![Some(0), Some(1), Some(2)]
        );
        assert!(tfhe_logs[0].is_allowed, "eager compute: always schedulable");
        assert!(tfhe_logs[1].is_allowed, "eager compute: always schedulable");
        assert!(tfhe_logs[2].is_allowed, "eager compute: always schedulable");
        assert!(matches!(
            tfhe_logs[0].event.data,
            TfheContractEvents::TrivialEncrypt(_)
        ));
        assert!(matches!(
            tfhe_logs[1].event.data,
            TfheContractEvents::FheRand(_)
        ));
        assert!(matches!(
            tfhe_logs[2].event.data,
            TfheContractEvents::FheIfThenElse(_)
        ));
    }

    fn anchor_event_cpi(name: &str, payload: Vec<u8>) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&ANCHOR_EVENT_IX_TAG_LE);
        encoded.extend_from_slice(&anchor_event(name, payload));
        encoded
    }

    fn program_data_log(name: &str, payload: Vec<u8>) -> String {
        format!(
            "Program data: {}",
            BASE64_STANDARD.encode(anchor_event(name, payload))
        )
    }

    fn anchor_event(name: &str, payload: Vec<u8>) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&anchor_event_discriminator(name));
        encoded.extend_from_slice(&payload);
        encoded
    }

    fn assert_request_fetch(
        fetch: &SolanaFinalizedAccountFetch,
        request_byte: u8,
        kind: SolanaFinalizedAccountFetchKind,
        reason: &'static str,
    ) {
        assert_eq!(fetch.account_key, [request_byte; 32]);
        assert_eq!(fetch.kind, kind);
        assert_eq!(fetch.reason, reason);
        assert_eq!(fetch.handle, None);
        assert_eq!(fetch.related_account, None);
        assert_eq!(fetch.subject, None);
    }

    fn acl_allowed_payload(handle: [u8; 32], subject: [u8; 32]) -> Vec<u8> {
        let mut payload = vec![EVENT_VERSION];
        payload.extend_from_slice(&handle);
        payload.extend_from_slice(&subject);
        payload
    }

    fn token_payload() -> Vec<u8> {
        vec![CONFIDENTIAL_TOKEN_EVENT_VERSION]
    }

    fn amount_disclosure_requested_payload(request_byte: u8) -> Vec<u8> {
        let mut payload = token_payload();
        payload.extend_from_slice(&[1; 32]);
        payload.extend_from_slice(&[2; 32]);
        payload.extend_from_slice(&[3; 32]);
        payload.extend_from_slice(&[4; 32]);
        payload.extend_from_slice(&[request_byte; 32]);
        payload.extend_from_slice(&[6; 32]);
        payload.extend_from_slice(&8_u64.to_le_bytes());
        payload.extend_from_slice(&9_u64.to_le_bytes());
        payload
    }

    fn balance_disclosure_requested_payload(request_byte: u8) -> Vec<u8> {
        let mut payload = token_payload();
        payload.extend_from_slice(&[10; 32]);
        payload.extend_from_slice(&[11; 32]);
        payload.extend_from_slice(&[12; 32]);
        payload.extend_from_slice(&[13; 32]);
        payload.extend_from_slice(&[14; 32]);
        payload.extend_from_slice(&[request_byte; 32]);
        payload.extend_from_slice(&[16; 32]);
        payload.extend_from_slice(&18_u64.to_le_bytes());
        payload.extend_from_slice(&19_u64.to_le_bytes());
        payload
    }

    fn burn_redemption_requested_payload(request_byte: u8) -> Vec<u8> {
        let mut payload = token_payload();
        payload.extend_from_slice(&[20; 32]);
        payload.extend_from_slice(&[21; 32]);
        payload.extend_from_slice(&[22; 32]);
        payload.extend_from_slice(&[23; 32]);
        payload.extend_from_slice(&[24; 32]);
        payload.extend_from_slice(&[25; 32]);
        payload.extend_from_slice(&[26; 32]);
        payload.extend_from_slice(&[request_byte; 32]);
        payload.extend_from_slice(&[28; 32]);
        payload.extend_from_slice(&30_u64.to_le_bytes());
        payload.extend_from_slice(&31_u64.to_le_bytes());
        payload
    }

    fn amount_disclosed_payload(request_byte: u8) -> Vec<u8> {
        let mut payload = token_payload();
        payload.extend_from_slice(&[40; 32]);
        payload.extend_from_slice(&[41; 32]);
        payload.extend_from_slice(&[request_byte; 32]);
        payload.extend_from_slice(&[43; 32]);
        payload.extend_from_slice(&44_u64.to_le_bytes());
        payload
    }

    fn balance_disclosed_payload(request_byte: u8) -> Vec<u8> {
        let mut payload = token_payload();
        payload.extend_from_slice(&[50; 32]);
        payload.extend_from_slice(&[51; 32]);
        payload.extend_from_slice(&[52; 32]);
        payload.extend_from_slice(&[53; 32]);
        payload.extend_from_slice(&[request_byte; 32]);
        payload.extend_from_slice(&[55; 32]);
        payload.extend_from_slice(&56_u64.to_le_bytes());
        payload
    }

    fn burn_redeemed_payload(request_byte: u8) -> Vec<u8> {
        let mut payload = token_payload();
        payload.extend_from_slice(&[60; 32]);
        payload.extend_from_slice(&[61; 32]);
        payload.extend_from_slice(&[62; 32]);
        payload.extend_from_slice(&[63; 32]);
        payload.extend_from_slice(&[64; 32]);
        payload.extend_from_slice(&[65; 32]);
        payload.extend_from_slice(&[request_byte; 32]);
        payload.extend_from_slice(&[67; 32]);
        payload.extend_from_slice(&68_u64.to_le_bytes());
        payload
    }

    fn binary_op_payload(
        op: u8,
        subject: [u8; 32],
        lhs: [u8; 32],
        rhs: [u8; 32],
        scalar: bool,
        result: [u8; 32],
    ) -> Vec<u8> {
        let mut payload = vec![EVENT_VERSION, op];
        payload.extend_from_slice(&subject);
        payload.extend_from_slice(&lhs);
        payload.extend_from_slice(&rhs);
        payload.push(u8::from(scalar));
        payload.extend_from_slice(&result);
        payload
    }
}
