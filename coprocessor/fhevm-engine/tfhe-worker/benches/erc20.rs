#[path = "./utils.rs"]
mod utils;

use crate::utils::{
    allow_handle, as_handle, as_scalar_uint, listener_event_db, next_handle,
    persist_main_block_one_shot_artifact, persist_main_block_provenance,
    persist_main_block_smoke_artifact, random_handle, scalar_flag, setup_test_app, tfhe_event,
    to_ty, upsert_legacy_dependence_chain, validate_main_block_run_policy,
    wait_until_all_allowed_handles_computed, wait_until_legacy_terminals_computed,
    write_atomic_u64_bench_params, zero_address, EnvConfig, LegacyTerminal,
};
use criterion::{
    async_executor::FuturesExecutor, measurement::WallTime, Bencher, Criterion, Throughput,
};
use fhevm_engine_common::tfhe_ops::current_ciphertext_version;
use fhevm_engine_common::types::SupportedFheCiphertexts;
use fhevm_engine_common::utils::safe_deserialize_key;
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{
    Database as ListenerDatabase, Handle as ListenerHandle,
};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime};
use tfhe::prelude::CiphertextList;
use tfhe::xof_key_set::CompressedXofKeySet;
use tfhe::CompactCiphertextListExpander;
use tfhe_worker::benchmark_exact_tuples::ExactLegacyTerminalObserver;
use tfhe_worker::tfhe_worker::TIMING;
use tokio::runtime::Runtime;

fn main() {
    let main_block_only = std::env::var("FHEVM_BENCH_MAIN_BLOCK_ONLY").as_deref() == Ok("1");
    let one_shot = std::env::var("FHEVM_BENCH_ONE_SHOT").as_deref() == Ok("1");
    if std::env::var("FHEVM_BENCH_CRITERION_PROBE").as_deref() == Ok("1") {
        assert!(
            main_block_only,
            "FHEVM_BENCH_CRITERION_PROBE is reserved for main_block_baseline"
        );
        assert_eq!(
            validate_main_block_run_policy()
                .expect("validate main_block_baseline Criterion control probe policy"),
            "smoke_only",
            "FHEVM_BENCH_CRITERION_PROBE requires FHEVM_BENCH_RUN_MODE=smoke_only"
        );
        run_main_block_criterion_control_probe(
            main_block_smoke_criterion_config()
                .expect("strict main_block_baseline smoke Criterion configuration"),
        );
        return;
    }
    if std::env::var("FHEVM_BENCH_DIRECT_SMOKE").as_deref() == Ok("1") {
        assert!(
            main_block_only,
            "FHEVM_BENCH_DIRECT_SMOKE is reserved for main_block_baseline"
        );
        Runtime::new()
            .unwrap()
            .block_on(run_main_block_direct_smoke())
            .expect("run main_block_baseline direct smoke");
        return;
    }
    if one_shot {
        assert!(
            main_block_only,
            "FHEVM_BENCH_ONE_SHOT is reserved for main_block_baseline"
        );
        assert_eq!(
            validate_main_block_run_policy()
                .expect("validate main_block_baseline reportable one-shot policy"),
            "reportable",
            "main_block_baseline one-shot is reportable-only"
        );
        Runtime::new()
            .unwrap()
            .block_on(run_main_block_one_shot())
            .expect("run main_block_baseline one-shot");
        return;
    }
    let main_block_run_mode = if main_block_only {
        let run_mode = validate_main_block_run_policy()
            .expect("validate main_block_baseline reportable or smoke-only policy");
        if run_mode == "reportable" {
            let artifact = persist_main_block_provenance()
                .expect("persist canonical main_block_baseline provenance");
            println!("main_block_baseline artifact: {}", artifact.display());
        }
        Some(run_mode)
    } else {
        None
    };
    let ecfg = EnvConfig::new();
    if main_block_only && ecfg.benchmark_type != "THROUGHPUT" {
        panic!("FHEVM_BENCH_MAIN_BLOCK_ONLY=1 requires BENCHMARK_TYPE=THROUGHPUT");
    }
    let mut c = Criterion::default();
    if main_block_run_mode == Some("smoke_only") {
        let smoke = main_block_smoke_criterion_config()
            .expect("strict main_block_baseline smoke Criterion configuration");
        c = c
            .sample_size(smoke.sample_size)
            .warm_up_time(std::time::Duration::from_secs(smoke.warmup_secs))
            .measurement_time(std::time::Duration::from_secs(smoke.measurement_secs))
            .nresamples(smoke.nresamples);
    } else {
        c = c
            .sample_size(10)
            .measurement_time(std::time::Duration::from_secs(1000));
    }
    let mut c = c.configure_from_args();
    let bench_name = "erc20::transfer";
    let bench_optimization_target = if cfg!(feature = "latency") {
        "opt_latency"
    } else {
        "opt_throughput"
    };

    let mut group = c.benchmark_group(bench_name);
    if !main_block_only && (ecfg.benchmark_type == "LATENCY" || ecfg.benchmark_type == "ALL") {
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

    if !main_block_only && (ecfg.benchmark_type == "THROUGHPUT" || ecfg.benchmark_type == "ALL") {
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
    if main_block_only {
        let main_block_sizes: &[u64] = if main_block_run_mode == Some("smoke_only") {
            &[2]
        } else {
            &[10, 50, 200, 500]
        };
        for &num_elems in main_block_sizes {
            group.throughput(Throughput::Elements(num_elems));
            let bench_id = format!(
                "{bench_name}::throughput::main_block_baseline::legacy_tx/dependence_chain_no_host_block_provenance::FHEUint64::{num_elems}_elems::{bench_optimization_target}"
            );
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                schedule_main_block_erc20(b, num_elems as usize, bench_id.clone())
                    .expect("run exact-terminal main_block_baseline ERC20 benchmark");
            });
        }
    }
    group.finish();
    c.final_summary();
}

struct MainBlockSmokeCriterionConfig {
    warmup_secs: u64,
    measurement_secs: u64,
    sample_size: usize,
    nresamples: usize,
    max_requested_iters: u64,
}

fn main_block_smoke_criterion_config() -> Result<MainBlockSmokeCriterionConfig, String> {
    let warmup_secs = parse_positive_smoke_setting("FHEVM_BENCH_SMOKE_WARMUP_SECS", 1)?;
    let measurement_secs = parse_positive_smoke_setting("FHEVM_BENCH_SMOKE_MEASUREMENT_SECS", 1)?;
    let sample_size = parse_positive_smoke_setting("FHEVM_BENCH_SMOKE_SAMPLE_SIZE", 10)?;
    let nresamples = parse_positive_smoke_setting("FHEVM_BENCH_SMOKE_NRESAMPLES", 100)?;
    let max_requested_iters =
        parse_positive_smoke_setting("FHEVM_BENCH_SMOKE_MAX_REQUESTED_ITERS", 4)?;
    if sample_size < 10 {
        return Err(format!(
            "FHEVM_BENCH_SMOKE_SAMPLE_SIZE must be at least 10 for Criterion, got {sample_size}"
        ));
    }
    if max_requested_iters > 64 {
        return Err(format!(
            "FHEVM_BENCH_SMOKE_MAX_REQUESTED_ITERS must be at most 64, got {max_requested_iters}"
        ));
    }
    if nresamples > 1_000 {
        return Err(format!(
            "FHEVM_BENCH_SMOKE_NRESAMPLES must be at most 1000, got {nresamples}"
        ));
    }
    Ok(MainBlockSmokeCriterionConfig {
        warmup_secs,
        measurement_secs,
        sample_size: sample_size as usize,
        nresamples: nresamples as usize,
        max_requested_iters,
    })
}

fn parse_positive_smoke_setting(variable: &str, default: u64) -> Result<u64, String> {
    match std::env::var(variable) {
        Ok(value) => value
            .parse::<u64>()
            .map_err(|_| format!("{variable} must be a positive integer, got {value:?}"))
            .and_then(|value| {
                (value > 0)
                    .then_some(value)
                    .ok_or_else(|| format!("{variable} must be greater than zero"))
            }),
        Err(std::env::VarError::NotPresent) => Ok(default),
        Err(error) => Err(format!("read {variable}: {error}")),
    }
}

