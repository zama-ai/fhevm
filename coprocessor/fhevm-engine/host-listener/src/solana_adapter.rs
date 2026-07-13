//! Transport-agnostic core of the Solana ingestion path: Anchor event decoding
//! (`zama-host`) and mapping into the coprocessor
//! database, shared by every transport ([`crate::solana_listener`] RPC polling,
//! [`crate::solana_grpc_listener`] Yellowstone, LiteSVM in-process tests) and by
//! the emitless reconstruction path ([`crate::solana_reconstruct`]).
//!
//! Design rationale lives in `solana/docs/DESIGN_DECISIONS.md` (event transport:
//! DD-003; eager ciphertext-material preparation: DD-024).

use std::{collections::HashSet, fmt};

use alloy_primitives::{Address, FixedBytes, Log};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use sha2::{Digest, Sha256};
use sqlx::Error as SqlxError;

use crate::generated::{
    decode_anchor_cpi_event as decode_zama_host_anchor_cpi_event,
    decode_anchor_event as decode_zama_host_anchor_event, FheBinaryOpCode,
    FheBinaryOpEvent, FheIsInEvent, FheMulDivEvent, FheRandBoundedEvent,
    FheRandEvent, FheSumEvent, FheTernaryOpCode, FheTernaryOpEvent,
    FheUnaryOpCode, FheUnaryOpEvent, TrivialEncryptEvent, ZamaHostEvent,
    EVENT_VERSION,
};

