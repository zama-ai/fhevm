#[path = "./utils.rs"]
mod utils;

use crate::utils::{
    as_scalar_uint, listener_event_db, next_handle, random_handle, scalar_flag, setup_test_app,
    tfhe_event, to_ty, wait_until_all_allowed_handles_computed, write_atomic_u64_bench_params,
    zero_address, EnvConfig,
};
use alloy::primitives::B256;
use bigdecimal::num_bigint::BigInt;
use criterion::{
    async_executor::FuturesExecutor, measurement::WallTime, Bencher, Criterion, Throughput,
};
use fhevm_engine_common::tfhe_ops::current_ciphertext_version;
use fhevm_engine_common::types::AllowEvents;
use fhevm_engine_common::types::SupportedFheCiphertexts;
use fhevm_engine_common::utils::safe_deserialize_key;
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{Handle, ProducerBlock};
use std::time::SystemTime;
use tfhe::prelude::CiphertextList;
use tfhe::xof_key_set::CompressedXofKeySet;
use tfhe::CompactCiphertextListExpander;
use tfhe_worker::tfhe_worker::{TFHE_EXECUTION_TIMING, TIMING};
use tokio::runtime::Runtime;

fn main() {
    let ecfg = EnvConfig::new();
    let mut c = Criterion::default()
        .sample_size(10)
        .measurement_time(std::time::Duration::from_secs(1000))
        .configure_from_args();
    let bench_name = "erc20::transfer";

    let mut group = c.benchmark_group(bench_name);
    if ecfg.benchmark_type == "LATENCY" || ecfg.benchmark_type == "ALL" {
        let num_elems = 1;
        let bench_id = format!("{bench_name}::latency::whitepaper::FHEUint64::{num_elems}_elems");
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            let _ = Runtime::new().unwrap().block_on(schedule_erc20_whitepaper(
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
                format!("{bench_name}::throughput::whitepaper::FHEUint64::{num_elems}_elems");
            group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
                Runtime::new()
                    .unwrap()
                    .block_on(schedule_erc20_whitepaper(
                        b,
                        num_elems as usize,
                        bench_id.clone(),
                    ))
                    .unwrap();
            });

            group.throughput(Throughput::Elements(num_elems));
            let bench_id = format!(
                "{bench_name}::throughput::dependent_whitepaper::FHEUint64::{num_elems}_elems"
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
        }
    }

    if (ecfg.benchmark_type == "THROUGHPUT" || ecfg.benchmark_type == "ALL")
        && std::env::var("FHEVM_BENCH_EXTENDED").as_deref() == Ok("1")
    {
        let num_elems = 300;
        group.throughput(Throughput::Elements(num_elems));
        let bench_id = format!(
            "{bench_name}::throughput::independent::FHEUint64::{num_elems}_elems::300_per_block"
        );
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            Runtime::new()
                .unwrap()
                .block_on(schedule_erc20_independent_300(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ))
                .unwrap();
        });

        let num_elems = 300;
        group.throughput(Throughput::Elements(num_elems));
        let bench_id = format!(
            "{bench_name}::throughput::dependent::FHEUint64::{num_elems}_elems::300_per_block::6x50_dependent"
        );
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            Runtime::new()
                .unwrap()
                .block_on(schedule_erc20_dependent_300(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ))
                .unwrap();
        });

        let num_elems = 5000;
        group.throughput(Throughput::Elements(num_elems));
        let bench_id = format!(
            "{bench_name}::throughput::independent::FHEUint64::{num_elems}_elems::500_per_block"
        );
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            Runtime::new()
                .unwrap()
                .block_on(schedule_erc20_independent_5000(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ))
                .unwrap();
        });

        let num_elems = 10000;
        group.throughput(Throughput::Elements(num_elems));
        let bench_id = format!(
            "{bench_name}::throughput::mixed::FHEUint64::{num_elems}_elems::1000_per_block::50x20_dependent"
        );
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            Runtime::new()
                .unwrap()
                .block_on(schedule_erc20_mixed_10000(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ))
                .unwrap();
        });

        let num_elems = 2000;
        group.throughput(Throughput::Elements(num_elems));
        let bench_id = format!(
            "{bench_name}::throughput::realistic::FHEUint64::{num_elems}_elems::40_per_block::10x4_dependent"
        );
        group.bench_with_input(bench_id.clone(), &num_elems, move |b, &num_elems| {
            Runtime::new()
                .unwrap()
                .block_on(schedule_erc20_realistic_2000(
                    b,
                    num_elems as usize,
                    bench_id.clone(),
                ))
                .unwrap();
        });
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