/// A no-DB smoke-only control that uses the exact main-block Criterion
/// configuration.  It documents Criterion's requested iteration sequence
/// without submitting any FHE work.
fn run_main_block_criterion_control_probe(smoke: MainBlockSmokeCriterionConfig) {
    eprintln!(
        "MAIN_BLOCK_TRACE probe_enter warmup_secs={} measurement_secs={} sample_size={} nresamples={} max_requested_iters={}",
        smoke.warmup_secs,
        smoke.measurement_secs,
        smoke.sample_size,
        smoke.nresamples,
        smoke.max_requested_iters,
    );
    let mut criterion = Criterion::default()
        .sample_size(smoke.sample_size)
        .warm_up_time(std::time::Duration::from_secs(smoke.warmup_secs))
        .measurement_time(std::time::Duration::from_secs(smoke.measurement_secs))
        .nresamples(smoke.nresamples)
        .configure_from_args();
    let mut group = criterion.benchmark_group("main_block_baseline::criterion_control_probe");
    group.bench_function("fixed_nonzero_iter_custom", |bencher| {
        bencher.iter_custom(|iters| {
            eprintln!("MAIN_BLOCK_TRACE probe_closure_enter requested_iters={iters}");
            let duration = std::time::Duration::from_millis(iters);
            eprintln!(
                "MAIN_BLOCK_TRACE probe_closure_return requested_iters={iters} returned_ms={}",
                duration.as_millis()
            );
            duration
        });
    });
    group.finish();
    criterion.final_summary();
    eprintln!("MAIN_BLOCK_TRACE probe_return");
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

/// Guards the custom Criterion contract: every requested iteration must
/// commit a fresh set of transaction/DCID identities before it is timed.
#[derive(Default)]
struct MainBlockSubmissionAudit {
    iterations: u64,
    transaction_ids: HashSet<Vec<u8>>,
    requested_calls: Vec<u64>,
    max_requested_iters: u64,
    db_computation_counts: Vec<i64>,
}

impl MainBlockSubmissionAudit {
    fn record_requested_call(&mut self, requested_iters: u64) {
        self.requested_calls.push(requested_iters);
        self.max_requested_iters = self.max_requested_iters.max(requested_iters);
    }

    fn record(
        &mut self,
        terminals: &[LegacyTerminal],
        expected_transactions: usize,
    ) -> Result<(), String> {
        let iteration_ids = terminals
            .iter()
            .map(|terminal| terminal.transaction_id.to_vec())
            .collect::<HashSet<_>>();
        if iteration_ids.len() != expected_transactions {
            return Err(format!(
                "main_block_baseline iteration staged {} transaction/DCID groups; expected {expected_transactions}",
                iteration_ids.len()
            ));
        }
        if iteration_ids
            .iter()
            .any(|transaction_id| !self.transaction_ids.insert(transaction_id.clone()))
        {
            return Err(
                "main_block_baseline reused a transaction/DCID across Criterion iterations".into(),
            );
        }
        self.iterations += 1;
        Ok(())
    }

    fn record_database_progress(&mut self, computation_count: i64) -> Result<(), String> {
        if self
            .db_computation_counts
            .last()
            .is_some_and(|previous| computation_count <= *previous)
        {
            return Err(format!(
                "main_block_baseline database computation count did not advance: {:?} then {computation_count}",
                self.db_computation_counts.last()
            ));
        }
        self.db_computation_counts.push(computation_count);
        Ok(())
    }
}

/// A self-contained legacy fixture: two transaction/DCID groups, where the
/// second consumes the first result through `public.ciphertexts`.  It does not
/// fabricate a host block or provenance context.
async fn run_main_block_direct_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = setup_test_app().await?;
    let result: Result<(), Box<dyn std::error::Error>> = async {
        let listener_db = listener_event_db(&app).await?;
        let mut counter = random_handle();
        let first_dcid = next_handle(&mut counter);
        let second_dcid = next_handle(&mut counter);
        let first_lhs = next_handle(&mut counter);
        let first_rhs = next_handle(&mut counter);
        let first_result = next_handle(&mut counter);
        let second_rhs = next_handle(&mut counter);
        let second_result = next_handle(&mut counter);
        let caller = zero_address();
        let started = Instant::now();
        let mut tx = listener_db
            .new_transaction()
            .await?
            .expect("new_transaction returns Some on a live legacy stack");

        for (value, result) in [(100_u64, first_lhs), (10_u64, first_rhs)] {
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    first_dcid,
                    tfhe_event(TfheContractEvents::TrivialEncrypt(
                        TfheContract::TrivialEncrypt {
                            caller,
                            pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(value)),
                            toType: to_ty(5),
                            result,
                        },
                    )),
                ),
                first_dcid,
                false,
            )
            .await?;
        }
        utils::insert_tfhe_event(
            &listener_db,
            &mut tx,
            log_with_tx(
                first_dcid,
                tfhe_event(TfheContractEvents::FheAdd(TfheContract::FheAdd {
                    caller,
                    lhs: first_lhs,
                    rhs: first_rhs,
                    scalarByte: scalar_flag(false),
                    result: first_result,
                })),
            ),
            first_dcid,
            true,
        )
        .await?;
        utils::insert_tfhe_event(
            &listener_db,
            &mut tx,
            log_with_tx(
                second_dcid,
                tfhe_event(TfheContractEvents::TrivialEncrypt(
                    TfheContract::TrivialEncrypt {
                        caller,
                        pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(2_u64)),
                        toType: to_ty(5),
                        result: second_rhs,
                    },
                )),
            ),
            second_dcid,
            false,
        )
        .await?;
        utils::insert_tfhe_event(
            &listener_db,
            &mut tx,
            log_with_tx(
                second_dcid,
                tfhe_event(TfheContractEvents::FheAdd(TfheContract::FheAdd {
                    caller,
                    lhs: first_result,
                    rhs: second_rhs,
                    scalarByte: scalar_flag(false),
                    result: second_result,
                })),
            ),
            second_dcid,
            true,
        )
        .await?;
        upsert_legacy_dependence_chain(&mut tx, &first_dcid, 0, &[second_dcid]).await?;
        upsert_legacy_dependence_chain(&mut tx, &second_dcid, 1, &[]).await?;
        let relation_snapshot: (Vec<Vec<u8>>, i32) = sqlx::query_as(
            "SELECT first.dependents, second.dependency_count \
             FROM dependence_chain first \
             JOIN dependence_chain second ON second.dependence_chain_id = $2 \
             WHERE first.dependence_chain_id = $1",
        )
        .bind(first_dcid.to_vec())
        .bind(second_dcid.to_vec())
        .fetch_one(tx.as_mut())
        .await?;
        if !relation_snapshot
            .0
            .iter()
            .any(|dependent| dependent == second_dcid.as_slice())
            || relation_snapshot.1 != 1
        {
            return Err("direct smoke failed to stage the native first->second DCID dependency".into());
        }
        allow_handle(&listener_db, &mut tx, &second_result).await?;
        tx.commit().await?;

        let terminals = [
            LegacyTerminal {
                handle: first_result,
                transaction_id: first_dcid,
            },
            LegacyTerminal {
                handle: second_result,
                transaction_id: second_dcid,
            },
        ];
        wait_until_legacy_terminals_computed(app.db_url().to_owned(), &terminals).await?;

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(app.db_url())
            .await?;
        let computation_count: i64 = sqlx::query_scalar(
            "SELECT count(1) FROM computations WHERE host_chain_id = 42",
        )
        .fetch_one(&pool)
        .await?;
        let terminal_ciphertext_count: i64 = sqlx::query_scalar(
            "SELECT count(1) FROM ciphertexts WHERE handle = ANY($1::bytea[])",
        )
        .bind(vec![first_result.to_vec(), second_result.to_vec()])
        .fetch_one(&pool)
        .await?;
        let host_block_count: i64 = sqlx::query_scalar(
            "SELECT count(1) FROM host_chain_blocks_valid WHERE chain_id = 42",
        )
        .fetch_one(&pool)
        .await?;
        let dcids: Vec<(Vec<u8>, String, i32)> = sqlx::query_as(
            "SELECT dependence_chain_id, status, dependency_count FROM dependence_chain \
             WHERE dependence_chain_id = ANY($1::bytea[]) ORDER BY dependence_chain_id",
        )
        .bind(vec![first_dcid.to_vec(), second_dcid.to_vec()])
        .fetch_all(&pool)
        .await?;
        let terminal_completion_ms: Vec<(Vec<u8>, f64)> = sqlx::query_as(
            "SELECT transaction_id, (EXTRACT(EPOCH FROM MAX(completed_at)) * 1000)::float8 \
             FROM computations \
             WHERE (output_handle, transaction_id) IN ( \
                 SELECT * FROM unnest($1::bytea[], $2::bytea[]) \
             ) \
             GROUP BY transaction_id",
        )
        .bind(vec![first_result.to_vec(), second_result.to_vec()])
        .bind(vec![first_dcid.to_vec(), second_dcid.to_vec()])
        .fetch_all(&pool)
        .await?;
        let first_completed_ms = terminal_completion_ms
            .iter()
            .find_map(|(transaction_id, completed_ms)| {
                (transaction_id.as_slice() == first_dcid.as_slice()).then_some(*completed_ms)
            });
        let second_completed_ms = terminal_completion_ms
            .iter()
            .find_map(|(transaction_id, completed_ms)| {
                (transaction_id.as_slice() == second_dcid.as_slice()).then_some(*completed_ms)
            });
        if computation_count != 5
            || terminal_ciphertext_count != 2
            || host_block_count != 0
            || dcids.len() != 2
            || dcids.iter().any(|(_, status, dependencies)| {
                status != "processed" || *dependencies != 0
            })
            || !matches!((first_completed_ms, second_completed_ms), (Some(first), Some(second)) if first < second)
        {
            return Err(format!(
                "legacy direct-smoke topology assertion failed: computations={computation_count}, terminal_ciphertexts={terminal_ciphertext_count}, host_blocks={host_block_count}, dcids={dcids:?}"
            )
            .into());
        }
        let artifact = persist_main_block_smoke_artifact(
            "direct_two_dcid_dependency",
            serde_json::json!({
                "logical_workload": "two_transaction_add_dependency",
                "topology": "legacy_tx/dependence_chain_no_host_block_provenance",
                "legacy_computations": computation_count,
                "terminal_ciphertexts": terminal_ciphertext_count,
                "host_chain_blocks_valid": host_block_count,
                "exact_completion_ms": started.elapsed().as_millis(),
                "initial_native_dcid_relation": {
                    "first": hex::encode(first_dcid),
                    "first_dependents": relation_snapshot.0.into_iter().map(hex::encode).collect::<Vec<_>>(),
                    "second": hex::encode(second_dcid),
                    "second_initial_dependency_count": relation_snapshot.1,
                },
                "terminal_completion_ms": {
                    "first": first_completed_ms,
                    "second": second_completed_ms,
                    "first_completed_before_second": true,
                },
                "dcids": dcids.into_iter().map(|(id, status, dependency_count)| serde_json::json!({
                    "id": hex::encode(id), "status": status, "dependency_count": dependency_count
                })).collect::<Vec<_>>(),
            }),
        )?;
        println!("main_block_baseline direct smoke artifact: {}", artifact.display());
        Ok(())
    }
    .await;
    let shutdown = app.shutdown().await;
    result?;
    shutdown?;
    Ok(())
}

#[derive(Clone, Copy)]
struct MainBlockOneShotScenario {
    name: &'static str,
    workload: MainBlockWorkload,
    transfers: usize,
    chain_len: usize,
    erc20_transaction_identity: Erc20TransactionIdentity,
    staging: MainBlockOneShotStaging,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MainBlockWorkload {
    Erc20,
    Auction,
}

/// Whether an ERC20 chain is represented by one legacy transaction identity
/// or by a distinct identity for every transfer. The latter is the targeted
/// same-block cross-transaction workload: its carried balances necessarily
/// cross a transaction boundary even though all work shares one dependence
/// chain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Erc20TransactionIdentity {
    PerChain,
    PerTransfer,
}

/// The targeted cross-transaction benchmark contains 50 independent ERC20
/// state chains, each with 20 transfers in the same logical block. Every
/// transfer has its own transaction identity, so both carried balance handles
/// cross a transaction boundary on every non-root link.
const SAME_BLOCK_CROSS_TX_ERC20_CHAINS: usize = 50;
const SAME_BLOCK_CROSS_TX_ERC20_CHAIN_LEN: usize = 20;
const SAME_BLOCK_CROSS_TX_ERC20_TRANSFERS: usize =
    SAME_BLOCK_CROSS_TX_ERC20_CHAINS * SAME_BLOCK_CROSS_TX_ERC20_CHAIN_LEN;
const SAME_BLOCK_CROSS_TX_ERC20_BALANCE_EDGES_PER_LINK: usize = 2;

/// The native Main control has no host-block provenance table.  This mode is
/// therefore deliberately a sequence of legacy fixture transactions: each is
/// committed immediately and the worker observes the same computation and
/// dependence-chain tables it uses in production.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MainBlockOneShotStaging {
    OneLogicalBlock,
    SequentialUnpacedTraffic {
        l1_blocks: usize,
        transfers_per_block: usize,
        chain_count: usize,
        dependence_lag_blocks: usize,
        join_braid: bool,
    },
}

impl MainBlockOneShotScenario {
    fn chain_count(self) -> usize {
        match self.staging {
            MainBlockOneShotStaging::OneLogicalBlock => self.transfers.div_ceil(self.chain_len),
            MainBlockOneShotStaging::SequentialUnpacedTraffic { chain_count, .. } => chain_count,
        }
    }

    fn terminal_handle_count(self) -> usize {
        match self.staging {
            MainBlockOneShotStaging::SequentialUnpacedTraffic {
                join_braid: true, ..
            } => self.chain_count(),
            _ => self.chain_count() * 2,
        }
    }

    fn is_unpaced_traffic(self) -> bool {
        matches!(
            self.staging,
            MainBlockOneShotStaging::SequentialUnpacedTraffic { .. }
        )
    }

    fn uses_distinct_transfer_transaction_ids(self) -> bool {
        self.workload == MainBlockWorkload::Erc20
            && self.erc20_transaction_identity == Erc20TransactionIdentity::PerTransfer
            && !self.is_unpaced_traffic()
    }

    fn transaction_identity_label(self) -> &'static str {
        if self.is_unpaced_traffic() {
            "one_transaction_id_per_transfer_cross_block_lag2"
        } else {
            match self.erc20_transaction_identity {
                Erc20TransactionIdentity::PerChain => "one_transaction_id_per_chain",
                Erc20TransactionIdentity::PerTransfer => {
                    "one_transaction_id_per_transfer_cross_transaction_dependencies"
                }
            }
        }
    }

    fn transaction_id_count(self) -> usize {
        if self.is_unpaced_traffic() || self.uses_distinct_transfer_transaction_ids() {
            self.transfers
        } else {
            self.chain_count()
        }
    }

    fn cross_transaction_balance_dependency_edge_count(self) -> usize {
        if self.uses_distinct_transfer_transaction_ids() {
            self.chain_count()
                * (self.chain_len - 1)
                * SAME_BLOCK_CROSS_TX_ERC20_BALANCE_EDGES_PER_LINK
        } else {
            0
        }
    }
}

