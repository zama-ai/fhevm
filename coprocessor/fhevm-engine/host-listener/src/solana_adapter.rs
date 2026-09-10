//! Transport-agnostic core of the Solana ingestion path: maps reconstructed
//! Solana compute and material requests into the coprocessor database.
//!
//! Design rationale lives in `solana/docs/DESIGN_DECISIONS.md` (event transport:
//! DD-003; eager ciphertext-material preparation: DD-024).

use std::collections::{HashMap, HashSet};

use crate::database::computation::{Computation, Operand};
use alloy_primitives::FixedBytes;
use fhevm_engine_common::types::SupportedFheOperations as O;
use sha2::{Digest, Sha256};
use sqlx::Error as SqlxError;
use Operand::{Clear as P, Encrypted as H};

use zama_host::records::{
    FheBinaryOp, FheIsIn, FheMulDiv, FheRand, FheRandBounded, FheSum,
    FheTernaryOp, FheUnaryOp, TrivialEncrypt,
};
use zama_host::state::{FheBinaryOpCode, FheTernaryOpCode, FheUnaryOpCode};

use crate::cmd::block_history::BlockSummary;
use crate::database::dependence_chains::dependence_chains;
use crate::database::ingest::{
    classify_slow_chains, populate_operand_boundary_masks,
};
use crate::database::tfhe_event_propagate::{
    Database, Handle, LogTfhe, Transaction, TransactionHash,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaMaterialRequest {
    pub handle: Handle,
}

#[derive(Clone, Debug)]
pub enum SolanaHostRecord {
    FheBinaryOp(FheBinaryOp),
    FheTernaryOp(FheTernaryOp),
    TrivialEncrypt(TrivialEncrypt),
    FheRand(FheRand),
    FheRandBounded(FheRandBounded),
    FheUnaryOp(FheUnaryOp),
    FheSum(FheSum),
    FheIsIn(FheIsIn),
    FheMulDiv(FheMulDiv),
    MaterialRequest(SolanaMaterialRequest),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolanaBlockMeta {
    pub block_number: u64,
    pub block_timestamp: time::PrimitiveDateTime,
    pub block_hash: [u8; 32],
    pub parent_hash: [u8; 32],
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SolanaIngestStats {
    pub tfhe_events: usize,
    pub material_requests: usize,
    pub inserted_rows: usize,
}

/// The coprocessor schema's 32-byte `transaction_id` for a Solana transaction is
/// `sha256(signature_bytes)`: Solana's native transaction id is the 64-byte ed25519 signature,
/// hashed down here to fit the EVM-shaped 32-byte column. Anything joining coprocessor rows back
/// to a Solana explorer must apply this same mapping.
pub fn solana_transaction_id(signature_bytes: &[u8]) -> TransactionHash {
    let digest: [u8; 32] = Sha256::digest(signature_bytes).into();
    TransactionHash::from(digest)
}

// Only referenced by `solana_grpc_listener` (feature-gated) outside of tests.
#[cfg_attr(not(feature = "solana-grpc"), allow(dead_code))]
pub(crate) fn material_request(handle: [u8; 32]) -> SolanaMaterialRequest {
    SolanaMaterialRequest {
        handle: Handle::from(handle),
    }
}

fn dedup_material_requests(requests: &mut Vec<SolanaMaterialRequest>) {
    let mut seen = HashSet::new();
    requests.retain(|request| seen.insert(request.handle));
}

// Solana computations and ciphertext-material preparation are scheduled as
// soon as their instruction confirms. The KMS independently validates the live
// EncryptedStore PDA and any MMR proof before releasing plaintext, so this eager
// work can waste cycles after a rare rollback but cannot authorize decryption.
pub fn normalize_solana_records_for_db(
    records: impl IntoIterator<Item = SolanaHostRecord>,
    transaction_id: TransactionHash,
    block: SolanaBlockMeta,
) -> Result<(Vec<LogTfhe>, Vec<SolanaMaterialRequest>), String> {
    let mut tfhe_logs = Vec::new();
    let mut material_requests = Vec::new();
    for record in records {
        let computation = match record {
            SolanaHostRecord::FheBinaryOp(record) => binary_computation(record),
            SolanaHostRecord::FheTernaryOp(record) => {
                ternary_computation(record)
            }
            SolanaHostRecord::TrivialEncrypt(record) => {
                trivial_computation(record)
            }
            SolanaHostRecord::FheRand(record) => random_computation(record),
            SolanaHostRecord::FheRandBounded(record) => {
                bounded_random_computation(record)
            }
            SolanaHostRecord::FheUnaryOp(record) => unary_computation(record),
            SolanaHostRecord::FheSum(record) => Computation::single(
                O::FheSum,
                record
                    .operands
                    .into_iter()
                    .map(Operand::encrypted)
                    .collect(),
                record.result.into(),
            ),
            SolanaHostRecord::FheIsIn(record) => Computation::single(
                O::FheIsIn,
                std::iter::once(record.value)
                    .chain(record.set)
                    .map(Operand::encrypted)
                    .collect(),
                record.result.into(),
            ),
            SolanaHostRecord::FheMulDiv(record) => Computation::single(
                O::FheMulDiv,
                vec![
                    H(record.factor1.into()),
                    Operand::binary_rhs(record.factor2.into(), record.scalar),
                    P(record.divisor.to_vec()),
                ],
                record.result.into(),
            ),
            SolanaHostRecord::MaterialRequest(request) => {
                material_requests.push(request);
                continue;
            }
        }?;
        tfhe_logs.push(to_log_tfhe(computation, transaction_id, block));
    }

    dedup_material_requests(&mut material_requests);
    Ok((tfhe_logs, material_requests))
}

/// Ingests ordered transaction groups from one sealed block. Dependency grouping sees the whole
/// block; operand origins and material requests retain their transaction identity.
pub async fn insert_solana_block_records(
    db: &Database,
    tx: &mut Transaction<'_>,
    transactions: impl IntoIterator<Item = (TransactionHash, Vec<SolanaHostRecord>)>,
    block: SolanaBlockMeta,
    dependent_ops_max_per_chain: u32,
) -> Result<SolanaIngestStats, SqlxError> {
    let mut tfhe_logs = Vec::new();
    let mut material_requests = Vec::new();
    for (transaction_id, records) in transactions {
        let (logs, requests) =
            normalize_solana_records_for_db(records, transaction_id, block)
                .map_err(SqlxError::Protocol)?;
        tfhe_logs.extend(logs);
        material_requests.extend(
            requests
                .into_iter()
                .map(|request| (transaction_id, request)),
        );
    }
    for (index, log) in tfhe_logs.iter_mut().enumerate() {
        log.log_index = Some(index as u64);
    }
    populate_operand_boundary_masks(&mut tfhe_logs)?;
    let chains = dependence_chains(
        &mut tfhe_logs,
        &db.dependence_chain,
        &db.consumed_boundaries,
        &db.sealed_chains,
        false,
        true,
    )
    .await;

    let mut inserted_compute = 0;
    let mut dependent_ops_by_chain = HashMap::new();
    for log in &tfhe_logs {
        let inserted = db.insert_tfhe_event(tx, log).await?;
        inserted_compute += inserted;
        if dependent_ops_max_per_chain > 0 && inserted > 0 {
            let count = dependent_ops_by_chain
                .entry(log.dependence_chain)
                .or_insert(0_u64);
            *count = count.saturating_add(inserted as u64);
        }
    }
    let mut inserted_rows = inserted_compute;
    for (transaction_id, request) in &material_requests {
        if db
            .insert_pbs_computations(
                tx,
                &[request.handle.to_vec()],
                Some(transaction_id.to_vec()),
                block.block_number,
            )
            .await?
        {
            inserted_rows += 1;
        }
    }
    // A complete replay must not rearm a processed chain.
    if inserted_compute > 0 {
        let slow_chains = classify_slow_chains(
            db,
            tx,
            &chains,
            &dependent_ops_by_chain,
            dependent_ops_max_per_chain,
        )
        .await?;
        db.update_dependence_chain(
            tx,
            chains,
            block.block_timestamp,
            &solana_block_summary(block),
            &slow_chains,
        )
        .await?;
    }
    Ok(SolanaIngestStats {
        tfhe_events: tfhe_logs.len(),
        material_requests: material_requests.len(),
        inserted_rows,
    })
}

fn solana_block_summary(block: SolanaBlockMeta) -> BlockSummary {
    BlockSummary {
        number: block.block_number,
        hash: FixedBytes::<32>::from(block.block_hash),
        parent_hash: FixedBytes::<32>::from(block.parent_hash),
        timestamp: 0,
    }
}

fn to_log_tfhe(
    computation: Computation,
    transaction_id: TransactionHash,
    block: SolanaBlockMeta,
) -> LogTfhe {
    LogTfhe {
        allowed_outputs: computation.outputs().iter().copied().collect(),
        computation,
        transaction_hash: Some(transaction_id),
        block_number: block.block_number,
        block_hash: FixedBytes::<32>::from(block.block_hash),
        block_timestamp: block.block_timestamp,
        // Placeholders: overwritten by the shared `dependence_chains()` union-find in
        // `insert_solana_block_records` before insertion, exactly like the EVM ingest path's
        // `Default::default()` placeholders.
        tx_depth_size: 0,
        dependence_chain: transaction_id,
        // Assigned across the complete sealed block before origin reconstruction.
        log_index: None,
        // Every reconstructed host op ran on-chain in this signature. Operand
        // origin bits are derived afterwards by `populate_operand_boundary_masks`,
        // the same walk the EVM ingest path uses.
        operand_boundary_mask: None,
        is_executor_minted: true,
    }
}

fn binary_computation(event: FheBinaryOp) -> Result<Computation, String> {
    let operation = match event.op {
        FheBinaryOpCode::Add => O::FheAdd,
        FheBinaryOpCode::Sub => O::FheSub,
        FheBinaryOpCode::Mul => O::FheMul,
        FheBinaryOpCode::Div => O::FheDiv,
        FheBinaryOpCode::Rem => O::FheRem,
        FheBinaryOpCode::And => O::FheBitAnd,
        FheBinaryOpCode::Or => O::FheBitOr,
        FheBinaryOpCode::Xor => O::FheBitXor,
        FheBinaryOpCode::Shl => O::FheShl,
        FheBinaryOpCode::Shr => O::FheShr,
        FheBinaryOpCode::Rotl => O::FheRotl,
        FheBinaryOpCode::Rotr => O::FheRotr,
        FheBinaryOpCode::Eq => O::FheEq,
        FheBinaryOpCode::Ne => O::FheNe,
        FheBinaryOpCode::Ge => O::FheGe,
        FheBinaryOpCode::Gt => O::FheGt,
        FheBinaryOpCode::Le => O::FheLe,
        FheBinaryOpCode::Lt => O::FheLt,
        FheBinaryOpCode::Min => O::FheMin,
        FheBinaryOpCode::Max => O::FheMax,
    };
    Computation::single(
        operation,
        vec![
            H(event.lhs.into()),
            Operand::binary_rhs(event.rhs.into(), event.scalar),
        ],
        event.result.into(),
    )
}

fn ternary_computation(event: FheTernaryOp) -> Result<Computation, String> {
    let operation = match event.op {
        FheTernaryOpCode::IfThenElse => O::FheIfThenElse,
    };
    Computation::single(
        operation,
        vec![
            H(event.control.into()),
            H(event.if_true.into()),
            H(event.if_false.into()),
        ],
        event.result.into(),
    )
}

fn trivial_computation(event: TrivialEncrypt) -> Result<Computation, String> {
    Ok(Computation::trivial(
        event.plaintext,
        event.fhe_type,
        event.result.into(),
    ))
}

fn random_computation(event: FheRand) -> Result<Computation, String> {
    Computation::single(
        O::FheRand,
        vec![P(event.seed.to_vec()), P(vec![event.fhe_type])],
        event.result.into(),
    )
}

fn bounded_random_computation(
    event: FheRandBounded,
) -> Result<Computation, String> {
    Computation::single(
        O::FheRandBounded,
        vec![
            P(event.seed.to_vec()),
            P(event.upper_bound.to_vec()),
            P(vec![event.fhe_type]),
        ],
        event.result.into(),
    )
}

fn unary_computation(event: FheUnaryOp) -> Result<Computation, String> {
    let mut operands = vec![H(event.operand.into())];
    let operation = match event.op {
        FheUnaryOpCode::Neg => O::FheNeg,
        FheUnaryOpCode::Not => O::FheNot,
        FheUnaryOpCode::Cast => {
            operands.push(P(vec![event.result[30]]));
            O::FheCast
        }
    };
    Computation::single(operation, operands, event.result.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::{Date, Month, PrimitiveDateTime, Time};
    use zama_host::EVENT_VERSION;

    #[test]
    fn malformed_collection_fails_normalization() {
        let result = normalize_solana_records_for_db(
            [SolanaHostRecord::FheSum(FheSum {
                version: EVENT_VERSION,
                fhe_type: 5,
                operands: vec![[1; 32]; 257],
                result: [2; 32],
            })],
            Handle::ZERO,
            SolanaBlockMeta {
                block_number: 1,
                block_timestamp: PrimitiveDateTime::MIN,
                block_hash: [1; 32],
                parent_hash: [0; 32],
            },
        );
        assert!(result.is_err());
    }

    fn handle(byte: u8) -> Handle {
        Handle::from([byte; 32])
    }

    #[test]
    fn maps_binary_add_to_computation() {
        let mapped = binary_computation(FheBinaryOp {
            version: EVENT_VERSION,
            op: FheBinaryOpCode::Add,
            lhs: [1; 32],
            rhs: [2; 32],
            scalar: false,
            result: [3; 32],
        })
        .unwrap();

        assert_eq!(
            mapped,
            Computation::single(
                O::FheAdd,
                vec![H(handle(1)), H(handle(2))],
                handle(3)
            )
            .unwrap()
        );
    }

    #[test]
    fn maps_binary_ge_to_computation() {
        let mapped = binary_computation(FheBinaryOp {
            version: EVENT_VERSION,
            op: FheBinaryOpCode::Ge,
            lhs: [1; 32],
            rhs: [2; 32],
            scalar: false,
            result: [3; 32],
        })
        .unwrap();

        assert_eq!(
            mapped,
            Computation::single(
                O::FheGe,
                vec![H(handle(1)), H(handle(2))],
                handle(3)
            )
            .unwrap()
        );
    }

    #[test]
    fn maps_ternary_if_then_else_to_computation() {
        let mapped = ternary_computation(FheTernaryOp {
            version: EVENT_VERSION,
            op: FheTernaryOpCode::IfThenElse,
            control: [1; 32],
            if_true: [2; 32],
            if_false: [3; 32],
            result: [4; 32],
        })
        .unwrap();

        assert_eq!(
            mapped,
            Computation::single(
                O::FheIfThenElse,
                vec![H(handle(1)), H(handle(2)), H(handle(3))],
                handle(4)
            )
            .unwrap()
        );
    }

    #[test]
    fn maps_trivial_encrypt_to_computation() {
        let mut plaintext = [0_u8; 32];
        plaintext[31] = 7;

        let mapped = trivial_computation(TrivialEncrypt {
            version: EVENT_VERSION,
            plaintext,
            fhe_type: 5,
            result: [8; 32],
        })
        .unwrap();

        assert_eq!(
            mapped,
            Computation::single(
                O::FheTrivialEncrypt,
                vec![P(plaintext.to_vec()), P(vec![5])],
                handle(8)
            )
            .unwrap()
        );
    }

    #[test]
    fn maps_random_to_computation() {
        let mapped = random_computation(FheRand {
            version: EVENT_VERSION,
            seed: [7; 16],
            fhe_type: 5,
            result: [8; 32],
        })
        .unwrap();

        assert_eq!(
            mapped,
            Computation::single(
                O::FheRand,
                vec![P(vec![7; 16]), P(vec![5])],
                handle(8)
            )
            .unwrap()
        );
    }

    #[test]
    fn binary_decoders_agree_on_operation_operand_order_and_scalar_encoding() {
        use crate::contracts::{
            TfheContract as C, TfheContract::TfheContractEvents as E,
        };
        let caller = alloy_primitives::Address::ZERO;
        let lhs = handle(1);
        let rhs = handle(2);
        let result = handle(3);
        for scalar in [false, true] {
            let scalar_byte = FixedBytes::from([u8::from(scalar)]);
            macro_rules! case {
                ($op:ident, $event:ident) => {
                    (
                        FheBinaryOpCode::$op,
                        O::$event,
                        E::$event(C::$event {
                            caller,
                            lhs,
                            rhs,
                            scalarByte: scalar_byte,
                            result,
                        }),
                    )
                };
            }
            let cases = [
                case!(Add, FheAdd),
                case!(Sub, FheSub),
                case!(Mul, FheMul),
                case!(Div, FheDiv),
                case!(Rem, FheRem),
                case!(And, FheBitAnd),
                case!(Or, FheBitOr),
                case!(Xor, FheBitXor),
                case!(Shl, FheShl),
                case!(Shr, FheShr),
                case!(Rotl, FheRotl),
                case!(Rotr, FheRotr),
                case!(Eq, FheEq),
                case!(Ne, FheNe),
                case!(Ge, FheGe),
                case!(Gt, FheGt),
                case!(Le, FheLe),
                case!(Lt, FheLt),
                case!(Min, FheMin),
                case!(Max, FheMax),
            ];
            for (op, operation, evm) in cases {
                let solana = binary_computation(FheBinaryOp {
                    version: EVENT_VERSION,
                    op,
                    lhs: lhs.0,
                    rhs: rhs.0,
                    scalar,
                    result: result.0,
                })
                .unwrap();
                assert_eq!(
                    solana,
                    Computation::from_evm(&evm).unwrap().unwrap()
                );
                assert_eq!(solana.operation(), operation);
                assert_eq!(
                    solana
                        .operands()
                        .iter()
                        .map(Operand::bytes)
                        .collect::<Vec<_>>(),
                    vec![lhs.to_vec(), rhs.to_vec()]
                );
                assert_eq!(solana.is_scalar(), scalar);
                assert_eq!(
                    solana.inputs(),
                    if scalar { vec![lhs] } else { vec![lhs, rhs] }
                );
                assert_eq!(
                    solana.boundary_mask(|_| false).unwrap()[31],
                    if scalar { 1 } else { 3 }
                );
            }
        }
    }

    #[test]
    fn native_special_operations_preserve_worker_operands() {
        let result = [8; 32];
        let version = EVENT_VERSION;
        let cases = [
            (
                SolanaHostRecord::FheUnaryOp(FheUnaryOp {
                    version,
                    op: FheUnaryOpCode::Neg,
                    operand: [1; 32],
                    result,
                }),
                O::FheNeg,
                vec![H(handle(1))],
                false,
            ),
            (
                SolanaHostRecord::FheUnaryOp(FheUnaryOp {
                    version,
                    op: FheUnaryOpCode::Not,
                    operand: [1; 32],
                    result,
                }),
                O::FheNot,
                vec![H(handle(1))],
                false,
            ),
            (
                SolanaHostRecord::FheUnaryOp(FheUnaryOp {
                    version,
                    op: FheUnaryOpCode::Cast,
                    operand: [1; 32],
                    result,
                }),
                O::FheCast,
                vec![H(handle(1)), P(vec![8])],
                true,
            ),
            (
                SolanaHostRecord::FheRandBounded(FheRandBounded {
                    version,
                    seed: [1; 16],
                    upper_bound: [2; 32],
                    fhe_type: 5,
                    result,
                }),
                O::FheRandBounded,
                vec![P(vec![1; 16]), P(vec![2; 32]), P(vec![5])],
                true,
            ),
            (
                SolanaHostRecord::FheSum(FheSum {
                    version,
                    fhe_type: 5,
                    operands: vec![[1; 32], [2; 32]],
                    result,
                }),
                O::FheSum,
                vec![H(handle(1)), H(handle(2))],
                false,
            ),
            (
                SolanaHostRecord::FheIsIn(FheIsIn {
                    version,
                    fhe_type: 5,
                    value: [1; 32],
                    set: vec![[2; 32], [3; 32]],
                    result,
                }),
                O::FheIsIn,
                vec![H(handle(1)), H(handle(2)), H(handle(3))],
                false,
            ),
            (
                SolanaHostRecord::FheMulDiv(FheMulDiv {
                    version,
                    factor1: [1; 32],
                    factor2: [2; 32],
                    divisor: [3; 32],
                    scalar: false,
                    result,
                }),
                O::FheMulDiv,
                vec![H(handle(1)), H(handle(2)), P(vec![3; 32])],
                false,
            ),
            (
                SolanaHostRecord::FheMulDiv(FheMulDiv {
                    version,
                    factor1: [1; 32],
                    factor2: [2; 32],
                    divisor: [3; 32],
                    scalar: true,
                    result,
                }),
                O::FheMulDiv,
                vec![H(handle(1)), P(vec![2; 32]), P(vec![3; 32])],
                true,
            ),
        ];
        for (record, operation, operands, scalar) in cases {
            let (logs, requests) = normalize_solana_records_for_db(
                [record],
                Handle::ZERO,
                SolanaBlockMeta {
                    block_number: 1,
                    block_timestamp: PrimitiveDateTime::MIN,
                    block_hash: [1; 32],
                    parent_hash: [0; 32],
                },
            )
            .unwrap();
            assert!(requests.is_empty());
            assert_eq!(logs.len(), 1);
            assert_eq!(
                logs[0].computation,
                Computation::single(operation, operands, result.into())
                    .unwrap()
            );
            assert_eq!(logs[0].computation.is_scalar(), scalar);
        }
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
    fn dependence_chain_uses_validator_block_hashes() {
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );
        let summary = solana_block_summary(SolanaBlockMeta {
            block_number: 42,
            block_timestamp,
            block_hash: [7; 32],
            parent_hash: [6; 32],
        });

        assert_eq!(summary.number, 42);
        assert_eq!(summary.hash, FixedBytes::<32>::from([7; 32]));
        assert_eq!(summary.parent_hash, FixedBytes::<32>::from([6; 32]));
    }

    #[test]
    fn builds_existing_db_log_shape() {
        let tx_id = solana_transaction_id(&[1_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );
        let event = binary_computation(FheBinaryOp {
            version: EVENT_VERSION,
            op: FheBinaryOpCode::Sub,
            lhs: [1; 32],
            rhs: [2; 32],
            scalar: true,
            result: [3; 32],
        })
        .unwrap();

        let log = to_log_tfhe(
            event,
            tx_id,
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
                block_hash: [1; 32],
                parent_hash: [0; 32],
            },
        );

        assert_eq!(log.transaction_hash, Some(tx_id));
        assert_eq!(log.block_number, 42);
        assert_eq!(log.block_timestamp, block_timestamp);
        assert!(!log.allowed_outputs.is_empty());
        assert_eq!(log.log_index, None);
        assert!(log.is_executor_minted);
        assert!(log.operand_boundary_mask.is_none());
    }

    #[test]
    fn compute_is_eager_regardless_of_same_tx_allow_signal() {
        // Historically, the execution's compute would only be marked
        // materializable when an allow for its result landed in the same tx.
        // Under eager compute (RFC-024 Q11), it is unconditionally scheduled;
        // KMS independently gates plaintext release against Solana ACL state.
        let tx_id = solana_transaction_id(&[7_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );

        let (tfhe_logs, material_requests) = normalize_solana_records_for_db(
            [
                SolanaHostRecord::TrivialEncrypt(TrivialEncrypt {
                    version: EVENT_VERSION,
                    plaintext: [55; 32],
                    fhe_type: 5,
                    result: [3; 32],
                }),
                SolanaHostRecord::MaterialRequest(material_request([3; 32])),
            ],
            tx_id,
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
                block_hash: [1; 32],
                parent_hash: [0; 32],
            },
        )
        .unwrap();

        assert_eq!(tfhe_logs.len(), 1);
        assert!(
            !tfhe_logs[0].allowed_outputs.is_empty(),
            "eager compute: schedulable independent of the allow signal"
        );
        // The persistent handle is queued directly for material preparation.
        assert_eq!(material_requests.len(), 1);
    }

    #[test]
    fn material_requests_keep_distinct_handles_in_one_batch() {
        let tx_id = solana_transaction_id(&[9_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );

        let (_, material_requests) = normalize_solana_records_for_db(
            [
                SolanaHostRecord::MaterialRequest(material_request([1; 32])),
                SolanaHostRecord::MaterialRequest(material_request([2; 32])),
            ],
            tx_id,
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
                block_hash: [1; 32],
                parent_hash: [0; 32],
            },
        )
        .unwrap();

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

        let (tfhe_logs, _) = normalize_solana_records_for_db(
            [
                SolanaHostRecord::TrivialEncrypt(TrivialEncrypt {
                    version: EVENT_VERSION,
                    plaintext: [55; 32],
                    fhe_type: 5,
                    result: [3; 32],
                }),
                SolanaHostRecord::MaterialRequest(material_request([4; 32])),
            ],
            tx_id,
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
                block_hash: [1; 32],
                parent_hash: [0; 32],
            },
        )
        .unwrap();

        assert_eq!(tfhe_logs.len(), 1);
        assert!(
            !tfhe_logs[0].allowed_outputs.is_empty(),
            "eager compute: always schedulable"
        );
    }

    #[test]
    fn normalizes_interleaved_batch_events_for_worker_replay() {
        let tx_id = solana_transaction_id(&[5_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );

        let (tfhe_logs, material_requests) = normalize_solana_records_for_db(
            [
                SolanaHostRecord::TrivialEncrypt(TrivialEncrypt {
                    version: EVENT_VERSION,
                    plaintext: {
                        let mut plaintext = [0_u8; 32];
                        plaintext[31] = 1;
                        plaintext
                    },
                    fhe_type: 0,
                    result: [1; 32],
                }),
                SolanaHostRecord::FheRand(FheRand {
                    version: EVENT_VERSION,
                    seed: [2; 16],
                    fhe_type: 5,
                    result: [2; 32],
                }),
                SolanaHostRecord::FheTernaryOp(FheTernaryOp {
                    version: EVENT_VERSION,
                    op: FheTernaryOpCode::IfThenElse,
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
                block_hash: [1; 32],
                parent_hash: [0; 32],
            },
        )
        .unwrap();

        assert!(material_requests.is_empty());
        assert_eq!(tfhe_logs.len(), 3);
        assert_eq!(
            tfhe_logs
                .iter()
                .map(|log| log.log_index)
                .collect::<Vec<_>>(),
            vec![None; 3],
            "block ingestion assigns indexes across transaction groups"
        );
        assert!(
            !tfhe_logs[0].allowed_outputs.is_empty(),
            "eager compute: always schedulable"
        );
        assert!(
            !tfhe_logs[1].allowed_outputs.is_empty(),
            "eager compute: always schedulable"
        );
        assert!(
            !tfhe_logs[2].allowed_outputs.is_empty(),
            "eager compute: always schedulable"
        );
        assert_eq!(tfhe_logs[0].computation.operation(), O::FheTrivialEncrypt);
        assert_eq!(tfhe_logs[1].computation.operation(), O::FheRand);
        assert_eq!(tfhe_logs[2].computation.operation(), O::FheIfThenElse);
        assert!(tfhe_logs[0].computation.inputs().is_empty());
        assert!(tfhe_logs[1].computation.inputs().is_empty());
        assert_eq!(
            tfhe_logs[2].computation.inputs(),
            vec![handle(1), handle(2), handle(1)]
        );
    }
}
