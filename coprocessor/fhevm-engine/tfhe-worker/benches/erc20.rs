#[path = "./utils.rs"]
mod utils;
use crate::utils::{
    default_api_key, default_tenant_id, query_tenant_keys, random_handle, setup_test_app,
    wait_until_all_allowed_handles_computed, write_to_json, EnvConfig, OperatorType,
};
use criterion::{
    async_executor::FuturesExecutor, measurement::WallTime, Bencher, Criterion, Throughput,
};
use fhevm_engine_common::utils::safe_serialize;
use std::str::FromStr;
use std::time::SystemTime;
use tfhe_worker::server::common::FheOperation;
use tfhe_worker::server::tfhe_worker::{async_computation_input::Input, AsyncComputationInput};
use tfhe_worker::server::tfhe_worker::{
    fhevm_coprocessor_client::FhevmCoprocessorClient, AsyncComputation, AsyncComputeRequest,
    InputToUpload, InputUploadBatch,
};

use tfhe_worker::tfhe_worker::TIMING;
use tokio::runtime::Runtime;
use tonic::metadata::MetadataValue;

fn test_random_user_address() -> String {
    let _private_key = "bd2400c676871534a682ca1c5e4cd647ec9c3e122f188c6e3f54e6900d586c7b";
    let public_key = "0x1BdA2a485c339C95a9AbfDe52E80ca38e34C199E";
    public_key.to_string()
}

fn test_random_contract_address() -> String {
    "0x76c222560Db6b8937B291196eAb4Dad8930043aE".to_string()
}

fn main() {
    let ecfg = EnvConfig::new();
    let mut c = Criterion::default()
        .sample_size(10)
        .measurement_time(std::time::Duration::from_secs(1000))
        .configure_from_args();
    let bench_name = "erc20::transfer";
    let bench_optimization_target = if cfg!(feature = "latency") {
        "opt_latency"
    } else {
        "opt_throughput"
    };

    let mut group = c.benchmark_group(bench_name);
    if ecfg.benchmark_type == "LATENCY" || ecfg.benchmark_type == "ALL" {
        let num_elems = 1;
        let bench_id = format!("{bench_name}::latency::whitepaper::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            let _ = Runtime::new().unwrap().block_on(schedule_erc20_whitepaper(
                b,
                num_elems as usize,
                bench_id.clone(),
            ));
        });

        let bench_id = format!("{bench_name}::latency::no_cmux::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            let _ = Runtime::new().unwrap().block_on(schedule_erc20_no_cmux(
                b,
                num_elems as usize,
                bench_id.clone(),
            ));
        });
    }

    if ecfg.benchmark_type == "THROUGHPUT" || ecfg.benchmark_type == "ALL" {
        for num_elems in [10, 50, 200, 500] {
            group.throughput(Throughput::Elements(num_elems));
            let bench_id =
                format!("{bench_name}::throughput::whitepaper::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new().unwrap().block_on(schedule_erc20_whitepaper(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ));
            });

            group.throughput(Throughput::Elements(num_elems));
            let bench_id =
                format!("{bench_name}::throughput::no_cmux::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new().unwrap().block_on(schedule_erc20_no_cmux(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ));
            });

            group.throughput(Throughput::Elements(num_elems));
            let bench_id = format!(
                "{bench_name}::throughput::dependent_whitepaper::FHEUint64::{num_elems}_elems::{bench_optimization_target}"
            );
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new()
                    .unwrap()
                    .block_on(schedule_dependent_erc20_whitepaper(
                        b,
                        num_elems as usize,
                        bench_id.clone(),
                    ));
            });
            group.throughput(Throughput::Elements(num_elems));
            let bench_id = format!(
                "{bench_name}::throughput::dependent_no_cmux::FHEUint64::{num_elems}_elems::{bench_optimization_target}"
            );
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new()
                    .unwrap()
                    .block_on(schedule_dependent_erc20_no_cmux(
                        b,
                        num_elems as usize,
                        bench_id.clone(),
                    ));
            });
        }
    }
    group.finish();

    c.final_summary();
}

async fn schedule_erc20_whitepaper(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
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
    let mut num_samples: usize = num_tx;
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
        .push(100_u64) // Balance source
        .push(10_u64) // Transfer amount
        .push(20_u64) // Balance destination
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
    assert_eq!(first_resp.input_handles.len(), 3);

    for _ in 0..=(num_samples - 1) as u32 {
        let transaction_id = next_handle();
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
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_target_handle.clone(),
            inputs: vec![bald.clone(), trxa.clone()],
            is_allowed: false,
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
            is_allowed: true,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheSub.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_from_amount_target_handle.clone(),
            inputs: vec![bals.clone(), trxa.clone()],
            is_allowed: false,
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
            is_allowed: true,
        });
    }

    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.clone().async_compute(compute_request).await.unwrap();
    let app_ref = &app;
    bencher
        .to_async(FuturesExecutor)
        .iter_custom(|iters| async move {
            let db_url = app_ref.db_url().to_string();
            let now = SystemTime::now();
            let _ = tokio::task::spawn_blocking(move || {
                Runtime::new().unwrap().block_on(async {
                    wait_until_all_allowed_handles_computed(db_url)
                        .await
                        .unwrap()
                });
                println!(
                    "Execution time: {} -- {}",
                    now.elapsed().unwrap().as_millis(),
                    TIMING.load(std::sync::atomic::Ordering::SeqCst) / 1000
                );
            })
            .await;
            std::time::Duration::from_micros(
                TIMING.swap(0, std::sync::atomic::Ordering::SeqCst) * iters.max(1),
            )
        });

    let params = keys.cks.computation_parameters();
    write_to_json::<u64, _>(
        &bench_id,
        params,
        "",
        "erc20-transfer",
        &OperatorType::Atomic,
        64,
        vec![],
    );

    Ok(())
}

