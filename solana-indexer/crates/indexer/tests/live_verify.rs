//! Live end-to-end verification of the indexer against a real `solana-test-validator`.
//!
//! Ignored by default (it spawns a validator, deploys `zama_host`, and needs a
//! reachable Postgres). Run explicitly:
//!
//! ```text
//! DATABASE_URL=postgres://postgres:postgres@localhost:5432/indexer \
//!   cargo test -p indexer --test live_verify -- --ignored --nocapture
//! ```
//!
//! It drives the four EV-ACL host instructions directly (initialize / allow /
//! rotate / mark_public), lets the Carbon pipeline ingest them from the chain,
//! and asserts: (1) the reconstructed lineage matches the on-chain account, (2)
//! every leaf's proof verifies against the LIVE on-chain peaks, and (3) a second
//! pipeline run resumes from the cursor without duplicating or dropping events.

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::str::FromStr;
use std::time::{Duration, Instant};

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::Keypair;
use solana_message::Message;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::Transaction;
use tokio_util::sync::CancellationToken;

use indexer::config::settings::SolanaConfig;
use indexer::decoder::{
    AllowSubjectsArgs, InitializeArgs, RotateArgs, ALLOW_SUBJECTS_DISCRIMINATOR,
    INITIALIZE_DISCRIMINATOR, MARK_PUBLIC_DISCRIMINATOR, ROTATE_DISCRIMINATOR,
};
use indexer::lineage::proof;
use indexer::metrics::Metrics;
use indexer::rpc::SolanaRpc;
use indexer::store::repositories::lineage_repo::LineageRepo;

const PROGRAM_ID: &str = "6AtbvED1rfX68aCT1tYgU1aeu4kFksPDxZG9gtB1Fgtu";
const RPC_PORT: u16 = 8899;
const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

fn rpc_url() -> String {
    format!("http://127.0.0.1:{RPC_PORT}")
}

/// `solana-indexer/crates/indexer` -> repo root is three levels up.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("canonicalize repo root")
}

fn system_program() -> Pubkey {
    Pubkey::from_str("11111111111111111111111111111111").unwrap()
}

/// Owns a `solana-test-validator` child; kills it (and frees the port) on drop.
struct Validator {
    child: Child,
    ledger: PathBuf,
}

impl Drop for Validator {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = std::fs::remove_dir_all(&self.ledger);
    }
}

