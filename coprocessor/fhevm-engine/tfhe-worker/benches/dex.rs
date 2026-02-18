#[path = "./utils.rs"]
mod utils;

use crate::utils::{
    as_scalar_uint, listener_event_db, next_handle, random_handle, scalar_flag, setup_test_app,
    tfhe_event, to_ty, wait_until_all_allowed_handles_computed, write_atomic_u64_bench_params,
    zero_address, EnvConfig,
};
use bigdecimal::num_bigint::BigInt;
use criterion::{
    async_executor::FuturesExecutor, measurement::WallTime, Bencher, Criterion, Throughput,
};
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{
    Database as ListenerDatabase, Handle, Transaction,
};
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

fn scalar_u128_handle(value: u128) -> Handle {
    let mut out = [0_u8; 32];
    out[16..].copy_from_slice(&value.to_be_bytes());
    Handle::from(out)
}

async fn insert_event(
    listener_db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    tx_id: Handle,
    event: TfheContractEvents,
    is_allowed: bool,
) -> Result<(), sqlx::Error> {
    utils::insert_tfhe_event(
        listener_db,
        tx,
        log_with_tx(tx_id, tfhe_event(event)),
        tx_id,
        is_allowed,
    )
    .await?;
    Ok(())
}

async fn insert_trivial_encrypt(
    listener_db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    tx_id: Handle,
    caller: alloy::primitives::Address,
    value: u64,
    to_type: i32,
    result: Handle,
) -> Result<(), sqlx::Error> {
    insert_event(
        listener_db,
        tx,
        tx_id,
        TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
            caller,
            pt: as_scalar_uint(&BigInt::from(value)),
            toType: to_ty(to_type),
            result,
        }),
        false,
    )
    .await
}

