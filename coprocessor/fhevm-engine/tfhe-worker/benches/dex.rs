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
    let bench_optimization_target = if cfg!(feature = "latency") {
        "opt_latency"
    } else {
        "opt_throughput"
    };

    let bench_name = "dex::swap_request";
    let mut group = c.benchmark_group(bench_name);
    if ecfg.benchmark_type == "LATENCY" || ecfg.benchmark_type == "ALL" {
        let num_elems = 1;
        let bench_id = format!("{bench_name}::latency::whitepaper::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            let _ = Runtime::new().unwrap().block_on(swap_request_whitepaper(
                b,
                num_elems as usize,
                bench_id.clone(),
            ));
        });

        let bench_id = format!("{bench_name}::latency::no_cmux::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            let _ = Runtime::new().unwrap().block_on(swap_request_no_cmux(
                b,
                num_elems as usize,
                bench_id.clone(),
            ));
        });
    }
    if ecfg.benchmark_type == "THROUGHPUT" || ecfg.benchmark_type == "ALL" {
        for num_elems in [10, 50] {
            group.throughput(Throughput::Elements(num_elems));
            let bench_id =
                format!("{bench_name}::throughput::whitepaper::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new().unwrap().block_on(swap_request_whitepaper(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ));
            });

            group.throughput(Throughput::Elements(num_elems));
            let bench_id =
                format!("{bench_name}::throughput::no_cmux::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new().unwrap().block_on(swap_request_no_cmux(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ));
            });
        }
    }
    group.finish();

    let bench_name = "dex::swap_claim";
    let mut group = c.benchmark_group(bench_name);
    if ecfg.benchmark_type == "LATENCY" || ecfg.benchmark_type == "ALL" {
        let num_elems = 1;
        let bench_id = format!("{bench_name}::latency::whitepaper::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            let _ = Runtime::new().unwrap().block_on(swap_claim_whitepaper(
                b,
                num_elems as usize,
                bench_id.clone(),
            ));
        });

        let bench_id = format!("{bench_name}::latency::no_cmux::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            let _ = Runtime::new().unwrap().block_on(swap_claim_no_cmux(
                b,
                num_elems as usize,
                bench_id.clone(),
            ));
        });
    }
    if ecfg.benchmark_type == "THROUGHPUT" || ecfg.benchmark_type == "ALL" {
        for num_elems in [10, 50] {
            group.throughput(Throughput::Elements(num_elems));
            let bench_id =
                format!("{bench_name}::throughput::whitepaper::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new().unwrap().block_on(swap_claim_whitepaper(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ));
            });

            group.throughput(Throughput::Elements(num_elems));
            let bench_id =
                format!("{bench_name}::throughput::no_cmux::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new().unwrap().block_on(swap_claim_no_cmux(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ));
            });
        }
    }
    group.finish();

    if ecfg.benchmark_type == "THROUGHPUT" || ecfg.benchmark_type == "ALL" {
        let bench_name = "dex::swap_request_dep";
        let mut group = c.benchmark_group(bench_name);
        for num_elems in [10, 50] {
            group.throughput(Throughput::Elements(num_elems));
            let bench_id =
                format!("{bench_name}::throughput::whitepaper::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new()
                    .unwrap()
                    .block_on(swap_request_whitepaper_dep(
                        b,
                        num_elems as usize,
                        bench_id.clone(),
                    ));
            });

            group.throughput(Throughput::Elements(num_elems));
            let bench_id =
                format!("{bench_name}::throughput::no_cmux::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new().unwrap().block_on(swap_request_no_cmux_dep(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ));
            });
        }
        group.finish();

        let bench_name = "dex::swap_claim_dep";
        let mut group = c.benchmark_group(bench_name);
        for num_elems in [10, 50] {
            group.throughput(Throughput::Elements(num_elems));
            let bench_id =
                format!("{bench_name}::throughput::whitepaper::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new().unwrap().block_on(swap_claim_whitepaper_dep(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ));
            });

            group.throughput(Throughput::Elements(num_elems));
            let bench_id =
                format!("{bench_name}::throughput::no_cmux::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new().unwrap().block_on(swap_claim_no_cmux_dep(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ));
            });
        }
        group.finish();
    }
    c.final_summary();
}

