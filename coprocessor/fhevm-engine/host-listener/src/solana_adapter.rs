use std::{collections::HashSet, fmt};

use alloy_primitives::{Address, FixedBytes, Log};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use fhevm_engine_common::types::AllowEvents;
use sha2::{Digest, Sha256};
use sqlx::Error as SqlxError;

use crate::generated::{
    decode_anchor_cpi_event as decode_zama_host_anchor_cpi_event,
    decode_anchor_event as decode_zama_host_anchor_event, FheBinaryOpCode,
    FheBinaryOpEvent, FheRandBoundedEvent, FheRandEvent, FheTernaryOpCode,
    FheTernaryOpEvent, TrivialEncryptEvent, ZamaHostEvent, EVENT_VERSION,
};

use crate::contracts::TfheContract;
use crate::contracts::TfheContract::TfheContractEvents;
use crate::database::dependence_chains::dependence_chains;
use crate::database::tfhe_event_propagate::{
    tfhe_result_handle, ClearConst, Database, Handle, LogTfhe, Transaction,
    TransactionHash,
};

#[derive(Clone, Debug)]
pub struct SolanaAclAllowedEvent {
    pub handle: Handle,
    pub subject: String,
    pub event_type: AllowEvents,
}

#[derive(Clone, Debug)]
pub enum SolanaHostEvent {
    FheBinaryOp(FheBinaryOpEvent),
    FheTernaryOp(FheTernaryOpEvent),
    TrivialEncrypt(TrivialEncryptEvent),
    FheRand(FheRandEvent),
    FheRandBounded(FheRandBoundedEvent),
    AclAllowed(SolanaAclAllowedEvent),
}

#[derive(Clone, Debug)]
pub enum SolanaMappedEvent {
    Tfhe(Log<TfheContractEvents>),
    AclAllowed(SolanaAclAllowedEvent),
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
    decode_solana_host_event(decode_zama_host_anchor_cpi_event(data)?)
}

pub fn decode_anchor_event(data: &[u8]) -> Option<SolanaHostEvent> {
    decode_solana_host_event(decode_zama_host_anchor_event(data)?)
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
        .filter_map(|(_, data)| decode_anchor_cpi_event(data))
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
        if let Some(event) = decode_anchor_event(&data) {
            events.push(event);
        }
    }

    events
}

pub fn merge_solana_transport_events(
    cpi_events: Vec<SolanaHostEvent>,
    log_events: Vec<SolanaHostEvent>,
) -> Result<Vec<SolanaHostEvent>, SolanaEventDecodeError> {
    match (cpi_events.is_empty(), log_events.is_empty()) {
        (true, _) => Ok(log_events),
        (_, true) => Ok(cpi_events),
        (false, false) => {
            Err(SolanaEventDecodeError::MixedHostEventTransport {
                cpi_events: cpi_events.len(),
                log_events: log_events.len(),
            })
        }
    }
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

fn pop_program_stack<'a>(stack: &mut Vec<&'a str>, program_id: &str) {
    if stack.last().copied() == Some(program_id) {
        stack.pop();
        return;
    }

    if let Some(index) = stack.iter().rposition(|entry| *entry == program_id) {
        stack.truncate(index);
    }
}

fn decode_solana_host_event(event: ZamaHostEvent) -> Option<SolanaHostEvent> {
    if zama_host_event_version(&event) != EVENT_VERSION {
        return None;
    }
    match event {
        ZamaHostEvent::FheBinaryOp(event) => {
            Some(SolanaHostEvent::FheBinaryOp(event))
        }
        ZamaHostEvent::FheTernaryOp(event) => {
            Some(SolanaHostEvent::FheTernaryOp(event))
        }
        ZamaHostEvent::TrivialEncrypt(event) => {
            Some(SolanaHostEvent::TrivialEncrypt(event))
        }
        ZamaHostEvent::FheRand(event) => Some(SolanaHostEvent::FheRand(event)),
        ZamaHostEvent::FheRandBounded(event) => {
            Some(SolanaHostEvent::FheRandBounded(event))
        }
        ZamaHostEvent::AclAllowed(event) => {
            Some(SolanaHostEvent::AclAllowed(SolanaAclAllowedEvent {
                handle: Handle::from(event.handle),
                subject: format!("0x{}", encode_hex(&event.subject)),
                event_type: AllowEvents::AllowedAccount,
            }))
        }
        ZamaHostEvent::InputVerified(_)
        | ZamaHostEvent::AclRecordBound(_)
        | ZamaHostEvent::AclSubjectAllowed(_)
        | ZamaHostEvent::DenySubjectUpdated(_)
        | ZamaHostEvent::HandleMaterialCommitted(_)
        | ZamaHostEvent::HandleMaterialSealed(_)
        | ZamaHostEvent::HostConfigInitialized(_)
        | ZamaHostEvent::HostConfigUpdated(_)
        | ZamaHostEvent::PublicDecryptAllowed(_)
        | ZamaHostEvent::UserDecryptionDelegationUpdated(_) => None,
    }
}