use crate::cmd::block_history::BlockSummary;
use crate::contracts::TfheContract;
use crate::contracts::TfheContract::TfheContractEvents;
use crate::database::dependence_chains::dependence_chains;
use crate::database::tfhe_event_propagate::{
    ClearConst, Database, Handle, LogTfhe, Transaction, TransactionHash,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaMaterialRequest {
    pub handle: Handle,
}

#[derive(Clone, Debug)]
pub enum SolanaHostEvent {
    FheBinaryOp(FheBinaryOpEvent),
    FheTernaryOp(FheTernaryOpEvent),
    TrivialEncrypt(TrivialEncryptEvent),
    FheRand(FheRandEvent),
    FheRandBounded(FheRandBoundedEvent),
    FheUnaryOp(FheUnaryOpEvent),
    FheSum(FheSumEvent),
    FheIsIn(FheIsInEvent),
    FheMulDiv(FheMulDivEvent),
    MaterialRequest(SolanaMaterialRequest),
}

#[derive(Clone, Debug)]
pub enum SolanaMappedEvent {
    Tfhe(Log<TfheContractEvents>),
    MaterialRequest(SolanaMaterialRequest),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolanaBlockMeta {
    pub block_number: u64,
    pub block_timestamp: time::PrimitiveDateTime,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SolanaIngestStats {
    pub tfhe_events: usize,
    pub material_requests: usize,
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
///   events alongside log-only material requests): concatenate CPI events
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
            | SolanaHostEvent::FheRandBounded(_)
            | SolanaHostEvent::FheUnaryOp(_)
            | SolanaHostEvent::FheSum(_)
            | SolanaHostEvent::FheIsIn(_)
            | SolanaHostEvent::FheMulDiv(_)
    )
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
        ZamaHostEvent::FheRandBounded(event) => {
            vec![SolanaHostEvent::FheRandBounded(event)]
        }
        ZamaHostEvent::FheUnaryOp(event) => {
            vec![SolanaHostEvent::FheUnaryOp(event)]
        }
        ZamaHostEvent::FheSum(event) => {
            vec![SolanaHostEvent::FheSum(event)]
        }
        ZamaHostEvent::FheIsIn(event) => {
            vec![SolanaHostEvent::FheIsIn(event)]
        }
        ZamaHostEvent::FheMulDiv(event) => {
            vec![SolanaHostEvent::FheMulDiv(event)]
        }
        ZamaHostEvent::DenySubjectUpdated(_)
        | ZamaHostEvent::HcuAppTrustUpdated(_)
        | ZamaHostEvent::HostConfigInitialized(_)
        | ZamaHostEvent::HostConfigUpdated(_)
        | ZamaHostEvent::NewKmsContext(_)
        | ZamaHostEvent::KmsContextDestroyed(_)
        | ZamaHostEvent::UserDecryptionDelegationUpdated(_) => Vec::new(),
    }
}

// Only referenced by `solana_reconstruct` (feature-gated) outside of tests.
#[cfg_attr(not(feature = "solana-reconstruct"), allow(dead_code))]
pub(crate) fn material_request(handle: [u8; 32]) -> SolanaMaterialRequest {
    SolanaMaterialRequest {
        handle: Handle::from(handle),
    }
}

fn dedup_material_requests(requests: &mut Vec<SolanaMaterialRequest>) {
    let mut seen = HashSet::new();
    requests.retain(|request| seen.insert(request.handle));
}

fn zama_host_event_version(event: &ZamaHostEvent) -> u8 {
    match event {
        ZamaHostEvent::DenySubjectUpdated(event) => event.version,
        ZamaHostEvent::FheBinaryOp(event) => event.version,
        ZamaHostEvent::FheRand(event) => event.version,
        ZamaHostEvent::FheRandBounded(event) => event.version,
        ZamaHostEvent::FheTernaryOp(event) => event.version,
        ZamaHostEvent::FheUnaryOp(event) => event.version,
        ZamaHostEvent::FheSum(event) => event.version,
        ZamaHostEvent::FheIsIn(event) => event.version,
        ZamaHostEvent::FheMulDiv(event) => event.version,
        ZamaHostEvent::HcuAppTrustUpdated(event) => event.version,
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
        SolanaHostEvent::FheRandBounded(event) => {
            SolanaMappedEvent::Tfhe(to_fhe_rand_bounded_event(event))
        }
        SolanaHostEvent::FheUnaryOp(event) => {
            SolanaMappedEvent::Tfhe(to_fhe_unary_event(event))
        }
        SolanaHostEvent::FheSum(event) => {
            SolanaMappedEvent::Tfhe(to_fhe_sum_event(event))
        }
        SolanaHostEvent::FheIsIn(event) => {
            SolanaMappedEvent::Tfhe(to_fhe_is_in_event(event))
        }
        SolanaHostEvent::FheMulDiv(event) => {
            SolanaMappedEvent::Tfhe(to_fhe_mul_div_event(event))
        }
        SolanaHostEvent::MaterialRequest(request) => {
            SolanaMappedEvent::MaterialRequest(request)
        }
    }
}

// Solana computations and ciphertext-material preparation are scheduled as
// soon as their instruction confirms. The KMS independently validates the live
// EncryptedValue PDA and any MMR proof before releasing plaintext, so this eager
// work can waste cycles after a rare rollback but cannot authorize decryption.
pub fn normalize_solana_events_for_db(
    events: impl IntoIterator<Item = SolanaHostEvent>,
    transaction_id: TransactionHash,
    block: SolanaBlockMeta,
) -> (Vec<LogTfhe>, Vec<SolanaMaterialRequest>) {
    let events = events.into_iter().collect::<Vec<_>>();

    let mut tfhe_logs = Vec::new();
    let mut material_requests = Vec::new();

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
            SolanaMappedEvent::MaterialRequest(request) => {
                material_requests.push(request);
            }
        }
    }

    dedup_material_requests(&mut material_requests);
    (tfhe_logs, material_requests)
}

