#[path = "./utils.rs"]
mod utils;

use crate::utils::{
    allow_handle, as_scalar_uint, listener_event_db, next_handle, random_handle, scalar_flag,
    setup_test_app, tfhe_event, to_ty, wait_until_all_allowed_handles_computed, zero_address,
    EnvConfig, OperatorType,
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

fn sample_count(default_count: usize) -> usize {
    std::env::var("FHEVM_TEST_NUM_SAMPLES")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(default_count)
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
        log_index: None,
        removed: false,
    }
}

async fn schedule_erc20(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    use_cmux: bool,
    dependent: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let listener_db = listener_event_db(&app).await?;
    let caller = zero_address();
    let num_samples = sample_count(num_tx);
    let mut handle_counter = random_handle();
    let shared_tx_id = next_handle(&mut handle_counter);

    let mut tx = listener_db.new_transaction().await?;
    let mut prev_from: Option<host_listener::database::tfhe_event_propagate::Handle> = None;
    let mut prev_to: Option<host_listener::database::tfhe_event_propagate::Handle> = None;

    for i in 0..num_samples {
        let tx_id = if dependent {
            shared_tx_id
        } else {
            next_handle(&mut handle_counter)
        };
        let from_balance = if let Some(h) = prev_from {
            h
        } else {
            let h = next_handle(&mut handle_counter);
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::TrivialEncrypt(
                        TfheContract::TrivialEncrypt {
                            caller,
                            pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(100_u64)),
                            toType: to_ty(5),
                            result: h,
                        },
                    )),
                ),
                tx_id,
                false,
            )
            .await?;
            h
        };
        let to_balance = if let Some(h) = prev_to {
            h
        } else {
            let h = next_handle(&mut handle_counter);
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::TrivialEncrypt(
                        TfheContract::TrivialEncrypt {
                            caller,
                            pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(20_u64)),
                            toType: to_ty(5),
                            result: h,
                        },
                    )),
                ),
                tx_id,
                false,
            )
            .await?;
            h
        };
        let transfer_amount = next_handle(&mut handle_counter);
        utils::insert_tfhe_event(
            &listener_db,
            &mut tx,
            log_with_tx(
                tx_id,
                tfhe_event(TfheContractEvents::TrivialEncrypt(
                    TfheContract::TrivialEncrypt {
                        caller,
                        pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(10_u64)),
                        toType: to_ty(5),
                        result: transfer_amount,
                    },
                )),
            ),
            tx_id,
            false,
        )
        .await?;

        let has_funds = next_handle(&mut handle_counter);
        utils::insert_tfhe_event(
            &listener_db,
            &mut tx,
            log_with_tx(
                tx_id,
                tfhe_event(TfheContractEvents::FheGe(TfheContract::FheGe {
                    caller,
                    lhs: from_balance,
                    rhs: transfer_amount,
                    scalarByte: scalar_flag(false),
                    result: has_funds,
                })),
            ),
            tx_id,
            false,
        )
        .await?;

        let new_to;
        let new_from;
        if use_cmux {
            let to_target = next_handle(&mut handle_counter);
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::FheAdd(TfheContract::FheAdd {
                        caller,
                        lhs: to_balance,
                        rhs: transfer_amount,
                        scalarByte: scalar_flag(false),
                        result: to_target,
                    })),
                ),
                tx_id,
                false,
            )
            .await?;
            new_to = next_handle(&mut handle_counter);
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::FheIfThenElse(
                        TfheContract::FheIfThenElse {
                            caller,
                            control: has_funds,
                            ifTrue: to_target,
                            ifFalse: to_balance,
                            result: new_to,
                        },
                    )),
                ),
                tx_id,
                true,
            )
            .await?;

            let from_target = next_handle(&mut handle_counter);
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::FheSub(TfheContract::FheSub {
                        caller,
                        lhs: from_balance,
                        rhs: transfer_amount,
                        scalarByte: scalar_flag(false),
                        result: from_target,
                    })),
                ),
                tx_id,
                false,
            )
            .await?;
            new_from = next_handle(&mut handle_counter);
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::FheIfThenElse(
                        TfheContract::FheIfThenElse {
                            caller,
                            control: has_funds,
                            ifTrue: from_target,
                            ifFalse: from_balance,
                            result: new_from,
                        },
                    )),
                ),
                tx_id,
                true,
            )
            .await?;
        } else {
            let funds_u64 = next_handle(&mut handle_counter);
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::Cast(TfheContract::Cast {
                        caller,
                        ct: has_funds,
                        toType: to_ty(5),
                        result: funds_u64,
                    })),
                ),
                tx_id,
                false,
            )
            .await?;
            let selected_amount = next_handle(&mut handle_counter);
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::FheMul(TfheContract::FheMul {
                        caller,
                        lhs: transfer_amount,
                        rhs: funds_u64,
                        scalarByte: scalar_flag(false),
                        result: selected_amount,
                    })),
                ),
                tx_id,
                false,
            )
            .await?;
            new_to = next_handle(&mut handle_counter);
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::FheAdd(TfheContract::FheAdd {
                        caller,
                        lhs: to_balance,
                        rhs: selected_amount,
                        scalarByte: scalar_flag(false),
                        result: new_to,
                    })),
                ),
                tx_id,
                true,
            )
            .await?;
            new_from = next_handle(&mut handle_counter);
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::FheSub(TfheContract::FheSub {
                        caller,
                        lhs: from_balance,
                        rhs: selected_amount,
                        scalarByte: scalar_flag(false),
                        result: new_from,
                    })),
                ),
                tx_id,
                true,
            )
            .await?;
        }

        if i == num_samples.saturating_sub(1) {
            allow_handle(&listener_db, &mut tx, &new_to).await?;
            allow_handle(&listener_db, &mut tx, &new_from).await?;
        }
        prev_from = Some(new_from);
        prev_to = Some(new_to);
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

    let _ = OperatorType::Atomic;
    Ok(())
}

async fn schedule_erc20_whitepaper(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    _bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(bencher, num_tx, true, false).await
}

async fn schedule_erc20_no_cmux(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    _bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(bencher, num_tx, false, false).await
}

async fn schedule_dependent_erc20_whitepaper(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    _bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(bencher, num_tx, true, true).await
}

async fn schedule_dependent_erc20_no_cmux(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    _bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(bencher, num_tx, false, true).await
}