fn unpaced_traffic_chain_index(
    block_index: usize,
    index_in_block: usize,
    transfers_per_block: usize,
    dependence_lag_blocks: usize,
) -> usize {
    assert!(dependence_lag_blocks > 0);
    (block_index % dependence_lag_blocks) * transfers_per_block + index_in_block
}

const MAIN_BLOCK_ONE_SHOT_SCENARIOS: &[MainBlockOneShotScenario] = &[
    MainBlockOneShotScenario {
        name: "independent_300",
        workload: MainBlockWorkload::Erc20,
        transfers: 300,
        chain_len: 1,
        erc20_transaction_identity: Erc20TransactionIdentity::PerChain,
        staging: MainBlockOneShotStaging::OneLogicalBlock,
    },
    // Canonical workload #2, derived from the gpu-e2e-perf dependent ERC20
    // pattern: 50 independent chains, each with 20 sequential transfers.
    MainBlockOneShotScenario {
        name: "dependent_1000_50x20",
        workload: MainBlockWorkload::Erc20,
        transfers: 1_000,
        chain_len: 20,
        erc20_transaction_identity: Erc20TransactionIdentity::PerChain,
        staging: MainBlockOneShotStaging::OneLogicalBlock,
    },
    // Canonical workload #3: unpaced L1 traffic. Five transfers are committed
    // in each of 200 sequential fixture transactions. Blocks of the same
    // parity advance the same five chains, so each transfer consumes both
    // balances emitted exactly two L1 blocks earlier.
    MainBlockOneShotScenario {
        name: "traffic_1000_200x5_10x100_lag2",
        workload: MainBlockWorkload::Erc20,
        transfers: 1_000,
        chain_len: 100,
        erc20_transaction_identity: Erc20TransactionIdentity::PerTransfer,
        staging: MainBlockOneShotStaging::SequentialUnpacedTraffic {
            l1_blocks: 200,
            transfers_per_block: 5,
            chain_count: 10,
            dependence_lag_blocks: 2,
            join_braid: false,
        },
    },
    // Canonical workload #6: braided cross-chain traffic through real
    // ingestion. Same cadence as workload #3, but every transfer moves value
    // between two accounts of its parity deck: each transfer joins two
    // account chains, so no linear extension is possible and formation must
    // gate every transfer on both parents.
    MainBlockOneShotScenario {
        name: "traffic_join_1000_200x5_20acct_lag2",
        workload: MainBlockWorkload::Erc20,
        transfers: 1_000,
        chain_len: 100,
        erc20_transaction_identity: Erc20TransactionIdentity::PerTransfer,
        staging: MainBlockOneShotStaging::SequentialUnpacedTraffic {
            l1_blocks: 200,
            transfers_per_block: 5,
            chain_count: 20,
            dependence_lag_blocks: 2,
            join_braid: true,
        },
    },
    // Canonical workload #5: the direct same-block cross-transaction case.
    // Every link carries both balances to a different transaction identity
    // while retaining the chain's one dependence-chain boundary.
    MainBlockOneShotScenario {
        name: "cross_tx_dependent_1000_50x20",
        workload: MainBlockWorkload::Erc20,
        transfers: SAME_BLOCK_CROSS_TX_ERC20_TRANSFERS,
        chain_len: SAME_BLOCK_CROSS_TX_ERC20_CHAIN_LEN,
        erc20_transaction_identity: Erc20TransactionIdentity::PerTransfer,
        staging: MainBlockOneShotStaging::OneLogicalBlock,
    },
    // Native port of gpu-e2e-perf's ConfidentialAuctionBidBench batch.  It
    // is one L1 block with 300 bids, 150 bidder chains, 39 price aggregates,
    // and 32 holding-wallet aggregates.
    MainBlockOneShotScenario {
        name: "auction_300",
        workload: MainBlockWorkload::Auction,
        transfers: 300,
        chain_len: 2,
        erc20_transaction_identity: Erc20TransactionIdentity::PerTransfer,
        staging: MainBlockOneShotStaging::OneLogicalBlock,
    },
];

fn main_block_one_shot_scenario() -> Result<MainBlockOneShotScenario, Box<dyn std::error::Error>> {
    let requested = std::env::var("FHEVM_BENCH_ONE_SHOT_SCENARIO")
        .unwrap_or_else(|_| "independent_300".to_owned());
    let Some(scenario) = MAIN_BLOCK_ONE_SHOT_SCENARIOS
        .iter()
        .copied()
        .find(|scenario| scenario.name == requested)
    else {
        return Err(format!(
            "unknown FHEVM_BENCH_ONE_SHOT_SCENARIO={requested:?}; expected one of {:?}",
            MAIN_BLOCK_ONE_SHOT_SCENARIOS
                .iter()
                .map(|scenario| scenario.name)
                .collect::<Vec<_>>(),
        )
        .into());
    };
    if scenario.name == "dependent_1000_50x20" {
        assert_eq!(scenario.transfers, 1_000);
        assert_eq!(scenario.chain_len, 20);
        assert_eq!(scenario.chain_count(), 50);
        assert_eq!(
            scenario.erc20_transaction_identity,
            Erc20TransactionIdentity::PerChain
        );
    }
    if scenario.name == "cross_tx_dependent_1000_50x20" {
        assert_eq!(scenario.transfers, SAME_BLOCK_CROSS_TX_ERC20_TRANSFERS);
        assert_eq!(scenario.chain_len, SAME_BLOCK_CROSS_TX_ERC20_CHAIN_LEN);
        assert_eq!(scenario.chain_count(), SAME_BLOCK_CROSS_TX_ERC20_CHAINS);
        assert!(scenario.uses_distinct_transfer_transaction_ids());
        assert_eq!(scenario.transaction_id_count(), 1_000);
        assert_eq!(
            scenario.cross_transaction_balance_dependency_edge_count(),
            1_900
        );
    }
    if scenario.name == "traffic_1000_200x5_10x100_lag2" {
        assert_eq!(scenario.transfers, 1_000);
        assert_eq!(scenario.chain_len, 100);
        assert_eq!(scenario.chain_count(), 10);
        assert_eq!(scenario.terminal_handle_count(), 20);
        assert_eq!(
            scenario.staging,
            MainBlockOneShotStaging::SequentialUnpacedTraffic {
                l1_blocks: 200,
                transfers_per_block: 5,
                chain_count: 10,
                dependence_lag_blocks: 2,
                join_braid: false,
            }
        );
    }
    if scenario.name == "traffic_join_1000_200x5_20acct_lag2" {
        assert_eq!(scenario.transfers, 1_000);
        assert_eq!(scenario.chain_count(), 20);
        assert_eq!(scenario.terminal_handle_count(), 20);
        assert_eq!(
            scenario.staging,
            MainBlockOneShotStaging::SequentialUnpacedTraffic {
                l1_blocks: 200,
                transfers_per_block: 5,
                chain_count: 20,
                dependence_lag_blocks: 2,
                join_braid: true,
            }
        );
    }
    if scenario.name == "auction_300" {
        assert_eq!(scenario.workload, MainBlockWorkload::Auction);
        assert_eq!(scenario.transfers, 300);
    }
    Ok(scenario)
}

struct MainBlockOneShotOutcome {
    post_commit_worker_visible_to_terminal_outputs: std::time::Duration,
    commit_start_to_terminal_outputs: std::time::Duration,
    terminal_handle_count: usize,
    computation_count: i64,
    transaction_id_count: i64,
    dependence_chain_count: i64,
    dependence_edge_count: i64,
    blocks_committed: usize,
    unpaced_ingestion: bool,
}

