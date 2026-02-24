use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anchor_lang::InstructionData;
use anyhow::{Context, Result};
use clap::{ArgAction, Parser, ValueEnum};
use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_listener::database::ingest::map_envelope_to_actions;
use solana_listener::database::solana_event_propagate::Database;
use solana_listener::poller::solana_rpc_source::SolanaRpcEventSource;
use solana_listener::poller::{Cursor, EventSource};
use solana_sdk::hash::hash;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Signature, Signer};
use solana_sdk::transaction::Transaction;
use sqlx::postgres::PgPoolOptions;
use tokio::time::{sleep, Instant};
use urlencoding::encode;

const DEFAULT_PROGRAM_ID: &str = "Fg6PaFpoGXkYsidMpWxTWqkZ4FK6s7vY8J3xA5rJQbSq";
const DEFAULT_TENANT_API_KEY: &str = "00000000-0000-0000-0000-000000000042";
// Must satisfy current `tenants_chain_id_check` until PR #1856 schema lands.
const DEFAULT_HOST_CHAIN_ID: i64 = 12_345;
const OP_ADD: u8 = 0;
const HCU_METER_SEED: &[u8] = b"hcu_meter";
const HCU_GLOBAL_SEED: &[u8] = b"hcu_global";

#[derive(Clone, Debug, ValueEnum)]
enum PostgresMode {
    External,
    Docker,
}

#[derive(Parser, Debug)]
#[command(name = "solana_poc_runner")]
#[command(about = "Explorer-visible Solana PoC runner for host-listener flow")]
struct Args {
    #[arg(long, default_value = "http://127.0.0.1:8899")]
    rpc_url: String,

    #[arg(long, default_value = DEFAULT_PROGRAM_ID)]
    program_id: String,

    #[arg(long, default_value_t = DEFAULT_HOST_CHAIN_ID)]
    host_chain_id: i64,

    #[arg(long, default_value = "~/.config/solana/id.json")]
    wallet: String,

    #[arg(long, action = ArgAction::Set, default_value_t = true)]
    publish_idl: bool,

    #[arg(long)]
    idl_path: Option<String>,

    #[arg(long, value_enum, default_value_t = PostgresMode::Docker)]
    postgres_mode: PostgresMode,

    #[arg(long)]
    database_url: Option<String>,

    #[arg(long, default_value = "postgres")]
    docker_postgres_image: String,

    #[arg(long, default_value = "15.7")]
    docker_postgres_tag: String,

    #[arg(long, action = ArgAction::Set, default_value_t = true)]
    docker_cleanup: bool,

    #[arg(long, default_value = DEFAULT_TENANT_API_KEY)]
    tenant_api_key: String,
}

struct DockerPostgres {
    container_id: String,
}

impl DockerPostgres {
    fn stop(&self) {
        let _ = Command::new("docker")
            .args(["rm", "-f", &self.container_id])
            .status();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    run(args).await
}

async fn run(args: Args) -> Result<()> {
    let wallet_path = expand_tilde(&args.wallet);
    let payer =
        read_keypair_file(&wallet_path).map_err(|e| anyhow::anyhow!("read wallet keypair: {e}"))?;
    let program_id = Pubkey::from_str(&args.program_id).context("parse program id")?;
    let rpc = RpcClient::new_with_commitment(args.rpc_url.clone(), CommitmentConfig::confirmed());

    ensure_program_deployed(&rpc, program_id, &args.program_id)?;
    airdrop_payer(&rpc, &payer)?;
    if args.publish_idl {
        publish_anchor_idl(&args, &wallet_path)?;
    }

    let (db_url, docker_pg) = match args.postgres_mode {
        PostgresMode::External => {
            let url = args
                .database_url
                .clone()
                .context("--database-url is required with --postgres-mode external")?;
            (url, None)
        }
        PostgresMode::Docker => {
            let (url, pg) =
                start_docker_postgres(&args.docker_postgres_image, &args.docker_postgres_tag)
                    .await?;
            (url, Some(pg))
        }
    };

    let result = run_add_flow(&args, &rpc, &payer, program_id, &db_url).await;

    if let Some(pg) = docker_pg {
        if args.docker_cleanup {
            pg.stop();
        } else {
            println!(
                "postgres_container_kept={} (set --docker-cleanup true to auto-remove)",
                pg.container_id
            );
        }
    }

    result
}

async fn run_add_flow(
    args: &Args,
    rpc: &RpcClient,
    payer: &solana_sdk::signature::Keypair,
    program_id: Pubkey,
    db_url: &str,
) -> Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(db_url)
        .await
        .context("connect postgres")?;
    sqlx::migrate!("../db-migration/migrations")
        .run(&pool)
        .await
        .context("run db migrations")?;

