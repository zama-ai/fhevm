use crate::db_queries::query_tenant_keys;
use crate::server::common::FheOperation;
use crate::server::coprocessor::{async_computation_input::Input, AsyncComputationInput};
use crate::server::coprocessor::{
    fhevm_coprocessor_client::FhevmCoprocessorClient, AsyncComputation, AsyncComputeRequest,
    InputToUpload, InputUploadBatch,
};
use crate::tests::utils::{
    allow_handle, decrypt_ciphertexts, default_api_key, default_tenant_id, random_handle,
    setup_test_app, wait_until_all_allowed_handles_computed,
};
use fhevm_engine_common::utils::safe_serialize;
use std::str::FromStr;
use tonic::metadata::MetadataValue;

pub fn test_random_user_address() -> String {
    let _private_key = "bd2400c676871534a682ca1c5e4cd647ec9c3e122f188c6e3f54e6900d586c7b";
    let public_key = "0x1BdA2a485c339C95a9AbfDe52E80ca38e34C199E";
    public_key.to_string()
}

pub fn test_random_contract_address() -> String {
    "0x76c222560Db6b8937B291196eAb4Dad8930043aE".to_string()
}

#[tokio::test]
async fn schedule_erc20_whitepaper() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut output_handles = vec![];
    let mut async_computations = vec![];
    let mut num_samples: usize = 2;
    let samples = std::env::var("FHEVM_TEST_NUM_SAMPLES");
    if let Ok(samples) = samples {
        num_samples = samples.parse::<usize>().unwrap();
    }

    let keys = query_tenant_keys(vec![default_tenant_id()], &pool)
        .await
        .map_err(|e| {
            let e: Box<dyn std::error::Error> = e;
            e
        })?;
    let keys = &keys[0];

    for _ in 0..=(num_samples - 1) as u32 {
        let transaction_id = next_handle();
        let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.pks);
        let the_list = builder
            .push(100_u64) // Balance source
            .push(10_u64) // Transfer amount
            .push(20_u64) // Balance destination
            .build_with_proof_packed(&keys.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
            .unwrap();

        let serialized = safe_serialize(&the_list);
        println!("Encrypting inputs...");
        let mut input_request = tonic::Request::new(InputUploadBatch {
            input_ciphertexts: vec![InputToUpload {
                input_payload: serialized,
                signatures: Vec::new(),
                user_address: test_random_user_address(),
                contract_address: test_random_contract_address(),
            }],
        });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        let resp = client.upload_inputs(input_request).await?;
        let resp = resp.get_ref();
        assert_eq!(resp.upload_responses.len(), 1);
        let first_resp = &resp.upload_responses[0];
        assert_eq!(first_resp.input_handles.len(), 3);

        let handle_bals = first_resp.input_handles[0].handle.clone();
        let bals = AsyncComputationInput {
            input: Some(Input::InputHandle(handle_bals.clone())),
        };
        let handle_trxa = first_resp.input_handles[1].handle.clone();
        let trxa = AsyncComputationInput {
            input: Some(Input::InputHandle(handle_trxa.clone())),
        };
        let handle_bald = first_resp.input_handles[2].handle.clone();
        let bald = AsyncComputationInput {
            input: Some(Input::InputHandle(handle_bald.clone())),
        };

        let has_enough_funds_handle = next_handle();
        output_handles.push(has_enough_funds_handle.clone());
        let new_to_amount_target_handle = next_handle();
        output_handles.push(new_to_amount_target_handle.clone());
        let new_to_amount_handle = next_handle();
        output_handles.push(new_to_amount_handle.clone());
        let new_from_amount_target_handle = next_handle();
        output_handles.push(new_from_amount_target_handle.clone());
        let new_from_amount_handle = next_handle();
        output_handles.push(new_from_amount_handle.clone());

        async_computations.push(AsyncComputation {
            operation: FheOperation::FheGe.into(),
            transaction_id: transaction_id.clone(),
            output_handle: has_enough_funds_handle.clone(),
            inputs: vec![bals.clone(), trxa.clone()],
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_target_handle.clone(),
            inputs: vec![bald.clone(), trxa.clone()],
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheIfThenElse.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_handle.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(has_enough_funds_handle.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(new_to_amount_target_handle.clone())),
                },
                bald.clone(),
            ],
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheSub.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_from_amount_target_handle.clone(),
            inputs: vec![bals.clone(), trxa.clone()],
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheIfThenElse.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_from_amount_handle.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(has_enough_funds_handle.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(new_from_amount_target_handle.clone())),
                },
                bals.clone(),
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

    for h in output_handles.iter() {
        allow_handle(h, &pool).await?;
    }

    println!("Computations scheduled, waiting upon completion...");
    wait_until_all_allowed_handles_computed(&app).await?;

    let decrypt_request = output_handles.clone();
    let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;

    assert_eq!(
        resp.len(),
        output_handles.len(),
        "Outputs length doesn't match"
    );
    for (i, r) in resp.iter().enumerate() {
        match r.value.as_str() {
            "true" if i % 5 == 0 => (), // trxa <= bals true
            "30" if i % 5 == 1 => (),   // bald + trxa
            "30" if i % 5 == 2 => (),   // select bald + trxa
            "90" if i % 5 == 3 => (),   // bals - trxa
            "90" if i % 5 == 4 => (),   // select bald - trxa
            s => panic!("unexpected result: {} for output {i}", s),
        }
    }
    Ok(())
}

