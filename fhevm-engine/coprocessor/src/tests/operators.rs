use crate::server::common::FheOperation;
use crate::server::coprocessor::fhevm_coprocessor_client::FhevmCoprocessorClient;
use crate::server::coprocessor::{
    AsyncComputation, AsyncComputeRequest, TrivialEncryptBatch, TrivialEncryptRequestSingle,
};
use crate::tests::utils::{
    decrypt_ciphertexts, random_handle, wait_until_all_ciphertexts_computed,
};
use crate::{
    server::coprocessor::{async_computation_input::Input, AsyncComputationInput},
    tests::utils::{default_api_key, setup_test_app},
};
use bigdecimal::num_bigint::BigInt;
use fhevm_engine_common::tfhe_ops::{
    does_fhe_operation_support_both_encrypted_operands, does_fhe_operation_support_scalar,
};
use fhevm_engine_common::types::{FheOperationType, SupportedFheOperations};
use std::{ops::Not, str::FromStr};
use strum::IntoEnumIterator;
use tonic::metadata::MetadataValue;

struct BinaryOperatorTestCase {
    bits: i32,
    operand: i32,
    input_types: i32,
    expected_output_type: i32,
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
    &[1, 4, 8, 16, 32, 64, 128, 160, 256, 512, 1024, 2048]
}

pub fn supported_types() -> &'static [i32] {
    &[
        0,  // bool
        1,  // 4 bit
        2,  // 8 bit
        3,  // 16 bit
        4,  // 32 bit
        5,  // 64 bit
        6,  // 128 bit
        7,  // 160 bit
        8,  // 256 bit
        9,  // ebytes 64
        10, // ebytes 128
        11, // ebytes 256
    ]
}

fn supported_bits_to_bit_type_in_db(inp: i32) -> i32 {
    match inp {
        1 => 0, // 1 bit - boolean
        4 => 1,
        8 => 2,
        16 => 3,
        32 => 4,
        64 => 5,
        128 => 6,
        160 => 7,
        256 => 8,
        512 => 9,
        1024 => 10,
        2048 => 11,
        other => panic!("unknown supported bits: {other}"),
    }
}

