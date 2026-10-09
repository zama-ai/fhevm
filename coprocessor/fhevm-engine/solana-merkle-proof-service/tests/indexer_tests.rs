//! The indexer sink against a real Postgres: a record built through crashes, redeliveries or a
//! dump and restore is the record an uninterrupted run builds, and its peaks are the peaks the
//! chain's own MMR appends produce.

mod support;

use std::collections::BTreeMap;
use std::process::Command;

use anchor_lang::InstructionData;
use serial_test::serial;
use solana_host_follower::host::DecodedInstruction;
use solana_host_follower::{
    BlockSink, IngestFailure, PreparedBlock, PreparedTransaction, SealedBlock,
};
use solana_merkle_proof_service::indexer::{IndexerStart, MerkleIndexerSink};
use solana_merkle_proof_service::store::{load_checkpoint, LeafReduceError};
use solana_merkle_proof_service::MIGRATOR;
use solana_sdk::signature::Signature;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use zama_solana_acl::{mmr_append, public_decrypt_leaf_commitment};

const STORES: [[u8; 32]; 2] = [[0xA1; 32], [0xB2; 32]];
const FIRST_SLOT: u64 = 100;
const BLOCKS: u64 = 12;

/// A `make_store_handle_public` of `handle` on `store`: one public-decrypt leaf.
fn make_public(
    store: [u8; 32],
    handle: [u8; 32],
    previous_leaf_count: u64,
) -> DecodedInstruction {
    let mut accounts = vec![[0; 32]; 3];
    accounts[2] = store;
    DecodedInstruction {
        data: zama_host::instruction::MakeStoreHandlePublic {
            key: [0; 32],
            handle,
            previous_leaf_count,
        }
        .data(),
        accounts,
    }
}

fn handle(slot: u64, store: usize, leaf: u64) -> [u8; 32] {
    let mut handle = [0; 32];
    handle[..8].copy_from_slice(&slot.to_be_bytes());
    handle[8] = store as u8;
    handle[9..17].copy_from_slice(&leaf.to_be_bytes());
    handle
}

fn block_hash(slot: u64) -> [u8; 32] {
    let mut hash = [0xBB; 32];
    hash[..8].copy_from_slice(&slot.to_be_bytes());
    hash
}

/// Twelve chained blocks. The first store gets one to three leaves per block, the second one
/// leaf every other block, and every fourth block has no host transaction, so blocks complete
/// nodes of several heights and some blocks only move the checkpoint.
fn blocks() -> Vec<PreparedBlock> {
    let mut leaf_counts = [0u64; 2];
    (FIRST_SLOT..FIRST_SLOT + BLOCKS)
        .map(|slot| {
            let mut transactions = Vec::new();
            if slot % 4 != 3 {
                let mut writes = |store: usize, leaves: u64| {
                    (0..leaves)
                        .map(|_| {
                            let leaf = leaf_counts[store];
                            leaf_counts[store] += 1;
                            make_public(
                                STORES[store],
                                handle(slot, store, leaf),
                                leaf,
                            )
                        })
                        .collect::<Vec<_>>()
                };
                let first = writes(0, 1 + slot % 3);
                let second = writes(1, (slot % 2 == 0).into());
                for (index, instructions) in
                    [first, second].into_iter().enumerate()
                {
                    if !instructions.is_empty() {
                        transactions.push(PreparedTransaction {
                            signature: Signature::from([slot as u8; 64]),
                            index: index as u64,
                            instructions,
                        });
                    }
                }
            }
            PreparedBlock {
                block: SealedBlock {
                    slot,
                    block_hash: block_hash(slot),
                    parent_slot: slot - 1,
                    parent_block_hash: block_hash(slot - 1),
                    block_time: Some(1_700_000_000 + slot as i64),
                    block_height: Some(slot),
                    executed_transaction_count: transactions.len() as u64,
                },
                transactions,
            }
        })
        .collect()
}

/// Each store's peaks as the host's own appends produce them from the blocks' writes.
fn chain_peaks(
    blocks: &[PreparedBlock],
) -> BTreeMap<[u8; 32], (u64, Vec<[u8; 32]>)> {
    let mut stores = BTreeMap::new();
    for block in blocks {
        for transaction in &block.transactions {
            for instruction in &transaction.instructions {
                let store = instruction.accounts[2];
                let args =
                    zama_host::decode::decode_instruction(&instruction.data);
                let Ok(Some(zama_host::decode::ZamaHostInstruction::MakeStoreHandlePublic {
                    handle,
                    ..
                })) = args
                else {
                    panic!("fixture holds only make_store_handle_public");
                };
                let (count, peaks) =
                    stores.entry(store).or_insert((0, Vec::new()));
                let commitment =
                    public_decrypt_leaf_commitment(store, *count, handle);
                mmr_append(peaks, count, commitment).unwrap();
            }
        }
    }
    stores
}