pub async fn insert_solana_events(
    db: &Database,
    tx: &mut Transaction<'_>,
    events: impl IntoIterator<Item = SolanaHostEvent>,
    transaction_id: TransactionHash,
    block: SolanaBlockMeta,
) -> Result<SolanaIngestStats, SqlxError> {
    let (mut tfhe_logs, material_requests) =
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
    for request in &material_requests {
        let handles = vec![request.handle.to_vec()];
        if db
            .insert_pbs_computations(
                tx,
                &handles,
                Some(transaction_id.to_vec()),
                block.block_number,
            )
            .await?
        {
            inserted_rows += 1;
        }
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
        material_requests: material_requests.len(),
        inserted_rows,
    })
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
        FheBinaryOpCode::Mul => {
            TfheContractEvents::FheMul(TfheContract::FheMul {
                caller,
                lhs: Handle::from(event.lhs),
                rhs: Handle::from(event.rhs),
                scalarByte: scalar_byte,
                result: Handle::from(event.result),
            })
        }
        FheBinaryOpCode::Div => {
            TfheContractEvents::FheDiv(TfheContract::FheDiv {
                caller,
                lhs: Handle::from(event.lhs),
                rhs: Handle::from(event.rhs),
                scalarByte: scalar_byte,
                result: Handle::from(event.result),
            })
        }
        FheBinaryOpCode::Rem => {
            TfheContractEvents::FheRem(TfheContract::FheRem {
                caller,
                lhs: Handle::from(event.lhs),
                rhs: Handle::from(event.rhs),
                scalarByte: scalar_byte,
                result: Handle::from(event.result),
            })
        }
        FheBinaryOpCode::And => {
            TfheContractEvents::FheBitAnd(TfheContract::FheBitAnd {
                caller,
                lhs: Handle::from(event.lhs),
                rhs: Handle::from(event.rhs),
                scalarByte: scalar_byte,
                result: Handle::from(event.result),
            })
        }
        FheBinaryOpCode::Or => {
            TfheContractEvents::FheBitOr(TfheContract::FheBitOr {
                caller,
                lhs: Handle::from(event.lhs),
                rhs: Handle::from(event.rhs),
                scalarByte: scalar_byte,
                result: Handle::from(event.result),
            })
        }
        FheBinaryOpCode::Xor => {
            TfheContractEvents::FheBitXor(TfheContract::FheBitXor {
                caller,
                lhs: Handle::from(event.lhs),
                rhs: Handle::from(event.rhs),
                scalarByte: scalar_byte,
                result: Handle::from(event.result),
            })
        }
        FheBinaryOpCode::Shl => {
            TfheContractEvents::FheShl(TfheContract::FheShl {
                caller,
                lhs: Handle::from(event.lhs),
                rhs: Handle::from(event.rhs),
                scalarByte: scalar_byte,
                result: Handle::from(event.result),
            })
        }
        FheBinaryOpCode::Shr => {
            TfheContractEvents::FheShr(TfheContract::FheShr {
                caller,
                lhs: Handle::from(event.lhs),
                rhs: Handle::from(event.rhs),
                scalarByte: scalar_byte,
                result: Handle::from(event.result),
            })
        }
        FheBinaryOpCode::Rotl => {
            TfheContractEvents::FheRotl(TfheContract::FheRotl {
                caller,
                lhs: Handle::from(event.lhs),
                rhs: Handle::from(event.rhs),
                scalarByte: scalar_byte,
                result: Handle::from(event.result),
            })
        }
        FheBinaryOpCode::Rotr => {
            TfheContractEvents::FheRotr(TfheContract::FheRotr {
                caller,
                lhs: Handle::from(event.lhs),
                rhs: Handle::from(event.rhs),
                scalarByte: scalar_byte,
                result: Handle::from(event.result),
            })
        }
        FheBinaryOpCode::Eq => TfheContractEvents::FheEq(TfheContract::FheEq {
            caller,
            lhs: Handle::from(event.lhs),
            rhs: Handle::from(event.rhs),
            scalarByte: scalar_byte,
            result: Handle::from(event.result),
        }),
        FheBinaryOpCode::Ne => TfheContractEvents::FheNe(TfheContract::FheNe {
            caller,
            lhs: Handle::from(event.lhs),
            rhs: Handle::from(event.rhs),
            scalarByte: scalar_byte,
            result: Handle::from(event.result),
        }),
        FheBinaryOpCode::Ge => TfheContractEvents::FheGe(TfheContract::FheGe {
            caller,
            lhs: Handle::from(event.lhs),
            rhs: Handle::from(event.rhs),
            scalarByte: scalar_byte,
            result: Handle::from(event.result),
        }),
        FheBinaryOpCode::Gt => TfheContractEvents::FheGt(TfheContract::FheGt {
            caller,
            lhs: Handle::from(event.lhs),
            rhs: Handle::from(event.rhs),
            scalarByte: scalar_byte,
            result: Handle::from(event.result),
        }),
        FheBinaryOpCode::Le => TfheContractEvents::FheLe(TfheContract::FheLe {
            caller,
            lhs: Handle::from(event.lhs),
            rhs: Handle::from(event.rhs),
            scalarByte: scalar_byte,
            result: Handle::from(event.result),
        }),
        FheBinaryOpCode::Lt => TfheContractEvents::FheLt(TfheContract::FheLt {
            caller,
            lhs: Handle::from(event.lhs),
            rhs: Handle::from(event.rhs),
            scalarByte: scalar_byte,
            result: Handle::from(event.result),
        }),
        FheBinaryOpCode::Min => {
            TfheContractEvents::FheMin(TfheContract::FheMin {
                caller,
                lhs: Handle::from(event.lhs),
                rhs: Handle::from(event.rhs),
                scalarByte: scalar_byte,
                result: Handle::from(event.result),
            })
        }
        FheBinaryOpCode::Max => {
            TfheContractEvents::FheMax(TfheContract::FheMax {
                caller,
                lhs: Handle::from(event.lhs),
                rhs: Handle::from(event.rhs),
                scalarByte: scalar_byte,
                result: Handle::from(event.result),
            })
        }
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

