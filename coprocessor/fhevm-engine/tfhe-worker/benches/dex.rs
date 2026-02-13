#[path = "./utils.rs"]
mod utils;

use crate::utils::{
    allow_handle, as_scalar_uint, listener_event_db, next_handle, random_handle, scalar_flag,
    setup_test_app, tfhe_event, to_ty, wait_until_all_allowed_handles_computed, zero_address,
    EnvConfig,
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
    let bench_optimization_target = if cfg!(feature = "latency") {
        "opt_latency"
    } else {
        "opt_throughput"
    };

    let bench_name = "dex::swap_request";
    let mut group = c.benchmark_group(bench_name);
    if ecfg.benchmark_type == "LATENCY" || ecfg.benchmark_type == "ALL" {
        let num_elems = 1;
        let bench_id = format!(
            "{bench_name}::latency::whitepaper::FHEUint64::{num_elems}_elems::{bench_optimization_target}"
        );
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            let _ = Runtime::new().unwrap().block_on(swap_request_whitepaper(
                b,
                num_elems as usize,
                bench_id.clone(),
            ));
        });

        let bench_id = format!(
            "{bench_name}::latency::no_cmux::FHEUint64::{num_elems}_elems::{bench_optimization_target}"
        );
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
            let bench_id = format!(
                "{bench_name}::throughput::whitepaper::FHEUint64::{num_elems}_elems::{bench_optimization_target}"
            );
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new().unwrap().block_on(swap_request_whitepaper(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ));
            });

            group.throughput(Throughput::Elements(num_elems));
            let bench_id = format!(
                "{bench_name}::throughput::no_cmux::FHEUint64::{num_elems}_elems::{bench_optimization_target}"
            );
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
        let bench_id = format!(
            "{bench_name}::latency::whitepaper::FHEUint64::{num_elems}_elems::{bench_optimization_target}"
        );
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            let _ = Runtime::new().unwrap().block_on(swap_claim_whitepaper(
                b,
                num_elems as usize,
                bench_id.clone(),
            ));
        });

        let bench_id = format!(
            "{bench_name}::latency::no_cmux::FHEUint64::{num_elems}_elems::{bench_optimization_target}"
        );
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
            let bench_id = format!(
                "{bench_name}::throughput::whitepaper::FHEUint64::{num_elems}_elems::{bench_optimization_target}"
            );
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new().unwrap().block_on(swap_claim_whitepaper(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ));
            });

            group.throughput(Throughput::Elements(num_elems));
            let bench_id = format!(
                "{bench_name}::throughput::no_cmux::FHEUint64::{num_elems}_elems::{bench_optimization_target}"
            );
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
            let bench_id = format!(
                "{bench_name}::throughput::whitepaper::FHEUint64::{num_elems}_elems::{bench_optimization_target}"
            );
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
            let bench_id = format!(
                "{bench_name}::throughput::no_cmux::FHEUint64::{num_elems}_elems::{bench_optimization_target}"
            );
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
            let bench_id = format!(
                "{bench_name}::throughput::whitepaper::FHEUint64::{num_elems}_elems::{bench_optimization_target}"
            );
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                let _ = Runtime::new().unwrap().block_on(swap_claim_whitepaper_dep(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ));
            });

            group.throughput(Throughput::Elements(num_elems));
            let bench_id = format!(
                "{bench_name}::throughput::no_cmux::FHEUint64::{num_elems}_elems::{bench_optimization_target}"
            );
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