/// Pinned-main equivalents of the canonical ERC20 one-shot workloads.
///
/// Main stores native legacy computations rather than branch/block contexts.
/// Each transfer receives the same five-operation graph: `Ge`, trivial
/// encrypted `0`, select, `Add`, `Sub`; a dependency chain forwards its two
/// balance outputs to its successor. Boundary ciphertexts are encrypted from
/// the shared XOF fixture before staging, so neither input encryption nor
/// staging is timed.
async fn run_main_block_one_shot() -> Result<(), Box<dyn std::error::Error>> {
    let scenario = main_block_one_shot_scenario()?;
    let dispatch = serde_json::json!({
        "work_items_batch_size": utils::benchmark_work_items_batch_size(EnvConfig::new().batch_size)?,
        "dependence_chains_per_batch": utils::benchmark_dependence_chains_per_batch(2000)?,
        "dcid_batch_execution": utils::benchmark_dcid_batch_execution()?,
        "dcid_adaptive_batch_execution": utils::benchmark_dcid_adaptive_batch_execution()?,
        // Recorded because it is not free: at INFO the listener logs every
        // ingested event, inside the window the traffic scenarios measure.
        "log_level": utils::benchmark_log_level()?.to_string(),
        // Per device, so a multi-GPU host runs this many times its device
        // count. Governs how much of a batch a GPU overlaps, which a reported
        // GPU number cannot be compared without.
        "gpu_streams_per_device": utils::benchmark_gpu_streams_per_device()?,
    });
    let mut app = setup_test_app().await?;
    // Read the key's parameters before staging, so the record costs nothing
    // inside the measured window.
    let bench_parameters = {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(app.db_url())
            .await?;
        utils::atomic_u64_bench_params_json(
            &pool,
            &format!("erc20::transfer::main_block_one_shot::{}", scenario.name),
        )
        .await?
    };
    let result: Result<MainBlockOneShotOutcome, Box<dyn std::error::Error>> = async {
        let listener_db = listener_event_db(&app).await?;
        if scenario.workload == MainBlockWorkload::Auction {
            return run_main_block_auction_300(&listener_db, app.db_url()).await;
        }
        if scenario.is_unpaced_traffic() {
            return run_main_block_unpaced_traffic(&listener_db, app.db_url(), scenario).await;
        }
        // Keep observer setup and LISTEN registration outside the reported
        // worker-visible interval, matching the branch benchmark harness.
        let mut terminal_observer = ExactLegacyTerminalObserver::connect(app.db_url()).await?;
        let inputs = encrypted_u64_inputs_from_xof()?;
        let caller = zero_address();
        let mut counter = random_handle();
        let mut terminals = Vec::with_capacity(scenario.terminal_handle_count());
        let mut transaction_ids = Vec::with_capacity(scenario.transaction_id_count());
        let mut dependence_chain_ids = Vec::with_capacity(scenario.chain_count());
        let mut tx = listener_db
            .new_transaction()
            .await?
            .expect("new_transaction returns Some on a live legacy stack");

        for _ in 0..scenario.chain_count() {
            // A legacy transaction/DCID carries one dependency chain. This is
            // the native-main representation of the gpu-e2e-perf dependency
            // pattern. The cross-transaction scenario deliberately decouples
            // the DCID from the transfer transaction identities.
            let dependence_chain_id = next_handle(&mut counter);
            let chain_transaction_id = (scenario.erc20_transaction_identity
                == Erc20TransactionIdentity::PerChain)
                .then_some(dependence_chain_id);
            let mut from = next_handle(&mut counter);
            let mut to = next_handle(&mut counter);
            seed_legacy_input_ciphertext(&mut tx, &from, &inputs[0]).await?;
            seed_legacy_input_ciphertext(&mut tx, &to, &inputs[1]).await?;
            let mut terminal_transaction_id = None;
            for position in 0..scenario.chain_len {
                let transaction_id =
                    chain_transaction_id.unwrap_or_else(|| next_handle(&mut counter));
                if scenario.uses_distinct_transfer_transaction_ids() {
                    transaction_ids.push(transaction_id);
                }
                let amount = next_handle(&mut counter);
                seed_legacy_input_ciphertext(&mut tx, &amount, &inputs[2]).await?;
                let has_funds = next_handle(&mut counter);
                insert_legacy_one_shot_event_in_dependence_chain(&listener_db, &mut tx, transaction_id, dependence_chain_id, TfheContractEvents::FheGe(TfheContract::FheGe { caller, lhs: from, rhs: amount, scalarByte: scalar_flag(false), result: has_funds }), false).await?;
                let zero = next_handle(&mut counter);
                insert_legacy_one_shot_event_in_dependence_chain(&listener_db, &mut tx, transaction_id, dependence_chain_id, TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt { caller, pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(0_u64)), toType: to_ty(5), result: zero }), false).await?;
                let selected = next_handle(&mut counter);
                insert_legacy_one_shot_event_in_dependence_chain(&listener_db, &mut tx, transaction_id, dependence_chain_id, TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse { caller, control: has_funds, ifTrue: amount, ifFalse: zero, result: selected }), false).await?;
                let new_to = next_handle(&mut counter);
                let terminal = position + 1 == scenario.chain_len;
                // A carried balance consumed by the NEXT transaction must be
                // allowed: production ACL semantics guarantee a persistent
                // allow() for every cross-transaction consumed handle, and
                // Main only schedules transactions holding an incomplete
                // allowed output. Intra-transaction carries (per-chain
                // identity) stay unallowed intermediates.
                let materialized_boundary =
                    terminal || scenario.uses_distinct_transfer_transaction_ids();
                insert_legacy_one_shot_event_in_dependence_chain(&listener_db, &mut tx, transaction_id, dependence_chain_id, TfheContractEvents::FheAdd(TfheContract::FheAdd { caller, lhs: to, rhs: selected, scalarByte: scalar_flag(false), result: new_to }), materialized_boundary).await?;
                let new_from = next_handle(&mut counter);
                insert_legacy_one_shot_event_in_dependence_chain(&listener_db, &mut tx, transaction_id, dependence_chain_id, TfheContractEvents::FheSub(TfheContract::FheSub { caller, lhs: from, rhs: selected, scalarByte: scalar_flag(false), result: new_from }), materialized_boundary).await?;
                from = new_from;
                to = new_to;
                terminal_transaction_id = Some(transaction_id);
            }
            let terminal_transaction_id =
                terminal_transaction_id.expect("chain has a transfer");
            if let Some(transaction_id) = chain_transaction_id {
                transaction_ids.push(transaction_id);
            }
            upsert_legacy_dependence_chain(&mut tx, &dependence_chain_id, 0, &[]).await?;
            terminals.extend([
                LegacyTerminal {
                    handle: to,
                    transaction_id: terminal_transaction_id,
                },
                LegacyTerminal {
                    handle: from,
                    transaction_id: terminal_transaction_id,
                },
            ]);
            dependence_chain_ids.push(dependence_chain_id);
        }

        let commit_started = Instant::now();
        tx.commit().await?;
        // The primary clock intentionally starts after the fixture transaction
        // has committed and is eligible for worker observation.
        let worker_visible_at = Instant::now();
        let exact_terminals = terminals
            .iter()
            .map(|terminal| (terminal.handle.to_vec(), terminal.transaction_id.to_vec()))
            .collect::<Vec<_>>();
        let completed_at = terminal_observer
            .wait_until_completed(
                &exact_terminals,
                utils::benchmark_wait_timeout()?,
                false,
            )
            .await?;
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(app.db_url())
            .await?;
        let transaction_ids = transaction_ids
            .iter()
            .map(|id| id.to_vec())
            .collect::<Vec<_>>();
        let transaction_id_count: i64 = sqlx::query_scalar(
            "SELECT count(DISTINCT transaction_id) FROM computations WHERE transaction_id = ANY($1::bytea[])",
        )
        .bind(&transaction_ids)
        .fetch_one(&pool)
        .await?;
        let computation_count: i64 = sqlx::query_scalar(
            "SELECT count(1) FROM computations WHERE transaction_id = ANY($1::bytea[])",
        )
        .bind(&transaction_ids)
        .fetch_one(&pool)
        .await?;
        let dependence_chain_count: i64 = sqlx::query_scalar(
            "SELECT count(1) FROM dependence_chain WHERE dependence_chain_id = ANY($1::bytea[])",
        )
        .bind(
            dependence_chain_ids
                .iter()
                .map(|id| id.to_vec())
                .collect::<Vec<_>>(),
        )
        .fetch_one(&pool)
        .await?;
        let expected_computations = (scenario.transfers * 5) as i64;
        if computation_count != expected_computations
            || transaction_id_count != scenario.transaction_id_count() as i64
            || dependence_chain_count != scenario.chain_count() as i64
        {
            return Err(format!(
                "main_block_baseline {} topology mismatch: computations={computation_count} (expected {expected_computations}), transaction_ids={transaction_id_count} (expected {}), dependence_chains={dependence_chain_count} (expected {})",
                scenario.name, scenario.transaction_id_count(), scenario.chain_count(),
            )
            .into());
        }
        Ok(MainBlockOneShotOutcome {
            post_commit_worker_visible_to_terminal_outputs: completed_at
                .duration_since(worker_visible_at),
            commit_start_to_terminal_outputs: completed_at.duration_since(commit_started),
            terminal_handle_count: terminals.len(),
            computation_count,
            transaction_id_count,
            dependence_chain_count,
            dependence_edge_count: 0,
            blocks_committed: 1,
            unpaced_ingestion: false,
        })
    }
    .await;
    let shutdown = app.shutdown().await;
    let outcome = result?;
    shutdown?;
    let artifact = persist_main_block_one_shot_artifact(serde_json::json!({
        "scenario": scenario.name,
        "logical_block": {
            "transfer_count": scenario.transfers,
            "block_count": outcome.blocks_committed,
        },
        "topology": "legacy_tx/dependence_chain_no_host_block_provenance",
        "worker_semantics": "main_native_legacy_computations_ciphertexts_dependence_chain",
        "measurement_methodology": {
            "primary_metric": if outcome.unpaced_ingestion {
                "post_first_commit_worker_visible_to_final_terminal_outputs"
            } else {
                "post_commit_worker_visible_to_terminal_outputs"
            },
            "ingestion": if outcome.unpaced_ingestion {
                "200 sequential L1 fixture commits without artificial pacing; worker compute overlaps ingestion"
            } else {
                "one logical block committed after fixture staging"
            },
            "fixture_staging_and_input_encryption_before_first_commit_in_primary_metric": false,
            "subsequent_l1_staging_and_input_encryption_in_primary_metric": outcome.unpaced_ingestion,
            "worker_readiness_barrier": "in_process_listener_dcid_acquisition_and_db_key_cache_ready_before_fixture_staging",
            "cold_worker_startup_and_key_initialization_in_primary_metric": false,
        },
        "transfer_topology": {
            "independent_dependence_chains": scenario.chain_count(),
            "dependent_transfers_per_chain": scenario.chain_len,
            "transaction_identity": scenario.transaction_identity_label(),
            "transaction_id_count": outcome.transaction_id_count,
            "same_block_cross_transaction_balance_dependency_edge_count": scenario.cross_transaction_balance_dependency_edge_count(),
            "dependence_lag_l1_blocks": if outcome.unpaced_ingestion { Some(2) } else { None },
            "l1_block_count": outcome.blocks_committed,
            "preencrypted_balance_inputs_per_chain": 2,
            "preencrypted_amount_inputs_per_transfer": 1,
            "operations_per_transfer": ["FheGe", "TrivialEncrypt(0,FheUint64)", "FheIfThenElse", "FheAdd", "FheSub"],
            "operations_per_transfer_count": 5,
        },
        "auction_staging": if scenario.workload == MainBlockWorkload::Auction {
            serde_json::json!({
                "l1_transaction_count": AUCTION_300_BIDS,
                "transaction_topology": "300_e2e_bid_transaction_ids_in_one_l1_block_one_legacy_dcid",
                "acl_projection": "five_persistent_state_outputs_per_bid_marked_allowed; terminal_decryption_permissions_staged_before_commit; transient_and_user_specific_e2e_allow_events_not_modeled",
                "acl_staging_in_primary_metric": false,
            })
        } else { serde_json::Value::Null },
        "dispatch": dispatch,
        // The parameter record reported points are stored under. The workload's
        // own facts stay in this artifact and in the reported test name.
        "bench_parameters": bench_parameters,
        "terminal_handle_count": outcome.terminal_handle_count,
        "computation_count": outcome.computation_count,
        "dependence_chain_count": outcome.dependence_chain_count,
        "native_dcid_dependency_edge_count": outcome.dependence_edge_count,
        "blocks_committed": outcome.blocks_committed,
        "post_commit_worker_visible_to_terminal_outputs_ms": outcome.post_commit_worker_visible_to_terminal_outputs.as_millis(),
        "commit_start_to_terminal_outputs_upper_bound_ms": outcome.commit_start_to_terminal_outputs.as_millis(),
    }))?;
    println!(
        "One-shot reportable ERC20: scope=main_legacy_block; backend={}; scenario={}; post_commit_worker_visible_to_terminal_outputs_ms={}; commit_start_to_terminal_outputs_upper_bound_ms={}; artifact={}",
        utils::compiled_benchmark_backend(),
        scenario.name,
        outcome.post_commit_worker_visible_to_terminal_outputs.as_millis(),
        outcome.commit_start_to_terminal_outputs.as_millis(),
        artifact.display(),
    );
    Ok(())
}

const AUCTION_300_BIDS: usize = 300;
const AUCTION_300_BIDDERS: usize = 150;
const AUCTION_300_PRICE_LEVELS: usize = 39;
const AUCTION_300_WALLETS: usize = 32;

type AuctionFixture = (
    Vec<(TfheContractEvents, bool)>,
    Vec<(ListenerHandle, usize)>,
    Vec<ListenerHandle>,
);

