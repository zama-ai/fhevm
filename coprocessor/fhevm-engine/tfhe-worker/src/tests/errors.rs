use std::str::FromStr;

use crate::{
    db_queries::query_tenant_keys,
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
        utils::{default_api_key, default_tenant_id, setup_test_app},
    },
};
use fhevm_engine_common::utils::safe_serialize;
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

    let keys = query_tenant_keys(vec![default_tenant_id()], &pool)
        .await
        .map_err(|e| {
            let e: Box<dyn std::error::Error> = e;
            e
        })?;
    let keys = &keys[0];

    {
        // too many uploads at once
        let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.pks);
        let the_list = builder
            .push(false)
            .push(1u8)
            .push(2u16)
            .push(3u32)
            .push(4u64)
            .build_with_proof_packed(&keys.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
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
        let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.pks);
        let the_list = builder
            .push(false)
            .push(1u8)
            .push(2u16)
            .push(3u32)
            .push(4u64)
            .build_with_proof_packed(&keys.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
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
        let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.pks);
        for _ in 0..300 {
            let _ = builder.push(false);
        }

        let the_list = builder
            .build_with_proof_packed(&keys.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
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
async fn test_coprocessor_api_key_errors() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;

    {
        // not provided api key
        println!("Encrypting inputs...");
        let input_request = tonic::Request::new(InputUploadBatch {
            input_ciphertexts: Vec::new(),
        });
        let resp = client.upload_inputs(input_request).await;
        match resp {
            Err(e) => {
                assert!(e
                    .to_string()
                    .contains("API key unknown/invalid/not provided"));
            }
            Ok(_) => {
                panic!("Should not have succeeded")
            }
        }
    }

    {
        // invalid api key
        println!("Encrypting inputs...");
        let mut input_request = tonic::Request::new(InputUploadBatch {
            input_ciphertexts: Vec::new(),
        });
        const API_KEY_HEADER: &str = "bearer invalid-guid";
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(API_KEY_HEADER).unwrap(),
        );
        let resp = client.upload_inputs(input_request).await;
        match resp {
            Err(e) => {
                assert!(e
                    .to_string()
                    .contains("API key unknown/invalid/not provided"));
            }
            Ok(_) => {
                panic!("Should not have succeeded")
            }
        }
    }

    {
        // non existing
        println!("Encrypting inputs...");
        let mut input_request = tonic::Request::new(InputUploadBatch {
            input_ciphertexts: Vec::new(),
        });

        const API_KEY_HEADER: &str = "bearer 9a671665-3842-400f-b4d1-37e194e5e9a0";
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(API_KEY_HEADER).unwrap(),
        );
        let resp = client.upload_inputs(input_request).await;
        match resp {
            Err(e) => {
                assert!(e
                    .to_string()
                    .contains("API key unknown/invalid/not provided"));
            }
            Ok(_) => {
                panic!("Should not have succeeded")
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

    let keys = query_tenant_keys(vec![default_tenant_id()], &pool)
        .await
        .map_err(|e| {
            let e: Box<dyn std::error::Error> = e;
            e
        })?;
    let keys = &keys[0];

    let mut handle_counter = 0;
    let mut next_handle = || {
        let out: i32 = handle_counter;
        handle_counter += 1;
        out.to_be_bytes().to_vec()
    };

    let initial_inputs_resp = {
        // not provided api key
        let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.pks);
        let the_list = builder
            .push(false)
            .push(1u8)
            .push(2u16)
            .push(3u32)
            .push(4u64)
            .build_with_proof_packed(&keys.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
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
                block_hash: vec![],
                block_number: 0,
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
                block_hash: vec![],
                block_number: 0,
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
                block_hash: vec![],
                block_number: 0,
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
            block_hash: vec![],
            block_number: 0,
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
            block_hash: vec![],
            block_number: 0,
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
            block_hash: vec![],
            block_number: 0,
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
            block_hash: vec![],
            block_number: 0,
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
            block_hash: vec![],
            block_number: 0,
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
            block_hash: vec![],
            block_number: 0,
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
            block_hash: vec![],
            block_number: 0,
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
            block_hash: vec![],
            block_number: 0,
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


#[tokio::test]
async fn test_coprocessor_reorb_tx_id_collision_errors() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
     tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .compact()
        .try_init()
        .ok();
    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;
    let api_key_header = format!("bearer {}", default_api_key());
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;

    let keys = query_tenant_keys(vec![default_tenant_id()], &pool)
        .await
        .map_err(|e| {
            let e: Box<dyn std::error::Error> = e;
            e
        })?;
    let keys = &keys[0];

    let mut handle_counter = 0;
    let mut next_handle = || {
        let out: i32 = handle_counter;
        handle_counter += 1;
        out.to_be_bytes().to_vec()
    };

    let initial_inputs_resp = {
        // not provided api key
        let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.pks);
        let the_list = builder
            .push(1u8)
            .push(2u8)
            .build_with_proof_packed(&keys.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
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
    assert_eq!(handles.len(), 2);
    let test_1_u8 = &handles[0];
    let test_2_u8 = &handles[1];

    let transaction_a_1 = next_handle();
    let transaction_b_1 = next_handle();
    let transaction_a_2 = transaction_b_1.clone(); // tx id swapped in reorg
    let transaction_b_2 = transaction_a_1.clone(); // tx id swapped in reorg
    let block_1 = next_handle();
    let block_2 = next_handle();
    let result_a_1 = next_handle();
    let result_b_1 = next_handle();
    let result_a_2 = next_handle();
    let result_b_2 = next_handle();
    //  test with 4 transaction without seeing block difference
    // create transaction_A_1 -> transaction_B_1
    // and    transaction_A_2 -> transaction_B_2
    let async_computations_1;
    {
        // without block hash difference it's considered as a cycle
        async_computations_1 = vec![
            // Before reorg
            AsyncComputation {
                operation: FheOperation::FheAdd.into(),
                transaction_id: transaction_a_1.clone(),
                output_handle: result_a_1.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(test_1_u8.handle.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(test_2_u8.handle.clone())),
                    },
                ],
                is_allowed: true,
                block_hash: block_1.clone(),
                block_number: 0,
            },
            AsyncComputation {
                operation: FheOperation::FheAdd.into(),
                transaction_id: transaction_b_1.clone(),
                output_handle: result_b_1.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(result_a_1.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(result_a_1.clone())),
                    },
                ],
                is_allowed: true,
                block_hash:  block_1.clone(),
                block_number: 0,
            },
            // After reorg
            AsyncComputation {
                operation: FheOperation::FheAdd.into(),
                transaction_id: transaction_a_2.clone(),
                output_handle: result_a_2.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(test_1_u8.handle.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(test_2_u8.handle.clone())),
                    },
                ],
                is_allowed: true,
                block_hash: block_1.clone(), // simulate block hash not used to distinguish
                block_number: 0,
            },
            AsyncComputation {
                operation: FheOperation::FheAdd.into(),
                transaction_id: transaction_b_2.clone(),
                output_handle: result_b_2.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(result_a_2.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(result_a_2.clone())),
                    },
                ],
                is_allowed: true,
                block_hash:  block_1.clone(),  // simulate block hash not used to distinguish
                block_number: 0,
            },
        ];
        let mut input_request = tonic::Request::new(AsyncComputeRequest {
            computations: async_computations_1.clone(),
        });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        client.async_compute(input_request).await?; // detected but silently ignored
        tracing::warn!("Waiting for computations to be processed...");
        for _ in 1..10 {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await; // wait for processing
            let not_completed = sqlx::query!(
                "SELECT output_handle, fhe_operation, block_hash, block_number, is_completed FROM computations WHERE (transaction_id = $1 OR transaction_id = $2) AND is_completed = false",
                transaction_a_1,
                transaction_b_1
                )
                .fetch_all(&pool)
                .await?;
            eprintln!("Not completed:");
            for r in &not_completed {
                eprintln!("\tNot completed {:?}", r);
            }
            assert!(!not_completed.is_empty(), "Expected not completed computations");
        }
        tracing::warn!("Waiting finished");
    }
    tracing::warn!("Cleaning");
    sqlx::query!(
        r#"
        DELETE FROM computations
        WHERE transaction_id = $1 OR transaction_id = $2
        "#,
        transaction_a_1,
        transaction_b_1
    )
    .execute(&pool)
    .await?;
    sqlx::query!(
        r#"
        DELETE FROM ciphertexts
        WHERE handle = $1 OR handle = $2 OR handle = $3 OR handle = $4
        "#,
        result_a_1,
        result_b_1,
        result_a_2,
        result_b_2,
    )
    .execute(&pool)
    .await?;
    // with block hash difference it's ok
    let mut async_computations_2 = async_computations_1.clone();
    async_computations_2[2].block_hash = block_2.clone();
    async_computations_2[3].block_hash = block_2.clone();
    {
        tracing::warn!("Retest");
        // with block hash difference it's ok
        let mut input_request = tonic::Request::new(AsyncComputeRequest {
            computations: async_computations_2,
        });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        client.async_compute(input_request).await?;
        tracing::warn!("Waiting for computations to be processed...");
        for i in 1..=10 {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await; // wait for processing
            let not_completed = sqlx::query!(
                "SELECT output_handle, fhe_operation, block_hash, block_number, is_completed FROM computations WHERE (transaction_id = $1 OR transaction_id = $2) AND is_completed = false",
                transaction_a_1,
                transaction_b_1
                )
                .fetch_all(&pool)
                .await?;
            eprintln!("Not completed:");
            for r in &not_completed {
                eprintln!("\tNot completed {:?}", r);
            }
            tracing::warn!("Not completed {:?}", not_completed);
            if not_completed.is_empty() {
                break;
            }
            if i == 10 {
                assert!(not_completed.is_empty(), "Expected completed computations");
            }
        }
        tracing::warn!("Waiting finished");
    }
    Ok(())
}
