//! Five coprocessors in process (prf-service and the stand-in pad-worker on one database each, no
//! proxy), and stand-in connectors that need a quorum of three. Needs a Postgres that this test may
//! create databases and a role in: `PRF_STANDINS_TEST_ADMIN_URL`, by default
//! `postgres://postgres@127.0.0.1:55432/postgres`.

use alloy::{
    primitives::{Address, B256},
    signers::local::PrivateKeySigner,
};
use prf_service::{AuthConfig, Config, PrfService};
use prf_standins::{
    connector::{Connector, Coprocessor, ProbeOptions, Retrieval},
    pad_worker::{PadWorker, PadWorkerConfig},
};
use reqwest::Method;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{net::TcpListener, num::NonZeroUsize, sync::Arc, time::Duration};
use tokio_util::sync::CancellationToken;

const CHAIN_ID: u64 = 12345;
const PROTOCOL_CONFIG: Address = Address::repeat_byte(0xc0);
/// Party 0 refuses more than one pending 2048-bit pad per connector.
const PARTY0_PENDING_BITS: i64 = 4096;
/// Party 2 refuses a third request in progress from one connector.
const PARTY2_IN_FLIGHT: usize = 2;

/// The faults of each party, by label first byte. Mirrors the preview check.
fn faults(party: usize) -> String {
    let mut faults = vec!["4a:delay=5s", "7f:delay=8s", "e1:fail"];
    if party == 0 {
        faults.extend([
            "d1:delay=3s",
            "b1:wrong_digest",
            "a1:unsigned",
            "a2:foreign_signer",
            "e2:fail",
        ]);
    }
    if party <= 1 {
        faults.push("b2:wrong_digest");
    }
    if party <= 2 {
        faults.extend(["b3:wrong_digest", "a5:unsigned"]);
    }
    faults.join(",")
}

fn admin_url() -> String {
    std::env::var("PRF_STANDINS_TEST_ADMIN_URL")
        .unwrap_or_else(|_| "postgres://postgres@127.0.0.1:55432/postgres".into())
}

fn with_database(url: &str, user: &str, database: &str) -> String {
    let mut url = reqwest::Url::parse(url).unwrap();
    url.set_username(user).unwrap();
    url.set_password(Some(user)).unwrap();
    url.set_path(database);
    url.to_string()
}

