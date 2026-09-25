//! The Solana leaf record against a real Postgres: rows round-trip through the
//! migration, the checkpoint moves, and the leaf-proof route answers from the
//! stored leaves and nodes with proofs that verify against the recorded peaks.
#![cfg(feature = "solana-reconstruct")]

use std::collections::BTreeMap;

use host_listener::database::solana_leaves::{
    load_block_leaves, load_checkpoint, load_encrypted_store_histories,
    load_encrypted_store_history, reduce_block_leaves, store_block_leaves,
    store_checkpoint, EncryptedStoreWrite, StoredCheckpoint,
    TransactionStoreWrites,
};
use host_listener::http_server::{
    ErrorCode, ErrorResponse, HttpServer, LeafProof, LeafProofRequest,
    LeafProofResponse, LeafQuery, LeafQueryKind, LEAF_PROOFS_PATH,
};
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use test_harness::instance::ImportMode;
use tokio_util::sync::CancellationToken;
use zama_solana_acl::{mmr_verify, MmrProof};

const ACCOUNT: [u8; 32] = [0xAC; 32];
const OWNER: [u8; 32] = [0xA1; 32];

fn hex32(bytes: &[u8; 32]) -> String {
    hex::encode(bytes)
}

fn write(
    previous_leaf_count: u64,
    handle: [u8; 32],
    allowed_keys: Vec<[u8; 32]>,
    make_public: bool,
) -> EncryptedStoreWrite {
    EncryptedStoreWrite {
        encrypted_store: ACCOUNT,
        previous_leaf_count,
        handle,
        allowed_keys,
        make_public,
    }
}

/// A reused local database keeps rows between runs.
async fn clear_leaf_record(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    for table in [
        "solana_encrypted_state_nodes",
        "solana_encrypted_state_leaves",
        "solana_encrypted_states",
        "solana_listener_checkpoint",
    ] {
        sqlx::query(&format!("DELETE FROM {table}"))
            .execute(pool)
            .await?;
    }
    Ok(())
}