async fn schedule_erc20_no_cmux(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
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
    let mut num_samples: usize = num_tx;
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
        .push(100_u64) // Balance source
        .push(10_u64) // Transfer amount
        .push(20_u64) // Balance destination
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
    assert_eq!(first_resp.input_handles.len(), 3);

    for _ in 0..=(num_samples - 1) as u32 {
        let transaction_id = next_handle();
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
            is_allowed: false,
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
            is_allowed: false,
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
            is_allowed: false,
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
            is_allowed: true,
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
            is_allowed: true,
        });
    }

    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.clone().async_compute(compute_request).await.unwrap();
    let app_ref = &app;
    bencher
        .to_async(FuturesExecutor)
        .iter_custom(|iters| async move {
            let db_url = app_ref.db_url().to_string();
            let now = SystemTime::now();
            let _ = tokio::task::spawn_blocking(move || {
                Runtime::new().unwrap().block_on(async {
                    wait_until_all_allowed_handles_computed(db_url)
                        .await
                        .unwrap()
                });
                println!(
                    "Execution time: {} -- {}",
                    now.elapsed().unwrap().as_millis(),
                    TIMING.load(std::sync::atomic::Ordering::SeqCst) / 1000
                );
            })
            .await;
            std::time::Duration::from_micros(
                TIMING.swap(0, std::sync::atomic::Ordering::SeqCst) * iters.max(1),
            )
        });

    let params = keys.cks.computation_parameters();
    write_to_json::<u64, _>(
        &bench_id,
        params,
        "",
        "erc20-transfer",
        &OperatorType::Atomic,
        64,
        vec![],
    );
    Ok(())
}

async fn schedule_dependent_erc20_whitepaper(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
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
    let mut num_samples: usize = num_tx;
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

    let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.pks);
    let the_list = builder
        .push(100_u64) // Balance source
        .push(10_u64) // Transfer amount
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

    for _ in 0..=(num_samples - 1) as u32 {
        let transaction_id = next_handle();
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
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_target_handle.clone(),
            inputs: vec![bald.clone(), trxa.clone()],
            is_allowed: false,
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
            is_allowed: true,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheSub.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_from_amount_target_handle.clone(),
            inputs: vec![bals.clone(), trxa.clone()],
            is_allowed: false,
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
            is_allowed: true,
        });

        bald = AsyncComputationInput {
            input: Some(Input::InputHandle(new_to_amount_handle.clone())),
        };
    }

    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.clone().async_compute(compute_request).await.unwrap();
    let app_ref = &app;
    bencher
        .to_async(FuturesExecutor)
        .iter_custom(|iters| async move {
            let db_url = app_ref.db_url().to_string();
            let now = SystemTime::now();
            let _ = tokio::task::spawn_blocking(move || {
                Runtime::new().unwrap().block_on(async {
                    wait_until_all_allowed_handles_computed(db_url)
                        .await
                        .unwrap()
                });
                println!(
                    "Execution time: {} -- {}",
                    now.elapsed().unwrap().as_millis(),
                    TIMING.load(std::sync::atomic::Ordering::SeqCst) / 1000
                );
            })
            .await;
            std::time::Duration::from_micros(
                TIMING.swap(0, std::sync::atomic::Ordering::SeqCst) * iters.max(1),
            )
        });

    let params = keys.cks.computation_parameters();
    write_to_json::<u64, _>(
        &bench_id,
        params,
        "",
        "erc20-transfer",
        &OperatorType::Atomic,
        64,
        vec![],
    );

    Ok(())
}

async fn schedule_dependent_erc20_no_cmux(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
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
    let mut num_samples: usize = num_tx;
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

    let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.pks);
    let the_list = builder
        .push(100_u64) // Balance source
        .push(10_u64) // Transfer amount
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

    for _ in 0..=(num_samples - 1) as u32 {
        let transaction_id = next_handle();
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
            is_allowed: false,
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
            is_allowed: false,
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
            is_allowed: false,
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
            is_allowed: true,
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
            is_allowed: true,
        });

        bald = AsyncComputationInput {
            input: Some(Input::InputHandle(new_to_amount_handle.clone())),
        };
    }

    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.clone().async_compute(compute_request).await.unwrap();
    let app_ref = &app;
    bencher
        .to_async(FuturesExecutor)
        .iter_custom(|iters| async move {
            let db_url = app_ref.db_url().to_string();
            let now = SystemTime::now();
            let _ = tokio::task::spawn_blocking(move || {
                Runtime::new().unwrap().block_on(async {
                    wait_until_all_allowed_handles_computed(db_url)
                        .await
                        .unwrap()
                });
                println!(
                    "Execution time: {} -- {}",
                    now.elapsed().unwrap().as_millis(),
                    TIMING.load(std::sync::atomic::Ordering::SeqCst) / 1000
                );
            })
            .await;
            std::time::Duration::from_micros(
                TIMING.swap(0, std::sync::atomic::Ordering::SeqCst) * iters.max(1),
            )
        });

    let params = keys.cks.computation_parameters();
    write_to_json::<u64, _>(
        &bench_id,
        params,
        "",
        "erc20-transfer",
        &OperatorType::Atomic,
        64,
        vec![],
    );
    Ok(())
}