async fn schedule_dex(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    use_cmux: bool,
    dependent: bool,
    is_claim: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let listener_db = listener_event_db(&app).await?;
    let caller = zero_address();
    let num_samples = sample_count(num_tx);
    let mut handle_counter = random_handle();
    let shared_tx_id = next_handle(&mut handle_counter);

    let mut tx = listener_db.new_transaction().await?;
    let mut prev_from: Option<host_listener::database::tfhe_event_propagate::Handle> = None;
    let mut prev_pool_in: Option<host_listener::database::tfhe_event_propagate::Handle> = None;
    let mut prev_pool_out: Option<host_listener::database::tfhe_event_propagate::Handle> = None;
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
        let pool_in = if let Some(h) = prev_pool_in {
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
                            pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(700_u64)),
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
        let pool_out = if let Some(h) = prev_pool_out {
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
                            pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(900_u64)),
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
                            pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(200_u64)),
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
        let amount = next_handle(&mut handle_counter);
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
                        result: amount,
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
                    rhs: amount,
                    scalarByte: scalar_flag(false),
                    result: has_funds,
                })),
            ),
            tx_id,
            false,
        )
        .await?;

        let denom = next_handle(&mut handle_counter);
        utils::insert_tfhe_event(
            &listener_db,
            &mut tx,
            log_with_tx(
                tx_id,
                tfhe_event(TfheContractEvents::FheAdd(TfheContract::FheAdd {
                    caller,
                    lhs: pool_in,
                    rhs: amount,
                    scalarByte: scalar_flag(false),
                    result: denom,
                })),
            ),
            tx_id,
            false,
        )
        .await?;
        let numerator = next_handle(&mut handle_counter);
        utils::insert_tfhe_event(
            &listener_db,
            &mut tx,
            log_with_tx(
                tx_id,
                tfhe_event(TfheContractEvents::FheMul(TfheContract::FheMul {
                    caller,
                    lhs: amount,
                    rhs: pool_out,
                    scalarByte: scalar_flag(false),
                    result: numerator,
                })),
            ),
            tx_id,
            false,
        )
        .await?;
        let quote = next_handle(&mut handle_counter);
        utils::insert_tfhe_event(
            &listener_db,
            &mut tx,
            log_with_tx(
                tx_id,
                tfhe_event(TfheContractEvents::FheDiv(TfheContract::FheDiv {
                    caller,
                    lhs: numerator,
                    rhs: denom,
                    scalarByte: scalar_flag(false),
                    result: quote,
                })),
            ),
            tx_id,
            false,
        )
        .await?;

        let selected_amount = if use_cmux {
            quote
        } else {
            let has_funds_u64 = next_handle(&mut handle_counter);
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::Cast(TfheContract::Cast {
                        caller,
                        ct: has_funds,
                        toType: to_ty(5),
                        result: has_funds_u64,
                    })),
                ),
                tx_id,
                false,
            )
            .await?;
            let out = next_handle(&mut handle_counter);
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::FheMul(TfheContract::FheMul {
                        caller,
                        lhs: quote,
                        rhs: has_funds_u64,
                        scalarByte: scalar_flag(false),
                        result: out,
                    })),
                ),
                tx_id,
                false,
            )
            .await?;
            out
        };

        let (new_from, new_pool_in, new_pool_out, new_to) = if is_claim {
            let new_from = next_handle(&mut handle_counter);
            let new_pool_out = next_handle(&mut handle_counter);
            let new_to = next_handle(&mut handle_counter);
            let new_pool_in = next_handle(&mut handle_counter);

            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::FheSub(TfheContract::FheSub {
                        caller,
                        lhs: from_balance,
                        rhs: amount,
                        scalarByte: scalar_flag(false),
                        result: new_from,
                    })),
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
                    tfhe_event(TfheContractEvents::FheSub(TfheContract::FheSub {
                        caller,
                        lhs: pool_out,
                        rhs: selected_amount,
                        scalarByte: scalar_flag(false),
                        result: new_pool_out,
                    })),
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
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::FheAdd(TfheContract::FheAdd {
                        caller,
                        lhs: pool_in,
                        rhs: amount,
                        scalarByte: scalar_flag(false),
                        result: new_pool_in,
                    })),
                ),
                tx_id,
                true,
            )
            .await?;
            (new_from, new_pool_in, new_pool_out, new_to)
        } else {
            let new_from = next_handle(&mut handle_counter);
            let new_pool_in = next_handle(&mut handle_counter);
            let new_pool_out = next_handle(&mut handle_counter);
            let new_to = next_handle(&mut handle_counter);

            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::FheSub(TfheContract::FheSub {
                        caller,
                        lhs: from_balance,
                        rhs: amount,
                        scalarByte: scalar_flag(false),
                        result: new_from,
                    })),
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
                    tfhe_event(TfheContractEvents::FheAdd(TfheContract::FheAdd {
                        caller,
                        lhs: pool_in,
                        rhs: amount,
                        scalarByte: scalar_flag(false),
                        result: new_pool_in,
                    })),
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
                    tfhe_event(TfheContractEvents::FheSub(TfheContract::FheSub {
                        caller,
                        lhs: pool_out,
                        rhs: selected_amount,
                        scalarByte: scalar_flag(false),
                        result: new_pool_out,
                    })),
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
            (new_from, new_pool_in, new_pool_out, new_to)
        };

        if i == num_samples.saturating_sub(1) {
            allow_handle(&listener_db, &mut tx, &new_to).await?;
            allow_handle(&listener_db, &mut tx, &new_from).await?;
        }

        prev_from = Some(new_from);
        prev_pool_in = Some(new_pool_in);
        prev_pool_out = Some(new_pool_out);
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

    Ok(())
}

async fn swap_request_whitepaper(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    _bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_dex(bencher, num_tx, true, false, false).await
}

async fn swap_request_no_cmux(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    _bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_dex(bencher, num_tx, false, false, false).await
}

async fn swap_claim_whitepaper(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    _bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_dex(bencher, num_tx, true, false, true).await
}

async fn swap_claim_no_cmux(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    _bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_dex(bencher, num_tx, false, false, true).await
}

async fn swap_request_whitepaper_dep(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    _bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_dex(bencher, num_tx, true, true, false).await
}

async fn swap_request_no_cmux_dep(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    _bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_dex(bencher, num_tx, false, true, false).await
}

async fn swap_claim_whitepaper_dep(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    _bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_dex(bencher, num_tx, true, true, true).await
}

async fn swap_claim_no_cmux_dep(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    _bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_dex(bencher, num_tx, false, true, true).await
}