#[tokio::test]
async fn schedule_erc20_no_cmux() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut output_handles = vec![];
    let mut async_computations = vec![];
    let mut num_samples: usize = 2;
    let samples = std::env::var("FHEVM_TEST_NUM_SAMPLES");
    if let Ok(samples) = samples {
        num_samples = samples.parse::<usize>().unwrap();
    }

    let keys = query_tenant_keys(vec![default_tenant_id()], &pool)
        .await
        .map_err(|e| {
            let e: Box<dyn std::error::Error> = e;
            e
        })?;
    let keys = &keys[0];

    for _ in 0..=(num_samples - 1) as u32 {
        let transaction_id = next_handle();
        let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.pks);
        let the_list = builder
            .push(100_u64) // Balance source
            .push(10_u64) // Transfer amount
            .push(20_u64) // Balance destination
            .build_with_proof_packed(&keys.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
            .unwrap();

        let serialized = safe_serialize(&the_list);
        println!("Encrypting inputs...");
        let mut input_request = tonic::Request::new(InputUploadBatch {
            input_ciphertexts: vec![InputToUpload {
                input_payload: serialized,
                signatures: Vec::new(),
                user_address: test_random_user_address(),
                contract_address: test_random_contract_address(),
            }],
        });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        let resp = client.upload_inputs(input_request).await?;
        let resp = resp.get_ref();
        assert_eq!(resp.upload_responses.len(), 1);
        let first_resp = &resp.upload_responses[0];
        assert_eq!(first_resp.input_handles.len(), 3);

        let handle_bals = first_resp.input_handles[0].handle.clone();
        let bals = AsyncComputationInput {
            input: Some(Input::InputHandle(handle_bals.clone())),
        };
        let handle_trxa = first_resp.input_handles[1].handle.clone();
        let trxa = AsyncComputationInput {
            input: Some(Input::InputHandle(handle_trxa.clone())),
        };
        let handle_bald = first_resp.input_handles[2].handle.clone();
        let bald = AsyncComputationInput {
            input: Some(Input::InputHandle(handle_bald.clone())),
        };

        let has_enough_funds_handle = next_handle();
        output_handles.push(has_enough_funds_handle.clone());
        let cast_has_enough_funds_handle = next_handle();
        output_handles.push(cast_has_enough_funds_handle.clone());
        let select_amount_handle = next_handle();
        output_handles.push(select_amount_handle.clone());
        let new_to_amount_handle = next_handle();
        output_handles.push(new_to_amount_handle.clone());
        let new_from_amount_handle = next_handle();
        output_handles.push(new_from_amount_handle.clone());

        async_computations.push(AsyncComputation {
            operation: FheOperation::FheGe.into(),
            transaction_id: transaction_id.clone(),
            output_handle: has_enough_funds_handle.clone(),
            inputs: vec![bals.clone(), trxa.clone()],
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheCast.into(),
            transaction_id: transaction_id.clone(),
            output_handle: cast_has_enough_funds_handle.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(has_enough_funds_handle.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![5u8])),
                },
            ],
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheMul.into(),
            transaction_id: transaction_id.clone(),
            output_handle: select_amount_handle.clone(),
            inputs: vec![
                trxa.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(cast_has_enough_funds_handle.clone())),
                },
            ],
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_handle.clone(),
            inputs: vec![
                bald.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(select_amount_handle.clone())),
                },
            ],
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheSub.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_from_amount_handle.clone(),
            inputs: vec![
                bals.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(select_amount_handle.clone())),
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

    for h in output_handles.iter() {
        allow_handle(h, &pool).await?;
    }

    println!("Computations scheduled, waiting upon completion...");
    wait_until_all_allowed_handles_computed(&app).await?;

    let decrypt_request = output_handles.clone();
    let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;

    assert_eq!(
        resp.len(),
        output_handles.len(),
        "Outputs length doesn't match"
    );
    for (i, r) in resp.iter().enumerate() {
        match r.value.as_str() {
            "true" if i % 5 == 0 => (), // trxa <= bals true
            "1" if i % 5 == 1 => (),    // cast
            "10" if i % 5 == 2 => (),   // select trxa * cast
            "30" if i % 5 == 3 => (),   // bals + selected trxa
            "90" if i % 5 == 4 => (),   // bald - selected trxa
            s => panic!("unexpected result: {} for output {i}", s),
        }
    }
    Ok(())
}