fn zama_host_event_version(event: &ZamaHostEvent) -> u8 {
    match event {
        ZamaHostEvent::AclAllowed(event) => event.version,
        ZamaHostEvent::AclRecordBound(event) => event.version,
        ZamaHostEvent::AclSubjectAllowed(event) => event.version,
        ZamaHostEvent::DenySubjectUpdated(event) => event.version,
        ZamaHostEvent::FheBinaryOp(event) => event.version,
        ZamaHostEvent::FheRand(event) => event.version,
        ZamaHostEvent::FheRandBounded(event) => event.version,
        ZamaHostEvent::FheTernaryOp(event) => event.version,
        ZamaHostEvent::HandleMaterialCommitted(event) => event.version,
        ZamaHostEvent::HandleMaterialSealed(event) => event.version,
        ZamaHostEvent::HostConfigInitialized(event) => event.version,
        ZamaHostEvent::HostConfigUpdated(event) => event.version,
        ZamaHostEvent::InputVerified(event) => event.version,
        ZamaHostEvent::PublicDecryptAllowed(event) => event.version,
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
        SolanaHostEvent::AclAllowed(event) => {
            SolanaMappedEvent::AclAllowed(event)
        }
    }
}

pub fn normalize_solana_events_for_db(
    events: impl IntoIterator<Item = SolanaHostEvent>,
    transaction_id: TransactionHash,
    block: SolanaBlockMeta,
) -> (Vec<LogTfhe>, Vec<SolanaAclAllowedEvent>) {
    let mut allowed_handles = HashSet::<Handle>::new();
    let mut tfhe_logs = Vec::new();
    let mut acl_events = Vec::new();

    for (index, event) in events.into_iter().enumerate() {
        match map_solana_event(event) {
            SolanaMappedEvent::Tfhe(event) => tfhe_logs.push(to_log_tfhe(
                event,
                transaction_id,
                block,
                false,
                index as u64,
            )),
            SolanaMappedEvent::AclAllowed(event) => {
                allowed_handles.insert(event.handle);
                acl_events.push(event);
            }
        }
    }

    for log in &mut tfhe_logs {
        log.is_allowed = tfhe_result_handle(&log.event.data)
            .map(|handle| allowed_handles.contains(&handle))
            .unwrap_or(false);
    }

    (tfhe_logs, acl_events)
}

pub async fn insert_solana_events(
    db: &Database,
    tx: &mut Transaction<'_>,
    events: impl IntoIterator<Item = SolanaHostEvent>,
    transaction_id: TransactionHash,
    block: SolanaBlockMeta,
) -> Result<SolanaIngestStats, SqlxError> {
    let (mut tfhe_logs, acl_events) =
        normalize_solana_events_for_db(events, transaction_id, block);
    let mut inserted_rows = 0;

    for event in &acl_events {
        if db
            .insert_allowed_handle(
                tx,
                event.handle.to_vec(),
                event.subject.clone(),
                event.event_type,
                Some(transaction_id.to_vec()),
                block.block_number,
            )
            .await?
        {
            inserted_rows += 1;
        }

        if db
            .insert_pbs_computations(
                tx,
                &vec![event.handle.to_vec()],
                Some(transaction_id.to_vec()),
                block.block_number,
            )
            .await?
        {
            inserted_rows += 1;
        }
    }

    dependence_chains(&mut tfhe_logs, &db.dependence_chain, false, true).await;

    for log in &tfhe_logs {
        if db.insert_tfhe_event(tx, log).await? {
            inserted_rows += 1;
        }
    }

    Ok(SolanaIngestStats {
        tfhe_events: tfhe_logs.len(),
        acl_events: acl_events.len(),
        inserted_rows,
    })
}

