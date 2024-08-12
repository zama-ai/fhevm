use bigdecimal::num_bigint::BigInt;
use strum::IntoEnumIterator;
use tonic::metadata::MetadataValue;
use std::{ops::Not, str::FromStr};
use crate::{tests::utils::{setup_test_app, default_api_key}, tfhe_ops::{does_fhe_operation_support_both_encrypted_operands, does_fhe_operation_support_scalar}, types::{FheOperationType, SupportedFheOperations}};
use crate::server::coprocessor::fhevm_coprocessor_client::FhevmCoprocessorClient;
use crate::server::coprocessor::{AsyncComputation, AsyncComputeRequest, DebugDecryptRequest, DebugEncryptRequest, DebugEncryptRequestSingle};


struct BinaryOperatorTestCase {
    bits: i32,
    operand: i32,
    operand_types: i32,
    lhs: BigInt,
    rhs: BigInt,
    expected_output: BigInt,
    is_scalar: bool,
}

struct UnaryOperatorTestCase {
    bits: i32,
    inp: BigInt,
    operand: i32,
    operand_types: i32,
    expected_output: BigInt,
}

fn supported_bits() -> &'static [i32] {
    &[
        8,
        16,
        32,
    ]
}

fn supported_bits_to_bit_type_in_db(inp: i32) -> i32 {
    match inp {
        8 => 2,
        16 => 3,
        32 => 4,
        other => panic!("unknown supported bits: {other}")
    }
}

#[tokio::test]
async fn test_fhe_binary_operands() -> Result<(), Box<dyn std::error::Error>> {
    let ops = generate_binary_test_cases();
    let app = setup_test_app().await?;
    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;
    // needed for polling status
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;

    let mut handle_counter = 0;
    let mut next_handle = || {
        let out = handle_counter;
        handle_counter += 1;
        format!("{:#08x}", out)
    };

    let api_key_header = format!("bearer {}", default_api_key());

    let mut output_handles = Vec::with_capacity(ops.len());
    let mut enc_request_payload = Vec::with_capacity(ops.len() * 2);
    let mut async_computations = Vec::with_capacity(ops.len());
    for op in &ops {
        let lhs_handle = next_handle();
        let rhs_handle = if op.is_scalar {
            let (_, bytes) = op.rhs.to_bytes_be();
            format!("0x{}", hex::encode(bytes))
        } else { next_handle() };
        let output_handle = next_handle();
        output_handles.push(output_handle.clone());

        let (_, lhs_bytes) = op.lhs.to_bytes_le();

        println!("Encrypting inputs for binary test bits:{} op:{} is_scalar:{} lhs:{} rhs:{}",
            op.bits, op.operand, op.is_scalar, op.lhs.to_string(), op.rhs.to_string());
        enc_request_payload.push(
            DebugEncryptRequestSingle {
                handle: lhs_handle.clone(),
                le_value: lhs_bytes,
                output_type: op.operand_types,
            }
        );
        if !op.is_scalar {
            let (_, rhs_bytes) = op.rhs.to_bytes_le();
            enc_request_payload.push(DebugEncryptRequestSingle {
                handle: rhs_handle.clone(),
                le_value: rhs_bytes,
                output_type: op.operand_types,
            });
        }

        println!("rhs handle:{}", rhs_handle);
        println!("Scheduling computation for binary test bits:{} op:{} is_scalar:{} lhs:{} rhs:{} output:{}",
            op.bits, op.operand, op.is_scalar, op.lhs.to_string(), op.rhs.to_string(), op.expected_output.to_string());
        async_computations.push(AsyncComputation {
            operation: op.operand,
            is_scalar: op.is_scalar,
            output_handle: output_handle,
            input_handles: vec![
                lhs_handle.clone(),
                rhs_handle.clone(),
            ]
        });
    }

    println!("Encrypting inputs...");
    let mut encrypt_request = tonic::Request::new(DebugEncryptRequest {
        values: enc_request_payload,
    });
    encrypt_request.metadata_mut().append("authorization", MetadataValue::from_str(&api_key_header).unwrap());
    let _resp = client.debug_encrypt_ciphertext(encrypt_request).await?;

    println!("Scheduling computations...");
    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations
    });
    compute_request.metadata_mut().append("authorization", MetadataValue::from_str(&api_key_header).unwrap());
    let _resp = client.async_compute(compute_request).await?;

    println!("Computations scheduled, waiting upon completion...");

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        let count =
            sqlx::query!("SELECT count(*) FROM computations WHERE NOT is_completed AND NOT is_error")
                .fetch_one(&pool)
                .await?;
        let current_count = count.count.unwrap();
        if current_count == 0 {
            println!("All computations completed");
            break;
        } else {
            println!("{current_count} computations remaining, waiting...");
        }
    }

    let mut decrypt_request = tonic::Request::new(DebugDecryptRequest {
        handles: output_handles.clone(),
    });
    decrypt_request.metadata_mut().append("authorization", MetadataValue::from_str(&api_key_header).unwrap());
    let resp = client.debug_decrypt_ciphertext(decrypt_request).await?;

    assert_eq!(resp.get_ref().values.len(), output_handles.len(), "Outputs length doesn't match");
    for (idx, op) in ops.iter().enumerate() {
        let decr_response = &resp.get_ref().values[idx];
        println!("Checking computation for binary test bits:{} op:{} is_scalar:{} lhs:{} rhs:{} output:{}",
            op.bits, op.operand, op.is_scalar, op.lhs.to_string(), op.rhs.to_string(), op.expected_output.to_string());
        assert_eq!(decr_response.output_type, op.operand_types, "operand types not equal");
        assert_eq!(decr_response.value, op.expected_output.to_string(), "operand output values not equal");
    }

    Ok(())
}

