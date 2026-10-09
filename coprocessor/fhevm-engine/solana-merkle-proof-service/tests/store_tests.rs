//! The leaf record against a real Postgres: rows round-trip through the migration, the
//! checkpoint moves, and the Merkle proof route answers from the stored leaves and nodes with
//! proofs that verify against the recorded peaks.

mod support;

use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
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
use solana_merkle_proof_service::server::HttpServer;
use solana_merkle_proof_service::store::{
    leaf_commitment, load_checkpoint, load_served_store, load_store_cursors,
    reduce_block_leaves, store_block_leaves, store_checkpoint,
    EncryptedStoreCursor, TransactionStoreWrites,
};
use solana_merkle_proof_service::store_check::{check_stores, StoreAccounts};
use solana_sdk::{account::Account, pubkey::Pubkey};
use tokio_util::sync::CancellationToken;
use zama_solana_acl::{
    encrypted_store_discriminator, mmr_append, mmr_verify, EncryptedStore,
    MmrProof,
};
use zama_solana_merkle_proofs::{
    ErrorCode, ErrorResponse, LeafQuery, LeafQueryKind, MerkleProofOutcome,
    MerkleProofRequest, MerkleProofResponse, MERKLE_PROOFS_PATH,
};

const ACCOUNT: [u8; 32] = [0xAC; 32];
const OWNER: [u8; 32] = [0xA1; 32];
/// The host program the store check's fake chain owns its stores by.
const HOST_PROGRAM: [u8; 32] = [0x40; 32];

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

/// The coprocessor signer address the server answers for.
const AUDIENCE: alloy::primitives::Address =
    alloy::primitives::Address::repeat_byte(0xCC);

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
            AUDIENCE,
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
        AUDIENCE,
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
            let siblings = siblings.clone();
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

    // A wrong leaf is answered `inconsistent` rather than with a path that misses the peaks
    // or proves another grant, and the other leaves of the request are still proved. Leaf 2,
    // the public leaf of 0x11, is its own peak; leaf 0's path is leaf 1. Leaf 2 is asked
    // `times` times: each request has its own body, so none is a copy of an earlier one that
    // the server answers from its cache.
    let leaf_0_and_leaf_2 = |key, times| {
        let leaf_2 = LeafQuery {
            encrypted_store: ACCOUNT,
            handle: [0x11; 32],
            kind: LeafQueryKind::Public,
            key: None,
        };
        let leaf_0 = LeafQuery {
            encrypted_store: ACCOUNT,
            handle: [0x10; 32],
            kind: LeafQueryKind::Allowed,
            key: Some(key),
        };
        MerkleProofRequest {
            leaves: [vec![leaf_0], vec![leaf_2; times]].concat(),
        }
    };
    let set_leaf_0_key = |key: &str| {
        format!(
            "UPDATE leaves SET allowed_key = decode(repeat('{key}', 32), 'hex') \
             WHERE leaf_index = 0"
        )
    };
    for (times, (corruption, key)) in (1..).zip([
        // Leaf 0's commitment allows OWNER: its row now names a key it never allowed.
        (vec![set_leaf_0_key("5e")], [0x5E; 32]),
        (
            vec![
                set_leaf_0_key("a1"),
                "UPDATE leaves SET commitment = decode(repeat('00', 32), 'hex') \
                 WHERE leaf_index = 1"
                    .to_owned(),
            ],
            OWNER,
        ),
        (vec!["DELETE FROM leaves WHERE leaf_index = 1".to_owned()], OWNER),
    ]) {
        for statement in &corruption {
            sqlx::query(statement).execute(&pool).await?;
        }
        let corruption = corruption.join("; ");
        let response = proofs.post(&leaf_0_and_leaf_2(key, times)).await?;
        assert_eq!(response.status(), 200, "{corruption}");
        let body: MerkleProofResponse = decode(response).await?;
        assert_eq!(body.proofs.len(), 1 + times, "{corruption}");
        assert_eq!(body.proofs[0], MerkleProofOutcome::Inconsistent, "{corruption}");
        for proof in &body.proofs[1..] {
            assert_eq!(verify(proof, second.leaves[1].commitment), 2, "{corruption}");
        }
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
        let siblings = siblings.clone();
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
    let error: ErrorResponse = serde_json::from_slice(&other.bytes().await?)?;
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
        match store {
            Some(store) => accounts.insert(address, store_account(store)),
            None => accounts.remove(&address),
        };
    }
}

