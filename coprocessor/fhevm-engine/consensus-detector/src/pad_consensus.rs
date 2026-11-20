//! Compares the pad outputs that this coprocessor computed with the outputs of the other
//! coprocessors in S3, with the rules of the manifest comparison. For each output, it reads the
//! attestation of the `ct128` object of each other coprocessor, checks its signature, and groups the
//! valid attestations by digests. Observation only: the result is a row, a metric and a log.

use crate::manifest_consensus::peer_http::object_url;
use alloy::primitives::{Address, B256};
use ciphertext_attestation::{
    Attestation, PrfOutputAttestation, PrfOutputRef, S3_METADATA_ATTESTATION_HEADER,
};
use fhevm_engine_common::prf_output::{prf_output_s3_key, PrfOutputForm};
use futures::StreamExt;
use prometheus::{register_int_counter_vec, IntCounterVec};
use sqlx::PgPool;
use std::{collections::HashMap, sync::LazyLock, time::Duration};
use tokio_util::sync::CancellationToken;
use tracing::{error, warn};

#[derive(Debug, Clone)]
pub struct Config {
    /// Time between two passes, and between two checks of one output.
    pub interval: Duration,
    /// Time after the start of a check until it ends without the missing coprocessors.
    pub deadline: Duration,
    /// Time after its computation until an output is too old to check, and after its end until a check
    /// that is not drift is deleted. Unlike the manifests, which keep all rows: there are many more
    /// pad outputs than blocks.
    pub retention: Duration,
    /// Outputs checked in each pass.
    pub batch: i64,
}

impl Default for Config {
    /// The window of the manifest verification: 10 s, then 59 retries every 10 s.
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(10),
            deadline: Duration::from_secs(10 * 60),
            retention: Duration::from_secs(24 * 60 * 60),
            batch: 100,
        }
    }
}

static RESULTS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_pad_consensus_results_total",
        "Ended checks of the pad outputs of this coprocessor",
        &["outcome", "local_result"]
    )
    .unwrap()
});

static MISSING: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_pad_consensus_missing_total",
        "Pad outputs that another coprocessor did not upload before the deadline",
        &["peer"]
    )
    .unwrap()
});

/// Outputs checked at the same time in a pass.
const CONCURRENT_CHECKS: usize = 16;

/// How long a pass holds its checks: longer than a batch with slow peers, so that another detector
/// does not ask the same peers again. At the end of its pass, a check that stays open sets its next
/// time.
const LEASE: Duration = Duration::from_secs(120);

/// Time between two deletions of the old checks.
const CLEANUP_INTERVAL: Duration = Duration::from_secs(60);

/// What a detector keeps from one pass to the next.
#[derive(Debug, Default)]
struct PassState {
    /// The latest `pad_ct64.created_at` that a pass found, in seconds since the epoch. `None`
    /// before the first pass: that pass reads the full retention.
    found_up_to: Option<f64>,
    /// When a pass last deleted the old checks.
    cleaned_at: Option<std::time::Instant>,
}

/// Runs a pass every `config.interval`, until `cancel`.
pub async fn run(pool: PgPool, http: reqwest::Client, config: Config, cancel: CancellationToken) {
    let mut state = PassState::default();
    loop {
        tokio::select! {
            () = cancel.cancelled() => return,
            () = tokio::time::sleep(config.interval) => {}
        }
        if let Err(e) = pass(&pool, &http, &config, &mut state).await {
            error!(error = %e, "Pad consensus pass failed");
        }
    }
}

/// The digests of one output.
type Digests = (B256, B256);

/// One output to check.
struct Output {
    subject: PrfOutputRef,
    digests: Digests,
    signer: Address,
    expired: bool,
}