async fn create_database(admin: &PgPool, name: &str) -> PgPool {
    sqlx::query(&format!("DROP DATABASE IF EXISTS {name} WITH (FORCE)"))
        .execute(admin)
        .await
        .unwrap();
    sqlx::query(&format!("CREATE DATABASE {name}"))
        .execute(admin)
        .await
        .unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(12)
        .connect(&with_database(&admin_url(), "postgres", name))
        .await
        .unwrap();
    sqlx::query(
        "CREATE TABLE _fhevm_versioning_bootstrap (singleton BOOLEAN PRIMARY KEY DEFAULT TRUE)",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::migrate!("../db-migration/migrations")
        .run(&pool)
        .await
        .unwrap();
    sqlx::raw_sql(include_str!(
        "../../db-migration/db-scripts/grant_pad_roles.sql"
    ))
    .execute(&pool)
    .await
    .unwrap();
    pool
}

struct Party {
    url: String,
    signer: Address,
    pool: PgPool,
}

async fn start_party(admin: &PgPool, i: usize, cancel: &CancellationToken) -> Party {
    let name = format!("prf_standins_party_{i}");
    let pool = create_database(admin, &name).await;
    let key = PrivateKeySigner::random();
    let port = TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let service = PrfService::new(
        Config {
            http_endpoint: ([127, 0, 0, 1], port).into(),
            // The role of prf-service, to check its grants.
            database_url: with_database(&admin_url(), "prf_service", &name),
            database_pool_size: 4,
            max_pending_bits_per_connector: if i == 0 { PARTY0_PENDING_BITS } else { 1 << 20 },
            max_in_flight_per_connector: NonZeroUsize::new(if i == 2 {
                PARTY2_IN_FLIGHT
            } else {
                1024
            })
            .unwrap(),
            max_adds_per_connector: 4,
            pad_poll_interval: Duration::from_secs(1),
            retry_after: Duration::from_secs(1),
            auth: AuthConfig {
                ethereum_chain_id: CHAIN_ID,
                protocol_config_address: PROTOCOL_CONFIG,
                coprocessor_signer_address: key.address(),
            },
        },
        cancel.clone(),
    )
    .await
    .unwrap();
    tokio::spawn(service.start());
    let worker = PadWorker::new(
        pool.clone(),
        key.clone(),
        PadWorkerConfig {
            faults: faults(i).parse().unwrap(),
            concurrency: 4,
            claim_timeout: Duration::from_secs(10),
            max_attempts: 3,
            poll_interval: Duration::from_secs(1),
            block_bytes: 32,
        },
    );
    tokio::spawn(worker.run(cancel.clone()));
    Party {
        url: format!("http://127.0.0.1:{port}"),
        signer: key.address(),
        pool,
    }
}

fn connector(parties: &[Party], down: &[usize], deadline: Duration) -> Arc<Connector> {
    Arc::new(Connector {
        key: PrivateKeySigner::random(),
        domain: fhevm_sig_auth::domain(CHAIN_ID, PROTOCOL_CONFIG),
        coprocessors: parties
            .iter()
            .enumerate()
            .map(|(i, p)| Coprocessor {
                // Nothing listens on port 1.
                url: if down.contains(&i) {
                    "http://127.0.0.1:1".into()
                } else {
                    p.url.clone()
                },
                signer: p.signer,
            })
            .collect(),
        threshold: NonZeroUsize::new(3).unwrap(),
        http: reqwest::Client::new(),
        header_validity: Duration::from_secs(60),
        deadline,
        backoff: Duration::from_millis(200),
        body_wait: None,
        fail_first_fallback: false,
    })
}

fn retrieval(prefix: u8, get_from: Option<usize>) -> Retrieval {
    let mut label = B256::from(rand::random::<[u8; 32]>());
    label.0[0] = prefix;
    Retrieval {
        prf: 0,
        label,
        pad_length: 64,
        get_from,
        first_expires: None,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn retrieval_matrix() {
    let plain = ProbeOptions::default();
    let admin = PgPoolOptions::new()
        .max_connections(2)
        .connect(&admin_url())
        .await
        .expect("PRF_STANDINS_TEST_ADMIN_URL: a Postgres where the test can create databases");
    sqlx::raw_sql(
        "DO $$ BEGIN
           IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'prf_service') THEN
             CREATE ROLE prf_service LOGIN PASSWORD 'prf_service';
           END IF;
         END $$",
    )
    .execute(&admin)
    .await
    .unwrap();
    let cancel = CancellationToken::new();
    let mut parties = Vec::new();
    for i in 0..5 {
        parties.push(start_party(&admin, i, &cancel).await);
    }
    let deadline = Duration::from_secs(15);

    // Four connectors on one label: every one gets the body, and the row is added by one of them.
    let connectors: Vec<_> = (0..4).map(|_| connector(&parties, &[], deadline)).collect();
    let happy = retrieval(0x00, None);
    let reports =
        futures_util::future::join_all(connectors.iter().map(|c| c.retrieve(&happy))).await;
    for report in &reports {
        assert!(report.ok, "{report:?}");
    }
    let senders: Vec<Address> = connectors.iter().map(|c| c.key.address()).collect();
    for party in &parties {
        let triggered_by: Vec<u8> =
            sqlx::query_scalar("SELECT triggered_by FROM pads WHERE label = $1")
                .bind(happy.label.as_slice())
                .fetch_one(&party.pool)
                .await
                .unwrap();
        assert!(senders.contains(&Address::from_slice(&triggered_by)));
    }

    let one = connector(&parties, &[], deadline);
    // A slow GET target, a wrong body, a failed row on the GET target: the body comes from a
    // quorum member.
    for prefix in [0xd1, 0xb1, 0xe2] {
        let report = one.retrieve(&retrieval(prefix, Some(0))).await;
        assert!(
            report.ok && report.fallback && report.body_from != Some(0),
            "{prefix:x}: {report:?}"
        );
    }
    // An unsigned attestation is discarded, but the GET target's body still matches the quorum.
    // The three HEADs can reach quorum before the GET answers (RFC-038 step 5 then falls back),
    // so retry on a new label until party 0's reply is in the report.
    let mut a1_checked = false;
    for _ in 0..5 {
        let report = one.retrieve(&retrieval(0xa1, Some(0))).await;
        assert!(report.ok, "{report:?}");
        let Some(reply) = report.replies.iter().find(|r| r.coprocessor == 0) else {
            continue;
        };
        assert_eq!(reply.attestation, Some("rejected"), "{report:?}");
        assert_eq!(report.body_from, Some(0), "{report:?}");
        a1_checked = true;
        break;
    }
    assert!(a1_checked, "party 0 never answered before quorum");
    // QA row S2: an attestation signed by a key that is not party 0's signer is discarded too.
    let mut a2_checked = false;
    for _ in 0..5 {
        let report = one.retrieve(&retrieval(0xa2, Some(0))).await;
        assert!(report.ok, "{report:?}");
        let Some(reply) = report.replies.iter().find(|r| r.coprocessor == 0) else {
            continue;
        };
        assert_eq!(reply.attestation, Some("rejected"), "{report:?}");
        a2_checked = true;
        break;
    }
    assert!(a2_checked, "party 0 never answered before quorum");
    // Fallback policy 2: when the GET target's attestation is in the quorum group, its body is
    // awaited, not fetched again. Three HEADs can still win before the target's headers arrive;
    // then, as for a slow target, the connector falls back at once.
    let mut waiting = Arc::into_inner(connector(&parties, &[], deadline)).unwrap();
    waiting.body_wait = Some(Duration::from_secs(2));
    let waiting = Arc::new(waiting);
    // 64 bits in 2-bit blocks of 32 bytes.
    let body = 32 * 32;
    let mut fallbacks = [0; 2];
    let mut bytes = [0; 2];
    for _ in 0..20 {
        for (policy, c) in [&one, &waiting].into_iter().enumerate() {
            let report = c.retrieve(&retrieval(0x00, Some(0))).await;
            assert!(report.ok, "{report:?}");
            if report.fallback {
                assert!(report.body_wait_ms.is_none(), "{report:?}");
                assert!(
                    report.gets == 2 && report.bytes >= body && report.bytes <= 2 * body,
                    "{report:?}"
                );
                fallbacks[policy] += 1;
            } else {
                assert!(report.gets == 1 && report.bytes == body, "{report:?}");
            }
            bytes[policy] += report.bytes;
        }
    }
    println!(
        "fallbacks in 20 retrievals: policy 1 {} ({} bytes), policy 2 {} ({} bytes)",
        fallbacks[0], bytes[0], fallbacks[1], bytes[1]
    );
    assert!(fallbacks[1] < 20, "policy 2 never used the target's body");
    let report = waiting.retrieve(&retrieval(0xd1, Some(0))).await;
    assert!(
        report.ok && report.fallback && report.body_wait_ms.is_none(),
        "{report:?}"
    );
    // QA row R7: the first fallback GET fails, the next quorum member serves the body.
    let mut failing = Arc::into_inner(connector(&parties, &[], deadline)).unwrap();
    failing.fail_first_fallback = true;
    let failing = Arc::new(failing);
    let report = failing.retrieve(&retrieval(0xd1, Some(0))).await;
    assert!(report.ok && report.fallback, "{report:?}");
    assert!(
        report
            .replies
            .iter()
            .any(|r| r.code.as_deref() == Some("simulated_failure")),
        "{report:?}"
    );
    // Two wrong digests are outvoted.
    let report = one.retrieve(&retrieval(0xb2, Some(4))).await;
    assert!(report.ok && !report.fallback, "{report:?}");
    // Three wrong digests, or three unsigned attestations: no quorum.
    for prefix in [0xb3, 0xa5] {
        let report = one.retrieve(&retrieval(prefix, None)).await;
        assert_eq!(report.error, Some("no_quorum"), "{prefix:x}: {report:?}");
    }
    // A permanent failure everywhere: `500` is not retried, by GET or HEAD, so the label fails
    // without waiting for the deadline.
    let report = connector(&parties, &[], Duration::from_secs(10))
        .retrieve(&retrieval(0xe1, Some(0)))
        .await;
    assert_eq!(report.error, Some("no_quorum"), "{report:?}");
    assert!(report.elapsed_ms < 5_000, "{report:?}");
    assert!(
        report
            .replies
            .iter()
            .all(|r| r.status == Some(500) && r.attempts == 1),
        "{report:?}"
    );
    // The request loop alone stops at the first `500`, GET or HEAD.
    let path = retrieval(0xe1, None).path();
    for method in [Method::GET, Method::HEAD] {
        let reply = one.ask_one(1, method.clone(), &path).await.unwrap();
        assert!(
            reply.status == Some(500)
                // A HEAD reply has no body, so no code.
                && (method == Method::HEAD
                    || reply.code.as_deref() == Some("prf_evaluation_failed"))
                && reply.attempts == 1
                && reply.retried.is_empty(),
            "{reply:?}"
        );
    }

    // Stale headers: re-signed after `auth_expired`.
    let mut stale = retrieval(0x00, None);
    stale.first_expires = Some(fhevm_sig_auth::unix_now() - 600);
    let report = one.retrieve(&stale).await;
    assert!(report.ok && report.resigns >= 3, "{report:?}");

    // Two coprocessors down: quorum and the body from the others.
    let report = connector(&parties, &[3, 4], deadline)
        .retrieve(&retrieval(0x00, Some(4)))
        .await;
    assert!(report.ok && report.fallback, "{report:?}");
    // Three down: no quorum before the deadline.
    let report = connector(&parties, &[2, 3, 4], Duration::from_secs(3))
        .retrieve(&retrieval(0x00, None))
        .await;
    assert_eq!(report.error, Some("deadline"), "{report:?}");

    // An evaluation longer than the deadline fails, then the stored output is served at once.
    let slow = retrieval(0x7f, None);
    let report = connector(&parties, &[], Duration::from_secs(4))
        .retrieve(&slow)
        .await;
    assert_eq!(report.error, Some("deadline"), "{report:?}");
    tokio::time::sleep(Duration::from_secs(6)).await;
    let report = one.retrieve(&slow).await;
    assert!(report.ok && report.elapsed_ms < 2_000, "{report:?}");

    // The evaluation cap of party 0: two 2048-bit pads pending at most, the others get 429.
    let flood = connector(&parties, &[], deadline);
    let paths: Vec<String> = (0..6)
        .map(|_| {
            let mut r = retrieval(0x4a, None);
            r.pad_length = 2048;
            r.path()
        })
        .collect();
    let results = futures_util::future::join_all(
        paths.iter().map(|p| flood.probe(0, Method::GET, p, &plain)),
    )
    .await;
    let mut statuses: Vec<u16> = results.iter().map(|r| r.as_ref().unwrap().0).collect();
    statuses.sort();
    assert_eq!(statuses, vec![200, 200, 429, 429, 429, 429], "{results:?}");
    assert_eq!(
        results
            .iter()
            .find(|r| r.as_ref().unwrap().0 == 429)
            .unwrap()
            .as_ref()
            .unwrap()
            .2
            .as_deref(),
        Some("1")
    );
    // One request per connector and label at a time.
    let path = retrieval(0x4a, None).path();
    let (first, second) = tokio::join!(flood.probe(1, Method::GET, &path, &plain), async {
        tokio::time::sleep(Duration::from_millis(500)).await;
        flood.probe(1, Method::HEAD, &path, &plain).await
    });
    assert_eq!((first.unwrap().0, second.unwrap().0), (200, 429));
    // The request cap of party 2, across labels: the third request in progress gets 429.
    let paths: Vec<String> = (0..=PARTY2_IN_FLIGHT)
        .map(|_| retrieval(0x4a, None).path())
        .collect();
    let results = futures_util::future::join_all(
        paths.iter().map(|p| flood.probe(2, Method::GET, p, &plain)),
    )
    .await;
    let mut statuses: Vec<u16> = results.iter().map(|r| r.as_ref().unwrap().0).collect();
    statuses.sort();
    assert_eq!(statuses, vec![200, 200, 429], "{results:?}");

    // Path checks of prf-service.
    let label = hex::encode(B256::repeat_byte(0x0a));
    for (path, status, code) in [
        (
            format!("/v1/prf/0/labels/{}/64", label.to_uppercase()),
            400,
            "non_canonical_path",
        ),
        (
            format!("/v1/prf/00/labels/{label}/64"),
            400,
            "non_canonical_path",
        ),
        (
            format!("/v1/prf/0/labels/{label}/0"),
            400,
            "non_canonical_path",
        ),
        (
            format!("/v1/prf/0/labels/{label}/2049"),
            400,
            "non_canonical_path",
        ),
        (format!("/v1/prf/1/labels/{label}/64"), 404, "unknown_prf"),
        // QA row G4: the encodings the proxy forwards unchanged.
        (
            format!("/v1/prf/0/labels/{label}/064"),
            400,
            "non_canonical_path",
        ),
        (
            format!("/v1/prf/0/labels/{label}/0x40"),
            400,
            "non_canonical_path",
        ),
        (
            format!("/v1/prf/0/labels/{label}/%36%34"),
            400,
            "non_canonical_path",
        ),
        (
            format!("/v1/prf/0/labels/{label}/+64"),
            400,
            "non_canonical_path",
        ),
        (
            format!("/v1/prf/0/labels/{label}/65536"),
            400,
            "non_canonical_path",
        ),
        (
            format!("/v1/prf/0/labels/{label}/{}", "9".repeat(30)),
            400,
            "non_canonical_path",
        ),
        (
            format!("/v1/prf/0/labels/0x{label}/64"),
            400,
            "non_canonical_path",
        ),
    ] {
        let rows_before: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pads")
            .fetch_one(&parties[1].pool)
            .await
            .unwrap();
        let (got, error, _) = one.probe(1, Method::GET, &path, &plain).await.unwrap();
        let got_code = error.map(|e| e.code);
        assert_eq!((got, got_code.as_deref()), (status, Some(code)), "{path}");
        let rows_after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pads")
            .fetch_one(&parties[1].pool)
            .await
            .unwrap();
        assert_eq!(rows_after, rows_before, "{path} must not create a row");
    }
    let (got, _, _) = one
        .probe(
            1,
            Method::HEAD,
            &format!("/v1/prf/0/labels/{label}/2048"),
            &plain,
        )
        .await
        .unwrap();
    assert_eq!(got, 200);
    cancel.cancel();
}