/// Starts the proof server on a free port and returns its leaf-proof URL once it answers.
async fn serve_proofs(
    pool: &sqlx::PgPool,
    cancel: &CancellationToken,
) -> (String, tokio::task::JoinHandle<anyhow::Result<()>>) {
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .and_then(|probe| probe.local_addr())
        .expect("free port")
        .port();
    let server = HttpServer::leaf_proofs(
        pool.clone(),
        "secret".to_owned(),
        port,
        cancel.clone(),
    );
    let task = tokio::spawn(async move { server.start().await });
    let liveness = format!("http://127.0.0.1:{port}/liveness");
    for _ in 0..50 {
        if reqwest::get(&liveness).await.is_ok() {
            return (
                format!("http://127.0.0.1:{port}{LEAF_PROOFS_PATH}"),
                task,
            );
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    panic!("the proof server did not come up");
}

#[tokio::test]
#[serial(db)]
async fn leaf_record_round_trips_and_serves_verifiable_proofs(
) -> Result<(), Box<dyn std::error::Error>> {
    let db_instance =
        test_harness::instance::setup_test_db(ImportMode::None).await?;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(db_instance.db_url())
        .await?;

    clear_leaf_record(&pool).await?;
    assert_eq!(load_checkpoint(&pool).await?, None);

    // Block 10 creates the value allowing the owner; block 11 replaces the handle,
    // allows the owner again and makes it public.
    let mut tx = pool.begin().await?;
    let existing = load_encrypted_store_histories(&mut tx, &[ACCOUNT]).await?;
    assert!(existing.is_empty());
    let first = reduce_block_leaves(
        10,
        &[TransactionStoreWrites {
            transaction_index: 0,
            sources: vec![write(0, [0x10; 32], vec![OWNER], false)],
        }],
        existing,
    )?;
    store_block_leaves(&mut tx, 10, &first).await?;
    store_checkpoint(
        &mut tx,
        &StoredCheckpoint {
            slot: 10,
            block_hash: [0x1A; 32],
        },
    )
    .await?;
    tx.commit().await?;

    let mut tx = pool.begin().await?;
    let existing = load_encrypted_store_histories(&mut tx, &[ACCOUNT]).await?;
    assert_eq!(existing.len(), 1);
    assert_eq!(existing[&ACCOUNT].leaf_count, 1);
    let second = reduce_block_leaves(
        11,
        &[TransactionStoreWrites {
            transaction_index: 2,
            sources: vec![write(1, [0x11; 32], vec![OWNER], true)],
        }],
        existing,
    )?;
    store_block_leaves(&mut tx, 11, &second).await?;
    store_checkpoint(
        &mut tx,
        &StoredCheckpoint {
            slot: 11,
            block_hash: [0x1B; 32],
        },
    )
    .await?;
    tx.commit().await?;

    assert_eq!(
        load_checkpoint(&pool).await?,
        Some(StoredCheckpoint {
            slot: 11,
            block_hash: [0x1B; 32],
        })
    );
    let state = load_encrypted_store_history(&pool, ACCOUNT)
        .await?
        .expect("account recorded");
    assert_eq!(state, second.states[&ACCOUNT]);
    let mut tx = pool.begin().await?;
    assert_eq!(
        load_block_leaves(&mut tx, 11, [ACCOUNT]).await?,
        BTreeMap::from([(ACCOUNT, second.leaves.clone())]),
        "leaves persist with their semantics and block position"
    );
    tx.rollback().await?;
    assert_eq!(load_encrypted_store_history(&pool, [0xFF; 32]).await?, None);

    // A repair replays recorded slots: the recomputed leaves equal the recorded ones
    // and nothing is appended. A replay that computes other leaves does not match.
    let replay_of_11 = |handle| {
        [TransactionStoreWrites {
            transaction_index: 2,
            sources: vec![write(1, handle, vec![OWNER], true)],
        }]
    };
    let mut tx = pool.begin().await?;
    let existing = load_encrypted_store_histories(&mut tx, &[ACCOUNT]).await?;
    let replay =
        reduce_block_leaves(11, &replay_of_11([0x11; 32]), existing.clone())?;
    assert!(replay.states.is_empty() && replay.leaves.is_empty());
    assert_eq!(replay.replayed[&ACCOUNT].len(), 2);
    assert_eq!(
        load_block_leaves(&mut tx, 11, replay.replayed.keys().copied()).await?,
        replay.replayed
    );
    let forged = reduce_block_leaves(11, &replay_of_11([0x12; 32]), existing)?;
    assert_ne!(
        load_block_leaves(&mut tx, 11, forged.replayed.keys().copied()).await?,
        forged.replayed
    );
    tx.rollback().await?;

    // The HTTP route builds proofs from the same rows.
    let cancel = CancellationToken::new();
    let (url, server_task) = serve_proofs(&pool, &cancel).await;
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .bearer_auth("secret")
        .json(&LeafProofRequest {
            leaves: vec![
                LeafQuery {
                    encrypted_store: hex32(&ACCOUNT),
                    handle: hex32(&[0x11; 32]),
                    kind: LeafQueryKind::Public,
                    key: None,
                },
                LeafQuery {
                    encrypted_store: hex32(&ACCOUNT),
                    handle: hex32(&[0x10; 32]),
                    kind: LeafQueryKind::Allowed,
                    key: Some(hex32(&OWNER)),
                },
                LeafQuery {
                    encrypted_store: hex32(&ACCOUNT),
                    handle: hex32(&[0x10; 32]),
                    kind: LeafQueryKind::Public,
                    key: None,
                },
                LeafQuery {
                    encrypted_store: hex32(&[0xFF; 32]),
                    handle: hex32(&[0x10; 32]),
                    kind: LeafQueryKind::Public,
                    key: None,
                },
            ],
        })
        .send()
        .await?;
    assert_eq!(response.status(), 200);
    let body: LeafProofResponse = response.json().await?;
    assert_eq!(body.proofs.len(), 4);

    let recorded_peaks = state.peaks.clone();
    let verify = |proof: &LeafProof, commitment: [u8; 32]| match proof {
        LeafProof::Found {
            leaf_index,
            leaf_count,
            siblings,
        } => {
            assert_eq!(*leaf_count, 3);
            let siblings = siblings
                .iter()
                .map(|sibling| {
                    <[u8; 32]>::try_from(hex::decode(sibling).unwrap()).unwrap()
                })
                .collect();
            assert!(mmr_verify(
                &recorded_peaks,
                3,
                commitment,
                &MmrProof {
                    leaf_index: *leaf_index,
                    siblings
                },
            ));
            *leaf_index
        }
        other => panic!("expected a proof, got {other:?}"),
    };
    assert_eq!(verify(&body.proofs[0], second.leaves[1].commitment), 2);
    assert_eq!(verify(&body.proofs[1], first.leaves[0].commitment), 0);
    assert_eq!(body.proofs[2], LeafProof::NotFound { leaf_count: 3 });
    assert_eq!(body.proofs[3], LeafProof::UnknownAccount);

    // A record missing a path row, or holding a wrong one, cannot serve a proof: the
    // route answers a retryable 502 rather than a path that misses the peaks. Leaf 0's
    // path is leaf 1.
    let owner_of_0x10 = LeafProofRequest {
        leaves: vec![LeafQuery {
            encrypted_store: hex32(&ACCOUNT),
            handle: hex32(&[0x10; 32]),
            kind: LeafQueryKind::Allowed,
            key: Some(hex32(&OWNER)),
        }],
    };
    for corruption in [
        "UPDATE solana_encrypted_state_leaves SET commitment = decode(repeat('00', 32), 'hex') WHERE leaf_index = 1",
        "DELETE FROM solana_encrypted_state_leaves WHERE leaf_index = 1",
    ] {
        sqlx::query(corruption).execute(&pool).await?;
        let response = client
            .post(&url)
            .bearer_auth("secret")
            .json(&owner_of_0x10)
            .send()
            .await?;
        assert_eq!(response.status(), 502, "{corruption}");
        let error: ErrorResponse = response.json().await?;
        assert_eq!(error.code, ErrorCode::UpstreamTransient);
        assert!(error.retryable);
    }

    // An account first seen through an update serves no proof.
    let mut tx = pool.begin().await?;
    let incomplete = reduce_block_leaves(
        12,
        &[TransactionStoreWrites {
            transaction_index: 0,
            sources: vec![EncryptedStoreWrite {
                encrypted_store: [0xBB; 32],
                previous_leaf_count: 7,
                handle: [0x21; 32],
                allowed_keys: vec![OWNER],
                make_public: false,
            }],
        }],
        BTreeMap::new(),
    )?;
    store_block_leaves(&mut tx, 12, &incomplete).await?;
    tx.commit().await?;
    let recorded_incomplete = load_encrypted_store_history(&pool, [0xBB; 32])
        .await?
        .expect("incomplete cursor recorded");
    assert_eq!(recorded_incomplete.leaf_count, 8);
    assert!(recorded_incomplete.peaks.is_empty());
    assert!(incomplete.leaves.is_empty() && incomplete.nodes.is_empty());
    let response = client
        .post(&url)
        .bearer_auth("secret")
        .json(&LeafProofRequest {
            leaves: vec![LeafQuery {
                encrypted_store: hex32(&[0xBB; 32]),
                handle: hex32(&[0x21; 32]),
                kind: LeafQueryKind::Allowed,
                key: Some(hex32(&OWNER)),
            }],
        })
        .send()
        .await?;
    let body: LeafProofResponse = response.json().await?;
    assert_eq!(body.proofs, vec![LeafProof::HistoryIncomplete]);

    cancel.cancel();
    server_task.await??;
    Ok(())
}

/// Stores `leaves` allowed keys on [`ACCOUNT`] through the ingest path, requests the proofs
/// of `proved` in one call and checks each against the stored peaks. Returns how long the
/// request took. It first deletes a leaf row that no requested path contains, so a route that
/// reads more than each path fails.
async fn prove_leaves_of_a_store(
    leaves: u64,
    proved: [u64; 8],
) -> Result<std::time::Duration, Box<dyn std::error::Error>> {
    const LEAVES_PER_BLOCK: u64 = 50_000;
    let key = |leaf_index: u64| {
        let mut key = [0u8; 32];
        key[..8].copy_from_slice(&leaf_index.to_be_bytes());
        key
    };
    let handle = |leaf_index: u64| [(leaf_index / LEAVES_PER_BLOCK) as u8; 32];

    let db_instance =
        test_harness::instance::setup_test_db(ImportMode::None).await?;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(db_instance.db_url())
        .await?;
    clear_leaf_record(&pool).await?;
    let mut states = BTreeMap::new();
    for first in (0..leaves).step_by(LEAVES_PER_BLOCK as usize) {
        let slot = first / LEAVES_PER_BLOCK;
        let block = reduce_block_leaves(
            slot,
            &[TransactionStoreWrites {
                transaction_index: 0,
                sources: vec![write(
                    first,
                    handle(first),
                    (first..leaves.min(first + LEAVES_PER_BLOCK))
                        .map(key)
                        .collect(),
                    false,
                )],
            }],
            states,
        )?;
        let mut tx = pool.begin().await?;
        store_block_leaves(&mut tx, slot, &block).await?;
        tx.commit().await?;
        states = block.states;
    }
    let state = &states[&ACCOUNT];
    assert_eq!(state.leaf_count, leaves);
    let off_path = (0..leaves)
        .find(|leaf| proved.iter().all(|&p| *leaf != p && *leaf != p ^ 1))
        .expect("a leaf off every requested path");
    let deleted = sqlx::query(
        "DELETE FROM solana_encrypted_state_leaves
         WHERE encrypted_state = $1 AND leaf_index = $2",
    )
    .bind(&ACCOUNT[..])
    .bind(off_path as i64)
    .execute(&pool)
    .await?;
    assert_eq!(deleted.rows_affected(), 1);

    let cancel = CancellationToken::new();
    let (url, server_task) = serve_proofs(&pool, &cancel).await;
    let started = std::time::Instant::now();
    let response = reqwest::Client::new()
        .post(&url)
        .bearer_auth("secret")
        .json(&LeafProofRequest {
            leaves: proved
                .iter()
                .map(|&leaf_index| LeafQuery {
                    encrypted_store: hex32(&ACCOUNT),
                    handle: hex32(&handle(leaf_index)),
                    kind: LeafQueryKind::Allowed,
                    key: Some(hex32(&key(leaf_index))),
                })
                .collect(),
        })
        .send()
        .await?;
    assert_eq!(response.status(), 200);
    let body: LeafProofResponse = response.json().await?;
    let elapsed = started.elapsed();

    for (&leaf_index, proof) in proved.iter().zip(&body.proofs) {
        let LeafProof::Found {
            leaf_index: found,
            leaf_count,
            siblings,
        } = proof
        else {
            panic!("leaf {leaf_index}: {proof:?}");
        };
        assert_eq!((*found, *leaf_count), (leaf_index, leaves));
        let commitment = zama_solana_acl::historical_access_leaf_commitment(
            ACCOUNT,
            leaf_index,
            handle(leaf_index),
            key(leaf_index),
        );
        let siblings = siblings
            .iter()
            .map(|sibling| {
                <[u8; 32]>::try_from(hex::decode(sibling).unwrap()).unwrap()
            })
            .collect();
        assert!(mmr_verify(
            &state.peaks,
            leaves,
            commitment,
            &MmrProof {
                leaf_index,
                siblings
            }
        ));
    }

    cancel.cancel();
    server_task.await??;
    Ok(elapsed)
}

/// Paths through every mountain of 1,000 = 2^9 + 2^8 + 2^7 + 2^6 + 2^5 + 2^3 leaves, across
/// two ingested blocks.
#[tokio::test]
#[serial(db)]
async fn proofs_read_their_path_by_position(
) -> Result<(), Box<dyn std::error::Error>> {
    prove_leaves_of_a_store(1_000, [0, 1, 511, 512, 767, 800, 991, 999])
        .await?;
    Ok(())
}

/// fhevm-internal#2104: rebuilding a path from every leaf took 30 to 40 seconds for 8
/// entries of a 1,000,000-leaf store, where the KMS connector waits 10 seconds. Storing the
/// store takes about a minute in a debug build, so this runs on request:
/// `cargo test -p host-listener --features solana-reconstruct --test solana_leaves_tests --
/// --ignored --nocapture`.
#[tokio::test]
#[serial(db)]
#[ignore = "stores 1,000,000 leaves; run on request"]
async fn eight_proofs_of_a_million_leaf_store_answer_well_within_the_connector_timeout(
) -> Result<(), Box<dyn std::error::Error>> {
    // 1,000,000 = 2^19 + 2^18 + 2^17 + 2^16 + 2^14 + 2^9 + 2^6.
    let elapsed = prove_leaves_of_a_store(
        1_000_000,
        [0, 1, 524_287, 524_288, 786_431, 917_600, 999_990, 999_999],
    )
    .await?;
    eprintln!("8 proofs of a 1,000,000-leaf store answered in {elapsed:?}");
    assert!(elapsed < std::time::Duration::from_secs(2), "{elapsed:?}");
    Ok(())
}
