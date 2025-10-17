use alloy::primitives::{FixedBytes, Log};
use bigdecimal::num_bigint::BigInt;

use fhevm_engine_common::types::AllowEvents;
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{
    ClearConst, Database as ListenerDatabase, Handle, LogTfhe, ToType, Transaction,
};

use crate::tests::operators::{generate_binary_test_cases, generate_unary_test_cases};
use crate::tests::utils::{decrypt_ciphertexts, wait_until_all_allowed_handles_computed};
use crate::tests::utils::{default_api_key, setup_test_app, TestInstance};

use crate::tests::operators::BinaryOperatorTestCase;
use crate::tests::operators::UnaryOperatorTestCase;

use super::utils::default_dependence_cache_size;

pub fn supported_types() -> &'static [i32] {
    &[
        0, // bool
        8, // 256 bit
    ]
}

fn tfhe_event(data: TfheContractEvents) -> Log<TfheContractEvents> {
    let address = "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap();
    Log::<TfheContractEvents> { address, data }
}

async fn insert_tfhe_event(
    db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    log: alloy::rpc::types::Log<TfheContractEvents>,
    is_allowed: bool,
) -> Result<(), sqlx::Error> {
    let event = LogTfhe {
        event: log.inner,
        transaction_hash: log.transaction_hash,
        is_allowed,
        block_number: log.block_number.unwrap_or(0),
    };
    db.insert_tfhe_event(tx, &event).await
}

pub async fn allow_handle(
    db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    handle: &[u8],
) -> Result<(), sqlx::Error> {
    let account_address = String::new();
    let event_type = AllowEvents::AllowedForDecryption;
    db.insert_allowed_handle(tx, handle.to_owned(), account_address, event_type, None)
        .await
}