async fn swap_request_whitepaper(
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
        .push(100_u64) // From balance 0
        .push(200_u64) // From balance 1
        .push(700_u64) // Current dex balance 0
        .push(900_u64) // Current dex balance 1
        .push(100_u64) // To balance 0
        .push(200_u64) // To balance 1
        .push(100_u64) // Total dex token in 0
        .push(200_u64) // Total dex token in 1
        .push(10_u64) // Amount 0
        .push(20_u64) // Amount 1
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
    assert_eq!(first_resp.input_handles.len(), 10);

    for _ in 0..=(num_samples - 1) as u32 {
        let transaction_id = next_handle();
        // Swap request inputs
        let from_balance_0 = first_resp.input_handles[0].handle.clone();
        let from_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(from_balance_0.clone())),
        };
        let from_balance_1 = first_resp.input_handles[1].handle.clone();
        let from_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(from_balance_1.clone())),
        };
        let current_dex_balance_0 = first_resp.input_handles[2].handle.clone();
        let current_dex_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(current_dex_balance_0.clone())),
        };
        let current_dex_balance_1 = first_resp.input_handles[3].handle.clone();
        let current_dex_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(current_dex_balance_1.clone())),
        };
        let to_balance_0 = first_resp.input_handles[4].handle.clone();
        let to_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(to_balance_0.clone())),
        };
        let to_balance_1 = first_resp.input_handles[5].handle.clone();
        let to_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(to_balance_1.clone())),
        };
        let total_dex_token_0_in = first_resp.input_handles[6].handle.clone();
        let total_dex_token_0_in = AsyncComputationInput {
            input: Some(Input::InputHandle(total_dex_token_0_in.clone())),
        };
        let total_dex_token_1_in = first_resp.input_handles[7].handle.clone();
        let total_dex_token_1_in = AsyncComputationInput {
            input: Some(Input::InputHandle(total_dex_token_1_in.clone())),
        };
        let amount_0 = first_resp.input_handles[8].handle.clone();
        let amount_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(amount_0.clone())),
        };
        let amount_1 = first_resp.input_handles[9].handle.clone();
        let amount_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(amount_1.clone())),
        };

        // First transfer
        let has_enough_funds_handle_0 = next_handle();
        let new_to_amount_target_handle_0 = next_handle();
        let new_to_amount_handle_0 = next_handle();
        let _new_from_amount_target_handle_0 = next_handle();
        let _new_from_amount_handle_0 = next_handle();
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheGe.into(),
            transaction_id: transaction_id.clone(),
            output_handle: has_enough_funds_handle_0.clone(),
            inputs: vec![from_balance_0.clone(), amount_0.clone()],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_target_handle_0.clone(),
            inputs: vec![current_dex_balance_0.clone(), amount_0.clone()],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheIfThenElse.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_handle_0.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(has_enough_funds_handle_0.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(new_to_amount_target_handle_0.clone())),
                },
                current_dex_balance_0.clone(),
            ],
            is_allowed: false,
        });
        // async_computations.push(AsyncComputation {
        //     operation: FheOperation::FheSub.into(),
        //     output_handle: new_from_amount_target_handle_0.clone(),
        //     inputs: vec![from_balance_0.clone(), amount_0.clone()],
        // });
        // async_computations.push(AsyncComputation {
        //     operation: FheOperation::FheIfThenElse.into(),
        //     output_handle: new_from_amount_handle_0.clone(),
        //     inputs: vec![
        //         AsyncComputationInput {
        //             input: Some(Input::InputHandle(has_enough_funds_handle_0.clone())),
        //         },
        //         AsyncComputationInput {
        //             input: Some(Input::InputHandle(new_from_amount_target_handle_0.clone())),
        //         },
        //         from_balance_0.clone(),
        //     ],
        // });
        // Second transfer
        let has_enough_funds_handle_1 = next_handle();
        let new_to_amount_target_handle_1 = next_handle();
        let new_to_amount_handle_1 = next_handle();
        let _new_from_amount_target_handle_1 = next_handle();
        let _new_from_amount_handle_1 = next_handle();
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheGe.into(),
            transaction_id: transaction_id.clone(),
            output_handle: has_enough_funds_handle_1.clone(),
            inputs: vec![from_balance_1.clone(), amount_1.clone()],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_target_handle_1.clone(),
            inputs: vec![current_dex_balance_1.clone(), amount_1.clone()],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheIfThenElse.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_handle_1.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(has_enough_funds_handle_1.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(new_to_amount_target_handle_1.clone())),
                },
                current_dex_balance_1.clone(),
            ],
            is_allowed: false,
        });
        // async_computations.push(AsyncComputation {
        //     operation: FheOperation::FheSub.into(),
        //     output_handle: new_from_amount_target_handle_1.clone(),
        //     inputs: vec![from_balance_1.clone(), amount_1.clone()],
        // });
        // async_computations.push(AsyncComputation {
        //     operation: FheOperation::FheIfThenElse.into(),
        //     output_handle: new_from_amount_handle_1.clone(),
        //     inputs: vec![
        //         AsyncComputationInput {
        //             input: Some(Input::InputHandle(has_enough_funds_handle_1.clone())),
        //         },
        //         AsyncComputationInput {
        //             input: Some(Input::InputHandle(new_from_amount_target_handle_1.clone())),
        //         },
        //         from_balance_1.clone(),
        //     ],
        // });
        let new_current_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(new_to_amount_handle_0.clone())),
        };
        let new_current_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(new_to_amount_handle_1.clone())),
        };
        let sent_0_handle = next_handle();
        let sent_1_handle = next_handle();
        let pending_0_in_handle = next_handle();
        let pending_1_in_handle = next_handle();
        let pending_total_token_0_in = next_handle();
        let pending_total_token_1_in = next_handle();
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheSub.into(),
            transaction_id: transaction_id.clone(),
            output_handle: sent_0_handle.clone(),
            inputs: vec![new_current_balance_0.clone(), current_dex_balance_0.clone()],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheSub.into(),
            transaction_id: transaction_id.clone(),
            output_handle: sent_1_handle.clone(),
            inputs: vec![new_current_balance_1.clone(), current_dex_balance_1.clone()],
            is_allowed: false,
        });
        let sent_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(sent_0_handle.clone())),
        };
        let sent_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(sent_1_handle.clone())),
        };
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: pending_0_in_handle.clone(),
            inputs: vec![to_balance_0.clone(), sent_0.clone()],
            is_allowed: true,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: pending_1_in_handle.clone(),
            inputs: vec![to_balance_1.clone(), sent_1.clone()],
            is_allowed: true,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: pending_total_token_0_in.clone(),
            inputs: vec![total_dex_token_0_in.clone(), sent_0.clone()],
            is_allowed: true,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: pending_total_token_1_in.clone(),
            inputs: vec![total_dex_token_1_in.clone(), sent_1.clone()],
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
    let _resp = client.async_compute(compute_request).await.unwrap();
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
        "swap-request",
        &OperatorType::Atomic,
        64,
        vec![],
    );

    Ok(())
}

async fn swap_request_no_cmux(
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
        .push(100_u64) // From balance 0
        .push(200_u64) // From balance 1
        .push(700_u64) // Current dex balance 0
        .push(900_u64) // Current dex balance 1
        .push(100_u64) // To balance 0
        .push(200_u64) // To balance 1
        .push(100_u64) // Total dex token in 0
        .push(200_u64) // Total dex token in 1
        .push(10_u64) // Amount 0
        .push(20_u64) // Amount 1
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
    assert_eq!(first_resp.input_handles.len(), 10);

    for _ in 0..=(num_samples - 1) as u32 {
        let transaction_id = next_handle();
        // Swap request inputs
        let from_balance_0 = first_resp.input_handles[0].handle.clone();
        let from_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(from_balance_0.clone())),
        };
        let from_balance_1 = first_resp.input_handles[1].handle.clone();
        let from_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(from_balance_1.clone())),
        };
        let current_dex_balance_0 = first_resp.input_handles[2].handle.clone();
        let current_dex_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(current_dex_balance_0.clone())),
        };
        let current_dex_balance_1 = first_resp.input_handles[3].handle.clone();
        let current_dex_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(current_dex_balance_1.clone())),
        };
        let to_balance_0 = first_resp.input_handles[4].handle.clone();
        let to_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(to_balance_0.clone())),
        };
        let to_balance_1 = first_resp.input_handles[5].handle.clone();
        let to_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(to_balance_1.clone())),
        };
        let total_dex_token_0_in = first_resp.input_handles[6].handle.clone();
        let total_dex_token_0_in = AsyncComputationInput {
            input: Some(Input::InputHandle(total_dex_token_0_in.clone())),
        };
        let total_dex_token_1_in = first_resp.input_handles[7].handle.clone();
        let total_dex_token_1_in = AsyncComputationInput {
            input: Some(Input::InputHandle(total_dex_token_1_in.clone())),
        };
        let amount_0 = first_resp.input_handles[8].handle.clone();
        let amount_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(amount_0.clone())),
        };
        let amount_1 = first_resp.input_handles[9].handle.clone();
        let amount_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(amount_1.clone())),
        };

        // First transfer
        let has_enough_funds_handle_0 = next_handle();
        let cast_has_enough_funds_handle_0 = next_handle();
        let select_amount_handle_0 = next_handle();
        let new_to_amount_handle_0 = next_handle();
        let _new_from_amount_handle_0 = next_handle();
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheGe.into(),
            transaction_id: transaction_id.clone(),
            output_handle: has_enough_funds_handle_0.clone(),
            inputs: vec![from_balance_0.clone(), amount_0.clone()],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheCast.into(),
            transaction_id: transaction_id.clone(),
            output_handle: cast_has_enough_funds_handle_0.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(has_enough_funds_handle_0.clone())),
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
            output_handle: select_amount_handle_0.clone(),
            inputs: vec![
                amount_0.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(cast_has_enough_funds_handle_0.clone())),
                },
            ],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_handle_0.clone(),
            inputs: vec![
                current_dex_balance_0.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(select_amount_handle_0.clone())),
                },
            ],
            is_allowed: false,
        });
        // async_computations.push(AsyncComputation {
        //     operation: FheOperation::FheSub.into(),
        //     output_handle: new_from_amount_handle_0.clone(),
        //     inputs: vec![
        //         from_balance_0.clone(),
        //         AsyncComputationInput {
        //             input: Some(Input::InputHandle(select_amount_handle_0.clone())),
        //         },
        //     ],
        // });

        // Second transfer
        let has_enough_funds_handle_1 = next_handle();
        let cast_has_enough_funds_handle_1 = next_handle();
        let select_amount_handle_1 = next_handle();
        let new_to_amount_handle_1 = next_handle();
        let _new_from_amount_handle_1 = next_handle();
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheGe.into(),
            transaction_id: transaction_id.clone(),
            output_handle: has_enough_funds_handle_1.clone(),
            inputs: vec![from_balance_1.clone(), amount_1.clone()],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheCast.into(),
            transaction_id: transaction_id.clone(),
            output_handle: cast_has_enough_funds_handle_1.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(has_enough_funds_handle_1.clone())),
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
            output_handle: select_amount_handle_1.clone(),
            inputs: vec![
                amount_1.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(cast_has_enough_funds_handle_1.clone())),
                },
            ],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_handle_1.clone(),
            inputs: vec![
                current_dex_balance_1.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(select_amount_handle_1.clone())),
                },
            ],
            is_allowed: false,
        });
        // async_computations.push(AsyncComputation {
        //     operation: FheOperation::FheSub.into(),
        //     output_handle: new_from_amount_handle_1.clone(),
        //     inputs: vec![
        //         from_balance_1.clone(),
        //         AsyncComputationInput {
        //             input: Some(Input::InputHandle(select_amount_handle_1.clone())),
        //         },
        //     ],
        // });

        let new_current_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(new_to_amount_handle_0.clone())),
        };
        let new_current_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(new_to_amount_handle_1.clone())),
        };
        let sent_0_handle = next_handle();
        let sent_1_handle = next_handle();
        let pending_0_in_handle = next_handle();
        let pending_1_in_handle = next_handle();
        let pending_total_token_0_in = next_handle();
        let pending_total_token_1_in = next_handle();
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheSub.into(),
            transaction_id: transaction_id.clone(),
            output_handle: sent_0_handle.clone(),
            inputs: vec![new_current_balance_0.clone(), current_dex_balance_0.clone()],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheSub.into(),
            transaction_id: transaction_id.clone(),
            output_handle: sent_1_handle.clone(),
            inputs: vec![new_current_balance_1.clone(), current_dex_balance_1.clone()],
            is_allowed: false,
        });
        let sent_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(sent_0_handle.clone())),
        };
        let sent_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(sent_1_handle.clone())),
        };
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: pending_0_in_handle.clone(),
            inputs: vec![to_balance_0.clone(), sent_0.clone()],
            is_allowed: true,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: pending_1_in_handle.clone(),
            inputs: vec![to_balance_1.clone(), sent_1.clone()],
            is_allowed: true,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: pending_total_token_0_in.clone(),
            inputs: vec![total_dex_token_0_in.clone(), sent_0.clone()],
            is_allowed: true,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: pending_total_token_1_in.clone(),
            inputs: vec![total_dex_token_1_in.clone(), sent_1.clone()],
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
    let _resp = client.async_compute(compute_request).await.unwrap();
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
        "swap-request",
        &OperatorType::Atomic,
        64,
        vec![],
    );

    Ok(())
}