#[tokio::test]
async fn test_fhe_binary_operands() -> Result<(), Box<dyn std::error::Error>> {
    let ops = generate_binary_test_cases();
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;

    let mut handle_counter: u64 = random_handle();
    let mut next_handle = || {
        let out: u64 = handle_counter;
        handle_counter += 1;
        out.to_be_bytes().to_vec()
    };

    let api_key_header = format!("bearer {}", default_api_key());

    let mut output_handles = Vec::with_capacity(ops.len());
    let mut enc_request_payload = Vec::with_capacity(ops.len() * 2);
    let mut async_computations = Vec::with_capacity(ops.len());
    for op in &ops {
        let lhs_handle = next_handle();
        let rhs_handle = if op.is_scalar {
            let (_, bytes) = op.rhs.to_bytes_be();
            bytes
        } else {
            next_handle()
        };
        let output_handle = next_handle();
        output_handles.push(output_handle.clone());

        let (_, lhs_bytes) = op.lhs.to_bytes_be();

        println!(
            "Encrypting inputs for binary test bits:{} op:{} is_scalar:{} lhs:{} rhs:{}",
            op.bits,
            op.operand,
            op.is_scalar,
            op.lhs.to_string(),
            op.rhs.to_string()
        );
        enc_request_payload.push(TrivialEncryptRequestSingle {
            handle: lhs_handle.clone(),
            be_value: lhs_bytes,
            output_type: op.input_types,
        });
        if !op.is_scalar {
            let (_, rhs_bytes) = op.rhs.to_bytes_be();
            enc_request_payload.push(TrivialEncryptRequestSingle {
                handle: rhs_handle.clone(),
                be_value: rhs_bytes,
                output_type: op.input_types,
            });
        }

        println!("rhs handle: 0x{}", hex::encode(&rhs_handle));
        println!("Scheduling computation for binary test bits:{} op:{} is_scalar:{} lhs:{} rhs:{} output:{}",
            op.bits, op.operand, op.is_scalar, op.lhs.to_string(), op.rhs.to_string(), op.expected_output.to_string());

        let mut inputs = vec![AsyncComputationInput {
            input: Some(Input::InputHandle(lhs_handle)),
        }];
        if op.is_scalar {
            inputs.push(AsyncComputationInput {
                input: Some(Input::Scalar(rhs_handle)),
            });
        } else {
            inputs.push(AsyncComputationInput {
                input: Some(Input::InputHandle(rhs_handle)),
            });
        }
        async_computations.push(AsyncComputation {
            operation: op.operand,
            output_handle: output_handle,
            inputs,
        });
    }

    println!("Encrypting inputs...");
    let mut encrypt_request = tonic::Request::new(TrivialEncryptBatch {
        values: enc_request_payload,
    });
    encrypt_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.trivial_encrypt_ciphertexts(encrypt_request).await?;

    println!("Scheduling computations...");
    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.async_compute(compute_request).await?;

    println!("Computations scheduled, waiting upon completion...");

    wait_until_all_ciphertexts_computed(&app).await?;

    let decrypt_request = output_handles.clone();
    let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;

    assert_eq!(
        resp.len(),
        output_handles.len(),
        "Outputs length doesn't match"
    );
    for (idx, op) in ops.iter().enumerate() {
        let decr_response = &resp[idx];
        println!("Checking computation for binary test bits:{} op:{} is_scalar:{} lhs:{} rhs:{} output:{}",
            op.bits, op.operand, op.is_scalar, op.lhs.to_string(), op.rhs.to_string(), op.expected_output.to_string());
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

#[tokio::test]
async fn test_fhe_unary_operands() -> Result<(), Box<dyn std::error::Error>> {
    let ops = generate_unary_test_cases();
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;

    let mut handle_counter: u64 = random_handle();
    let mut next_handle = || {
        let out: u64 = handle_counter;
        handle_counter += 1;
        out.to_be_bytes().to_vec()
    };

    let api_key_header = format!("bearer {}", default_api_key());

    let mut output_handles = Vec::with_capacity(ops.len());
    let mut enc_request_payload = Vec::with_capacity(ops.len() * 2);
    let mut async_computations = Vec::with_capacity(ops.len());
    for op in &ops {
        let input_handle = next_handle();
        let output_handle = next_handle();
        output_handles.push(output_handle.clone());

        let (_, inp_bytes) = op.inp.to_bytes_be();

        println!(
            "Encrypting inputs for unary test bits:{} op:{} input:{}",
            op.bits,
            op.operand,
            op.inp.to_string()
        );
        enc_request_payload.push(TrivialEncryptRequestSingle {
            handle: input_handle.clone(),
            be_value: inp_bytes,
            output_type: op.operand_types,
        });

        println!(
            "Scheduling computation for binary test bits:{} op:{} input:{} output:{}",
            op.bits,
            op.operand,
            op.inp.to_string(),
            op.expected_output.to_string()
        );
        async_computations.push(AsyncComputation {
            operation: op.operand,
            output_handle: output_handle,
            inputs: vec![AsyncComputationInput {
                input: Some(Input::InputHandle(input_handle)),
            }],
        });
    }

    println!("Encrypting inputs...");
    let mut encrypt_request = tonic::Request::new(TrivialEncryptBatch {
        values: enc_request_payload,
    });
    encrypt_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.trivial_encrypt_ciphertexts(encrypt_request).await?;

    println!("Scheduling computations...");
    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.async_compute(compute_request).await?;

    println!("Computations scheduled, waiting upon completion...");

    wait_until_all_ciphertexts_computed(&app).await?;

    let decrypt_request = output_handles.clone();
    let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;

    assert_eq!(
        resp.len(),
        output_handles.len(),
        "Outputs length doesn't match"
    );
    for (idx, op) in ops.iter().enumerate() {
        let decr_response = &resp[idx];
        println!(
            "Checking computation for binary test bits:{} op:{} input:{} output:{}",
            op.bits,
            op.operand,
            op.inp.to_string(),
            op.expected_output.to_string()
        );
        assert_eq!(
            decr_response.output_type, op.operand_types as i16,
            "operand types not equal"
        );
        let expected_value = if op.bits == 1 {
            op.expected_output.gt(&BigInt::from(0)).to_string()
        } else { op.expected_output.to_string() };
        assert_eq!(
            decr_response.value,
            expected_value,
            "operand output values not equal"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_fhe_casts() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;

    let mut handle_counter = random_handle();
    let mut next_handle = || {
        let out: u64 = handle_counter;
        handle_counter += 1;
        out.to_be_bytes().to_vec()
    };

    let api_key_header = format!("bearer {}", default_api_key());

    struct CastOutput {
        type_from: i32,
        type_to: i32,
        input: i32,
        expected_result: String,
    }

    let fhe_bool = 0;
    let mut output_handles = Vec::new();
    let mut enc_request_payload = Vec::new();
    let mut async_computations = Vec::new();
    let mut cast_outputs: Vec<CastOutput> = Vec::new();
    for type_from in supported_types() {
        for type_to in supported_types() {
            let input_handle = next_handle();
            let output_handle = next_handle();
            let input = 7;
            let (_, inp_bytes) = BigInt::from(input).to_bytes_be();
            let output = if *type_to == fhe_bool || *type_from == fhe_bool {
                // if bool output is 1
                1
            } else {
                input
            };

            println!(
                "Encrypting inputs for cast test type from:{type_from} type to:{type_to} input:{input} output:{output}",
            );
            enc_request_payload.push(TrivialEncryptRequestSingle {
                handle: input_handle.clone(),
                be_value: inp_bytes,
                output_type: *type_from,
            });
            cast_outputs.push(CastOutput {
                type_from: *type_from,
                type_to: *type_to,
                input,
                expected_result: if *type_to == fhe_bool {
                    (output > 0).to_string()
                } else {
                    output.to_string()
                },
            });

            output_handles.push(output_handle.clone());
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                output_handle,
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(input_handle.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![*type_to as u8])),
                    },
                ],
            });
        }
    }

    println!("Encrypting inputs...");
    let mut encrypt_request = tonic::Request::new(TrivialEncryptBatch {
        values: enc_request_payload,
    });
    encrypt_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.trivial_encrypt_ciphertexts(encrypt_request).await?;

    println!("Scheduling computations...");
    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.async_compute(compute_request).await?;

    println!("Computations scheduled, waiting upon completion...");

    wait_until_all_ciphertexts_computed(&app).await?;

    let decrypt_request = output_handles.clone();
    let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;

    assert_eq!(
        resp.len(),
        output_handles.len(),
        "Outputs length doesn't match"
    );
    for (idx, co) in cast_outputs.iter().enumerate() {
        let decr_response = &resp[idx];
        println!(
            "Checking computation for cast test from:{} to:{} input:{} output:{}",
            co.type_from, co.type_to, co.input, co.expected_result,
        );
        println!(
            "Response output type: {}, response result: {}",
            decr_response.output_type, decr_response.value
        );
        assert_eq!(
            decr_response.output_type, co.type_to as i16,
            "operand types not equal"
        );
        assert_eq!(
            decr_response.value, co.expected_result,
            "operand output values not equal"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_op_trivial_encrypt() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;

    let mut handle_counter = random_handle();
    let mut next_handle = || {
        let out: u64 = handle_counter;
        handle_counter += 1;
        out.to_be_bytes().to_vec()
    };

    let api_key_header = format!("bearer {}", default_api_key());

    struct TrivialEncryptionTestCase {
        inp_type: i32,
        inp: BigInt,
    }

    let mut test_cases: Vec<TrivialEncryptionTestCase> = Vec::new();
    test_cases.push(TrivialEncryptionTestCase {
        inp_type: 0,
        inp: BigInt::from(1),
    });

    let max_num: BigInt = BigInt::from(1) << 256 - 1;
    for bits in supported_bits() {
        let bits = *bits;
        let inp_type = supported_bits_to_bit_type_in_db(bits);
        let shift_by = bits - 1;
        let mut inp = BigInt::from(1);
        inp <<= shift_by;
        let inp = inp.min(max_num.clone());
        test_cases.push(TrivialEncryptionTestCase { inp_type, inp });
    }

    let mut async_computations = Vec::new();
    let mut output_handles = Vec::new();
    for case in &test_cases {
        let output_handle = next_handle();
        let (_, be_bytes) = case.inp.to_bytes_be();
        output_handles.push(output_handle.clone());
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheTrivialEncrypt.into(),
            output_handle,
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::Scalar(be_bytes)),
                },
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![case.inp_type as u8])),
                },
            ],
        });
    }

    println!("Scheduling computations...");
    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.async_compute(compute_request).await?;

    println!("Computations scheduled, waiting upon completion...");

    wait_until_all_ciphertexts_computed(&app).await?;

    let decrypt_request = output_handles.clone();
    let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;

    assert_eq!(
        resp.len(),
        output_handles.len(),
        "Outputs length doesn't match"
    );
    for (idx, co) in test_cases.iter().enumerate() {
        let decr_response = &resp[idx];
        let value_to_compare = match decr_response.value.as_str() {
            // for FheBool outputs
            "true" => "1",
            "false" => "0",
            other => other,
        };
        println!(
            "Checking trivial encryption input:{} type:{}",
            co.inp, co.inp_type
        );
        println!(
            "Response output type: {}, response result: {}",
            decr_response.output_type, decr_response.value
        );
        assert_eq!(
            decr_response.output_type, co.inp_type as i16,
            "operand types not equal"
        );
        assert_eq!(
            value_to_compare, co.inp.to_string(),
            "operand output values not equal"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_fhe_if_then_else() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;

    let mut handle_counter = random_handle();
    let mut next_handle = || {
        let out: u64 = handle_counter;
        handle_counter += 1;
        out.to_be_bytes().to_vec()
    };

    let api_key_header = format!("bearer {}", default_api_key());

    struct IfThenElseOutput {
        input_type: i32,
        left_input: i32,
        right_input: i32,
        input_bool: bool,
        expected_result: String,
    }

    let mut output_handles = Vec::new();
    let mut enc_request_payload = Vec::new();
    let mut async_computations = Vec::new();
    let mut if_then_else_outputs: Vec<IfThenElseOutput> = Vec::new();

    let fhe_bool_type = 0;
    let false_handle = next_handle();
    let true_handle = next_handle();
    enc_request_payload.push(TrivialEncryptRequestSingle {
        handle: false_handle.clone(),
        be_value: BigInt::from(0).to_bytes_be().1,
        output_type: fhe_bool_type,
    });
    enc_request_payload.push(TrivialEncryptRequestSingle {
        handle: true_handle.clone(),
        be_value: BigInt::from(1).to_bytes_be().1,
        output_type: fhe_bool_type,
    });

    let fhe_bool_type = 0;
    for input_types in supported_types() {
        let left_handle = next_handle();
        let right_handle = next_handle();
        let is_input_bool = *input_types == fhe_bool_type;
        let (left_input, right_input) = if is_input_bool { (0, 1) } else { (7, 12) };
        enc_request_payload.push(TrivialEncryptRequestSingle {
            handle: left_handle.clone(),
            be_value: BigInt::from(left_input).to_bytes_be().1,
            output_type: *input_types,
        });
        enc_request_payload.push(TrivialEncryptRequestSingle {
            handle: right_handle.clone(),
            be_value: BigInt::from(right_input).to_bytes_be().1,
            output_type: *input_types,
        });

        for test_value in [false, true] {
            let output_handle = next_handle();
            let (expected_result, input_handle) = if test_value {
                (left_input, &true_handle)
            } else {
                (right_input, &false_handle)
            };
            if_then_else_outputs.push(IfThenElseOutput {
                input_type: *input_types,
                input_bool: test_value,
                left_input,
                right_input,
                expected_result: if *input_types == fhe_bool_type {
                    (expected_result > 0).to_string()
                } else {
                    expected_result.to_string()
                },
            });

            output_handles.push(output_handle.clone());
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheIfThenElse.into(),
                output_handle,
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(input_handle.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(left_handle.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(right_handle.clone())),
                    },
                ],
            });
        }
    }

    println!("Encrypting inputs...");
    let mut encrypt_request = tonic::Request::new(TrivialEncryptBatch {
        values: enc_request_payload,
    });
    encrypt_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.trivial_encrypt_ciphertexts(encrypt_request).await?;

    println!("Scheduling computations...");
    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.async_compute(compute_request).await?;

    println!("Computations scheduled, waiting upon completion...");

    wait_until_all_ciphertexts_computed(&app).await?;

    let decrypt_request = output_handles.clone();
    let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;

    assert_eq!(
        resp.len(),
        output_handles.len(),
        "Outputs length doesn't match"
    );
    for (idx, co) in if_then_else_outputs.iter().enumerate() {
        let decr_response = &resp[idx];
        println!(
            "Checking if then else computation for test type:{} control:{} lhs:{} rhs:{} output:{}",
            co.input_type, co.input_bool, co.left_input, co.right_input, co.expected_result,
        );
        println!(
            "Response output type: {}, response result: {}",
            decr_response.output_type, decr_response.value
        );
        assert_eq!(
            decr_response.output_type, co.input_type as i16,
            "operand types not equal"
        );
        assert_eq!(
            decr_response.value, co.expected_result,
            "operand output values not equal"
        );
    }

    Ok(())
}

