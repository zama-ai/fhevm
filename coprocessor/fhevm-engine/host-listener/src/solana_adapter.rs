use std::collections::HashSet;

use alloy_primitives::{Address, FixedBytes, Log};
use fhevm_engine_common::types::AllowEvents;
use sha2::{Digest, Sha256};
use sqlx::Error as SqlxError;

use crate::contracts::TfheContract;
use crate::contracts::TfheContract::TfheContractEvents;
use crate::database::dependence_chains::dependence_chains;
use crate::database::tfhe_event_propagate::{
    tfhe_result_handle, ClearConst, Database, Handle, LogTfhe, Transaction,
    TransactionHash,
};

const ANCHOR_EVENT_IX_TAG_LE: [u8; 8] = 0x1d9acb512ea545e4_u64.to_le_bytes();
const SOLANA_EVENT_VERSION: u8 = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolanaFheBinaryOp {
    Add,
    Sub,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaFheBinaryOpEvent {
    pub op: SolanaFheBinaryOp,
    pub lhs: Handle,
    pub rhs: Handle,
    pub scalar: bool,
    pub result: Handle,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaTrivialEncryptEvent {
    pub plaintext: [u8; 32],
    pub fhe_type: u8,
    pub result: Handle,
}

#[derive(Clone, Debug)]
pub struct SolanaAclAllowedEvent {
    pub handle: Handle,
    pub subject: String,
    pub event_type: AllowEvents,
}

#[derive(Clone, Debug)]
pub enum SolanaHostEvent {
    FheBinaryOp(SolanaFheBinaryOpEvent),
    TrivialEncrypt(SolanaTrivialEncryptEvent),
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

pub fn solana_transaction_id(signature_bytes: &[u8]) -> TransactionHash {
    let digest: [u8; 32] = Sha256::digest(signature_bytes).into();
    TransactionHash::from(digest)
}

pub fn decode_anchor_cpi_event(data: &[u8]) -> Option<SolanaHostEvent> {
    let data = data.strip_prefix(&ANCHOR_EVENT_IX_TAG_LE)?;
    if data.len() < 8 {
        return None;
    }
    let (discriminator, payload) = data.split_at(8);

    if discriminator == event_discriminator("FheBinaryOpEvent") {
        return decode_binary_op_event(payload)
            .map(SolanaHostEvent::FheBinaryOp);
    }
    if discriminator == event_discriminator("TrivialEncryptEvent") {
        return decode_trivial_encrypt_event(payload)
            .map(SolanaHostEvent::TrivialEncrypt);
    }
    if discriminator == event_discriminator("AclAllowedEvent") {
        return decode_acl_allowed_event(payload)
            .map(SolanaHostEvent::AclAllowed);
    }

    None
}

pub fn map_solana_event(event: SolanaHostEvent) -> SolanaMappedEvent {
    match event {
        SolanaHostEvent::FheBinaryOp(event) => {
            SolanaMappedEvent::Tfhe(to_tfhe_event(event))
        }
        SolanaHostEvent::TrivialEncrypt(event) => {
            SolanaMappedEvent::Tfhe(to_trivial_encrypt_event(event))
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

fn decode_binary_op_event(payload: &[u8]) -> Option<SolanaFheBinaryOpEvent> {
    let mut cursor = Cursor::new(payload);
    read_version(&mut cursor)?;
    let op = match cursor.read_u8()? {
        0 => SolanaFheBinaryOp::Add,
        1 => SolanaFheBinaryOp::Sub,
        _ => return None,
    };
    let _subject = cursor.read_bytes()?;
    Some(SolanaFheBinaryOpEvent {
        op,
        lhs: Handle::from(cursor.read_bytes()?),
        rhs: Handle::from(cursor.read_bytes()?),
        scalar: cursor.read_u8()? != 0,
        result: Handle::from(cursor.read_bytes()?),
    })
}

fn decode_trivial_encrypt_event(
    payload: &[u8],
) -> Option<SolanaTrivialEncryptEvent> {
    let mut cursor = Cursor::new(payload);
    read_version(&mut cursor)?;
    let _subject = cursor.read_bytes()?;
    Some(SolanaTrivialEncryptEvent {
        plaintext: cursor.read_bytes()?,
        fhe_type: cursor.read_u8()?,
        result: Handle::from(cursor.read_bytes()?),
    })
}

fn decode_acl_allowed_event(payload: &[u8]) -> Option<SolanaAclAllowedEvent> {
    let mut cursor = Cursor::new(payload);
    read_version(&mut cursor)?;
    let handle = Handle::from(cursor.read_bytes()?);
    let subject = format!("0x{}", encode_hex(&cursor.read_bytes()?));
    let event_type = match cursor.read_u8()? {
        0 => AllowEvents::AllowedAccount,
        1 | 2 => AllowEvents::AllowedForDecryption,
        _ => return None,
    };
    Some(SolanaAclAllowedEvent {
        handle,
        subject,
        event_type,
    })
}

fn read_version(cursor: &mut Cursor<'_>) -> Option<()> {
    (cursor.read_u8()? == SOLANA_EVENT_VERSION).then_some(())
}

fn event_discriminator(name: &str) -> [u8; 8] {
    let digest = Sha256::digest(format!("event:{name}"));
    digest[..8].try_into().expect("slice has 8 bytes")
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

struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn read_u8(&mut self) -> Option<u8> {
        let byte = *self.bytes.get(self.offset)?;
        self.offset += 1;
        Some(byte)
    }

    fn read_bytes(&mut self) -> Option<[u8; 32]> {
        let end = self.offset.checked_add(32)?;
        let bytes = self.bytes.get(self.offset..end)?;
        self.offset = end;
        bytes.try_into().ok()
    }
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
pub fn to_tfhe_event(event: SolanaFheBinaryOpEvent) -> Log<TfheContractEvents> {
    let caller = Address::ZERO;
    let scalar_byte = FixedBytes::<1>::from([u8::from(event.scalar)]);
    let data = match event.op {
        SolanaFheBinaryOp::Add => {
            TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller,
                lhs: event.lhs,
                rhs: event.rhs,
                scalarByte: scalar_byte,
                result: event.result,
            })
        }
        SolanaFheBinaryOp::Sub => {
            TfheContractEvents::FheSub(TfheContract::FheSub {
                caller,
                lhs: event.lhs,
                rhs: event.rhs,
                scalarByte: scalar_byte,
                result: event.result,
            })
        }
    };

    Log {
        address: caller,
        data,
    }
}

pub fn to_trivial_encrypt_event(
    event: SolanaTrivialEncryptEvent,
) -> Log<TfheContractEvents> {
    let caller = Address::ZERO;
    Log {
        address: caller,
        data: TfheContractEvents::TrivialEncrypt(
            TfheContract::TrivialEncrypt {
                caller,
                pt: ClearConst::from_be_slice(&event.plaintext),
                toType: event.fhe_type,
                result: event.result,
            },
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::{Date, Month, PrimitiveDateTime, Time};

    fn handle(byte: u8) -> Handle {
        Handle::from([byte; 32])
    }

    #[test]
    fn decodes_anchor_cpi_binary_event_to_existing_tfhe_event() {
        let encoded = anchor_cpi_event(
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
    fn decodes_anchor_cpi_trivial_encrypt_event() {
        let encoded = anchor_cpi_event("TrivialEncryptEvent", {
            let mut payload = vec![SOLANA_EVENT_VERSION];
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
    fn decodes_anchor_cpi_acl_allowed_event() {
        let encoded = anchor_cpi_event("AclAllowedEvent", {
            let mut payload = vec![SOLANA_EVENT_VERSION];
            payload.extend_from_slice(&[7; 32]);
            payload.extend_from_slice(&[8; 32]);
            payload.push(1);
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
            AllowEvents::AllowedForDecryption as i16
        );
    }

    #[test]
    fn maps_binary_add_to_existing_tfhe_event() {
        let mapped = to_tfhe_event(SolanaFheBinaryOpEvent {
            op: SolanaFheBinaryOp::Add,
            lhs: handle(1),
            rhs: handle(2),
            scalar: false,
            result: handle(3),
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
    fn maps_trivial_encrypt_to_existing_tfhe_event() {
        let mut plaintext = [0_u8; 32];
        plaintext[31] = 7;

        let mapped = to_trivial_encrypt_event(SolanaTrivialEncryptEvent {
            plaintext,
            fhe_type: 5,
            result: handle(8),
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
    fn keeps_acl_allowance_outside_evm_address_shape() {
        let event = SolanaAclAllowedEvent {
            handle: handle(9),
            subject: "6tc9KsnQ1nRGqGX97AQvCNnuhZ5SpQe68LiiFbG88kM5".to_owned(),
            event_type: AllowEvents::AllowedAccount,
        };

        let SolanaMappedEvent::AclAllowed(mapped) =
            map_solana_event(SolanaHostEvent::AclAllowed(event))
        else {
            panic!("expected ACL allowance event");
        };

        assert_eq!(mapped.handle, handle(9));
        assert_eq!(
            mapped.subject,
            "6tc9KsnQ1nRGqGX97AQvCNnuhZ5SpQe68LiiFbG88kM5"
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
            solana_transaction_id(&signature)
        );
        assert_ne!(solana_transaction_id(&signature), handle(7));
    }

    #[test]
    fn builds_existing_db_log_shape() {
        let tx_id = solana_transaction_id(&[1_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );
        let event = to_tfhe_event(SolanaFheBinaryOpEvent {
            op: SolanaFheBinaryOp::Sub,
            lhs: handle(1),
            rhs: handle(2),
            scalar: true,
            result: handle(3),
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
                SolanaHostEvent::FheBinaryOp(SolanaFheBinaryOpEvent {
                    op: SolanaFheBinaryOp::Add,
                    lhs: handle(1),
                    rhs: handle(2),
                    scalar: false,
                    result: handle(3),
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
                SolanaHostEvent::FheBinaryOp(SolanaFheBinaryOpEvent {
                    op: SolanaFheBinaryOp::Sub,
                    lhs: handle(1),
                    rhs: handle(2),
                    scalar: false,
                    result: handle(3),
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

    fn anchor_cpi_event(name: &str, payload: Vec<u8>) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&ANCHOR_EVENT_IX_TAG_LE);
        encoded.extend_from_slice(&event_discriminator(name));
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
        let mut payload = vec![SOLANA_EVENT_VERSION, op];
        payload.extend_from_slice(&subject);
        payload.extend_from_slice(&lhs);
        payload.extend_from_slice(&rhs);
        payload.push(u8::from(scalar));
        payload.extend_from_slice(&result);
        payload
    }
}