/// `store` as the host program's account.
fn store_account(store: &EncryptedStore) -> Account {
    let mut data = encrypted_store_discriminator().to_vec();
    borsh::to_writer(&mut data, store).expect("encode");
    Account {
        lamports: 1,
        data,
        owner: Pubkey::new_from_array(HOST_PROGRAM),
        executable: false,
        rent_epoch: 0,
    }
}

/// A chain whose store gains a leaf while the check reads it: the indexer records the leaf before
/// the account read answers, as it can while a check runs.
struct ChainWritingDuringTheRead<'a> {
    pool: &'a sqlx::PgPool,
    store: EncryptedStore,
    write: Mutex<Option<EncryptedStoreWrite>>,
}

impl StoreAccounts for ChainWritingDuringTheRead<'_> {
    async fn accounts(
        &self,
        stores: &[[u8; 32]],
    ) -> anyhow::Result<Vec<Option<Account>>> {
        let write = self.write.lock().unwrap().take().expect("read once");
        let cursor = record(self.pool, 11, vec![write])
            .await
            .map_err(|err| anyhow::anyhow!("{err}"))?;
        let store = EncryptedStore {
            leaf_count: cursor.leaf_count,
            peaks: cursor.peaks,
            ..self.store.clone()
        };
        Ok(stores.iter().map(|_| Some(store_account(&store))).collect())
    }
}

/// What a [`ChainByPage`] does before it answers a page of the check.
enum BeforePage {
    Nothing,
    /// The indexer records this write, as it can between two pages.
    Record(EncryptedStoreWrite),
    Fail,
}

/// [`Chain`]'s accounts, each page answered after its scripted step.
struct ChainByPage<'a> {
    pool: &'a sqlx::PgPool,
    chain: Chain,
    pages: Mutex<VecDeque<BeforePage>>,
}

impl StoreAccounts for ChainByPage<'_> {
    async fn accounts(
        &self,
        stores: &[[u8; 32]],
    ) -> anyhow::Result<Vec<Option<Account>>> {
        let step = self.pages.lock().unwrap().pop_front();
        match step.expect("a scripted page") {
            BeforePage::Nothing => {}
            BeforePage::Record(write) => {
                record(self.pool, 11, vec![write])
                    .await
                    .map_err(|err| anyhow::anyhow!("{err}"))?;
            }
            BeforePage::Fail => anyhow::bail!("account read failed"),
        }
        self.chain.accounts(stores).await
    }
}

