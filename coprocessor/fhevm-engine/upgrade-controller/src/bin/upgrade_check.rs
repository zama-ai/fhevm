//! upgrade-check: operator-side, read-only check of a Blue/Green upgrade
//! attempt. It tells apart an attempt that cut over, one that rolled back and
//! one that has not executed yet, and for a cutover it checks the result.
//!
//! Any operator runs this independently with a read-only DB role, a Gateway
//! RPC, HTTP read access to every member's decision bucket, and read-only S3
//! access to its own ciphertext bucket. No signing key is involved: the tool
//! only reads and verifies. See `upgrade-check.md` next to this crate's README.
//!
//! The code is deliberately script-like - one linear pass, every SQL query and
//! S3 key printed - so an operator can audit it and replay any step by hand
//! with psql and the aws CLI. It reuses the record FORMAT (types, domain tag,
//! object key) and the `GatewayConfig` member lookup from consensus-detector;
//! every check is reimplemented here, so a bug in the detector's checking
//! logic cannot vouch for itself.
//!
//! Checks, in order:
//!   1. Gate (DB): `upgrade_state` must hold a finished attempt. No rows, no
//!      GCS rows or any `in_progress` row means the cutover has not executed:
//!      the rows are printed and every later check is skipped. For a
//!      completed attempt, `versioning.consensus_version` and the
//!      `consensus_epoch_history` row must agree with the target.
//!   2. Decision records (Gateway + S3 over HTTP): for every member
//!      registered in `GatewayConfig` (read at `gw_start_block`, or at the
//!      Gateway tip when that block's state is no longer served), read its
//!      signed record (`outcome.json`) and the records it republished
//!      (`seen.json`). Each record is verified against its signer; invalid
//!      ones are dropped and the serving bucket is named. The valid records
//!      are counted against the fleet thresholds: with
//!      `max_faulty = (members - 1) / 3`, a cutover needs `quorum` from
//!      `members - max_faulty` members and a rollback needs `abort` from
//!      `2 * max_faulty + 1`. A completed attempt must reach the cutover
//!      threshold. On a failed attempt the tool instead looks for the
//!      rollback threshold, reports "rolled back" and stops.
//!      The same records expose misbehavior: a member with a valid `quorum`
//!      AND a valid `abort` record is reported as dishonest with both signed
//!      records as proof (exit code 5); a member whose record a peer relays
//!      while its own bucket serves nothing or another record is a suspect
//!      (no proof); a member that signed for another release or member set is
//!      a mismatch and is not counted.
//!   3. Ciphertext versions (DB): every allowed, completed output handle
//!      computed at or after each chain's `start_block`, and every input
//!      handle verified at or after `gw_start_block` (a Gateway block), carries
//!      `ciphertexts.consensus_version` equal to the target. The scan stops
//!      at the cutover time (`consensus_epoch_history.completed_at`) plus
//!      `--post-cutover-duration` (default 60m), measured by this operator's
//!      ingest times. A host chain with no block ingested after that point,
//!      or a gw-listener that has not scanned the Gateway up to it, makes the
//!      run inconclusive. A quiet Gateway (no block past that point) is fine.
//!   4. Digest consistency (DB vs S3): for those outputs and for the inputs
//!      present in `allowed_handles`, the keccak256 of the DB blob, the
//!      `ciphertext_digest` row and the S3 object agree for ct64 and ct128.
//!      Anomalies on an output computed exactly at a chain's `start_block` are
//!      reported as the known boundary condition (a blue digest row that
//!      survived the merge), not as failures.
//!
//! Exit codes: 0 pass, 1 fail, 2 cutover not executed, 3 rolled back,
//! 4 inconclusive (an endpoint was unreachable or data was missing),
//! 5 a member is proven dishonest (and no check failed).

use std::collections::{BTreeMap, BTreeSet};
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
    name = "upgrade-check",
    about = "Check a Blue/Green upgrade attempt: cut over, rolled back or not executed (read-only)"
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

    /// How far past the cutover the checks reach. Steps 3 and 4 cover what
    /// this operator ingested up to the cutover time
    /// (`consensus_epoch_history.completed_at`) plus this duration. If a chain
    /// has not been ingested past that point yet, the run is inconclusive.
    #[arg(long, default_value = "60m", value_parser = humantime::parse_duration)]
    post_cutover_duration: Duration,

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
    /// A member provably signed both `quorum` and `abort` for this attempt.
    DishonestMember,
}

