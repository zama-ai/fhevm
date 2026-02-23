use alloy::primitives::FixedBytes;
use bigdecimal::num_bigint::BigInt;
use serial_test::serial;
use sqlx::types::time::PrimitiveDateTime;

use fhevm_engine_common::types::AllowEvents;
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{
    ClearConst, Database as ListenerDatabase, Handle, LogTfhe, Transaction,
};

use crate::tests::event_helpers::{
    log_with_tx, next_handle, setup_event_harness, tfhe_event, to_ty, zero_address,
};
use crate::tests::test_cases::{
    generate_binary_test_cases, generate_unary_test_cases, BinaryOperatorTestCase,
    UnaryOperatorTestCase,
};
use crate::tests::utils::{decrypt_ciphertexts, wait_until_all_allowed_handles_computed};

const LOCAL_SUPPORTED_TYPES: &[i32] = &[
    0, // bool
    1, // 4 bit
    2, // 8 bit
    3, // 16 bit
    4, // 32 bit
    5, // 64 bit
];

const FULL_SUPPORTED_TYPES: &[i32] = &[
    0,  // bool
    1,  // 4 bit
    2,  // 8 bit
    3,  // 16 bit
    4,  // 32 bit
    5,  // 64 bit
    6,  // 128 bit
    7,  // 160 bit
    8,  // 256 bit
    9,  // 512 bit
    10, // 1024 bit
    11, // 2048 bit
];

pub fn supported_types() -> &'static [i32] {
    match std::env::var("TFHE_WORKER_EVENT_TYPE_MATRIX") {
        Ok(mode) if mode.eq_ignore_ascii_case("local") => LOCAL_SUPPORTED_TYPES,
        _ => FULL_SUPPORTED_TYPES,
    }
}

async fn insert_tfhe_event(
    db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    log: alloy::rpc::types::Log<TfheContractEvents>,
    is_allowed: bool,
) -> Result<bool, sqlx::Error> {
    let event = LogTfhe {
        event: log.inner,
        transaction_hash: log.transaction_hash,
        is_allowed,
        block_number: log.block_number.unwrap_or(0),
        block_timestamp: PrimitiveDateTime::MAX,
        dependence_chain: log.transaction_hash.unwrap_or_default(),
        tx_depth_size: 0,
        log_index: log.log_index,
    };
    db.insert_tfhe_event(tx, &event).await
}

async fn allow_handle(
    db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    handle: &[u8],
) -> Result<bool, sqlx::Error> {
    let account_address = String::new();
    let event_type = AllowEvents::AllowedForDecryption;
    db.insert_allowed_handle(tx, handle.to_owned(), account_address, event_type, None)
        .await
}

fn as_scalar_handle(big_int: &BigInt) -> Handle {
    let (_, mut bytes) = big_int.to_bytes_le();
    while bytes.len() < 32 {
        bytes.push(0_u8)
    }
    bytes.reverse();
    Handle::from_slice(&bytes)
}

fn as_scalar_uint(big_int: &BigInt) -> ClearConst {
    let (_, bytes) = big_int.to_bytes_be();
    ClearConst::from_be_slice(&bytes)
}

