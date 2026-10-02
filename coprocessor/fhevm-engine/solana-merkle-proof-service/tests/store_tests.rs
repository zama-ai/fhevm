//! The leaf record against a real Postgres: rows round-trip through the migration, the
//! checkpoint moves, and the Merkle proof route answers from the stored leaves and nodes with
//! proofs that verify against the recorded peaks.

mod support;

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use alloy::signers::local::PrivateKeySigner;
use request_authorization::KeyRegistry;

use serial_test::serial;
use solana_host_follower::host::EncryptedStoreWrite;
use solana_host_follower::BlockCheckpoint;
use solana_merkle_proof_service::kms_tx_senders::{
    KmsTxSenderSet, KmsTxSenders,
};
use solana_merkle_proof_service::server::{
    ErrorCode, ErrorResponse, HttpServer, LeafQuery, LeafQueryKind,
    MerkleProofOutcome, MerkleProofRequest, MerkleProofResponse,
    MERKLE_PROOFS_PATH,
};
use solana_merkle_proof_service::store::{
    load_block_leaves, load_checkpoint, load_served_store, load_store_cursors,
    reduce_block_leaves, store_block_leaves, store_checkpoint,
    TransactionStoreWrites,
};
use solana_merkle_proof_service::store_check::{check_stores, StoreAccounts};
use solana_sdk::{account::Account, pubkey::Pubkey};
use tokio_util::sync::CancellationToken;
use zama_solana_acl::{
    encrypted_store_discriminator, mmr_verify, EncryptedStore, MmrProof,
};

const ACCOUNT: [u8; 32] = [0xAC; 32];
const OWNER: [u8; 32] = [0xA1; 32];

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

const REGISTRY: KeyRegistry = KeyRegistry {
    chain_id: 12345,
    contract: alloy::primitives::Address::repeat_byte(0xC0),
};

/// Posts Merkle proof requests signed by a KMS tx-sender the server accepts.
struct ProofClient {
    url: String,
    client: reqwest::Client,
    connector: PrivateKeySigner,
}

impl ProofClient {
    async fn post(
        &self,
        request: &MerkleProofRequest,
    ) -> reqwest::Result<reqwest::Response> {
        let (body, authorization) = self.sign(request).await;
        self.send(body, &authorization).await
    }

    /// The body of `request` and its authorization header, valid for 30 seconds as the connector signs.
    async fn sign(&self, request: &MerkleProofRequest) -> (Vec<u8>, String) {
        let mut body = Vec::new();
        ciborium::into_writer(request, &mut body).expect("encode");
        let expires = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_secs()
            + 30;
        let authorization = request_authorization::authorize(
            &self.connector,
            &REGISTRY,
            MERKLE_PROOFS_PATH,
            &body,
            expires,
        )
        .await
        .expect("sign");
        (body, authorization)
    }

    async fn send(
        &self,
        body: Vec<u8>,
        authorization: &str,
    ) -> reqwest::Result<reqwest::Response> {
        self.client
            .post(&self.url)
            .header(reqwest::header::AUTHORIZATION, authorization)
            .body(body)
            .send()
            .await
    }
}

async fn decode<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> anyhow::Result<T> {
    Ok(ciborium::from_reader(&response.bytes().await?[..])?)
}

/// A rate no test reaches.
const UNLIMITED: u32 = 100_000;