#[tokio::test]
async fn test_fhe_unary_operands() -> Result<(), Box<dyn std::error::Error>> {
    let ops = generate_unary_test_cases();
    let app = setup_test_app().await?;
    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;
    // needed for polling status
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;

    let mut handle_counter = 0;
    let mut next_handle = || {
        let out = handle_counter;
        handle_counter += 1;
        format!("{:#08x}", out)
    };

    let api_key_header = format!("bearer {}", default_api_key());

    let mut output_handles = Vec::with_capacity(ops.len());
    let mut enc_request_payload = Vec::with_capacity(ops.len() * 2);
    let mut async_computations = Vec::with_capacity(ops.len());
    for op in &ops {
        let input_handle = next_handle();
        let output_handle = next_handle();
        output_handles.push(output_handle.clone());

        let (_, inp_bytes) = op.inp.to_bytes_le();

        println!("Encrypting inputs for unary test bits:{} op:{} input:{}",
            op.bits, op.operand, op.inp.to_string());
        enc_request_payload.push(
            DebugEncryptRequestSingle {
                handle: input_handle.clone(),
                le_value: inp_bytes,
                output_type: op.operand_types,
            }
        );

        println!("Scheduling computation for binary test bits:{} op:{} input:{} output:{}",
            op.bits, op.operand, op.inp.to_string(), op.expected_output.to_string());
        async_computations.push(AsyncComputation {
            operation: op.operand,
            is_scalar: false,
            output_handle: output_handle,
            input_handles: vec![
                input_handle.clone(),
            ]
        });
    }

    println!("Encrypting inputs...");
    let mut encrypt_request = tonic::Request::new(DebugEncryptRequest {
        values: enc_request_payload,
    });
    encrypt_request.metadata_mut().append("authorization", MetadataValue::from_str(&api_key_header).unwrap());
    let _resp = client.debug_encrypt_ciphertext(encrypt_request).await?;

    println!("Scheduling computations...");
    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations
    });
    compute_request.metadata_mut().append("authorization", MetadataValue::from_str(&api_key_header).unwrap());
    let _resp = client.async_compute(compute_request).await?;

    println!("Computations scheduled, waiting upon completion...");

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        let count =
            sqlx::query!("SELECT count(*) FROM computations WHERE NOT is_completed AND NOT is_error")
                .fetch_one(&pool)
                .await?;
        let current_count = count.count.unwrap();
        if current_count == 0 {
            println!("All computations completed");
            break;
        } else {
            println!("{current_count} computations remaining, waiting...");
        }
    }

    let mut decrypt_request = tonic::Request::new(DebugDecryptRequest {
        handles: output_handles.clone(),
    });
    decrypt_request.metadata_mut().append("authorization", MetadataValue::from_str(&api_key_header).unwrap());
    let resp = client.debug_decrypt_ciphertext(decrypt_request).await?;

    assert_eq!(resp.get_ref().values.len(), output_handles.len(), "Outputs length doesn't match");
    for (idx, op) in ops.iter().enumerate() {
        let decr_response = &resp.get_ref().values[idx];
        println!("Checking computation for binary test bits:{} op:{} input:{} output:{}",
            op.bits, op.operand, op.inp.to_string(), op.expected_output.to_string());
        assert_eq!(decr_response.output_type, op.operand_types, "operand types not equal");
        assert_eq!(decr_response.value, op.expected_output.to_string(), "operand output values not equal");
    }

    Ok(())
}