    let tenant_id = seed_tenant_with_keys(&pool, &args.tenant_api_key, args.host_chain_id).await?;

    let start_cursor = finalized_cursor(rpc)?;
    let lhs = [0x11_u8; 32];
    let rhs = [0x22_u8; 32];
    let is_scalar = false;
    let result_handle = derive_add_result_handle(lhs, rhs, is_scalar);
    let allowed_account = Pubkey::new_unique();

    let request_add_sig = send_instruction(
        rpc,
        payer,
        build_request_add_instruction(program_id, payer.pubkey(), lhs, rhs, is_scalar),
    )?;
    let allow_sig = send_instruction(
        rpc,
        payer,
        build_allow_instruction(program_id, payer.pubkey(), result_handle, allowed_account),
    )?;

    println!("request_add_signature={request_add_sig}");
    println!("allow_signature={allow_sig}");
    println!(
        "request_add_explorer_url=https://explorer.solana.com/tx/{}?cluster=custom&customUrl={}",
        request_add_sig,
        encode(&args.rpc_url)
    );
    println!(
        "allow_explorer_url=https://explorer.solana.com/tx/{}?cluster=custom&customUrl={}",
        allow_sig,
        encode(&args.rpc_url)
    );

    let mut source =
        SolanaRpcEventSource::new(args.rpc_url.clone(), &args.program_id, args.host_chain_id);
    let mut db = Database::connect(db_url, args.host_chain_id, tenant_id).await?;
    let (computations, allowed, pbs, next_cursor) =
        ingest_expected(&mut source, &mut db, tenant_id, start_cursor, 2).await?;

    println!("ingest_inserted_computations={computations}");
    println!("ingest_inserted_allowed_handles={allowed}");
    println!("ingest_inserted_pbs_computations={pbs}");
    println!(
        "cursor_slot_start={} cursor_slot_end={}",
        start_cursor.slot, next_cursor.slot
    );
    anyhow::ensure!(computations == 1, "expected one computation insert");
    anyhow::ensure!(allowed == 1, "expected one allowed_handle insert");
    anyhow::ensure!(pbs == 1, "expected one pbs_computation insert");

    Ok(())
}

fn ensure_program_deployed(
    rpc: &RpcClient,
    program_id: Pubkey,
    program_id_str: &str,
) -> Result<()> {
    let account = rpc
        .get_account_with_commitment(&program_id, CommitmentConfig::confirmed())
        .context("query program account")?
        .value;
    let Some(account) = account else {
        anyhow::bail!(
            "program {} is not deployed on this RPC. deploy it first (anchor deploy).",
            program_id_str
        );
    };
    anyhow::ensure!(
        account.executable,
        "account {} exists but is not executable",
        program_id_str
    );
    Ok(())
}

fn publish_anchor_idl(args: &Args, wallet_path: &str) -> Result<()> {
    let root = repo_root()?;
    let host_program_dir = root.join("solana/host-programs");
    let idl_path = match &args.idl_path {
        Some(path) => PathBuf::from(expand_tilde(path)),
        None => host_program_dir.join("target/idl/zama_host.json"),
    };

    anyhow::ensure!(
        idl_path.exists(),
        "IDL file not found at {}. Run `anchor build` in solana/host-programs first or pass --idl-path.",
        idl_path.display()
    );

    let init = run_anchor_idl_command(
        &host_program_dir,
        &args.rpc_url,
        wallet_path,
        "init",
        &args.program_id,
        &idl_path,
    )?;
    if init.success {
        println!("idl_publish=init_ok idl_path={}", idl_path.display());
        return Ok(());
    }

    let upgrade = run_anchor_idl_command(
        &host_program_dir,
        &args.rpc_url,
        wallet_path,
        "upgrade",
        &args.program_id,
        &idl_path,
    )?;
    if upgrade.success {
        println!("idl_publish=upgrade_ok idl_path={}", idl_path.display());
        return Ok(());
    }

    anyhow::bail!(
        "anchor idl publish failed.\ninit stderr:\n{}\nupgrade stderr:\n{}",
        init.stderr,
        upgrade.stderr
    );
}