#[derive(Clone, Copy)]
struct BenchmarkBlock {
    hash: B256,
    number: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TransferCircuit {
    CoprocessorCmux,
    NoCmux,
    TfheRsWhitepaper,
}

#[derive(Clone, Copy)]
struct EncryptedTransferInputs {
    from_balance: Handle,
    to_balance: Handle,
    amount: Handle,
}

#[derive(Default)]
struct TfheRsWhitepaperCounts {
    ge: usize,
    trivial_zero: usize,
    select: usize,
    add: usize,
    sub: usize,
}

impl TfheRsWhitepaperCounts {
    fn assert_exact(&self, transfers: usize) {
        assert_eq!(self.ge, transfers, "one encrypted ge per transfer");
        assert_eq!(
            self.trivial_zero, transfers,
            "one trivial u64 zero per transfer"
        );
        assert_eq!(self.select, transfers, "one encrypted select per transfer");
        assert_eq!(self.add, transfers, "one encrypted add per transfer");
        assert_eq!(self.sub, transfers, "one encrypted sub per transfer");
    }
}

fn benchmark_block(handle_counter: &mut u64, number: u64) -> BenchmarkBlock {
    let mut hash = [0_u8; 32];
    let block_nonce = *handle_counter;
    *handle_counter += 1;
    hash[24..].copy_from_slice(&block_nonce.to_be_bytes());
    BenchmarkBlock {
        hash: B256::from(hash),
        number,
    }
}

fn log_with_tx(
    tx_hash: host_listener::database::tfhe_event_propagate::Handle,
    inner: alloy::primitives::Log<TfheContractEvents>,
    block: BenchmarkBlock,
) -> alloy::rpc::types::Log<TfheContractEvents> {
    alloy::rpc::types::Log {
        inner,
        block_hash: Some(block.hash),
        block_number: Some(block.number),
        block_timestamp: None,
        transaction_hash: Some(tx_hash),
        transaction_index: Some(0),
        log_index: Some(next_log_index()),
        removed: false,
    }
}

async fn allow_handle_at_block(
    db: &host_listener::database::tfhe_event_propagate::Database,
    tx: &mut host_listener::database::tfhe_event_propagate::Transaction<'_>,
    handle: &host_listener::database::tfhe_event_propagate::Handle,
    block: BenchmarkBlock,
) -> Result<bool, sqlx::Error> {
    db.insert_allowed_handle(
        tx,
        handle.to_vec(),
        String::new(),
        AllowEvents::AllowedForDecryption,
        None,
        ProducerBlock::new(block.hash.as_slice(), block.number),
    )
    .await
}

fn encrypted_input_ciphertexts() -> Result<[Vec<u8>; 3], Box<dyn std::error::Error>> {
    let keyset_bytes = std::fs::read("../fhevm-keys/xof-keyset")?;
    let keyset: CompressedXofKeySet = safe_deserialize_key(&keyset_bytes)?;
    let (compact_public_key, server_key) = keyset.decompress()?.into_raw_parts();
    tfhe::set_server_key(server_key);

    let mut builder = tfhe::CompactCiphertextList::builder(&compact_public_key);
    builder.push(100_u64);
    builder.push(10_u64);
    builder.push(20_u64);
    let expanded: CompactCiphertextListExpander = builder.build().expand()?;
    let encrypt = |index| {
        let ciphertext: tfhe::FheUint64 = expanded
            .get(index)?
            .ok_or_else(|| "missing expanded input ciphertext".to_string())?;
        SupportedFheCiphertexts::FheUint64(ciphertext)
            .compress()
            .map_err(|err| -> Box<dyn std::error::Error> {
                format!("compress input: {err:?}").into()
            })
    };
    Ok([encrypt(0)?, encrypt(1)?, encrypt(2)?])
}

async fn seed_input_ciphertext(
    db: &host_listener::database::tfhe_event_propagate::Database,
    tx: &mut host_listener::database::tfhe_event_propagate::Transaction<'_>,
    handle: &host_listener::database::tfhe_event_propagate::Handle,
    ciphertext: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let producer_block_hash: &[u8] = &[];
    sqlx::query(
        "INSERT INTO ciphertexts_branch(
             handle, producer_block_hash, block_number, ciphertext,
             ciphertext_version, ciphertext_type
         ) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(handle.as_slice())
    .bind(producer_block_hash)
    .bind(0_i64)
    .bind(ciphertext)
    .bind(current_ciphertext_version())
    .bind(5_i16)
    .execute(tx.as_mut())
    .await?;
    db.insert_allowed_handle(
        tx,
        handle.to_vec(),
        String::new(),
        AllowEvents::AllowedForDecryption,
        None,
        ProducerBlock::new(&[], 0),
    )
    .await?;
    Ok(())
}

async fn schedule_erc20(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    circuit: TransferCircuit,
    dependent: bool,
    block_size: usize,
    chain_len: usize,
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
    let input_ciphertexts = encrypted_input_ciphertexts()?;
    let block_size = block_size.max(1);
    let chain_len = chain_len.max(1);
    let block_count = num_samples.div_ceil(block_size);
    let chains_per_block = block_size.div_ceil(chain_len);
    let blocks_and_chain_ids: Vec<(BenchmarkBlock, Vec<Handle>)> = (0..block_count)
        .map(|block_index| {
            let remaining = num_samples.saturating_sub(block_index * block_size);
            let transfers_in_block = remaining.min(block_size);
            let chains_in_block = transfers_in_block.div_ceil(chain_len);
            let block = benchmark_block(&mut handle_counter, block_index as u64 + 1);
            let chain_ids = (0..chains_in_block)
                .map(|_| next_handle(&mut handle_counter))
                .collect();
            (block, chain_ids)
        })
        .collect();

    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    let mut seeded_boundary_inputs = 0_usize;
    let transfer_inputs = if circuit == TransferCircuit::TfheRsWhitepaper {
        // tfhe-rs encrypts three independent non-trivial FheUint64 values for
        // every independent transfer. For a dependency chain, only its first
        // balances enter from the block boundary; every transfer still gets a
        // distinct encrypted amount. Reusing the real noisy ciphertext bytes
        // as templates keeps setup tractable, while unique handles force the
        // coprocessor to materialize each logical input independently.
        let mut inputs = Vec::with_capacity(num_samples);
        let mut chain_roots: Vec<Option<(Handle, Handle)>> = vec![None; chains_per_block];
        for i in 0..num_samples {
            let index_in_block = i % block_size;
            let chain_index = index_in_block / chain_len;
            let chain_position = index_in_block % chain_len;
            if index_in_block == 0 {
                chain_roots.fill(None);
            }

            let (from_balance, to_balance) = if dependent && chain_position > 0 {
                chain_roots[chain_index].expect("dependency chain root inputs")
            } else {
                let from_balance = next_handle(&mut handle_counter);
                let to_balance = next_handle(&mut handle_counter);
                seed_input_ciphertext(&listener_db, &mut tx, &from_balance, &input_ciphertexts[0])
                    .await?;
                seed_input_ciphertext(&listener_db, &mut tx, &to_balance, &input_ciphertexts[2])
                    .await?;
                seeded_boundary_inputs += 2;
                if dependent {
                    chain_roots[chain_index] = Some((from_balance, to_balance));
                }
                (from_balance, to_balance)
            };
            let amount = next_handle(&mut handle_counter);
            seed_input_ciphertext(&listener_db, &mut tx, &amount, &input_ciphertexts[1]).await?;
            seeded_boundary_inputs += 1;
            inputs.push(EncryptedTransferInputs {
                from_balance,
                to_balance,
                amount,
            });
        }
        inputs
    } else {
        // The legacy coprocessor scenarios intentionally deduplicate all
        // boundary inputs to three shared real noisy ciphertexts.
        let shared = EncryptedTransferInputs {
            from_balance: next_handle(&mut handle_counter),
            amount: next_handle(&mut handle_counter),
            to_balance: next_handle(&mut handle_counter),
        };
        for (handle, ciphertext) in [shared.from_balance, shared.amount, shared.to_balance]
            .iter()
            .zip(input_ciphertexts.iter())
        {
            seed_input_ciphertext(&listener_db, &mut tx, handle, ciphertext).await?;
        }
        vec![shared; num_samples]
    };

    if circuit == TransferCircuit::TfheRsWhitepaper {
        let expected_balance_inputs = if dependent {
            (0..block_count)
                .map(|block_index| {
                    let transfers = (num_samples - block_index * block_size).min(block_size);
                    2 * transfers.div_ceil(chain_len)
                })
                .sum::<usize>()
        } else {
            2 * num_samples
        };
        assert_eq!(
            seeded_boundary_inputs,
            expected_balance_inputs + num_samples,
            "tfhe-rs parity boundary input count"
        );
    }
    let mut prev_from: Vec<Option<Handle>> = vec![None; chains_per_block];
    let mut prev_to: Vec<Option<Handle>> = vec![None; chains_per_block];
    let mut tfhe_rs_counts = TfheRsWhitepaperCounts::default();

    for i in 0..num_samples {
        let block_index = i / block_size;
        let index_in_block = i % block_size;
        let chain_index = index_in_block / chain_len;
        let chain_position = index_in_block % chain_len;
        if index_in_block == 0 {
            prev_from.fill(None);
            prev_to.fill(None);
        }
        let (block, chain_ids) = &blocks_and_chain_ids[block_index];
        let block = *block;
        let tx_id = if dependent {
            chain_ids[chain_index]
        } else {
            next_handle(&mut handle_counter)
        };
        let from_balance = if dependent {
            if let Some(h) = prev_from[chain_index] {
                h
            } else {
                transfer_inputs[i].from_balance
            }
        } else {
            transfer_inputs[i].from_balance
        };
        let to_balance = if dependent {
            if let Some(h) = prev_to[chain_index] {
                h
            } else {
                transfer_inputs[i].to_balance
            }
        } else {
            transfer_inputs[i].to_balance
        };
        let transfer_amount = transfer_inputs[i].amount;

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
                block,
            ),
            tx_id,
            false,
        )
        .await?;
        if circuit == TransferCircuit::TfheRsWhitepaper {
            tfhe_rs_counts.ge += 1;
        }

