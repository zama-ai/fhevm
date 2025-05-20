#[path = "./utils.rs"]
mod utils;
use crate::utils::{
    default_api_key, default_tenant_id, query_tenant_keys, random_handle, setup_test_app,
    wait_until_all_ciphertexts_computed, write_to_json, OperatorType,
};
use coprocessor::server::common::FheOperation;
use coprocessor::server::coprocessor::{async_computation_input::Input, AsyncComputationInput};
use coprocessor::server::coprocessor::{
    fhevm_coprocessor_client::FhevmCoprocessorClient, AsyncComputation, AsyncComputeRequest,
    InputToUpload, InputUploadBatch,
};
use criterion::{
    async_executor::FuturesExecutor, measurement::WallTime, Bencher, Criterion, Throughput,
};
use fhevm_engine_common::utils::safe_serialize;
use std::str::FromStr;
use std::time::SystemTime;
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
    let mut c = Criterion::default().sample_size(10).configure_from_args();
    let bench_name = "synthetic";

    let mut group = c.benchmark_group(bench_name);
    for num_elems in [10, 50, 200, 500] {
        group.throughput(Throughput::Elements(num_elems));
        let bench_id = format!("{bench_name}::throughput::counter::FHEUint64::{num_elems}_elems");
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            let _ = Runtime::new().unwrap().block_on(counter_increment(
                b,
                num_elems as usize,
                bench_id.clone(),
            ));
        });

        group.throughput(Throughput::Elements(num_elems));
        let bench_id =
            format!("{bench_name}::throughput::tree_reduction::FHEUint64::{num_elems}_elems");
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            let _ = Runtime::new().unwrap().block_on(tree_reduction(
                b,
                num_elems as usize,
                bench_id.clone(),
            ));
        });
    }
    group.finish();

    c.final_summary();
}

async fn counter_increment(
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

    let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.0.pks);
    let the_list = builder
        .push(42_u64) // Initial counter value
        .build_with_proof_packed(&keys.0.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
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
        let new_counter = next_handle();
        output_handles.push(new_counter.clone());

        async_computations.push(AsyncComputation {
            operation: FheOperation::FheAdd.into(),
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

    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.async_compute(compute_request).await?;

    bencher.to_async(FuturesExecutor).iter(|| async {
        let now = SystemTime::now();
        wait_until_all_ciphertexts_computed(&app).await.unwrap();
        println!("Execution time: {}", now.elapsed().unwrap().as_millis());
    });

    let params = keys.1.computation_parameters();
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

async fn tree_reduction(
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

    let num_levels = (num_samples as f64).log2().ceil() as usize;
    let mut num_comps_at_level = 2f64.powi((num_levels - 1) as i32) as usize;
    let target = num_comps_at_level * 2;
    let mut level_inputs = vec![];
    let mut level_outputs = vec![];

    for _ in 0..num_comps_at_level {
        let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.0.pks);
        let the_list = builder
            .push(1_u64) // Initial value
            .push(1_u64) // Initial value
            .build_with_proof_packed(&keys.0.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
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
    for _ in 0..num_levels {
        for i in 0..num_comps_at_level {
            output_handle = next_handle();
            level_outputs.push(AsyncComputationInput {
                input: Some(Input::InputHandle(output_handle.clone())),
            });
            async_computations.push(AsyncComputation {
                operation: FheOperation::FheAdd.into(),
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

    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.async_compute(compute_request).await?;

    bencher.to_async(FuturesExecutor).iter(|| async {
        let now = SystemTime::now();
        wait_until_all_ciphertexts_computed(&app).await.unwrap();
        println!("Execution time: {}", now.elapsed().unwrap().as_millis());
    });

    let params = keys.1.computation_parameters();
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