#[tokio::test]
async fn schedule_dependent_erc20_no_cmux() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut output_handles = vec![];
    let mut async_computations = vec![];
    let mut num_samples: usize = 2;
    let samples = std::env::var("FHEVM_TEST_NUM_SAMPLES");
    if let Ok(samples) = samples {
        num_samples = samples.parse::<usize>().unwrap();
    }

    let keys = query_tenant_keys(vec![default_tenant_id()], &pool)
        .await
        .map_err(|e| {
            let e: Box<dyn std::error::Error> = e;
            e
        })?;
    let keys = &keys[0];

    let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.pks);
    let the_list = builder
        .push(20_u64) // Initial balance destination
        .build_with_proof_packed(&keys.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
        .unwrap();
    let serialized = safe_serialize(&the_list);
    let mut input_request = tonic::Request::new(InputUploadBatch {
        input_ciphertexts: vec![InputToUpload {
            input_payload: serialized,
            signatures: Vec::new(),
            user_address: test_random_user_address(),
            contract_address: test_random_contract_address(),
        }],
    });
    input_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let resp = client.upload_inputs(input_request).await?;
    let resp = resp.get_ref();
    assert_eq!(resp.upload_responses.len(), 1);
    let first_resp = &resp.upload_responses[0];
    assert_eq!(first_resp.input_handles.len(), 1);
    let handle_bald = first_resp.input_handles[0].handle.clone();
    let mut bald = AsyncComputationInput {
        input: Some(Input::InputHandle(handle_bald.clone())),
    };

    for _ in 0..=(num_samples - 1) as u32 {
        let transaction_id = next_handle();
        let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.pks);
        let the_list = builder
            .push(100_u64) // Balance source
            .push(10_u64) // Transfer amount
            .build_with_proof_packed(&keys.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
            .unwrap();

        let serialized = safe_serialize(&the_list);
        println!("Encrypting inputs...");
        let mut input_request = tonic::Request::new(InputUploadBatch {
            input_ciphertexts: vec![InputToUpload {
                input_payload: serialized,
                signatures: Vec::new(),
                user_address: test_random_user_address(),
                contract_address: test_random_contract_address(),
            }],
        });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        let resp = client.upload_inputs(input_request).await?;
        let resp = resp.get_ref();
        assert_eq!(resp.upload_responses.len(), 1);
        let first_resp = &resp.upload_responses[0];
        assert_eq!(first_resp.input_handles.len(), 2);

        let handle_bals = first_resp.input_handles[0].handle.clone();
        let bals = AsyncComputationInput {
            input: Some(Input::InputHandle(handle_bals.clone())),
        };
        let handle_trxa = first_resp.input_handles[1].handle.clone();
        let trxa = AsyncComputationInput {
            input: Some(Input::InputHandle(handle_trxa.clone())),
        };

        let has_enough_funds_handle = next_handle();
        output_handles.push(has_enough_funds_handle.clone());
        let cast_has_enough_funds_handle = next_handle();
        output_handles.push(cast_has_enough_funds_handle.clone());
        let select_amount_handle = next_handle();
        output_handles.push(select_amount_handle.clone());
        let new_to_amount_handle = next_handle();
        output_handles.push(new_to_amount_handle.clone());
        let new_from_amount_handle = next_handle();
        output_handles.push(new_from_amount_handle.clone());

        async_computations.push(AsyncComputation {
            operation: FheOperation::FheGe.into(),
            transaction_id: transaction_id.clone(),
            output_handle: has_enough_funds_handle.clone(),
            inputs: vec![bals.clone(), trxa.clone()],
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheCast.into(),
            transaction_id: transaction_id.clone(),
            output_handle: cast_has_enough_funds_handle.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(has_enough_funds_handle.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![5u8])),
                },
            ],
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheMul.into(),
            transaction_id: transaction_id.clone(),
            output_handle: select_amount_handle.clone(),
            inputs: vec![
                trxa.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(cast_has_enough_funds_handle.clone())),
                },
            ],
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_handle.clone(),
            inputs: vec![
                bald.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(select_amount_handle.clone())),
                },
            ],
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheSub.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_from_amount_handle.clone(),
            inputs: vec![
                bals.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(select_amount_handle.clone())),
                },
            ],
        });

        bald = AsyncComputationInput {
            input: Some(Input::InputHandle(new_to_amount_handle.clone())),
        };
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

    for h in output_handles.iter() {
        allow_handle(h, &pool).await?;
    }

    println!("Computations scheduled, waiting upon completion...");
    wait_until_all_allowed_handles_computed(&app).await?;

    let decrypt_request = output_handles.clone();
    let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;

    assert_eq!(
        resp.len(),
        output_handles.len(),
        "Outputs length doesn't match"
    );
    for (i, r) in resp.iter().enumerate() {
        let to_bal = (20 + (i / 5 + 1) * 10).to_string();
        match r.value.as_str() {
            "true" if i % 5 == 0 => (), // trxa <= bals true
            "1" if i % 5 == 1 => (),    // cast
            "10" if i % 5 == 2 => (),   // select trxa * cast
            val if i % 5 == 3 => assert_eq!(val, to_bal, "Destination balances don't match."), // bals + selected trxa
            "90" if i % 5 == 4 => (), // bald - selected trxa
            s => panic!("unexpected result: {} for output {i}", s),
        }
    }
    Ok(())
}