        let new_to;
        let new_from;
        if circuit == TransferCircuit::CoprocessorCmux {
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
                    block,
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
                    block,
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
                    block,
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
                    block,
                ),
                tx_id,
                true,
            )
            .await?;
        } else if circuit == TransferCircuit::TfheRsWhitepaper {
            // Exact tfhe-rs GPU whitepaper circuit:
            //   has_funds = from.ge(amount)
            //   zero = FheUint64::encrypt_trivial(0)
            //   selected = has_funds.select(amount, zero)
            //   new_to = to + selected
            //   new_from = from - selected
            let zero_amount = next_handle(&mut handle_counter);
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::TrivialEncrypt(
                        TfheContract::TrivialEncrypt {
                            caller,
                            pt: as_scalar_uint(&BigInt::from(0_u64)),
                            toType: to_ty(5),
                            result: zero_amount,
                        },
                    )),
                    block,
                ),
                tx_id,
                false,
            )
            .await?;
            tfhe_rs_counts.trivial_zero += 1;

            let selected_amount = next_handle(&mut handle_counter);
            utils::insert_tfhe_event(
                &listener_db,
                &mut tx,
                log_with_tx(
                    tx_id,
                    tfhe_event(TfheContractEvents::FheIfThenElse(
                        TfheContract::FheIfThenElse {
                            caller,
                            control: has_funds,
                            ifTrue: transfer_amount,
                            ifFalse: zero_amount,
                            result: selected_amount,
                        },
                    )),
                    block,
                ),
                tx_id,
                false,
            )
            .await?;
            tfhe_rs_counts.select += 1;

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
                    block,
                ),
                tx_id,
                true,
            )
            .await?;
            tfhe_rs_counts.add += 1;

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
                    block,
                ),
                tx_id,
                true,
            )
            .await?;
            tfhe_rs_counts.sub += 1;
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
                    block,
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
                    block,
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
                    block,
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
                    block,
                ),
                tx_id,
                true,
            )
            .await?;
        }

        let chain_complete = !dependent || chain_position + 1 == chain_len || i + 1 == num_samples;
        if chain_complete {
            allow_handle_at_block(&listener_db, &mut tx, &new_to, block).await?;
            allow_handle_at_block(&listener_db, &mut tx, &new_from, block).await?;
        }
        if dependent {
            prev_from[chain_index] = Some(new_from);
            prev_to[chain_index] = Some(new_to);
        }
    }
    if circuit == TransferCircuit::TfheRsWhitepaper {
        tfhe_rs_counts.assert_exact(num_samples);
    }
    tx.commit().await?;

    let app_ref = &app;
    bencher
        .to_async(FuturesExecutor)
        .iter_custom(|iters| async move {
            let db_url = app_ref.db_url().to_string();
            let now = SystemTime::now();
            let end_to_end = tokio::task::spawn_blocking(move || {
                Runtime::new().unwrap().block_on(async {
                    wait_until_all_allowed_handles_computed(db_url)
                        .await
                        .unwrap()
                });
                now.elapsed().unwrap()
            })
            .await
            .unwrap();
            let worker_active = TIMING.swap(0, std::sync::atomic::Ordering::SeqCst);
            let tfhe_execution = TFHE_EXECUTION_TIMING.swap(0, std::sync::atomic::Ordering::SeqCst);
            println!(
                "TFHE execution time: {} ms; worker active time: {} ms; end-to-end time: {} ms",
                tfhe_execution / 1000,
                worker_active / 1000,
                end_to_end.as_millis(),
            );
            std::time::Duration::from_micros(tfhe_execution * iters.max(1))
        });

    write_atomic_u64_bench_params(&pool, bench_id, display_name).await?;
    Ok(())
}

