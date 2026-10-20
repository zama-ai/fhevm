//! Shared fixtures for tests composing ingestion with actual commitment writers.
// Reuse the listener's event fixtures instead of maintaining another ABI encoder.
// That crate owns its formatting (and uses a different rustfmt configuration).
#[rustfmt::skip]
#[path = "../../host-listener/tests/ingest_fixtures/mod.rs"]
pub(crate) mod fixtures;

use fhevm_engine_common::{
    chain_id::ChainId,
    versioning::{begin_write_guarded, GcsRollbackPolicy, WriteGuard},
};
use fixtures::*;
use host_listener::database::{
    ingest::{ingest_block_logs, BlockLogs},
    tfhe_event_propagate::Database,
};
use sqlx::PgPool;
use test_harness::instance::DBInstance;

pub(crate) async fn partial_block() -> anyhow::Result<(Database, PgPool, DBInstance)> {
    let (mut db, pool, instance) = setup().await?;
    let b = block();
    let a = computed_handle(1);
    let tx = computed_handle(0xe1);
    ingest_block_logs(
        ChainId::try_from(42u64)?,
        &mut db,
        &BlockLogs {
            summary: b,
            finalized: true,
            catchup: true,
            logs: vec![
                rpc_log(
                    &host_listener::contracts::TfheContract::FheNot {
                        caller: alloy::primitives::Address::ZERO,
                        ct: computed_handle(0x11),
                        result: a,
                    },
                    &b,
                    tx,
                    0,
                ),
                rpc_log(&allowed(a), &b, tx, 1),
                rpc_log(&trivial_encrypt(computed_handle(2)), &b, tx, 2),
            ],
        },
        &None,
        &None,
        &None,
        &None,
        &None,
        OPTIONS,
    )
    .await?
    .require_complete()?;
    // Simulate a completed FHE operation; the commitment itself is never seeded.
    sqlx::query("UPDATE computations SET is_completed=TRUE WHERE output_handle=$1")
        .bind(a.as_slice())
        .execute(&pool)
        .await?;
    sqlx::query("INSERT INTO ciphertexts (tenant_id,handle,ciphertext,ciphertext_version,ciphertext_type) VALUES (0,$1,'\\xaabb',0,5)")
        .bind(a.as_slice()).execute(&pool).await?;
    Ok((db, pool, instance))
}

pub(crate) async fn repair_is_contained(db: &mut Database, pool: &PgPool) -> anyhow::Result<()> {
    let b = block();
    let error = ingest(
        db,
        &b,
        vec![rpc_log(
            &allowed(computed_handle(2)),
            &b,
            computed_handle(0xe1),
            3,
        )],
        true,
    )
    .await
    .unwrap_err();
    assert!(
        error.to_string().contains("inventory already committed"),
        "{error}"
    );
    assert!(fhevm_engine_common::chain_recovery::is_halted(pool).await?);
    assert!(fhevm_engine_common::chain_recovery::readiness_blocked(pool).await?);
    assert_eq!(
        computation_flags(pool, computed_handle(2)).await?,
        (false, true)
    );
    let producers: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM handle_producer_block")
        .fetch_one(pool)
        .await?;
    assert_eq!(
        producers, 1,
        "repair must roll back inventory as well as work eligibility"
    );
    // This is the same fence used by workers, SNS uploads and Gateway sends.
    assert!(matches!(
        begin_write_guarded(pool, false, GcsRollbackPolicy::Skip).await?,
        WriteGuard::Skip
    ));
    Ok(())
}

/// Capture the actual signed S3 PUT without Docker or cloud credentials.
pub(crate) async fn object_store(
) -> anyhow::Result<(aws_sdk_s3::Client, tokio::task::JoinHandle<Vec<u8>>)> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let endpoint = format!("http://{}", listener.local_addr()?);
    let task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut request = Vec::new();
        let body = loop {
            let mut buf = [0; 4096];
            let n = socket.read(&mut buf).await.unwrap();
            assert!(n > 0);
            request.extend_from_slice(&buf[..n]);
            if let Some(end) = request.windows(4).position(|w| w == b"\r\n\r\n") {
                let headers = String::from_utf8_lossy(&request[..end]).to_ascii_lowercase();
                assert!(headers.starts_with("put "));
                let length: usize = headers
                    .lines()
                    .find_map(|l| {
                        l.strip_prefix("content-length:")
                            .map(|s| s.trim().parse().unwrap())
                    })
                    .unwrap();
                if request.len() >= end + 4 + length {
                    break request[end + 4..end + 4 + length].to_vec();
                }
            }
        };
        socket
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
            .await
            .unwrap();
        body
    });
    let client = aws_sdk_s3::Client::from_conf(
        aws_sdk_s3::Config::builder()
            .behavior_version_latest()
            .region(aws_sdk_s3::config::Region::new("us-east-1"))
            .credentials_provider(aws_sdk_s3::config::Credentials::new(
                "test", "test", None, None, "test",
            ))
            .endpoint_url(endpoint)
            .force_path_style(true)
            .build(),
    );
    Ok((client, task))
}