async fn swap_claim_whitepaper(
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
        .push(100_u64) // Pending 0 in
        .push(200_u64) // Pending 1 in
        .push(700_u64) // Old balance 0
        .push(900_u64) // Old balance 1
        .push(100_u64) // Current dex balance 0
        .push(200_u64) // Current dex balance  1
        .build_with_proof_packed(&keys.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
        .unwrap();
    let total_dex_token_0_in: u64 = 300;
    let total_dex_token_1_in: u64 = 600;
    let total_dex_token_0_out: u64 = 100;
    let total_dex_token_1_out: u64 = 200;

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
    assert_eq!(first_resp.input_handles.len(), 6);

    for _ in 0..=(num_samples - 1) as u32 {
        let transaction_id = next_handle();
        // Swap claim inputs
        let pending_0_in = first_resp.input_handles[0].handle.clone();
        let pending_0_in = AsyncComputationInput {
            input: Some(Input::InputHandle(pending_0_in.clone())),
        };
        let pending_1_in = first_resp.input_handles[1].handle.clone();
        let pending_1_in = AsyncComputationInput {
            input: Some(Input::InputHandle(pending_1_in.clone())),
        };
        let old_balance_0 = first_resp.input_handles[2].handle.clone();
        let old_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(old_balance_0.clone())),
        };
        let old_balance_1 = first_resp.input_handles[3].handle.clone();
        let old_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(old_balance_1.clone())),
        };
        let current_dex_balance_0 = first_resp.input_handles[4].handle.clone();
        let current_dex_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(current_dex_balance_0.clone())),
        };
        let current_dex_balance_1 = first_resp.input_handles[5].handle.clone();
        let current_dex_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(current_dex_balance_1.clone())),
        };

        let big_pending_0_in = next_handle();
        let big_pending_1_in = next_handle();
        let big_amount_0_out = next_handle();
        let big_amount_1_out = next_handle();
        let amount_0_out = next_handle();
        let amount_1_out = next_handle();
        let _new_balance_0 = next_handle();
        let _new_balance_1 = next_handle();
        if total_dex_token_1_in != 0 {
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: big_pending_1_in.clone(),
                inputs: vec![
                    pending_1_in,
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![6u8])),
                    },
                ],
                is_allowed: false,
            });
            let mul_temp = next_handle();
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheMul.into(),
                transaction_id: transaction_id.clone(),
                output_handle: mul_temp.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(big_pending_1_in.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(
                            (total_dex_token_0_out as u128).to_be_bytes().to_vec(),
                        )),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheDiv.into(),
                transaction_id: transaction_id.clone(),
                output_handle: big_amount_0_out.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(mul_temp.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(
                            (total_dex_token_1_in as u128).to_be_bytes().to_vec(),
                        )),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: amount_0_out.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(big_amount_0_out.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![5u8])),
                    },
                ],
                is_allowed: false,
            });
            // Transfer
            let has_enough_funds_handle_0 = next_handle();
            let new_to_amount_target_handle_0 = next_handle();
            let new_to_amount_handle_0 = next_handle();
            let new_from_amount_target_handle_0 = next_handle();
            let new_from_amount_handle_0 = next_handle();
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheGe.into(),
                transaction_id: transaction_id.clone(),
                output_handle: has_enough_funds_handle_0.clone(),
                inputs: vec![
                    current_dex_balance_0.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_0_out.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheAdd.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_to_amount_target_handle_0.clone(),
                inputs: vec![
                    old_balance_0.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_0_out.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheIfThenElse.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_to_amount_handle_0.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(has_enough_funds_handle_0.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(new_to_amount_target_handle_0.clone())),
                    },
                    old_balance_0.clone(),
                ],
                is_allowed: true,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheSub.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_from_amount_target_handle_0.clone(),
                inputs: vec![
                    current_dex_balance_0.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_0_out.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheIfThenElse.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_from_amount_handle_0.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(has_enough_funds_handle_0.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(new_from_amount_target_handle_0.clone())),
                    },
                    current_dex_balance_0.clone(),
                ],
                is_allowed: true,
            });
        }
        if total_dex_token_0_in != 0 {
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: big_pending_0_in.clone(),
                inputs: vec![
                    pending_0_in,
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![6u8])),
                    },
                ],
                is_allowed: false,
            });
            let mul_temp = next_handle();
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheMul.into(),
                transaction_id: transaction_id.clone(),
                output_handle: mul_temp.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(big_pending_0_in.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(
                            (total_dex_token_1_out as u128).to_be_bytes().to_vec(),
                        )),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheDiv.into(),
                transaction_id: transaction_id.clone(),
                output_handle: big_amount_1_out.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(mul_temp.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(
                            (total_dex_token_0_in as u128).to_be_bytes().to_vec(),
                        )),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: amount_1_out.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(big_amount_1_out.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![5u8])),
                    },
                ],
                is_allowed: false,
            });
            // Transfer
            let has_enough_funds_handle_1 = next_handle();
            let new_to_amount_target_handle_1 = next_handle();
            let new_to_amount_handle_1 = next_handle();
            let new_from_amount_target_handle_1 = next_handle();
            let new_from_amount_handle_1 = next_handle();
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheGe.into(),
                transaction_id: transaction_id.clone(),
                output_handle: has_enough_funds_handle_1.clone(),
                inputs: vec![
                    current_dex_balance_1.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_1_out.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheAdd.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_to_amount_target_handle_1.clone(),
                inputs: vec![
                    old_balance_1.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_1_out.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheIfThenElse.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_to_amount_handle_1.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(has_enough_funds_handle_1.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(new_to_amount_target_handle_1.clone())),
                    },
                    old_balance_1.clone(),
                ],
                is_allowed: true,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheSub.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_from_amount_target_handle_1.clone(),
                inputs: vec![
                    current_dex_balance_1.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_1_out.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheIfThenElse.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_from_amount_handle_1.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(has_enough_funds_handle_1.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(new_from_amount_target_handle_1.clone())),
                    },
                    current_dex_balance_1.clone(),
                ],
                is_allowed: true,
            });
        }
    }

    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.async_compute(compute_request).await.unwrap();
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
        "swap-request",
        &OperatorType::Atomic,
        64,
        vec![],
    );

    Ok(())
}