pub fn to_fhe_rand_bounded_event(
    event: FheRandBoundedEvent,
) -> Log<TfheContractEvents> {
    let caller = Address::ZERO;
    Log {
        address: caller,
        data: TfheContractEvents::FheRandBounded(
            TfheContract::FheRandBounded {
                caller,
                upperBound: ClearConst::from_be_slice(&event.upper_bound),
                randType: event.fhe_type,
                seed: FixedBytes::<16>::from(event.seed),
                result: Handle::from(event.result),
            },
        ),
    }
}

pub fn to_fhe_unary_event(event: FheUnaryOpEvent) -> Log<TfheContractEvents> {
    let caller = Address::ZERO;
    let ct = Handle::from(event.operand);
    let result = Handle::from(event.result);
    let data = match event.op {
        FheUnaryOpCode::Neg => {
            TfheContractEvents::FheNeg(TfheContract::FheNeg {
                caller,
                ct,
                result,
            })
        }
        FheUnaryOpCode::Not => {
            TfheContractEvents::FheNot(TfheContract::FheNot {
                caller,
                ct,
                result,
            })
        }
        FheUnaryOpCode::Cast => TfheContractEvents::Cast(TfheContract::Cast {
            caller,
            ct,
            toType: event.result[30],
            result,
        }),
    };
    Log {
        address: caller,
        data,
    }
}

pub fn to_fhe_sum_event(event: FheSumEvent) -> Log<TfheContractEvents> {
    let caller = Address::ZERO;
    Log {
        address: caller,
        data: TfheContractEvents::FheSum(TfheContract::FheSum {
            caller,
            values: event.operands.into_iter().map(Handle::from).collect(),
            result: Handle::from(event.result),
        }),
    }
}

pub fn to_fhe_is_in_event(event: FheIsInEvent) -> Log<TfheContractEvents> {
    let caller = Address::ZERO;
    Log {
        address: caller,
        data: TfheContractEvents::FheIsIn(TfheContract::FheIsIn {
            caller,
            value: Handle::from(event.value),
            values: event.set.into_iter().map(Handle::from).collect(),
            result: Handle::from(event.result),
        }),
    }
}