struct CmdResult {
    success: bool,
    stderr: String,
}

fn run_anchor_idl_command(
    cwd: &PathBuf,
    rpc_url: &str,
    wallet_path: &str,
    action: &str,
    program_id: &str,
    idl_path: &PathBuf,
) -> Result<CmdResult> {
    let output = Command::new("anchor")
        .current_dir(cwd)
        .env("ANCHOR_PROVIDER_URL", rpc_url)
        .env("ANCHOR_WALLET", wallet_path)
        .args(["idl", action, program_id, "-f"])
        .arg(idl_path)
        .output()
        .with_context(|| format!("run `anchor idl {action}`"))?;

    Ok(CmdResult {
        success: output.status.success(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}

fn airdrop_payer(rpc: &RpcClient, payer: &solana_sdk::signature::Keypair) -> Result<()> {
    let sig = rpc
        .request_airdrop(&payer.pubkey(), 2_000_000_000)
        .context("request airdrop")?;
    rpc.poll_for_signature_with_commitment(&sig, CommitmentConfig::confirmed())
        .context("airdrop confirmation")?;
    Ok(())
}

fn finalized_cursor(rpc: &RpcClient) -> Result<Cursor> {
    let slot = rpc
        .get_slot_with_commitment(CommitmentConfig::finalized())
        .context("get finalized slot")?;
    Ok(Cursor {
        slot,
        tx_index: 0,
        op_index: 0,
    })
}

fn build_request_add_instruction(
    program_id: Pubkey,
    signer: Pubkey,
    lhs: [u8; 32],
    rhs: [u8; 32],
    is_scalar: bool,
) -> Instruction {
    let data = zama_host::instruction::RequestAdd {
        lhs,
        rhs,
        is_scalar,
    }
    .data();
    Instruction {
        program_id,
        accounts: vec![AccountMeta::new(signer, true)],
        data,
    }
}

fn build_allow_instruction(
    program_id: Pubkey,
    signer: Pubkey,
    handle: [u8; 32],
    account: Pubkey,
) -> Instruction {
    let account = anchor_lang::prelude::Pubkey::new_from_array(account.to_bytes());
    let data = zama_host::instruction::Allow { handle, account }.data();
    Instruction {
        program_id,
        accounts: vec![AccountMeta::new(signer, true)],
        data,
    }
}

fn derive_add_result_handle(lhs: [u8; 32], rhs: [u8; 32], is_scalar: bool) -> [u8; 32] {
    let mut output = [0u8; 32];
    for i in 0..32 {
        output[i] = lhs[i] ^ rhs[i];
    }
    output[29] ^= OP_ADD;
    if is_scalar {
        output[31] ^= 0x01;
    }
    output
}

fn anchor_global_discriminator(name: &str) -> [u8; 8] {
    let preimage = format!("global:{name}");
    let digest = hash(preimage.as_bytes()).to_bytes();
    let mut out = [0u8; 8];
    out.copy_from_slice(&digest[..8]);
    out
}

fn build_begin_hcu_meter_instruction(
    program_id: Pubkey,
    payer: Pubkey,
    authority: Pubkey,
    meter: Pubkey,
    hcu_global: Pubkey,
    meter_id: [u8; 16],
) -> Instruction {
    let mut data = Vec::with_capacity(8 + 16);
    data.extend_from_slice(&anchor_global_discriminator("begin_hcu_meter"));
    data.extend_from_slice(&meter_id);
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(meter, false),
            AccountMeta::new(hcu_global, false),
            AccountMeta::new_readonly(system_program_id(), false),
        ],
        data,
    }
}

fn build_close_hcu_meter_instruction(
    program_id: Pubkey,
    payer: Pubkey,
    authority: Pubkey,
    meter: Pubkey,
    hcu_global: Pubkey,
    meter_id: [u8; 16],
) -> Instruction {
    let mut data = Vec::with_capacity(8 + 16);
    data.extend_from_slice(&anchor_global_discriminator("close_hcu_meter"));
    data.extend_from_slice(&meter_id);
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(meter, false),
            AccountMeta::new(hcu_global, false),
        ],
        data,
    }
}