async fn swap_claim_no_cmux(
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
        .push(100_u64) // Pending 0 in
        .push(200_u64) // Pending 1 in
        .push(700_u64) // Old balance 0
        .push(900_u64) // Old balance 1
        .push(100_u64) // Current dex balance 0
        .push(200_u64) // Current dex balance  1
        .build_with_proof_packed(&keys.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
        .unwrap();
    let total_dex_token_0_in: u64 = 300;
    let total_dex_token_1_in: u64 = 600;
    let total_dex_token_0_out: u64 = 100;
    let total_dex_token_1_out: u64 = 200;

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
    assert_eq!(first_resp.input_handles.len(), 6);

    for _ in 0..=(num_samples - 1) as u32 {
        let transaction_id = next_handle();
        // Swap claim inputs
        let pending_0_in = first_resp.input_handles[0].handle.clone();
        let pending_0_in = AsyncComputationInput {
            input: Some(Input::InputHandle(pending_0_in.clone())),
        };
        let pending_1_in = first_resp.input_handles[1].handle.clone();
        let pending_1_in = AsyncComputationInput {
            input: Some(Input::InputHandle(pending_1_in.clone())),
        };
        let old_balance_0 = first_resp.input_handles[2].handle.clone();
        let old_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(old_balance_0.clone())),
        };
        let old_balance_1 = first_resp.input_handles[3].handle.clone();
        let old_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(old_balance_1.clone())),
        };
        let current_dex_balance_0 = first_resp.input_handles[4].handle.clone();
        let current_dex_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(current_dex_balance_0.clone())),
        };
        let current_dex_balance_1 = first_resp.input_handles[5].handle.clone();
        let current_dex_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(current_dex_balance_1.clone())),
        };

        let big_pending_0_in = next_handle();
        let big_pending_1_in = next_handle();
        let big_amount_0_out = next_handle();
        let big_amount_1_out = next_handle();
        let amount_0_out = next_handle();
        let amount_1_out = next_handle();
        let _new_balance_0 = next_handle();
        let _new_balance_1 = next_handle();
        if total_dex_token_1_in != 0 {
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: big_pending_1_in.clone(),
                inputs: vec![
                    pending_1_in,
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![6u8])),
                    },
                ],
                is_allowed: false,
            });
            let mul_temp = next_handle();
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheMul.into(),
                transaction_id: transaction_id.clone(),
                output_handle: mul_temp.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(big_pending_1_in.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(
                            (total_dex_token_0_out as u128).to_be_bytes().to_vec(),
                        )),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheDiv.into(),
                transaction_id: transaction_id.clone(),
                output_handle: big_amount_0_out.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(mul_temp.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(
                            (total_dex_token_1_in as u128).to_be_bytes().to_vec(),
                        )),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: amount_0_out.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(big_amount_0_out.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![5u8])),
                    },
                ],
                is_allowed: false,
            });

            // Transfer
            let has_enough_funds_handle_0 = next_handle();
            let cast_has_enough_funds_handle_0 = next_handle();
            let select_amount_handle_0 = next_handle();
            let new_to_amount_handle_0 = next_handle();
            let new_from_amount_handle_0 = next_handle();
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheGe.into(),
                transaction_id: transaction_id.clone(),
                output_handle: has_enough_funds_handle_0.clone(),
                inputs: vec![
                    current_dex_balance_0.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_0_out.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: cast_has_enough_funds_handle_0.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(has_enough_funds_handle_0.clone())),
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
                output_handle: select_amount_handle_0.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_0_out.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(cast_has_enough_funds_handle_0.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheAdd.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_to_amount_handle_0.clone(),
                inputs: vec![
                    old_balance_0.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(select_amount_handle_0.clone())),
                    },
                ],
                is_allowed: true,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheSub.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_from_amount_handle_0.clone(),
                inputs: vec![
                    current_dex_balance_0.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(select_amount_handle_0.clone())),
                    },
                ],
                is_allowed: true,
            });
        }

        if total_dex_token_0_in != 0 {
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: big_pending_0_in.clone(),
                inputs: vec![
                    pending_0_in,
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![6u8])),
                    },
                ],
                is_allowed: false,
            });
            let mul_temp = next_handle();
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheMul.into(),
                transaction_id: transaction_id.clone(),
                output_handle: mul_temp.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(big_pending_0_in.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(
                            (total_dex_token_1_out as u128).to_be_bytes().to_vec(),
                        )),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheDiv.into(),
                transaction_id: transaction_id.clone(),
                output_handle: big_amount_1_out.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(mul_temp.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(
                            (total_dex_token_0_in as u128).to_be_bytes().to_vec(),
                        )),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: amount_1_out.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(big_amount_1_out.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![5u8])),
                    },
                ],
                is_allowed: false,
            });
            // Transfer
            let has_enough_funds_handle_1 = next_handle();
            let cast_has_enough_funds_handle_1 = next_handle();
            let select_amount_handle_1 = next_handle();
            let new_to_amount_handle_1 = next_handle();
            let new_from_amount_handle_1 = next_handle();
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheGe.into(),
                transaction_id: transaction_id.clone(),
                output_handle: has_enough_funds_handle_1.clone(),
                inputs: vec![
                    current_dex_balance_1.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_1_out.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: cast_has_enough_funds_handle_1.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(has_enough_funds_handle_1.clone())),
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
                output_handle: select_amount_handle_1.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_1_out.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(cast_has_enough_funds_handle_1.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheAdd.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_to_amount_handle_1.clone(),
                inputs: vec![
                    old_balance_1.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(select_amount_handle_1.clone())),
                    },
                ],
                is_allowed: true,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheSub.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_from_amount_handle_1.clone(),
                inputs: vec![
                    current_dex_balance_1.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(select_amount_handle_1.clone())),
                    },
                ],
                is_allowed: true,
            });
        }
    }

    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.async_compute(compute_request).await.unwrap();
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
        "swap-request",
        &OperatorType::Atomic,
        64,
        vec![],
    );

    Ok(())
}