/// Build the operation graph from `ConfidentialAuctionBidBench` in the former
/// gpu e2e fixture.  This deliberately retains the graph's state ownership:
/// bidder quota/payment/total-paid, price-level totals, and holding-wallet
/// totals are all carried by the handles consumed by the next relevant bid.
fn auction_300_events(counter: &mut u64) -> AuctionFixture {
    assert_eq!(AUCTION_300_BIDDERS * 2, AUCTION_300_BIDS);
    let caller = zero_address();
    let mut events = Vec::with_capacity(AUCTION_300_BIDS * 15 + AUCTION_300_BIDDERS * 2);
    let mut quota = vec![None; AUCTION_300_BIDDERS];
    let mut payment = vec![None; AUCTION_300_BIDDERS];
    let zero_root = next_handle(counter);
    // These are fixture ciphertexts, not computations: this exactly mirrors
    // the e2e encrypted quantity and empty-map zero boundary values.
    let mut encrypted_inputs = vec![(zero_root, 3)];
    let mut total_paid = vec![zero_root; AUCTION_300_BIDDERS];
    let mut price_total = vec![zero_root; AUCTION_300_PRICE_LEVELS];
    let mut wallet_total = vec![zero_root; AUCTION_300_WALLETS];
    let mut confirmed_paid = Vec::with_capacity(AUCTION_300_BIDS);

    for bid_index in 0..AUCTION_300_BIDS {
        let bidder = bid_index / 2;
        let price_level = bid_index % AUCTION_300_PRICE_LEVELS;
        let wallet = bidder & (AUCTION_300_WALLETS - 1);
        let quantity = next_handle(counter);
        encrypted_inputs.push((quantity, 2));
        let bidder_quota = quota[bidder].unwrap_or_else(|| {
            let handle = next_handle(counter);
            events.push((
                TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                    caller,
                    pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(2_u64)),
                    toType: to_ty(5),
                    result: handle,
                }),
                false,
            ));
            handle
        });
        let bidder_payment = payment[bidder].unwrap_or_else(|| {
            let handle = next_handle(counter);
            events.push((
                TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                    caller,
                    pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(400_000_u64)),
                    toType: to_ty(5),
                    result: handle,
                }),
                false,
            ));
            handle
        });
        macro_rules! op {
            ($event:expr, $allowed:expr) => {{
                events.push(($event, $allowed));
            }};
        }
        let capped = next_handle(counter);
        op!(
            TfheContractEvents::FheMin(TfheContract::FheMin {
                caller,
                lhs: quantity,
                rhs: bidder_quota,
                scalarByte: scalar_flag(false),
                result: capped
            }),
            false
        );
        let paid = next_handle(counter);
        op!(
            TfheContractEvents::FheMul(TfheContract::FheMul {
                caller,
                lhs: capped,
                rhs: as_handle(10_000_u64 + price_level as u64 * 5_000),
                scalarByte: scalar_flag(true),
                result: paid
            }),
            false
        );
        let can_pay = next_handle(counter);
        op!(
            TfheContractEvents::FheLe(TfheContract::FheLe {
                caller,
                lhs: paid,
                rhs: bidder_payment,
                scalarByte: scalar_flag(false),
                result: can_pay
            }),
            false
        );
        let zero_payment = next_handle(counter);
        op!(
            TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                caller,
                pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(0_u64)),
                toType: to_ty(5),
                result: zero_payment
            }),
            false
        );
        let transferred = next_handle(counter);
        op!(
            TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                caller,
                control: can_pay,
                ifTrue: paid,
                ifFalse: zero_payment,
                result: transferred
            }),
            false
        );
        let next_payment = next_handle(counter);
        op!(
            TfheContractEvents::FheSub(TfheContract::FheSub {
                caller,
                lhs: bidder_payment,
                rhs: transferred,
                scalarByte: scalar_flag(false),
                result: next_payment
            }),
            true
        );
        payment[bidder] = Some(next_payment);
        let next_wallet = next_handle(counter);
        op!(
            TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller,
                lhs: wallet_total[wallet],
                rhs: transferred,
                scalarByte: scalar_flag(false),
                result: next_wallet
            }),
            true
        );
        wallet_total[wallet] = next_wallet;
        let confirmed = next_handle(counter);
        op!(
            TfheContractEvents::FheEq(TfheContract::FheEq {
                caller,
                lhs: transferred,
                rhs: paid,
                scalarByte: scalar_flag(false),
                result: confirmed
            }),
            false
        );
        let next_paid = next_handle(counter);
        op!(
            TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller,
                lhs: total_paid[bidder],
                rhs: transferred,
                scalarByte: scalar_flag(false),
                result: next_paid
            }),
            true
        );
        total_paid[bidder] = next_paid;
        let zero_quantity = next_handle(counter);
        op!(
            TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                caller,
                pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(0_u64)),
                toType: to_ty(5),
                result: zero_quantity
            }),
            false
        );
        let confirmed_quantity = next_handle(counter);
        op!(
            TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                caller,
                control: confirmed,
                ifTrue: capped,
                ifFalse: zero_quantity,
                result: confirmed_quantity
            }),
            false
        );
        let zero_paid = next_handle(counter);
        op!(
            TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                caller,
                pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(0_u64)),
                toType: to_ty(5),
                result: zero_paid
            }),
            false
        );
        let confirmed_paid_handle = next_handle(counter);
        op!(
            TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                caller,
                control: confirmed,
                ifTrue: paid,
                ifFalse: zero_paid,
                result: confirmed_paid_handle
            }),
            false
        );
        confirmed_paid.push(confirmed_paid_handle);
        let next_quota = next_handle(counter);
        op!(
            TfheContractEvents::FheSub(TfheContract::FheSub {
                caller,
                lhs: bidder_quota,
                rhs: confirmed_quantity,
                scalarByte: scalar_flag(false),
                result: next_quota
            }),
            true
        );
        quota[bidder] = Some(next_quota);
        let next_price = next_handle(counter);
        op!(
            TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller,
                lhs: price_total[price_level],
                rhs: confirmed_quantity,
                scalarByte: scalar_flag(false),
                result: next_price
            }),
            true
        );
        price_total[price_level] = next_price;
    }
    assert_eq!(
        events.len(),
        AUCTION_300_BIDS * 15 + AUCTION_300_BIDDERS * 2
    );
    let mut terminals = quota
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .expect("quota initialized");
    terminals.extend(
        payment
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .expect("payment initialized"),
    );
    terminals.extend(total_paid);
    terminals.extend(price_total);
    terminals.extend(wallet_total);
    terminals.extend(confirmed_paid);
    assert_eq!(
        terminals.len(),
        3 * AUCTION_300_BIDDERS + AUCTION_300_PRICE_LEVELS + AUCTION_300_WALLETS + AUCTION_300_BIDS
    );
    (events, encrypted_inputs, terminals)
}

fn auction_300_legacy_terminals(
    handles: Vec<host_listener::database::tfhe_event_propagate::Handle>,
    transaction_ids: &[host_listener::database::tfhe_event_propagate::Handle],
) -> Vec<LegacyTerminal> {
    assert_eq!(transaction_ids.len(), AUCTION_300_BIDS);
    let expected =
        3 * AUCTION_300_BIDDERS + AUCTION_300_PRICE_LEVELS + AUCTION_300_WALLETS + AUCTION_300_BIDS;
    assert_eq!(handles.len(), expected);
    let mut handles = handles.into_iter();
    let mut terminals = Vec::with_capacity(expected);
    // quota, payment, and total-paid each end at the bidder's second bid.
    for _ in 0..3 {
        for bidder in 0..AUCTION_300_BIDDERS {
            terminals.push(LegacyTerminal {
                handle: handles.next().expect("quota/payment/total terminal"),
                transaction_id: transaction_ids[bidder * 2 + 1],
            });
        }
    }
    // Each price/wallet aggregate ends at its final bid in the deterministic
    // e2e-compatible assignment.
    for price_level in 0..AUCTION_300_PRICE_LEVELS {
        let last_bid = (0..AUCTION_300_BIDS)
            .rev()
            .find(|bid| bid % AUCTION_300_PRICE_LEVELS == price_level)
            .expect("every price level is used");
        terminals.push(LegacyTerminal {
            handle: handles.next().expect("price aggregate terminal"),
            transaction_id: transaction_ids[last_bid],
        });
    }
    for wallet in 0..AUCTION_300_WALLETS {
        let last_bid = (0..AUCTION_300_BIDS)
            .rev()
            .find(|bid| (bid / 2) & (AUCTION_300_WALLETS - 1) == wallet)
            .expect("every wallet is used");
        terminals.push(LegacyTerminal {
            handle: handles.next().expect("wallet aggregate terminal"),
            transaction_id: transaction_ids[last_bid],
        });
    }
    for transaction_id in transaction_ids {
        terminals.push(LegacyTerminal {
            handle: handles.next().expect("confirmed-paid terminal"),
            transaction_id: *transaction_id,
        });
    }
    assert!(handles.next().is_none());
    terminals
}

/// Native-main one-shot executor for the confidential-auction graph.  The
/// timer starts after the transaction commit returns and ends at all exact
/// final state and confirmation ciphertext rows, identical to the ERC20
/// harness boundary.
async fn run_main_block_auction_300(
    listener_db: &ListenerDatabase,
    db_url: &str,
) -> Result<MainBlockOneShotOutcome, Box<dyn std::error::Error>> {
    let mut observer = ExactLegacyTerminalObserver::connect(db_url).await?;
    let mut counter = random_handle();
    // The e2e workload submits 300 EVM transactions into one manually mined
    // L1 block. Preserve those IDs while keeping its connected same-block
    // graph in one legacy scheduling chain.
    let dependence_chain_id = next_handle(&mut counter);
    let transaction_ids = (0..AUCTION_300_BIDS)
        .map(|_| next_handle(&mut counter))
        .collect::<Vec<_>>();
    let (events, encrypted_inputs, terminal_handles) = auction_300_events(&mut counter);
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("live legacy transaction");
    let inputs = auction_encrypted_u64_inputs_from_xof()?;
    for (handle, input_index) in encrypted_inputs {
        seed_legacy_input_ciphertext(&mut tx, &handle, &inputs[input_index]).await?;
    }
    let mut events = events.into_iter();
    for (bid_index, transaction_id) in transaction_ids.iter().copied().enumerate() {
        // A bidder's first bid includes two lazy state initializations, then
        // every bid contributes the contract's 15 FHE events.
        let event_count = if bid_index % 2 == 0 { 17 } else { 15 };
        for _ in 0..event_count {
            let (event, allowed) = events
                .next()
                .expect("auction event count matches the deterministic bid graph");
            insert_legacy_one_shot_event_in_dependence_chain(
                listener_db,
                &mut tx,
                transaction_id,
                dependence_chain_id,
                event,
                allowed,
            )
            .await?;
        }
    }
    assert!(
        events.next().is_none(),
        "all auction events are assigned to bids"
    );
    upsert_legacy_dependence_chain(&mut tx, &dependence_chain_id, 0, &[]).await?;
    let terminals = auction_300_legacy_terminals(terminal_handles, &transaction_ids);
    // All variants mark the five persistent per-bid state outputs allowed and
    // stage terminal decryption permissions before commit. The primary metric
    // excludes this common ACL projection; transient/user-specific e2e allow
    // events are outside this worker-only fixture.
    for terminal in &terminals {
        allow_handle(listener_db, &mut tx, &terminal.handle).await?;
    }
    let commit_started = Instant::now();
    tx.commit().await?;
    let worker_visible_at = Instant::now();
    let exact = terminals
        .iter()
        .map(|terminal| (terminal.handle.to_vec(), terminal.transaction_id.to_vec()))
        .collect::<Vec<_>>();
    let completed_at = observer
        .wait_until_completed(&exact, utils::benchmark_wait_timeout()?, false)
        .await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(db_url)
        .await?;
    let computation_count: i64 =
        sqlx::query_scalar("SELECT count(1) FROM computations WHERE dependence_chain_id = $1")
            .bind(dependence_chain_id.to_vec())
            .fetch_one(&pool)
            .await?;
    let expected = (AUCTION_300_BIDS * 15 + AUCTION_300_BIDDERS * 2) as i64;
    if computation_count != expected {
        return Err(format!(
            "auction_300 computation count {computation_count}, expected {expected}"
        )
        .into());
    }
    Ok(MainBlockOneShotOutcome {
        post_commit_worker_visible_to_terminal_outputs: completed_at
            .duration_since(worker_visible_at),
        commit_start_to_terminal_outputs: completed_at.duration_since(commit_started),
        terminal_handle_count: terminals.len(),
        computation_count,
        transaction_id_count: transaction_ids.len() as i64,
        dependence_chain_count: 1,
        dependence_edge_count: 0,
        blocks_committed: 1,
        unpaced_ingestion: false,
    })
}

