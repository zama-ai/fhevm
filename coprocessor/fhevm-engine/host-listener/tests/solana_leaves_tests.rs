//! The Solana leaf record against a real Postgres: rows round-trip through the
//! migration, the checkpoint moves, and the leaf-proof route answers from the
//! stored leaves with proofs that verify against the recorded peaks.
#![cfg(feature = "solana-reconstruct")]

use std::collections::BTreeMap;

use host_listener::database::solana_leaves::{
    load_checkpoint, load_encrypted_value_states, load_recorded_leaves,
    reduce_block_leaves, store_block_leaves, store_checkpoint,
    EncryptedValueWrite, LeafSource, StoredCheckpoint, TransactionLeafSources,
};
use host_listener::http_server::{
    HttpServer, LeafProof, LeafProofRequest, LeafProofResponse, LeafQuery,
    LeafQueryKind, LEAF_PROOFS_PATH,
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
    previous_handle: Option<[u8; 32]>,
    handle: [u8; 32],
    allowed_keys: Vec<[u8; 32]>,
    make_public: bool,
) -> LeafSource {
    LeafSource::Write(EncryptedValueWrite {
        encrypted_value_account: ACCOUNT,
        program: [1; 32],
        encrypted_value_account_authority: [2; 32],
        scope: [3; 32],
        label: [4; 32],
        previous_handle,
        handle,
        allowed_keys,
        make_public,
    })
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

    // A reused local database keeps rows between runs.
    sqlx::query("DELETE FROM solana_encrypted_value_leaves")
        .execute(&pool)
        .await?;
    sqlx::query("DELETE FROM solana_encrypted_value_accounts")
        .execute(&pool)
        .await?;
    sqlx::query("DELETE FROM solana_listener_checkpoint")
        .execute(&pool)
        .await?;
    assert_eq!(load_checkpoint(&pool).await?, None);

    // Block 10 creates the value allowing the owner; block 11 replaces the handle,
    // allows the owner again and makes it public.
    let mut tx = pool.begin().await?;
    let existing = load_encrypted_value_states(&mut tx, &[ACCOUNT]).await?;
    assert!(existing.is_empty());
    let first = reduce_block_leaves(
        &[TransactionLeafSources {
            transaction_index: 0,
            sources: vec![write(None, [0x10; 32], vec![OWNER], false)],
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
    let existing = load_encrypted_value_states(&mut tx, &[ACCOUNT]).await?;
    assert_eq!(existing.len(), 1);
    assert_eq!(existing[&ACCOUNT].current_handle, [0x10; 32]);
    let second = reduce_block_leaves(
        &[TransactionLeafSources {
            transaction_index: 2,
            sources: vec![write(
                Some([0x10; 32]),
                [0x11; 32],
                vec![OWNER],
                true,
            )],
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
    let recorded = load_recorded_leaves(&pool, ACCOUNT)
        .await?
        .expect("account recorded");
    assert_eq!(recorded.state, second.accounts[&ACCOUNT]);
    assert_eq!(recorded.leaves.len(), 3);
    assert_eq!(
        recorded.leaves[1..],
        second.leaves[..],
        "leaves persist with their semantics and block position"
    );
    assert_eq!(recorded.leaves[2].transaction_index, 2);
    assert_eq!(load_recorded_leaves(&pool, [0xFF; 32]).await?, None);

    // The HTTP route builds proofs from the same rows.
    let port = {
        let probe = std::net::TcpListener::bind("127.0.0.1:0")?;
        probe.local_addr()?.port()
    };
    let cancel = CancellationToken::new();
    let server = HttpServer::new(
        pool.clone(),
        "secret".to_owned(),
        port,
        cancel.clone(),
    );
    let server_task = tokio::spawn(async move { server.start().await });
    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{port}{LEAF_PROOFS_PATH}");
    let mut response = None;
    for _ in 0..50 {
        match client
            .post(&url)
            .bearer_auth("secret")
            .json(&LeafProofRequest {
                leaves: vec![
                    LeafQuery {
                        encrypted_value_account: hex32(&ACCOUNT),
                        handle: hex32(&[0x11; 32]),
                        kind: LeafQueryKind::Public,
                        key: None,
                    },
                    LeafQuery {
                        encrypted_value_account: hex32(&ACCOUNT),
                        handle: hex32(&[0x10; 32]),
                        kind: LeafQueryKind::Allowed,
                        key: Some(hex32(&OWNER)),
                    },
                    LeafQuery {
                        encrypted_value_account: hex32(&ACCOUNT),
                        handle: hex32(&[0x10; 32]),
                        kind: LeafQueryKind::Public,
                        key: None,
                    },
                    LeafQuery {
                        encrypted_value_account: hex32(&[0xFF; 32]),
                        handle: hex32(&[0x10; 32]),
                        kind: LeafQueryKind::Public,
                        key: None,
                    },
                ],
            })
            .send()
            .await
        {
            Ok(sent) => {
                response = Some(sent);
                break;
            }
            Err(_) => {
                tokio::time::sleep(std::time::Duration::from_millis(50)).await
            }
        }
    }
    let response = response.expect("server came up");
    assert_eq!(response.status(), 200);
    let body: LeafProofResponse = response.json().await?;
    assert_eq!(body.proofs.len(), 4);

    let recorded_peaks = recorded.state.peaks.clone();
    let verify = |proof: &LeafProof, commitment: [u8; 32]| match proof {
        LeafProof::Found {
            leaf_index,
            leaf_count,
            peaks,
            siblings,
        } => {
            assert_eq!(*leaf_count, 3);
            assert_eq!(
                *peaks,
                recorded_peaks.iter().map(hex::encode).collect::<Vec<_>>()
            );
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
    assert_eq!(verify(&body.proofs[0], recorded.leaves[2].commitment), 2);
    assert_eq!(verify(&body.proofs[1], recorded.leaves[0].commitment), 0);
    assert_eq!(body.proofs[2], LeafProof::NotFound { leaf_count: 3 });
    assert_eq!(body.proofs[3], LeafProof::UnknownAccount);

    // An account first seen through an update serves no proof.
    let mut tx = pool.begin().await?;
    let incomplete = reduce_block_leaves(
        &[TransactionLeafSources {
            transaction_index: 0,
            sources: vec![LeafSource::Write(EncryptedValueWrite {
                encrypted_value_account: [0xBB; 32],
                program: [1; 32],
                encrypted_value_account_authority: [2; 32],
                scope: [3; 32],
                label: [4; 32],
                previous_handle: Some([0x20; 32]),
                handle: [0x21; 32],
                allowed_keys: vec![OWNER],
                make_public: false,
            })],
        }],
        BTreeMap::new(),
    )?;
    store_block_leaves(&mut tx, 12, &incomplete).await?;
    tx.commit().await?;
    let response = client
        .post(&url)
        .bearer_auth("secret")
        .json(&LeafProofRequest {
            leaves: vec![LeafQuery {
                encrypted_value_account: hex32(&[0xBB; 32]),
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