fn generate_binary_test_cases() -> Vec<BinaryOperatorTestCase> {
    let mut cases = Vec::new();
    let bit_shift_ops = [
        SupportedFheOperations::FheShl,
        SupportedFheOperations::FheShr,
        SupportedFheOperations::FheRotl,
        SupportedFheOperations::FheRotr,
    ];
    let fhe_bool_type = 0;
    let mut push_case = |bits: i32, is_scalar: bool, shift_by: i32, op: SupportedFheOperations| {
        let mut lhs = BigInt::from(6);
        let mut rhs = BigInt::from(2);
        lhs <<= shift_by;
        // don't shift by much for bit shift opts not to make result 0
        if bit_shift_ops.contains(&op) {
            rhs = BigInt::from(1);
        } else {
            rhs <<= shift_by;
        }
        let expected_output = compute_expected_binary_output(&lhs, &rhs, op);
        let operand = op as i32;
        let expected_output_type = if op.is_comparison() {
            fhe_bool_type
        } else {
            supported_bits_to_bit_type_in_db(bits)
        };
        cases.push(BinaryOperatorTestCase {
            bits,
            operand,
            expected_output_type,
            input_types: supported_bits_to_bit_type_in_db(bits),
            lhs,
            rhs,
            expected_output,
            is_scalar,
        });
    };

    let mut bool_cases = Vec::new();

    for bits in supported_bits() {
        let bits = *bits;
        let mut shift_by =
            if bits > 4 { bits - 8 } else { 0 };
        for op in SupportedFheOperations::iter() {
            if op.op_type() != FheOperationType::Binary {
                continue;
            }

            if bits > 256 && !op.supports_ebytes_inputs() {
                continue;
            }

            if bits == 1 {
                if !op.supports_bool_inputs() {
                    continue;
                }

                let lhs = BigInt::from(0);
                let rhs = BigInt::from(1);
                let expected_output = compute_expected_binary_output(&lhs, &rhs, op);
                if does_fhe_operation_support_both_encrypted_operands(&op) {
                    bool_cases.push(BinaryOperatorTestCase {
                        bits,
                        operand: op as i32,
                        expected_output_type: fhe_bool_type,
                        input_types: supported_bits_to_bit_type_in_db(bits),
                        lhs: lhs.clone(),
                        rhs: rhs.clone(),
                        expected_output: expected_output.clone(),
                        is_scalar: false,
                    });
                }

                if does_fhe_operation_support_scalar(&op) {
                    bool_cases.push(BinaryOperatorTestCase {
                        bits,
                        operand: op as i32,
                        expected_output_type: fhe_bool_type,
                        input_types: supported_bits_to_bit_type_in_db(bits),
                        lhs,
                        rhs,
                        expected_output,
                        is_scalar: true,
                    });
                }
            } else {
                if op == SupportedFheOperations::FheMul {
                    // don't go out of bit bounds when multiplying two numbers, so we shift by less
                    shift_by /= 2;
                }
                if op.op_type() == FheOperationType::Binary {
                    if does_fhe_operation_support_both_encrypted_operands(&op) {
                        push_case(bits, false, shift_by, op);
                    }

                    if does_fhe_operation_support_scalar(&op) && bits <= 256 {
                        push_case(bits, true, shift_by, op);
                    }
                }
            }
        }
    }

    cases.extend(bool_cases);

    cases
}

