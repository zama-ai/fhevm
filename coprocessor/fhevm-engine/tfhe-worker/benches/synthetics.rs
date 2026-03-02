#[path = "./utils.rs"]
mod utils;

use crate::utils::{
    allow_handle, as_scalar_uint, listener_event_db, next_handle, random_handle, scalar_flag,
    setup_test_app, tfhe_event, to_ty, wait_until_all_allowed_handles_computed,
    write_atomic_u64_bench_params, zero_address, EnvConfig,
};
use criterion::{
    async_executor::FuturesExecutor, measurement::WallTime, Bencher, Criterion, Throughput,
};
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use std::time::SystemTime;
use tfhe_worker::tfhe_worker::TIMING;
use tokio::runtime::Runtime;

fn main() {
    let ecfg = EnvConfig::new();
    let mut c = Criterion::default()
        .sample_size(10)
        .measurement_time(std::time::Duration::from_secs(1000))
        .configure_from_args();
    let bench_name = "synthetic";
    let bench_optimization_target = if cfg!(feature = "latency") {
        "opt_latency"
    } else {
        "opt_throughput"
    };

    let mut group = c.benchmark_group(bench_name);
    if ecfg.benchmark_type == "LATENCY" || ecfg.benchmark_type == "ALL" {
        let num_elems = 1;
        let bench_id = format!("{bench_name}::latency::counter::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            let _ = Runtime::new().unwrap().block_on(counter_increment(
                b,
                num_elems as usize,
                bench_id.clone(),
            ));
        });

        let bench_id =
            format!("{bench_name}::latency::tree_reduction::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            let _ = Runtime::new().unwrap().block_on(tree_reduction(
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
                format!("{bench_name}::throughput::counter::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new().unwrap().block_on(counter_increment(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ));
            });

            group.throughput(Throughput::Elements(num_elems));
            let bench_id =
                format!("{bench_name}::throughput::tree_reduction::FHEUint64::{num_elems}_elems::{bench_optimization_target}");
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new().unwrap().block_on(tree_reduction(
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

fn sample_count(default_count: usize) -> usize {
    std::env::var("FHEVM_TEST_NUM_SAMPLES")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(default_count)
}

fn next_log_index() -> u64 {
    static COUNT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

fn log_with_tx(
    tx_hash: host_listener::database::tfhe_event_propagate::Handle,
    inner: alloy::primitives::Log<TfheContractEvents>,
) -> alloy::rpc::types::Log<TfheContractEvents> {
    alloy::rpc::types::Log {
        inner,
        block_hash: None,
        block_number: None,
        block_timestamp: None,
        transaction_hash: Some(tx_hash),
        transaction_index: Some(0),
        log_index: Some(next_log_index()),
        removed: false,
    }
}

async fn counter_increment(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let listener_db = listener_event_db(&app).await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(app.db_url())
        .await?;
    let mut handle_counter: u64 = random_handle();
    let caller = zero_address();
    let num_samples = sample_count(num_tx);

    let tx_id = next_handle(&mut handle_counter);
    let initial_counter = next_handle(&mut handle_counter);
    let increment_by = next_handle(&mut handle_counter);
    let mut tx = listener_db.new_transaction().await?;

    utils::insert_tfhe_event(
        &listener_db,
        &mut tx,
        log_with_tx(
            tx_id,
            tfhe_event(TfheContractEvents::TrivialEncrypt(
                TfheContract::TrivialEncrypt {
                    caller,
                    pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(42_u64)),
                    toType: to_ty(5),
                    result: initial_counter,
                },
            )),
        ),
        tx_id,
        false,
    )
    .await?;
    utils::insert_tfhe_event(
        &listener_db,
        &mut tx,
        log_with_tx(
            tx_id,
            tfhe_event(TfheContractEvents::TrivialEncrypt(
                TfheContract::TrivialEncrypt {
                    caller,
                    pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(7_u64)),
                    toType: to_ty(5),
                    result: increment_by,
                },
            )),
        ),
        tx_id,
        false,
    )
    .await?;

    let mut counter = initial_counter;
    for i in 0..num_samples {
        let output = next_handle(&mut handle_counter);
        let is_last = i == num_samples.saturating_sub(1);
        utils::insert_tfhe_event(
            &listener_db,
            &mut tx,
            log_with_tx(
                tx_id,
                tfhe_event(TfheContractEvents::FheAdd(TfheContract::FheAdd {
                    caller,
                    lhs: counter,
                    rhs: increment_by,
                    scalarByte: scalar_flag(false),
                    result: output,
                })),
            ),
            tx_id,
            is_last,
        )
        .await?;
        if is_last {
            allow_handle(&listener_db, &mut tx, &output).await?;
        }
        counter = output;
    }
    tx.commit().await?;

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

    write_atomic_u64_bench_params(&pool, &bench_id, "counter-increment").await?;
    Ok(())
}

async fn tree_reduction(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let listener_db = listener_event_db(&app).await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(app.db_url())
        .await?;
    let mut handle_counter: u64 = random_handle();
    let caller = zero_address();
    let num_samples = sample_count(num_tx).max(2);
    let tx_id = next_handle(&mut handle_counter);
    let mut tx = listener_db.new_transaction().await?;

    let mut current_level = Vec::with_capacity(num_samples);
    for _ in 0..num_samples {
        let h = next_handle(&mut handle_counter);
        utils::insert_tfhe_event(
            &listener_db,
            &mut tx,
            log_with_tx(
                tx_id,
                tfhe_event(TfheContractEvents::TrivialEncrypt(
                    TfheContract::TrivialEncrypt {
                        caller,
                        pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(1_u64)),
                        toType: to_ty(5),
                        result: h,
                    },
                )),
            ),
            tx_id,
            false,
        )
        .await?;
        current_level.push(h);
    }

    while current_level.len() > 1 {
        let mut next_level = Vec::with_capacity(current_level.len().div_ceil(2));
        let input_len = current_level.len();
        for (idx, pair) in current_level.chunks(2).enumerate() {
            if pair.len() == 1 {
                next_level.push(pair[0]);
                continue;
            }
            let out = next_handle(&mut handle_counter);
            let is_last = input_len == 2 && idx == 0;
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::FheAdd(TfheContract::FheAdd {
                        caller,
                        lhs: pair[0],
                        rhs: pair[1],
                        scalarByte: scalar_flag(false),
                        result: out,
                    })),
                ),
                tx_id,
                is_last,
            )
            .await?;
            if is_last {
                allow_handle(&listener_db, &mut tx, &out).await?;
            }
            next_level.push(out);
        }
        current_level = next_level;
    }

    tx.commit().await?;

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

    write_atomic_u64_bench_params(&pool, &bench_id, "tree-reduction").await?;
    Ok(())
}