/// Starts the proof server on a free port, allowing its KMS tx-sender
/// `leaves_per_second`, and returns a client once it answers.
async fn serve_proofs(
    pool: &sqlx::PgPool,
    leaves_per_second: u32,
    cancel: &CancellationToken,
) -> (ProofClient, tokio::task::JoinHandle<anyhow::Result<()>>) {
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .and_then(|probe| probe.local_addr())
        .expect("free port")
        .port();
    let connector = PrivateKeySigner::random();
    let server = HttpServer::merkle_proofs(
        pool.clone(),
        KmsTxSenders::fixed(KmsTxSenderSet {
            registry: REGISTRY,
            senders: HashSet::from([connector.address()]),
        }),
        std::num::NonZeroU32::new(leaves_per_second).expect("a rate"),
        1 << 20,
        port,
        cancel.clone(),
    );
    let task = tokio::spawn(async move { server.start().await });
    let liveness = format!("http://127.0.0.1:{port}/liveness");
    for _ in 0..50 {
        if reqwest::get(&liveness).await.is_ok() {
            let proofs = ProofClient {
                url: format!("http://127.0.0.1:{port}{MERKLE_PROOFS_PATH}"),
                client: reqwest::Client::builder()
                    .http2_prior_knowledge()
                    .build()
                    .expect("client"),
                connector,
            };
            return (proofs, task);
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    panic!("the proof server did not come up");
}

#[tokio::test]
#[serial(db)]
async fn leaf_record_round_trips_and_serves_verifiable_proofs(
) -> Result<(), Box<dyn std::error::Error>> {
    let (_db, pool) = support::record_db().await;
    assert_eq!(load_checkpoint(&pool).await?, None);

    // Block 10 creates the value allowing the owner; block 11 replaces the handle,
    // allows the owner again and makes it public.
    let mut tx = pool.begin().await?;
    let existing = load_store_cursors(&mut tx, &[ACCOUNT]).await?;
    assert!(existing.is_empty());
    let first = reduce_block_leaves(
        &[TransactionStoreWrites {
            transaction_index: 0,
            sources: vec![write(0, [0x10; 32], vec![OWNER], false)],
        }],
        existing,
    )?;
    store_block_leaves(&mut tx, 10, &first).await?;
    store_checkpoint(
        &mut tx,
        &BlockCheckpoint {
            slot: 10,
            block_hash: [0x1A; 32],
        },
    )
    .await?;
    tx.commit().await?;

    let mut tx = pool.begin().await?;
    let existing = load_store_cursors(&mut tx, &[ACCOUNT]).await?;
    assert_eq!(existing.len(), 1);
    assert_eq!(existing[&ACCOUNT].leaf_count, 1);
    let second = reduce_block_leaves(
        &[TransactionStoreWrites {
            transaction_index: 2,
            sources: vec![write(1, [0x11; 32], vec![OWNER], true)],
        }],
        existing,
    )?;
    store_block_leaves(&mut tx, 11, &second).await?;
    store_checkpoint(
        &mut tx,
        &BlockCheckpoint {
            slot: 11,
            block_hash: [0x1B; 32],
        },
    )
    .await?;
    tx.commit().await?;

    assert_eq!(
        load_checkpoint(&pool).await?,
        Some(BlockCheckpoint {
            slot: 11,
            block_hash: [0x1B; 32],
        })
    );
    let served = load_served_store(&pool, ACCOUNT)
        .await?
        .expect("account recorded");
    assert_eq!(served.cursor, second.stores[&ACCOUNT]);
    assert!(!served.quarantined);
    let mut tx = pool.begin().await?;
    assert_eq!(
        load_block_leaves(&mut tx, 11).await?,
        BTreeMap::from([(ACCOUNT, second.leaves.clone())]),
        "leaves persist with their semantics and block position"
    );
    tx.rollback().await?;
    assert_eq!(load_served_store(&pool, [0xFF; 32]).await?, None);

    // The HTTP route builds proofs from the same rows.
    let cancel = CancellationToken::new();
    let (proofs, server_task) = serve_proofs(&pool, UNLIMITED, &cancel).await;
    let response = proofs
        .post(&MerkleProofRequest {
            leaves: vec![
                LeafQuery {
                    encrypted_store: ACCOUNT,
                    handle: [0x11; 32],
                    kind: LeafQueryKind::Public,
                    key: None,
                },
                LeafQuery {
                    encrypted_store: ACCOUNT,
                    handle: [0x10; 32],
                    kind: LeafQueryKind::Allowed,
                    key: Some(OWNER),
                },
                LeafQuery {
                    encrypted_store: ACCOUNT,
                    handle: [0x10; 32],
                    kind: LeafQueryKind::Public,
                    key: None,
                },
                LeafQuery {
                    encrypted_store: [0xFF; 32],
                    handle: [0x10; 32],
                    kind: LeafQueryKind::Public,
                    key: None,
                },
            ],
        })
        .await?;
    assert_eq!(response.status(), 200);
    let body: MerkleProofResponse = decode(response).await?;
    assert_eq!(body.proofs.len(), 4);

    let recorded_peaks = served.cursor.peaks.clone();
    let verify = |proof: &MerkleProofOutcome, commitment: [u8; 32]| match proof
    {
        MerkleProofOutcome::Found {
            leaf_index,
            leaf_count,
            siblings,
        } => {
            assert_eq!(*leaf_count, 3);
            let siblings = siblings.iter().map(|sibling| **sibling).collect();
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
    assert_eq!(
        body.proofs[2],
        MerkleProofOutcome::NotFound { leaf_count: 3 }
    );
    assert_eq!(body.proofs[3], MerkleProofOutcome::UnknownAccount);

    // A leaf at or past the store's `leaf_count`, as a block committed after the route
    // read the store row leaves it, is not proved against that count.
    sqlx::query(
        "INSERT INTO leaves
             (encrypted_store, leaf_index, commitment, leaf_kind, handle, allowed_key,
              block_slot, transaction_index)
         SELECT encrypted_store, 3, commitment, leaf_kind, decode(repeat('13', 32), 'hex'),
                allowed_key, 12, 0
         FROM leaves WHERE leaf_index = 0",
    )
    .execute(&pool)
    .await?;
    let response = proofs
        .post(&MerkleProofRequest {
            leaves: vec![LeafQuery {
                encrypted_store: ACCOUNT,
                handle: [0x13; 32],
                kind: LeafQueryKind::Allowed,
                key: Some(OWNER),
            }],
        })
        .await?;
    assert_eq!(response.status(), 200);
    let body: MerkleProofResponse = decode(response).await?;
    assert_eq!(
        body.proofs,
        vec![MerkleProofOutcome::NotFound { leaf_count: 3 }]
    );

    // A record missing a path row, or holding a wrong one, cannot serve a proof: the
    // route answers a retryable 502 rather than a path that misses the peaks. Leaf 0's
    // path is leaf 1.
    let owner_of_0x10 = MerkleProofRequest {
        leaves: vec![LeafQuery {
            encrypted_store: ACCOUNT,
            handle: [0x10; 32],
            kind: LeafQueryKind::Allowed,
            key: Some(OWNER),
        }],
    };
    for corruption in [
        "UPDATE leaves SET commitment = decode(repeat('00', 32), 'hex') WHERE leaf_index = 1",
        "DELETE FROM leaves WHERE leaf_index = 1",
    ] {
        sqlx::query(corruption).execute(&pool).await?;
        let response = proofs.post(&owner_of_0x10).await?;
        assert_eq!(response.status(), 502, "{corruption}");
        let error: ErrorResponse = decode(response).await?;
        assert_eq!(error.code, ErrorCode::UpstreamTransient);
        assert!(error.retryable);
    }

    cancel.cancel();
    server_task.await??;
    Ok(())
}

/// Stores `leaves` allowed keys on [`ACCOUNT`] through the ingest path, `leaves_per_block` per
/// block, requests the proofs of `proved` in one call and checks each against the stored peaks. Returns how long the
/// request took. It first deletes a leaf row that no requested path contains, so a route that
/// reads more than each path fails.
async fn prove_leaves_of_a_store(
    leaves: u64,
    leaves_per_block: u64,
    proved: [u64; 8],
) -> Result<std::time::Duration, Box<dyn std::error::Error>> {
    let key = |leaf_index: u64| {
        let mut key = [0u8; 32];
        key[..8].copy_from_slice(&leaf_index.to_be_bytes());
        key
    };
    let handle = |leaf_index: u64| [(leaf_index / leaves_per_block) as u8; 32];

    let (_db, pool) = support::record_db().await;
    let mut stores = BTreeMap::new();
    for first in (0..leaves).step_by(leaves_per_block as usize) {
        let slot = first / leaves_per_block;
        let block = reduce_block_leaves(
            &[TransactionStoreWrites {
                transaction_index: 0,
                sources: vec![write(
                    first,
                    handle(first),
                    (first..leaves.min(first + leaves_per_block))
                        .map(key)
                        .collect(),
                    false,
                )],
            }],
            stores,
        )?;
        let mut tx = pool.begin().await?;
        store_block_leaves(&mut tx, slot, &block).await?;
        tx.commit().await?;
        stores = block.stores;
    }
    let state = &stores[&ACCOUNT];
    assert_eq!(state.leaf_count, leaves);
    let off_path = (0..leaves)
        .find(|leaf| proved.iter().all(|&p| *leaf != p && *leaf != p ^ 1))
        .expect("a leaf off every requested path");
    let deleted = sqlx::query(
        "DELETE FROM leaves WHERE encrypted_store = $1 AND leaf_index = $2",
    )
    .bind(&ACCOUNT[..])
    .bind(off_path as i64)
    .execute(&pool)
    .await?;
    assert_eq!(deleted.rows_affected(), 1);

    let cancel = CancellationToken::new();
    let (proofs, server_task) = serve_proofs(&pool, UNLIMITED, &cancel).await;
    let started = std::time::Instant::now();
    let response = proofs
        .post(&MerkleProofRequest {
            leaves: proved
                .iter()
                .map(|&leaf_index| LeafQuery {
                    encrypted_store: ACCOUNT,
                    handle: handle(leaf_index),
                    kind: LeafQueryKind::Allowed,
                    key: Some(key(leaf_index)),
                })
                .collect(),
        })
        .await?;
    assert_eq!(response.status(), 200);
    let body: MerkleProofResponse = decode(response).await?;
    let elapsed = started.elapsed();

    for (&leaf_index, proof) in proved.iter().zip(&body.proofs) {
        let MerkleProofOutcome::Found {
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
        let siblings = siblings.iter().map(|sibling| **sibling).collect();
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

/// Paths through every mountain of 1,000 = 2^9 + 2^8 + 2^7 + 2^6 + 2^5 + 2^3 leaves, ingested
/// 300 per block so nodes span blocks.
#[tokio::test]
#[serial(db)]
async fn proofs_read_their_path_by_position(
) -> Result<(), Box<dyn std::error::Error>> {
    prove_leaves_of_a_store(1_000, 300, [0, 511, 512, 767, 800, 900, 991, 999])
        .await?;
    Ok(())
}

/// fhevm-internal#2104: rebuilding a path from every leaf took 30 to 40 seconds for 8
/// entries of a 1,000,000-leaf store, where the KMS connector waits 10 seconds. Storing the
/// store takes about a minute in a debug build, so this runs on request:
/// `cargo test -p solana-merkle-proof-service --test store_tests -- --ignored --nocapture`.
#[tokio::test]
#[serial(db)]
#[ignore = "stores 1,000,000 leaves; run on request"]
async fn eight_proofs_of_a_million_leaf_store_answer_well_within_the_connector_timeout(
) -> Result<(), Box<dyn std::error::Error>> {
    // 1,000,000 = 2^19 + 2^18 + 2^17 + 2^16 + 2^14 + 2^9 + 2^6.
    let elapsed = prove_leaves_of_a_store(
        1_000_000,
        50_000,
        [0, 1, 524_287, 524_288, 786_431, 917_600, 999_990, 999_999],
    )
    .await?;
    eprintln!("8 proofs of a 1,000,000-leaf store answered in {elapsed:?}");
    assert!(elapsed < std::time::Duration::from_secs(2), "{elapsed:?}");
    Ok(())
}

/// A coprocessor that received a signed request can resend it to the others while it is valid.
/// A server answers such a repeat as it answered the request, without charging the signer
/// again: here the first request spends the signer's whole burst, its repeat still gets the
/// same bytes, and only a new request is refused.
#[tokio::test]
#[serial(db)]
async fn a_repeated_request_gets_its_first_answer_at_no_cost(
) -> Result<(), Box<dyn std::error::Error>> {
    let (_db, pool) = support::record_db().await;
    let mut tx = pool.begin().await?;
    let block = reduce_block_leaves(
        &[TransactionStoreWrites {
            transaction_index: 0,
            sources: vec![write(0, [0x10; 32], vec![OWNER], false)],
        }],
        load_store_cursors(&mut tx, &[ACCOUNT]).await?,
    )?;
    store_block_leaves(&mut tx, 10, &block).await?;
    tx.commit().await?;

    let cancel = CancellationToken::new();
    // One leaf per second, so the burst is the 64 leaves of the largest request.
    let (proofs, server_task) = serve_proofs(&pool, 1, &cancel).await;
    let full_request = |handle| MerkleProofRequest {
        leaves: vec![
            LeafQuery {
                encrypted_store: ACCOUNT,
                handle,
                kind: LeafQueryKind::Allowed,
                key: Some(OWNER),
            };
            64
        ],
    };
    let (body, authorization) = proofs.sign(&full_request([0x10; 32])).await;
    let first = proofs.send(body.clone(), &authorization).await?;
    assert_eq!(first.status(), 200);
    let first = first.bytes().await?;
    let repeat = proofs.send(body, &authorization).await?;
    assert_eq!(repeat.status(), 200);
    assert_eq!(repeat.bytes().await?, first);

    let other = proofs.post(&full_request([0x11; 32])).await?;
    assert_eq!(other.status(), 429);
    let error: ErrorResponse = decode(other).await?;
    assert_eq!(error.code, ErrorCode::RateLimited);

    cancel.cancel();
    server_task.await??;
    Ok(())
}

/// The accounts a test sets, as `getMultipleAccounts` would return them.
#[derive(Default)]
struct Chain(Mutex<HashMap<[u8; 32], Account>>);

impl Chain {
    fn set(&self, address: [u8; 32], store: Option<&EncryptedStore>) {
        let mut accounts = self.0.lock().unwrap();
        let Some(store) = store else {
            accounts.remove(&address);
            return;
        };
        let mut data = encrypted_store_discriminator().to_vec();
        borsh::to_writer(&mut data, store).expect("encode");
        accounts.insert(
            address,
            Account {
                lamports: 1,
                data,
                owner: Pubkey::new_from_array(HOST_PROGRAM),
                executable: false,
                rent_epoch: 0,
            },
        );
    }
}

impl StoreAccounts for Chain {
    async fn accounts(
        &self,
        stores: &[[u8; 32]],
    ) -> anyhow::Result<Vec<Option<Account>>> {
        let accounts = self.0.lock().unwrap();
        Ok(stores
            .iter()
            .map(|store| accounts.get(store).cloned())
            .collect())
    }
}

const HOST_PROGRAM: [u8; 32] = [0x40; 32];

/// The store check quarantines a store whose recorded peaks differ from the chain's, the proof
/// server then refuses its proofs as inconsistent, and a later check that matches releases it.
/// A record behind the chain, or a closed store, is left serving.
#[tokio::test]
#[serial(db)]
async fn a_store_that_disagrees_with_the_chain_is_quarantined_until_it_matches(
) -> Result<(), Box<dyn std::error::Error>> {
    let (_db, pool) = support::record_db().await;
    let mut chain_store = EncryptedStore {
        program: [1; 32],
        authority: [2; 32],
        scope: [3; 32],
        ..EncryptedStore::default()
    };
    let (address, bump) = Pubkey::find_program_address(
        &chain_store.seeds(),
        &Pubkey::new_from_array(HOST_PROGRAM),
    );
    let address = address.to_bytes();
    chain_store.bump = bump;
    let mut tx = pool.begin().await?;
    let block = reduce_block_leaves(
        &[TransactionStoreWrites {
            transaction_index: 0,
            sources: vec![EncryptedStoreWrite {
                encrypted_store: address,
                previous_leaf_count: 0,
                handle: [0x10; 32],
                allowed_keys: vec![OWNER],
                make_public: true,
            }],
        }],
        BTreeMap::new(),
    )?;
    store_block_leaves(&mut tx, 10, &block).await?;
    tx.commit().await?;
    let recorded = &block.stores[&address];
    assert_eq!(recorded.leaf_count, 2);
    chain_store.leaf_count = recorded.leaf_count;
    chain_store.peaks = recorded.peaks.clone();

    let chain = Chain::default();
    let host_program = Pubkey::new_from_array(HOST_PROGRAM);
    let check = || check_stores(&pool, &chain, &host_program, "1");
    let quarantined = || async {
        load_served_store(&pool, address)
            .await
            .map(|store| store.expect("recorded").quarantined)
    };
    let cancel = CancellationToken::new();
    let (proofs, server_task) = serve_proofs(&pool, UNLIMITED, &cancel).await;
    // The public leaf asked `copies` times: each request has its own body, so none is a copy
    // of an earlier one that the server answers from its cache.
    let public_leaf = |copies| MerkleProofRequest {
        leaves: vec![
            LeafQuery {
                encrypted_store: address,
                handle: [0x10; 32],
                kind: LeafQueryKind::Public,
                key: None,
            };
            copies
        ],
    };

    chain.set(address, Some(&chain_store));
    check().await?;
    assert!(!quarantined().await?);
    assert_eq!(proofs.post(&public_leaf(1)).await?.status(), 200);

    let mut diverged = chain_store.clone();
    diverged.peaks[0][0] ^= 1;
    chain.set(address, Some(&diverged));
    check().await?;
    assert!(quarantined().await?);
    let refused = proofs.post(&public_leaf(2)).await?;
    assert_eq!(refused.status(), 502);
    let error: ErrorResponse = decode(refused).await?;
    assert_eq!(
        (error.code, error.message.as_str()),
        (ErrorCode::UpstreamTransient, "leaf record inconsistent")
    );

    // Behind (a third leaf on chain) and closed leave the quarantine as it is.
    let mut ahead = chain_store.clone();
    ahead.leaf_count = 3;
    ahead.peaks.push([0x33; 32]);
    chain.set(address, Some(&ahead));
    check().await?;
    assert!(quarantined().await?);
    chain.set(address, None);
    check().await?;
    assert!(quarantined().await?);

    chain.set(address, Some(&chain_store));
    check().await?;
    assert!(!quarantined().await?);
    assert_eq!(proofs.post(&public_leaf(3)).await?.status(), 200);

    cancel.cancel();
    server_task.await??;
    Ok(())
}