fn generate_unary_test_cases() -> Vec<UnaryOperatorTestCase> {
    let mut cases = Vec::new();

    for bits in supported_bits() {
        let bits = *bits;
        let shift_by = bits - 3;
        let max_bits_value = (BigInt::from(1) << bits) - 1;
        for op in SupportedFheOperations::iter() {
            if bits == 1 && !op.supports_bool_inputs() {
                continue;
            }

            if op.op_type() == FheOperationType::Unary {
                let inp =
                    if bits == 1 {
                        BigInt::from(1)
                    } else {
                        let mut res = BigInt::from(3);
                        res <<= shift_by;
                        res
                    };
                let expected_output = compute_expected_unary_output(&inp, op) & &max_bits_value;
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

fn compute_expected_unary_output(inp: &BigInt, op: SupportedFheOperations) -> BigInt {
    match op {
        SupportedFheOperations::FheNot => {
            let (_, mut bytes) = inp.to_bytes_be();
            for byte in bytes.iter_mut() {
                *byte = byte.not();
            }
            BigInt::from_bytes_be(bigdecimal::num_bigint::Sign::Plus, &bytes)
        }
        SupportedFheOperations::FheNeg => {
            let (_, mut bytes) = inp.to_bytes_be();
            for byte in bytes.iter_mut() {
                *byte = byte.not();
            }
            let num = BigInt::from_bytes_be(bigdecimal::num_bigint::Sign::Plus, &bytes);
            num + 1
        }
        other => panic!("unsupported unary operation: {:?}", other),
    }
}

fn compute_expected_binary_output(
    lhs: &BigInt,
    rhs: &BigInt,
    op: SupportedFheOperations,
) -> BigInt {
    match op {
        SupportedFheOperations::FheEq => BigInt::from(lhs.eq(rhs)),
        SupportedFheOperations::FheNe => BigInt::from(lhs.ne(rhs)),
        SupportedFheOperations::FheGe => BigInt::from(lhs.ge(rhs)),
        SupportedFheOperations::FheGt => BigInt::from(lhs.gt(rhs)),
        SupportedFheOperations::FheLe => BigInt::from(lhs.le(rhs)),
        SupportedFheOperations::FheLt => BigInt::from(lhs.lt(rhs)),
        SupportedFheOperations::FheMin => lhs.min(rhs).clone(),
        SupportedFheOperations::FheMax => lhs.max(rhs).clone(),
        SupportedFheOperations::FheAdd => lhs + rhs,
        SupportedFheOperations::FheSub => lhs - rhs,
        SupportedFheOperations::FheMul => lhs * rhs,
        SupportedFheOperations::FheDiv => lhs / rhs,
        SupportedFheOperations::FheRem => lhs % rhs,
        SupportedFheOperations::FheBitAnd => lhs & rhs,
        SupportedFheOperations::FheBitOr => lhs | rhs,
        SupportedFheOperations::FheBitXor => lhs ^ rhs,
        SupportedFheOperations::FheShl => lhs << (TryInto::<u64>::try_into(rhs).unwrap()),
        SupportedFheOperations::FheShr => lhs >> (TryInto::<u64>::try_into(rhs).unwrap()),
        // we don't shift by as much as to overlap the register
        // in tests, so should be same as bit shifts
        SupportedFheOperations::FheRotl => lhs << (TryInto::<u64>::try_into(rhs).unwrap()),
        SupportedFheOperations::FheRotr => lhs >> (TryInto::<u64>::try_into(rhs).unwrap()),
        other => panic!("unsupported binary operation: {:?}", other),
    }
}