/// Adds the new outputs, deletes the old ended checks except drift, and checks a batch.
/// Returns the checks that this pass ended.
async fn pass(
    pool: &PgPool,
    http: &reqwest::Client,
    config: &Config,
    state: &mut PassState,
) -> sqlx::Result<u64> {
    // The coprocessors and the quorum threshold, from the registry that the manifests use.
    let registry = sqlx::query!(
        "SELECT signer_address, s3_bucket_url, coprocessor_threshold FROM gateway_config_coprocessors"
    )
    .fetch_all(pool)
    .await?;
    let members: Vec<(Address, String)> = registry
        .iter()
        .map(|r| {
            (
                Address::from_slice(&r.signer_address),
                r.s3_bucket_url.clone(),
            )
        })
        .collect();
    let threshold = registry
        .first()
        .and_then(|r| usize::try_from(r.coprocessor_threshold).ok())
        .filter(|t| (1..=members.len()).contains(t));
    let Some(threshold) = threshold else {
        warn!("Pad consensus: no valid GatewayConfig registry snapshot, the pass waits");
        return Ok(0);
    };

    let deadline = config.deadline.as_secs_f64();
    let retention = config.retention.as_secs_f64();
    // Find only the outputs after the last output that a pass found. The overlap of 1 minute finds
    // a row that committed after a newer row, because `created_at` is the start of its transaction.
    let found_up_to = sqlx::query_scalar!(
        r#"WITH found AS (
               SELECT prf_instance, label, pad_length, ciphertext_digest, ciphertext128_digest,
                      signer, created_at
                 FROM pad_ct64
                WHERE created_at > GREATEST(NOW() - make_interval(secs => $1),
                                            to_timestamp($2::FLOAT8) - INTERVAL '1 minute')
           ), inserted AS (
               INSERT INTO pad_consensus (prf_instance, label, pad_length, ciphertext_digest,
                                          ciphertext128_digest, signer)
               SELECT prf_instance, label, pad_length, ciphertext_digest, ciphertext128_digest,
                      signer
                 FROM found
               ON CONFLICT DO NOTHING
           )
           SELECT EXTRACT(EPOCH FROM MAX(created_at))::FLOAT8 FROM found"#,
        retention,
        state.found_up_to,
    )
    .fetch_one(pool)
    .await?;
    if found_up_to > state.found_up_to {
        state.found_up_to = found_up_to;
    }
    // The drift evidence stays, as for the manifests. A check that never ends is deleted when it
    // is older than the retention.
    if state
        .cleaned_at
        .is_none_or(|at| at.elapsed() >= CLEANUP_INTERVAL)
    {
        sqlx::query!(
            "DELETE FROM pad_consensus
              WHERE COALESCE(done_at, created_at) < NOW() - make_interval(secs => $1)
                AND outcome IS DISTINCT FROM 'drift'",
            retention,
        )
        .execute(pool)
        .await?;
        state.cleaned_at = Some(std::time::Instant::now());
    }
    // The next check time is a lease: another detector skips these rows until then.
    let rows = sqlx::query!(
        r#"UPDATE pad_consensus SET next_check_at = NOW() + make_interval(secs => $1)
            WHERE (prf_instance, label, pad_length) IN (
                  SELECT prf_instance, label, pad_length FROM pad_consensus
                   WHERE done_at IS NULL AND next_check_at <= NOW()
                   ORDER BY next_check_at
                   LIMIT $2
                     FOR UPDATE SKIP LOCKED)
        RETURNING prf_instance, label, pad_length, ciphertext_digest, ciphertext128_digest,
                  signer, created_at <= NOW() - make_interval(secs => $3) AS "expired!""#,
        LEASE.as_secs_f64(),
        config.batch,
        deadline,
    )
    .fetch_all(pool)
    .await?;
    let outputs = rows.into_iter().map(|row| Output {
        subject: PrfOutputRef::new(
            u16::try_from(row.prf_instance).expect("prf_instance is in range"),
            B256::from_slice(&row.label),
            u16::try_from(row.pad_length).expect("pad_length is in range"),
        ),
        digests: (
            B256::from_slice(&row.ciphertext_digest),
            B256::from_slice(&row.ciphertext128_digest),
        ),
        signer: Address::from_slice(&row.signer),
        expired: row.expired,
    });
    // As for the manifests: our output counts only when our signer is in the registry.
    let (outputs, unregistered): (Vec<Output>, Vec<Output>) =
        outputs.partition(|output| members.iter().any(|(signer, _)| *signer == output.signer));
    if !unregistered.is_empty() {
        warn!(
            outputs = unregistered.len(),
            signer = %unregistered[0].signer,
            "Pad consensus: our signer is not in the GatewayConfig registry, the checks wait"
        );
    }
    // At the same time: an unreachable coprocessor costs one timeout, not one for each output.
    let results: Vec<sqlx::Result<bool>> = futures::stream::iter(outputs)
        .map(|output| {
            let members = &members;
            async move { check(pool, http, members, threshold, config.interval, &output).await }
        })
        .buffer_unordered(CONCURRENT_CHECKS)
        .collect()
        .await;
    let mut ended = 0;
    for result in results {
        ended += u64::from(result?);
    }
    Ok(ended)
}