pub fn to_fhe_mul_div_event(event: FheMulDivEvent) -> Log<TfheContractEvents> {
    let caller = Address::ZERO;
    Log {
        address: caller,
        data: TfheContractEvents::FheMulDiv(TfheContract::FheMulDiv {
            caller,
            factor1: Handle::from(event.factor1),
            factor2: Handle::from(event.factor2),
            divisor: FixedBytes::<32>::from(event.divisor),
            // fheMulDiv scalarByte bitmask (EVM parity): bit0 divisor (always) | bit1 factor2-scalar → 0x01 enc, 0x03 scalar.
            scalarByte: FixedBytes::<1>::from([
                0x01 | (u8::from(event.scalar) << 1)
            ]),
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
        SolanaHostEvent::MaterialRequest(material_request([account; 32]))
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
        // CPI compute event alongside log-only material requests: neither
        // transport is preferred, but CPI events are emitted first so the single
        // compute side keeps a deterministic DB log index.
        let merged = merge_solana_transport_events(
            vec![compute_event(1)],
            vec![fetch_event(7), fetch_event(8)],
        )
        .expect("only the cpi side carries compute events");

        assert!(matches!(merged[0], SolanaHostEvent::FheBinaryOp(_)));
        assert!(matches!(merged[1], SolanaHostEvent::MaterialRequest(_)));
        assert!(matches!(merged[2], SolanaHostEvent::MaterialRequest(_)));
    }

    #[test]
    fn merge_allows_non_compute_events_on_both_transports() {
        // Material requests are not order-sensitive, so both sides carrying
        // only requests is not a mixed-transport conflict.
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
            anchor_event("FheUnaryOpEvent", {
                let mut payload = vec![EVENT_VERSION, 0];
                payload.extend_from_slice(&[9; 32]);
                payload.extend_from_slice(&[1; 32]);
                payload.extend_from_slice(&[2; 32]);
                payload
            }),
            anchor_event("FheSumEvent", {
                let mut payload = vec![EVENT_VERSION];
                payload.extend_from_slice(&[9; 32]);
                payload.extend_from_slice(&3_u32.to_le_bytes());
                payload.extend_from_slice(&[1; 32]);
                payload.extend_from_slice(&[2; 32]);
                payload.extend_from_slice(&[3; 32]);
                payload.push(5);
                payload.extend_from_slice(&[4; 32]);
                payload
            }),
            anchor_event("FheIsInEvent", {
                let mut payload = vec![EVENT_VERSION];
                payload.extend_from_slice(&[9; 32]);
                payload.extend_from_slice(&[1; 32]);
                payload.extend_from_slice(&2_u32.to_le_bytes());
                payload.extend_from_slice(&[2; 32]);
                payload.extend_from_slice(&[3; 32]);
                payload.push(5);
                payload.extend_from_slice(&[4; 32]);
                payload
            }),
            anchor_event("FheMulDivEvent", {
                let mut payload = vec![EVENT_VERSION];
                payload.extend_from_slice(&[9; 32]);
                payload.extend_from_slice(&[1; 32]);
                payload.extend_from_slice(&[2; 32]);
                payload.extend_from_slice(&[3; 32]);
                payload.push(false.into());
                payload.extend_from_slice(&[4; 32]);
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
        // Under eager compute (RFC-024 Q11), it is unconditionally scheduled;
        // KMS independently gates plaintext release against Solana ACL state.
        let tx_id = solana_transaction_id(&[7_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );

        let (tfhe_logs, material_requests) = normalize_solana_events_for_db(
            [
                SolanaHostEvent::TrivialEncrypt(TrivialEncryptEvent {
                    version: EVENT_VERSION,
                    subject: [0; 32],
                    plaintext: [55; 32],
                    fhe_type: 5,
                    result: [3; 32],
                }),
                SolanaHostEvent::MaterialRequest(material_request([3; 32])),
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
        // The durable handle is queued directly for material preparation.
        assert_eq!(material_requests.len(), 1);
    }

    #[test]
    fn material_requests_keep_distinct_handles_in_one_batch() {
        let tx_id = solana_transaction_id(&[9_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );

        let (_, material_requests) = normalize_solana_events_for_db(
            [
                SolanaHostEvent::MaterialRequest(material_request([1; 32])),
                SolanaHostEvent::MaterialRequest(material_request([2; 32])),
            ],
            tx_id,
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
            },
        );

        assert_eq!(material_requests.len(), 2);
        assert!(material_requests
            .iter()
            .any(|request| request.handle == handle(1)));
        assert!(material_requests
            .iter()
            .any(|request| request.handle == handle(2)));
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
                SolanaHostEvent::MaterialRequest(material_request([4; 32])),
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

        let (tfhe_logs, material_requests) = normalize_solana_events_for_db(
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

        assert!(material_requests.is_empty());
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

    fn anchor_event(name: &str, payload: Vec<u8>) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&anchor_event_discriminator(name));
        encoded.extend_from_slice(&payload);
        encoded
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
