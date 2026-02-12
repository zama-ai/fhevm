use std::str::FromStr;

use crate::{
    server::{
        common::FheOperation,
        tfhe_worker::{
            async_computation_input::Input, fhevm_coprocessor_client::FhevmCoprocessorClient,
            AsyncComputation, AsyncComputationInput, AsyncComputeRequest, InputToUpload,
            InputUploadBatch,
        },
    },
    tests::{
        inputs::{test_random_contract_address, test_random_user_address},
        utils::{default_api_key, latest_db_key, setup_test_app},
    },
};
use fhevm_engine_common::{crs::CrsCache, db_keys::DbKeyCache, utils::safe_serialize};
use tonic::metadata::MetadataValue;

#[tokio::test]
async fn test_coprocessor_input_errors() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;
    let api_key_header = format!("bearer {}", default_api_key());
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;

    let (key, crs) = latest_db_key(&pool).await;

    {
        // too many uploads at once
        let mut builder = tfhe::ProvenCompactCiphertextList::builder(&key.pks);
        let the_list = builder
            .push(false)
            .push(1u8)
            .push(2u16)
            .push(3u32)
            .push(4u64)
            .build_with_proof_packed(&crs.crs, &[], tfhe::zk::ZkComputeLoad::Proof)
            .unwrap();

        let serialized = safe_serialize(&the_list);

        let mut input_ciphertexts = Vec::new();
        for _ in 0..12 {
            input_ciphertexts.push(InputToUpload {
                input_payload: serialized.clone(),
                signatures: Vec::new(),
                user_address: test_random_user_address(),
                contract_address: test_random_contract_address(),
            });
        }

        println!("Encrypting inputs...");
        let mut input_request = tonic::Request::new(InputUploadBatch { input_ciphertexts });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        let resp = client.upload_inputs(input_request).await;
        match resp {
            Err(e) => {
                assert!(e.to_string().contains(
                    "More than maximum input blobs uploaded, maximum allowed: 10, uploaded: 12"
                ));
            }
            Ok(_) => {
                panic!("Should not have succeeded")
            }
        }
    }

    {
        // garbage ciphertext
        let mut builder = tfhe::ProvenCompactCiphertextList::builder(&key.pks);
        let the_list = builder
            .push(false)
            .push(1u8)
            .push(2u16)
            .push(3u32)
            .push(4u64)
            .build_with_proof_packed(&crs.crs, &[], tfhe::zk::ZkComputeLoad::Proof)
            .unwrap();

        let serialized = safe_serialize(&the_list);

        let input_ciphertexts = vec![InputToUpload {
            input_payload: serialized[0..32].to_vec(),
            signatures: Vec::new(),
            user_address: test_random_user_address(),
            contract_address: test_random_contract_address(),
        }];

        println!("Encrypting inputs...");
        let mut input_request = tonic::Request::new(InputUploadBatch { input_ciphertexts });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        let resp = client.upload_inputs(input_request).await;
        match resp {
            Err(e) => {
                assert!(e.to_string().contains("error deserializing ciphertext"));
            }
            Ok(_) => {
                panic!("Should not have succeeded")
            }
        }
    }

    {
        // more ciphertexts than limit
        let mut builder = tfhe::ProvenCompactCiphertextList::builder(&key.pks);
        for _ in 0..300 {
            let _ = builder.push(false);
        }

        let the_list = builder
            .build_with_proof_packed(&crs.crs, &[], tfhe::zk::ZkComputeLoad::Proof)
            .unwrap();
        let serialized = safe_serialize(&the_list);

        let input_ciphertexts = vec![InputToUpload {
            input_payload: serialized,
            signatures: Vec::new(),
            user_address: test_random_user_address(),
            contract_address: test_random_contract_address(),
        }];

        println!("Encrypting inputs...");
        let mut input_request = tonic::Request::new(InputUploadBatch { input_ciphertexts });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        let resp = client.upload_inputs(input_request).await;
        match resp {
            Err(e) => {
                eprintln!("error: {e}");
                assert!(e
                    .to_string()
                    .contains("Input blob contains too many ciphertexts"));
            }
            Ok(_) => {
                panic!("Should not have succeeded")
            }
        }
    }

    {
        // empty payload ok
        let input_ciphertexts = Vec::new();

        println!("Encrypting inputs...");
        let mut input_request = tonic::Request::new(InputUploadBatch { input_ciphertexts });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        let resp = client.upload_inputs(input_request).await;
        match resp {
            Ok(_) => {}
            Err(e) => {
                panic!("unexpected error: {e}")
            }
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_coprocessor_computation_errors() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;
    let api_key_header = format!("bearer {}", default_api_key());
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;

    let db_key_cache = DbKeyCache::new(100).unwrap();
    let key = db_key_cache.fetch_latest(&pool).await?;
    let crs_cache = CrsCache::load(&pool).await?;
    let crs = crs_cache.get_latest().unwrap();

    let mut handle_counter = 0;
    let mut next_handle = || {
        let out: i32 = handle_counter;
        handle_counter += 1;
        out.to_be_bytes().to_vec()
    };

    let initial_inputs_resp = {
        // not provided api key
        let mut builder = tfhe::ProvenCompactCiphertextList::builder(&key.pks);
        let the_list = builder
            .push(false)
            .push(1u8)
            .push(2u16)
            .push(3u32)
            .push(4u64)
            .build_with_proof_packed(&crs.crs, &[], tfhe::zk::ZkComputeLoad::Proof)
            .unwrap();

        let serialized = safe_serialize(&the_list);

        let input_ciphertexts = vec![InputToUpload {
            input_payload: serialized,
            signatures: Vec::new(),
            user_address: test_random_user_address(),
            contract_address: test_random_contract_address(),
        }];

        println!("Encrypting inputs...");
        let mut input_request = tonic::Request::new(InputUploadBatch { input_ciphertexts });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        client.upload_inputs(input_request).await?
    };

    let ct_vec = &initial_inputs_resp.get_ref().upload_responses;
    assert_eq!(ct_vec.len(), 1);
    let handles = &ct_vec[0].input_handles;
    assert_eq!(handles.len(), 5);
    let test_bool = &handles[0];
    let test_u8 = &handles[1];
    let test_u16 = &handles[2];
    let test_u32 = &handles[3];
    let test_u64 = &handles[4];

    {
        let transaction_id = next_handle();
        // test circular dependencies
        let output_handle_a = next_handle();
        let output_handle_b = next_handle();
        let output_handle_c = next_handle();
        // make circular dependency wheel
        // a depends on c
        // c depends on b
        // b depends on a
        let async_computations = vec![
            AsyncComputation {
                operation: FheOperation::FheAdd.into(),
                transaction_id: transaction_id.clone(),
                output_handle: output_handle_a.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(test_u8.handle.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(output_handle_c.clone())),
                    },
                ],
                is_allowed: true,
            },
            AsyncComputation {
                operation: FheOperation::FheAdd.into(),
                transaction_id: transaction_id.clone(),
                output_handle: output_handle_b.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(test_u8.handle.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(output_handle_a.clone())),
                    },
                ],
                is_allowed: true,
            },
            AsyncComputation {
                operation: FheOperation::FheAdd.into(),
                transaction_id: transaction_id.clone(),
                output_handle: output_handle_c.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(test_u8.handle.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(output_handle_b.clone())),
                    },
                ],
                is_allowed: true,
            },
        ];
        let mut input_request = tonic::Request::new(AsyncComputeRequest {
            computations: async_computations,
        });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        match client.async_compute(input_request).await {
            Ok(_) => {
                panic!("Expected failure")
            }
            Err(e) => {
                eprintln!("error: {}", e);
                assert!(e
                    .to_string()
                    .contains("has circular dependency and is uncomputable"));
            }
        }
    }

    {
        let transaction_id = next_handle();
        // test invalid binary op between uncast types
        let output_handle_a = next_handle();
        let async_computations = vec![AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id,
            output_handle: output_handle_a.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(test_u8.handle.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(test_u16.handle.clone())),
                },
            ],
            is_allowed: true,
        }];
        let mut input_request = tonic::Request::new(AsyncComputeRequest {
            computations: async_computations,
        });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        match client.async_compute(input_request).await {
            Ok(_) => (),
            Err(_e) => panic!("No type error detections."),
        }
    }

    {
        let transaction_id = next_handle();
        // empty ciphertext handle
        let output_handle_a = next_handle();
        let async_computations = vec![AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id,
            output_handle: output_handle_a.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(test_u32.handle.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(vec![])),
                },
            ],
            is_allowed: true,
        }];
        let mut input_request = tonic::Request::new(AsyncComputeRequest {
            computations: async_computations,
        });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        match client.async_compute(input_request).await {
            Ok(_) => {
                panic!("Expected failure")
            }
            Err(e) => {
                eprintln!("error: {}", e);
                assert!(e.to_string().contains("Found ciphertext handle is empty"));
            }
        }
    }

    {
        let transaction_id = next_handle();
        // ciphertext handle too long
        let output_handle_a = next_handle();
        let async_computations = vec![AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id,
            output_handle: output_handle_a.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(test_u32.handle.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(vec![0; 257])),
                },
            ],
            is_allowed: true,
        }];
        let mut input_request = tonic::Request::new(AsyncComputeRequest {
            computations: async_computations,
        });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        match client.async_compute(input_request).await {
            Ok(_) => {
                panic!("Expected failure")
            }
            Err(e) => {
                eprintln!("error: {}", e);
                assert!(e
                    .to_string()
                    .contains("Found ciphertext handle longer than 256 bytes"));
            }
        }
    }

    {
        let transaction_id = next_handle();
        // computation too many inputs
        let output_handle_a = next_handle();
        let async_computations = vec![AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id,
            output_handle: output_handle_a.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(test_u64.handle.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(test_u64.handle.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(test_u64.handle.clone())),
                },
            ],
            is_allowed: true,
        }];
        let mut input_request = tonic::Request::new(AsyncComputeRequest {
            computations: async_computations,
        });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        match client.async_compute(input_request).await {
            Ok(_) => {
                panic!("Expected failure")
            }
            Err(e) => {
                eprintln!("error: {}", e);
                assert!(e
                    .to_string()
                    .contains("fhevm error: UnexpectedOperandCountForFheOperation"));
            }
        }
    }

    {
        let transaction_id = next_handle();
        // scalar operand on the left
        let output_handle_a = next_handle();
        let async_computations = vec![AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id,
            output_handle: output_handle_a.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![123])),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(test_u64.handle.clone())),
                },
            ],
            is_allowed: true,
        }];
        let mut input_request = tonic::Request::new(AsyncComputeRequest {
            computations: async_computations,
        });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        match client.async_compute(input_request).await {
            Ok(_) => {
                panic!("Expected failure")
            }
            Err(e) => {
                eprintln!("error: {}", e);
                assert!(e
                    .to_string()
                    .contains("fhevm error: FheOperationOnlySecondOperandCanBeScalar"));
            }
        }
    }

    {
        let transaction_id = next_handle();
        // scalar division by zero
        let output_handle_a = next_handle();
        let async_computations = vec![AsyncComputation {
            operation: FheOperation::FheDiv.into(),
            transaction_id,
            output_handle: output_handle_a.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(test_u64.handle.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![0])),
                },
            ],
            is_allowed: true,
        }];
        let mut input_request = tonic::Request::new(AsyncComputeRequest {
            computations: async_computations,
        });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        match client.async_compute(input_request).await {
            Ok(_) => {
                panic!("Expected failure")
            }
            Err(e) => {
                eprintln!("error: {}", e);
                assert!(e
                    .to_string()
                    .contains("fhevm error: FheOperationScalarDivisionByZero"));
            }
        }
    }

    {
        let transaction_id = next_handle();
        // binary boolean inputs
        let output_handle_a = next_handle();
        let async_computations = vec![AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id,
            output_handle: output_handle_a.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(test_bool.handle.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(test_bool.handle.clone())),
                },
            ],
            is_allowed: true,
        }];
        let mut input_request = tonic::Request::new(AsyncComputeRequest {
            computations: async_computations,
        });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        match client.async_compute(input_request).await {
            Ok(_) => (),
            Err(_e) => panic!("No type error detections."),
        }
    }

    {
        let transaction_id = next_handle();
        // unary boolean inputs
        let output_handle_a = next_handle();
        let async_computations = vec![AsyncComputation {
            operation: FheOperation::FheNeg.into(),
            transaction_id,
            output_handle: output_handle_a.clone(),
            inputs: vec![AsyncComputationInput {
                input: Some(Input::InputHandle(test_bool.handle.clone())),
            }],
            is_allowed: true,
        }];
        let mut input_request = tonic::Request::new(AsyncComputeRequest {
            computations: async_computations,
        });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        match client.async_compute(input_request).await {
            Ok(_) => (),
            Err(_e) => panic!("No type error detections."),
        }
    }

    Ok(())
}
