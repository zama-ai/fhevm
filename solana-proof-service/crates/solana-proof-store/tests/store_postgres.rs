//! Real-Postgres store tests. Skipped unless `DATABASE_URL` or
//! `SOLANA_PROOF_TEST_DATABASE_URL` is set.
//!
//! ```bash
//! DATABASE_URL=postgres://work@localhost:55432/solana_proof_service?host=/tmp make test-db
//! ```

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};

use borsh::BorshSerialize;
use sha2::{Digest, Sha256};
use solana_proof_source::{CanonicalTransaction, CompletedBlock, RawInstruction};
use solana_proof_store::{
    apply_instruction, decode_program_instructions, reduce_completed_block, ApplyOutcome,
    DecodedInstruction, LineageReplayState, PriorLineageState, SqlProofStore, SubjectGrant,
};
use zama_solana_acl::lineage::reconstruct;
use zama_solana_acl::mmr::mmr_peaks_from_leaves;

fn database_url() -> String {
    std::env::var("SOLANA_PROOF_TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("set DATABASE_URL or SOLANA_PROOF_TEST_DATABASE_URL for ignored postgres tests")
}

fn pk(tag: u8) -> [u8; 32] {
    [tag; 32]
}

fn discriminator(name: &str) -> [u8; 8] {
    let digest = Sha256::digest(format!("global:{name}").as_bytes());
    let mut out = [0u8; 8];
    out.copy_from_slice(&digest[..8]);
    out
}

fn ix(accounts: Vec<[u8; 32]>, name: &str, args: impl BorshSerialize) -> RawInstruction {
    let mut data = discriminator(name).to_vec();
    args.serialize(&mut data).unwrap();
    RawInstruction {
        program_id: pk(7),
        accounts,
        data,
        top_level_index: 0,
        stack_height: Some(1),
    }
}

fn create_ix(ev: [u8; 32], handle: [u8; 32], subject: [u8; 32]) -> RawInstruction {
    #[derive(BorshSerialize)]
    struct Args {
        acl_domain_key: [u8; 32],
        app_account: [u8; 32],
        label: [u8; 32],
        handle: [u8; 32],
        subjects: Vec<SubjectGrant>,
    }
    ix(
        vec![pk(0xA), pk(0xB), ev, pk(0xC), pk(0xD)],
        "create_encrypted_value",
        Args {
            acl_domain_key: pk(0x10),
            app_account: pk(0x11),
            label: pk(0x12),
            handle,
            subjects: vec![SubjectGrant { subject }],
        },
    )
}

fn update_ix(
    ev: [u8; 32],
    new_handle: [u8; 32],
    previous_handle: [u8; 32],
    previous_subjects: Vec<[u8; 32]>,
) -> RawInstruction {
    #[derive(BorshSerialize)]
    struct Args {
        new_handle: [u8; 32],
        previous_handle: [u8; 32],
        previous_subjects: Vec<[u8; 32]>,
    }
    ix(
        vec![pk(0xA), pk(0xB), ev, pk(0xC), pk(0xD)],
        "update_encrypted_value",
        Args {
            new_handle,
            previous_handle,
            previous_subjects,
        },
    )
}

fn make_public_ix(ev: [u8; 32], handle: [u8; 32]) -> RawInstruction {
    ix(
        vec![pk(0xA), pk(0xB), ev, pk(0xC), pk(0xD)],
        "make_handle_public",
        handle,
    )
}

static DB_SEQ: AtomicU64 = AtomicU64::new(1);

async fn fresh_store() -> SqlProofStore {
    let url = database_url();
    let seq = DB_SEQ.fetch_add(1, Ordering::SeqCst);
    let db_name = format!("solana_proof_it_{seq}_{}", std::process::id());
    let admin = sqlx::PgPool::connect(&url).await.expect("connect admin");
    sqlx::query(&format!("DROP DATABASE IF EXISTS {db_name}"))
        .execute(&admin)
        .await
        .ok();
    sqlx::query(&format!("CREATE DATABASE {db_name}"))
        .execute(&admin)
        .await
        .expect("create database");
    admin.close().await;

    let per_db = replace_database_name(&url, &db_name);
    let store = SqlProofStore::connect(&per_db, pk(7))
        .await
        .expect("connect store");
    store.migrate().await.expect("migrate");
    store
}

/// Replace only the path database name, preserving query params such as
/// `host=/tmp&port=55432`.
fn replace_database_name(url: &str, db_name: &str) -> String {
    let (base, query) = url
        .split_once('?')
        .map(|(base, query)| (base, Some(query)))
        .unwrap_or((url, None));
    let Some((prefix, _)) = base.rsplit_once('/') else {
        panic!("DATABASE_URL missing database path: {url}");
    };
    match query {
        Some(query) => format!("{prefix}/{db_name}?{query}"),
        None => format!("{prefix}/{db_name}"),
    }
}

fn block(
    slot: u64,
    parent_slot: u64,
    parent_hash: [u8; 32],
    block_hash: [u8; 32],
    txs: Vec<CanonicalTransaction>,
) -> CompletedBlock {
    CompletedBlock {
        slot,
        block_hash,
        parent_slot,
        parent_hash,
        block_time: Some(1_700_000_000),
        block_height: Some(slot),
        executed_transaction_count: txs.iter().map(|tx| tx.index + 1).max().unwrap_or(0),
        transactions: txs,
    }
}

fn sig(tag: u8) -> [u8; 64] {
    [tag; 64]
}

#[ignore = "requires DATABASE_URL / SOLANA_PROOF_TEST_DATABASE_URL"]
#[tokio::test]
async fn empty_block_atomic_checkpoint() {
    let store = fresh_store().await;
    let b = block(10, 9, pk(0x90), pk(0xA0), Vec::new());
    assert_eq!(
        store.apply_completed_block(&b).await.unwrap(),
        ApplyOutcome::Applied
    );
    let status = store.integrity_status().await.unwrap();
    assert!(!status.history_complete);
    assert_eq!(status.history_start.unwrap().slot, 10);
    assert_eq!(status.checkpoint.unwrap().slot, 10);
    assert!(!status.integrity_halted);
}

#[ignore = "requires DATABASE_URL / SOLANA_PROOF_TEST_DATABASE_URL"]
#[tokio::test]
async fn failed_and_vote_zero_leaf_identities() {
    let store = fresh_store().await;
    let b = block(
        10,
        9,
        pk(0x90),
        pk(0xA0),
        vec![
            CanonicalTransaction {
                signature: sig(1),
                index: 0,
                succeeded: false,
                is_vote: false,
                instructions: Vec::new(),
            },
            CanonicalTransaction {
                signature: sig(2),
                index: 2,
                succeeded: true,
                is_vote: true,
                instructions: Vec::new(),
            },
        ],
    );
    assert_eq!(
        store.apply_completed_block(&b).await.unwrap(),
        ApplyOutcome::Applied
    );
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM solana_proof_leaves")
        .fetch_one(store.pool())
        .await
        .unwrap();
    assert_eq!(count.0, 0);
    let txs: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM solana_proof_transactions")
        .fetch_one(store.pool())
        .await
        .unwrap();
    assert_eq!(txs.0, 2);
}

#[ignore = "requires DATABASE_URL / SOLANA_PROOF_TEST_DATABASE_URL"]
#[tokio::test]
async fn deterministic_leaf_order_and_mmr_reconstruction() {
    let store = fresh_store().await;
    let ev = pk(0xE1);
    let owner = pk(0x30);
    let create = create_ix(ev, pk(0x10), owner);
    let update = update_ix(ev, pk(0x11), pk(0x10), vec![owner]);
    let make_public = make_public_ix(ev, pk(0x11));
    let b = block(
        10,
        9,
        pk(0x90),
        pk(0xA0),
        vec![CanonicalTransaction {
            signature: sig(3),
            index: 1,
            succeeded: true,
            is_vote: false,
            instructions: vec![create, update, make_public],
        }],
    );
    assert_eq!(
        store.apply_completed_block(&b).await.unwrap(),
        ApplyOutcome::Applied
    );
    let snap = store.proof_snapshot(ev).await.unwrap().unwrap();
    assert_eq!(snap.leaf_count, 2);
    assert_eq!(snap.leaves.len(), 2);
    assert_eq!(snap.peaks, mmr_peaks_from_leaves(&snap.leaves));

    let mut state = None;
    let decoded = decode_program_instructions(pk(7), &b.transactions[0].instructions).unwrap();
    let mut events = Vec::new();
    for instruction in &decoded {
        events.extend(apply_instruction(&mut state, instruction).unwrap());
    }
    let reconstructed = reconstruct(ev, &events).unwrap();
    assert_eq!(reconstructed.leaves, snap.leaves);
    assert_eq!(reconstructed.peaks, snap.peaks);
}

#[ignore = "requires DATABASE_URL / SOLANA_PROOF_TEST_DATABASE_URL"]
#[tokio::test]
async fn exact_overlap_and_reopen_replay_are_noop() {
    let store = fresh_store().await;
    let b = block(10, 9, pk(0x90), pk(0xA0), Vec::new());
    assert_eq!(
        store.apply_completed_block(&b).await.unwrap(),
        ApplyOutcome::Applied
    );
    assert_eq!(
        store.apply_completed_block(&b).await.unwrap(),
        ApplyOutcome::AlreadyApplied
    );
    // Re-open style: new store handle, same DB URL.
    let store2 = SqlProofStore::new(store.pool().clone(), pk(7));
    assert_eq!(
        store2.apply_completed_block(&b).await.unwrap(),
        ApplyOutcome::AlreadyApplied
    );
}

#[ignore = "requires DATABASE_URL / SOLANA_PROOF_TEST_DATABASE_URL"]
#[tokio::test]
async fn conflicting_replay_halts_without_partial_domain_state() {
    let store = fresh_store().await;
    let first = block(10, 9, pk(0x90), pk(0xA0), Vec::new());
    assert_eq!(
        store.apply_completed_block(&first).await.unwrap(),
        ApplyOutcome::Applied
    );
    let conflict = block(10, 9, pk(0x90), pk(0xFF), Vec::new());
    match store.apply_completed_block(&conflict).await.unwrap() {
        ApplyOutcome::IntegrityHalted { reason } => {
            assert!(reason.contains("conflicting"));
        }
        other => panic!("expected halt, got {other:?}"),
    }
    let status = store.integrity_status().await.unwrap();
    assert!(status.integrity_halted);
    // Checkpoint remains the original applied block; conflict wrote no domain rows.
    assert_eq!(status.checkpoint.unwrap().block_hash, pk(0xA0));
    let blocks: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM solana_proof_blocks")
        .fetch_one(store.pool())
        .await
        .unwrap();
    assert_eq!(blocks.0, 1);
}

#[ignore = "requires DATABASE_URL / SOLANA_PROOF_TEST_DATABASE_URL"]
#[tokio::test]
async fn rollback_before_commit_then_successful_replay() {
    let store = fresh_store().await;
    let mut tx = store.pool().begin().await.unwrap();
    sqlx::query(
        r#"
        INSERT INTO solana_proof_blocks (
            slot, block_hash, parent_slot, parent_hash,
            block_time, block_height, executed_transaction_count
        ) VALUES (10, $1, 9, $2, NULL, NULL, 0)
        "#,
    )
    .bind(pk(0xA0).as_slice())
    .bind(pk(0x90).as_slice())
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.rollback().await.unwrap();

    let blocks: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM solana_proof_blocks")
        .fetch_one(store.pool())
        .await
        .unwrap();
    assert_eq!(blocks.0, 0);

    let b = block(10, 9, pk(0x90), pk(0xA0), Vec::new());
    assert_eq!(
        store.apply_completed_block(&b).await.unwrap(),
        ApplyOutcome::Applied
    );
}

#[ignore = "requires DATABASE_URL / SOLANA_PROOF_TEST_DATABASE_URL"]
#[tokio::test]
async fn proof_snapshot_isolation_sees_complete_old_or_new() {
    let store = fresh_store().await;
    let ev = pk(0xE2);
    let owner = pk(0x30);
    let first = block(
        10,
        9,
        pk(0x90),
        pk(0xA0),
        vec![CanonicalTransaction {
            signature: sig(4),
            index: 0,
            succeeded: true,
            is_vote: false,
            instructions: vec![create_ix(ev, pk(0x10), owner)],
        }],
    );
    store.apply_completed_block(&first).await.unwrap();
    let before = store.proof_snapshot(ev).await.unwrap().unwrap();
    assert_eq!(before.leaf_count, 0);
    assert_eq!(before.leaves.len(), 0);

    let second = block(
        11,
        10,
        pk(0xA0),
        pk(0xA1),
        vec![CanonicalTransaction {
            signature: sig(5),
            index: 0,
            succeeded: true,
            is_vote: false,
            instructions: vec![update_ix(ev, pk(0x11), pk(0x10), vec![owner])],
        }],
    );

    // Race a real apply against concurrent snapshots; every view must be
    // consistent (no torn leaf_count vs leaves / peaks).
    let store_apply = SqlProofStore::new(store.pool().clone(), pk(7));
    let apply =
        tokio::spawn(async move { store_apply.apply_completed_block(&second).await.unwrap() });

    let mut saw_old = false;
    let mut saw_new = false;
    for _ in 0..200 {
        let snap = store.proof_snapshot(ev).await.unwrap().unwrap();
        assert_eq!(
            snap.leaf_count as usize,
            snap.leaves.len(),
            "torn snapshot: leaf_count={} leaves={}",
            snap.leaf_count,
            snap.leaves.len()
        );
        assert_eq!(
            snap.peaks,
            mmr_peaks_from_leaves(&snap.leaves),
            "torn snapshot: peaks inconsistent with leaves"
        );
        match snap.leaf_count {
            0 => saw_old = true,
            1 => saw_new = true,
            other => panic!("unexpected leaf_count {other}"),
        }
        if apply.is_finished() && saw_new {
            break;
        }
        tokio::task::yield_now().await;
    }

    assert_eq!(apply.await.unwrap(), ApplyOutcome::Applied);
    let after = store.proof_snapshot(ev).await.unwrap().unwrap();
    assert_eq!(after.leaf_count, 1);
    assert_eq!(after.leaves.len(), 1);
    assert_eq!(after.peaks, mmr_peaks_from_leaves(&after.leaves));
    assert!(
        saw_old || saw_new,
        "race observed at least one consistent snapshot"
    );
    let _ = (saw_old, saw_new);
}

#[ignore = "requires DATABASE_URL / SOLANA_PROOF_TEST_DATABASE_URL"]
#[tokio::test]
async fn parent_slot_mismatch_is_recovery_required_without_domain_writes() {
    let store = fresh_store().await;
    store
        .apply_completed_block(&block(10, 9, pk(0x90), pk(0xA0), Vec::new()))
        .await
        .unwrap();
    let gap = block(12, 11, pk(0xA1), pk(0xA2), Vec::new());
    match store.apply_completed_block(&gap).await.unwrap() {
        ApplyOutcome::RecoveryRequired {
            reason,
            gap_end_slot,
        } => {
            assert!(reason.contains("recovery required"));
            assert!(reason.contains("contiguous ingest gap"));
            assert_eq!(gap_end_slot, 12);
        }
        other => panic!("expected RecoveryRequired, got {other:?}"),
    }
    let status = store.integrity_status().await.unwrap();
    assert!(!status.integrity_halted);
    assert_eq!(status.checkpoint.unwrap().slot, 10);
    let blocks: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM solana_proof_blocks")
        .fetch_one(store.pool())
        .await
        .unwrap();
    assert_eq!(blocks.0, 1);
}

#[ignore = "requires DATABASE_URL / SOLANA_PROOF_TEST_DATABASE_URL"]
#[tokio::test]
async fn parent_hash_mismatch_halts_without_domain_writes() {
    let store = fresh_store().await;
    store
        .apply_completed_block(&block(10, 9, pk(0x90), pk(0xA0), Vec::new()))
        .await
        .unwrap();
    let fork = block(11, 10, pk(0xFF), pk(0xA1), Vec::new());
    match store.apply_completed_block(&fork).await.unwrap() {
        ApplyOutcome::IntegrityHalted { reason } => {
            assert!(reason.contains("ancestry does not extend checkpoint"));
        }
        other => panic!("expected halt, got {other:?}"),
    }
    let status = store.integrity_status().await.unwrap();
    assert!(status.integrity_halted);
    assert_eq!(status.checkpoint.unwrap().slot, 10);
    let blocks: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM solana_proof_blocks")
        .fetch_one(store.pool())
        .await
        .unwrap();
    assert_eq!(blocks.0, 1);
}

#[ignore = "requires DATABASE_URL / SOLANA_PROOF_TEST_DATABASE_URL"]
#[tokio::test]
async fn slot_behind_checkpoint_without_exact_replay_halts() {
    let store = fresh_store().await;
    store
        .apply_completed_block(&block(10, 9, pk(0x90), pk(0xA0), Vec::new()))
        .await
        .unwrap();
    store
        .apply_completed_block(&block(11, 10, pk(0xA0), pk(0xA1), Vec::new()))
        .await
        .unwrap();
    // Slot 9 was never stored; it is behind checkpoint 11.
    let behind = block(9, 8, pk(0x80), pk(0x89), Vec::new());
    match store.apply_completed_block(&behind).await.unwrap() {
        ApplyOutcome::IntegrityHalted { reason } => {
            assert!(reason.contains("at or behind checkpoint"));
        }
        other => panic!("expected halt, got {other:?}"),
    }
    assert!(store.integrity_status().await.unwrap().integrity_halted);
    let blocks: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM solana_proof_blocks")
        .fetch_one(store.pool())
        .await
        .unwrap();
    assert_eq!(blocks.0, 2);
}

#[ignore = "requires DATABASE_URL / SOLANA_PROOF_TEST_DATABASE_URL"]
#[tokio::test]
async fn same_slot_signature_index_conflict_halts() {
    let store = fresh_store().await;
    let first = block(
        10,
        9,
        pk(0x90),
        pk(0xA0),
        vec![CanonicalTransaction {
            signature: sig(0xAA),
            index: 0,
            succeeded: true,
            is_vote: false,
            instructions: Vec::new(),
        }],
    );
    store.apply_completed_block(&first).await.unwrap();
    let conflict = block(
        10,
        9,
        pk(0x90),
        pk(0xA0),
        vec![CanonicalTransaction {
            signature: sig(0xBB),
            index: 0,
            succeeded: true,
            is_vote: false,
            instructions: Vec::new(),
        }],
    );
    match store.apply_completed_block(&conflict).await.unwrap() {
        ApplyOutcome::IntegrityHalted { reason } => {
            assert!(reason.contains("conflicting"));
        }
        other => panic!("expected halt, got {other:?}"),
    }
    assert!(store.integrity_status().await.unwrap().integrity_halted);
}

#[ignore = "requires DATABASE_URL / SOLANA_PROOF_TEST_DATABASE_URL"]
#[tokio::test]
async fn cross_slot_signature_reuse_halts_as_constraint_conflict() {
    let store = fresh_store().await;
    let first = block(
        10,
        9,
        pk(0x90),
        pk(0xA0),
        vec![CanonicalTransaction {
            signature: sig(0xCC),
            index: 0,
            succeeded: true,
            is_vote: false,
            instructions: Vec::new(),
        }],
    );
    store.apply_completed_block(&first).await.unwrap();
    let reuse = block(
        11,
        10,
        pk(0xA0),
        pk(0xA1),
        vec![CanonicalTransaction {
            signature: sig(0xCC),
            index: 0,
            succeeded: true,
            is_vote: false,
            instructions: Vec::new(),
        }],
    );
    match store.apply_completed_block(&reuse).await.unwrap() {
        ApplyOutcome::IntegrityHalted { reason } => {
            assert!(reason.contains("deterministic constraint conflict"));
        }
        other => panic!("expected halt, got {other:?}"),
    }
    assert!(store.integrity_status().await.unwrap().integrity_halted);
    let blocks: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM solana_proof_blocks")
        .fetch_one(store.pool())
        .await
        .unwrap();
    assert_eq!(blocks.0, 1);
}

#[ignore = "requires DATABASE_URL / SOLANA_PROOF_TEST_DATABASE_URL"]
#[tokio::test]
async fn incomplete_bootstrap_and_post_bootstrap_birth() {
    let store = fresh_store().await;
    let empty = block(10, 9, pk(0x90), pk(0xA0), Vec::new());
    store.apply_completed_block(&empty).await.unwrap();
    assert!(!store.integrity_status().await.unwrap().history_complete);

    let ev = pk(0xE3);
    let birth = block(
        11,
        10,
        pk(0xA0),
        pk(0xA1),
        vec![CanonicalTransaction {
            signature: sig(6),
            index: 0,
            succeeded: true,
            is_vote: false,
            instructions: vec![create_ix(ev, pk(0x10), pk(0x30))],
        }],
    );
    assert_eq!(
        store.apply_completed_block(&birth).await.unwrap(),
        ApplyOutcome::Applied
    );
    assert!(store.proof_snapshot(ev).await.unwrap().is_some());
    assert!(!store.integrity_status().await.unwrap().history_complete);

    // Recovery seam can flip the flag later; store records and exposes it.
    store
        .set_history_complete_after_recovery(true)
        .await
        .unwrap();
    assert!(store.integrity_status().await.unwrap().history_complete);
}

#[ignore = "requires DATABASE_URL / SOLANA_PROOF_TEST_DATABASE_URL"]
#[tokio::test]
async fn unknown_pre_bootstrap_lineage_halts() {
    let store = fresh_store().await;
    store
        .apply_completed_block(&block(10, 9, pk(0x90), pk(0xA0), Vec::new()))
        .await
        .unwrap();
    let ev = pk(0xE4);
    let orphan = block(
        11,
        10,
        pk(0xA0),
        pk(0xA1),
        vec![CanonicalTransaction {
            signature: sig(7),
            index: 0,
            succeeded: true,
            is_vote: false,
            instructions: vec![make_public_ix(ev, pk(0x20))],
        }],
    );
    match store.apply_completed_block(&orphan).await.unwrap() {
        ApplyOutcome::IntegrityHalted { reason } => {
            assert!(reason.contains("unknown pre-bootstrap lineage"));
        }
        other => panic!("expected halt, got {other:?}"),
    }
    let lineages: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM solana_proof_lineages")
        .fetch_one(store.pool())
        .await
        .unwrap();
    assert_eq!(lineages.0, 0);
    let leaves: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM solana_proof_leaves")
        .fetch_one(store.pool())
        .await
        .unwrap();
    assert_eq!(leaves.0, 0);
}

#[ignore = "requires DATABASE_URL / SOLANA_PROOF_TEST_DATABASE_URL"]
#[tokio::test]
async fn recovered_gap_fill_applies_contiguous_parents() {
    let store = fresh_store().await;
    // First live block (incomplete bootstrap).
    assert_eq!(
        store
            .apply_completed_block(&block(10, 9, pk(0x90), pk(0xA0), Vec::new()))
            .await
            .unwrap(),
        ApplyOutcome::Applied
    );
    assert!(!store.integrity_status().await.unwrap().history_complete);

    // Contiguous parent gap: next observed would be slot 12 with parent 11.
    let observed = block(12, 11, pk(0xB0), pk(0xC0), Vec::new());
    match store.apply_completed_block(&observed).await.unwrap() {
        ApplyOutcome::RecoveryRequired {
            reason,
            gap_end_slot,
        } => {
            assert!(reason.contains("contiguous ingest gap"));
            assert_eq!(gap_end_slot, 12);
        }
        other => panic!("expected RecoveryRequired, got {other:?}"),
    }

    // Recovered empty intermediate block fills the parent chain.
    assert_eq!(
        store
            .apply_completed_block(&block(11, 10, pk(0xA0), pk(0xB0), Vec::new()))
            .await
            .unwrap(),
        ApplyOutcome::Applied
    );
    assert_eq!(
        store.apply_completed_block(&observed).await.unwrap(),
        ApplyOutcome::Applied
    );
    let status = store.integrity_status().await.unwrap();
    assert_eq!(status.checkpoint.as_ref().map(|c| c.slot), Some(12));
    assert!(!status.history_complete);

    // Seam flips only when caller proves start continuity (Bootstrap A).
    store
        .set_history_complete_after_recovery(true)
        .await
        .unwrap();
    assert!(store.integrity_status().await.unwrap().history_complete);
}

#[ignore = "requires DATABASE_URL / SOLANA_PROOF_TEST_DATABASE_URL"]
#[tokio::test]
async fn conflicting_recovered_ancestry_halts() {
    let store = fresh_store().await;
    store
        .apply_completed_block(&block(10, 9, pk(0x90), pk(0xA0), Vec::new()))
        .await
        .unwrap();

    // Recovered block claims parent slot 10 but wrong parent hash → halt.
    match store
        .apply_completed_block(&block(11, 10, pk(0xFF), pk(0xB0), Vec::new()))
        .await
        .unwrap()
    {
        ApplyOutcome::IntegrityHalted { reason } => {
            assert!(reason.contains("ancestry") || reason.contains("parent hash"));
        }
        other => panic!("expected IntegrityHalted, got {other:?}"),
    }
    assert!(store.integrity_status().await.unwrap().integrity_halted);
    let blocks: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM solana_proof_blocks")
        .fetch_one(store.pool())
        .await
        .unwrap();
    assert_eq!(blocks.0, 1);
}

#[tokio::test]
async fn reduce_unit_leaf_order_matches_instruction_event_subject_order() {
    // Pure reduce path (no DB): two subjects must append in subject order.
    let ev = pk(0xE5);
    let s1 = pk(0x30);
    let s2 = pk(0x31);
    let mut existing = BTreeMap::new();
    existing.insert(
        ev,
        PriorLineageState {
            replay: LineageReplayState {
                current_handle: Some(pk(0x10)),
                subjects: vec![s1, s2],
            },
            leaf_count: 0,
            peaks: Vec::new(),
        },
    );
    let b = block(
        20,
        19,
        pk(0x19),
        pk(0x20),
        vec![CanonicalTransaction {
            signature: sig(8),
            index: 3,
            succeeded: true,
            is_vote: false,
            instructions: vec![update_ix(ev, pk(0x11), pk(0x10), vec![s1, s2])],
        }],
    );
    let staged = reduce_completed_block(pk(7), &b, &existing).unwrap();
    assert_eq!(staged.leaves.len(), 2);
    assert_eq!(staged.leaves[0].leaf_index, 0);
    assert_eq!(staged.leaves[1].leaf_index, 1);
    assert_eq!(staged.leaves[0].transaction_index, 3);
    assert_eq!(staged.leaves[1].transaction_index, 3);
    let _ = DecodedInstruction::MakeHandlePublic {
        encrypted_value: ev,
        handle: pk(0x11),
    };
}