fn binary_op_to_event(
    op: &BinaryOperatorTestCase,
    lhs: &Handle,
    rhs: &Handle,
    r_scalar: &BigInt,
    result: &Handle,
) -> TfheContractEvents {
    use fhevm_engine_common::types::SupportedFheOperations as S;
    use host_listener::contracts::TfheContract as C;
    use host_listener::contracts::TfheContract::TfheContractEvents as E;
    use host_listener::database::tfhe_event_propagate::ScalarByte;
    let caller = zero_address();
    let s_byte = |is_scalar: bool| ScalarByte::from(is_scalar as u8);
    #[expect(non_snake_case)]
    let scalarByte = s_byte(op.is_scalar);
    let lhs = *lhs;
    let rhs = if op.is_scalar && op.bits <= 256 {
        as_scalar_handle(r_scalar)
    } else {
        *rhs
    };
    let result = *result;
    match S::try_from(op.operator).unwrap() {
        S::FheAdd => E::FheAdd(C::FheAdd {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheSub => E::FheSub(C::FheSub {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheMul => E::FheMul(C::FheMul {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheDiv => E::FheDiv(C::FheDiv {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheRem => E::FheRem(C::FheRem {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheBitAnd => E::FheBitAnd(C::FheBitAnd {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheBitOr => E::FheBitOr(C::FheBitOr {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheBitXor => E::FheBitXor(C::FheBitXor {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheShl => E::FheShl(C::FheShl {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheShr => E::FheShr(C::FheShr {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheRotl => E::FheRotl(C::FheRotl {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheRotr => E::FheRotr(C::FheRotr {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheMax => E::FheMax(C::FheMax {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheMin => E::FheMin(C::FheMin {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheGe => E::FheGe(C::FheGe {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheGt => E::FheGt(C::FheGt {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheLe => E::FheLe(C::FheLe {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheLt => E::FheLt(C::FheLt {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheEq => E::FheEq(C::FheEq {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        S::FheNe => E::FheNe(C::FheNe {
            caller,
            lhs,
            rhs,
            scalarByte,
            result,
        }),
        _ => panic!("unknown operation: {:?}", op.operator),
    }
}

#[tokio::test]
#[serial(db)]
async fn test_fhe_binary_operands_events() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let mut cases = vec![];
    for op in generate_binary_test_cases() {
        if !supported_types().contains(&op.input_types) {
            continue;
        }
        // TrivialEncrypt test setup uses ClearConst (up to 256-bit payloads).
        if op.bits > 256 {
            continue;
        }
        let lhs_handle = next_handle();
        let rhs_handle = next_handle();
        let output_handle = next_handle();
        let transaction_id = next_handle();

        let lhs_bytes = as_scalar_uint(&op.lhs);
        let rhs_bytes = as_scalar_uint(&op.rhs);

        println!(
            "Operations for binary test bits:{} op:{} is_scalar:{} lhs:{} rhs:{}",
            op.bits, op.operator, op.is_scalar, op.lhs, op.rhs
        );
        let caller = zero_address();
        let log = log_with_tx(
            transaction_id,
            tfhe_event(TfheContractEvents::TrivialEncrypt(
                TfheContract::TrivialEncrypt {
                    caller,
                    pt: lhs_bytes,
                    toType: to_ty(op.input_types),
                    result: lhs_handle,
                },
            )),
        );

        let mut tx = harness.listener_db.new_transaction().await?;
        insert_tfhe_event(&harness.listener_db, &mut tx, log, true).await?;
        allow_handle(&harness.listener_db, &mut tx, lhs_handle.as_ref()).await?;
        if !op.is_scalar {
            let log = log_with_tx(
                transaction_id,
                tfhe_event(TfheContractEvents::TrivialEncrypt(
                    TfheContract::TrivialEncrypt {
                        caller,
                        pt: rhs_bytes,
                        toType: to_ty(op.input_types),
                        result: rhs_handle,
                    },
                )),
            );
            insert_tfhe_event(&harness.listener_db, &mut tx, log, true).await?;
            allow_handle(&harness.listener_db, &mut tx, rhs_handle.as_ref()).await?;
        }
        let op_event = binary_op_to_event(&op, &lhs_handle, &rhs_handle, &op.rhs, &output_handle);
        let log = log_with_tx(transaction_id, tfhe_event(op_event));
        insert_tfhe_event(&harness.listener_db, &mut tx, log, true).await?;
        allow_handle(&harness.listener_db, &mut tx, output_handle.as_ref()).await?;
        tx.commit().await?;

        cases.push((op, output_handle));
    }

    wait_until_all_allowed_handles_computed(&harness.app).await?;
    for (op, output_handle) in cases {
        let decrypt_request = vec![output_handle.to_vec()];
        let resp = decrypt_ciphertexts(&harness.pool, decrypt_request).await?;
        let decr_response = &resp[0];
        println!("Checking computation for binary test bits:{} op:{} is_scalar:{} lhs:{} rhs:{} output:{}",
            op.bits, op.operator, op.is_scalar, op.lhs, op.rhs, decr_response.value);
        assert_eq!(
            decr_response.output_type, op.expected_output_type as i16,
            "operand types not equal"
        );
        let value_to_compare = match decr_response.value.as_str() {
            // for FheBool outputs
            "true" => "1",
            "false" => "0",
            other => other,
        };
        assert_eq!(
            value_to_compare,
            op.expected_output.to_string(),
            "operand output values not equal"
        );
    }

    Ok(())
}

fn unary_op_to_event(
    op: &UnaryOperatorTestCase,
    input: &Handle,
    result: &Handle,
) -> TfheContractEvents {
    use fhevm_engine_common::types::SupportedFheOperations as S;
    use host_listener::contracts::TfheContract as C;
    use host_listener::contracts::TfheContract::TfheContractEvents as E;

    let caller = zero_address();
    let input = *input;
    let result = *result;

    match S::try_from(op.operand).unwrap() {
        S::FheNot => E::FheNot(C::FheNot {
            caller,
            ct: input,
            result,
        }),
        S::FheNeg => E::FheNeg(C::FheNeg {
            caller,
            ct: input,
            result,
        }),
        _ => panic!("unknown unary operation: {:?}", op.operand),
    }
}

#[tokio::test]
#[serial(db)]
async fn test_fhe_unary_operands_events() -> Result<(), Box<dyn std::error::Error>> {
    let ops = generate_unary_test_cases();
    let harness = setup_event_harness().await?;

    for op in &ops {
        if !supported_types().contains(&op.operand_types) {
            continue;
        }
        // TrivialEncrypt test setup uses ClearConst (up to 256-bit payloads).
        if op.bits > 256 {
            continue;
        }
        let input_handle = next_handle();
        let output_handle = next_handle();
        let transaction_id = next_handle();

        let inp_bytes = as_scalar_uint(&op.inp);

        println!(
            "Operations for unary test bits:{} op:{} input:{}",
            op.bits, op.operand, op.inp
        );

        let caller = zero_address();
        let log = log_with_tx(
            transaction_id,
            tfhe_event(TfheContractEvents::TrivialEncrypt(
                TfheContract::TrivialEncrypt {
                    caller,
                    pt: inp_bytes,
                    toType: to_ty(op.operand_types),
                    result: input_handle,
                },
            )),
        );

        let mut tx = harness.listener_db.new_transaction().await?;
        insert_tfhe_event(&harness.listener_db, &mut tx, log, true).await?;
        allow_handle(&harness.listener_db, &mut tx, input_handle.as_ref()).await?;

        let op_event = unary_op_to_event(op, &input_handle, &output_handle);
        let log = log_with_tx(transaction_id, tfhe_event(op_event));
        insert_tfhe_event(&harness.listener_db, &mut tx, log, true).await?;
        allow_handle(&harness.listener_db, &mut tx, output_handle.as_ref()).await?;
        tx.commit().await?;
        wait_until_all_allowed_handles_computed(&harness.app).await?;

        let decrypt_request = vec![output_handle.to_vec()];
        let resp = decrypt_ciphertexts(&harness.pool, decrypt_request).await?;
        let decr_response = &resp[0];
        println!(
            "Checking computation for unary test bits:{} op:{} input:{} output:{}",
            op.bits, op.operand, op.inp, decr_response.value
        );
        assert_eq!(
            decr_response.output_type, op.operand_types as i16,
            "operand types not equal"
        );
        let expected_value = if op.bits == 1 {
            op.expected_output.gt(&BigInt::from(0)).to_string()
        } else {
            op.expected_output.to_string()
        };
        assert_eq!(
            decr_response.value, expected_value,
            "operand output values not equal"
        );
    }

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_fhe_if_then_else_events() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;

    let transaction_id = next_handle();
    let fhe_bool_type = 0;
    let false_handle = next_handle();
    let true_handle = next_handle();
    let caller = zero_address();

    let log = log_with_tx(
        transaction_id,
        tfhe_event(TfheContractEvents::TrivialEncrypt(
            TfheContract::TrivialEncrypt {
                caller,
                pt: as_scalar_uint(&BigInt::from(0)),
                toType: to_ty(fhe_bool_type),
                result: false_handle,
            },
        )),
    );
    let mut tx = harness.listener_db.new_transaction().await?;
    insert_tfhe_event(&harness.listener_db, &mut tx, log, true).await?;
    allow_handle(&harness.listener_db, &mut tx, false_handle.as_ref()).await?;

    let log = log_with_tx(
        transaction_id,
        tfhe_event(TfheContractEvents::TrivialEncrypt(
            TfheContract::TrivialEncrypt {
                caller,
                pt: as_scalar_uint(&BigInt::from(1)),
                toType: to_ty(fhe_bool_type),
                result: true_handle,
            },
        )),
    );
    insert_tfhe_event(&harness.listener_db, &mut tx, log, true).await?;
    allow_handle(&harness.listener_db, &mut tx, true_handle.as_ref()).await?;
    tx.commit().await?;

    for input_types in supported_types() {
        let is_input_bool = *input_types == fhe_bool_type;
        let (left_input, right_input) = if is_input_bool {
            (BigInt::from(0), BigInt::from(1))
        } else {
            (BigInt::from(7), BigInt::from(12))
        };

        for test_value in [false, true] {
            let left_handle = next_handle();
            let right_handle = next_handle();
            let transaction_id = next_handle();
            let log = log_with_tx(
                transaction_id,
                tfhe_event(TfheContractEvents::TrivialEncrypt(
                    TfheContract::TrivialEncrypt {
                        caller,
                        pt: as_scalar_uint(&left_input),
                        toType: to_ty(*input_types),
                        result: left_handle,
                    },
                )),
            );
            let mut tx = harness.listener_db.new_transaction().await?;
            insert_tfhe_event(&harness.listener_db, &mut tx, log, true).await?;
            allow_handle(&harness.listener_db, &mut tx, left_handle.as_ref()).await?;

            let log = log_with_tx(
                transaction_id,
                tfhe_event(TfheContractEvents::TrivialEncrypt(
                    TfheContract::TrivialEncrypt {
                        caller,
                        pt: as_scalar_uint(&right_input),
                        toType: to_ty(*input_types),
                        result: right_handle,
                    },
                )),
            );
            insert_tfhe_event(&harness.listener_db, &mut tx, log, true).await?;
            allow_handle(&harness.listener_db, &mut tx, right_handle.as_ref()).await?;

            let output_handle = next_handle();
            let (expected_result, input_handle) = if test_value {
                (&left_input, &true_handle)
            } else {
                (&right_input, &false_handle)
            };
            let expected_result = if *input_types == fhe_bool_type {
                (expected_result > &BigInt::from(0)).to_string()
            } else {
                expected_result.to_string()
            };

            let log = log_with_tx(
                transaction_id,
                tfhe_event(TfheContractEvents::FheIfThenElse(
                    TfheContract::FheIfThenElse {
                        caller,
                        control: *input_handle,
                        ifTrue: left_handle,
                        ifFalse: right_handle,
                        result: output_handle,
                    },
                )),
            );
            insert_tfhe_event(&harness.listener_db, &mut tx, log, true).await?;
            allow_handle(&harness.listener_db, &mut tx, output_handle.as_ref()).await?;
            tx.commit().await?;
            wait_until_all_allowed_handles_computed(&harness.app).await?;
            let decrypt_request = vec![output_handle.to_vec()];
            let resp = decrypt_ciphertexts(&harness.pool, decrypt_request).await?;
            let decr_response = &resp[0];
            println!(
                "Checking if then else computation for test type:{} control:{} lhs:{} rhs:{} output:{}",
                *input_types, test_value, left_input, right_input, decr_response.value
            );
            assert_eq!(
                decr_response.output_type, *input_types as i16,
                "operand types not equal"
            );
            assert_eq!(
                decr_response.value.to_string(),
                expected_result,
                "operand output values not equal"
            );
        }
    }
    wait_until_all_allowed_handles_computed(&harness.app).await?;

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_fhe_cast_events() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;

    let caller = zero_address();

    let fhe_bool = 0;
    for type_from in supported_types() {
        for type_to in supported_types() {
            let input_handle = next_handle();
            let output_handle = next_handle();
            let transaction_id = next_handle();
            let input = 7;
            let output = if *type_to == fhe_bool || *type_from == fhe_bool {
                // if bool output is 1
                1
            } else {
                input
            };

            println!(
                "Encrypting inputs for cast test type from:{type_from} type to:{type_to} input:{input} output:{output}",
            );

            let log = log_with_tx(
                transaction_id,
                tfhe_event(TfheContractEvents::TrivialEncrypt(
                    TfheContract::TrivialEncrypt {
                        caller,
                        pt: as_scalar_uint(&BigInt::from(input)),
                        toType: to_ty(*type_from),
                        result: input_handle,
                    },
                )),
            );

            let mut tx = harness.listener_db.new_transaction().await?;
            insert_tfhe_event(&harness.listener_db, &mut tx, log, true).await?;
            allow_handle(&harness.listener_db, &mut tx, input_handle.as_ref()).await?;

            let log = log_with_tx(
                transaction_id,
                tfhe_event(TfheContractEvents::Cast(TfheContract::Cast {
                    caller,
                    ct: input_handle,
                    toType: to_ty(*type_to),
                    result: output_handle,
                })),
            );
            insert_tfhe_event(&harness.listener_db, &mut tx, log, true).await?;
            allow_handle(&harness.listener_db, &mut tx, output_handle.as_ref()).await?;
            tx.commit().await?;

            wait_until_all_allowed_handles_computed(&harness.app).await?;
            let decrypt_request = vec![output_handle.to_vec()];
            let resp = decrypt_ciphertexts(&harness.pool, decrypt_request).await?;
            let decr_response = &resp[0];

            println!(
                "Checking computation for cast test from:{} to:{} input:{} output:{}",
                type_from, type_to, input, decr_response.value,
            );

            assert_eq!(
                decr_response.output_type, *type_to as i16,
                "operand types not equal"
            );
            assert_eq!(
                decr_response.value.to_string(),
                if *type_to == fhe_bool {
                    (output > 0).to_string()
                } else {
                    output.to_string()
                },
                "operand output values not equal"
            );
        }
    }

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_op_trivial_encrypt() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;

    let tx_id = next_handle();
    let output = next_handle();

    let mut tx = harness.listener_db.new_transaction().await?;
    let log = log_with_tx(
        tx_id,
        tfhe_event(TfheContractEvents::TrivialEncrypt(
            TfheContract::TrivialEncrypt {
                caller: zero_address(),
                pt: as_scalar_uint(&BigInt::from(123)),
                toType: to_ty(5),
                result: output,
            },
        )),
    );
    insert_tfhe_event(&harness.listener_db, &mut tx, log, true).await?;
    allow_handle(&harness.listener_db, &mut tx, output.as_ref()).await?;
    tx.commit().await?;

    wait_until_all_allowed_handles_computed(&harness.app).await?;
    let decrypted = decrypt_ciphertexts(&harness.pool, vec![output.to_vec()]).await?;
    assert_eq!(decrypted.len(), 1);
    assert_eq!(decrypted[0].output_type, 5);
    assert_eq!(decrypted[0].value, "123");

    Ok(())
}

#[tokio::test]
#[serial(db)]
pub(super) async fn test_fhe_rand_events() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;

    for &rand_type in supported_types() {
        let output1_handle = next_handle();
        let output2_handle = next_handle();
        let output3_handle = next_handle();
        let transaction_id = next_handle();

        let caller = zero_address();
        let log = log_with_tx(
            transaction_id,
            tfhe_event(TfheContractEvents::FheRand(TfheContract::FheRand {
                caller,
                randType: to_ty(rand_type),
                seed: FixedBytes::from([0u8; 16]),
                result: output1_handle,
            })),
        );

        let mut tx = harness.listener_db.new_transaction().await?;
        insert_tfhe_event(&harness.listener_db, &mut tx, log, true).await?;
        allow_handle(&harness.listener_db, &mut tx, output1_handle.as_ref()).await?;

        let log = log_with_tx(
            transaction_id,
            tfhe_event(TfheContractEvents::FheRand(TfheContract::FheRand {
                caller,
                randType: to_ty(rand_type),
                seed: FixedBytes::from([
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ]),
                result: output2_handle,
            })),
        );
        insert_tfhe_event(&harness.listener_db, &mut tx, log, true).await?;
        allow_handle(&harness.listener_db, &mut tx, output2_handle.as_ref()).await?;

        let log = log_with_tx(
            transaction_id,
            tfhe_event(TfheContractEvents::FheRandBounded(
                TfheContract::FheRandBounded {
                    caller,
                    upperBound: as_scalar_uint(&BigInt::from(1)),
                    randType: to_ty(rand_type),
                    seed: FixedBytes::from([
                        1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                        0u8,
                    ]),
                    result: output3_handle,
                },
            )),
        );
        insert_tfhe_event(&harness.listener_db, &mut tx, log, true).await?;
        allow_handle(&harness.listener_db, &mut tx, output3_handle.as_ref()).await?;
        tx.commit().await?;

        wait_until_all_allowed_handles_computed(&harness.app).await?;

        let decrypt_request = vec![
            output1_handle.to_vec(),
            output2_handle.to_vec(),
            output3_handle.to_vec(),
        ];
        let resp = decrypt_ciphertexts(&harness.pool, decrypt_request).await?;
        assert_eq!(resp[0].output_type, rand_type as i16);
        assert_eq!(resp[1].output_type, rand_type as i16);
        assert_eq!(resp[2].output_type, rand_type as i16);
        if rand_type != 0 {
            assert_eq!(resp[2].value, "0");
        }
    }

    Ok(())
}