/// Asks each other coprocessor that did not answer yet, then ends the check on consensus, when
/// every coprocessor answered, or at the deadline. Returns true if this call ended it.
async fn check(
    pool: &PgPool,
    http: &reqwest::Client,
    members: &[(Address, String)],
    threshold: usize,
    interval: Duration,
    output: &Output,
) -> sqlx::Result<bool> {
    let subject = &output.subject;
    let prf = i32::from(subject.prf_id);
    let pad_length = i32::from(subject.pad_length);
    let key = prf_output_s3_key(
        subject.prf_id,
        subject.label,
        subject.pad_length,
        PrfOutputForm::Ct128,
    );
    // The answers so far: `None` for an attestation that is not valid.
    let mut answers: HashMap<Address, Option<Digests>> = sqlx::query!(
        "SELECT peer_signer, ciphertext_digest, ciphertext128_digest FROM pad_consensus_answer
          WHERE prf_instance = $1 AND label = $2 AND pad_length = $3",
        prf,
        subject.label.as_slice(),
        pad_length,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|a| {
        let digests = a
            .ciphertext_digest
            .zip(a.ciphertext128_digest)
            .map(|(ct64, ct128)| (B256::from_slice(&ct64), B256::from_slice(&ct128)));
        (Address::from_slice(&a.peer_signer), digests)
    })
    .collect();

    let peers: Vec<&(Address, String)> = members
        .iter()
        .filter(|(signer, _)| *signer != output.signer)
        .collect();
    let unanswered: Vec<&&(Address, String)> = peers
        .iter()
        .filter(|(signer, _)| !answers.contains_key(signer))
        .collect();
    let headers = futures::future::join_all(
        unanswered
            .iter()
            .map(|(_, base_url)| attestation_header(http, object_url(base_url, &key))),
    )
    .await;
    for ((signer, _), header) in unanswered.into_iter().zip(headers) {
        let Some(header) = header else {
            continue;
        };
        let answer = match serde_json::from_str::<PrfOutputAttestation>(&header) {
            Err(e) => Err(format!("bad attestation: {e}")),
            Ok(attestation) => attestation
                .verify(subject, *signer)
                .map(|()| {
                    (
                        attestation.ciphertext_digest,
                        attestation.sns_ciphertext_digest,
                    )
                })
                .map_err(|e| format!("bad signature: {e}")),
        };
        if let Err(reason) = &answer {
            warn!(output = %subject, peer = %signer, reason, "Pad attestation of another coprocessor is not valid");
        }
        let digests = answer.as_ref().ok();
        sqlx::query!(
            "INSERT INTO pad_consensus_answer (prf_instance, label, pad_length, peer_signer,
                                               ciphertext_digest, ciphertext128_digest, reason,
                                               peer_attestation)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             ON CONFLICT DO NOTHING",
            prf,
            subject.label.as_slice(),
            pad_length,
            signer.as_slice(),
            digests.map(|d| d.0.to_vec()),
            digests.map(|d| d.1.to_vec()),
            answer.as_ref().err().map(String::as_str),
            header,
        )
        .execute(pool)
        .await?;
        answers.insert(*signer, digests.copied());
    }

    let (outcome, local_result) = compare(output.digests, answers.values().flatten(), threshold);
    let all_answered = peers.iter().all(|(signer, _)| answers.contains_key(signer));
    if !(outcome == "consensus" || all_answered || output.expired) {
        // The next check of this output comes after `interval`, not at the end of the lease.
        sqlx::query!(
            "UPDATE pad_consensus SET next_check_at = NOW() + make_interval(secs => $4)
              WHERE prf_instance = $1 AND label = $2 AND pad_length = $3 AND done_at IS NULL",
            prf,
            subject.label.as_slice(),
            pad_length,
            interval.as_secs_f64(),
        )
        .execute(pool)
        .await?;
        return Ok(false);
    }
    // Only one detector ends a check, so the metrics and logs count it once.
    let ended = sqlx::query!(
        "UPDATE pad_consensus SET outcome = $4, local_result = $5, done_at = NOW()
          WHERE prf_instance = $1 AND label = $2 AND pad_length = $3 AND done_at IS NULL",
        prf,
        subject.label.as_slice(),
        pad_length,
        outcome,
        local_result,
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1;
    if !ended {
        return Ok(false);
    }
    if !all_answered {
        for (signer, _) in &peers {
            if !answers.contains_key(signer) {
                MISSING.with_label_values(&[&signer.to_string()]).inc();
                warn!(output = %subject, peer = %signer, "No pad output from another coprocessor before the deadline");
            }
        }
    }
    RESULTS.with_label_values(&[outcome, local_result]).inc();
    if outcome == "drift" {
        error!(output = %subject, local_result, "Pad outputs differ between coprocessors");
    }
    Ok(true)
}