fn as_handle(big_int: &BigInt) -> Handle {
    let (_, bytes) = big_int.to_bytes_be();
    Handle::right_padding_from(&bytes)
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

fn to_ty(ty: i32) -> ToType {
    ToType::from(ty as u8)
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
    let caller = "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap();
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

fn next_handle() -> Handle {
    #[expect(non_upper_case_globals)]
    static count: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let v = count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    as_handle(&BigInt::from(v))
}

async fn listener_event_to_db(app: &TestInstance) -> ListenerDatabase {
    let coprocessor_api_key = sqlx::types::Uuid::parse_str(default_api_key()).unwrap();
    let url = app.db_url().to_string();
    ListenerDatabase::new(&url, &coprocessor_api_key, default_dependence_cache_size())
        .await
        .unwrap()
}

#[tokio::test]
async fn test_fhe_binary_operands_events() -> Result<(), Box<dyn std::error::Error>> {
    use fhevm_engine_common::types::SupportedFheOperations as S;
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let listener_event_to_db = listener_event_to_db(&app).await;
    let mut cases = vec![];
    for op in generate_binary_test_cases() {
        if !supported_types().contains(&op.input_types) {
            continue;
        }
        let support_bytes = matches!(S::try_from(op.operator).unwrap(), S::FheEq | S::FheNe);
        if op.bits > 256 && op.is_scalar && !support_bytes {
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
        let caller = "0x0000000000000000000000000000000000000000"
            .parse()
            .unwrap();
        let log = alloy::rpc::types::Log {
            inner: tfhe_event(TfheContractEvents::TrivialEncrypt(
                TfheContract::TrivialEncrypt {
                    caller,
                    pt: lhs_bytes,
                    toType: to_ty(op.input_types),
                    result: lhs_handle,
                },
            )),
            block_hash: None,
            block_number: None,
            block_timestamp: None,
            transaction_hash: Some(transaction_id),
            transaction_index: Some(0),
            log_index: None,
            removed: false,
        };

        let mut tx = listener_event_to_db.new_transaction().await?;
        insert_tfhe_event(&listener_event_to_db, &mut tx, log, false).await?;
        if !op.is_scalar {
            let log = alloy::rpc::types::Log {
                inner: tfhe_event(TfheContractEvents::TrivialEncrypt(
                    TfheContract::TrivialEncrypt {
                        caller,
                        pt: rhs_bytes,
                        toType: to_ty(op.input_types),
                        result: rhs_handle,
                    },
                )),
                block_hash: None,
                block_number: None,
                block_timestamp: None,
                transaction_hash: Some(transaction_id),
                transaction_index: Some(0),
                log_index: None,
                removed: false,
            };
            insert_tfhe_event(&listener_event_to_db, &mut tx, log, false).await?;
        }
        let op_event = binary_op_to_event(&op, &lhs_handle, &rhs_handle, &op.rhs, &output_handle);
        eprintln!("op_event: {:?}", &op_event);
        let log = alloy::rpc::types::Log {
            inner: tfhe_event(op_event),
            block_hash: None,
            block_number: None,
            block_timestamp: None,
            transaction_hash: Some(transaction_id),
            transaction_index: Some(0),
            log_index: None,
            removed: false,
        };
        insert_tfhe_event(&listener_event_to_db, &mut tx, log, true).await?;
        allow_handle(&listener_event_to_db, &mut tx, output_handle.as_ref()).await?;
        tx.commit().await?;

        cases.push((op, output_handle));
    }

    wait_until_all_allowed_handles_computed(&app).await?;
    for (op, output_handle) in cases {
        let decrypt_request = vec![output_handle.to_vec()];
        let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;
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

    let caller = "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap();
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
async fn test_fhe_unary_operands_events() -> Result<(), Box<dyn std::error::Error>> {
    let ops = generate_unary_test_cases();
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let listener_event_to_db = listener_event_to_db(&app).await;

    for op in &ops {
        if !supported_types().contains(&op.operand_types) {
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

        let caller = "0x0000000000000000000000000000000000000000"
            .parse()
            .unwrap();
        let log = alloy::rpc::types::Log {
            inner: tfhe_event(TfheContractEvents::TrivialEncrypt(
                TfheContract::TrivialEncrypt {
                    caller,
                    pt: inp_bytes,
                    toType: to_ty(op.operand_types),
                    result: input_handle,
                },
            )),
            block_hash: None,
            block_number: None,
            block_timestamp: None,
            transaction_hash: Some(transaction_id),
            transaction_index: Some(0),
            log_index: None,
            removed: false,
        };

        let mut tx = listener_event_to_db.new_transaction().await?;
        insert_tfhe_event(&listener_event_to_db, &mut tx, log, false).await?;

        let op_event = unary_op_to_event(op, &input_handle, &output_handle);
        eprintln!("op_event: {:?}", &op_event);
        let log = alloy::rpc::types::Log {
            inner: tfhe_event(op_event),
            block_hash: None,
            block_number: None,
            block_timestamp: None,
            transaction_hash: Some(transaction_id),
            transaction_index: Some(0),
            log_index: None,
            removed: false,
        };
        insert_tfhe_event(&listener_event_to_db, &mut tx, log, true).await?;
        allow_handle(&listener_event_to_db, &mut tx, output_handle.as_ref()).await?;
        tx.commit().await?;
        wait_until_all_allowed_handles_computed(&app).await?;

        let decrypt_request = vec![output_handle.to_vec()];
        let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;
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
async fn test_fhe_if_then_else_events() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let listener_event_to_db = listener_event_to_db(&app).await;

    let transaction_id = next_handle();
    let fhe_bool_type = 0;
    let false_handle = next_handle();
    let true_handle = next_handle();
    let caller = "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap();

    let log = alloy::rpc::types::Log {
        inner: tfhe_event(TfheContractEvents::TrivialEncrypt(
            TfheContract::TrivialEncrypt {
                caller,
                pt: as_scalar_uint(&BigInt::from(0)),
                toType: to_ty(fhe_bool_type),
                result: false_handle,
            },
        )),
        block_hash: None,
        block_number: None,
        block_timestamp: None,
        transaction_hash: Some(transaction_id),
        transaction_index: Some(0),
        log_index: None,
        removed: false,
    };
    let mut tx = listener_event_to_db.new_transaction().await?;
    insert_tfhe_event(&listener_event_to_db, &mut tx, log, true).await?;
    allow_handle(&listener_event_to_db, &mut tx, false_handle.as_ref()).await?;

    let log = alloy::rpc::types::Log {
        inner: tfhe_event(TfheContractEvents::TrivialEncrypt(
            TfheContract::TrivialEncrypt {
                caller,
                pt: as_scalar_uint(&BigInt::from(1)),
                toType: to_ty(fhe_bool_type),
                result: true_handle,
            },
        )),
        block_hash: None,
        block_number: None,
        block_timestamp: None,
        transaction_hash: Some(transaction_id),
        transaction_index: Some(0),
        log_index: None,
        removed: false,
    };
    insert_tfhe_event(&listener_event_to_db, &mut tx, log, true).await?;
    allow_handle(&listener_event_to_db, &mut tx, true_handle.as_ref()).await?;
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
            let log = alloy::rpc::types::Log {
                inner: tfhe_event(TfheContractEvents::TrivialEncrypt(
                    TfheContract::TrivialEncrypt {
                        caller,
                        pt: as_scalar_uint(&left_input),
                        toType: to_ty(*input_types),
                        result: left_handle,
                    },
                )),
                block_hash: None,
                block_number: None,
                block_timestamp: None,
                transaction_hash: Some(transaction_id),
                transaction_index: Some(0),
                log_index: None,
                removed: false,
            };
            let mut tx = listener_event_to_db.new_transaction().await?;
            insert_tfhe_event(&listener_event_to_db, &mut tx, log, false).await?;

            let log = alloy::rpc::types::Log {
                inner: tfhe_event(TfheContractEvents::TrivialEncrypt(
                    TfheContract::TrivialEncrypt {
                        caller,
                        pt: as_scalar_uint(&right_input),
                        toType: to_ty(*input_types),
                        result: right_handle,
                    },
                )),
                block_hash: None,
                block_number: None,
                block_timestamp: None,
                transaction_hash: Some(transaction_id),
                transaction_index: Some(0),
                log_index: None,
                removed: false,
            };
            insert_tfhe_event(&listener_event_to_db, &mut tx, log, false).await?;

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

            let log = alloy::rpc::types::Log {
                inner: tfhe_event(TfheContractEvents::FheIfThenElse(
                    TfheContract::FheIfThenElse {
                        caller,
                        control: *input_handle,
                        ifTrue: left_handle,
                        ifFalse: right_handle,
                        result: output_handle,
                    },
                )),
                block_hash: None,
                block_number: None,
                block_timestamp: None,
                transaction_hash: Some(transaction_id),
                transaction_index: Some(0),
                log_index: None,
                removed: false,
            };
            insert_tfhe_event(&listener_event_to_db, &mut tx, log, true).await?;
            allow_handle(&listener_event_to_db, &mut tx, output_handle.as_ref()).await?;
            tx.commit().await?;
            wait_until_all_allowed_handles_computed(&app).await?;
            let decrypt_request = vec![output_handle.to_vec()];
            let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;
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
    wait_until_all_allowed_handles_computed(&app).await?;

    Ok(())
}

#[tokio::test]
async fn test_fhe_cast_events() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let listener_event_to_db = listener_event_to_db(&app).await;

    let caller = "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap();

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

            let log = alloy::rpc::types::Log {
                inner: tfhe_event(TfheContractEvents::TrivialEncrypt(
                    TfheContract::TrivialEncrypt {
                        caller,
                        pt: as_scalar_uint(&BigInt::from(input)),
                        toType: to_ty(*type_from),
                        result: input_handle,
                    },
                )),
                block_hash: None,
                block_number: None,
                block_timestamp: None,
                transaction_hash: Some(transaction_id),
                transaction_index: Some(0),
                log_index: None,
                removed: false,
            };

            let mut tx = listener_event_to_db.new_transaction().await?;
            insert_tfhe_event(&listener_event_to_db, &mut tx, log, false).await?;

            let log = alloy::rpc::types::Log {
                inner: tfhe_event(TfheContractEvents::Cast(TfheContract::Cast {
                    caller,
                    ct: input_handle,
                    toType: to_ty(*type_to),
                    result: output_handle,
                })),
                block_hash: None,
                block_number: None,
                block_timestamp: None,
                transaction_hash: Some(transaction_id),
                transaction_index: Some(0),
                log_index: None,
                removed: false,
            };
            insert_tfhe_event(&listener_event_to_db, &mut tx, log, true).await?;
            allow_handle(&listener_event_to_db, &mut tx, output_handle.as_ref()).await?;
            tx.commit().await?;

            wait_until_all_allowed_handles_computed(&app).await?;
            let decrypt_request = vec![output_handle.to_vec()];
            let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;
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
async fn test_fhe_rand_events() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let listener_event_to_db = listener_event_to_db(&app).await;

    for &rand_type in supported_types() {
        let output1_handle = next_handle();
        let output2_handle = next_handle();
        let output3_handle = next_handle();
        let transaction_id = next_handle();

        let caller = "0x0000000000000000000000000000000000000000"
            .parse()
            .unwrap();
        let log = alloy::rpc::types::Log {
            inner: tfhe_event(TfheContractEvents::FheRand(TfheContract::FheRand {
                caller,
                randType: to_ty(rand_type),
                seed: FixedBytes::from([0u8; 16]),
                result: output1_handle,
            })),
            block_hash: None,
            block_number: None,
            block_timestamp: None,
            transaction_hash: Some(transaction_id),
            transaction_index: Some(0),
            log_index: None,
            removed: false,
        };

        let mut tx = listener_event_to_db.new_transaction().await?;
        insert_tfhe_event(&listener_event_to_db, &mut tx, log, true).await?;
        allow_handle(&listener_event_to_db, &mut tx, output1_handle.as_ref()).await?;

        let log = alloy::rpc::types::Log {
            inner: tfhe_event(TfheContractEvents::FheRand(TfheContract::FheRand {
                caller,
                randType: to_ty(rand_type),
                seed: FixedBytes::from([
                    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ]),
                result: output2_handle,
            })),
            block_hash: None,
            block_number: None,
            block_timestamp: None,
            transaction_hash: Some(transaction_id),
            transaction_index: Some(0),
            log_index: None,
            removed: false,
        };
        insert_tfhe_event(&listener_event_to_db, &mut tx, log, true).await?;
        allow_handle(&listener_event_to_db, &mut tx, output2_handle.as_ref()).await?;

        let log = alloy::rpc::types::Log {
            inner: tfhe_event(TfheContractEvents::FheRandBounded(
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
            block_hash: None,
            block_number: None,
            block_timestamp: None,
            transaction_hash: Some(transaction_id),
            transaction_index: Some(0),
            log_index: None,
            removed: false,
        };
        insert_tfhe_event(&listener_event_to_db, &mut tx, log, true).await?;
        allow_handle(&listener_event_to_db, &mut tx, output3_handle.as_ref()).await?;
        tx.commit().await?;

        wait_until_all_allowed_handles_computed(&app).await?;

        let decrypt_request = vec![
            output1_handle.to_vec(),
            output2_handle.to_vec(),
            output3_handle.to_vec(),
        ];
        let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;
        assert_eq!(resp[0].output_type, rand_type as i16);
        assert_eq!(resp[1].output_type, rand_type as i16);
        assert_eq!(resp[2].output_type, rand_type as i16);
        if rand_type != 0 {
            assert_eq!(resp[2].value, "0");
        }
    }

    Ok(())
}