async fn schedule_erc20_whitepaper(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(
        bencher,
        num_tx,
        TransferCircuit::TfheRsWhitepaper,
        false,
        num_tx,
        1,
        &bench_id,
        "erc20-transfer",
    )
    .await
}

async fn schedule_erc20_no_cmux(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(
        bencher,
        num_tx,
        TransferCircuit::NoCmux,
        false,
        num_tx,
        1,
        &bench_id,
        "erc20-transfer",
    )
    .await
}

async fn schedule_dependent_erc20_whitepaper(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(
        bencher,
        num_tx,
        TransferCircuit::TfheRsWhitepaper,
        true,
        num_tx,
        num_tx,
        &bench_id,
        "erc20-transfer",
    )
    .await
}

async fn schedule_dependent_erc20_no_cmux(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(
        bencher,
        num_tx,
        TransferCircuit::NoCmux,
        true,
        num_tx,
        num_tx,
        &bench_id,
        "erc20-transfer",
    )
    .await
}

async fn schedule_erc20_independent_5000(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(
        bencher,
        num_tx,
        TransferCircuit::TfheRsWhitepaper,
        false,
        500,
        1,
        &bench_id,
        "erc20-independent-5000-500-per-block",
    )
    .await
}

async fn schedule_erc20_independent_300(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(
        bencher,
        num_tx,
        TransferCircuit::TfheRsWhitepaper,
        false,
        300,
        1,
        &bench_id,
        "erc20-independent-300-300-per-block",
    )
    .await
}

