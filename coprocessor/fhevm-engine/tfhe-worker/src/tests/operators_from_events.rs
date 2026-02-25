use bigdecimal::num_bigint::BigInt;
use serial_test::serial;

use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::Handle;

use crate::tests::event_helpers::{
    allow_handle, as_scalar_uint, insert_event, next_handle, setup_event_harness, to_ty,
    zero_address, EventHarness,
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

fn as_scalar_handle(big_int: &BigInt) -> Handle {
    let (_, mut bytes) = big_int.to_bytes_le();
    while bytes.len() < 32 {
        bytes.push(0_u8)
    }
    bytes.reverse();
    Handle::from_slice(&bytes)
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
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;
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

        let mut tx = listener_db.new_transaction().await?;
        insert_event(
            &listener_db,
            &mut tx,
            transaction_id,
            TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                caller,
                pt: lhs_bytes,
                toType: to_ty(op.input_types),
                result: lhs_handle,
            }),
            true,
        )
        .await?;
        allow_handle(&listener_db, &mut tx, &lhs_handle).await?;
        if !op.is_scalar {
            insert_event(
                &listener_db,
                &mut tx,
                transaction_id,
                TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                    caller,
                    pt: rhs_bytes,
                    toType: to_ty(op.input_types),
                    result: rhs_handle,
                }),
                true,
            )
            .await?;
            allow_handle(&listener_db, &mut tx, &rhs_handle).await?;
        }
        let op_event = binary_op_to_event(&op, &lhs_handle, &rhs_handle, &op.rhs, &output_handle);
        insert_event(&listener_db, &mut tx, transaction_id, op_event, true).await?;
        allow_handle(&listener_db, &mut tx, &output_handle).await?;
        tx.commit().await?;

        cases.push((op, output_handle));
    }

    wait_until_all_allowed_handles_computed(&app).await?;
    for (op, output_handle) in cases {
        let decrypt_request = vec![output_handle.to_vec()];
        let resp = decrypt_ciphertexts(&pool, decrypt_request).await?;
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
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let mut cases = vec![];
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

        let mut tx = listener_db.new_transaction().await?;
        insert_event(
            &listener_db,
            &mut tx,
            transaction_id,
            TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                caller,
                pt: inp_bytes,
                toType: to_ty(op.operand_types),
                result: input_handle,
            }),
            true,
        )
        .await?;
        allow_handle(&listener_db, &mut tx, &input_handle).await?;

        let op_event = unary_op_to_event(op, &input_handle, &output_handle);
        insert_event(&listener_db, &mut tx, transaction_id, op_event, true).await?;
        allow_handle(&listener_db, &mut tx, &output_handle).await?;
        tx.commit().await?;

        cases.push((op, output_handle));
    }

    wait_until_all_allowed_handles_computed(&app).await?;
    for (op, output_handle) in cases {
        let decrypt_request = vec![output_handle.to_vec()];
        let resp = decrypt_ciphertexts(&pool, decrypt_request).await?;
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
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let transaction_id = next_handle();
    let fhe_bool_type = 0;
    let false_handle = next_handle();
    let true_handle = next_handle();
    let caller = zero_address();

    let mut tx = listener_db.new_transaction().await?;
    insert_event(
        &listener_db,
        &mut tx,
        transaction_id,
        TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
            caller,
            pt: as_scalar_uint(&BigInt::from(0)),
            toType: to_ty(fhe_bool_type),
            result: false_handle,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &false_handle).await?;

    insert_event(
        &listener_db,
        &mut tx,
        transaction_id,
        TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
            caller,
            pt: as_scalar_uint(&BigInt::from(1)),
            toType: to_ty(fhe_bool_type),
            result: true_handle,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &true_handle).await?;
    tx.commit().await?;

    let mut cases = vec![];
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
            let mut tx = listener_db.new_transaction().await?;
            insert_event(
                &listener_db,
                &mut tx,
                transaction_id,
                TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                    caller,
                    pt: as_scalar_uint(&left_input),
                    toType: to_ty(*input_types),
                    result: left_handle,
                }),
                true,
            )
            .await?;
            allow_handle(&listener_db, &mut tx, &left_handle).await?;

            insert_event(
                &listener_db,
                &mut tx,
                transaction_id,
                TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                    caller,
                    pt: as_scalar_uint(&right_input),
                    toType: to_ty(*input_types),
                    result: right_handle,
                }),
                true,
            )
            .await?;
            allow_handle(&listener_db, &mut tx, &right_handle).await?;

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

            insert_event(
                &listener_db,
                &mut tx,
                transaction_id,
                TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                    caller,
                    control: *input_handle,
                    ifTrue: left_handle,
                    ifFalse: right_handle,
                    result: output_handle,
                }),
                true,
            )
            .await?;
            allow_handle(&listener_db, &mut tx, &output_handle).await?;
            tx.commit().await?;

            cases.push((output_handle, *input_types, expected_result));
        }
    }

    wait_until_all_allowed_handles_computed(&app).await?;
    for (output_handle, expected_type, expected_result) in cases {
        let decrypt_request = vec![output_handle.to_vec()];
        let resp = decrypt_ciphertexts(&pool, decrypt_request).await?;
        let decr_response = &resp[0];
        println!(
            "Checking if then else computation for type:{} output:{}",
            expected_type, decr_response.value
        );
        assert_eq!(
            decr_response.output_type, expected_type as i16,
            "operand types not equal"
        );
        assert_eq!(
            decr_response.value.to_string(),
            expected_result,
            "operand output values not equal"
        );
    }

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_fhe_cast_events() -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let caller = zero_address();

    let fhe_bool = 0;
    let mut cases = vec![];
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

            let mut tx = listener_db.new_transaction().await?;
            insert_event(
                &listener_db,
                &mut tx,
                transaction_id,
                TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                    caller,
                    pt: as_scalar_uint(&BigInt::from(input)),
                    toType: to_ty(*type_from),
                    result: input_handle,
                }),
                true,
            )
            .await?;
            allow_handle(&listener_db, &mut tx, &input_handle).await?;

            insert_event(
                &listener_db,
                &mut tx,
                transaction_id,
                TfheContractEvents::Cast(TfheContract::Cast {
                    caller,
                    ct: input_handle,
                    toType: to_ty(*type_to),
                    result: output_handle,
                }),
                true,
            )
            .await?;
            allow_handle(&listener_db, &mut tx, &output_handle).await?;
            tx.commit().await?;

            cases.push((*type_from, *type_to, input, output, output_handle));
        }
    }

    wait_until_all_allowed_handles_computed(&app).await?;
    for (type_from, type_to, input, output, output_handle) in cases {
        let decrypt_request = vec![output_handle.to_vec()];
        let resp = decrypt_ciphertexts(&pool, decrypt_request).await?;
        let decr_response = &resp[0];

        println!(
            "Checking computation for cast test from:{} to:{} input:{} output:{}",
            type_from, type_to, input, decr_response.value,
        );

        assert_eq!(
            decr_response.output_type, type_to as i16,
            "operand types not equal"
        );
        assert_eq!(
            decr_response.value.to_string(),
            if type_to == fhe_bool {
                (output > 0).to_string()
            } else {
                output.to_string()
            },
            "operand output values not equal"
        );
    }

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_op_trivial_encrypt() -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    fn bits_for_type(ty: i32) -> u32 {
        match ty {
            0 => 1,
            1 => 4,
            2 => 8,
            3 => 16,
            4 => 32,
            5 => 64,
            6 => 128,
            7 => 160,
            8 => 256,
            9 => 512,
            10 => 1024,
            11 => 2048,
            _ => panic!("unknown type {ty}"),
        }
    }

    let mut cases: Vec<(Handle, i32, BigInt)> = vec![];
    let mut tx = listener_db.new_transaction().await?;
    let tx_id = next_handle();
    for &fhe_type in supported_types() {
        let bits = bits_for_type(fhe_type);
        let value = if fhe_type == 0 {
            BigInt::from(1)
        } else if bits <= 256 {
            BigInt::from(1) << (bits - 1)
        } else {
            // Types 9-11 (>256-bit): max ClearConst can represent is 256 bits.
            BigInt::from(1) << 255
        };

        let output = next_handle();
        insert_event(
            &listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                caller: zero_address(),
                pt: as_scalar_uint(&value),
                toType: to_ty(fhe_type),
                result: output,
            }),
            true,
        )
        .await?;
        allow_handle(&listener_db, &mut tx, &output).await?;
        cases.push((output, fhe_type, value));
    }
    tx.commit().await?;

    wait_until_all_allowed_handles_computed(&app).await?;

    for (output, fhe_type, value) in &cases {
        let decrypted = decrypt_ciphertexts(&pool, vec![output.to_vec()]).await?;
        assert_eq!(decrypted.len(), 1);
        assert_eq!(
            decrypted[0].output_type, *fhe_type as i16,
            "type mismatch for fhe_type={fhe_type}"
        );
        let expected = if *fhe_type == 0 {
            // Bool decrypts as "true"/"false"
            (value > &BigInt::from(0)).to_string()
        } else {
            value.to_string()
        };
        assert_eq!(
            decrypted[0].value, expected,
            "value mismatch for fhe_type={fhe_type}"
        );
    }

    Ok(())
}