/// Records 100 stores after `address`, so a check reads `address` on its first page of 100 and
/// then a second page. Nothing is on chain at their addresses.
async fn record_a_second_page(
    pool: &sqlx::PgPool,
    address: [u8; 32],
) -> Result<(), Box<dyn std::error::Error>> {
    for filler in 0..100u8 {
        let mut store = [0xFF; 32];
        store[31] = filler;
        assert!(address < store, "the store under test sorts first");
        record(
            pool,
            10,
            vec![EncryptedStoreWrite {
                encrypted_store: store,
                previous_leaf_count: 0,
                handle: [0x10; 32],
                allowed_keys: vec![OWNER],
                make_public: false,
            }],
        )
        .await?;
    }
    Ok(())
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

/// An encrypted store of [`HOST_PROGRAM`] at its derived address, with no leaves yet.
fn host_store() -> ([u8; 32], EncryptedStore) {
    let mut store = EncryptedStore {
        program: [1; 32],
        authority: [2; 32],
        scope: [3; 32],
        ..EncryptedStore::default()
    };
    let (address, bump) = Pubkey::find_program_address(
        &store.seeds(),
        &Pubkey::new_from_array(HOST_PROGRAM),
    );
    store.bump = bump;
    (address.to_bytes(), store)
}

/// Records `writes` at `slot` and returns the store's cursor after them.
async fn record(
    pool: &sqlx::PgPool,
    slot: u64,
    writes: Vec<EncryptedStoreWrite>,
) -> Result<EncryptedStoreCursor, Box<dyn std::error::Error>> {
    let address = writes[0].encrypted_store;
    let mut tx = pool.begin().await?;
    let existing = load_store_cursors(&mut tx, &[address]).await?;
    let block = reduce_block_leaves(
        &[TransactionStoreWrites {
            transaction_index: 0,
            sources: writes,
        }],
        existing,
    )?;
    store_block_leaves(&mut tx, slot, &block).await?;
    tx.commit().await?;
    Ok(block.stores[&address].clone())
}

/// The store check quarantines a store whose recorded peaks differ from the chain's, the proof
/// server then answers its leaves `inconsistent`, and a later check that matches lifts it. A
/// record behind the chain leaves the quarantine as it is; a closed store lifts it.
#[tokio::test]
#[serial(db)]
async fn a_store_that_disagrees_with_the_chain_is_quarantined_until_it_matches(
) -> Result<(), Box<dyn std::error::Error>> {
    let (_db, pool) = support::record_db().await;
    let (address, mut chain_store) = host_store();
    let recorded = record(
        &pool,
        10,
        vec![EncryptedStoreWrite {
            encrypted_store: address,
            previous_leaf_count: 0,
            handle: [0x10; 32],
            allowed_keys: vec![OWNER],
            make_public: true,
        }],
    )
    .await?;
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
    // The public leaf asked `times` times: each request has its own body, so none is a copy
    // of an earlier one that the server answers from its cache.
    let public_leaf = |times| MerkleProofRequest {
        leaves: vec![
            LeafQuery {
                encrypted_store: address,
                handle: [0x10; 32],
                kind: LeafQueryKind::Public,
                key: None,
            };
            times
        ],
    };

    chain.set(address, Some(&chain_store));
    check().await?;
    assert!(!quarantined().await?);
    assert_eq!(proofs.post(&public_leaf(1)).await?.status(), 200);

    // A third leaf on chain the record does not hold yet is behind: no peaks are compared, and
    // the store neither enters nor leaves the quarantine.
    let mut chain_grew = chain_store.clone();
    chain_grew.leaf_count = 3;
    chain_grew.peaks.push([0x33; 32]);
    chain.set(address, Some(&chain_grew));
    check().await?;
    assert!(!quarantined().await?);

    let mut diverged = chain_store.clone();
    diverged.peaks[0][0] ^= 1;
    chain.set(address, Some(&diverged));
    check().await?;
    assert!(quarantined().await?);
    let refused = proofs.post(&public_leaf(2)).await?;
    assert_eq!(refused.status(), 200);
    let body: MerkleProofResponse = decode(refused).await?;
    assert_eq!(body.proofs, vec![MerkleProofOutcome::Inconsistent; 2]);

    chain.set(address, Some(&chain_grew));
    check().await?;
    assert!(quarantined().await?);

    chain.set(address, Some(&chain_store));
    check().await?;
    assert!(!quarantined().await?);
    assert_eq!(proofs.post(&public_leaf(3)).await?.status(), 200);

    chain.set(address, Some(&diverged));
    check().await?;
    assert!(quarantined().await?);
    chain.set(address, None);
    check().await?;
    assert!(!quarantined().await?);

    cancel.cancel();
    server_task.await??;
    Ok(())
}

/// The store check compares the record at the chain's leaf count, whatever the record holds past
/// it: a record of 3 leaves matches a chain of 1, 2 or 3 leaves.
#[tokio::test]
#[serial(db)]
async fn the_store_check_compares_the_record_at_the_chain_leaf_count(
) -> Result<(), Box<dyn std::error::Error>> {
    let (_db, pool) = support::record_db().await;
    let (address, chain_store) = host_store();
    let mut cursors = Vec::new();
    for (slot, handle) in [(10, 0x10), (11, 0x11), (12, 0x12)] {
        let write = EncryptedStoreWrite {
            encrypted_store: address,
            previous_leaf_count: cursors.len() as u64,
            handle: [handle; 32],
            allowed_keys: vec![OWNER],
            make_public: false,
        };
        cursors.push(record(&pool, slot, vec![write]).await?);
    }
    let chain = Chain::default();
    let host_program = Pubkey::new_from_array(HOST_PROGRAM);
    let quarantined = || async {
        load_served_store(&pool, address)
            .await
            .map(|store| store.expect("recorded").quarantined)
    };
    for cursor in &cursors {
        let at_count = EncryptedStore {
            leaf_count: cursor.leaf_count,
            peaks: cursor.peaks.clone(),
            ..chain_store.clone()
        };
        chain.set(address, Some(&at_count));
        check_stores(&pool, &chain, &host_program, "1").await?;
        assert!(!quarantined().await?, "at {} leaves", cursor.leaf_count);
    }
    let mut diverged = EncryptedStore {
        leaf_count: 3,
        peaks: cursors[2].peaks.clone(),
        ..chain_store
    };
    diverged.peaks[0][0] ^= 1;
    chain.set(address, Some(&diverged));
    check_stores(&pool, &chain, &host_program, "1").await?;
    assert!(quarantined().await?);
    Ok(())
}

/// A store the indexer writes while the check reads the chain is compared at the chain's new
/// count, not left behind: here its matching peaks lift an earlier quarantine.
#[tokio::test]
#[serial(db)]
async fn a_store_written_during_the_check_is_compared(
) -> Result<(), Box<dyn std::error::Error>> {
    let (_db, pool) = support::record_db().await;
    let (address, chain_store) = host_store();
    let key_write = |previous_leaf_count, handle| EncryptedStoreWrite {
        encrypted_store: address,
        previous_leaf_count,
        handle: [handle; 32],
        allowed_keys: vec![OWNER],
        make_public: false,
    };
    let first = record(&pool, 10, vec![key_write(0, 0x10)]).await?;
    let host_program = Pubkey::new_from_array(HOST_PROGRAM);
    let quarantined = || async {
        load_served_store(&pool, address)
            .await
            .map(|store| store.expect("recorded").quarantined)
    };

    let mut diverged = EncryptedStore {
        leaf_count: first.leaf_count,
        peaks: first.peaks,
        ..chain_store.clone()
    };
    diverged.peaks[0][0] ^= 1;
    let chain = Chain::default();
    chain.set(address, Some(&diverged));
    check_stores(&pool, &chain, &host_program, "1").await?;
    assert!(quarantined().await?);

    let growing = ChainWritingDuringTheRead {
        pool: &pool,
        store: chain_store,
        write: Mutex::new(Some(key_write(1, 0x11))),
    };
    check_stores(&pool, &growing, &host_program, "1").await?;
    assert!(!quarantined().await?);
    Ok(())
}

/// A store the record is behind on until after its page is read is compared once more at the end
/// of the check: here the indexer records its missing leaf while the check reads the next page,
/// and the comparison at the end lifts an earlier quarantine.
#[tokio::test]
#[serial(db)]
async fn a_store_written_after_its_page_is_compared_at_the_end_of_the_check(
) -> Result<(), Box<dyn std::error::Error>> {
    let (_db, pool) = support::record_db().await;
    let (address, chain_store) = host_store();
    let key_write = |previous_leaf_count, handle| EncryptedStoreWrite {
        encrypted_store: address,
        previous_leaf_count,
        handle: [handle; 32],
        allowed_keys: vec![OWNER],
        make_public: false,
    };
    let first = record(&pool, 10, vec![key_write(0, 0x10)]).await?;
    record_a_second_page(&pool, address).await?;
    let host_program = Pubkey::new_from_array(HOST_PROGRAM);
    let quarantined = || async {
        load_served_store(&pool, address)
            .await
            .map(|store| store.expect("recorded").quarantined)
    };

    let mut diverged = EncryptedStore {
        leaf_count: first.leaf_count,
        peaks: first.peaks.clone(),
        ..chain_store.clone()
    };
    diverged.peaks[0][0] ^= 1;
    let chain = Chain::default();
    chain.set(address, Some(&diverged));
    check_stores(&pool, &chain, &host_program, "1").await?;
    assert!(quarantined().await?);

    // The chain holds the store's second leaf; the record gets it only while the check reads
    // the second page.
    let mut grown = EncryptedStore {
        leaf_count: first.leaf_count,
        peaks: first.peaks,
        ..chain_store
    };
    mmr_append(
        &mut grown.peaks,
        &mut grown.leaf_count,
        leaf_commitment(address, 1, [0x11; 32], Some(OWNER)),
    )
    .expect("append");
    let chain = ChainByPage {
        pool: &pool,
        chain: Chain::default(),
        pages: Mutex::new(VecDeque::from([
            BeforePage::Nothing,
            BeforePage::Record(key_write(1, 0x11)),
        ])),
    };
    chain.chain.set(address, Some(&grown));
    check_stores(&pool, &chain, &host_program, "1").await?;
    assert!(!quarantined().await?);
    Ok(())
}

/// A check that stops partway keeps what it found: a store quarantined on the first page stays in
/// the table and in the gauge when the second page's account read fails.
#[tokio::test]
#[serial(db)]
async fn a_check_that_fails_partway_keeps_the_quarantine_it_found(
) -> Result<(), Box<dyn std::error::Error>> {
    let (_db, pool) = support::record_db().await;
    let (address, chain_store) = host_store();
    let recorded = record(
        &pool,
        10,
        vec![EncryptedStoreWrite {
            encrypted_store: address,
            previous_leaf_count: 0,
            handle: [0x10; 32],
            allowed_keys: vec![OWNER],
            make_public: false,
        }],
    )
    .await?;
    record_a_second_page(&pool, address).await?;
    let mut diverged = EncryptedStore {
        leaf_count: recorded.leaf_count,
        peaks: recorded.peaks,
        ..chain_store
    };
    diverged.peaks[0][0] ^= 1;
    let chain = ChainByPage {
        pool: &pool,
        chain: Chain::default(),
        pages: Mutex::new(VecDeque::from([
            BeforePage::Nothing,
            BeforePage::Fail,
        ])),
    };
    chain.chain.set(address, Some(&diverged));
    // A label of its own, so no other test's check moves this gauge.
    let host_chain_id = "fails-partway";

    let host_program = Pubkey::new_from_array(HOST_PROGRAM);
    assert!(check_stores(&pool, &chain, &host_program, host_chain_id)
        .await
        .is_err());

    let served = load_served_store(&pool, address).await?.expect("recorded");
    assert!(served.quarantined);
    let gauge = prometheus::gather()
        .into_iter()
        .find(|family| {
            family.name() == "solana_merkle_indexer_quarantined_stores"
        })
        .and_then(|family| {
            family.get_metric().iter().find_map(|metric| {
                metric
                    .get_label()
                    .iter()
                    .any(|label| label.value() == host_chain_id)
                    .then(|| metric.get_gauge().value())
            })
        });
    assert_eq!(gauge, Some(1.0));
    Ok(())
}