fn encode_hex(bytes: &[u8; 32]) -> String {
    const ALPHABET: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(64);
    for byte in bytes {
        out.push(ALPHABET[(byte >> 4) as usize] as char);
        out.push(ALPHABET[(byte & 0x0f) as usize] as char);
    }
    out
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

        for value in 0_u8..5 {
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
        for value in 10_u8..17 {
            logs.push(format!(
                "Program data: {}",
                BASE64_STANDARD.encode(anchor_event("AclAllowedEvent", {
                    let mut payload = vec![EVENT_VERSION];
                    payload.extend_from_slice(&[value; 32]);
                    payload.extend_from_slice(&[8; 32]);
                    payload
                }))
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
            5
        );
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(event, SolanaHostEvent::AclAllowed(_)))
                .count(),
            7
        );
        assert!(matches!(
            &events[0],
            SolanaHostEvent::FheBinaryOp(event) if event.result == [0; 32]
        ));
        assert!(matches!(
            &events[5],
            SolanaHostEvent::AclAllowed(event) if event.handle == handle(10)
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
                    "AclAllowedEvent",
                    acl_allowed_payload([3; 32], [4; 32]),
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

    #[test]
    fn ignored_metadata_logs_do_not_make_cpi_transport_mixed() {
        let host_program = "ZamaHost111111111111111111111111111111111";
        let cpi_event = anchor_event_cpi(
            "FheBinaryOpEvent",
            binary_op_payload(0, [9; 32], [1; 32], [2; 32], false, [3; 32]),
        );
        let logs = vec![
            format!("Program {host_program} invoke [1]"),
            program_data_log(
                "AclRecordBoundEvent",
                acl_record_bound_payload([3; 32]),
            ),
            program_data_log(
                "AclSubjectAllowedEvent",
                acl_subject_allowed_payload([3; 32], [4; 32]),
            ),
            format!("Program {host_program} success"),
        ];

        let events = decode_solana_transaction_events(
            &logs,
            [(host_program, cpi_event.as_slice())],
            host_program,
        )
        .expect("ignored metadata logs must not conflict with CPI transport");

        assert!(matches!(
            events.as_slice(),
            [SolanaHostEvent::FheBinaryOp(event)] if event.result == [3; 32]
        ));
    }

    #[test]
    fn input_verified_events_are_not_worker_compute_events() {
        let cpi_encoded = anchor_event_cpi(
            "InputVerifiedEvent",
            input_verified_payload([7; 32]),
        );
        let log_encoded =
            anchor_event("InputVerifiedEvent", input_verified_payload([8; 32]));

        assert!(decode_anchor_cpi_event(&cpi_encoded).is_none());
        assert!(decode_anchor_event(&log_encoded).is_none());
    }

    #[test]
    fn input_verified_logs_do_not_hide_acl_allowances() {
        let host_program = "ZamaHost111111111111111111111111111111111";
        let logs = vec![
            format!("Program {host_program} invoke [1]"),
            program_data_log(
                "InputVerifiedEvent",
                input_verified_payload([7; 32]),
            ),
            program_data_log(
                "AclAllowedEvent",
                acl_allowed_payload([7; 32], [8; 32]),
            ),
            format!("Program {host_program} success"),
        ];

        let events = decode_solana_transaction_events(&logs, [], host_program)
            .expect("input frame log transport should decode");

        assert!(matches!(
            events.as_slice(),
            [SolanaHostEvent::AclAllowed(event)]
                if event.handle == handle(7)
                    && event.subject == "0x0808080808080808080808080808080808080808080808080808080808080808"
        ));

        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );
        let (tfhe_logs, acl_events) = normalize_solana_events_for_db(
            events,
            solana_transaction_id(&[4_u8; 64]),
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
            },
        );

        assert!(tfhe_logs.is_empty());
        assert_eq!(acl_events.len(), 1);
        assert_eq!(acl_events[0].handle, handle(7));
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
            anchor_event("AclAllowedEvent", {
                let payload = acl_allowed_payload([7; 32], [8; 32]);
                payload
            }),
            anchor_event("AclRecordBoundEvent", {
                let mut payload = vec![EVENT_VERSION];
                payload.extend_from_slice(&[1; 32]);
                payload.extend_from_slice(&[2; 32]);
                payload.extend_from_slice(&[3; 32]);
                payload.extend_from_slice(&4_u64.to_le_bytes());
                payload.extend_from_slice(&[5; 32]);
                payload.extend_from_slice(&[6; 32]);
                payload.extend_from_slice(&[7; 32]);
                payload.push(8);
                payload.push(0);
                payload.extend_from_slice(&9_u64.to_le_bytes());
                payload
            }),
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
    fn decodes_anchor_event_cpi_acl_allowed_event() {
        let encoded = anchor_event_cpi("AclAllowedEvent", {
            let mut payload = vec![EVENT_VERSION];
            payload.extend_from_slice(&[7; 32]);
            payload.extend_from_slice(&[8; 32]);
            payload
        });

        let SolanaHostEvent::AclAllowed(decoded) =
            decode_anchor_cpi_event(&encoded).expect("expected ACL event")
        else {
            panic!("expected ACL event");
        };

        assert_eq!(decoded.handle, handle(7));
        assert_eq!(
            decoded.subject,
            "0x0808080808080808080808080808080808080808080808080808080808080808"
        );
        assert_eq!(
            decoded.event_type as i16,
            AllowEvents::AllowedAccount as i16
        );
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
    fn ignores_public_decrypt_allowed_event_for_coprocessor_ingest() {
        let encoded = anchor_event_cpi("PublicDecryptAllowedEvent", {
            let mut payload = vec![EVENT_VERSION];
            payload.extend_from_slice(&[1; 32]);
            payload.extend_from_slice(&[2; 32]);
            payload.extend_from_slice(&[3; 32]);
            payload.extend_from_slice(&42_u64.to_le_bytes());
            payload
        });

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
    fn maps_bounded_random_to_existing_tfhe_event() {
        let mut upper_bound = [0_u8; 32];
        upper_bound[30] = 1;

        let mapped = to_fhe_rand_bounded_event(FheRandBoundedEvent {
            version: EVENT_VERSION,
            subject: [0; 32],
            upper_bound,
            seed: [7; 16],
            fhe_type: 3,
            result: [8; 32],
        });

        assert!(matches!(
            mapped.data,
            TfheContractEvents::FheRandBounded(TfheContract::FheRandBounded {
                upperBound,
                randType,
                seed,
                result,
                ..
            }) if upperBound == ClearConst::from(256_u64)
                && randType == 3
                && seed == FixedBytes::<16>::from([7; 16])
                && result == handle(8)
        ));
    }

    #[test]
    fn formats_acl_allowed_subject_as_full_solana_pubkey_hex() {
        let decoded =
            decode_anchor_cpi_event(&anchor_event_cpi("AclAllowedEvent", {
                let mut payload = vec![EVENT_VERSION];
                payload.extend_from_slice(&[9; 32]);
                payload.extend_from_slice(&[0xab; 32]);
                payload
            }))
            .expect("expected ACL event");

        let SolanaMappedEvent::AclAllowed(mapped) = map_solana_event(decoded)
        else {
            panic!("expected ACL allowance event");
        };

        assert_eq!(mapped.handle, handle(9));
        assert_eq!(
            mapped.subject,
            "0xabababababababababababababababababababababababababababababababab"
        );
        assert_eq!(
            mapped.event_type as i16,
            AllowEvents::AllowedAccount as i16
        );
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
    fn normalizes_same_transaction_acl_into_allowed_tfhe_log() {
        let tx_id = solana_transaction_id(&[2_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );

        let (tfhe_logs, acl_events) = normalize_solana_events_for_db(
            [
                SolanaHostEvent::FheBinaryOp(FheBinaryOpEvent {
                    version: EVENT_VERSION,
                    op: FheBinaryOpCode::Add,
                    subject: [0; 32],
                    lhs: [1; 32],
                    rhs: [2; 32],
                    scalar: false,
                    result: [3; 32],
                }),
                SolanaHostEvent::AclAllowed(SolanaAclAllowedEvent {
                    handle: handle(3),
                    subject:
                        "0x0404040404040404040404040404040404040404040404040404040404040404"
                            .to_owned(),
                    event_type: AllowEvents::AllowedAccount,
                }),
            ],
            tx_id,
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
            },
        );

        assert_eq!(acl_events.len(), 1);
        assert_eq!(tfhe_logs.len(), 1);
        assert!(tfhe_logs[0].is_allowed);
        assert_eq!(tfhe_logs[0].transaction_hash, Some(tx_id));
        assert_eq!(tfhe_logs[0].dependence_chain, tx_id);
        assert_eq!(tfhe_logs[0].log_index, Some(0));
    }

    #[test]
    fn leaves_unallowed_tfhe_result_pending() {
        let tx_id = solana_transaction_id(&[3_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );

        let (tfhe_logs, acl_events) = normalize_solana_events_for_db(
            [
                SolanaHostEvent::FheBinaryOp(FheBinaryOpEvent {
                    version: EVENT_VERSION,
                    op: FheBinaryOpCode::Sub,
                    subject: [0; 32],
                    lhs: [1; 32],
                    rhs: [2; 32],
                    scalar: false,
                    result: [3; 32],
                }),
                SolanaHostEvent::AclAllowed(SolanaAclAllowedEvent {
                    handle: handle(9),
                    subject:
                        "0x0404040404040404040404040404040404040404040404040404040404040404"
                            .to_owned(),
                    event_type: AllowEvents::AllowedAccount,
                }),
            ],
            tx_id,
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
            },
        );

        assert_eq!(acl_events.len(), 1);
        assert_eq!(tfhe_logs.len(), 1);
        assert!(!tfhe_logs[0].is_allowed);
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
                SolanaHostEvent::AclAllowed(SolanaAclAllowedEvent {
                    handle: handle(1),
                    subject:
                        "0x0404040404040404040404040404040404040404040404040404040404040404"
                            .to_owned(),
                    event_type: AllowEvents::AllowedAccount,
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
                SolanaHostEvent::AclAllowed(SolanaAclAllowedEvent {
                    handle: handle(3),
                    subject:
                        "0x0505050505050505050505050505050505050505050505050505050505050505"
                            .to_owned(),
                    event_type: AllowEvents::AllowedAccount,
                }),
                SolanaHostEvent::FheRandBounded(FheRandBoundedEvent {
                    version: EVENT_VERSION,
                    subject: [0; 32],
                    upper_bound: {
                        let mut upper_bound = [0_u8; 32];
                        upper_bound[31] = 8;
                        upper_bound
                    },
                    seed: [4; 16],
                    fhe_type: 5,
                    result: [4; 32],
                }),
            ],
            tx_id,
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
            },
        );

        assert_eq!(acl_events.len(), 2);
        assert_eq!(tfhe_logs.len(), 4);
        assert_eq!(
            tfhe_logs
                .iter()
                .map(|log| log.log_index)
                .collect::<Vec<_>>(),
            vec![Some(0), Some(2), Some(3), Some(5)]
        );
        assert!(tfhe_logs[0].is_allowed);
        assert!(!tfhe_logs[1].is_allowed);
        assert!(tfhe_logs[2].is_allowed);
        assert!(!tfhe_logs[3].is_allowed);
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
        assert!(matches!(
            tfhe_logs[3].event.data,
            TfheContractEvents::FheRandBounded(_)
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

    fn acl_allowed_payload(handle: [u8; 32], subject: [u8; 32]) -> Vec<u8> {
        let mut payload = vec![EVENT_VERSION];
        payload.extend_from_slice(&handle);
        payload.extend_from_slice(&subject);
        payload
    }

    fn input_verified_payload(handle: [u8; 32]) -> Vec<u8> {
        let mut payload = vec![EVENT_VERSION];
        payload.extend_from_slice(&handle);
        payload.extend_from_slice(&handle);
        payload.extend_from_slice(&[8; 32]);
        payload.extend_from_slice(&[9; 32]);
        payload
    }

    fn acl_record_bound_payload(handle: [u8; 32]) -> Vec<u8> {
        let mut payload = vec![EVENT_VERSION];
        payload.extend_from_slice(&[1; 32]);
        payload.extend_from_slice(&handle);
        payload.extend_from_slice(&[2; 32]);
        payload.extend_from_slice(&3_u64.to_le_bytes());
        payload.extend_from_slice(&[4; 32]);
        payload.extend_from_slice(&[5; 32]);
        payload.extend_from_slice(&[6; 32]);
        payload.push(1);
        payload.push(0);
        payload.extend_from_slice(&7_u64.to_le_bytes());
        payload
    }

    fn acl_subject_allowed_payload(
        handle: [u8; 32],
        subject: [u8; 32],
    ) -> Vec<u8> {
        let mut payload = vec![EVENT_VERSION];
        payload.extend_from_slice(&[1; 32]);
        payload.extend_from_slice(&handle);
        payload.extend_from_slice(&[2; 32]);
        payload.extend_from_slice(&subject);
        payload.push(1);
        payload.extend_from_slice(&[3; 32]);
        payload.push(0);
        payload.extend_from_slice(&4_u64.to_le_bytes());
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