fn auction_encrypted_u64_inputs_from_xof() -> Result<[Vec<u8>; 4], Box<dyn std::error::Error>> {
    let keyset_bytes = std::fs::read("../fhevm-keys/xof-keyset")?;
    let keyset: CompressedXofKeySet = safe_deserialize_key(&keyset_bytes)?;
    let (compact_public_key, server_key) = keyset.decompress()?.into_raw_parts();
    tfhe::set_server_key(server_key);
    let mut builder = tfhe::CompactCiphertextList::builder(&compact_public_key);
    for value in [400_000_u64, 2, 1, 0] {
        builder.push(value);
    }
    let expanded: CompactCiphertextListExpander = builder.build().expand()?;
    let encrypt = |index| -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let ciphertext: tfhe::FheUint64 = expanded
            .get(index)?
            .ok_or_else(|| "missing auction ciphertext".to_owned())?;
        SupportedFheCiphertexts::FheUint64(ciphertext)
            .compress()
            .map_err(|err| format!("compress auction benchmark input: {err:?}").into())
    };
    Ok([encrypt(0)?, encrypt(1)?, encrypt(2)?, encrypt(3)?])
}

/// Builds one raw host-chain log carrying an encoded contract event, shaped
/// exactly as the production poller hands it to `ingest_block_logs`.
fn raw_event_log(
    address: alloy::primitives::Address,
    transaction_hash: host_listener::database::tfhe_event_propagate::Handle,
    log_index: u64,
    data: alloy::primitives::LogData,
) -> alloy::rpc::types::Log {
    alloy::rpc::types::Log {
        inner: alloy::primitives::Log { address, data },
        transaction_hash: Some(transaction_hash),
        log_index: Some(log_index),
        ..Default::default()
    }
}

/// Unpaced L1 traffic staged through the REAL ingestion path: every block is
/// a `BlockLogs` of raw TFHE + ACL logs handed to `ingest_block_logs`, so
/// dependence chains come from production formation
/// (`grouping_to_chains_no_fork`, cross-block extension, gating) instead of
/// hand-written `dependence_chain` rows. The fixture can no longer drift
/// from formation semantics.
///
/// Two shapes share this runner:
/// - linear (`join_braid = false`): ten disjoint balance chains, advanced
///   lag-2; production formation extends each into ONE cross-block DCID.
/// - braid (`join_braid = true`): every transfer moves value between two
///   accounts of its parity deck, so each transfer joins two account chains
///   and must form its own gated chain; no linear extension is possible.
///
/// Carried balances are allowed on every link (production ACL rule: a
/// handle consumed by another transaction is allow()ed by consumption time).
async fn run_main_block_unpaced_traffic(
    listener_db: &ListenerDatabase,
    db_url: &str,
    scenario: MainBlockOneShotScenario,
) -> Result<MainBlockOneShotOutcome, Box<dyn std::error::Error>> {
    use alloy::sol_types::SolEvent;
    use host_listener::contracts::AclContract;
    use host_listener::database::ingest::{ingest_block_logs, BlockLogs, IngestOptions};
    type LHandle = host_listener::database::tfhe_event_propagate::Handle;

    let MainBlockOneShotStaging::SequentialUnpacedTraffic {
        l1_blocks,
        transfers_per_block,
        chain_count,
        dependence_lag_blocks,
        join_braid,
    } = scenario.staging
    else {
        unreachable!("unpaced traffic runner requires an unpaced traffic scenario");
    };
    assert_eq!(l1_blocks * transfers_per_block, scenario.transfers);
    if join_braid {
        // Each braid transfer touches two accounts of its parity deck.
        assert_eq!(transfers_per_block * dependence_lag_blocks * 2, chain_count);
    } else {
        assert_eq!(transfers_per_block * dependence_lag_blocks, chain_count);
    }

    // Both observer setup and LISTEN acquisition precede staging and the
    // first commit, so neither is included in the primary interval.
    let mut terminal_observer = ExactLegacyTerminalObserver::connect(db_url).await?;
    let inputs = encrypted_u64_inputs_from_xof()?;
    let caller = zero_address();
    let mut counter = random_handle();
    let chain_id = fhevm_engine_common::chain_id::ChainId::try_from(42_u64)?;
    let tfhe_address = alloy::primitives::Address::repeat_byte(0x7E);
    let acl_address = alloy::primitives::Address::repeat_byte(0xAC);
    let ingest_options = || IngestOptions {
        dependence_by_connexity: false,
        dependence_cross_block: true,
        // The slow lane is a fairness mechanism for shared deployments; a
        // long-lived benchmark chain must not be demoted mid-run.
        dependent_ops_max_per_chain: 0,
        is_protocol_config_listener: false,
    };
    let mut ingest_db = listener_db.clone();
    // Real wall-clock block timestamps: the worker's processed-chain cleanup
    // reaps chains whose last_updated_at is older than its threshold, so a
    // historic epoch here deletes chains mid-run and skews scheduling.
    let staging_epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    // Account state: the current pair of balance tails and the eth
    // transaction that produced them. Linear mode keeps (from, to) tails per
    // chain; braid mode keeps one tail per account.
    let mut linear_tails: Vec<Option<(LHandle, LHandle)>> = vec![None; chain_count];
    let mut braid_tails: Vec<Option<(LHandle, LHandle)>> = vec![None; chain_count];
    let mut expected_linear_chain_heads: Vec<Vec<u8>> = Vec::new();
    let mut transaction_ids: Vec<LHandle> = Vec::with_capacity(scenario.transfers);
    let mut last_touch: Vec<Option<(LHandle, LHandle)>> = vec![None; chain_count];
    let mut commit_started = None;
    let mut worker_visible_at = None;
    let mut parent_block_hash =
        host_listener::database::tfhe_event_propagate::Handle::from([0u8; 32]);

    for block_index in 0..l1_blocks {
        // Pre-seed this block's encrypted inputs OUTSIDE the ingested block
        // (inputs come from the zkproof path in production, not from events).
        let mut seed_tx = listener_db
            .new_transaction()
            .await?
            .expect("new_transaction returns Some on a live legacy stack");
        let mut logs: Vec<alloy::rpc::types::Log> = Vec::with_capacity(transfers_per_block * 8);
        let mut log_index: u64 = 0;
        let mut li = || {
            log_index += 1;
            log_index
        };
        for index_in_block in 0..transfers_per_block {
            let transfer_txh = next_handle(&mut counter);
            transaction_ids.push(transfer_txh);
            let (from, to, from_slot, to_slot);
            if join_braid {
                // Parity decks: even blocks braid accounts 0..deck-1, odd
                // blocks deck..2*deck-1. Rotation alternates neighboring
                // pairs so consecutive deck-blocks join different chains.
                let deck = chain_count / 2;
                let deck_base = (block_index % 2) * deck;
                let rotation = (block_index / 2) % 2;
                let a = deck_base + (2 * index_in_block + rotation) % deck;
                let b = deck_base + (2 * index_in_block + 1 + rotation) % deck;
                from_slot = a;
                to_slot = b;
                let mut tail_of = |slot: usize, seed_index: usize| {
                    if let Some((tail, _txh)) = braid_tails[slot] {
                        Ok::<_, sqlx::Error>((tail, false, seed_index))
                    } else {
                        Ok((next_handle(&mut counter), true, seed_index))
                    }
                };
                let (f, seed_f, _) = tail_of(a, 0)?;
                let (t, seed_t, _) = tail_of(b, 1)?;
                if seed_f {
                    seed_legacy_input_ciphertext(&mut seed_tx, &f, &inputs[0]).await?;
                }
                if seed_t {
                    seed_legacy_input_ciphertext(&mut seed_tx, &t, &inputs[1]).await?;
                }
                from = f;
                to = t;
            } else {
                let slot = unpaced_traffic_chain_index(
                    block_index,
                    index_in_block,
                    transfers_per_block,
                    dependence_lag_blocks,
                );
                from_slot = slot;
                to_slot = slot;
                if let Some((f, t)) = linear_tails[slot] {
                    from = f;
                    to = t;
                } else {
                    from = next_handle(&mut counter);
                    to = next_handle(&mut counter);
                    seed_legacy_input_ciphertext(&mut seed_tx, &from, &inputs[0]).await?;
                    seed_legacy_input_ciphertext(&mut seed_tx, &to, &inputs[1]).await?;
                    // The chain head transaction hash becomes the DCID under
                    // linear cross-block extension.
                    expected_linear_chain_heads.push(transfer_txh.to_vec());
                }
            }
            let amount = next_handle(&mut counter);
            seed_legacy_input_ciphertext(&mut seed_tx, &amount, &inputs[2]).await?;

            let has_funds = next_handle(&mut counter);
            logs.push(raw_event_log(
                tfhe_address,
                transfer_txh,
                li(),
                TfheContract::FheGe {
                    caller,
                    lhs: from,
                    rhs: amount,
                    scalarByte: scalar_flag(false),
                    result: has_funds,
                }
                .encode_log_data(),
            ));
            let zero = next_handle(&mut counter);
            logs.push(raw_event_log(
                tfhe_address,
                transfer_txh,
                li(),
                TfheContract::TrivialEncrypt {
                    caller,
                    pt: as_scalar_uint(&bigdecimal::num_bigint::BigInt::from(0_u64)),
                    toType: to_ty(5),
                    result: zero,
                }
                .encode_log_data(),
            ));
            let selected = next_handle(&mut counter);
            logs.push(raw_event_log(
                tfhe_address,
                transfer_txh,
                li(),
                TfheContract::FheIfThenElse {
                    caller,
                    control: has_funds,
                    ifTrue: amount,
                    ifFalse: zero,
                    result: selected,
                }
                .encode_log_data(),
            ));
            let new_to = next_handle(&mut counter);
            logs.push(raw_event_log(
                tfhe_address,
                transfer_txh,
                li(),
                TfheContract::FheAdd {
                    caller,
                    lhs: to,
                    rhs: selected,
                    scalarByte: scalar_flag(false),
                    result: new_to,
                }
                .encode_log_data(),
            ));
            let new_from = next_handle(&mut counter);
            logs.push(raw_event_log(
                tfhe_address,
                transfer_txh,
                li(),
                TfheContract::FheSub {
                    caller,
                    lhs: from,
                    rhs: selected,
                    scalarByte: scalar_flag(false),
                    result: new_from,
                }
                .encode_log_data(),
            ));
            // Production ACL rule: both carried balances are consumed by a
            // later transaction, so both hold a persistent allow() by
            // consumption time. The allow events ride in the same block.
            for handle in [new_from, new_to] {
                logs.push(raw_event_log(
                    acl_address,
                    transfer_txh,
                    li(),
                    AclContract::Allowed {
                        caller,
                        account: caller,
                        handle,
                    }
                    .encode_log_data(),
                ));
            }
            if join_braid {
                braid_tails[from_slot] = Some((new_from, transfer_txh));
                braid_tails[to_slot] = Some((new_to, transfer_txh));
                last_touch[from_slot] = Some((new_from, transfer_txh));
                last_touch[to_slot] = Some((new_to, transfer_txh));
            } else {
                linear_tails[from_slot] = Some((new_from, new_to));
                last_touch[from_slot] = Some((new_from, transfer_txh));
                // The paired terminal for a linear chain is new_to of the
                // same closing transfer; reuse to_slot's storage.
                braid_tails[to_slot] = Some((new_to, transfer_txh));
            }
        }
        seed_tx.commit().await?;

        let block_hash = next_handle(&mut counter);
        let block_logs = BlockLogs {
            logs,
            summary: host_listener::cmd::block_history::BlockSummary {
                number: (block_index + 1) as u64,
                hash: block_hash,
                parent_hash: parent_block_hash,
                timestamp: staging_epoch + block_index as u64,
            },
            catchup: false,
            finalized: true,
        };
        parent_block_hash = block_hash;
        if block_index == 0 {
            commit_started = Some(Instant::now());
        }
        ingest_block_logs(
            chain_id,
            &mut ingest_db,
            &block_logs,
            &Some(acl_address),
            &Some(tfhe_address),
            &None,
            &None,
            &None,
            ingest_options(),
        )
        .await?;
        if block_index == 0 {
            worker_visible_at = Some(Instant::now());
        }
        // Deliberately no wait, notification drain, or pacing between L1
        // commits: staging and worker compute overlap throughout the stream.
    }

    let mut terminals = Vec::with_capacity(scenario.terminal_handle_count());
    if join_braid {
        for last_touch in last_touch.iter().take(chain_count) {
            let (handle, txh) = last_touch.expect("every account was touched");
            terminals.push(LegacyTerminal {
                handle,
                transaction_id: txh,
            });
        }
    } else {
        for slot in 0..chain_count {
            let (new_from, txh) = last_touch[slot].expect("every chain closed");
            let (new_to, _txh) = braid_tails[slot].expect("every chain closed");
            terminals.push(LegacyTerminal {
                handle: new_from,
                transaction_id: txh,
            });
            terminals.push(LegacyTerminal {
                handle: new_to,
                transaction_id: txh,
            });
        }
    }
    assert_eq!(terminals.len(), scenario.terminal_handle_count());

    let exact_terminals = terminals
        .iter()
        .map(|terminal| (terminal.handle.to_vec(), terminal.transaction_id.to_vec()))
        .collect::<Vec<_>>();
    let completed_at = terminal_observer
        .wait_until_completed(&exact_terminals, utils::benchmark_wait_timeout()?, false)
        .await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(db_url)
        .await?;
    let transaction_ids_bytes = transaction_ids
        .iter()
        .map(|id| id.to_vec())
        .collect::<Vec<_>>();
    let computation_count: i64 = sqlx::query_scalar(
        "SELECT count(1) FROM computations WHERE transaction_id = ANY($1::bytea[])",
    )
    .bind(&transaction_ids_bytes)
    .fetch_one(&pool)
    .await?;
    let dependence_chain_count: i64 = sqlx::query_scalar(
        "SELECT count(DISTINCT dependence_chain_id) FROM computations \
         WHERE transaction_id = ANY($1::bytea[])",
    )
    .bind(&transaction_ids_bytes)
    .fetch_one(&pool)
    .await?;
    let dependence_edge_count: i64 = sqlx::query_scalar(
        "SELECT COALESCE(sum(cardinality(dependents)), 0)::bigint \
         FROM dependence_chain WHERE dependence_chain_id IN ( \
             SELECT DISTINCT dependence_chain_id FROM computations \
             WHERE transaction_id = ANY($1::bytea[]))",
    )
    .bind(&transaction_ids_bytes)
    .fetch_one(&pool)
    .await?;
    let expected_computations = (scenario.transfers * 5) as i64;
    let expected_chains: i64 = if join_braid {
        // Every braid transfer joins two account chains and forms its own
        // gated chain; deck-initial transfers have no parents and also form
        // their own chains.
        scenario.transfers as i64
    } else {
        chain_count as i64
    };
    if computation_count != expected_computations || dependence_chain_count != expected_chains {
        return Err(format!(
            "main_block_baseline traffic topology mismatch: computations={computation_count} (expected {expected_computations}), formed_chains={dependence_chain_count} (expected {expected_chains})",
        )
        .into());
    }
    if !join_braid {
        let linear_chains: i64 = sqlx::query_scalar(
            "SELECT count(1) FROM dependence_chain WHERE dependence_chain_id = ANY($1::bytea[])",
        )
        .bind(&expected_linear_chain_heads)
        .fetch_one(&pool)
        .await?;
        if linear_chains != chain_count as i64 {
            let actual_ids: Vec<Vec<u8>> = sqlx::query_scalar(
                "SELECT DISTINCT dependence_chain_id FROM computations \
                 WHERE transaction_id = ANY($1::bytea[])",
            )
            .bind(&transaction_ids_bytes)
            .fetch_all(&pool)
            .await?;
            let chain_rows: Vec<Vec<u8>> = sqlx::query_scalar(
                "SELECT dependence_chain_id FROM dependence_chain \
                 WHERE dependence_chain_id = ANY($1::bytea[])",
            )
            .bind(&actual_ids)
            .fetch_all(&pool)
            .await?;
            return Err(format!(
                "linear traffic chains must be rooted at their chain-head transactions: found {linear_chains}, expected {chain_count}; \
                 expected heads: {:?}; distinct computation chain ids: {:?}; of which with chain rows: {:?}",
                expected_linear_chain_heads.iter().map(hex::encode).collect::<Vec<_>>(),
                actual_ids.iter().map(hex::encode).collect::<Vec<_>>(),
                chain_rows.iter().map(hex::encode).collect::<Vec<_>>(),
            )
            .into());
        }
    }
    Ok(MainBlockOneShotOutcome {
        post_commit_worker_visible_to_terminal_outputs: completed_at
            .duration_since(worker_visible_at.expect("first block commit must make work visible")),
        commit_start_to_terminal_outputs: completed_at
            .duration_since(commit_started.expect("first block commit timing must be armed")),
        terminal_handle_count: terminals.len(),
        computation_count,
        transaction_id_count: scenario.transfers as i64,
        dependence_chain_count,
        dependence_edge_count,
        blocks_committed: l1_blocks,
        unpaced_ingestion: true,
    })
}

