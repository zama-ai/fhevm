use std::collections::HashSet;

use alloy_primitives::{hex, Address, FixedBytes, Log};
use fhevm_engine_common::types::AllowEvents;
use sha2::{Digest, Sha256};
use sqlx::Error as SqlxError;

use crate::generated::{
    FheBinaryOpCode, FheBinaryOpEvent, FheRandEvent, TrivialEncryptEvent,
    ZamaHostEvent,
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

fn solana_acl_event(
    handle: [u8; 32],
    subject: [u8; 32],
    event_type: AllowEvents,
) -> SolanaAclAllowedEvent {
    SolanaAclAllowedEvent {
        handle: Handle::from(handle),
        subject: format!("0x{}", hex::encode(subject)),
        event_type,
    }
}

/// Maps decoded ZamaHost CPI events into the existing coprocessor DB model.
///
/// TFHE events become `LogTfhe` rows; ACL allow events become allowance rows and
/// mark any matching TFHE result handle as allowed in the same transaction.
/// `InputVerified` carries no DB work (input ciphertext material is registered
/// out of band) and is dropped before the remaining events are indexed.
pub fn normalize_solana_events_for_db(
    events: impl IntoIterator<Item = ZamaHostEvent>,
    transaction_id: TransactionHash,
    block: SolanaBlockMeta,
) -> (Vec<LogTfhe>, Vec<SolanaAclAllowedEvent>) {
    let mut allowed_handles = HashSet::<Handle>::new();
    let mut tfhe_logs = Vec::new();
    let mut acl_events = Vec::new();

    let host_events = events
        .into_iter()
        .filter(|event| !matches!(event, ZamaHostEvent::InputVerified(_)));

    for (index, event) in host_events.enumerate() {
        let tfhe_event = match event {
            ZamaHostEvent::FheBinaryOp(event) => to_tfhe_event(event),
            ZamaHostEvent::TrivialEncrypt(event) => {
                to_trivial_encrypt_event(event)
            }
            ZamaHostEvent::FheRand(event) => to_fhe_rand_event(event),
            ZamaHostEvent::AclAllowed(event) => {
                let acl = solana_acl_event(
                    event.handle,
                    event.subject,
                    AllowEvents::AllowedAccount,
                );
                allowed_handles.insert(acl.handle);
                acl_events.push(acl);
                continue;
            }
            ZamaHostEvent::AclPublicDecryptAllowed(event) => {
                let acl = solana_acl_event(
                    event.handle,
                    event.subject,
                    AllowEvents::AllowedForDecryption,
                );
                allowed_handles.insert(acl.handle);
                acl_events.push(acl);
                continue;
            }
            ZamaHostEvent::InputVerified(_) => continue,
        };
        tfhe_logs.push(to_log_tfhe(
            tfhe_event,
            transaction_id,
            block,
            false,
            index as u64,
        ));
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
    events: impl IntoIterator<Item = ZamaHostEvent>,
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
    let lhs = Handle::from(event.lhs);
    let rhs = Handle::from(event.rhs);
    let result = Handle::from(event.result);
    let scalar_byte = FixedBytes::<1>::from([u8::from(event.scalar)]);
    let data = match event.op {
        FheBinaryOpCode::Add => {
            TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller,
                lhs,
                rhs,
                scalarByte: scalar_byte,
                result,
            })
        }
        FheBinaryOpCode::Sub => {
            TfheContractEvents::FheSub(TfheContract::FheSub {
                caller,
                lhs,
                rhs,
                scalarByte: scalar_byte,
                result,
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
        anchor_event_discriminator, decode_anchor_cpi_event, AclAllowedEvent,
        ANCHOR_EVENT_IX_TAG_LE, EVENT_VERSION,
    };
    use time::{Date, Month, PrimitiveDateTime, Time};

    fn handle(byte: u8) -> Handle {
        Handle::from([byte; 32])
    }

    fn test_block() -> SolanaBlockMeta {
        SolanaBlockMeta {
            block_number: 42,
            block_timestamp: PrimitiveDateTime::new(
                Date::from_calendar_date(2026, Month::May, 9).unwrap(),
                Time::MIDNIGHT,
            ),
        }
    }

    #[test]
    fn decodes_anchor_cpi_binary_event_to_existing_tfhe_event() {
        let encoded = anchor_cpi_event(
            "FheBinaryOpEvent",
            binary_op_payload(1, [9; 32], [1; 32], [2; 32], false, [3; 32]),
        );

        let ZamaHostEvent::FheBinaryOp(event) =
            decode_anchor_cpi_event(&encoded)
                .expect("expected binary op event")
        else {
            panic!("expected binary op event");
        };
        let mapped = to_tfhe_event(event);

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
            let mut payload = vec![EVENT_VERSION];
            payload.extend_from_slice(&[9; 32]);
            payload.extend_from_slice(&[0; 31]);
            payload.push(7);
            payload.push(5);
            payload.extend_from_slice(&[8; 32]);
            payload
        });

        let ZamaHostEvent::TrivialEncrypt(event) =
            decode_anchor_cpi_event(&encoded).expect("expected trivial event")
        else {
            panic!("expected trivial encrypt event");
        };
        let mapped = to_trivial_encrypt_event(event);

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
            let mut payload = vec![EVENT_VERSION];
            payload.extend_from_slice(&[7; 32]);
            payload.extend_from_slice(&[8; 32]);
            payload
        });

        let decoded =
            decode_anchor_cpi_event(&encoded).expect("expected ACL event");
        let (_, acl_events) = normalize_solana_events_for_db(
            [decoded],
            solana_transaction_id(&[0; 64]),
            test_block(),
        );

        assert_eq!(acl_events.len(), 1);
        assert_eq!(acl_events[0].handle, handle(7));
        assert_eq!(
            acl_events[0].subject,
            "0x0808080808080808080808080808080808080808080808080808080808080808"
        );
        assert_eq!(
            acl_events[0].event_type as i16,
            AllowEvents::AllowedAccount as i16
        );
    }

    #[test]
    fn decodes_anchor_cpi_public_decrypt_acl_event() {
        let encoded = anchor_cpi_event("AclPublicDecryptAllowedEvent", {
            let mut payload = vec![EVENT_VERSION];
            payload.extend_from_slice(&[7; 32]);
            payload.extend_from_slice(&[8; 32]);
            payload
        });

        let decoded = decode_anchor_cpi_event(&encoded)
            .expect("expected public decrypt ACL event");
        let (_, acl_events) = normalize_solana_events_for_db(
            [decoded],
            solana_transaction_id(&[0; 64]),
            test_block(),
        );

        assert_eq!(acl_events.len(), 1);
        assert_eq!(acl_events[0].handle, handle(7));
        assert_eq!(
            acl_events[0].event_type as i16,
            AllowEvents::AllowedForDecryption as i16
        );
    }

    #[test]
    fn rejects_anchor_cpi_event_with_wrong_version() {
        let encoded = anchor_cpi_event("AclAllowedEvent", {
            let mut payload = vec![EVENT_VERSION + 1];
            payload.extend_from_slice(&[7; 32]);
            payload.extend_from_slice(&[8; 32]);
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
    fn normalizes_acl_subject_as_full_32_byte_hex() {
        let (_, acl_events) = normalize_solana_events_for_db(
            [ZamaHostEvent::AclAllowed(AclAllowedEvent {
                version: EVENT_VERSION,
                handle: [9; 32],
                subject: [0xab; 32],
            })],
            solana_transaction_id(&[0; 64]),
            test_block(),
        );

        assert_eq!(acl_events.len(), 1);
        assert_eq!(acl_events[0].handle, handle(9));
        // Full 32-byte Solana pubkey hex (0x + 64 chars), not a 20-byte EVM address.
        assert_eq!(acl_events[0].subject, format!("0x{}", "ab".repeat(32)));
        assert_eq!(acl_events[0].subject.len(), 66);
        assert_eq!(
            acl_events[0].event_type as i16,
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

        let (tfhe_logs, acl_events) = normalize_solana_events_for_db(
            [
                ZamaHostEvent::FheBinaryOp(FheBinaryOpEvent {
                    version: EVENT_VERSION,
                    op: FheBinaryOpCode::Add,
                    subject: [0; 32],
                    lhs: [1; 32],
                    rhs: [2; 32],
                    scalar: false,
                    result: [3; 32],
                }),
                ZamaHostEvent::AclAllowed(AclAllowedEvent {
                    version: EVENT_VERSION,
                    handle: [3; 32],
                    subject: [4; 32],
                }),
            ],
            tx_id,
            test_block(),
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

        let (tfhe_logs, acl_events) = normalize_solana_events_for_db(
            [
                ZamaHostEvent::FheBinaryOp(FheBinaryOpEvent {
                    version: EVENT_VERSION,
                    op: FheBinaryOpCode::Sub,
                    subject: [0; 32],
                    lhs: [1; 32],
                    rhs: [2; 32],
                    scalar: false,
                    result: [3; 32],
                }),
                ZamaHostEvent::AclAllowed(AclAllowedEvent {
                    version: EVENT_VERSION,
                    handle: [9; 32],
                    subject: [4; 32],
                }),
            ],
            tx_id,
            test_block(),
        );

        assert_eq!(acl_events.len(), 1);
        assert_eq!(tfhe_logs.len(), 1);
        assert!(!tfhe_logs[0].is_allowed);
    }

    #[test]
    fn decodes_anchor_cpi_rand_event() {
        let encoded = anchor_cpi_event(
            "FheRandEvent",
            rand_event_payload([9; 32], [0xAB; 16], 5, [8; 32]),
        );

        let ZamaHostEvent::FheRand(event) =
            decode_anchor_cpi_event(&encoded).expect("expected rand event")
        else {
            panic!("expected rand event");
        };

        assert_eq!(event.version, EVENT_VERSION);
        assert_eq!(event.subject, [9; 32]);
        assert_eq!(event.seed, [0xAB; 16]);
        assert_eq!(event.fhe_type, 5);
        assert_eq!(event.result, [8; 32]);
    }

    #[test]
    fn maps_rand_to_existing_tfhe_event() {
        let seed = [0xCD; 16];
        let mapped = to_fhe_rand_event(FheRandEvent {
            version: EVENT_VERSION,
            subject: [0; 32],
            seed,
            fhe_type: 5,
            result: [4; 32],
        });

        assert!(matches!(
            mapped.data,
            TfheContractEvents::FheRand(TfheContract::FheRand {
                randType,
                seed: mapped_seed,
                result,
                ..
            }) if randType == 5
                && mapped_seed == FixedBytes::<16>::from(seed)
                && result == handle(4)
        ));
    }

    fn anchor_cpi_event(name: &str, payload: Vec<u8>) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&ANCHOR_EVENT_IX_TAG_LE);
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

    fn rand_event_payload(
        subject: [u8; 32],
        seed: [u8; 16],
        fhe_type: u8,
        result: [u8; 32],
    ) -> Vec<u8> {
        let mut payload = vec![EVENT_VERSION];
        payload.extend_from_slice(&subject);
        payload.extend_from_slice(&seed);
        payload.push(fhe_type);
        payload.extend_from_slice(&result);
        payload
    }
}