fn derive_hcu_meter_pda(program_id: Pubkey, authority: Pubkey, meter_id: [u8; 16]) -> Pubkey {
    let (pda, _bump) = Pubkey::find_program_address(
        &[HCU_METER_SEED, authority.as_ref(), meter_id.as_ref()],
        &program_id,
    );
    pda
}

fn derive_hcu_global_pda(program_id: Pubkey) -> Pubkey {
    let (pda, _bump) = Pubkey::find_program_address(&[HCU_GLOBAL_SEED], &program_id);
    pda
}

fn meter_id_for_instruction(instruction: &Instruction) -> [u8; 16] {
    let mut preimage = Vec::with_capacity(
        instruction.data.len() + instruction.accounts.len() * 34 + std::mem::size_of::<u128>(),
    );
    preimage.extend_from_slice(&instruction.data);
    for meta in &instruction.accounts {
        preimage.extend_from_slice(meta.pubkey.as_ref());
        preimage.push(u8::from(meta.is_signer));
        preimage.push(u8::from(meta.is_writable));
    }
    let now_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .to_le_bytes();
    preimage.extend_from_slice(&now_nanos);

    let digest = hash(&preimage).to_bytes();
    let mut meter_id = [0u8; 16];
    meter_id.copy_from_slice(&digest[..16]);
    meter_id
}

fn instruction_requires_hcu_meter(instruction: &Instruction) -> bool {
    let Some(discriminator) = instruction.data.get(..8) else {
        return false;
    };
    [
        "request_add",
        "request_sub",
        "request_binary_op",
        "request_unary_op",
        "request_if_then_else",
        "request_cast",
        "request_trivial_encrypt",
        "request_rand",
        "request_rand_bounded",
    ]
    .iter()
    .any(|name| discriminator == anchor_global_discriminator(name).as_slice())
}

fn wrap_metered_instruction(
    signer: Pubkey,
    mut request_instruction: Instruction,
) -> Vec<Instruction> {
    let program_id = request_instruction.program_id;
    let meter_id = meter_id_for_instruction(&request_instruction);
    let meter = derive_hcu_meter_pda(program_id, signer, meter_id);
    let hcu_global = derive_hcu_global_pda(program_id);
    request_instruction
        .accounts
        .push(AccountMeta::new(meter, false));

    vec![
        build_begin_hcu_meter_instruction(program_id, signer, signer, meter, hcu_global, meter_id),
        request_instruction,
        build_close_hcu_meter_instruction(program_id, signer, signer, meter, hcu_global, meter_id),
    ]
}

fn system_program_id() -> Pubkey {
    Pubkey::from_str("11111111111111111111111111111111").expect("valid system program id")
}

fn send_instruction(
    rpc: &RpcClient,
    signer: &solana_sdk::signature::Keypair,
    instruction: Instruction,
) -> Result<Signature> {
    let instructions = if instruction_requires_hcu_meter(&instruction) {
        wrap_metered_instruction(signer.pubkey(), instruction)
    } else {
        vec![instruction]
    };
    let blockhash = rpc.get_latest_blockhash().context("latest blockhash")?;
    let tx = Transaction::new_signed_with_payer(
        &instructions,
        Some(&signer.pubkey()),
        &[signer],
        blockhash,
    );
    let signature = rpc.send_transaction(&tx).context("send transaction")?;
    rpc.poll_for_signature_with_commitment(&signature, CommitmentConfig::finalized())
        .context("wait tx finalized")?;
    Ok(signature)
}

async fn ingest_expected(
    source: &mut SolanaRpcEventSource,
    db: &mut Database,
    tenant_id: i32,
    start_cursor: Cursor,
    expected_events: usize,
) -> Result<(usize, usize, usize, Cursor)> {
    let mut cursor = start_cursor;
    let mut inserted_computations = 0usize;
    let mut inserted_allowed = 0usize;
    let mut inserted_pbs = 0usize;
    let mut seen = 0usize;

    for _ in 0..24 {
        let batch = source.next_batch(cursor, 128, true).await?;
        cursor = batch.next_cursor;
        if batch.events.is_empty() {
            sleep(Duration::from_millis(500)).await;
            continue;
        }

        for envelope in batch.events {
            seen += 1;
            let actions = map_envelope_to_actions(&envelope, tenant_id)?;
            let stats = db.apply_actions(&actions).await?;
            inserted_computations += stats.inserted_computations;
            inserted_allowed += stats.inserted_allowed_handles;
            inserted_pbs += stats.inserted_pbs_computations;
        }
        db.set_cursor(cursor.slot as i64).await?;
        if seen >= expected_events {
            break;
        }
    }

    anyhow::ensure!(
        seen >= expected_events,
        "expected at least {expected_events} events, saw {seen}"
    );
    Ok((
        inserted_computations,
        inserted_allowed,
        inserted_pbs,
        cursor,
    ))
}