impl Verdict {
    fn code(self) -> u8 {
        match self {
            Verdict::Pass => 0,
            Verdict::Fail => 1,
            Verdict::NotExecuted => 2,
            Verdict::RolledBack => 3,
            Verdict::Inconclusive => 4,
            Verdict::DishonestMember => 5,
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

/// What a member's own bucket served as its `outcome.json`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum OwnBucket {
    /// A record that verifies as this member's.
    Valid,
    /// No object (HTTP 404).
    #[default]
    Absent,
    /// Transport or HTTP error.
    Unreadable,
    /// An object that does not parse or does not verify.
    Invalid,
}

/// Everything the buckets say about one member's vote.
#[derive(Debug, Default, Serialize)]
struct MemberVotes {
    address: String,
    own_bucket: OwnBucket,
    /// Where a counted `quorum` record of this member was served
    /// (`<bucket owner>/<object>`). Empty = no counted `quorum` record.
    quorum_sources: Vec<String>,
    /// Same for `abort`.
    abort_sources: Vec<String>,
    /// Proof: counted `quorum` AND `abort` records, both signed by the member.
    dishonest: bool,
    /// No proof: a peer relays this member's record, but its own bucket serves
    /// nothing or a different record. Can also be an outage.
    suspect: Option<String>,
    /// The member signed for another release or member set; not counted.
    mismatch: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quorum_record: Option<Record>,
    #[serde(skip_serializing_if = "Option::is_none")]
    abort_record: Option<Record>,
}

#[derive(Debug, Default, Serialize)]
struct ChainVersionReport {
    chain_id: i64,
    start_block: i64,
    /// Last host block of the output scan (inclusive). None for the inputs,
    /// which are bounded by ingest time instead.
    end_block: Option<i64>,
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
    members: Vec<MemberVotes>,
    quorum_votes: usize,
    abort_votes: usize,
    cutover_threshold: usize,
    abort_threshold: usize,
    dishonest_members: Vec<String>,
    /// Highest consensus anchor block per chain across the verified quorum
    /// records (host chains and the Gateway). Informational.
    anchors: BTreeMap<i64, i64>,
    /// `consensus_epoch_history.completed_at`: when this operator cut over.
    cutover_time: Option<String>,
    /// cutover_time + --post-cutover-duration: where steps 3 and 4 stop.
    scan_cutoff: Option<String>,
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
    // The signature does not cover the size, and the detector refuses a larger
    // record, so a larger one never counted for any operator.
    if serde_json::to_vec(record).map_or(true, |body| body.len() > MAX_RECORD_BYTES) {
        return Err(format!("record is larger than {MAX_RECORD_BYTES} bytes"));
    }
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

/// The largest record the detector counts, as serialized.
const MAX_RECORD_BYTES: usize = 16 * 1024;
const OUTCOME_OBJECT: &str = "outcome.json";
const RELAY_OBJECT: &str = "seen.json";

/// The largest relay the detector accepts: two records per member.
fn max_relay_bytes(members: usize) -> usize {
    2 * members * (MAX_RECORD_BYTES + 1) + 2
}

/// How many of `members` may be faulty: `members > 3 * max_faulty`.
/// 0 of 3, 1 of 4, 1 of 5, 2 of 7.
fn max_faulty(members: usize) -> usize {
    members.saturating_sub(1) / 3
}

/// `quorum` records needed to cut over: `members - max_faulty`.
fn cutover_threshold(members: usize) -> usize {
    members - max_faulty(members)
}

/// `abort` records needed to roll back: `2 * max_faulty + 1`.
fn abort_threshold(members: usize) -> usize {
    2 * max_faulty(members) + 1
}

/// The fleet decision from the number of members that signed each decision.
/// Abort is checked first: if both thresholds are reached the fault assumption
/// is broken and the fleet stays on blue. None = nothing decided.
fn fleet_outcome(quorums: usize, aborts: usize, members: usize) -> Option<Decision> {
    if members == 0 {
        return None;
    }
    if aborts >= abort_threshold(members) {
        return Some(Decision::Abort);
    }
    if quorums >= cutover_threshold(members) {
        return Some(Decision::Quorum);
    }
    None
}

/// One verified record and the bucket object that served it.
#[derive(Debug, Clone)]
struct Served {
    record: Record,
    /// The member whose bucket served the record.
    bucket: String,
    object: &'static str,
}

/// What a record must name to count for this attempt. The detector counts a
/// record of another release, or of another member set, for neither decision.
struct ExpectedVote<'a> {
    /// None when `upgrade_state` does not name one version: not filtered then.
    stack_version: Option<&'a str>,
    consensus_version: i64,
    /// Sorted registered member addresses.
    member_set: &'a [String],
}

/// Group every verified record by its signer and classify each member.
/// `served` holds only records whose signature was verified against a
/// registered member, read from the signer's own `outcome.json` or from any
/// member's `seen.json` relay.
fn tally_votes(
    own: &BTreeMap<String, OwnBucket>,
    served: &[Served],
    expected: &ExpectedVote,
) -> Vec<MemberVotes> {
    let mut votes: BTreeMap<&str, MemberVotes> = expected
        .member_set
        .iter()
        .map(|address| {
            let member = MemberVotes {
                address: address.clone(),
                own_bucket: own.get(address).copied().unwrap_or_default(),
                ..Default::default()
            };
            (address.as_str(), member)
        })
        .collect();
    for item in served {
        let payload = &item.record.payload;
        let Some(member) = votes.get_mut(payload.signer.as_str()) else {
            continue;
        };
        let mut mismatch = Vec::new();
        if expected
            .stack_version
            .is_some_and(|version| version != payload.target_stack_version)
        {
            mismatch.push(format!(
                "target_stack_version '{}'",
                payload.target_stack_version
            ));
        }
        if payload.target_consensus_version != expected.consensus_version {
            mismatch.push(format!(
                "target_consensus_version {}",
                payload.target_consensus_version
            ));
        }
        let record_set = sorted_unique(payload.addresses().iter().map(|a| (*a).to_string()));
        if record_set != expected.member_set {
            mismatch.push(format!("member set {record_set:?}"));
        }
        if !mismatch.is_empty() {
            for reason in mismatch {
                if !member.mismatch.contains(&reason) {
                    member.mismatch.push(reason);
                }
            }
            continue;
        }
        let (sources, kept) = match payload.decision {
            Decision::Quorum => (&mut member.quorum_sources, &mut member.quorum_record),
            Decision::Abort => (&mut member.abort_sources, &mut member.abort_record),
        };
        let source = format!("{}/{}", item.bucket, item.object);
        if !sources.contains(&source) {
            sources.push(source);
        }
        if kept.is_none() {
            *kept = Some(item.record.clone());
        }
    }
    for member in votes.values_mut() {
        member.dishonest = member.quorum_record.is_some() && member.abort_record.is_some();
        if member.dishonest {
            continue;
        }
        // Records of this member that a PEER relays.
        let mut relayed = served
            .iter()
            .filter(|item| item.record.payload.signer == member.address)
            .filter(|item| item.bucket != member.address)
            .peekable();
        if relayed.peek().is_none() {
            continue;
        }
        let own_record = served
            .iter()
            .find(|item| item.bucket == member.address && item.object == OUTCOME_OBJECT)
            .map(|item| &item.record);
        member.suspect = match (member.own_bucket, own_record) {
            (OwnBucket::Valid, Some(own_record)) => relayed
                .any(|item| &item.record != own_record)
                .then(|| "its own bucket serves a different record than a peer relays".to_string()),
            (OwnBucket::Absent, _) => {
                Some("a peer relays its record, but its own bucket serves none".to_string())
            }
            (OwnBucket::Unreadable, _) => {
                Some("a peer relays its record, but its own bucket is unreadable".to_string())
            }
            _ => Some(
                "a peer relays its record, but its own bucket serves an invalid one".to_string(),
            ),
        };
    }
    votes.into_values().collect()
}

/// What a GET of one decision object produced.
enum Fetched {
    Body(Vec<u8>),
    /// HTTP 404.
    Absent,
    /// Transport or HTTP error, or a body over the cap.
    Error(String),
}

/// GET `url`, reading at most `max_bytes`: a member serves these bodies, so a
/// larger one is refused before it is buffered.
async fn fetch_capped(http: &reqwest::Client, url: &str, max_bytes: usize) -> Fetched {
    let mut response = match http.get(url).send().await {
        Err(e) => return Fetched::Error(format!("transport error: {e}")),
        Ok(response) if response.status() == reqwest::StatusCode::NOT_FOUND => {
            return Fetched::Absent
        }
        Ok(response) if !response.status().is_success() => {
            return Fetched::Error(format!("HTTP {}", response.status()))
        }
        Ok(response) => response,
    };
    let mut body = Vec::new();
    loop {
        match response.chunk().await {
            Err(e) => return Fetched::Error(format!("body read error: {e}")),
            Ok(None) => return Fetched::Body(body),
            Ok(Some(chunk)) => {
                if body.len() + chunk.len() > max_bytes {
                    return Fetched::Error(format!("body is larger than {max_bytes} bytes"));
                }
                body.extend_from_slice(&chunk);
            }
        }
    }
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

/// Lowest and highest consensus anchor block per chain over the anchors of all
/// verified quorum records. Each member anchors on the first block it saw every
/// member agree on, so two members can sign different blocks for one chain.
fn anchor_ranges<I: IntoIterator<Item = (i64, i64)>>(anchors: I) -> BTreeMap<i64, (i64, i64)> {
    let mut ranges: BTreeMap<i64, (i64, i64)> = BTreeMap::new();
    for (chain_id, block) in anchors {
        ranges
            .entry(chain_id)
            .and_modify(|(low, high)| {
                *low = (*low).min(block);
                *high = (*high).max(block);
            })
            .or_insert((block, block));
    }
    ranges
}

/// Whether the gw-listener has scanned every Gateway block up to the scan
/// cutoff. `next_block_time` is the timestamp of the first Gateway block the
/// listener has NOT scanned yet, or None when the listener is at the Gateway
/// tip. The Gateway only produces a block when it has a transaction, so a
/// quiet Gateway has no block past the cutoff: being at the tip once the cutoff
/// has passed is then complete coverage, not a lagging listener.
fn gateway_scanned_to_cutoff(
    next_block_time: Option<u64>,
    cutoff_unix: i64,
    cutoff_passed: bool,
) -> bool {
    match next_block_time {
        Some(time) => i64::try_from(time).is_ok_and(|time| time > cutoff_unix),
        None => cutoff_passed,
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

    println!("upgrade-check: target consensus_version {target}");
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

    // The scan cutoff is computed by the DB so it uses the same clock as the
    // timestamps it is compared with. It comes back as text and is bound again
    // as `$n::timestamptz`, which also makes every later query replayable.
    const EPOCH_SQL: &str = "SELECT consensus_epoch, outcome, consensus_version, stack_version, \
         completed_at::TEXT AS cutover_time, \
         (completed_at + make_interval(secs => $3))::TEXT AS scan_cutoff, \
         FLOOR(EXTRACT(EPOCH FROM completed_at + make_interval(secs => $3)))::BIGINT \
           AS scan_cutoff_unix, \
         NOW() >= completed_at + make_interval(secs => $3) AS cutoff_passed \
         FROM consensus_epoch_history WHERE proposal_id = $1 AND proposal_block = $2";
    let post_cutover_secs = args.post_cutover_duration.as_secs_f64();
    println!(
        "SQL: {EPOCH_SQL} [$1={proposal_id_hex}, $2={proposal_block}, $3={post_cutover_secs}]"
    );
    let epoch = sqlx::query(EPOCH_SQL)
        .bind(&proposal_id)
        .bind(proposal_block)
        .bind(post_cutover_secs)
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
    // (cutoff as text, cutoff as unix seconds, whether the cutoff has passed).
    // NULL while the epoch is still pending.
    let mut scan_cutoff: Option<(String, i64, bool)> = None;
    if let Some(row) = &epoch {
        report.cutover_time = row.try_get("cutover_time")?;
        let cutoff: Option<String> = row.try_get("scan_cutoff")?;
        let cutoff_unix: Option<i64> = row.try_get("scan_cutoff_unix")?;
        let cutoff_passed: Option<bool> = row.try_get("cutoff_passed")?;
        if let (Some(cutoff), Some(unix), Some(passed)) = (cutoff, cutoff_unix, cutoff_passed) {
            report.scan_cutoff = Some(cutoff.clone());
            scan_cutoff = Some((cutoff, unix, passed));
        }
    }

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
    let service = S3Service::new(provider.clone(), gateway_config_address);
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
    let member_addresses: BTreeMap<String, Address> = members
        .iter()
        .map(|(address, _)| (decision::name_of(*address), *address))
        .collect();
    let record_key = decision::key(&proposal_id, proposal_block);
    let relay_key = decision::relay_key(&proposal_id, proposal_block);
    // Every verified record, from a member's own outcome.json or from any
    // member's seen.json relay. A record is signed, so one a peer passes on
    // counts like one read from its signer.
    let mut served: Vec<Served> = Vec::new();
    let mut own_buckets: BTreeMap<String, OwnBucket> = BTreeMap::new();
    let mut bucket_errors = 0usize;
    for (address, bucket_url) in &members {
        let name = decision::name_of(*address);
        let base = bucket_url.trim_end_matches('/');

        let url = format!("{base}/{record_key}");
        println!("GET {url}");
        let own_bucket = match fetch_capped(&http, &url, MAX_RECORD_BYTES).await {
            Fetched::Absent => OwnBucket::Absent,
            Fetched::Error(e) => {
                bucket_errors += 1;
                report.warn(format!("{OUTCOME_OBJECT} of {name} unreadable ({e})"));
                OwnBucket::Unreadable
            }
            Fetched::Body(body) => {
                let verified = serde_json::from_slice::<Record>(&body)
                    .map_err(|e| format!("does not parse: {e}"))
                    .and_then(|record| {
                        verify_record(&record, *address, proposal_id_hex, proposal_block)
                            .map(|()| record)
                    });
                match verified {
                    Ok(record) => {
                        served.push(Served {
                            record,
                            bucket: name.clone(),
                            object: OUTCOME_OBJECT,
                        });
                        OwnBucket::Valid
                    }
                    Err(e) => {
                        report.warn(format!(
                            "the bucket of {name} serves an invalid {OUTCOME_OBJECT} ({e}); \
                             record dropped"
                        ));
                        OwnBucket::Invalid
                    }
                }
            }
        };
        own_buckets.insert(name.clone(), own_bucket);

        let url = format!("{base}/{relay_key}");
        println!("GET {url}");
        match fetch_capped(&http, &url, max_relay_bytes(members.len())).await {
            Fetched::Absent => println!("  no {RELAY_OBJECT}"),
            Fetched::Error(e) => {
                bucket_errors += 1;
                report.warn(format!("{RELAY_OBJECT} of {name} unreadable ({e})"));
            }
            Fetched::Body(body) => match serde_json::from_slice::<Vec<Record>>(&body) {
                Err(e) => report.warn(format!(
                    "the bucket of {name} serves a {RELAY_OBJECT} that does not parse ({e}); \
                     ignored"
                )),
                Ok(records) if records.len() > 2 * members.len() => report.warn(format!(
                    "the bucket of {name} serves a {RELAY_OBJECT} with {} records, more than \
                     two per member; ignored",
                    records.len()
                )),
                Ok(records) => {
                    let total = records.len();
                    let mut dropped = 0usize;
                    for record in records {
                        let verified = member_addresses
                            .get(&record.payload.signer)
                            .ok_or_else(|| {
                                format!(
                                    "signer '{}' is not a registered member",
                                    record.payload.signer
                                )
                            })
                            .and_then(|signer| {
                                verify_record(&record, *signer, proposal_id_hex, proposal_block)
                            });
                        match verified {
                            Ok(()) => served.push(Served {
                                record,
                                bucket: name.clone(),
                                object: RELAY_OBJECT,
                            }),
                            Err(e) => {
                                dropped += 1;
                                report.warn(format!(
                                    "the bucket of {name} relays an invalid record ({e}); \
                                     record dropped"
                                ));
                            }
                        }
                    }
                    println!("  {total} relayed records, {dropped} dropped");
                }
            },
        }
    }

    // The release the records must name: the proposal's version, as the
    // detector reads it from upgrade_state.
    let proposal_versions = sorted_unique(gcs.iter().filter_map(|r| r.version.clone()));
    let expected_stack_version = match proposal_versions.as_slice() {
        [version] => Some(version.as_str()),
        _ => {
            report.warn(format!(
                "GCS rows do not name exactly one version ({proposal_versions:?}); records \
                 are not filtered by target_stack_version"
            ));
            None
        }
    };
    if let (false, Some(version)) = (rolled_back, expected_stack_version) {
        if version != live_stack_version {
            report.fail(format!(
                "versioning.stack_version is '{live_stack_version}', the proposal's version \
                 is '{version}'"
            ));
        }
    }
    let votes = tally_votes(
        &own_buckets,
        &served,
        &ExpectedVote {
            stack_version: expected_stack_version,
            consensus_version: target,
            member_set: &gateway_set,
        },
    );

    let member_count = votes.len();
    let quorums = votes.iter().filter(|m| m.quorum_record.is_some()).count();
    let aborts = votes.iter().filter(|m| m.abort_record.is_some()).count();
    report.quorum_votes = quorums;
    report.abort_votes = aborts;
    report.cutover_threshold = cutover_threshold(member_count);
    report.abort_threshold = abort_threshold(member_count);
    for member in &votes {
        let decision = match (&member.quorum_record, &member.abort_record) {
            (Some(_), Some(_)) => "quorum AND abort",
            (Some(_), None) => "quorum",
            (None, Some(_)) => "abort",
            (None, None) => "no counted record",
        };
        println!(
            "  {}: {decision} (own bucket: {:?})",
            member.address, member.own_bucket
        );
        if let (true, Some(quorum), Some(abort)) = (
            member.dishonest,
            &member.quorum_record,
            &member.abort_record,
        ) {
            println!(
                "DISHONEST (proof): {} signed both decisions for this attempt",
                member.address
            );
            println!(
                "  quorum record, served by {:?}: {}",
                member.quorum_sources,
                serde_json::to_string(quorum)?
            );
            println!(
                "  abort record, served by {:?}: {}",
                member.abort_sources,
                serde_json::to_string(abort)?
            );
            report.dishonest_members.push(member.address.clone());
        }
        if let Some(why) = &member.suspect {
            report.warn(format!(
                "SUSPECT (no proof, can be an outage): {}: {why}",
                member.address
            ));
        }
        if !member.mismatch.is_empty() {
            report.warn(format!(
                "MISMATCH: {} signed for {}; that record is not counted",
                member.address,
                member.mismatch.join(", ")
            ));
        }
        if member.quorum_record.is_none() && member.abort_record.is_none() {
            println!(
                "  note: {} has no counted record (it never signed, or it was down)",
                member.address
            );
        }
    }
    println!(
        "votes of {member_count} members (at most {} faulty): {quorums} quorum (cutover needs \
         {}), {aborts} abort (rollback needs {})",
        max_faulty(member_count),
        report.cutover_threshold,
        report.abort_threshold
    );
    // Informational: where the fleet reached consensus. The cutover itself ran
    // later, at the cutover time printed in step 3.
    let quorum_anchors = votes
        .iter()
        .filter_map(|m| m.quorum_record.as_ref())
        .flat_map(|record| record.payload.anchors.iter().copied());
    for (chain_id, (low, high)) in anchor_ranges(quorum_anchors) {
        if low == high {
            println!("consensus anchor: chain {chain_id} block {high}");
        } else {
            println!("consensus anchor: chain {chain_id} blocks {low}..{high} across members");
        }
        report.anchors.insert(chain_id, high);
    }
    let dishonest = !report.dishonest_members.is_empty();
    report.members = votes;

    let outcome = fleet_outcome(quorums, aborts, member_count);
    println!(
        "fleet outcome: {}",
        outcome.map_or("undecided", decision_name)
    );
    if rolled_back {
        match outcome {
            Some(Decision::Abort) => {
                println!("rollback confirmed: the abort records reach the rollback threshold");
            }
            Some(Decision::Quorum) => report.fail(
                "upgrade_state says failed but the quorum records reach the cutover threshold"
                    .to_string(),
            ),
            None if bucket_errors > 0 => {
                report.warn(
                    "could not confirm the rollback from the records; some buckets were \
                     unreadable"
                        .to_string(),
                );
            }
            None => report.warn(
                "the abort records are below the rollback threshold; the records were \
                 cleaned up or never written"
                    .to_string(),
            ),
        }
        if !report.failures.is_empty() {
            return Ok(Verdict::Fail);
        }
        if dishonest {
            return Ok(Verdict::DishonestMember);
        }
        if outcome.is_none() && bucket_errors > 0 {
            return Ok(Verdict::Inconclusive);
        }
        return Ok(Verdict::RolledBack);
    }
    // Set when the records do not decide and a bucket was unreadable: the
    // missing records may exist, so this is not a failure.
    let mut quorum_unknown = false;
    match outcome {
        Some(Decision::Quorum) => {}
        Some(Decision::Abort) => report.fail(
            "the abort records reach the rollback threshold although upgrade_state says the \
             cutover completed"
                .to_string(),
        ),
        None if bucket_errors > 0 => {
            report.warn(
                "the quorum records are below the cutover threshold, but some buckets were \
                 unreadable"
                    .to_string(),
            );
            quorum_unknown = true;
        }
        None => report.fail(
            "the quorum records are below the cutover threshold although upgrade_state says \
             the cutover completed"
                .to_string(),
        ),
    }

    // ---- Step 3: ciphertext consensus_version --------------------------
    println!("\n== Step 3: ciphertext consensus_version (DB) ==");
    let mut digest_targets: Vec<DigestTarget> = Vec::new();
    let Some((scan_cutoff, scan_cutoff_unix, cutoff_passed)) = scan_cutoff else {
        report.fail(
            "consensus_epoch_history has no completed_at for this attempt: the cutover time \
             is unknown, so steps 3 and 4 cannot run"
                .to_string(),
        );
        return Ok(Verdict::Fail);
    };
    println!(
        "cutover time {}; checking what was ingested up to {scan_cutoff} (cutover + {})",
        report.cutover_time.as_deref().unwrap_or("-"),
        humantime::format_duration(args.post_cutover_duration)
    );
    // Set when the scan could not cover everything up to the cutoff: the run
    // is then inconclusive, never a pass over less than was asked for.
    let mut coverage_incomplete = false;
    if !cutoff_passed {
        report.warn(format!(
            "the scan cutoff {scan_cutoff} is still in the future; re-run after it for a \
             complete check"
        ));
        coverage_incomplete = true;
    }

    const OUTPUT_HANDLES_SQL: &str = "SELECT DISTINCT comp.output_handle AS handle, \
         comp.block_number AS block_number, \
         COALESCE(c.consensus_version, 1)::BIGINT AS consensus_version, \
         (c.handle IS NULL) AS missing \
         FROM computations comp \
         LEFT JOIN ciphertexts c ON c.handle = comp.output_handle \
         WHERE comp.host_chain_id = $1 AND comp.block_number >= $2 \
           AND comp.block_number <= $3 \
           AND comp.is_error = false AND comp.is_allowed = true \
           AND comp.is_completed = true \
         ORDER BY comp.block_number";
    // `created_at` is when this operator ingested the block. The last block
    // ingested at or before the cutoff ends the scan; a block ingested after
    // the cutoff proves ingestion went past it.
    const HOST_BOUND_SQL: &str = "SELECT \
         MAX(block_number) FILTER (WHERE created_at <= $2::timestamptz) AS end_block, \
         COALESCE(bool_or(created_at > $2::timestamptz), false) AS ingested_past_cutoff \
         FROM host_chain_blocks_valid WHERE chain_id = $1 AND block_status <> 'orphaned'";
    for chain in &gcs {
        let Some(start_block) = chain.start_block else {
            report.fail(format!(
                "GCS row for chain {} has no start_block",
                chain.host_chain_id
            ));
            continue;
        };
        println!(
            "SQL: {HOST_BOUND_SQL} [$1={}, $2={scan_cutoff}]",
            chain.host_chain_id
        );
        let bound = sqlx::query(HOST_BOUND_SQL)
            .bind(chain.host_chain_id)
            .bind(&scan_cutoff)
            .fetch_one(&pool)
            .await
            .context("querying the host scan bound")?;
        let end_block: Option<i64> = bound.try_get("end_block")?;
        let ingested_past_cutoff: bool = bound.try_get("ingested_past_cutoff")?;
        if !ingested_past_cutoff {
            report.warn(format!(
                "chain {}: no block ingested after the scan cutoff {scan_cutoff} yet, so \
                 coverage up to the cutoff is not proven; re-run once the chain moves on",
                chain.host_chain_id
            ));
            coverage_incomplete = true;
        }
        let Some(end_block) = end_block else {
            report.warn(format!(
                "chain {}: no block ingested at or before the scan cutoff; chain skipped",
                chain.host_chain_id
            ));
            coverage_incomplete = true;
            continue;
        };
        println!(
            "SQL: {OUTPUT_HANDLES_SQL} [$1={}, $2={start_block}, $3={end_block}]",
            chain.host_chain_id
        );
        let rows = sqlx::query(OUTPUT_HANDLES_SQL)
            .bind(chain.host_chain_id)
            .bind(start_block)
            .bind(end_block)
            .fetch_all(&pool)
            .await
            .context("querying output handles")?;
        let mut chain_report = ChainVersionReport {
            chain_id: chain.host_chain_id,
            start_block,
            end_block: Some(end_block),
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
            "chain {}: {} outputs in blocks [{start_block}..{end_block}], {} missing, {} on the \
             wrong version",
            chain.host_chain_id, chain_report.total, chain_report.missing, chain_report.mismatched
        );
        report.output_versions.push(chain_report);
    }

    let gw_start_block = gcs.iter().filter_map(|r| r.gw_start_block).min();
    match gw_start_block {
        None => report.fail("no GCS row carries a gw_start_block".to_string()),
        Some(gw_start_block) => {
            // Has the gw-listener scanned every Gateway block up to the cutoff?
            // Ask the Gateway for the first block the listener has not scanned.
            const GW_PROGRESS_SQL: &str =
                "SELECT last_block_num FROM gw_listener_last_block WHERE dummy_id";
            println!("SQL: {GW_PROGRESS_SQL}");
            let last_scanned: Option<i64> = sqlx::query_scalar(GW_PROGRESS_SQL)
                .fetch_optional(&pool)
                .await
                .context("querying gw_listener_last_block")?
                .flatten();
            match last_scanned {
                None => {
                    report.warn(
                        "gw_listener_last_block is empty: cannot tell how far the gw-listener \
                         scanned the Gateway"
                            .to_string(),
                    );
                    coverage_incomplete = true;
                }
                Some(last_scanned) => {
                    let next =
                        u64::try_from(last_scanned).context("negative gw_listener_last_block")? + 1;
                    let next_block_time = provider
                        .get_block_by_number(next.into())
                        .await
                        .context("reading the Gateway block after the gw-listener watermark")?
                        .map(|block| block.header.timestamp);
                    if gateway_scanned_to_cutoff(next_block_time, scan_cutoff_unix, cutoff_passed) {
                        match next_block_time {
                            Some(_) => println!(
                                "gw-listener scanned to Gateway block {last_scanned}; block \
                                 {next} is past the scan cutoff"
                            ),
                            None => println!(
                                "gw-listener scanned to Gateway block {last_scanned}, the \
                                 Gateway tip: the Gateway produced no block after it (no \
                                 transactions), so no input is missing"
                            ),
                        }
                    } else {
                        if next_block_time.is_some() {
                            report.warn(format!(
                                "gw-listener scanned only to Gateway block {last_scanned}, and \
                                 block {next} is not past the scan cutoff: inputs up to the \
                                 cutoff may not be ingested yet"
                            ));
                        }
                        coverage_incomplete = true;
                    }
                }
            }
            // `created_at` is when the gw-listener ingested the proof request.
            const INPUT_HANDLES_SQL: &str = "SELECT zk_proof_id, block_number, handles \
                 FROM verify_proofs WHERE verified = true AND block_number >= $1 \
                   AND created_at <= $2::timestamptz";
            println!(
                "SQL: {INPUT_HANDLES_SQL} [$1={gw_start_block}, $2={scan_cutoff}] (Gateway blocks)"
            );
            let rows = sqlx::query(INPUT_HANDLES_SQL)
                .bind(gw_start_block)
                .bind(&scan_cutoff)
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
            // S3 stores only allowed handles, so only an allowed input's ct64/ct128 is
            // expected there. An un-allowed input keeps its ct64 in the DB but is never
            // uploaded (correct, not a gap), so checking it against S3 would be a false
            // MissingInS3. Gate the S3 digest target on membership in allowed_handles,
            // mirroring the is_allowed = true filter the computed-output query already uses.
            const INPUT_ALLOWED_SQL: &str =
                "SELECT DISTINCT handle FROM allowed_handles WHERE handle = ANY($1)";
            let rows = sqlx::query(INPUT_ALLOWED_SQL)
                .bind(&handle_list)
                .fetch_all(&pool)
                .await
                .context("querying allowed input handles")?;
            let mut allowed: BTreeSet<Vec<u8>> = BTreeSet::new();
            for row in rows {
                allowed.insert(row.try_get("handle")?);
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
                // Only allowed inputs are uploaded to S3; an un-allowed input's ct64 lives
                // only in the DB, so checking it against S3 would be a false MissingInS3.
                if allowed.contains(handle) {
                    digest_targets.push(DigestTarget {
                        handle: handle.clone(),
                        origin: "input",
                        boundary: false,
                    });
                }
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
    if dishonest {
        return Ok(Verdict::DishonestMember);
    }
    if quorum_unknown || unreachable > 0 || coverage_incomplete {
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
    fn thresholds_follow_members_greater_than_3_max_faulty() {
        // (members, max faulty, cutover needs, rollback needs)
        for (members, faulty, cutover, abort) in [
            (1, 0, 1, 1),
            (3, 0, 3, 1),
            (4, 1, 3, 3),
            (5, 1, 4, 3),
            (7, 2, 5, 5),
        ] {
            assert_eq!(max_faulty(members), faulty, "{members} members");
            assert_eq!(cutover_threshold(members), cutover, "{members} members");
            assert_eq!(abort_threshold(members), abort, "{members} members");
        }
    }

    #[test]
    fn fleet_outcome_counts_records_against_both_thresholds() {
        use Decision::{Abort, Quorum};
        assert_eq!(fleet_outcome(0, 0, 0), None);
        // 3 members tolerate no fault: every quorum, or one abort.
        assert_eq!(fleet_outcome(3, 0, 3), Some(Quorum));
        assert_eq!(fleet_outcome(2, 0, 3), None);
        assert_eq!(fleet_outcome(2, 1, 3), Some(Abort));
        // 5 members: 4 quorum cut over, 3 abort roll back.
        assert_eq!(fleet_outcome(4, 0, 5), Some(Quorum));
        assert_eq!(fleet_outcome(4, 1, 5), Some(Quorum));
        assert_eq!(fleet_outcome(3, 2, 5), None);
        assert_eq!(fleet_outcome(2, 3, 5), Some(Abort));
        // Both reached (assumption broken): stay on blue.
        assert_eq!(fleet_outcome(4, 3, 5), Some(Abort));
    }

    /// `count` members, their sorted names, and one signed record each.
    fn fleet(count: usize) -> (Vec<PrivateKeySigner>, Vec<String>) {
        let signers: Vec<PrivateKeySigner> =
            (0..count).map(|_| PrivateKeySigner::random()).collect();
        let names = sorted_unique(signers.iter().map(|s| decision::name_of(s.address())));
        (signers, names)
    }

    fn record_of(
        signer: &PrivateKeySigner,
        fleet: &[PrivateKeySigner],
        decision: Decision,
    ) -> Record {
        let peers: Vec<Address> = fleet
            .iter()
            .map(PrivateKeySigner::address)
            .filter(|address| *address != signer.address())
            .collect();
        let mut payload = payload_for(signer, &peers);
        payload.decision = decision;
        signed_record(signer, payload)
    }

    fn served(record: &Record, bucket: &PrivateKeySigner, object: &'static str) -> Served {
        Served {
            record: record.clone(),
            bucket: decision::name_of(bucket.address()),
            object,
        }
    }

    fn expected(names: &[String]) -> ExpectedVote<'_> {
        ExpectedVote {
            stack_version: Some("v0.15"),
            consensus_version: 2,
            member_set: names,
        }
    }

    fn votes_of<'a>(votes: &'a [MemberVotes], signer: &PrivateKeySigner) -> &'a MemberVotes {
        let name = decision::name_of(signer.address());
        votes.iter().find(|m| m.address == name).unwrap()
    }

    #[test]
    fn honest_fleet_has_one_decision_per_member() {
        let (fleet, names) = fleet(4);
        let mut items = Vec::new();
        let mut own = BTreeMap::new();
        for signer in &fleet {
            let record = record_of(signer, &fleet, Decision::Quorum);
            items.push(served(&record, signer, OUTCOME_OBJECT));
            // Every peer relays it too.
            for peer in &fleet {
                items.push(served(&record, peer, RELAY_OBJECT));
            }
            own.insert(decision::name_of(signer.address()), OwnBucket::Valid);
        }
        let votes = tally_votes(&own, &items, &expected(&names));
        assert_eq!(votes.len(), 4);
        for member in &votes {
            assert!(member.quorum_record.is_some() && member.abort_record.is_none());
            assert!(!member.dishonest);
            assert_eq!(member.suspect, None);
            assert!(member.mismatch.is_empty());
            // Own outcome.json plus four relays, each listed once.
            assert_eq!(member.quorum_sources.len(), 5);
        }
    }

    #[test]
    fn member_signing_both_decisions_is_proven_dishonest() {
        let (fleet, names) = fleet(4);
        let liar = &fleet[0];
        let quorum = record_of(liar, &fleet, Decision::Quorum);
        let abort = record_of(liar, &fleet, Decision::Abort);
        // Its own bucket serves quorum; one peer was shown abort and relays it.
        let items = vec![
            served(&quorum, liar, OUTCOME_OBJECT),
            served(&abort, &fleet[1], RELAY_OBJECT),
        ];
        let own = BTreeMap::from([(decision::name_of(liar.address()), OwnBucket::Valid)]);
        let votes = tally_votes(&own, &items, &expected(&names));
        let member = votes_of(&votes, liar);
        assert!(member.dishonest);
        assert_eq!(member.quorum_record.as_ref(), Some(&quorum));
        assert_eq!(member.abort_record.as_ref(), Some(&abort));
        // It counts in both tallies, as in the detector.
        assert_eq!(
            votes.iter().filter(|m| m.quorum_record.is_some()).count(),
            1
        );
        assert_eq!(votes.iter().filter(|m| m.abort_record.is_some()).count(), 1);
        assert!(!votes_of(&votes, &fleet[1]).dishonest);
    }

    #[test]
    fn relayed_record_with_a_silent_own_bucket_is_only_suspect() {
        let (fleet, names) = fleet(4);
        let quiet = &fleet[0];
        let record = record_of(quiet, &fleet, Decision::Quorum);
        let items = vec![served(&record, &fleet[1], RELAY_OBJECT)];
        for status in [OwnBucket::Absent, OwnBucket::Unreadable, OwnBucket::Invalid] {
            let own = BTreeMap::from([(decision::name_of(quiet.address()), status)]);
            let votes = tally_votes(&own, &items, &expected(&names));
            let member = votes_of(&votes, quiet);
            assert!(member.suspect.is_some(), "{status:?}");
            assert!(!member.dishonest);
            // The relayed record is signed, so it still counts.
            assert!(member.quorum_record.is_some());
        }
        // A member nobody has a record of is silent, not suspect.
        let votes = tally_votes(&BTreeMap::new(), &items, &expected(&names));
        let silent = votes_of(&votes, &fleet[2]);
        assert_eq!(silent.suspect, None);
        assert!(silent.quorum_record.is_none() && silent.abort_record.is_none());
    }

    #[test]
    fn different_record_with_the_same_decision_is_only_suspect() {
        let (fleet, names) = fleet(4);
        let member = &fleet[0];
        let record = record_of(member, &fleet, Decision::Quorum);
        let mut other_payload = record.payload.clone();
        other_payload.anchors = vec![(1, 999)];
        let other = signed_record(member, other_payload);
        let items = vec![
            served(&record, member, OUTCOME_OBJECT),
            served(&other, &fleet[1], RELAY_OBJECT),
        ];
        let own = BTreeMap::from([(decision::name_of(member.address()), OwnBucket::Valid)]);
        let votes = tally_votes(&own, &items, &expected(&names));
        let found = votes_of(&votes, member);
        assert!(found
            .suspect
            .as_deref()
            .unwrap()
            .contains("different record"));
        assert!(!found.dishonest);
        // The same record relayed by a peer is not suspect.
        let same = vec![
            served(&record, member, OUTCOME_OBJECT),
            served(&record, &fleet[1], RELAY_OBJECT),
        ];
        let votes = tally_votes(&own, &same, &expected(&names));
        assert_eq!(votes_of(&votes, member).suspect, None);
    }

    #[test]
    fn record_for_another_release_or_member_set_is_a_mismatch_and_not_counted() {
        let (fleet, names) = fleet(4);
        let own = BTreeMap::new();
        let base = record_of(&fleet[0], &fleet, Decision::Quorum);

        let mut stack = base.payload.clone();
        stack.target_stack_version = "v0.14".to_string();
        let mut consensus = base.payload.clone();
        consensus.target_consensus_version = 3;
        let mut set = base.payload.clone();
        set.members.pop();
        for (payload, needle) in [
            (stack, "target_stack_version"),
            (consensus, "target_consensus_version"),
            (set, "member set"),
        ] {
            let record = signed_record(&fleet[0], payload);
            let items = vec![served(&record, &fleet[0], OUTCOME_OBJECT)];
            let votes = tally_votes(&own, &items, &expected(&names));
            let member = votes_of(&votes, &fleet[0]);
            assert!(
                member.mismatch.iter().any(|m| m.contains(needle)),
                "{needle}"
            );
            assert!(member.quorum_record.is_none(), "{needle}: must not count");
        }
        // A quorum for this release and an abort for another one is not a proof.
        let mut abort_other = base.payload.clone();
        abort_other.decision = Decision::Abort;
        abort_other.target_stack_version = "v0.14".to_string();
        let items = vec![
            served(&base, &fleet[0], OUTCOME_OBJECT),
            served(
                &signed_record(&fleet[0], abort_other),
                &fleet[1],
                RELAY_OBJECT,
            ),
        ];
        let votes = tally_votes(&own, &items, &expected(&names));
        assert!(!votes_of(&votes, &fleet[0]).dishonest);
    }

    #[test]
    fn oversized_record_fails_verification() {
        let signer = PrivateKeySigner::random();
        let mut payload = payload_for(&signer, &[]);
        payload.members[0].url = "x".repeat(MAX_RECORD_BYTES);
        let record = signed_record(&signer, payload);
        let result = verify_record(&record, signer.address(), &"ab".repeat(32), 42);
        assert!(result.unwrap_err().contains("larger than"));
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
    fn anchor_ranges_keep_lowest_and_highest_per_chain() {
        let ranges = anchor_ranges([(1, 150), (2, 40), (1, 152), (1, 149)]);
        assert_eq!(ranges.get(&1), Some(&(149, 152)));
        assert_eq!(ranges.get(&2), Some(&(40, 40)));
        assert_eq!(ranges.get(&3), None);
        assert!(anchor_ranges([]).is_empty());
    }

    #[test]
    fn gateway_coverage_handles_a_quiet_gateway() {
        // The next unscanned block is past the cutoff: everything is scanned.
        assert!(gateway_scanned_to_cutoff(Some(1001), 1000, true));
        // The next unscanned block is at or before the cutoff: listener behind.
        assert!(!gateway_scanned_to_cutoff(Some(1000), 1000, true));
        assert!(!gateway_scanned_to_cutoff(Some(900), 1000, true));
        // Listener at the tip of a quiet Gateway: complete once the cutoff
        // has passed, not before (a block before the cutoff can still come).
        assert!(gateway_scanned_to_cutoff(None, 1000, true));
        assert!(!gateway_scanned_to_cutoff(None, 1000, false));
        // A timestamp that does not fit i64 never counts as coverage.
        assert!(!gateway_scanned_to_cutoff(Some(u64::MAX), 1000, true));
    }

    #[test]
    fn verdict_exit_codes_are_stable() {
        assert_eq!(Verdict::Pass.code(), 0);
        assert_eq!(Verdict::Fail.code(), 1);
        assert_eq!(Verdict::NotExecuted.code(), 2);
        assert_eq!(Verdict::RolledBack.code(), 3);
        assert_eq!(Verdict::Inconclusive.code(), 4);
        assert_eq!(Verdict::DishonestMember.code(), 5);
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