// DEPENDENT versions (single DEX)
async fn swap_request_whitepaper_dep(
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
        .push(100_u64) // From balance 0
        .push(200_u64) // From balance 1
        .push(700_u64) // Current dex balance 0
        .push(900_u64) // Current dex balance 1
        .push(100_u64) // To balance 0
        .push(200_u64) // To balance 1
        .push(100_u64) // Total dex token in 0
        .push(200_u64) // Total dex token in 1
        .push(10_u64) // Amount 0
        .push(20_u64) // Amount 1
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
    assert_eq!(first_resp.input_handles.len(), 10);

    // Dependence on DEX balances
    let current_dex_balance_0 = first_resp.input_handles[2].handle.clone();
    let mut current_dex_balance_0 = AsyncComputationInput {
        input: Some(Input::InputHandle(current_dex_balance_0.clone())),
    };
    let current_dex_balance_1 = first_resp.input_handles[3].handle.clone();
    let mut current_dex_balance_1 = AsyncComputationInput {
        input: Some(Input::InputHandle(current_dex_balance_1.clone())),
    };

    for _ in 0..=(num_samples - 1) as u32 {
        let transaction_id = next_handle();
        // Swap request inputs
        let from_balance_0 = first_resp.input_handles[0].handle.clone();
        let from_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(from_balance_0.clone())),
        };
        let from_balance_1 = first_resp.input_handles[1].handle.clone();
        let from_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(from_balance_1.clone())),
        };
        let to_balance_0 = first_resp.input_handles[4].handle.clone();
        let to_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(to_balance_0.clone())),
        };
        let to_balance_1 = first_resp.input_handles[5].handle.clone();
        let to_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(to_balance_1.clone())),
        };
        let total_dex_token_0_in = first_resp.input_handles[6].handle.clone();
        let total_dex_token_0_in = AsyncComputationInput {
            input: Some(Input::InputHandle(total_dex_token_0_in.clone())),
        };
        let total_dex_token_1_in = first_resp.input_handles[7].handle.clone();
        let total_dex_token_1_in = AsyncComputationInput {
            input: Some(Input::InputHandle(total_dex_token_1_in.clone())),
        };
        let amount_0 = first_resp.input_handles[8].handle.clone();
        let amount_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(amount_0.clone())),
        };
        let amount_1 = first_resp.input_handles[9].handle.clone();
        let amount_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(amount_1.clone())),
        };

        // First transfer
        let has_enough_funds_handle_0 = next_handle();
        let new_to_amount_target_handle_0 = next_handle();
        let new_to_amount_handle_0 = next_handle();
        let _new_from_amount_target_handle_0 = next_handle();
        let _new_from_amount_handle_0 = next_handle();
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheGe.into(),
            transaction_id: transaction_id.clone(),
            output_handle: has_enough_funds_handle_0.clone(),
            inputs: vec![from_balance_0.clone(), amount_0.clone()],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_target_handle_0.clone(),
            inputs: vec![current_dex_balance_0.clone(), amount_0.clone()],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheIfThenElse.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_handle_0.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(has_enough_funds_handle_0.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(new_to_amount_target_handle_0.clone())),
                },
                current_dex_balance_0.clone(),
            ],
            is_allowed: false,
        });
        // async_computations.push(AsyncComputation {
        //     operation: FheOperation::FheSub.into(),
        //     output_handle: new_from_amount_target_handle_0.clone(),
        //     inputs: vec![from_balance_0.clone(), amount_0.clone()],
        // });
        // async_computations.push(AsyncComputation {
        //     operation: FheOperation::FheIfThenElse.into(),
        //     output_handle: new_from_amount_handle_0.clone(),
        //     inputs: vec![
        //         AsyncComputationInput {
        //             input: Some(Input::InputHandle(has_enough_funds_handle_0.clone())),
        //         },
        //         AsyncComputationInput {
        //             input: Some(Input::InputHandle(new_from_amount_target_handle_0.clone())),
        //         },
        //         from_balance_0.clone(),
        //     ],
        // });
        // Second transfer
        let has_enough_funds_handle_1 = next_handle();
        let new_to_amount_target_handle_1 = next_handle();
        let new_to_amount_handle_1 = next_handle();
        let _new_from_amount_target_handle_1 = next_handle();
        let _new_from_amount_handle_1 = next_handle();
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheGe.into(),
            transaction_id: transaction_id.clone(),
            output_handle: has_enough_funds_handle_1.clone(),
            inputs: vec![from_balance_1.clone(), amount_1.clone()],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_target_handle_1.clone(),
            inputs: vec![current_dex_balance_1.clone(), amount_1.clone()],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheIfThenElse.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_handle_1.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(has_enough_funds_handle_1.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(new_to_amount_target_handle_1.clone())),
                },
                current_dex_balance_1.clone(),
            ],
            is_allowed: false,
        });
        // async_computations.push(AsyncComputation {
        //     operation: FheOperation::FheSub.into(),
        //     output_handle: new_from_amount_target_handle_1.clone(),
        //     inputs: vec![from_balance_1.clone(), amount_1.clone()],
        // });
        // async_computations.push(AsyncComputation {
        //     operation: FheOperation::FheIfThenElse.into(),
        //     output_handle: new_from_amount_handle_1.clone(),
        //     inputs: vec![
        //         AsyncComputationInput {
        //             input: Some(Input::InputHandle(has_enough_funds_handle_1.clone())),
        //         },
        //         AsyncComputationInput {
        //             input: Some(Input::InputHandle(new_from_amount_target_handle_1.clone())),
        //         },
        //         from_balance_1.clone(),
        //     ],
        // });
        let new_current_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(new_to_amount_handle_0.clone())),
        };
        let new_current_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(new_to_amount_handle_1.clone())),
        };
        let sent_0_handle = next_handle();
        let sent_1_handle = next_handle();
        let pending_0_in_handle = next_handle();
        let pending_1_in_handle = next_handle();
        let pending_total_token_0_in = next_handle();
        let pending_total_token_1_in = next_handle();
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheSub.into(),
            transaction_id: transaction_id.clone(),
            output_handle: sent_0_handle.clone(),
            inputs: vec![new_current_balance_0.clone(), current_dex_balance_0.clone()],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheSub.into(),
            transaction_id: transaction_id.clone(),
            output_handle: sent_1_handle.clone(),
            inputs: vec![new_current_balance_1.clone(), current_dex_balance_1.clone()],
            is_allowed: false,
        });
        let sent_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(sent_0_handle.clone())),
        };
        let sent_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(sent_1_handle.clone())),
        };
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: pending_0_in_handle.clone(),
            inputs: vec![to_balance_0.clone(), sent_0.clone()],
            is_allowed: true,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: pending_1_in_handle.clone(),
            inputs: vec![to_balance_1.clone(), sent_1.clone()],
            is_allowed: true,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: pending_total_token_0_in.clone(),
            inputs: vec![total_dex_token_0_in.clone(), sent_0.clone()],
            is_allowed: true,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: pending_total_token_1_in.clone(),
            inputs: vec![total_dex_token_1_in.clone(), sent_1.clone()],
            is_allowed: true,
        });
        // Update DEX balance handles
        current_dex_balance_0 = new_current_balance_0.clone();
        current_dex_balance_1 = new_current_balance_1.clone();
    }

    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.async_compute(compute_request).await.unwrap();
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
        "swap-request",
        &OperatorType::Atomic,
        64,
        vec![],
    );

    Ok(())
}

