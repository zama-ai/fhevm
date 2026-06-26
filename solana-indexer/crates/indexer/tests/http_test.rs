//! axum-level tests: seed a lineage in a transactional test DB, then exercise
//! POST /build_proof (200/404/422), the GET leaf lookup (200/404), and the
//! health/version endpoints.
//!
//! Requires DATABASE_URL pointing at the migrated indexer DB. Each test uses a
//! unique value_key/PDA so they do not collide, and cleans up its rows.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use indexer::http::server::router;
use indexer::http::AppState;
use indexer::metrics::Metrics;
use indexer::store::repositories::lineage_repo::{EventInsert, LineageRepo, LineageState};
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower::ServiceExt;
use zama_solana_acl::historical_access_leaf_commitment;
use zama_solana_acl::lineage::LineageEvent;

fn database_url() -> Option<String> {
    std::env::var("DATABASE_URL").ok()
}

async fn seed_lineage(repo: &LineageRepo, pda: [u8; 32], value_key: [u8; 32]) {
    // Clean any prior rows for this pda/value_key.
    sqlx::query("DELETE FROM lineage_events WHERE pda = $1")
        .bind(&pda[..])
        .execute(repo.pool())
        .await
        .unwrap();
    sqlx::query("DELETE FROM lineage_state WHERE pda = $1 OR value_key = $2")
        .bind(&pda[..])
        .bind(&value_key[..])
        .execute(repo.pool())
        .await
        .unwrap();

    let mut tx = repo.pool().begin().await.unwrap();
    // A single rotation over [subject] for old handle h10 => leaf 0 historical.
    let old_handle = [10u8; 32];
    let subject = [1u8; 32];
    repo.insert_event(
        &mut tx,
        &EventInsert {
            pda: &pda,
            event_index: 0,
            signature: "sig-1",
            slot: 100,
        },
        &LineageEvent::Rotation {
            old_handle,
            subjects_before_rotation: vec![subject],
        },
    )
    .await
    .unwrap();
    repo.upsert_state(
        &mut tx,
        &LineageState {
            pda,
            value_key: Some(value_key),
            current_handle: [11u8; 32],
            current_subjects: vec![subject],
            leaf_count: 1,
        },
    )
    .await
    .unwrap();
    tx.commit().await.unwrap();
}

fn app(repo: LineageRepo) -> axum::Router {
    let state = Arc::new(AppState {
        repo,
        rpc: None, // unverified path: proofs returned with verified=false
        metrics: Metrics::new(),
    });
    router(state)
}

async fn body_json(resp: axum::http::Response<Body>) -> Value {
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn build_proof_and_leaf_lookup() {
    let Some(url) = database_url() else {
        eprintln!("skipping http_test: DATABASE_URL not set");
        return;
    };
    let pool = PgPoolOptions::new().connect(&url).await.unwrap();
    let repo = LineageRepo::new(pool);

    let pda = [0x77u8; 32];
    let value_key = [0x88u8; 32];
    seed_lineage(&repo, pda, value_key).await;

    // --- POST /build_proof 200 ---
    let resp = app(repo.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/build_proof")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"value_key": hex::encode(value_key), "leaf_index": 0})
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let json = body_json(resp).await;
    assert_eq!(json["leaf_count"], 1);
    assert_eq!(json["verified"], false);
    let proof_bytes = hex::decode(json["mmr_proof_bytes"].as_str().unwrap()).unwrap();
    assert_eq!(
        proof_bytes[0], 0x01,
        "leaf 0 from a Rotation -> historical prefix"
    );

    // The proof verifies against the reconstructed leaf via the shared oracle.
    use borsh::BorshDeserialize;
    use zama_solana_acl::{mmr_peaks_from_leaves, mmr_verify, MmrProof};
    let leaf = historical_access_leaf_commitment(pda, 0, [10u8; 32], [1u8; 32]);
    let peaks = mmr_peaks_from_leaves(&[leaf]);
    let proof = MmrProof::try_from_slice(&proof_bytes[1..]).unwrap();
    assert!(mmr_verify(&peaks, 1, leaf, &proof));

    // --- POST /build_proof 404 unknown value_key ---
    let resp = app(repo.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/build_proof")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"value_key": hex::encode([0x00u8; 32]), "leaf_index": 0})
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    assert_eq!(body_json(resp).await["label"], "lineage_not_found");

    // --- POST /build_proof 422 out-of-range ---
    let resp = app(repo.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/build_proof")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"value_key": hex::encode(value_key), "leaf_index": 5})
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body_json(resp).await["label"], "leaf_index_out_of_range");

    // --- GET leaf lookup 200 ---
    let uri = format!(
        "/lineage/{}/leaf?subject={}&handle={}",
        hex::encode(value_key),
        hex::encode([1u8; 32]),
        hex::encode([10u8; 32]),
    );
    let resp = app(repo.clone())
        .oneshot(Request::builder().uri(&uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let json = body_json(resp).await;
    assert_eq!(json["leaf_index"], 0);
    assert_eq!(json["leaf_count"], 1);

    // --- GET leaf lookup 404 (wrong subject) ---
    let uri = format!(
        "/lineage/{}/leaf?subject={}&handle={}",
        hex::encode(value_key),
        hex::encode([0xEEu8; 32]),
        hex::encode([10u8; 32]),
    );
    let resp = app(repo.clone())
        .oneshot(Request::builder().uri(&uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    assert_eq!(body_json(resp).await["label"], "leaf_not_found");

    // cleanup
    sqlx::query("DELETE FROM lineage_events WHERE pda = $1")
        .bind(&pda[..])
        .execute(repo.pool())
        .await
        .unwrap();
    sqlx::query("DELETE FROM lineage_state WHERE pda = $1")
        .bind(&pda[..])
        .execute(repo.pool())
        .await
        .unwrap();
}

#[tokio::test]
async fn health_and_version_endpoints() {
    let Some(url) = database_url() else {
        eprintln!("skipping http_test: DATABASE_URL not set");
        return;
    };
    let pool = PgPoolOptions::new().connect(&url).await.unwrap();
    let repo = LineageRepo::new(pool);

    let resp = app(repo.clone())
        .oneshot(
            Request::builder()
                .uri("/liveness")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = app(repo.clone())
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = app(repo)
        .oneshot(
            Request::builder()
                .uri("/version")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let json = body_json(resp).await;
    assert!(json["version"].is_string());
    assert!(json["git_sha"].is_string());
}