async fn insert_legacy_one_shot_event_in_dependence_chain(
    listener_db: &ListenerDatabase,
    tx: &mut host_listener::database::tfhe_event_propagate::Transaction<'_>,
    transaction_id: host_listener::database::tfhe_event_propagate::Handle,
    dependence_chain_id: host_listener::database::tfhe_event_propagate::Handle,
    event: TfheContractEvents,
    is_allowed: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    utils::insert_tfhe_event_with_dependence_chain(
        listener_db,
        tx,
        log_with_tx(transaction_id, tfhe_event(event)),
        transaction_id,
        dependence_chain_id,
        is_allowed,
    )
    .await?;
    Ok(())
}

async fn seed_legacy_input_ciphertext(
    tx: &mut host_listener::database::tfhe_event_propagate::Transaction<'_>,
    handle: &host_listener::database::tfhe_event_propagate::Handle,
    ciphertext: &[u8],
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO ciphertexts(handle, ciphertext, ciphertext_version, ciphertext_type, is_input) \
         VALUES ($1, $2, $3, $4, TRUE)",
    )
    .bind(handle.to_vec())
    .bind(ciphertext)
    .bind(current_ciphertext_version())
    .bind(5_i16)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

fn encrypted_u64_inputs_from_xof() -> Result<[Vec<u8>; 3], Box<dyn std::error::Error>> {
    let keyset_bytes = std::fs::read("../fhevm-keys/xof-keyset")?;
    let keyset: CompressedXofKeySet = safe_deserialize_key(&keyset_bytes)?;
    let (compact_public_key, cpu_server_key) = keyset.decompress()?.into_raw_parts();
    let inputs = tfhe::with_server_key_as_context(cpu_server_key, || {
        let mut builder = tfhe::CompactCiphertextList::builder(&compact_public_key);
        for value in [100_u64, 20_u64, 10_u64] {
            builder.push(value);
        }
        let expanded: CompactCiphertextListExpander = builder.build().expand()?;
        (0..3)
            .map(|index| -> Result<Vec<u8>, Box<dyn std::error::Error>> {
                let ciphertext: tfhe::FheUint64 = expanded
                    .get(index)?
                    .ok_or_else(|| format!("missing encrypted benchmark input at index {index}"))?;
                SupportedFheCiphertexts::FheUint64(ciphertext)
                    .compress()
                    .map_err(|error| format!("compress benchmark input: {error:?}").into())
            })
            .collect::<Result<Vec<_>, _>>()
    })?;
    inputs.try_into().map_err(|inputs: Vec<Vec<u8>>| {
        format!("expected 3 encrypted inputs, got {}", inputs.len()).into()
    })
}

async fn schedule_erc20(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    use_cmux: bool,
    dependent: bool,
    bench_id: &str,
    display_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = setup_test_app().await?;
    let listener_db = listener_event_db(&app).await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(app.db_url())
        .await?;
    let mut handle_counter = random_handle();
    let _terminal_legacy_rows = submit_erc20_workload(
        &listener_db,
        sample_count(num_tx),
        use_cmux,
        dependent,
        false,
        &mut handle_counter,
        None,
    )
    .await?;

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
            })
            .await;
            println!(
                "Execution time: {} -- {}",
                now.elapsed().unwrap().as_millis(),
                TIMING.load(std::sync::atomic::Ordering::SeqCst) / 1000
            );
            std::time::Duration::from_micros(
                TIMING.swap(0, std::sync::atomic::Ordering::SeqCst) * iters.max(1),
            )
        });

    let write_params = write_atomic_u64_bench_params(&pool, bench_id, display_name).await;
    let shutdown = app.shutdown().await;
    write_params?;
    shutdown?;
    Ok(())
}