async fn swap_request_no_cmux_dep(
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
        .push(100_u64) // From balance 0
        .push(200_u64) // From balance 1
        .push(700_u64) // Current dex balance 0
        .push(900_u64) // Current dex balance 1
        .push(100_u64) // To balance 0
        .push(200_u64) // To balance 1
        .push(100_u64) // Total dex token in 0
        .push(200_u64) // Total dex token in 1
        .push(10_u64) // Amount 0
        .push(20_u64) // Amount 1
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
    assert_eq!(first_resp.input_handles.len(), 10);

    // Dependence on DEX balances
    let current_dex_balance_0 = first_resp.input_handles[2].handle.clone();
    let mut current_dex_balance_0 = AsyncComputationInput {
        input: Some(Input::InputHandle(current_dex_balance_0.clone())),
    };
    let current_dex_balance_1 = first_resp.input_handles[3].handle.clone();
    let mut current_dex_balance_1 = AsyncComputationInput {
        input: Some(Input::InputHandle(current_dex_balance_1.clone())),
    };

    for _ in 0..=(num_samples - 1) as u32 {
        let transaction_id = next_handle();
        // Swap request inputs
        let from_balance_0 = first_resp.input_handles[0].handle.clone();
        let from_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(from_balance_0.clone())),
        };
        let from_balance_1 = first_resp.input_handles[1].handle.clone();
        let from_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(from_balance_1.clone())),
        };
        let to_balance_0 = first_resp.input_handles[4].handle.clone();
        let to_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(to_balance_0.clone())),
        };
        let to_balance_1 = first_resp.input_handles[5].handle.clone();
        let to_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(to_balance_1.clone())),
        };
        let total_dex_token_0_in = first_resp.input_handles[6].handle.clone();
        let total_dex_token_0_in = AsyncComputationInput {
            input: Some(Input::InputHandle(total_dex_token_0_in.clone())),
        };
        let total_dex_token_1_in = first_resp.input_handles[7].handle.clone();
        let total_dex_token_1_in = AsyncComputationInput {
            input: Some(Input::InputHandle(total_dex_token_1_in.clone())),
        };
        let amount_0 = first_resp.input_handles[8].handle.clone();
        let amount_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(amount_0.clone())),
        };
        let amount_1 = first_resp.input_handles[9].handle.clone();
        let amount_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(amount_1.clone())),
        };

        // First transfer
        let has_enough_funds_handle_0 = next_handle();
        let cast_has_enough_funds_handle_0 = next_handle();
        let select_amount_handle_0 = next_handle();
        let new_to_amount_handle_0 = next_handle();
        let _new_from_amount_handle_0 = next_handle();
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheGe.into(),
            transaction_id: transaction_id.clone(),
            output_handle: has_enough_funds_handle_0.clone(),
            inputs: vec![from_balance_0.clone(), amount_0.clone()],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheCast.into(),
            transaction_id: transaction_id.clone(),
            output_handle: cast_has_enough_funds_handle_0.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(has_enough_funds_handle_0.clone())),
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
            output_handle: select_amount_handle_0.clone(),
            inputs: vec![
                amount_0.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(cast_has_enough_funds_handle_0.clone())),
                },
            ],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_handle_0.clone(),
            inputs: vec![
                current_dex_balance_0.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(select_amount_handle_0.clone())),
                },
            ],
            is_allowed: false,
        });
        // async_computations.push(AsyncComputation {
        //     operation: FheOperation::FheSub.into(),
        //     output_handle: new_from_amount_handle_0.clone(),
        //     inputs: vec![
        //         from_balance_0.clone(),
        //         AsyncComputationInput {
        //             input: Some(Input::InputHandle(select_amount_handle_0.clone())),
        //         },
        //     ],
        // });

        // Second transfer
        let has_enough_funds_handle_1 = next_handle();
        let cast_has_enough_funds_handle_1 = next_handle();
        let select_amount_handle_1 = next_handle();
        let new_to_amount_handle_1 = next_handle();
        let _new_from_amount_handle_1 = next_handle();
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheGe.into(),
            transaction_id: transaction_id.clone(),
            output_handle: has_enough_funds_handle_1.clone(),
            inputs: vec![from_balance_1.clone(), amount_1.clone()],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheCast.into(),
            transaction_id: transaction_id.clone(),
            output_handle: cast_has_enough_funds_handle_1.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(has_enough_funds_handle_1.clone())),
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
            output_handle: select_amount_handle_1.clone(),
            inputs: vec![
                amount_1.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(cast_has_enough_funds_handle_1.clone())),
                },
            ],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: new_to_amount_handle_1.clone(),
            inputs: vec![
                current_dex_balance_1.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(select_amount_handle_1.clone())),
                },
            ],
            is_allowed: false,
        });
        // async_computations.push(AsyncComputation {
        //     operation: FheOperation::FheSub.into(),
        //     output_handle: new_from_amount_handle_1.clone(),
        //     inputs: vec![
        //         from_balance_1.clone(),
        //         AsyncComputationInput {
        //             input: Some(Input::InputHandle(select_amount_handle_1.clone())),
        //         },
        //     ],
        // });

        let new_current_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(new_to_amount_handle_0.clone())),
        };
        let new_current_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(new_to_amount_handle_1.clone())),
        };
        let sent_0_handle = next_handle();
        let sent_1_handle = next_handle();
        let pending_0_in_handle = next_handle();
        let pending_1_in_handle = next_handle();
        let pending_total_token_0_in = next_handle();
        let pending_total_token_1_in = next_handle();
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheSub.into(),
            transaction_id: transaction_id.clone(),
            output_handle: sent_0_handle.clone(),
            inputs: vec![new_current_balance_0.clone(), current_dex_balance_0.clone()],
            is_allowed: false,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheSub.into(),
            transaction_id: transaction_id.clone(),
            output_handle: sent_1_handle.clone(),
            inputs: vec![new_current_balance_1.clone(), current_dex_balance_1.clone()],
            is_allowed: false,
        });
        let sent_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(sent_0_handle.clone())),
        };
        let sent_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(sent_1_handle.clone())),
        };
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: pending_0_in_handle.clone(),
            inputs: vec![to_balance_0.clone(), sent_0.clone()],
            is_allowed: true,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: pending_1_in_handle.clone(),
            inputs: vec![to_balance_1.clone(), sent_1.clone()],
            is_allowed: true,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: pending_total_token_0_in.clone(),
            inputs: vec![total_dex_token_0_in.clone(), sent_0.clone()],
            is_allowed: true,
        });
        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: transaction_id.clone(),
            output_handle: pending_total_token_1_in.clone(),
            inputs: vec![total_dex_token_1_in.clone(), sent_1.clone()],
            is_allowed: true,
        });
        // Update DEX balance handles
        current_dex_balance_0 = new_current_balance_0.clone();
        current_dex_balance_1 = new_current_balance_1.clone();
    }

    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.async_compute(compute_request).await.unwrap();
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
        "swap-request",
        &OperatorType::Atomic,
        64,
        vec![],
    );

    Ok(())
}