fn generate_binary_test_cases() -> Vec<BinaryOperatorTestCase> {
    let mut cases = Vec::new();
    let mut push_case = |bits: i32, is_scalar: bool, shift_by: i32, op: SupportedFheOperations| {
        let mut lhs = BigInt::from(12);
        let mut rhs = BigInt::from(7);
        lhs <<= shift_by;
        rhs <<= shift_by;
        let expected_output = compute_expected_binary_output(&lhs, &rhs, op);
        let operand = op as i32;
        cases.push(BinaryOperatorTestCase {
            bits,
            operand,
            operand_types: supported_bits_to_bit_type_in_db(bits),
            lhs,
            rhs,
            expected_output,
            is_scalar,
        });
    };

    for bits in supported_bits() {
        let bits = *bits;
        let mut shift_by = bits - 8;
        for op in SupportedFheOperations::iter() {
            if op == SupportedFheOperations::FheMul {
                // don't go out of bit bounds when multiplying two numbers, so we shift by less
                shift_by /= 2;
            }
            if op.op_type() == FheOperationType::Binary {
                if does_fhe_operation_support_both_encrypted_operands(&op) {
                    push_case(bits, false, shift_by, op);
                }

                if does_fhe_operation_support_scalar(&op) {
                    push_case(bits, true, shift_by, op);
                }
            }
        }
    }

    cases
}

fn generate_unary_test_cases() -> Vec<UnaryOperatorTestCase> {
    let mut cases = Vec::new();

    for bits in supported_bits() {
        let bits = *bits;
        let shift_by = bits - 8;
        for op in SupportedFheOperations::iter() {
            if op.op_type() == FheOperationType::Unary {
                let mut inp = BigInt::from(7);
                inp <<= shift_by;
                let expected_output = compute_expected_unary_output(&inp, op, bits);
                let operand = op as i32;
                cases.push(UnaryOperatorTestCase {
                    bits,
                    operand,
                    operand_types: supported_bits_to_bit_type_in_db(bits),
                    inp,
                    expected_output,
                });
            }
        }
    }

    cases
}

fn compute_expected_unary_output(inp: &BigInt, op: SupportedFheOperations, bits: i32) -> BigInt {
    match op {
        SupportedFheOperations::FheNot => {
            // TODO: find how this is done appropriately in big int crate
            match bits {
                8 => {
                    let inp: u8 = inp.try_into().unwrap();
                    BigInt::from(inp.not())
                }
                16 => {
                    let inp: u16 = inp.try_into().unwrap();
                    BigInt::from(inp.not())
                }
                32 => {
                    let inp: u32 = inp.try_into().unwrap();
                    BigInt::from(inp.not())
                }
                other => {
                    panic!("unknown bits: {other}")
                }
            }
        },
        other => panic!("unsupported binary operation: {:?}", other),
    }
}

fn compute_expected_binary_output(lhs: &BigInt, rhs: &BigInt, op: SupportedFheOperations) -> BigInt {
    match op {
        SupportedFheOperations::FheAdd => lhs + rhs,
        SupportedFheOperations::FheSub => lhs - rhs,
        SupportedFheOperations::FheMul => lhs * rhs,
        SupportedFheOperations::FheDiv => lhs / rhs,
        other => panic!("unsupported binary operation: {:?}", other),
    }
}