async fn submit_erc20_workload(
    listener_db: &host_listener::database::tfhe_event_propagate::Database,
    num_samples: usize,
    use_cmux: bool,
    dependent: bool,
    main_block_exact: bool,
    handle_counter: &mut u64,
    visibility_started: Option<&mut Instant>,
) -> Result<Vec<LegacyTerminal>, Box<dyn std::error::Error>> {
    let caller = zero_address();
    let shared_tx_id = next_handle(handle_counter);

    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    let mut prev_from: Option<host_listener::database::tfhe_event_propagate::Handle> = None;
    let mut prev_to: Option<host_listener::database::tfhe_event_propagate::Handle> = None;
    let mut terminal_legacy_rows = Vec::new();
    let mut dependence_chain_ids = Vec::new();

    for i in 0..num_samples {
        let tx_id = if dependent {
            shared_tx_id
        } else {
            next_handle(handle_counter)
        };
        if main_block_exact && (!dependent || i == 0) {
            dependence_chain_ids.push(tx_id);
        }
        let from_balance = if let Some(h) = prev_from {
            h
        } else {
            let h = next_handle(handle_counter);
            utils::insert_tfhe_event(
                listener_db,
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
            let h = next_handle(handle_counter);
            utils::insert_tfhe_event(
                listener_db,
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
        let transfer_amount = next_handle(handle_counter);
        utils::insert_tfhe_event(
            listener_db,
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

        let has_funds = next_handle(handle_counter);
        utils::insert_tfhe_event(
            listener_db,
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
            let to_target = next_handle(handle_counter);
            utils::insert_tfhe_event(
                listener_db,
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
            new_to = next_handle(handle_counter);
            utils::insert_tfhe_event(
                listener_db,
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

            let from_target = next_handle(handle_counter);
            utils::insert_tfhe_event(
                listener_db,
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
            new_from = next_handle(handle_counter);
            utils::insert_tfhe_event(
                listener_db,
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
            let funds_u64 = next_handle(handle_counter);
            utils::insert_tfhe_event(
                listener_db,
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
            let selected_amount = next_handle(handle_counter);
            utils::insert_tfhe_event(
                listener_db,
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
            new_to = next_handle(handle_counter);
            utils::insert_tfhe_event(
                listener_db,
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
            new_from = next_handle(handle_counter);
            utils::insert_tfhe_event(
                listener_db,
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
            allow_handle(listener_db, &mut tx, &new_to).await?;
            allow_handle(listener_db, &mut tx, &new_from).await?;
        }
        prev_from = Some(new_from);
        prev_to = Some(new_to);
        if main_block_exact {
            terminal_legacy_rows.push(LegacyTerminal {
                handle: new_to,
                transaction_id: tx_id,
            });
            terminal_legacy_rows.push(LegacyTerminal {
                handle: new_from,
                transaction_id: tx_id,
            });
        }
    }
    if main_block_exact {
        for (index, dependence_chain_id) in dependence_chain_ids.iter().enumerate() {
            let dependents = dependence_chain_ids
                .get(index + 1)
                .map(std::slice::from_ref)
                .unwrap_or(&[]);
            upsert_legacy_dependence_chain(
                &mut tx,
                dependence_chain_id,
                i32::from(index != 0),
                dependents,
            )
            .await?;
        }
    }
    if let Some(visibility_started) = visibility_started {
        *visibility_started = Instant::now();
    }
    tx.commit().await?;
    Ok(terminal_legacy_rows)
}

async fn schedule_erc20_whitepaper(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(bencher, num_tx, true, false, &bench_id, "erc20-transfer").await
}

async fn schedule_erc20_no_cmux(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(bencher, num_tx, false, false, &bench_id, "erc20-transfer").await
}

async fn schedule_dependent_erc20_whitepaper(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(bencher, num_tx, true, true, &bench_id, "erc20-transfer").await
}

async fn schedule_dependent_erc20_no_cmux(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(bencher, num_tx, false, true, &bench_id, "erc20-transfer").await
}

/// Execute the fresh-submission portion of the main-block Criterion closure.
struct MainBlockIterationContext<'a> {
    listener_db: &'a ListenerDatabase,
    num_tx: usize,
    handle_counter: &'a tokio::sync::Mutex<u64>,
    submission_audit: &'a Mutex<MainBlockSubmissionAudit>,
    progress_pool: &'a sqlx::PgPool,
    db_url: &'a str,
    trace: bool,
}

async fn execute_main_block_requested_iterations(
    requested_iters: u64,
    context: MainBlockIterationContext<'_>,
) -> Result<std::time::Duration, String> {
    let MainBlockIterationContext {
        listener_db,
        num_tx,
        handle_counter,
        submission_audit,
        progress_pool,
        db_url,
        trace,
    } = context;
    if trace {
        eprintln!("MAIN_BLOCK_TRACE closure_enter requested_iters={requested_iters}");
    }
    submission_audit
        .lock()
        .map_err(|_| "main_block submission audit mutex poisoned".to_owned())?
        .record_requested_call(requested_iters);
    let mut elapsed = std::time::Duration::ZERO;
    let mut submitted_iterations = 0_u64;
    for iteration_index in 0..requested_iters {
        if trace {
            eprintln!(
                "MAIN_BLOCK_TRACE iteration_prepare index={} requested_iters={requested_iters}",
                iteration_index + 1
            );
        }
        let expected_transactions = sample_count(num_tx);
        let mut visibility_started = Instant::now();
        let terminals = {
            let mut counter = handle_counter.lock().await;
            submit_erc20_workload(
                listener_db,
                expected_transactions,
                false,
                false,
                true,
                &mut counter,
                Some(&mut visibility_started),
            )
            .await
            .map_err(|error| format!("submit main_block_baseline iteration: {error}"))?
        };
        if trace {
            eprintln!(
                "MAIN_BLOCK_TRACE iteration_committed index={} terminals={}",
                iteration_index + 1,
                terminals.len()
            );
        }
        submission_audit
            .lock()
            .map_err(|_| "main_block submission audit mutex poisoned".to_owned())?
            .record(&terminals, expected_transactions)?;
        submitted_iterations += 1;
        let completed_at = wait_until_legacy_terminals_computed(db_url.to_owned(), &terminals)
            .await
            .map_err(|error| format!("wait exact main_block_baseline terminals: {error}"))?;
        if trace {
            eprintln!(
                "MAIN_BLOCK_TRACE iteration_exact_wait_complete index={}",
                iteration_index + 1
            );
        }
        elapsed += completed_at.duration_since(visibility_started);
        let computation_count: i64 =
            sqlx::query_scalar("SELECT count(1) FROM computations WHERE host_chain_id = 42")
                .fetch_one(progress_pool)
                .await
                .map_err(|error| format!("read main_block_baseline database progress: {error}"))?;
        submission_audit
            .lock()
            .map_err(|_| "main_block submission audit mutex poisoned".to_owned())?
            .record_database_progress(computation_count)?;
        if trace {
            eprintln!(
                "MAIN_BLOCK_TRACE iteration_db_progress index={} legacy_computations={computation_count}",
                iteration_index + 1
            );
        }
    }
    if submitted_iterations != requested_iters {
        return Err(format!(
            "main_block_baseline submitted {submitted_iterations} iterations for request {requested_iters}"
        ));
    }
    if trace {
        eprintln!(
            "MAIN_BLOCK_TRACE closure_return requested_iters={requested_iters} submitted_iterations={submitted_iterations} elapsed_ms={}",
            elapsed.as_millis()
        );
    }
    Ok(elapsed)
}

fn schedule_main_block_erc20(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Criterion's FuturesExecutor is not a Tokio reactor.  Keep the
    // benchmark callback synchronous and drive every database/timer future
    // through this owned Tokio runtime instead.
    let runtime = Arc::new(Runtime::new()?);
    let mut app = runtime.block_on(setup_test_app())?;
    let listener_db = runtime.block_on(listener_event_db(&app))?;
    let pool = runtime.block_on(
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(app.db_url()),
    )?;
    let handle_counter = Arc::new(tokio::sync::Mutex::new(random_handle()));
    let submission_audit = Arc::new(Mutex::new(MainBlockSubmissionAudit::default()));
    let db_url = app.db_url().to_owned();
    let smoke_config = (std::env::var("FHEVM_BENCH_RUN_MODE").as_deref() == Ok("smoke_only"))
        .then(main_block_smoke_criterion_config)
        .transpose()?;
    let trace = smoke_config.is_some();
    if trace {
        eprintln!("MAIN_BLOCK_TRACE benchmark_enter benchmark_id={bench_id} num_tx={num_tx}");
    }

    bencher.iter_custom({
        let runtime = Arc::clone(&runtime);
        let listener_db = listener_db;
        let handle_counter = Arc::clone(&handle_counter);
        let submission_audit = Arc::clone(&submission_audit);
        let progress_pool = pool.clone();
        let smoke_config = smoke_config.as_ref().map(|config| config.max_requested_iters);
        move |iters| {
            let listener_db = listener_db.clone();
            let handle_counter = Arc::clone(&handle_counter);
            let submission_audit = Arc::clone(&submission_audit);
            let progress_pool = progress_pool.clone();
            let db_url = db_url.clone();
            let max_requested_iters = smoke_config;
            let trace = trace;
            runtime.block_on(async move {
                if let Some(max_requested_iters) = max_requested_iters {
                    if iters > max_requested_iters {
                        panic!(
                            "smoke-only main_block_baseline Criterion requested {iters} iterations in one call; maximum is {max_requested_iters}. Set FHEVM_BENCH_SMOKE_MAX_REQUESTED_ITERS to an integer in 1..=64 only when validating that many fresh submissions."
                        );
                    }
                }
                execute_main_block_requested_iterations(
                    iters,
                    MainBlockIterationContext {
                        listener_db: &listener_db,
                        num_tx,
                        handle_counter: &handle_counter,
                        submission_audit: &submission_audit,
                        progress_pool: &progress_pool,
                        db_url: &db_url,
                        trace,
                    },
                )
                .await
                .unwrap_or_else(|error| panic!("main_block_baseline Criterion closure: {error}"))
            })
        }
    });
    if trace {
        eprintln!("MAIN_BLOCK_TRACE benchmark_iter_custom_return");
    }

    // `TestInstance` owns an async Testcontainers handle. Drop it while the
    // Tokio reactor is entered; dropping it after `block_on` causes
    // Testcontainers to panic because its cleanup is asynchronous.
    let (write_params, shutdown) = runtime.block_on(async {
        let write_params = write_atomic_u64_bench_params(&pool, &bench_id, "erc20-transfer").await;
        let shutdown = app.shutdown().await;
        drop(app);
        (write_params, shutdown)
    });
    write_params?;
    shutdown?;
    if let Some(smoke_config) = smoke_config {
        let audit = submission_audit
            .lock()
            .expect("main_block submission audit mutex");
        let artifact = persist_main_block_smoke_artifact(
            "criterion_shortened",
            serde_json::json!({
                "worker_semantics": "main_native_legacy_computations_ciphertexts_dependence_chain",
                "template": {
                    "input_source": "legacy TrivialEncrypt events; no client-encrypted-input fixture exists in this benchmark",
                    "preparation": "events, legacy computations, and DCID relation are staged before commit visibility",
                    "unique_submission_identity": "each iteration advances fresh random-seeded legacy handles and transaction/DCID ids",
                },
                "timer": "starts immediately before transaction commit/worker visibility and ends when exact terminal rows are completed",
                "criterion": {
                    "requested_calls": audit.requested_calls,
                    "requested_iterations_total": audit.requested_calls.iter().sum::<u64>(),
                    "submitted_iterations": audit.iterations,
                    "unique_transaction_dcids": audit.transaction_ids.len(),
                    "database_legacy_computation_counts": audit.db_computation_counts,
                    "max_requested_iterations_observed": audit.max_requested_iters,
                    "max_requested_iterations_allowed": smoke_config.max_requested_iters,
                    "warmup_secs": smoke_config.warmup_secs,
                    "measurement_secs": smoke_config.measurement_secs,
                    "sample_size": smoke_config.sample_size,
                    "nresamples": smoke_config.nresamples,
                },
            }),
        )?;
        println!(
            "main_block_baseline Criterion smoke artifact: {}",
            artifact.display()
        );
    }
    if trace {
        eprintln!("MAIN_BLOCK_TRACE benchmark_return");
    }
    Ok(())
}

#[cfg(test)]
mod canonical_workload_tests {
    #[test]
    fn traffic_workload_maps_each_chain_to_the_block_two_back() {
        for block_index in 0..200 {
            for index_in_block in 0..5 {
                assert_eq!(
                    super::unpaced_traffic_chain_index(block_index, index_in_block, 5, 2),
                    (block_index % 2) * 5 + index_in_block,
                );
            }
        }
    }

    #[test]
    fn traffic_workload_contract_has_ten_chains_and_twenty_terminals() {
        let scenario = super::MAIN_BLOCK_ONE_SHOT_SCENARIOS
            .iter()
            .copied()
            .find(|scenario| scenario.name == "traffic_1000_200x5_10x100_lag2")
            .expect("canonical traffic scenario");
        assert_eq!(scenario.transfers, 1_000);
        assert_eq!(scenario.chain_len, 100);
        assert_eq!(scenario.chain_count(), 10);
        assert_eq!(scenario.terminal_handle_count(), 20);
        assert_eq!(
            scenario.staging,
            super::MainBlockOneShotStaging::SequentialUnpacedTraffic {
                l1_blocks: 200,
                transfers_per_block: 5,
                chain_count: 10,
                dependence_lag_blocks: 2,
            }
        );
    }
}