impl Validator {
    fn start() -> Validator {
        let ledger = std::env::temp_dir().join("indexer-live-verify-ledger");
        let _ = std::fs::remove_dir_all(&ledger);
        let child = Command::new("solana-test-validator")
            .args([
                "--reset",
                "--quiet",
                "--rpc-port",
                &RPC_PORT.to_string(),
                "--ledger",
                ledger.to_str().unwrap(),
                "--bind-address",
                "127.0.0.1",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn solana-test-validator (is it on PATH?)");
        Validator { child, ledger }
    }
}

async fn wait_for_rpc(client: &RpcClient) {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        if client.get_health().await.is_ok() {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "validator RPC never became healthy"
        );
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

/// `solana program deploy` returns once the deploy tx confirms, but the program
/// account can lag behind at the client's commitment. Poll until it is visible
/// and executable so the first instruction does not race the loader.
async fn wait_for_program(client: &RpcClient, program: &Pubkey) {
    let deadline = Instant::now() + Duration::from_secs(40);
    loop {
        if let Ok(acc) = client.get_account(program).await {
            if acc.executable {
                break;
            }
        }
        assert!(
            Instant::now() < deadline,
            "deployed program never became executable"
        );
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    // Delay visibility: an upgradeable program is only callable one slot AFTER
    // its deploy slot, so wait for the slot to advance before the first call.
    let start = client.get_slot().await.unwrap_or(0);
    loop {
        let now = client.get_slot().await.unwrap_or(start);
        if now > start + 1 {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "slot did not advance past the deploy slot"
        );
        tokio::time::sleep(Duration::from_millis(300)).await;
    }
}

/// Writes a keypair to a CLI-compatible JSON byte-array file.
fn write_keypair_file(kp: &Keypair, path: &Path) {
    let bytes = kp.to_bytes();
    let json = format!(
        "[{}]",
        bytes
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    std::fs::write(path, json).expect("write keypair file");
}

/// Deploys `zama_host.so` at the committed `6Atbv…` program id (matching
/// `declare_id!`), paid for by `payer`.
fn deploy_program(payer_path: &Path) {
    let root = repo_root();
    let so = root.join("solana/target/deploy/zama_host.so");
    let program_keypair = root.join("solana/scripts/poc/test-keypairs/zama_host-keypair.json");
    assert!(
        so.exists(),
        "missing {}; run `cargo build-sbf`",
        so.display()
    );
    let status = Command::new("solana")
        .args([
            "program",
            "deploy",
            "--url",
            &rpc_url(),
            "--keypair",
            payer_path.to_str().unwrap(),
            "--upgrade-authority",
            payer_path.to_str().unwrap(),
            "--program-id",
            program_keypair.to_str().unwrap(),
            so.to_str().unwrap(),
        ])
        .status()
        .expect("run `solana program deploy`");
    assert!(status.success(), "program deploy failed");
}

fn ev_acl_pda(program: &Pubkey, value_key: &[u8; 32]) -> Pubkey {
    Pubkey::find_program_address(
        &[zama_solana_acl::ENCRYPTED_VALUE_ACL_SEED, value_key],
        program,
    )
    .0
}

fn ev_acl_metas(payer: &Pubkey, authority: &Pubkey, pda: &Pubkey) -> Vec<AccountMeta> {
    vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*authority, true),
        AccountMeta::new(*pda, false),
        AccountMeta::new_readonly(system_program(), false),
    ]
}

fn ix(program: Pubkey, metas: Vec<AccountMeta>, disc: [u8; 8], args: Vec<u8>) -> Instruction {
    let mut data = disc.to_vec();
    data.extend_from_slice(&args);
    Instruction {
        program_id: program,
        accounts: metas,
        data,
    }
}

async fn send(client: &RpcClient, payer: &Keypair, authority: &Keypair, instruction: Instruction) {
    let blockhash = client.get_latest_blockhash().await.expect("blockhash");
    let message = Message::new(&[instruction], Some(&payer.pubkey()));
    let tx = Transaction::new(&[payer, authority], message, blockhash);
    client
        .send_and_confirm_transaction(&tx)
        .await
        .expect("send_and_confirm EV-ACL instruction");
}

async fn pg_pool() -> sqlx::PgPool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/indexer".to_string());
    let pool = sqlx::PgPool::connect(&url)
        .await
        .expect("connect indexer DB");
    // Idempotent schema + clean slate for a deterministic run.
    let migration = std::fs::read_to_string(
        repo_root().join("solana-indexer/crates/indexer-migrate/migrations/0001_create_tables.sql"),
    )
    .expect("read migration");
    sqlx::raw_sql(&migration)
        .execute(&pool)
        .await
        .expect("apply migration");
    // Clean slate, but keep the singleton cursor row (get_cursor fetch_one needs it).
    sqlx::raw_sql(
        "TRUNCATE lineage_state, lineage_events; \
         INSERT INTO indexer_cursor (id) VALUES (1) ON CONFLICT (id) DO NOTHING; \
         UPDATE indexer_cursor SET last_signature = '', last_slot = 0 WHERE id = 1;",
    )
    .execute(&pool)
    .await
    .expect("reset tables");
    pool
}

fn solana_config(commitment: &str) -> SolanaConfig {
    SolanaConfig {
        rpc_url: rpc_url(),
        program_id: PROGRAM_ID.to_string(),
        commitment: commitment.to_string(),
        poll_interval: Duration::from_secs(1),
        backfill_batch: 100,
    }
}

/// A running Carbon pipeline; cancels and joins on drop.
struct RunningPipeline {
    cancel: CancellationToken,
    handle: tokio::task::JoinHandle<anyhow::Result<()>>,
}

impl RunningPipeline {
    fn spawn(pool: &sqlx::PgPool) -> RunningPipeline {
        let cancel = CancellationToken::new();
        let pipe_cancel = cancel.clone();
        let pipe_pool = pool.clone();
        let handle = tokio::spawn(async move {
            let cfg = solana_config("confirmed");
            let repo = LineageRepo::new(pipe_pool);
            let metrics = Metrics::new();
            let res = indexer::pipeline::run(&cfg, repo, metrics, pipe_cancel).await;
            if let Err(e) = &res {
                eprintln!("PIPELINE EXITED WITH ERROR: {e:?}");
            }
            res
        });
        RunningPipeline { cancel, handle }
    }

    async fn stop(self) {
        self.cancel.cancel();
        let _ = self.handle.await;
    }
}

/// Polls `repo.get_state(pda).leaf_count` until it equals `target` (or times out).
async fn wait_for_leaf_count(repo: &LineageRepo, pda: &[u8; 32], target: u64) {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        if let Ok(Some(s)) = repo.get_state(pda).await {
            if s.leaf_count as u64 == target {
                return;
            }
        }
        if Instant::now() >= deadline {
            let cur = repo.get_cursor().await;
            let st = repo.get_state(pda).await;
            panic!("indexer never reached leaf_count={target}; cursor={cur:?} state={st:?}");
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "spawns solana-test-validator + deploys zama_host; run with --ignored"]
async fn indexer_live_verify_end_to_end() {
    let program = Pubkey::from_str(PROGRAM_ID).unwrap();
    let _validator = Validator::start();
    let client = RpcClient::new_with_commitment(rpc_url(), CommitmentConfig::confirmed());
    wait_for_rpc(&client).await;

    // Fund a payer and deploy the program at its declared id.
    let payer = Keypair::new();
    let payer_path = std::env::temp_dir().join("indexer-live-verify-payer.json");
    write_keypair_file(&payer, &payer_path);
    let sig = client
        .request_airdrop(&payer.pubkey(), 500 * LAMPORTS_PER_SOL)
        .await
        .expect("airdrop");
    let deadline = Instant::now() + Duration::from_secs(30);
    while client.get_balance(&payer.pubkey()).await.unwrap_or(0) == 0 {
        assert!(Instant::now() < deadline, "airdrop never landed");
        tokio::time::sleep(Duration::from_millis(300)).await;
    }
    let _ = client.confirm_transaction(&sig).await;
    deploy_program(&payer_path);
    wait_for_program(&client, &program).await;

    // Start the indexer FIRST so it forward-polls each instruction as it lands —
    // mirroring production. (A cold backfill of already-finalized history arrives
    // newest-first, which the processor intentionally skips: a rotate/mark before
    // its initialize is a partial-backfill miss. Continuous forward-polling is the
    // real ingestion mode.)
    let pool = pg_pool().await;
    let pipeline = RunningPipeline::spawn(&pool);
    let repo = LineageRepo::new(pool.clone());

    // Drive a lineage: initialize -> allow (2nd subject) -> rotate x3 -> mark_public.
    // Space each send beyond the 1s poll interval so each lands in its own poll
    // batch and is ingested in chronological order.
    let authority = Keypair::new();
    let subject2 = Keypair::new();
    let domain = Pubkey::new_unique();
    let label: [u8; 32] = *b"balance_________________________";
    let value_key =
        zama_solana_acl::acl_nonce_key(domain.to_bytes(), authority.pubkey().to_bytes(), label);
    let pda = ev_acl_pda(&program, &value_key);
    let pda_bytes = pda.to_bytes();
    let metas = || ev_acl_metas(&payer.pubkey(), &authority.pubkey(), &pda);
    let space = || tokio::time::sleep(Duration::from_millis(1500));

    let init = InitializeArgs {
        value_key,
        acl_domain_key: domain,
        encrypted_value_label: label,
        handle: [1u8; 32],
        subjects: vec![authority.pubkey()],
    };
    send(
        &client,
        &payer,
        &authority,
        ix(
            program,
            metas(),
            INITIALIZE_DISCRIMINATOR,
            borsh::to_vec(&init).unwrap(),
        ),
    )
    .await;
    space().await;

    let allow = AllowSubjectsArgs {
        subjects: vec![subject2.pubkey()],
    };
    send(
        &client,
        &payer,
        &authority,
        ix(
            program,
            metas(),
            ALLOW_SUBJECTS_DISCRIMINATOR,
            borsh::to_vec(&allow).unwrap(),
        ),
    )
    .await;
    space().await;

    for handle in [[2u8; 32], [3u8; 32], [4u8; 32]] {
        let rotate = RotateArgs {
            new_handle: handle,
            new_subjects: vec![authority.pubkey(), subject2.pubkey()],
        };
        send(
            &client,
            &payer,
            &authority,
            ix(
                program,
                metas(),
                ROTATE_DISCRIMINATOR,
                borsh::to_vec(&rotate).unwrap(),
            ),
        )
        .await;
        space().await;
    }
    send(
        &client,
        &payer,
        &authority,
        ix(program, metas(), MARK_PUBLIC_DISCRIMINATOR, Vec::new()),
    )
    .await;

    // Ground truth: decode the on-chain account.
    let account = client
        .get_account(&pda)
        .await
        .expect("lineage account on chain");
    let on_chain = zama_solana_acl::decode_account(&account.data).expect("decode lineage");
    let on_chain_leaf_count = on_chain.leaf_count;
    assert!(
        on_chain_leaf_count >= 4,
        "expected >=4 leaves, got {on_chain_leaf_count}"
    );

    // Wait for the running pipeline to ingest up to the chain's leaf_count.
    wait_for_leaf_count(&repo, &pda_bytes, on_chain_leaf_count).await;
    pipeline.stop().await;

    let state = repo
        .get_state(&pda_bytes)
        .await
        .unwrap()
        .expect("indexed state");
    assert_eq!(
        state.leaf_count as u64, on_chain_leaf_count,
        "indexer leaf_count vs chain"
    );
    assert_eq!(
        state.current_handle, on_chain.current_handle,
        "indexer current_handle vs chain"
    );
    assert_eq!(
        repo.pda_for_value_key(&value_key).await.unwrap(),
        Some(pda_bytes),
        "value_key -> PDA mapping must be recorded from initialize"
    );

    // Every leaf's proof must verify against the LIVE on-chain peaks.
    let events = repo.events_for_pda(&pda_bytes).await.unwrap();
    let rpc = SolanaRpc::new(rpc_url(), CommitmentConfig::confirmed());
    let data = rpc.account_data(pda_bytes).await.expect("rpc account data");
    let peaks = proof::on_chain_peaks_from_account(&data);
    assert!(peaks.is_some(), "on-chain peaks must decode");
    for leaf in 0..on_chain_leaf_count {
        let built = proof::build(pda_bytes, &events, leaf, peaks.clone())
            .unwrap_or_else(|e| panic!("build proof for leaf {leaf}: {e:?}"));
        assert!(
            built.verified,
            "leaf {leaf} proof did not verify against on-chain peaks"
        );
        assert_eq!(
            built.leaf_count, on_chain_leaf_count,
            "proof leaf_count vs chain"
        );
    }

    // Cross-component bridge: feed the indexer-reconstructed proofs into the SAME authorization
    // predicates the KMS connector runs (`zama_solana_acl::authorize_*`), against the LIVE on-chain
    // account — the decrypt-authorization matrix end to end. The driving sequence produces leaves:
    // 0=(h1,auth) 1=(h1,subject2) 2=(h2,auth) 3=(h2,subject2) 4=(h3,auth) 5=(h3,subject2)
    // 6=public(h4). (The stale-merged -> Recoverable and drift-survivor -> accept *classification*
    // is covered by the kms-worker unit tests over these same MMR primitives.)
    let (peaks_vec, peaks_lc) = peaks.clone().expect("peaks");
    let events_le: Vec<_> = events.iter().map(|r| r.event.clone()).collect();
    let auth_b = authority.pubkey().to_bytes();
    let stranger = [0x9eu8; 32];
    let h1 = [1u8; 32];
    let h4 = [4u8; 32];

    // current accept: the live handle decrypts for a subject; a non-subject and a rotated-away
    // handle are both rejected on the no-proof current path (the latter needs a historical proof).
    assert_eq!(on_chain.current_handle, h4);
    zama_solana_acl::authorize_current(&on_chain, on_chain.current_handle, auth_b)
        .expect("current: live handle by a subject must authorize");
    assert!(zama_solana_acl::authorize_current(&on_chain, h4, stranger).is_err());
    assert!(zama_solana_acl::authorize_current(&on_chain, h1, auth_b).is_err());

    // historical accept: leaf 0 commits (h1, auth); the proof authorizes that exact (handle,subject),
    // and a wrong subject for the same leaf is rejected (the commitment binds the subject).
    let proof0 = zama_solana_acl::build_verified_proof_from_events(
        pda_bytes, &events_le, &peaks_vec, peaks_lc, 0,
    )
    .expect("build historical proof for leaf 0");
    zama_solana_acl::authorize_historical(pda_bytes, &on_chain, h1, auth_b, &proof0)
        .expect("historical decrypt of (h1, auth) must authorize");
    assert!(
        zama_solana_acl::authorize_historical(pda_bytes, &on_chain, h1, stranger, &proof0).is_err()
    );

    // public accept: the mark_public leaf authorizes an exact public decrypt of h4; a public proof
    // for one handle never authorizes another.
    let public_leaf = on_chain_leaf_count - 1;
    let proof_pub = zama_solana_acl::build_verified_proof_from_events(
        pda_bytes,
        &events_le,
        &peaks_vec,
        peaks_lc,
        public_leaf,
    )
    .expect("build public proof for the mark_public leaf");
    zama_solana_acl::authorize_public(pda_bytes, &on_chain, h4, &proof_pub)
        .expect("public decrypt of h4 must authorize");
    assert!(zama_solana_acl::authorize_public(pda_bytes, &on_chain, h1, &proof_pub).is_err());

    // Cursor resume: a second run must not duplicate or drop events. It resumes
    // from the persisted cursor (its `until` bound), so it should crawl back only
    // to already-processed signatures and add nothing.
    let events_before = repo.events_for_pda(&pda_bytes).await.unwrap().len();
    let (cursor_sig, _) = repo.get_cursor().await.unwrap();
    assert!(
        !cursor_sig.is_empty(),
        "cursor must be persisted after the first run"
    );
    let resume = RunningPipeline::spawn(&pool);
    tokio::time::sleep(Duration::from_secs(8)).await;
    resume.stop().await;
    let events_after = repo.events_for_pda(&pda_bytes).await.unwrap().len();
    assert_eq!(
        events_after, events_before,
        "cursor resume duplicated or dropped events"
    );
    let state_after = repo.get_state(&pda_bytes).await.unwrap().unwrap();
    assert_eq!(
        state_after.leaf_count as u64, on_chain_leaf_count,
        "leaf_count must be stable across a resume"
    );
}