async fn swap_claim_whitepaper_dep(
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
        .push(100_u64) // Pending 0 in
        .push(200_u64) // Pending 1 in
        .push(700_u64) // Old balance 0
        .push(900_u64) // Old balance 1
        .push(100_u64) // Current dex balance 0
        .push(200_u64) // Current dex balance  1
        .build_with_proof_packed(&keys.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
        .unwrap();
    let total_dex_token_0_in: u64 = 300;
    let total_dex_token_1_in: u64 = 600;
    let total_dex_token_0_out: u64 = 100;
    let total_dex_token_1_out: u64 = 200;

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
    assert_eq!(first_resp.input_handles.len(), 6);

    // Dependence on DEX balances
    let current_dex_balance_0 = first_resp.input_handles[4].handle.clone();
    let mut current_dex_balance_0 = AsyncComputationInput {
        input: Some(Input::InputHandle(current_dex_balance_0.clone())),
    };
    let current_dex_balance_1 = first_resp.input_handles[5].handle.clone();
    let mut current_dex_balance_1 = AsyncComputationInput {
        input: Some(Input::InputHandle(current_dex_balance_1.clone())),
    };

    for _ in 0..=(num_samples - 1) as u32 {
        let transaction_id = next_handle();
        // Swap claim inputs
        let pending_0_in = first_resp.input_handles[0].handle.clone();
        let pending_0_in = AsyncComputationInput {
            input: Some(Input::InputHandle(pending_0_in.clone())),
        };
        let pending_1_in = first_resp.input_handles[1].handle.clone();
        let pending_1_in = AsyncComputationInput {
            input: Some(Input::InputHandle(pending_1_in.clone())),
        };
        let old_balance_0 = first_resp.input_handles[2].handle.clone();
        let old_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(old_balance_0.clone())),
        };
        let old_balance_1 = first_resp.input_handles[3].handle.clone();
        let old_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(old_balance_1.clone())),
        };

        let big_pending_0_in = next_handle();
        let big_pending_1_in = next_handle();
        let big_amount_0_out = next_handle();
        let big_amount_1_out = next_handle();
        let amount_0_out = next_handle();
        let amount_1_out = next_handle();
        let _new_balance_0 = next_handle();
        let _new_balance_1 = next_handle();
        if total_dex_token_1_in != 0 {
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: big_pending_1_in.clone(),
                inputs: vec![
                    pending_1_in,
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![6u8])),
                    },
                ],
                is_allowed: false,
            });
            let mul_temp = next_handle();
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheMul.into(),
                transaction_id: transaction_id.clone(),
                output_handle: mul_temp.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(big_pending_1_in.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(
                            (total_dex_token_0_out as u128).to_be_bytes().to_vec(),
                        )),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheDiv.into(),
                transaction_id: transaction_id.clone(),
                output_handle: big_amount_0_out.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(mul_temp.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(
                            (total_dex_token_1_in as u128).to_be_bytes().to_vec(),
                        )),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: amount_0_out.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(big_amount_0_out.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![5u8])),
                    },
                ],
                is_allowed: false,
            });
            // Transfer
            let has_enough_funds_handle_0 = next_handle();
            let new_to_amount_target_handle_0 = next_handle();
            let new_to_amount_handle_0 = next_handle();
            let new_from_amount_target_handle_0 = next_handle();
            let new_from_amount_handle_0 = next_handle();
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheGe.into(),
                transaction_id: transaction_id.clone(),
                output_handle: has_enough_funds_handle_0.clone(),
                inputs: vec![
                    current_dex_balance_0.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_0_out.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheAdd.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_to_amount_target_handle_0.clone(),
                inputs: vec![
                    old_balance_0.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_0_out.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheIfThenElse.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_to_amount_handle_0.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(has_enough_funds_handle_0.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(new_to_amount_target_handle_0.clone())),
                    },
                    old_balance_0.clone(),
                ],
                is_allowed: true,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheSub.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_from_amount_target_handle_0.clone(),
                inputs: vec![
                    current_dex_balance_0.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_0_out.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheIfThenElse.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_from_amount_handle_0.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(has_enough_funds_handle_0.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(new_from_amount_target_handle_0.clone())),
                    },
                    current_dex_balance_0.clone(),
                ],
                is_allowed: true,
            });
            // Update DEX balance handles
            current_dex_balance_0 = AsyncComputationInput {
                input: Some(Input::InputHandle(new_from_amount_handle_0.clone())),
            };
        }
        if total_dex_token_0_in != 0 {
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: big_pending_0_in.clone(),
                inputs: vec![
                    pending_0_in,
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![6u8])),
                    },
                ],
                is_allowed: false,
            });
            let mul_temp = next_handle();
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheMul.into(),
                transaction_id: transaction_id.clone(),
                output_handle: mul_temp.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(big_pending_0_in.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(
                            (total_dex_token_1_out as u128).to_be_bytes().to_vec(),
                        )),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheDiv.into(),
                transaction_id: transaction_id.clone(),
                output_handle: big_amount_1_out.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(mul_temp.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(
                            (total_dex_token_0_in as u128).to_be_bytes().to_vec(),
                        )),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: amount_1_out.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(big_amount_1_out.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![5u8])),
                    },
                ],
                is_allowed: false,
            });
            // Transfer
            let has_enough_funds_handle_1 = next_handle();
            let new_to_amount_target_handle_1 = next_handle();
            let new_to_amount_handle_1 = next_handle();
            let new_from_amount_target_handle_1 = next_handle();
            let new_from_amount_handle_1 = next_handle();
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheGe.into(),
                transaction_id: transaction_id.clone(),
                output_handle: has_enough_funds_handle_1.clone(),
                inputs: vec![
                    current_dex_balance_1.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_1_out.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheAdd.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_to_amount_target_handle_1.clone(),
                inputs: vec![
                    old_balance_1.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_1_out.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheIfThenElse.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_to_amount_handle_1.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(has_enough_funds_handle_1.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(new_to_amount_target_handle_1.clone())),
                    },
                    old_balance_1.clone(),
                ],
                is_allowed: true,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheSub.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_from_amount_target_handle_1.clone(),
                inputs: vec![
                    current_dex_balance_1.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_1_out.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheIfThenElse.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_from_amount_handle_1.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(has_enough_funds_handle_1.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(new_from_amount_target_handle_1.clone())),
                    },
                    current_dex_balance_1.clone(),
                ],
                is_allowed: true,
            });
            // Update DEX balance handles
            current_dex_balance_1 = AsyncComputationInput {
                input: Some(Input::InputHandle(new_from_amount_handle_1.clone())),
            };
        }
    }

    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.async_compute(compute_request).await.unwrap();
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
        "swap-request",
        &OperatorType::Atomic,
        64,
        vec![],
    );

    Ok(())
}