#[tokio::test]
async fn counter_increment() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut output_handles = vec![];
    let mut async_computations = vec![];
    let mut num_samples: usize = 2;
    let samples = std::env::var("FHEVM_TEST_NUM_SAMPLES");
    if let Ok(samples) = samples {
        num_samples = samples.parse::<usize>().unwrap();
    }

    let keys = query_tenant_keys(vec![default_tenant_id()], &pool)
        .await
        .map_err(|e| {
            let e: Box<dyn std::error::Error> = e;
            e
        })?;
    let keys = &keys[0];

    let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.pks);
    let the_list = builder
        .push(42_u64) // Initial counter value
        .build_with_proof_packed(&keys.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
        .unwrap();
    let serialized = safe_serialize(&the_list);
    let mut input_request = tonic::Request::new(InputUploadBatch {
        input_ciphertexts: vec![InputToUpload {
            input_payload: serialized,
            signatures: Vec::new(),
            user_address: test_random_user_address(),
            contract_address: test_random_contract_address(),
        }],
    });
    input_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let resp = client.upload_inputs(input_request).await?;
    let resp = resp.get_ref();
    assert_eq!(resp.upload_responses.len(), 1);
    let first_resp = &resp.upload_responses[0];
    assert_eq!(first_resp.input_handles.len(), 1);
    let handle_counter = first_resp.input_handles[0].handle.clone();
    let mut counter = AsyncComputationInput {
        input: Some(Input::InputHandle(handle_counter.clone())),
    };

    for _ in 0..=(num_samples - 1) as u32 {
        let transaction_id = next_handle();
        let new_counter = next_handle();
        output_handles.push(new_counter.clone());

        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_counter.clone(),
            inputs: vec![
                counter.clone(),
                // Counter increment
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![7u8])),
                },
            ],
        });

        counter = AsyncComputationInput {
            input: Some(Input::InputHandle(new_counter.clone())),
        };
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

    for h in output_handles.iter() {
        allow_handle(h, &pool).await?;
    }
    println!("Computations scheduled, waiting upon completion...");
    wait_until_all_allowed_handles_computed(&app).await?;

    let decrypt_request = output_handles.clone();
    let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;

    assert_eq!(
        resp.len(),
        output_handles.len(),
        "Outputs length doesn't match"
    );
    for (i, r) in resp.iter().enumerate() {
        let target = (42 + (i + 1) * 7).to_string();
        assert_eq!(r.value.as_str(), target, "Counter value incorrect.");
    }
    Ok(())
}

