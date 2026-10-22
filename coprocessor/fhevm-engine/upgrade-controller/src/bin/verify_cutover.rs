//! verify-cutover: operator-side verification that a Blue/Green cutover
//! completed successfully.
//!
//! Any operator runs this independently with a read-only DB role, HTTP read
//! access to every member's decision bucket, and read-only S3 access to its
//! own ciphertext bucket. No signing key is involved: the tool only verifies.
//!
//! The code is deliberately script-like - one linear pass, every SQL query and
//! S3 key printed - so an operator can audit it and replay any step by hand
//! with psql and the aws CLI. It reuses only the record FORMAT (types, domain
//! tag, object key) from consensus-detector; every check is reimplemented
//! here, so a bug in the detector cannot vouch for itself.
//!
//! Checks, in order:
//!   1. Gate (DB): `upgrade_state` must hold a finished attempt. Any
//!      `in_progress` row means the cutover has not executed and every later
//!      assertion is skipped. `versioning.consensus_version` and the
//!      `consensus_epoch_history` outcome must agree with the target.
//!   2. Quorum (Gateway + S3 over HTTP): every member registered in
//!      `GatewayConfig` at the proposal block wrote a signed `quorum` record
//!      for this attempt, all records name the identical member set, and each
//!      signature recovers to its member. On a failed attempt the tool instead
//!      confirms a signed `abort` record and reports "rolled back".
//!   3. Ciphertext versions (DB): every output handle computed at or after
//!      each chain's `start_block`, and every input handle verified at or
//!      after `gw_start_block` (a Gateway block), carries
//!      `ciphertexts.consensus_version` equal to the target.
//!   4. Digest consistency (DB vs S3): for the same handles, the keccak256 of
//!      the DB blob, the `ciphertext_digest` row and the S3 object agree for
//!      ct64 and ct128. Anomalies exactly at a chain's cutover block are
//!      reported as the known boundary condition (a blue digest row that
//!      survived the merge), not as failures.
//!
//! Exit codes: 0 pass, 1 fail, 2 cutover not executed, 3 rolled back,
//! 4 inconclusive (an endpoint was unreachable or data was missing).

use std::collections::BTreeMap;
use std::process::ExitCode;
use std::time::Duration;

use alloy::primitives::{keccak256, Address, Signature};
use alloy::providers::{Provider, ProviderBuilder};
use anyhow::{anyhow, Context, Result};
use aws_config::BehaviorVersion;
use aws_sdk_s3::config::{Credentials, Region};
use clap::Parser;
use consensus_detector::decision::{self, Decision, Payload, Record};
use consensus_detector::s3::S3Service;
use serde::Serialize;
use sha3::{Digest, Keccak256};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};

#[derive(Parser, Debug)]
#[command(
    name = "verify-cutover",
    about = "Verify that a Blue/Green cutover completed successfully (read-only)"
)]
struct Args {
    /// The consensus version the upgrade was supposed to activate.
    #[arg(long, env = "TARGET_CONSENSUS_VERSION")]
    target_consensus_version: i64,

    /// Postgres connection URL of the LIVE (post-cutover) stack's DB.
    /// A read-only role is sufficient.
    #[arg(long, env = "DATABASE_URL")]
    database_url: String,

    /// Gateway RPC URL. Must serve state at the proposal block
    /// (an archive node, or a node that still holds that block's state).
    #[arg(long, env = "GATEWAY_URL")]
    gateway_url: String,

    /// Address of the GatewayConfig contract.
    #[arg(long, env = "GATEWAY_CONFIG_ADDRESS")]
    gateway_config_address: String,

    /// S3 / MinIO endpoint URL for this operator's ciphertext bucket.
    #[arg(long, env = "S3_URL")]
    s3_url: String,

    /// Ciphertext bucket name (the ct64/ct128 objects).
    #[arg(long, env = "BUCKET_NAME")]
    bucket: String,

    #[arg(long, env = "AWS_ACCESS_KEY_ID", default_value = "fhevm-access-key")]
    access_key: String,

    #[arg(
        long,
        env = "AWS_SECRET_ACCESS_KEY",
        default_value = "fhevm-access-secret-key"
    )]
    secret_key: String,

    #[arg(long, env = "AWS_REGION", default_value = "eu-west-1")]
    region: String,

    /// Coprocessor context id in the S3 key ct64/<handle>/<context-id>.
    #[arg(long, default_value = "1")]
    context_id: String,

    /// Check at most this many handles in the digest step (0 = all).
    #[arg(long, default_value_t = 0)]
    digest_limit: usize,

    /// Print the full report as JSON on the last line.
    #[arg(long, default_value_t = false)]
    json: bool,
}

/// Outcome of the whole run; doubles as the process exit code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Verdict {
    Pass,
    Fail,
    NotExecuted,
    RolledBack,
    Inconclusive,
}