/// The manifest rules on our digests and the valid digests of the other coprocessors: the outcome,
/// and our result against the group that reaches `threshold`.
fn compare<'a>(
    ours: Digests,
    peers: impl Iterator<Item = &'a Digests>,
    threshold: usize,
) -> (&'static str, &'static str) {
    let mut groups: HashMap<Digests, usize> = HashMap::from([(ours, 1)]);
    let mut any_peer = false;
    for digests in peers {
        any_peer = true;
        *groups.entry(*digests).or_default() += 1;
    }
    let quorum: Vec<&Digests> = groups
        .iter()
        .filter(|(_, count)| **count >= threshold)
        .map(|(digests, _)| digests)
        .collect();
    let local_result = match quorum.as_slice() {
        [digests] if **digests == ours => "matches_quorum",
        [_] => "differs_from_quorum",
        _ => "inconclusive",
    };
    let outcome = if !any_peer {
        "unknown"
    } else if groups.len() > 1 {
        "drift"
    } else if quorum.is_empty() {
        "unknown_but_equal"
    } else {
        "consensus"
    };
    (outcome, local_result)
}

/// The attestation header of the object at `url`, or `None` if the object is not there yet or
/// the request failed: a later check asks again.
async fn attestation_header(http: &reqwest::Client, url: String) -> Option<String> {
    let mut response = http.head(&url).send().await.ok()?;
    // As the manifest probes: a bucket that grants reads alone answers 403 for a key it does not
    // hold, and some origins refuse HEAD. A ranged GET of one byte decides.
    if matches!(
        response.status(),
        reqwest::StatusCode::FORBIDDEN | reqwest::StatusCode::METHOD_NOT_ALLOWED
    ) {
        response = http
            .get(&url)
            .header(reqwest::header::RANGE, "bytes=0-0")
            .send()
            .await
            .ok()?;
    }
    if !response.status().is_success() {
        return None;
    }
    // The object is there, so a missing attestation is an answer.
    Some(
        response
            .headers()
            .get(S3_METADATA_ATTESTATION_HEADER)
            .and_then(|h| h.to_str().ok())
            .unwrap_or_default()
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::signers::{local::PrivateKeySigner, SignerSync};
    use axum::{http::StatusCode, Router};
    use ciphertext_attestation::{prf::Version, PrfOutputAttestationPayload};
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    use test_harness::instance::{setup_test_db, ImportMode};
    use tokio::net::TcpListener;

    const LABEL: B256 = B256::repeat_byte(0xab);
    const OURS: Digests = (B256::repeat_byte(1), B256::repeat_byte(2));
    const OTHER: Digests = (B256::repeat_byte(1), B256::repeat_byte(3));

    #[test]
    fn follows_the_rules_of_the_manifest_comparison() {
        let cases = [
            (vec![], ("unknown", "inconclusive")),
            (vec![OURS], ("unknown_but_equal", "inconclusive")),
            (vec![OURS, OURS], ("consensus", "matches_quorum")),
            (vec![OURS, OURS, OTHER], ("drift", "matches_quorum")),
            (vec![OTHER, OTHER, OTHER], ("drift", "differs_from_quorum")),
            (vec![OTHER], ("drift", "inconclusive")),
        ];
        for (peers, expected) in cases {
            assert_eq!(compare(OURS, peers.iter(), 3), expected, "{peers:?}");
        }
    }

    /// The attestation header of `key` over `digests`, with `claimed` as its signer.
    fn attestation(key: &PrivateKeySigner, claimed: Address, digests: Digests) -> String {
        let payload =
            PrfOutputAttestationPayload::new(Version::V1, 0, LABEL, 64, digests.0, digests.1);
        let signature = key.sign_hash_sync(&payload.canonical_digest()).unwrap();
        serde_json::to_string(&PrfOutputAttestation {
            version: payload.version,
            ciphertext_digest: digests.0,
            sns_ciphertext_digest: digests.1,
            signer: claimed,
            signature: signature.as_bytes().to_vec(),
        })
        .unwrap()
    }

    /// A peer bucket that answers every request with `status` and the attestation header.
    async fn peer(status: StatusCode, header: Option<String>) -> String {
        serve(move |_| (status, header.clone())).await
    }

    /// A peer bucket that refuses HEAD, and answers a GET with the attestation header.
    async fn get_only_peer(header: String) -> String {
        serve(move |method| match method {
            axum::http::Method::HEAD => (StatusCode::METHOD_NOT_ALLOWED, None),
            _ => (StatusCode::PARTIAL_CONTENT, Some(header.clone())),
        })
        .await
    }

    async fn serve<F>(answer: F) -> String
    where
        F: Fn(axum::http::Method) -> (StatusCode, Option<String>) + Clone + Send + Sync + 'static,
    {
        let app = Router::new().fallback(move |method: axum::http::Method| {
            let (status, header) = answer(method);
            async move {
                let mut headers = axum::http::HeaderMap::new();
                if let Some(header) = header {
                    headers.insert(S3_METADATA_ATTESTATION_HEADER, header.parse().unwrap());
                }
                (status, headers)
            }
        });
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        base_url
    }

    /// A peer bucket that answers after `delay`.
    async fn slow_peer(delay: Duration) -> String {
        let app = Router::new().fallback(move || async move {
            tokio::time::sleep(delay).await;
            StatusCode::NOT_FOUND
        });
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        base_url
    }

    /// A peer bucket that answers 404 after `delay`, and counts the requests.
    async fn counting_peer(delay: Duration) -> (String, Arc<AtomicUsize>) {
        let requests = Arc::new(AtomicUsize::new(0));
        let counter = requests.clone();
        let app = Router::new().fallback(move || {
            counter.fetch_add(1, Ordering::SeqCst);
            async move {
                tokio::time::sleep(delay).await;
                StatusCode::NOT_FOUND
            }
        });
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        (base_url, requests)
    }

    /// Adds our output for `label`, computed 2 hours ago: the ct128 GC deleted its `pads` row.
    async fn add_output(pool: &PgPool, ours: Address, label: B256) {
        add_output_aged(pool, ours, label, 7200.0).await;
    }

    /// Adds our output for `label`, computed `before` seconds before the output of `database`.
    async fn add_output_before(pool: &PgPool, ours: Address, label: B256, before: f64) {
        sqlx::query(
            "INSERT INTO pad_ct64 (prf_instance, label, pad_length, ciphertext64,
                                   ciphertext_digest, ciphertext128_digest, signer, signature,
                                   attestation_version, created_at)
             SELECT 0, $1, 64, '\\x00', $2, $3, $4, $5, 1,
                    created_at - make_interval(secs => $6)
               FROM pad_ct64 WHERE label = $7",
        )
        .bind(label.as_slice())
        .bind(OURS.0.as_slice())
        .bind(OURS.1.as_slice())
        .bind(ours.as_slice())
        .bind([0u8; 65].as_slice())
        .bind(before)
        .bind(LABEL.as_slice())
        .execute(pool)
        .await
        .unwrap();
    }

    /// Adds our output for `label`, computed `age` seconds ago.
    async fn add_output_aged(pool: &PgPool, ours: Address, label: B256, age: f64) {
        sqlx::query(
            "INSERT INTO pad_ct64 (prf_instance, label, pad_length, ciphertext64,
                                   ciphertext_digest, ciphertext128_digest, signer, signature,
                                   attestation_version, created_at)
             VALUES (0, $1, 64, '\\x00', $2, $3, $4, $5, 1, NOW() - make_interval(secs => $6))",
        )
        .bind(label.as_slice())
        .bind(OURS.0.as_slice())
        .bind(OURS.1.as_slice())
        .bind(ours.as_slice())
        .bind([0u8; 65].as_slice())
        .bind(age)
        .execute(pool)
        .await
        .unwrap();
    }

    /// A database with our output, and the registry of `members` with `threshold`.
    async fn database(
        ours: Address,
        members: &[(Address, String)],
        threshold: i64,
    ) -> (test_harness::instance::DBInstance, PgPool) {
        let instance = setup_test_db(ImportMode::None).await.unwrap();
        let pool = PgPool::connect(instance.db_url()).await.unwrap();
        add_output(&pool, ours, LABEL).await;
        for (i, (signer, url)) in members.iter().enumerate() {
            sqlx::query(
                "INSERT INTO gateway_config_coprocessors (tx_sender_address, signer_address,
                     s3_bucket_url, coprocessor_threshold, gateway_chain_id,
                     gateway_config_address, snapshot_block_number, snapshot_block_hash)
                 VALUES ($1, $2, $3, $4, 1, $5, 1, $6)",
            )
            .bind([u8::try_from(i).unwrap(); 20].as_slice())
            .bind(signer.as_slice())
            .bind(url)
            .bind(threshold)
            .bind([0u8; 20].as_slice())
            .bind([0u8; 32].as_slice())
            .execute(&pool)
            .await
            .unwrap();
        }
        (instance, pool)
    }

    async fn result(pool: &PgPool) -> (Option<String>, Option<String>) {
        sqlx::query_as("SELECT outcome, local_result FROM pad_consensus")
            .fetch_one(pool)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn ends_on_consensus_without_the_last_peers() {
        let ours = PrivateKeySigner::random();
        let [a, b, late] = [(); 3].map(|()| PrivateKeySigner::random());
        let members = vec![
            (ours.address(), peer(StatusCode::OK, None).await),
            (
                a.address(),
                peer(StatusCode::OK, Some(attestation(&a, a.address(), OURS))).await,
            ),
            // Its bucket refuses HEAD: the ranged GET reads the attestation.
            (
                b.address(),
                get_only_peer(attestation(&b, b.address(), OURS)).await,
            ),
            (late.address(), peer(StatusCode::NOT_FOUND, None).await),
        ];
        let (_instance, pool) = database(ours.address(), &members, 3).await;
        let http = reqwest::Client::new();

        // Two detectors, as Blue and Green during a round: one of them ends the check. The output
        // is 2 hours old, older than the ct128 GC and the deadline: the retention still finds it.
        let config = Config::default();
        let (mut blue, mut green) = (PassState::default(), PassState::default());
        let (first, second) = tokio::join!(
            pass(&pool, &http, &config, &mut blue),
            pass(&pool, &http, &config, &mut green)
        );
        assert_eq!(first.unwrap() + second.unwrap(), 1);
        assert_eq!(
            result(&pool).await,
            (Some("consensus".into()), Some("matches_quorum".into()))
        );
    }

    #[tokio::test]
    async fn waits_while_our_signer_is_not_registered() {
        let ours = PrivateKeySigner::random();
        let [a, b] = [(); 2].map(|()| PrivateKeySigner::random());
        let members = vec![
            (
                a.address(),
                peer(StatusCode::OK, Some(attestation(&a, a.address(), OURS))).await,
            ),
            (
                b.address(),
                peer(StatusCode::OK, Some(attestation(&b, b.address(), OURS))).await,
            ),
        ];
        let (_instance, pool) = database(ours.address(), &members, 2).await;

        // With our output, three outputs agree. Our output does not count, so the pass asks no peer
        // and ends no check.
        let ended = pass(
            &pool,
            &reqwest::Client::new(),
            &Config::default(),
            &mut PassState::default(),
        )
        .await;
        assert_eq!(ended.unwrap(), 0);
        assert_eq!(result(&pool).await, (None, None));
        let answers: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pad_consensus_answer")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(answers, 0);
    }

    #[tokio::test]
    async fn finds_only_the_outputs_after_the_last_one_found() {
        let ours = PrivateKeySigner::random();
        let members = vec![(ours.address(), peer(StatusCode::OK, None).await)];
        let (_instance, pool) = database(ours.address(), &members, 1).await;
        let mut state = PassState::default();
        let config = Config::default();
        let http = reqwest::Client::new();

        // The first pass finds the output of `database`, 2 hours old.
        pass(&pool, &http, &config, &mut state).await.unwrap();
        sqlx::query("DELETE FROM pad_consensus")
            .execute(&pool)
            .await
            .unwrap();
        // One output 30 s before it, in the overlap of 1 minute. One output 90 s before it, which is
        // too old. One new output.
        let [overlap, old, new] = [1u8, 2, 3].map(B256::repeat_byte);
        add_output_before(&pool, ours.address(), overlap, 30.0).await;
        add_output_before(&pool, ours.address(), old, 90.0).await;
        add_output_aged(&pool, ours.address(), new, 0.0).await;

        pass(&pool, &http, &config, &mut state).await.unwrap();
        let labels: Vec<Vec<u8>> =
            sqlx::query_scalar("SELECT label FROM pad_consensus ORDER BY label")
                .fetch_all(&pool)
                .await
                .unwrap();
        // The output of `database` is in the overlap too.
        let expected = [overlap, new, LABEL].map(|label| label.to_vec());
        assert_eq!(labels, expected);
    }

    #[tokio::test]
    async fn deletes_old_checks_except_drift() {
        let ours = PrivateKeySigner::random();
        let members = vec![(ours.address(), peer(StatusCode::OK, None).await)];
        let (_instance, pool) = database(ours.address(), &members, 1).await;
        // All three are older than the retention: an open check, an ended check and a drift check.
        for (label, outcome, done) in [
            (1u8, None, false),
            (2, Some("consensus"), true),
            (3, Some("drift"), true),
        ] {
            sqlx::query(
                "INSERT INTO pad_consensus (prf_instance, label, pad_length, ciphertext_digest,
                                            ciphertext128_digest, signer, created_at, outcome,
                                            done_at)
                 VALUES (0, $1, 64, $2, $3, $4, NOW() - INTERVAL '3 days', $5,
                         CASE WHEN $6 THEN NOW() - INTERVAL '2 days' END)",
            )
            .bind(B256::repeat_byte(label).as_slice())
            .bind(OURS.0.as_slice())
            .bind(OURS.1.as_slice())
            .bind(ours.address().as_slice())
            .bind(outcome)
            .bind(done)
            .execute(&pool)
            .await
            .unwrap();
        }

        pass(
            &pool,
            &reqwest::Client::new(),
            &Config::default(),
            &mut PassState::default(),
        )
        .await
        .unwrap();
        let labels: Vec<Vec<u8>> =
            sqlx::query_scalar("SELECT label FROM pad_consensus ORDER BY label")
                .fetch_all(&pool)
                .await
                .unwrap();
        // The drift check stays. The pass also adds the output of `database`.
        assert_eq!(labels, vec![B256::repeat_byte(3).to_vec(), LABEL.to_vec()]);
    }

    #[tokio::test]
    async fn another_detector_does_not_ask_again_during_a_slow_batch() {
        let ours = PrivateKeySigner::random();
        let peer_key = PrivateKeySigner::random();
        let (url, requests) = counting_peer(Duration::from_secs(3)).await;
        let members = vec![
            (ours.address(), peer(StatusCode::OK, None).await),
            (peer_key.address(), url),
        ];
        let (_instance, pool) = database(ours.address(), &members, 2).await;
        let config = Config {
            interval: Duration::from_secs(1),
            ..Config::default()
        };
        let http = reqwest::Client::new();

        // The second detector starts after the interval, while the first one waits for the peer.
        let mut blue = PassState::default();
        let first = pass(&pool, &http, &config, &mut blue);
        let second = async {
            tokio::time::sleep(Duration::from_millis(1500)).await;
            pass(&pool, &http, &config, &mut PassState::default()).await
        };
        let (first, second) = tokio::join!(first, second);
        assert_eq!((first.unwrap(), second.unwrap()), (0, 0));
        assert_eq!(requests.load(Ordering::SeqCst), 1);

        // The check stays open: its next time is after the interval, not at the end of the lease.
        let soon: bool = sqlx::query_scalar(
            "SELECT next_check_at < NOW() + INTERVAL '1 minute' FROM pad_consensus",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert!(soon);
    }

    #[tokio::test]
    async fn an_unreachable_peer_costs_one_timeout() {
        let ours = PrivateKeySigner::random();
        let [a, slow] = [(); 2].map(|()| PrivateKeySigner::random());
        let members = vec![
            (ours.address(), peer(StatusCode::OK, None).await),
            (
                a.address(),
                peer(StatusCode::OK, Some(attestation(&a, a.address(), OURS))).await,
            ),
            (slow.address(), slow_peer(Duration::from_secs(30)).await),
        ];
        let (_instance, pool) = database(ours.address(), &members, 3).await;
        for label in 1..20u8 {
            add_output(&pool, ours.address(), B256::repeat_byte(label)).await;
        }
        let timeout = Duration::from_secs(1);
        let http = reqwest::Client::builder().timeout(timeout).build().unwrap();

        // 20 outputs, checked one after another, would take 20 timeouts. At the same time, 2.
        let started = std::time::Instant::now();
        pass(&pool, &http, &Config::default(), &mut PassState::default())
            .await
            .unwrap();
        assert!(started.elapsed() < timeout * 10, "{:?}", started.elapsed());
    }

    #[tokio::test]
    async fn records_drift_and_ends_at_the_deadline() {
        let ours = PrivateKeySigner::random();
        let [a, b, c, forged, late] = [(); 5].map(|()| PrivateKeySigner::random());
        let mut members = vec![(ours.address(), peer(StatusCode::OK, None).await)];
        for key in [&a, &b, &c] {
            let header = attestation(key, key.address(), OTHER);
            members.push((key.address(), peer(StatusCode::OK, Some(header)).await));
        }
        // Signed by another key than the registered signer: not a vote.
        let header = attestation(&a, forged.address(), OURS);
        members.push((forged.address(), peer(StatusCode::OK, Some(header)).await));
        members.push((late.address(), peer(StatusCode::NOT_FOUND, None).await));
        let (_instance, pool) = database(ours.address(), &members, 4).await;
        let http = reqwest::Client::new();
        let config = Config {
            interval: Duration::ZERO,
            ..Config::default()
        };

        // Drift does not end the check while a peer can still answer.
        pass(&pool, &http, &config, &mut PassState::default())
            .await
            .unwrap();
        assert_eq!(result(&pool).await, (None, None));
        let invalid: Vec<(Vec<u8>, String)> = sqlx::query_as(
            "SELECT peer_signer, reason FROM pad_consensus_answer WHERE reason IS NOT NULL",
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(invalid.len(), 1);
        assert_eq!(invalid[0].0, forged.address().to_vec());
        assert!(
            invalid[0].1.starts_with("bad signature"),
            "{}",
            invalid[0].1
        );

        // Three peers agree on other digests: with the threshold of 4, no group has a quorum.
        let expired = Config {
            deadline: Duration::ZERO,
            ..config
        };
        pass(&pool, &http, &expired, &mut PassState::default())
            .await
            .unwrap();
        assert_eq!(
            result(&pool).await,
            (Some("drift".into()), Some("inconclusive".into()))
        );
        // The drift evidence stays after the deadline.
        pass(&pool, &http, &expired, &mut PassState::default())
            .await
            .unwrap();
        assert_eq!(result(&pool).await.0, Some("drift".into()));
    }
}