type StoreRow = (Vec<u8>, i64, Vec<Vec<u8>>);
type LeafRow = (
    Vec<u8>,
    i64,
    Vec<u8>,
    i16,
    Vec<u8>,
    Option<Vec<u8>>,
    i64,
    i64,
);
type NodeRow = (Vec<u8>, i16, i64, Vec<u8>);

/// Every row of the record, in key order.
#[derive(Debug, PartialEq)]
struct Record {
    stores: Vec<StoreRow>,
    leaves: Vec<LeafRow>,
    nodes: Vec<NodeRow>,
    checkpoint: Option<(i64, Vec<u8>)>,
}

impl Record {
    async fn read(pool: &PgPool) -> Self {
        Self {
            stores: sqlx::query_as(
                "SELECT encrypted_store, leaf_count, peaks FROM encrypted_stores \
                 ORDER BY encrypted_store",
            )
            .fetch_all(pool)
            .await
            .unwrap(),
            leaves: sqlx::query_as(
                "SELECT encrypted_store, leaf_index, commitment, leaf_kind, handle, allowed_key, \
                 block_slot, transaction_index FROM leaves ORDER BY encrypted_store, leaf_index",
            )
            .fetch_all(pool)
            .await
            .unwrap(),
            nodes: sqlx::query_as(
                "SELECT encrypted_store, height, node_index, node FROM nodes \
                 ORDER BY encrypted_store, height, node_index",
            )
            .fetch_all(pool)
            .await
            .unwrap(),
            checkpoint: sqlx::query_as(
                "SELECT slot, block_hash FROM checkpoint",
            )
                .fetch_optional(pool)
                .await
                .unwrap(),
        }
    }

    /// The recorded peaks equal the chain's for every store.
    fn assert_matches_chain(&self, blocks: &[PreparedBlock]) {
        let recorded: BTreeMap<[u8; 32], (u64, Vec<[u8; 32]>)> = self
            .stores
            .iter()
            .map(|(store, leaf_count, peaks)| {
                (
                    store.as_slice().try_into().unwrap(),
                    (
                        *leaf_count as u64,
                        peaks
                            .iter()
                            .map(|peak| peak.as_slice().try_into().unwrap())
                            .collect(),
                    ),
                )
            })
            .collect();
        assert_eq!(recorded, chain_peaks(blocks));
    }
}

async fn apply(
    pool: &PgPool,
    block: &PreparedBlock,
) -> Result<(), IngestFailure> {
    MerkleIndexerSink::new(pool.clone()).apply(block).await
}

async fn apply_all(pool: &PgPool, blocks: &[PreparedBlock]) {
    for block in blocks {
        apply(pool, block).await.unwrap();
    }
}

/// A process killed at any point of a block, or restarted after it, leaves the record it would
/// have built uninterrupted. A crash inside a block's transaction commits none of the block, its
/// checkpoint included, and is retryable; the follower then hands the block again. An `apply`
/// dropped on timeout may still commit, so the follower can also hand a committed block again,
/// which must change nothing.
#[tokio::test]
#[serial(db)]
async fn a_record_built_through_crashes_is_the_uninterrupted_record() {
    let blocks = blocks();
    let (_reference_db, reference) = support::record_db().await;
    apply_all(&reference, &blocks).await;
    let expected = Record::read(&reference).await;
    expected.assert_matches_chain(&blocks);
    assert!(!expected.nodes.is_empty());

    let (_db, pool) = support::record_db().await;
    sqlx::raw_sql(
        "CREATE FUNCTION injected_crash() RETURNS trigger LANGUAGE plpgsql AS $$
         BEGIN RAISE EXCEPTION 'injected crash'; END $$",
    )
    .execute(&pool)
    .await
    .unwrap();
    // The block's statements in order: store cursors, leaves, nodes, then the checkpoint.
    let crashes = BTreeMap::from([
        (FIRST_SLOT + 1, "encrypted_stores"),
        (FIRST_SLOT + 2, "leaves"),
        (FIRST_SLOT + 5, "nodes"),
        (FIRST_SLOT + 6, "checkpoint"),
        (FIRST_SLOT + 7, "checkpoint"),
    ]);
    for block in &blocks {
        let before = Record::read(&pool).await;
        if let Some(table) = crashes.get(&block.block.slot) {
            sqlx::query(&format!(
                "CREATE TRIGGER injected_crash BEFORE INSERT ON {table} \
                 FOR EACH STATEMENT EXECUTE FUNCTION injected_crash()"
            ))
            .execute(&pool)
            .await
            .unwrap();
            let failure = apply(&pool, block).await.unwrap_err();
            assert!(
                !failure.is_fatal(),
                "slot {}: {failure}",
                block.block.slot
            );
            assert_eq!(
                Record::read(&pool).await,
                before,
                "slot {}: a crash writing {table} left part of its block",
                block.block.slot
            );
            sqlx::query(&format!("DROP TRIGGER injected_crash ON {table}"))
                .execute(&pool)
                .await
                .unwrap();
        }
        apply(&pool, block).await.unwrap();
        let after = Record::read(&pool).await;
        apply(&pool, block).await.unwrap();
        assert_eq!(
            Record::read(&pool).await,
            after,
            "slot {}: re-applying the committed block changed the record",
            block.block.slot
        );
    }
    assert_eq!(Record::read(&pool).await, expected);
}