#[tokio::test]
async fn tree_reduction() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut output_handles = vec![];
    let mut async_computations = vec![];
    let mut num_samples: usize = 16;
    let samples = std::env::var("FHEVM_TEST_NUM_SAMPLES");
    if let Ok(samples) = samples {
        num_samples = samples.parse::<usize>().unwrap();
    }

    let keys = query_tenant_keys(vec![default_tenant_id()], &pool)
        .await
        .map_err(|e| {
            let e: Box<dyn std::error::Error> = e;
            e
        })?;
    let keys = &keys[0];

    let num_levels = (num_samples as f64).log2().ceil() as usize;
    let mut num_comps_at_level = 2f64.powi((num_levels - 1) as i32) as usize;
    let target = num_comps_at_level * 2;
    let mut level_inputs = vec![];
    let mut level_outputs = vec![];

    for _ in 0..num_comps_at_level {
        let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.pks);
        let the_list = builder
            .push(1_u64) // Initial value
            .push(1_u64) // Initial value
            .build_with_proof_packed(&keys.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
            .unwrap();
        let serialized = safe_serialize(&the_list);
        let mut input_request = tonic::Request::new(InputUploadBatch {
            input_ciphertexts: vec![InputToUpload {
                input_payload: serialized,
                signatures: Vec::new(),
                user_address: test_random_user_address(),
                contract_address: test_random_contract_address(),
            }],
        });
        input_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        let resp = client.upload_inputs(input_request).await?;
        let resp = resp.get_ref();
        assert_eq!(resp.upload_responses.len(), 1);
        let first_resp = &resp.upload_responses[0];
        assert_eq!(first_resp.input_handles.len(), 2);
        let lhs_handle = first_resp.input_handles[0].handle.clone();
        let rhs_handle = first_resp.input_handles[1].handle.clone();
        level_inputs.push(AsyncComputationInput {
            input: Some(Input::InputHandle(lhs_handle.clone())),
        });
        level_inputs.push(AsyncComputationInput {
            input: Some(Input::InputHandle(rhs_handle.clone())),
        });
    }
    let mut output_handle = next_handle();
    let transaction_id = next_handle();
    for _ in 0..num_levels {
        for i in 0..num_comps_at_level {
            output_handle = next_handle();
            level_outputs.push(AsyncComputationInput {
                input: Some(Input::InputHandle(output_handle.clone())),
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheAdd.into(),
                transaction_id: transaction_id.clone(),
                output_handle: output_handle.clone(),
                inputs: vec![level_inputs[2 * i].clone(), level_inputs[2 * i + 1].clone()],
            });
        }
        num_comps_at_level /= 2;
        if num_comps_at_level < 1 {
            break;
        }
        level_inputs = std::mem::take(&mut level_outputs);
    }
    output_handles.push(output_handle);

    println!("Scheduling computations...");
    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.async_compute(compute_request).await?;

    for h in output_handles.iter() {
        allow_handle(h, &pool).await?;
    }

    println!("Computations scheduled, waiting upon completion...");
    wait_until_all_allowed_handles_computed(&app).await?;

    let decrypt_request = output_handles.clone();
    let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;

    assert_eq!(
        resp.len(),
        output_handles.len(),
        "Outputs length doesn't match"
    );
    assert_eq!(
        resp[0].value.as_str(),
        target.to_string(),
        "Incorrect result."
    );
    Ok(())
}