async fn schedule_erc20_dependent_300(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(
        bencher,
        num_tx,
        TransferCircuit::TfheRsWhitepaper,
        true,
        300,
        50,
        &bench_id,
        "erc20-dependent-300-300-per-block-6x50-dependent",
    )
    .await
}

async fn schedule_erc20_mixed_10000(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(
        bencher,
        num_tx,
        TransferCircuit::TfheRsWhitepaper,
        true,
        1000,
        20,
        &bench_id,
        "erc20-mixed-10000-1000-per-block-50x20-dependent",
    )
    .await
}

async fn schedule_erc20_realistic_2000(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    schedule_erc20(
        bencher,
        num_tx,
        TransferCircuit::TfheRsWhitepaper,
        true,
        40,
        4,
        &bench_id,
        "erc20-realistic-2000-40-per-block-10x4-dependent",
    )
    .await
}

async fn schedule_tfhe_rs_whitepaper_scenario(
    bencher: &mut Bencher<'_, WallTime>,
    num_tx: usize,
    scenario: &str,
    bench_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let (dependent, block_size, chain_len, display_name) = match scenario {
        "independent_300" => (
            false,
            300,
            1,
            "tfhe-rs-whitepaper-independent-300-300-per-block",
        ),
        "dependent_300" => (
            true,
            300,
            50,
            "tfhe-rs-whitepaper-dependent-300-300-per-block-6x50-dependent",
        ),
        "realistic_2000" => (
            true,
            40,
            4,
            "tfhe-rs-whitepaper-realistic-2000-40-per-block-10x4-dependent",
        ),
        "independent_5000" => (
            false,
            500,
            1,
            "tfhe-rs-whitepaper-independent-5000-500-per-block",
        ),
        "mixed_10000" => (
            true,
            1000,
            20,
            "tfhe-rs-whitepaper-mixed-10000-1000-per-block-50x20-dependent",
        ),
        _ => return Err(format!("unknown tfhe-rs ERC20 scenario: {scenario}").into()),
    };

    schedule_erc20(
        bencher,
        num_tx,
        TransferCircuit::TfheRsWhitepaper,
        dependent,
        block_size,
        chain_len,
        &bench_id,
        display_name,
    )
    .await
}