async fn seed_tenant_with_keys(
    pool: &sqlx::PgPool,
    tenant_api_key: &str,
    host_chain_id: i64,
) -> Result<i32> {
    let root = repo_root()?;
    let key_dir = root.join("coprocessor/fhevm-engine/fhevm-keys");
    let sks = tokio::fs::read(key_dir.join("sks"))
        .await
        .context("read sks")?;
    let pks = tokio::fs::read(key_dir.join("pks"))
        .await
        .context("read pks")?;
    let cks = tokio::fs::read(key_dir.join("cks"))
        .await
        .context("read cks")?;
    let public_params = tokio::fs::read(key_dir.join("pp"))
        .await
        .context("read pp")?;

    let tenant_id = sqlx::query_scalar::<_, i32>(
        "
        INSERT INTO tenants(
            tenant_api_key,
            chain_id,
            acl_contract_address,
            verifying_contract_address,
            pks_key,
            sks_key,
            public_params,
            cks_key
        )
        VALUES (
            $1::uuid,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8
        )
        RETURNING tenant_id
        ",
    )
    .bind(tenant_api_key)
    .bind(host_chain_id)
    .bind("0x339EcE85B9E11a3A3AA557582784a15d7F82AAf2")
    .bind("0x69dE3158643e738a0724418b21a35FAA20CBb1c5")
    .bind(pks)
    .bind(sks)
    .bind(public_params)
    .bind(cks)
    .fetch_one(pool)
    .await
    .context("insert tenant")?;
    Ok(tenant_id)
}

async fn start_docker_postgres(image: &str, tag: &str) -> Result<(String, DockerPostgres)> {
    let output = Command::new("docker")
        .args([
            "run",
            "-d",
            "-e",
            "POSTGRES_USER=postgres",
            "-e",
            "POSTGRES_PASSWORD=postgres",
            "-e",
            "POSTGRES_DB=coprocessor",
            "-p",
            "127.0.0.1::5432",
            &format!("{image}:{tag}"),
        ])
        .output()
        .context("start postgres docker container")?;
    anyhow::ensure!(
        output.status.success(),
        "docker run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let container_id = String::from_utf8(output.stdout)
        .context("decode docker run output")?
        .trim()
        .to_string();
    anyhow::ensure!(!container_id.is_empty(), "empty docker container id");

    let port_out = Command::new("docker")
        .args(["port", &container_id, "5432/tcp"])
        .output()
        .context("inspect docker postgres port")?;
    anyhow::ensure!(
        port_out.status.success(),
        "docker port failed: {}",
        String::from_utf8_lossy(&port_out.stderr)
    );
    let port_text = String::from_utf8(port_out.stdout)
        .context("decode docker port output")?
        .trim()
        .to_string();
    let host_port = port_text
        .rsplit(':')
        .next()
        .context("parse mapped postgres port")?
        .parse::<u16>()
        .context("mapped postgres port is not u16")?;
    let db_url = format!("postgresql://postgres:postgres@127.0.0.1:{host_port}/coprocessor");

    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        match PgPoolOptions::new()
            .max_connections(1)
            .connect(&db_url)
            .await
        {
            Ok(pool) => {
                pool.close().await;
                break;
            }
            Err(_) if Instant::now() < deadline => sleep(Duration::from_millis(500)).await,
            Err(err) => {
                let pg = DockerPostgres { container_id };
                pg.stop();
                return Err(err).context("postgres docker did not become ready");
            }
        }
    }

    Ok((db_url, DockerPostgres { container_id }))
}

fn repo_root() -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .join("../../..")
        .canonicalize()
        .context("resolve repo root")
}

fn expand_tilde(path: &str) -> String {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{home}/{rest}");
        }
    }
    path.to_string()
}