async fn swap_claim_no_cmux_dep(
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
        .push(100_u64) // Pending 0 in
        .push(200_u64) // Pending 1 in
        .push(700_u64) // Old balance 0
        .push(900_u64) // Old balance 1
        .push(100_u64) // Current dex balance 0
        .push(200_u64) // Current dex balance  1
        .build_with_proof_packed(&keys.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
        .unwrap();
    let total_dex_token_0_in: u64 = 300;
    let total_dex_token_1_in: u64 = 600;
    let total_dex_token_0_out: u64 = 100;
    let total_dex_token_1_out: u64 = 200;

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
    assert_eq!(first_resp.input_handles.len(), 6);

    // Dependence on DEX balances
    let current_dex_balance_0 = first_resp.input_handles[4].handle.clone();
    let mut current_dex_balance_0 = AsyncComputationInput {
        input: Some(Input::InputHandle(current_dex_balance_0.clone())),
    };
    let current_dex_balance_1 = first_resp.input_handles[5].handle.clone();
    let mut current_dex_balance_1 = AsyncComputationInput {
        input: Some(Input::InputHandle(current_dex_balance_1.clone())),
    };

    for _ in 0..=(num_samples - 1) as u32 {
        let transaction_id = next_handle();
        // Swap claim inputs
        let pending_0_in = first_resp.input_handles[0].handle.clone();
        let pending_0_in = AsyncComputationInput {
            input: Some(Input::InputHandle(pending_0_in.clone())),
        };
        let pending_1_in = first_resp.input_handles[1].handle.clone();
        let pending_1_in = AsyncComputationInput {
            input: Some(Input::InputHandle(pending_1_in.clone())),
        };
        let old_balance_0 = first_resp.input_handles[2].handle.clone();
        let old_balance_0 = AsyncComputationInput {
            input: Some(Input::InputHandle(old_balance_0.clone())),
        };
        let old_balance_1 = first_resp.input_handles[3].handle.clone();
        let old_balance_1 = AsyncComputationInput {
            input: Some(Input::InputHandle(old_balance_1.clone())),
        };

        let big_pending_0_in = next_handle();
        let big_pending_1_in = next_handle();
        let big_amount_0_out = next_handle();
        let big_amount_1_out = next_handle();
        let amount_0_out = next_handle();
        let amount_1_out = next_handle();
        let _new_balance_0 = next_handle();
        let _new_balance_1 = next_handle();
        if total_dex_token_1_in != 0 {
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: big_pending_1_in.clone(),
                inputs: vec![
                    pending_1_in,
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![6u8])),
                    },
                ],
                is_allowed: false,
            });
            let mul_temp = next_handle();
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheMul.into(),
                transaction_id: transaction_id.clone(),
                output_handle: mul_temp.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(big_pending_1_in.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(
                            (total_dex_token_0_out as u128).to_be_bytes().to_vec(),
                        )),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheDiv.into(),
                transaction_id: transaction_id.clone(),
                output_handle: big_amount_0_out.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(mul_temp.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(
                            (total_dex_token_1_in as u128).to_be_bytes().to_vec(),
                        )),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: amount_0_out.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(big_amount_0_out.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![5u8])),
                    },
                ],
                is_allowed: false,
            });

            // Transfer
            let has_enough_funds_handle_0 = next_handle();
            let cast_has_enough_funds_handle_0 = next_handle();
            let select_amount_handle_0 = next_handle();
            let new_to_amount_handle_0 = next_handle();
            let new_from_amount_handle_0 = next_handle();
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheGe.into(),
                transaction_id: transaction_id.clone(),
                output_handle: has_enough_funds_handle_0.clone(),
                inputs: vec![
                    current_dex_balance_0.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_0_out.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: cast_has_enough_funds_handle_0.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(has_enough_funds_handle_0.clone())),
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
                output_handle: select_amount_handle_0.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_0_out.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(cast_has_enough_funds_handle_0.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheAdd.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_to_amount_handle_0.clone(),
                inputs: vec![
                    old_balance_0.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(select_amount_handle_0.clone())),
                    },
                ],
                is_allowed: true,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheSub.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_from_amount_handle_0.clone(),
                inputs: vec![
                    current_dex_balance_0.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(select_amount_handle_0.clone())),
                    },
                ],
                is_allowed: true,
            });
            // Update DEX balance handles
            current_dex_balance_0 = AsyncComputationInput {
                input: Some(Input::InputHandle(new_from_amount_handle_0.clone())),
            };
        }

        if total_dex_token_0_in != 0 {
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: big_pending_0_in.clone(),
                inputs: vec![
                    pending_0_in,
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![6u8])),
                    },
                ],
                is_allowed: false,
            });
            let mul_temp = next_handle();
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheMul.into(),
                transaction_id: transaction_id.clone(),
                output_handle: mul_temp.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(big_pending_0_in.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(
                            (total_dex_token_1_out as u128).to_be_bytes().to_vec(),
                        )),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheDiv.into(),
                transaction_id: transaction_id.clone(),
                output_handle: big_amount_1_out.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(mul_temp.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(
                            (total_dex_token_0_in as u128).to_be_bytes().to_vec(),
                        )),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: amount_1_out.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(big_amount_1_out.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::Scalar(vec![5u8])),
                    },
                ],
                is_allowed: false,
            });
            // Transfer
            let has_enough_funds_handle_1 = next_handle();
            let cast_has_enough_funds_handle_1 = next_handle();
            let select_amount_handle_1 = next_handle();
            let new_to_amount_handle_1 = next_handle();
            let new_from_amount_handle_1 = next_handle();
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheGe.into(),
                transaction_id: transaction_id.clone(),
                output_handle: has_enough_funds_handle_1.clone(),
                inputs: vec![
                    current_dex_balance_1.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_1_out.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheCast.into(),
                transaction_id: transaction_id.clone(),
                output_handle: cast_has_enough_funds_handle_1.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(has_enough_funds_handle_1.clone())),
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
                output_handle: select_amount_handle_1.clone(),
                inputs: vec![
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(amount_1_out.clone())),
                    },
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(cast_has_enough_funds_handle_1.clone())),
                    },
                ],
                is_allowed: false,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheAdd.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_to_amount_handle_1.clone(),
                inputs: vec![
                    old_balance_1.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(select_amount_handle_1.clone())),
                    },
                ],
                is_allowed: true,
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheSub.into(),
                transaction_id: transaction_id.clone(),
                output_handle: new_from_amount_handle_1.clone(),
                inputs: vec![
                    current_dex_balance_1.clone(),
                    AsyncComputationInput {
                        input: Some(Input::InputHandle(select_amount_handle_1.clone())),
                    },
                ],
                is_allowed: true,
            });
            // Update DEX balance handles
            current_dex_balance_1 = AsyncComputationInput {
                input: Some(Input::InputHandle(new_from_amount_handle_1.clone())),
            };
        }
    }

    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.async_compute(compute_request).await.unwrap();
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
        "swap-request",
        &OperatorType::Atomic,
        64,
        vec![],
    );

    Ok(())
}