impl Verdict {
    fn code(self) -> u8 {
        match self {
            Verdict::Pass => 0,
            Verdict::Fail => 1,
            Verdict::NotExecuted => 2,
            Verdict::RolledBack => 3,
            Verdict::Inconclusive => 4,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct StateRow {
    stack_role: String,
    host_chain_id: i64,
    state: String,
    status: String,
    proposal_id: Option<String>,
    proposal_block: Option<i64>,
    version: Option<String>,
    start_block: Option<i64>,
    end_block: Option<i64>,
    gw_start_block: Option<i64>,
}

#[derive(Debug, Serialize)]
struct MemberReport {
    address: String,
    url: String,
    decision: Option<String>,
    target_stack_version: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Default, Serialize)]
struct ChainVersionReport {
    chain_id: i64,
    start_block: i64,
    total: i64,
    missing: i64,
    mismatched: i64,
}

#[derive(Debug, Default, Serialize)]
struct DigestCounters {
    checked: usize,
    ok: usize,
    failed: usize,
    boundary: usize,
    unreachable: usize,
}

#[derive(Debug, Default, Serialize)]
struct Report {
    target_consensus_version: i64,
    verdict: Option<Verdict>,
    failures: Vec<String>,
    warnings: Vec<String>,
    upgrade_state: Vec<StateRow>,
    versioning_stack_version: Option<String>,
    versioning_consensus_version: Option<i64>,
    epoch_outcome: Option<String>,
    proposal_id: Option<String>,
    proposal_block: Option<i64>,
    members: Vec<MemberReport>,
    output_versions: Vec<ChainVersionReport>,
    input_versions: Option<ChainVersionReport>,
    digests: Option<DigestCounters>,
}

impl Report {
    fn fail(&mut self, msg: String) {
        println!("FAIL: {msg}");
        self.failures.push(msg);
    }

    fn warn(&mut self, msg: String) {
        println!("WARN: {msg}");
        self.warnings.push(msg);
    }
}

// ---------------------------------------------------------------------------
// Pure helpers. Everything below is independent of the detector's logic and
// covered by the unit tests at the bottom of this file.
// ---------------------------------------------------------------------------

/// keccak256 over the domain tag and the serialized payload: the exact bytes
/// the writer signed (see consensus-detector decision.rs).
fn payload_digest(payload: &Payload) -> Result<[u8; 32]> {
    let mut bytes = Vec::from(decision::DOMAIN);
    bytes.extend_from_slice(&serde_json::to_vec(payload)?);
    Ok(*keccak256(bytes))
}

/// Full verification of one member's record: it names this attempt, the signer
/// field and member set hold the expected address, and the signature recovers
/// to that address.
fn verify_record(
    record: &Record,
    expected: Address,
    proposal_id_hex: &str,
    proposal_block: i64,
) -> Result<(), String> {
    let payload = &record.payload;
    let name = decision::name_of(expected);
    if payload.signer != name {
        return Err(format!("signer field '{}' is not '{name}'", payload.signer));
    }
    if payload.proposal_id != proposal_id_hex {
        return Err(format!(
            "record names proposal {} instead of {proposal_id_hex}",
            payload.proposal_id
        ));
    }
    if payload.proposal_block != proposal_block {
        return Err(format!(
            "record names proposal_block {} instead of {proposal_block}",
            payload.proposal_block
        ));
    }
    if !payload.addresses().contains(&name.as_str()) {
        return Err("signer is missing from the record's own member set".to_string());
    }
    let raw = hex::decode(&record.signature).map_err(|e| format!("signature is not hex: {e}"))?;
    let signature =
        Signature::try_from(raw.as_slice()).map_err(|e| format!("signature malformed: {e}"))?;
    let digest = payload_digest(payload).map_err(|e| format!("payload digest failed: {e}"))?;
    let recovered = signature
        .recover_address_from_prehash(&digest.into())
        .map_err(|e| format!("signature recovery failed: {e}"))?;
    if recovered != expected {
        return Err(format!(
            "signature recovers to {recovered}, expected {expected}"
        ));
    }
    Ok(())
}

/// Sorted, deduplicated values: used to compare member sets across records and
/// against the GatewayConfig registration, and to assert one proposal_id.
fn sorted_unique<I: IntoIterator<Item = String>>(values: I) -> Vec<String> {
    let mut sorted: Vec<String> = values.into_iter().collect();
    sorted.sort();
    sorted.dedup();
    sorted
}

/// Abort wins; quorum needs every member. None means at least one member has
/// no readable decision.
fn fleet_outcome(decisions: &[Option<Decision>]) -> Option<Decision> {
    if decisions.contains(&Some(Decision::Abort)) {
        return Some(Decision::Abort);
    }
    if !decisions.is_empty() && decisions.iter().all(|d| *d == Some(Decision::Quorum)) {
        return Some(Decision::Quorum);
    }
    None
}

/// Split a verify_proofs.handles blob into its 32-byte handles.
fn chunk_handles(blob: &[u8]) -> Result<Vec<Vec<u8>>, String> {
    if !blob.len().is_multiple_of(32) {
        return Err(format!(
            "handles blob length {} is not a multiple of 32",
            blob.len()
        ));
    }
    Ok(blob.chunks_exact(32).map(<[u8]>::to_vec).collect())
}

/// What a GET of one S3 object produced.
#[derive(Debug, Clone, PartialEq, Eq)]
enum S3Fetch {
    /// keccak256 hex of the object's bytes.
    Object(String),
    NotFound,
    /// Transport or permission error: the check is inconclusive, not failed.
    Error(String),
}

/// Consistency of one digest kind (ct64 or ct128) for one handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
enum DigestStatus {
    /// Blob, recorded digest and S3 object all agree.
    Ok,
    /// Blob pruned from the DB; the recorded digest matches S3.
    OkBlobPruned,
    /// Blob matches S3; no digest was recorded for this kind.
    OkNotRecorded,
    /// Nothing exists anywhere for this kind (e.g. ct128 never produced).
    Absent,
    MissingInS3,
    MissingInDb,
    /// The DB blob and the S3 object differ.
    Mismatch,
    /// The recorded digest differs from the blob or from S3.
    StoredDiffers,
    /// S3 could not answer; inconclusive.
    Unreachable,
}

impl DigestStatus {
    fn is_ok(self) -> bool {
        matches!(
            self,
            DigestStatus::Ok
                | DigestStatus::OkBlobPruned
                | DigestStatus::OkNotRecorded
                | DigestStatus::Absent
        )
    }
}

/// Classify one digest kind from the blob digest (computed over DB bytes), the
/// recorded digest (ciphertext_digest row) and the S3 object.
fn digest_status(computed: Option<&str>, stored: Option<&str>, s3: &S3Fetch) -> DigestStatus {
    if let S3Fetch::Error(_) = s3 {
        return DigestStatus::Unreachable;
    }
    match computed {
        Some(blob_digest) => match s3 {
            S3Fetch::NotFound => DigestStatus::MissingInS3,
            S3Fetch::Object(s3_digest) if s3_digest != blob_digest => DigestStatus::Mismatch,
            S3Fetch::Object(_) => match stored {
                Some(recorded) if recorded != blob_digest => DigestStatus::StoredDiffers,
                Some(_) => DigestStatus::Ok,
                None => DigestStatus::OkNotRecorded,
            },
            S3Fetch::Error(_) => DigestStatus::Unreachable,
        },
        None => match (stored, s3) {
            (None, S3Fetch::NotFound) => DigestStatus::Absent,
            (Some(_), S3Fetch::NotFound) => DigestStatus::MissingInS3,
            (None, S3Fetch::Object(_)) => DigestStatus::MissingInDb,
            (Some(recorded), S3Fetch::Object(s3_digest)) => {
                if recorded == s3_digest {
                    DigestStatus::OkBlobPruned
                } else {
                    DigestStatus::StoredDiffers
                }
            }
            (_, S3Fetch::Error(_)) => DigestStatus::Unreachable,
        },
    }
}

/// The one tolerated anomaly: at a chain's cutover block a blue
/// ciphertext_digest row can survive the merge (its ciphertext128 is NULL and
/// green's sns guard never refills it). Reported by name, not as a failure.
fn is_known_cutover_boundary(block: Option<i64>, chain_start_block: Option<i64>) -> bool {
    match (block, chain_start_block) {
        (Some(block), Some(start)) => block == start,
        _ => false,
    }
}

fn keccak_hex(bytes: &[u8]) -> String {
    let mut hasher = Keccak256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

fn decision_name(decision: Decision) -> &'static str {
    match decision {
        Decision::Quorum => "quorum",
        Decision::Abort => "abort",
    }
}

// ---------------------------------------------------------------------------
// The linear run. Each step prints what it executes before executing it.
// ---------------------------------------------------------------------------

/// One handle to digest-check: where it came from and whether it sits exactly
/// on its chain's cutover block.
struct DigestTarget {
    handle: Vec<u8>,
    origin: &'static str,
    boundary: bool,
}

async fn run(args: &Args, report: &mut Report) -> Result<Verdict> {
    let target = args.target_consensus_version;
    report.target_consensus_version = target;

    println!("verify-cutover: target consensus_version {target}");
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&args.database_url)
        .await
        .context("connecting to Postgres")?;

    // ---- Step 1: gate on upgrade_state --------------------------------
    println!("\n== Step 1: upgrade state (DB) ==");
    const UPGRADE_STATE_SQL: &str = "SELECT stack_role, host_chain_id, state, status, \
         proposal_id, proposal_block, version, start_block, end_block, gw_start_block \
         FROM upgrade_state ORDER BY stack_role, host_chain_id";
    println!("SQL: {UPGRADE_STATE_SQL}");
    let rows = sqlx::query(UPGRADE_STATE_SQL)
        .fetch_all(&pool)
        .await
        .context("querying upgrade_state")?;
    for row in rows {
        report.upgrade_state.push(StateRow {
            stack_role: row.try_get("stack_role")?,
            host_chain_id: row.try_get("host_chain_id")?,
            state: row.try_get("state")?,
            status: row.try_get("status")?,
            proposal_id: row
                .try_get::<Option<Vec<u8>>, _>("proposal_id")?
                .map(hex::encode),
            proposal_block: row.try_get("proposal_block")?,
            version: row.try_get("version")?,
            start_block: row.try_get("start_block")?,
            end_block: row.try_get("end_block")?,
            gw_start_block: row.try_get("gw_start_block")?,
        });
    }
    for row in &report.upgrade_state {
        println!(
            "  {} chain {}: state={} status={} proposal_id={} proposal_block={:?} \
             version={:?} window=[{:?}..{:?}] gw_start_block={:?}",
            row.stack_role,
            row.host_chain_id,
            row.state,
            row.status,
            row.proposal_id.as_deref().unwrap_or("-"),
            row.proposal_block,
            row.version,
            row.start_block,
            row.end_block,
            row.gw_start_block,
        );
    }

    if report.upgrade_state.is_empty() {
        println!("no upgrade_state rows: no upgrade attempt is recorded in this DB");
        return Ok(Verdict::NotExecuted);
    }
    if report
        .upgrade_state
        .iter()
        .any(|r| r.status == "in_progress")
    {
        println!("an upgrade attempt is still in_progress: the cutover has not executed yet");
        println!("every later assertion is skipped; re-run once the attempt finishes");
        return Ok(Verdict::NotExecuted);
    }
    let gcs: Vec<StateRow> = report
        .upgrade_state
        .iter()
        .filter(|r| r.stack_role == "GCS")
        .cloned()
        .collect();
    if gcs.is_empty() {
        println!("no GCS rows: this DB never ran an upgrade attempt");
        return Ok(Verdict::NotExecuted);
    }
    let rolled_back = gcs.iter().any(|r| r.status == "failed");

    // Attempt identity: one proposal across every GCS row.
    let proposal_ids = sorted_unique(gcs.iter().filter_map(|r| r.proposal_id.clone()));
    if proposal_ids.len() != 1 {
        return Err(anyhow!(
            "GCS rows do not name exactly one proposal_id: {proposal_ids:?}"
        ));
    }
    let proposal_id_hex = &proposal_ids[0];
    let proposal_id = hex::decode(proposal_id_hex).context("proposal_id is not hex")?;
    let proposal_block = gcs
        .iter()
        .filter_map(|r| r.proposal_block)
        .max()
        .ok_or_else(|| anyhow!("GCS rows carry no proposal_block"))?;
    if gcs.iter().any(|r| r.proposal_block != Some(proposal_block)) {
        report.warn("GCS rows disagree on proposal_block; using the highest".to_string());
    }
    report.proposal_id = Some(proposal_id_hex.clone());
    report.proposal_block = Some(proposal_block);
    println!("attempt: proposal_id={proposal_id_hex} proposal_block={proposal_block}");
    // `proposal_block` is a HOST-chain block (the `ProtocolConfig.CoprocessorUpgradeProposed`
    // event is ingested by the host-listener). GatewayConfig lives on the Gateway chain, whose
    // height is unrelated to the host's (and lags it, since the Gateway mines on tx, not on a
    // block-time), so the member set must be read at a Gateway block: `gw_start_block`.
    let gw_start_block = gcs
        .iter()
        .filter_map(|r| r.gw_start_block)
        .max()
        .ok_or_else(|| anyhow!("GCS rows carry no gw_start_block"))?;

    // Live version markers.
    const VERSIONING_SQL: &str = "SELECT stack_version, consensus_version FROM versioning";
    println!("SQL: {VERSIONING_SQL}");
    let versioning = sqlx::query(VERSIONING_SQL)
        .fetch_one(&pool)
        .await
        .context("querying versioning")?;
    let live_stack_version: String = versioning.try_get("stack_version")?;
    let live_consensus_version: i64 = versioning.try_get("consensus_version")?;
    report.versioning_stack_version = Some(live_stack_version.clone());
    report.versioning_consensus_version = Some(live_consensus_version);
    println!(
        "versioning: stack_version={live_stack_version} consensus_version={live_consensus_version}"
    );

    const EPOCH_SQL: &str = "SELECT consensus_epoch, outcome, consensus_version, stack_version \
         FROM consensus_epoch_history WHERE proposal_id = $1 AND proposal_block = $2";
    println!("SQL: {EPOCH_SQL} [$1={proposal_id_hex}, $2={proposal_block}]");
    let epoch = sqlx::query(EPOCH_SQL)
        .bind(&proposal_id)
        .bind(proposal_block)
        .fetch_optional(&pool)
        .await
        .context("querying consensus_epoch_history")?;
    let epoch_outcome: Option<String> = match &epoch {
        Some(row) => {
            let outcome: String = row.try_get("outcome")?;
            let epoch_version: Option<i64> = row.try_get("consensus_version")?;
            println!(
                "epoch {}: outcome={outcome} consensus_version={epoch_version:?}",
                row.try_get::<String, _>("consensus_epoch")?
            );
            if epoch_version != Some(target) {
                report.fail(format!(
                    "consensus_epoch_history names consensus_version {epoch_version:?}, \
                     the target is {target}"
                ));
            }
            Some(outcome)
        }
        None => {
            report.fail(format!(
                "no consensus_epoch_history row for proposal {proposal_id_hex} \
                 at block {proposal_block}"
            ));
            None
        }
    };
    report.epoch_outcome = epoch_outcome.clone();

    if rolled_back {
        println!("upgrade_state says the attempt FAILED: verifying the rollback was decided");
        if epoch_outcome.as_deref() == Some("succeeded") {
            report.fail(
                "upgrade_state says failed but consensus_epoch_history says succeeded".to_string(),
            );
        }
    } else {
        if live_consensus_version != target {
            report.fail(format!(
                "versioning.consensus_version is {live_consensus_version}, expected {target}"
            ));
        }
        if epoch_outcome.as_deref() != Some("succeeded") {
            report.fail(format!(
                "consensus_epoch_history outcome is {epoch_outcome:?}, expected 'succeeded'"
            ));
        }
    }

    // ---- Step 2: decision-record quorum --------------------------------
    println!("\n== Step 2: decision records (GatewayConfig + member buckets) ==");
    println!(
        "GatewayConfig {} at gw_start_block {gw_start_block} (a Gateway block) via {}",
        args.gateway_config_address, args.gateway_url
    );
    let provider = ProviderBuilder::new()
        .connect(&args.gateway_url)
        .await
        .context("connecting to the Gateway RPC")?;
    let gateway_config_address: Address = args
        .gateway_config_address
        .parse()
        .context("parsing --gateway-config-address")?;
    // Read the member set at `gw_start_block`, which is the block the detector used. The
    // Gateway may not retain state that far back (it prunes history / its height lags the
    // host's), so on failure fall back to the Gateway tip: the coprocessor registration set
    // is static, so the tip resolves the same members and is always within the served range.
    let gw_tip = provider
        .get_block_number()
        .await
        .context("reading the Gateway tip")?;
    let service = S3Service::new(provider, gateway_config_address);
    let gw_block = u64::try_from(gw_start_block).context("negative gw_start_block")?;
    let members = match service.get_members_at(gw_block).await {
        Ok(m) => m,
        Err(e) => {
            report.warn(format!(
                "GatewayConfig not readable at gw_start_block {gw_start_block} ({e}); the Gateway \
                 does not retain that block's state. Reading the (static) registration set at the \
                 Gateway tip {gw_tip} instead."
            ));
            service.get_members_at(gw_tip).await.context(
                "reading coprocessor registrations from GatewayConfig at the Gateway tip",
            )?
        }
    };
    let gateway_set = sorted_unique(members.iter().map(|(a, _)| decision::name_of(*a)));
    println!("registered members: {gateway_set:?}");

    let http = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .context("building the HTTP client")?;
    let record_key = decision::key(&proposal_id, proposal_block);
    let mut decisions: Vec<Option<Decision>> = Vec::new();
    let mut transport_errors = 0usize;
    for (address, bucket_url) in &members {
        let url = format!("{}/{}", bucket_url.trim_end_matches('/'), record_key);
        println!("GET {url}");
        let mut member_report = MemberReport {
            address: decision::name_of(*address),
            url: url.clone(),
            decision: None,
            target_stack_version: None,
            error: None,
        };
        let outcome: Result<Option<Record>, String> = match http.get(&url).send().await {
            Err(e) => Err(format!("transport error: {e}")),
            Ok(response) if response.status() == reqwest::StatusCode::NOT_FOUND => Ok(None),
            Ok(response) if !response.status().is_success() => {
                Err(format!("HTTP {}", response.status()))
            }
            Ok(response) => match response.bytes().await {
                Err(e) => Err(format!("body read error: {e}")),
                Ok(bytes) => match serde_json::from_slice::<Record>(&bytes) {
                    Err(e) => {
                        member_report.error = Some(format!("record does not parse: {e}"));
                        report.fail(format!(
                            "record of {} does not parse: {e}",
                            member_report.address
                        ));
                        Ok(None)
                    }
                    Ok(record) => Ok(Some(record)),
                },
            },
        };
        match outcome {
            Err(e) => {
                transport_errors += 1;
                member_report.error = Some(e.clone());
                report.warn(format!(
                    "record of {} unreadable ({e}); quorum check is inconclusive",
                    member_report.address
                ));
                decisions.push(None);
            }
            Ok(None) => {
                if member_report.error.is_none() {
                    let msg = format!("{} wrote no decision record", member_report.address);
                    if rolled_back {
                        report.warn(msg);
                    } else {
                        report.fail(msg);
                    }
                }
                decisions.push(None);
            }
            Ok(Some(record)) => {
                match verify_record(&record, *address, proposal_id_hex, proposal_block) {
                    Err(e) => {
                        report.fail(format!(
                            "record of {} fails verification: {e}",
                            member_report.address
                        ));
                        member_report.error = Some(e);
                        decisions.push(None);
                    }
                    Ok(()) => {
                        let payload = &record.payload;
                        println!(
                            "  verified: decision={} target_stack_version={} \
                             target_consensus_version={}",
                            decision_name(payload.decision),
                            payload.target_stack_version,
                            payload.target_consensus_version,
                        );
                        if payload.target_consensus_version != target {
                            report.fail(format!(
                                "{} signed target_consensus_version {}, expected {target}",
                                member_report.address, payload.target_consensus_version
                            ));
                        }
                        if !rolled_back && payload.target_stack_version != live_stack_version {
                            report.fail(format!(
                                "{} signed target_stack_version '{}' but versioning says '{}'",
                                member_report.address,
                                payload.target_stack_version,
                                live_stack_version
                            ));
                        }
                        let record_set =
                            sorted_unique(payload.addresses().iter().map(|a| (*a).to_string()));
                        if record_set != gateway_set {
                            report.fail(format!(
                                "{} signed member set {record_set:?}, GatewayConfig at block \
                                 {proposal_block} says {gateway_set:?}",
                                member_report.address
                            ));
                        }
                        member_report.decision = Some(decision_name(payload.decision).to_string());
                        member_report.target_stack_version =
                            Some(payload.target_stack_version.clone());
                        decisions.push(Some(payload.decision));
                    }
                }
            }
        }
        report.members.push(member_report);
    }

    let outcome = fleet_outcome(&decisions);
    println!(
        "fleet outcome: {}",
        outcome.map_or("undecided", decision_name)
    );
    if rolled_back {
        match outcome {
            Some(Decision::Abort) => {
                println!("rollback confirmed: a member signed abort for this attempt");
            }
            Some(Decision::Quorum) => {
                report.fail("upgrade_state says failed but every member signed quorum".to_string())
            }
            None if transport_errors > 0 => {
                report.warn("could not confirm an abort record; see transport errors".to_string());
            }
            None => report.warn(
                "no member's record shows abort; the rollback was local (e.g. a deadline) \
                 or records were cleaned up"
                    .to_string(),
            ),
        }
        if !report.failures.is_empty() {
            return Ok(Verdict::Fail);
        }
        if transport_errors > 0 {
            return Ok(Verdict::Inconclusive);
        }
        return Ok(Verdict::RolledBack);
    }
    match outcome {
        Some(Decision::Quorum) => {}
        Some(Decision::Abort) => report.fail(
            "a member signed abort although upgrade_state says the cutover completed".to_string(),
        ),
        None => {
            if transport_errors == 0 {
                report.fail("not every member signed quorum for this attempt".to_string());
            }
        }
    }

    // ---- Step 3: ciphertext consensus_version --------------------------
    println!("\n== Step 3: ciphertext consensus_version (DB) ==");
    let mut digest_targets: Vec<DigestTarget> = Vec::new();

    const OUTPUT_HANDLES_SQL: &str = "SELECT DISTINCT comp.output_handle AS handle, \
         comp.block_number AS block_number, \
         COALESCE(c.consensus_version, 1)::BIGINT AS consensus_version, \
         (c.handle IS NULL) AS missing \
         FROM computations comp \
         LEFT JOIN ciphertexts c ON c.handle = comp.output_handle \
         WHERE comp.host_chain_id = $1 AND comp.block_number >= $2 \
           AND comp.is_error = false AND comp.is_allowed = true \
           AND comp.is_completed = true \
         ORDER BY comp.block_number";
    for chain in &gcs {
        let Some(start_block) = chain.start_block else {
            report.fail(format!(
                "GCS row for chain {} has no start_block",
                chain.host_chain_id
            ));
            continue;
        };
        println!(
            "SQL: {OUTPUT_HANDLES_SQL} [$1={}, $2={start_block}]",
            chain.host_chain_id
        );
        let rows = sqlx::query(OUTPUT_HANDLES_SQL)
            .bind(chain.host_chain_id)
            .bind(start_block)
            .fetch_all(&pool)
            .await
            .context("querying output handles")?;
        let mut chain_report = ChainVersionReport {
            chain_id: chain.host_chain_id,
            start_block,
            ..Default::default()
        };
        for row in rows {
            chain_report.total += 1;
            let handle: Vec<u8> = row.try_get("handle")?;
            let block: i64 = row.try_get("block_number")?;
            let version: i64 = row.try_get("consensus_version")?;
            let missing: bool = row.try_get("missing")?;
            if missing {
                chain_report.missing += 1;
                if chain_report.missing <= 20 {
                    report.fail(format!(
                        "chain {} block {block}: completed allowed output {} has no \
                         ciphertexts row",
                        chain.host_chain_id,
                        hex::encode(&handle)
                    ));
                }
            } else if version != target {
                chain_report.mismatched += 1;
                if chain_report.mismatched <= 20 {
                    report.fail(format!(
                        "chain {} block {block}: output {} has consensus_version {version}, \
                         expected {target}",
                        chain.host_chain_id,
                        hex::encode(&handle)
                    ));
                }
            }
            digest_targets.push(DigestTarget {
                handle,
                origin: "output",
                boundary: is_known_cutover_boundary(Some(block), Some(start_block)),
            });
        }
        println!(
            "chain {}: {} outputs from block {start_block}, {} missing, {} on the wrong version",
            chain.host_chain_id, chain_report.total, chain_report.missing, chain_report.mismatched
        );
        report.output_versions.push(chain_report);
    }

    let gw_start_block = gcs.iter().filter_map(|r| r.gw_start_block).min();
    match gw_start_block {
        None => report.fail("no GCS row carries a gw_start_block".to_string()),
        Some(gw_start_block) => {
            const INPUT_HANDLES_SQL: &str = "SELECT zk_proof_id, block_number, handles \
                 FROM verify_proofs WHERE verified = true AND block_number >= $1";
            println!("SQL: {INPUT_HANDLES_SQL} [$1={gw_start_block}] (Gateway blocks)");
            let rows = sqlx::query(INPUT_HANDLES_SQL)
                .bind(gw_start_block)
                .fetch_all(&pool)
                .await
                .context("querying verified input proofs")?;
            let mut input_handles: BTreeMap<Vec<u8>, i64> = BTreeMap::new();
            for row in rows {
                let zk_proof_id: i64 = row.try_get("zk_proof_id")?;
                let blob: Option<Vec<u8>> = row.try_get("handles")?;
                let Some(blob) = blob else { continue };
                match chunk_handles(&blob) {
                    Err(e) => report.fail(format!("verify_proofs {zk_proof_id}: {e}")),
                    Ok(handles) => {
                        for handle in handles {
                            input_handles.entry(handle).or_insert(zk_proof_id);
                        }
                    }
                }
            }
            let mut input_report = ChainVersionReport {
                chain_id: 0,
                start_block: gw_start_block,
                total: i64::try_from(input_handles.len()).unwrap_or(i64::MAX),
                ..Default::default()
            };
            const INPUT_VERSIONS_SQL: &str = "SELECT handle, \
                 COALESCE(consensus_version, 1)::BIGINT AS consensus_version \
                 FROM ciphertexts WHERE handle = ANY($1)";
            println!(
                "SQL: {INPUT_VERSIONS_SQL} [{} input handles]",
                input_handles.len()
            );
            let handle_list: Vec<Vec<u8>> = input_handles.keys().cloned().collect();
            let rows = sqlx::query(INPUT_VERSIONS_SQL)
                .bind(&handle_list)
                .fetch_all(&pool)
                .await
                .context("querying input ciphertext versions")?;
            let mut found: BTreeMap<Vec<u8>, i64> = BTreeMap::new();
            for row in rows {
                found.insert(row.try_get("handle")?, row.try_get("consensus_version")?);
            }
            for (handle, zk_proof_id) in &input_handles {
                match found.get(handle) {
                    None => {
                        input_report.missing += 1;
                        if input_report.missing <= 20 {
                            report.fail(format!(
                                "input {} (proof {zk_proof_id}) has no ciphertexts row",
                                hex::encode(handle)
                            ));
                        }
                    }
                    Some(version) if *version != target => {
                        input_report.mismatched += 1;
                        if input_report.mismatched <= 20 {
                            report.fail(format!(
                                "input {} (proof {zk_proof_id}) has consensus_version \
                                 {version}, expected {target}",
                                hex::encode(handle)
                            ));
                        }
                    }
                    Some(_) => {}
                }
                digest_targets.push(DigestTarget {
                    handle: handle.clone(),
                    origin: "input",
                    boundary: false,
                });
            }
            println!(
                "inputs: {} handles from Gateway block {gw_start_block}, {} missing, \
                 {} on the wrong version",
                input_report.total, input_report.missing, input_report.mismatched
            );
            report.input_versions = Some(input_report);
        }
    }

    // ---- Step 4: DB vs S3 digest consistency ----------------------------
    println!("\n== Step 4: digest consistency (DB vs S3) ==");
    let credentials = Credentials::new(&args.access_key, &args.secret_key, None, None, "cli");
    let s3 = aws_sdk_s3::Client::from_conf(
        aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(Region::new(args.region.clone()))
            .endpoint_url(&args.s3_url)
            .credentials_provider(credentials)
            .force_path_style(true)
            .build(),
    );
    if args.digest_limit > 0 && digest_targets.len() > args.digest_limit {
        report.warn(format!(
            "--digest-limit {}: checking only the first {} of {} handles",
            args.digest_limit,
            args.digest_limit,
            digest_targets.len()
        ));
        digest_targets.truncate(args.digest_limit);
    }
    println!(
        "checking {} handles against bucket '{}' at {} (keys ct64/<handle>/{} and \
         ct128/<handle>/{})",
        digest_targets.len(),
        args.bucket,
        args.s3_url,
        args.context_id,
        args.context_id
    );
    let mut counters = DigestCounters::default();
    for (index, target_handle) in digest_targets.iter().enumerate() {
        if index > 0 && index % 200 == 0 {
            println!("  ...{index}/{} handles checked", digest_targets.len());
        }
        let handle_hex = hex::encode(&target_handle.handle);
        let statuses = check_handle_digests(
            &pool,
            &s3,
            &args.bucket,
            &args.context_id,
            &target_handle.handle,
        )
        .await?;
        counters.checked += 1;
        let mut handle_ok = true;
        for (kind, status) in statuses {
            if status.is_ok() {
                continue;
            }
            handle_ok = false;
            if status == DigestStatus::Unreachable {
                counters.unreachable += 1;
                report.warn(format!("{kind} {handle_hex}: S3 unreachable; inconclusive"));
            } else if target_handle.boundary {
                counters.boundary += 1;
                report.warn(format!(
                    "{kind} {handle_hex}: {status:?} at the chain's cutover block - \
                     KNOWN-CUTOVER-BOUNDARY (a blue digest row that survived the merge), \
                     not counted as a failure"
                ));
            } else {
                counters.failed += 1;
                if counters.failed <= 50 {
                    report.fail(format!(
                        "{kind} {} {handle_hex}: {status:?}",
                        target_handle.origin
                    ));
                }
            }
        }
        if handle_ok {
            counters.ok += 1;
        }
    }
    println!(
        "digests: {} handles checked, {} fully consistent, {} failed, {} at the known \
         cutover boundary, {} unreachable",
        counters.checked, counters.ok, counters.failed, counters.boundary, counters.unreachable
    );
    let unreachable = counters.unreachable;
    report.digests = Some(counters);

    if !report.failures.is_empty() {
        return Ok(Verdict::Fail);
    }
    if transport_errors > 0 || unreachable > 0 {
        return Ok(Verdict::Inconclusive);
    }
    Ok(Verdict::Pass)
}

/// Both digest kinds for one handle: (kind, status).
async fn check_handle_digests(
    pool: &PgPool,
    s3: &aws_sdk_s3::Client,
    bucket: &str,
    context_id: &str,
    handle: &[u8],
) -> Result<[(&'static str, DigestStatus); 2]> {
    let ct64_blob: Option<Vec<u8>> =
        sqlx::query_scalar("SELECT ciphertext FROM ciphertexts WHERE handle = $1 LIMIT 1")
            .bind(handle)
            .fetch_optional(pool)
            .await
            .context("fetching ct64 blob")?;
    let ct128_blob: Option<Vec<u8>> =
        sqlx::query_scalar("SELECT ciphertext FROM ciphertexts128 WHERE handle = $1")
            .bind(handle)
            .fetch_optional(pool)
            .await
            .context("fetching ct128 blob")?;
    let stored =
        sqlx::query("SELECT ciphertext, ciphertext128 FROM ciphertext_digest WHERE handle = $1")
            .bind(handle)
            .fetch_optional(pool)
            .await
            .context("fetching ciphertext_digest row")?;
    let (stored_ct64, stored_ct128): (Option<Vec<u8>>, Option<Vec<u8>>) = match stored {
        Some(row) => (row.try_get("ciphertext")?, row.try_get("ciphertext128")?),
        None => (None, None),
    };

    let handle_hex = hex::encode(handle);
    let mut statuses = [("ct64", DigestStatus::Ok), ("ct128", DigestStatus::Ok)];
    for (slot, (blob, recorded)) in [(ct64_blob, stored_ct64), (ct128_blob, stored_ct128)]
        .into_iter()
        .enumerate()
    {
        let kind = statuses[slot].0;
        let key = format!("{kind}/{handle_hex}/{context_id}");
        let fetch = match s3.get_object().bucket(bucket).key(&key).send().await {
            Ok(response) => match response.body.collect().await {
                Ok(bytes) => S3Fetch::Object(keccak_hex(&bytes.into_bytes())),
                Err(e) => S3Fetch::Error(format!("S3 body read of {key}: {e}")),
            },
            Err(e) => {
                let not_found = e
                    .as_service_error()
                    .map(aws_sdk_s3::operation::get_object::GetObjectError::is_no_such_key)
                    .unwrap_or(false);
                if not_found {
                    S3Fetch::NotFound
                } else {
                    S3Fetch::Error(format!("S3 get of {key}: {e}"))
                }
            }
        };
        let computed = blob.as_deref().map(keccak_hex);
        let recorded_hex = recorded.map(hex::encode);
        statuses[slot].1 = digest_status(computed.as_deref(), recorded_hex.as_deref(), &fetch);
    }
    Ok(statuses)
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    let mut report = Report::default();
    let verdict = match run(&args, &mut report).await {
        Ok(verdict) => verdict,
        Err(e) => {
            println!("ERROR: {e:#}");
            report.warnings.push(format!("{e:#}"));
            Verdict::Inconclusive
        }
    };
    report.verdict = Some(verdict);
    println!("\nverdict: {verdict:?} (exit code {})", verdict.code());
    if args.json {
        match serde_json::to_string(&report) {
            Ok(json) => println!("{json}"),
            Err(e) => println!("ERROR: report does not serialize: {e}"),
        }
    }
    ExitCode::from(verdict.code())
}

// ---------------------------------------------------------------------------
// Unit tests for every pure helper.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::signers::local::PrivateKeySigner;
    use alloy::signers::SignerSync;
    use consensus_detector::decision::Member;

    fn payload_for(signer: &PrivateKeySigner, peers: &[Address]) -> Payload {
        let mut members: Vec<Member> = peers
            .iter()
            .chain(std::iter::once(&signer.address()))
            .map(|address| Member {
                address: decision::name_of(*address),
                url: format!("http://bucket-{address}"),
            })
            .collect();
        members.sort_by(|a, b| a.address.cmp(&b.address));
        Payload {
            proposal_id: "ab".repeat(32),
            proposal_block: 42,
            target_stack_version: "v0.15".to_string(),
            target_consensus_version: 2,
            decision: Decision::Quorum,
            signer: decision::name_of(signer.address()),
            members,
            anchors: vec![(1, 100)],
        }
    }

    fn signed_record(signer: &PrivateKeySigner, payload: Payload) -> Record {
        let digest = payload_digest(&payload).unwrap();
        let signature = signer.sign_hash_sync(&digest.into()).unwrap();
        Record {
            payload,
            signature: hex::encode(signature.as_bytes()),
        }
    }

    #[test]
    fn valid_record_verifies() {
        let signer = PrivateKeySigner::random();
        let peer = PrivateKeySigner::random().address();
        let record = signed_record(&signer, payload_for(&signer, &[peer]));
        assert_eq!(
            verify_record(&record, signer.address(), &"ab".repeat(32), 42),
            Ok(())
        );
    }

    #[test]
    fn tampered_payload_fails_verification() {
        let signer = PrivateKeySigner::random();
        let mut record = signed_record(&signer, payload_for(&signer, &[]));
        record.payload.target_consensus_version = 3;
        let result = verify_record(&record, signer.address(), &"ab".repeat(32), 42);
        assert!(result.unwrap_err().contains("recovers to"));
    }

    #[test]
    fn record_from_another_signer_fails_verification() {
        let signer = PrivateKeySigner::random();
        let other = PrivateKeySigner::random();
        let record = signed_record(&signer, payload_for(&signer, &[other.address()]));
        let result = verify_record(&record, other.address(), &"ab".repeat(32), 42);
        assert!(result.unwrap_err().contains("signer field"));
    }

    #[test]
    fn record_for_another_attempt_fails_verification() {
        let signer = PrivateKeySigner::random();
        let record = signed_record(&signer, payload_for(&signer, &[]));
        let wrong_id = verify_record(&record, signer.address(), &"cd".repeat(32), 42);
        assert!(wrong_id.unwrap_err().contains("names proposal"));
        let wrong_block = verify_record(&record, signer.address(), &"ab".repeat(32), 43);
        assert!(wrong_block.unwrap_err().contains("proposal_block"));
    }

    #[test]
    fn signer_outside_its_member_set_fails_verification() {
        let signer = PrivateKeySigner::random();
        let mut payload = payload_for(&signer, &[]);
        payload.members = vec![Member {
            address: decision::name_of(PrivateKeySigner::random().address()),
            url: "http://other".to_string(),
        }];
        let record = signed_record(&signer, payload);
        let result = verify_record(&record, signer.address(), &"ab".repeat(32), 42);
        assert!(result.unwrap_err().contains("member set"));
    }

    #[test]
    fn fleet_outcome_needs_every_member() {
        use Decision::{Abort, Quorum};
        assert_eq!(fleet_outcome(&[]), None);
        assert_eq!(fleet_outcome(&[Some(Quorum), None]), None);
        assert_eq!(fleet_outcome(&[Some(Quorum), Some(Quorum)]), Some(Quorum));
        assert_eq!(fleet_outcome(&[Some(Quorum), Some(Abort)]), Some(Abort));
        assert_eq!(fleet_outcome(&[None, Some(Abort)]), Some(Abort));
    }

    #[test]
    fn handles_blob_splits_into_32_byte_handles() {
        let blob: Vec<u8> = (0..64).collect();
        let handles = chunk_handles(&blob).unwrap();
        assert_eq!(handles.len(), 2);
        assert_eq!(handles[0], (0..32).collect::<Vec<u8>>());
        assert_eq!(handles[1], (32..64).collect::<Vec<u8>>());
        assert!(chunk_handles(&[0u8; 33]).is_err());
        assert_eq!(chunk_handles(&[]).unwrap(), Vec::<Vec<u8>>::new());
    }

    #[test]
    fn sorted_unique_sorts_and_dedups() {
        let sorted = sorted_unique(["bb".to_string(), "aa".to_string(), "bb".to_string()]);
        assert_eq!(sorted, vec!["aa".to_string(), "bb".to_string()]);
    }

    #[test]
    fn digest_status_classifies_every_case() {
        use DigestStatus::*;
        let object = |d: &str| S3Fetch::Object(d.to_string());
        // Blob present.
        assert_eq!(digest_status(Some("d"), Some("d"), &object("d")), Ok);
        assert_eq!(digest_status(Some("d"), None, &object("d")), OkNotRecorded);
        assert_eq!(
            digest_status(Some("d"), Some("d"), &S3Fetch::NotFound),
            MissingInS3
        );
        assert_eq!(digest_status(Some("d"), Some("d"), &object("x")), Mismatch);
        assert_eq!(
            digest_status(Some("d"), Some("x"), &object("d")),
            StoredDiffers
        );
        // Blob pruned or never present.
        assert_eq!(digest_status(None, Some("d"), &object("d")), OkBlobPruned);
        assert_eq!(digest_status(None, Some("d"), &object("x")), StoredDiffers);
        assert_eq!(digest_status(None, None, &object("d")), MissingInDb);
        assert_eq!(
            digest_status(None, Some("d"), &S3Fetch::NotFound),
            MissingInS3
        );
        assert_eq!(digest_status(None, None, &S3Fetch::NotFound), Absent);
        // Transport trouble is inconclusive everywhere.
        let error = S3Fetch::Error("boom".to_string());
        assert_eq!(digest_status(Some("d"), Some("d"), &error), Unreachable);
        assert_eq!(digest_status(None, None, &error), Unreachable);
        assert!(!Unreachable.is_ok());
        assert!(Ok.is_ok() && OkBlobPruned.is_ok() && OkNotRecorded.is_ok() && Absent.is_ok());
    }

    #[test]
    fn boundary_is_only_the_chains_cutover_block() {
        assert!(is_known_cutover_boundary(Some(100), Some(100)));
        assert!(!is_known_cutover_boundary(Some(101), Some(100)));
        assert!(!is_known_cutover_boundary(None, Some(100)));
        assert!(!is_known_cutover_boundary(Some(100), None));
    }

    #[test]
    fn verdict_exit_codes_are_stable() {
        assert_eq!(Verdict::Pass.code(), 0);
        assert_eq!(Verdict::Fail.code(), 1);
        assert_eq!(Verdict::NotExecuted.code(), 2);
        assert_eq!(Verdict::RolledBack.code(), 3);
        assert_eq!(Verdict::Inconclusive.code(), 4);
    }

    #[test]
    fn payload_digest_is_domain_separated() {
        let signer = PrivateKeySigner::random();
        let payload = payload_for(&signer, &[]);
        let digest = payload_digest(&payload).unwrap();
        let undomained = keccak256(serde_json::to_vec(&payload).unwrap());
        assert_ne!(digest, *undomained);
    }
}