/// Rewinding only the checkpoint leaves the store cursors ahead of the block's writes.
#[tokio::test]
#[serial(db)]
async fn a_rewound_checkpoint_cannot_reapply_recorded_leaves() {
    let blocks = blocks();
    let (_db, pool) = support::record_db().await;
    apply_all(&pool, &blocks).await;

    let rewind = &blocks[4].block;
    sqlx::query("UPDATE checkpoint SET slot = $1, block_hash = $2")
        .bind(rewind.slot as i64)
        .bind(&rewind.block_hash[..])
        .execute(&pool)
        .await
        .unwrap();
    let before = Record::read(&pool).await;
    let checkpoint: String = sqlx::query_scalar(
        "SELECT row_to_json(checkpoint)::text FROM checkpoint",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let failure = apply(&pool, &blocks[5]).await.unwrap_err();
    assert!(failure.is_fatal(), "{failure}");
    let expected_error = LeafReduceError::PreviousLeafCountMismatch {
        encrypted_store: STORES[0],
        declared: chain_peaks(&blocks[..5])[&STORES[0]].0,
        recorded: before.stores[0].1 as u64,
    };
    assert!(
        format!("{failure:?}").contains(&expected_error.to_string()),
        "{failure:?}"
    );
    assert_eq!(Record::read(&pool).await, before);
    assert_eq!(
        sqlx::query_scalar::<_, String>(
            "SELECT row_to_json(checkpoint)::text FROM checkpoint",
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        checkpoint
    );
}

/// A block at or below the checkpoint was already recorded and changes nothing. Only a different
/// hash at the checkpoint's slot stops the indexer.
#[tokio::test]
#[serial(db)]
async fn only_a_different_hash_at_the_checkpoint_stops_the_indexer() {
    let blocks = blocks();
    let (_db, pool) = support::record_db().await;
    apply_all(&pool, &blocks[..3]).await;
    let before = Record::read(&pool).await;
    let checkpoint: String = sqlx::query_scalar(
        "SELECT row_to_json(checkpoint)::text FROM checkpoint",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    for block in [&blocks[2], &blocks[1]] {
        apply(&pool, block).await.unwrap();
        assert_eq!(Record::read(&pool).await, before);
    }
    let mut conflicting = blocks[2].clone();
    conflicting.block.block_hash = [0xEE; 32];
    let failure = apply(&pool, &conflicting).await.unwrap_err();
    assert!(failure.is_fatal(), "{failure}");
    assert!(
        failure
            .to_string()
            .contains("block hash differs from the recorded checkpoint"),
        "{failure}"
    );
    assert_eq!(Record::read(&pool).await, before);
    assert_eq!(
        sqlx::query_scalar::<_, String>(
            "SELECT row_to_json(checkpoint)::text FROM checkpoint",
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        checkpoint
    );
}

/// Two replicas, each with its own pool, apply every block to one record: one after the other,
/// overlapping, and concurrently. The record is the one a single run builds, and the checkpoint
/// never moves back. The replica behind the checkpoint does not stop. The checkpoint is the only
/// guard: the leaf writes alone would apply a block twice, as
/// `a_rewound_checkpoint_cannot_reapply_recorded_leaves` shows.
#[tokio::test]
#[serial(db)]
async fn two_replicas_build_the_single_run_record() {
    let blocks = blocks();
    let (_reference_db, reference) = support::record_db().await;
    apply_all(&reference, &blocks).await;
    let expected = Record::read(&reference).await;

    let all = 0..blocks.len();
    let schedules: [Vec<(usize, usize)>; 2] = [
        all.clone()
            .map(|i| (0, i))
            .chain(all.clone().map(|i| (1, i)))
            .collect(),
        (0..6)
            .map(|i| (0, i))
            .chain((0..9).map(|i| (1, i)))
            .chain((6..12).map(|i| (0, i)))
            .chain((9..12).map(|i| (1, i)))
            .collect(),
    ];
    for schedule in schedules {
        let (db, pool) = support::record_db().await;
        let replicas = [
            MerkleIndexerSink::new(pool.clone()),
            MerkleIndexerSink::new(
                PgPoolOptions::new().connect(db.db_url()).await.unwrap(),
            ),
        ];
        let mut highest = 0;
        for (replica, index) in schedule {
            replicas[replica].apply(&blocks[index]).await.unwrap();
            let slot = load_checkpoint(&pool).await.unwrap().unwrap().slot;
            assert!(
                slot >= highest,
                "checkpoint moved back from {highest} to {slot}"
            );
            highest = slot;
        }
        assert_eq!(Record::read(&pool).await, expected);
    }

    let (db, pool) = support::record_db().await;
    let second = MerkleIndexerSink::new(
        PgPoolOptions::new().connect(db.db_url()).await.unwrap(),
    );
    let first = MerkleIndexerSink::new(pool.clone());
    for block in &blocks {
        let (a, b) = tokio::join!(first.apply(block), second.apply(block));
        a.unwrap();
        b.unwrap();
    }
    assert_eq!(Record::read(&pool).await, expected);
}

/// A store whose first write the record sees already held leaves was created before the
/// record's start block: the indexer stops instead of recording a partial history.
#[tokio::test]
#[serial(db)]
async fn a_store_first_seen_above_leaf_zero_stops_the_indexer() {
    let blocks = blocks();
    let (_db, pool) = support::record_db().await;
    apply_all(&pool, &blocks[..2]).await;
    let before = Record::read(&pool).await;

    let mut late = blocks[2].clone();
    late.transactions[0]
        .instructions
        .push(make_public([0xC3; 32], [0xEE; 32], 3));
    let failure = apply(&pool, &late).await.unwrap_err();
    assert!(failure.is_fatal(), "{failure}");
    assert_eq!(Record::read(&pool).await, before);
}

/// A `pg_dump` of the record taken at some block, restored into an empty database, resumes from
/// its own checkpoint and catches up to the record an uninterrupted run builds. `pg_dump` and
/// `pg_restore` run inside the database container, so their version is the server's.
#[tokio::test]
#[serial(db)]
async fn a_restored_dump_resumes_and_catches_up() {
    let blocks = blocks();
    let (db, pool) = support::record_db().await;
    let container = db
        .container_id()
        .expect("the dump runs inside the test database container");
    let dumped_at = 7;
    apply_all(&pool, &blocks[..dumped_at]).await;
    let database = pool.connect_options().get_database().unwrap().to_owned();
    let docker = |args: &[&str]| {
        let output = Command::new("docker")
            .args(["exec", container])
            .args(args)
            .output()
            .expect("run docker");
        assert!(
            output.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    };
    docker(&[
        "pg_dump",
        "-U",
        "postgres",
        "-Fc",
        "-f",
        "/tmp/record.dump",
        &database,
    ]);
    docker(&["createdb", "-U", "postgres", "restored"]);
    docker(&[
        "pg_restore",
        "-U",
        "postgres",
        "-d",
        "restored",
        "/tmp/record.dump",
    ]);

    let restored = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(
            pool.connect_options().as_ref().clone().database("restored"),
        )
        .await
        .unwrap();
    // The dump carries the migration history, so the indexer's start accepts it unchanged.
    MIGRATOR.run(&restored).await.unwrap();
    let start =
        IndexerStart::resolve(load_checkpoint(&restored).await.unwrap(), None)
            .unwrap();
    assert_eq!(
        start,
        IndexerStart::Resume(blocks[dumped_at - 1].block.checkpoint())
    );
    apply_all(&restored, &blocks[dumped_at..]).await;

    apply_all(&pool, &blocks[dumped_at..]).await;
    let expected = Record::read(&pool).await;
    assert_eq!(Record::read(&restored).await, expected);
    expected.assert_matches_chain(&blocks);
}