async fn schedule_dex(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    use_cmux: bool,
    dependent: bool,
    is_claim: bool,
    bench_id: &str,
    display_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let listener_db = listener_event_db(&app).await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(app.db_url())
        .await?;
    let caller = zero_address();
    let num_samples = sample_count(num_tx);
    let mut handle_counter = random_handle();
    let mut tx = listener_db.new_transaction().await?;
    let shared_tx_id = next_handle(&mut handle_counter);
    let setup_tx_id = if dependent {
        shared_tx_id
    } else {
        next_handle(&mut handle_counter)
    };

    if is_claim {
        let pending_0_in = next_handle(&mut handle_counter);
        insert_trivial_encrypt(
            &listener_db,
            &mut tx,
            setup_tx_id,
            caller,
            100,
            5,
            pending_0_in,
        )
        .await?;
        let pending_1_in = next_handle(&mut handle_counter);
        insert_trivial_encrypt(
            &listener_db,
            &mut tx,
            setup_tx_id,
            caller,
            200,
            5,
            pending_1_in,
        )
        .await?;
        let old_balance_0 = next_handle(&mut handle_counter);
        insert_trivial_encrypt(
            &listener_db,
            &mut tx,
            setup_tx_id,
            caller,
            700,
            5,
            old_balance_0,
        )
        .await?;
        let old_balance_1 = next_handle(&mut handle_counter);
        insert_trivial_encrypt(
            &listener_db,
            &mut tx,
            setup_tx_id,
            caller,
            900,
            5,
            old_balance_1,
        )
        .await?;
        let mut current_dex_balance_0 = next_handle(&mut handle_counter);
        insert_trivial_encrypt(
            &listener_db,
            &mut tx,
            setup_tx_id,
            caller,
            100,
            5,
            current_dex_balance_0,
        )
        .await?;
        let mut current_dex_balance_1 = next_handle(&mut handle_counter);
        insert_trivial_encrypt(
            &listener_db,
            &mut tx,
            setup_tx_id,
            caller,
            200,
            5,
            current_dex_balance_1,
        )
        .await?;

        let total_dex_token_0_in: u128 = 300;
        let total_dex_token_1_in: u128 = 600;
        let total_dex_token_0_out: u128 = 100;
        let total_dex_token_1_out: u128 = 200;

        for _ in 0..num_samples {
            let tx_id = if dependent {
                shared_tx_id
            } else {
                next_handle(&mut handle_counter)
            };

            if total_dex_token_1_in != 0 {
                let big_pending_1_in = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::Cast(TfheContract::Cast {
                        caller,
                        ct: pending_1_in,
                        toType: to_ty(6),
                        result: big_pending_1_in,
                    }),
                    false,
                )
                .await?;
                let mul_temp = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheMul(TfheContract::FheMul {
                        caller,
                        lhs: big_pending_1_in,
                        rhs: scalar_u128_handle(total_dex_token_0_out),
                        scalarByte: scalar_flag(true),
                        result: mul_temp,
                    }),
                    false,
                )
                .await?;
                let big_amount_0_out = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheDiv(TfheContract::FheDiv {
                        caller,
                        lhs: mul_temp,
                        rhs: scalar_u128_handle(total_dex_token_1_in),
                        scalarByte: scalar_flag(true),
                        result: big_amount_0_out,
                    }),
                    false,
                )
                .await?;
                let amount_0_out = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::Cast(TfheContract::Cast {
                        caller,
                        ct: big_amount_0_out,
                        toType: to_ty(5),
                        result: amount_0_out,
                    }),
                    false,
                )
                .await?;

                let has_enough_funds_handle_0 = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheGe(TfheContract::FheGe {
                        caller,
                        lhs: current_dex_balance_0,
                        rhs: amount_0_out,
                        scalarByte: scalar_flag(false),
                        result: has_enough_funds_handle_0,
                    }),
                    false,
                )
                .await?;

                if use_cmux {
                    let new_to_amount_target_handle_0 = next_handle(&mut handle_counter);
                    insert_event(
                        &listener_db,
                        &mut tx,
                        tx_id,
                        TfheContractEvents::FheAdd(TfheContract::FheAdd {
                            caller,
                            lhs: old_balance_0,
                            rhs: amount_0_out,
                            scalarByte: scalar_flag(false),
                            result: new_to_amount_target_handle_0,
                        }),
                        false,
                    )
                    .await?;
                    let new_to_amount_handle_0 = next_handle(&mut handle_counter);
                    insert_event(
                        &listener_db,
                        &mut tx,
                        tx_id,
                        TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                            caller,
                            control: has_enough_funds_handle_0,
                            ifTrue: new_to_amount_target_handle_0,
                            ifFalse: old_balance_0,
                            result: new_to_amount_handle_0,
                        }),
                        true,
                    )
                    .await?;
                    let new_from_amount_target_handle_0 = next_handle(&mut handle_counter);
                    insert_event(
                        &listener_db,
                        &mut tx,
                        tx_id,
                        TfheContractEvents::FheSub(TfheContract::FheSub {
                            caller,
                            lhs: current_dex_balance_0,
                            rhs: amount_0_out,
                            scalarByte: scalar_flag(false),
                            result: new_from_amount_target_handle_0,
                        }),
                        false,
                    )
                    .await?;
                    let new_from_amount_handle_0 = next_handle(&mut handle_counter);
                    insert_event(
                        &listener_db,
                        &mut tx,
                        tx_id,
                        TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                            caller,
                            control: has_enough_funds_handle_0,
                            ifTrue: new_from_amount_target_handle_0,
                            ifFalse: current_dex_balance_0,
                            result: new_from_amount_handle_0,
                        }),
                        true,
                    )
                    .await?;
                    if dependent {
                        current_dex_balance_0 = new_from_amount_handle_0;
                    }
                } else {
                    let cast_has_enough_funds_handle_0 = next_handle(&mut handle_counter);
                    insert_event(
                        &listener_db,
                        &mut tx,
                        tx_id,
                        TfheContractEvents::Cast(TfheContract::Cast {
                            caller,
                            ct: has_enough_funds_handle_0,
                            toType: to_ty(5),
                            result: cast_has_enough_funds_handle_0,
                        }),
                        false,
                    )
                    .await?;
                    let select_amount_handle_0 = next_handle(&mut handle_counter);
                    insert_event(
                        &listener_db,
                        &mut tx,
                        tx_id,
                        TfheContractEvents::FheMul(TfheContract::FheMul {
                            caller,
                            lhs: amount_0_out,
                            rhs: cast_has_enough_funds_handle_0,
                            scalarByte: scalar_flag(false),
                            result: select_amount_handle_0,
                        }),
                        false,
                    )
                    .await?;
                    let new_to_amount_handle_0 = next_handle(&mut handle_counter);
                    insert_event(
                        &listener_db,
                        &mut tx,
                        tx_id,
                        TfheContractEvents::FheAdd(TfheContract::FheAdd {
                            caller,
                            lhs: old_balance_0,
                            rhs: select_amount_handle_0,
                            scalarByte: scalar_flag(false),
                            result: new_to_amount_handle_0,
                        }),
                        true,
                    )
                    .await?;
                    let new_from_amount_handle_0 = next_handle(&mut handle_counter);
                    insert_event(
                        &listener_db,
                        &mut tx,
                        tx_id,
                        TfheContractEvents::FheSub(TfheContract::FheSub {
                            caller,
                            lhs: current_dex_balance_0,
                            rhs: select_amount_handle_0,
                            scalarByte: scalar_flag(false),
                            result: new_from_amount_handle_0,
                        }),
                        true,
                    )
                    .await?;
                    if dependent {
                        current_dex_balance_0 = new_from_amount_handle_0;
                    }
                }
            }

            if total_dex_token_0_in != 0 {
                let big_pending_0_in = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::Cast(TfheContract::Cast {
                        caller,
                        ct: pending_0_in,
                        toType: to_ty(6),
                        result: big_pending_0_in,
                    }),
                    false,
                )
                .await?;
                let mul_temp = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheMul(TfheContract::FheMul {
                        caller,
                        lhs: big_pending_0_in,
                        rhs: scalar_u128_handle(total_dex_token_1_out),
                        scalarByte: scalar_flag(true),
                        result: mul_temp,
                    }),
                    false,
                )
                .await?;
                let big_amount_1_out = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheDiv(TfheContract::FheDiv {
                        caller,
                        lhs: mul_temp,
                        rhs: scalar_u128_handle(total_dex_token_0_in),
                        scalarByte: scalar_flag(true),
                        result: big_amount_1_out,
                    }),
                    false,
                )
                .await?;
                let amount_1_out = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::Cast(TfheContract::Cast {
                        caller,
                        ct: big_amount_1_out,
                        toType: to_ty(5),
                        result: amount_1_out,
                    }),
                    false,
                )
                .await?;

                let has_enough_funds_handle_1 = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheGe(TfheContract::FheGe {
                        caller,
                        lhs: current_dex_balance_1,
                        rhs: amount_1_out,
                        scalarByte: scalar_flag(false),
                        result: has_enough_funds_handle_1,
                    }),
                    false,
                )
                .await?;

                if use_cmux {
                    let new_to_amount_target_handle_1 = next_handle(&mut handle_counter);
                    insert_event(
                        &listener_db,
                        &mut tx,
                        tx_id,
                        TfheContractEvents::FheAdd(TfheContract::FheAdd {
                            caller,
                            lhs: old_balance_1,
                            rhs: amount_1_out,
                            scalarByte: scalar_flag(false),
                            result: new_to_amount_target_handle_1,
                        }),
                        false,
                    )
                    .await?;
                    let new_to_amount_handle_1 = next_handle(&mut handle_counter);
                    insert_event(
                        &listener_db,
                        &mut tx,
                        tx_id,
                        TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                            caller,
                            control: has_enough_funds_handle_1,
                            ifTrue: new_to_amount_target_handle_1,
                            ifFalse: old_balance_1,
                            result: new_to_amount_handle_1,
                        }),
                        true,
                    )
                    .await?;
                    let new_from_amount_target_handle_1 = next_handle(&mut handle_counter);
                    insert_event(
                        &listener_db,
                        &mut tx,
                        tx_id,
                        TfheContractEvents::FheSub(TfheContract::FheSub {
                            caller,
                            lhs: current_dex_balance_1,
                            rhs: amount_1_out,
                            scalarByte: scalar_flag(false),
                            result: new_from_amount_target_handle_1,
                        }),
                        false,
                    )
                    .await?;
                    let new_from_amount_handle_1 = next_handle(&mut handle_counter);
                    insert_event(
                        &listener_db,
                        &mut tx,
                        tx_id,
                        TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                            caller,
                            control: has_enough_funds_handle_1,
                            ifTrue: new_from_amount_target_handle_1,
                            ifFalse: current_dex_balance_1,
                            result: new_from_amount_handle_1,
                        }),
                        true,
                    )
                    .await?;
                    if dependent {
                        current_dex_balance_1 = new_from_amount_handle_1;
                    }
                } else {
                    let cast_has_enough_funds_handle_1 = next_handle(&mut handle_counter);
                    insert_event(
                        &listener_db,
                        &mut tx,
                        tx_id,
                        TfheContractEvents::Cast(TfheContract::Cast {
                            caller,
                            ct: has_enough_funds_handle_1,
                            toType: to_ty(5),
                            result: cast_has_enough_funds_handle_1,
                        }),
                        false,
                    )
                    .await?;
                    let select_amount_handle_1 = next_handle(&mut handle_counter);
                    insert_event(
                        &listener_db,
                        &mut tx,
                        tx_id,
                        TfheContractEvents::FheMul(TfheContract::FheMul {
                            caller,
                            lhs: amount_1_out,
                            rhs: cast_has_enough_funds_handle_1,
                            scalarByte: scalar_flag(false),
                            result: select_amount_handle_1,
                        }),
                        false,
                    )
                    .await?;
                    let new_to_amount_handle_1 = next_handle(&mut handle_counter);
                    insert_event(
                        &listener_db,
                        &mut tx,
                        tx_id,
                        TfheContractEvents::FheAdd(TfheContract::FheAdd {
                            caller,
                            lhs: old_balance_1,
                            rhs: select_amount_handle_1,
                            scalarByte: scalar_flag(false),
                            result: new_to_amount_handle_1,
                        }),
                        true,
                    )
                    .await?;
                    let new_from_amount_handle_1 = next_handle(&mut handle_counter);
                    insert_event(
                        &listener_db,
                        &mut tx,
                        tx_id,
                        TfheContractEvents::FheSub(TfheContract::FheSub {
                            caller,
                            lhs: current_dex_balance_1,
                            rhs: select_amount_handle_1,
                            scalarByte: scalar_flag(false),
                            result: new_from_amount_handle_1,
                        }),
                        true,
                    )
                    .await?;
                    if dependent {
                        current_dex_balance_1 = new_from_amount_handle_1;
                    }
                }
            }
        }
    } else {
        let from_balance_0 = next_handle(&mut handle_counter);
        insert_trivial_encrypt(
            &listener_db,
            &mut tx,
            setup_tx_id,
            caller,
            100,
            5,
            from_balance_0,
        )
        .await?;
        let from_balance_1 = next_handle(&mut handle_counter);
        insert_trivial_encrypt(
            &listener_db,
            &mut tx,
            setup_tx_id,
            caller,
            200,
            5,
            from_balance_1,
        )
        .await?;
        let mut current_dex_balance_0 = next_handle(&mut handle_counter);
        insert_trivial_encrypt(
            &listener_db,
            &mut tx,
            setup_tx_id,
            caller,
            700,
            5,
            current_dex_balance_0,
        )
        .await?;
        let mut current_dex_balance_1 = next_handle(&mut handle_counter);
        insert_trivial_encrypt(
            &listener_db,
            &mut tx,
            setup_tx_id,
            caller,
            900,
            5,
            current_dex_balance_1,
        )
        .await?;
        let to_balance_0 = next_handle(&mut handle_counter);
        insert_trivial_encrypt(
            &listener_db,
            &mut tx,
            setup_tx_id,
            caller,
            100,
            5,
            to_balance_0,
        )
        .await?;
        let to_balance_1 = next_handle(&mut handle_counter);
        insert_trivial_encrypt(
            &listener_db,
            &mut tx,
            setup_tx_id,
            caller,
            200,
            5,
            to_balance_1,
        )
        .await?;
        let total_dex_token_0_in = next_handle(&mut handle_counter);
        insert_trivial_encrypt(
            &listener_db,
            &mut tx,
            setup_tx_id,
            caller,
            100,
            5,
            total_dex_token_0_in,
        )
        .await?;
        let total_dex_token_1_in = next_handle(&mut handle_counter);
        insert_trivial_encrypt(
            &listener_db,
            &mut tx,
            setup_tx_id,
            caller,
            200,
            5,
            total_dex_token_1_in,
        )
        .await?;
        let amount_0 = next_handle(&mut handle_counter);
        insert_trivial_encrypt(&listener_db, &mut tx, setup_tx_id, caller, 10, 5, amount_0).await?;
        let amount_1 = next_handle(&mut handle_counter);
        insert_trivial_encrypt(&listener_db, &mut tx, setup_tx_id, caller, 20, 5, amount_1).await?;

        for _ in 0..num_samples {
            let tx_id = if dependent {
                shared_tx_id
            } else {
                next_handle(&mut handle_counter)
            };

            let (new_current_balance_0, new_current_balance_1) = if use_cmux {
                let has_enough_funds_handle_0 = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheGe(TfheContract::FheGe {
                        caller,
                        lhs: from_balance_0,
                        rhs: amount_0,
                        scalarByte: scalar_flag(false),
                        result: has_enough_funds_handle_0,
                    }),
                    false,
                )
                .await?;
                let new_to_amount_target_handle_0 = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheAdd(TfheContract::FheAdd {
                        caller,
                        lhs: current_dex_balance_0,
                        rhs: amount_0,
                        scalarByte: scalar_flag(false),
                        result: new_to_amount_target_handle_0,
                    }),
                    false,
                )
                .await?;
                let new_to_amount_handle_0 = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                        caller,
                        control: has_enough_funds_handle_0,
                        ifTrue: new_to_amount_target_handle_0,
                        ifFalse: current_dex_balance_0,
                        result: new_to_amount_handle_0,
                    }),
                    false,
                )
                .await?;

                let has_enough_funds_handle_1 = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheGe(TfheContract::FheGe {
                        caller,
                        lhs: from_balance_1,
                        rhs: amount_1,
                        scalarByte: scalar_flag(false),
                        result: has_enough_funds_handle_1,
                    }),
                    false,
                )
                .await?;
                let new_to_amount_target_handle_1 = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheAdd(TfheContract::FheAdd {
                        caller,
                        lhs: current_dex_balance_1,
                        rhs: amount_1,
                        scalarByte: scalar_flag(false),
                        result: new_to_amount_target_handle_1,
                    }),
                    false,
                )
                .await?;
                let new_to_amount_handle_1 = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                        caller,
                        control: has_enough_funds_handle_1,
                        ifTrue: new_to_amount_target_handle_1,
                        ifFalse: current_dex_balance_1,
                        result: new_to_amount_handle_1,
                    }),
                    false,
                )
                .await?;
                (new_to_amount_handle_0, new_to_amount_handle_1)
            } else {
                let has_enough_funds_handle_0 = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheGe(TfheContract::FheGe {
                        caller,
                        lhs: from_balance_0,
                        rhs: amount_0,
                        scalarByte: scalar_flag(false),
                        result: has_enough_funds_handle_0,
                    }),
                    false,
                )
                .await?;
                let cast_has_enough_funds_handle_0 = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::Cast(TfheContract::Cast {
                        caller,
                        ct: has_enough_funds_handle_0,
                        toType: to_ty(5),
                        result: cast_has_enough_funds_handle_0,
                    }),
                    false,
                )
                .await?;
                let select_amount_handle_0 = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheMul(TfheContract::FheMul {
                        caller,
                        lhs: amount_0,
                        rhs: cast_has_enough_funds_handle_0,
                        scalarByte: scalar_flag(false),
                        result: select_amount_handle_0,
                    }),
                    false,
                )
                .await?;
                let new_to_amount_handle_0 = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheAdd(TfheContract::FheAdd {
                        caller,
                        lhs: current_dex_balance_0,
                        rhs: select_amount_handle_0,
                        scalarByte: scalar_flag(false),
                        result: new_to_amount_handle_0,
                    }),
                    false,
                )
                .await?;

                let has_enough_funds_handle_1 = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheGe(TfheContract::FheGe {
                        caller,
                        lhs: from_balance_1,
                        rhs: amount_1,
                        scalarByte: scalar_flag(false),
                        result: has_enough_funds_handle_1,
                    }),
                    false,
                )
                .await?;
                let cast_has_enough_funds_handle_1 = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::Cast(TfheContract::Cast {
                        caller,
                        ct: has_enough_funds_handle_1,
                        toType: to_ty(5),
                        result: cast_has_enough_funds_handle_1,
                    }),
                    false,
                )
                .await?;
                let select_amount_handle_1 = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheMul(TfheContract::FheMul {
                        caller,
                        lhs: amount_1,
                        rhs: cast_has_enough_funds_handle_1,
                        scalarByte: scalar_flag(false),
                        result: select_amount_handle_1,
                    }),
                    false,
                )
                .await?;
                let new_to_amount_handle_1 = next_handle(&mut handle_counter);
                insert_event(
                    &listener_db,
                    &mut tx,
                    tx_id,
                    TfheContractEvents::FheAdd(TfheContract::FheAdd {
                        caller,
                        lhs: current_dex_balance_1,
                        rhs: select_amount_handle_1,
                        scalarByte: scalar_flag(false),
                        result: new_to_amount_handle_1,
                    }),
                    false,
                )
                .await?;
                (new_to_amount_handle_0, new_to_amount_handle_1)
            };

            let sent_0_handle = next_handle(&mut handle_counter);
            insert_event(
                &listener_db,
                &mut tx,
                tx_id,
                TfheContractEvents::FheSub(TfheContract::FheSub {
                    caller,
                    lhs: new_current_balance_0,
                    rhs: current_dex_balance_0,
                    scalarByte: scalar_flag(false),
                    result: sent_0_handle,
                }),
                false,
            )
            .await?;
            let sent_1_handle = next_handle(&mut handle_counter);
            insert_event(
                &listener_db,
                &mut tx,
                tx_id,
                TfheContractEvents::FheSub(TfheContract::FheSub {
                    caller,
                    lhs: new_current_balance_1,
                    rhs: current_dex_balance_1,
                    scalarByte: scalar_flag(false),
                    result: sent_1_handle,
                }),
                false,
            )
            .await?;
            let pending_0_in_handle = next_handle(&mut handle_counter);
            insert_event(
                &listener_db,
                &mut tx,
                tx_id,
                TfheContractEvents::FheAdd(TfheContract::FheAdd {
                    caller,
                    lhs: to_balance_0,
                    rhs: sent_0_handle,
                    scalarByte: scalar_flag(false),
                    result: pending_0_in_handle,
                }),
                true,
            )
            .await?;
            let pending_1_in_handle = next_handle(&mut handle_counter);
            insert_event(
                &listener_db,
                &mut tx,
                tx_id,
                TfheContractEvents::FheAdd(TfheContract::FheAdd {
                    caller,
                    lhs: to_balance_1,
                    rhs: sent_1_handle,
                    scalarByte: scalar_flag(false),
                    result: pending_1_in_handle,
                }),
                true,
            )
            .await?;
            let pending_total_token_0_in = next_handle(&mut handle_counter);
            insert_event(
                &listener_db,
                &mut tx,
                tx_id,
                TfheContractEvents::FheAdd(TfheContract::FheAdd {
                    caller,
                    lhs: total_dex_token_0_in,
                    rhs: sent_0_handle,
                    scalarByte: scalar_flag(false),
                    result: pending_total_token_0_in,
                }),
                true,
            )
            .await?;
            let pending_total_token_1_in = next_handle(&mut handle_counter);
            insert_event(
                &listener_db,
                &mut tx,
                tx_id,
                TfheContractEvents::FheAdd(TfheContract::FheAdd {
                    caller,
                    lhs: total_dex_token_1_in,
                    rhs: sent_1_handle,
                    scalarByte: scalar_flag(false),
                    result: pending_total_token_1_in,
                }),
                true,
            )
            .await?;

            if dependent {
                current_dex_balance_0 = new_current_balance_0;
                current_dex_balance_1 = new_current_balance_1;
            }
        }
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

    write_atomic_u64_bench_params(&pool, bench_id, display_name).await?;
    Ok(())
}

async fn swap_request_whitepaper(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_dex(
        bencher,
        num_tx,
        true,
        false,
        false,
        &bench_id,
        "swap-request",
    )
    .await
}

async fn swap_request_no_cmux(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_dex(
        bencher,
        num_tx,
        false,
        false,
        false,
        &bench_id,
        "swap-request",
    )
    .await
}

async fn swap_claim_whitepaper(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_dex(bencher, num_tx, true, false, true, &bench_id, "swap-claim").await
}

async fn swap_claim_no_cmux(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_dex(bencher, num_tx, false, false, true, &bench_id, "swap-claim").await
}

async fn swap_request_whitepaper_dep(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_dex(
        bencher,
        num_tx,
        true,
        true,
        false,
        &bench_id,
        "swap-request",
    )
    .await
}

async fn swap_request_no_cmux_dep(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_dex(
        bencher,
        num_tx,
        false,
        true,
        false,
        &bench_id,
        "swap-request",
    )
    .await
}

async fn swap_claim_whitepaper_dep(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_dex(bencher, num_tx, true, true, true, &bench_id, "swap-claim").await
}

async fn swap_claim_no_cmux_dep(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_dex(bencher, num_tx, false, true, true, &bench_id, "swap-claim").await
}
