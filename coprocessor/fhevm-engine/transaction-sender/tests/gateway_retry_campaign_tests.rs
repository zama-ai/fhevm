//! Focused preservation/recovery regressions. The broader campaign is described
//! in docs/transaction-sender-http-proof-retry-test-protocol.md.
mod common;
mod support;

use alloy::primitives::U256;
use alloy::providers::ProviderBuilder;
use common::{CiphertextCommits, InputVerification, SignerType, TestEnvironment};
use rstest::rstest;
use serial_test::serial;
use std::time::Duration;
use support::{Fault, FaultProxy};
use transaction_sender::{
    gateway_http_client, ConfigSettings, FillersWithoutNonceManagement, NonceManagedProvider,
    TransactionSender,
};

#[rstest]
#[case::estimate_408("eth_estimateGas", Fault::HttpError(408), true, true, 0)]
#[case::nonce_408("eth_getTransactionCount", Fault::HttpError(408), true, true, 0)]
#[case::send_408("eth_sendRawTransactionSync", Fault::HttpError(408), true, false, 0)]
#[case::estimate_429("eth_estimateGas", Fault::HttpError(429), false, true, 0)]
#[case::nonce_429("eth_getTransactionCount", Fault::HttpError(429), false, true, 0)]
#[case::send_429("eth_sendRawTransactionSync", Fault::HttpError(429), false, false, 0)]
#[case::estimate_500("eth_estimateGas", Fault::HttpError(500), true, true, 0)]
#[case::nonce_500("eth_getTransactionCount", Fault::HttpError(500), true, true, 0)]
#[case::send_500("eth_sendRawTransactionSync", Fault::HttpError(500), true, false, 0)]
#[case::estimate_502("eth_estimateGas", Fault::HttpError(502), true, true, 0)]
#[case::nonce_502("eth_getTransactionCount", Fault::HttpError(502), true, true, 0)]
#[case::send_502("eth_sendRawTransactionSync", Fault::HttpError(502), true, false, 0)]
#[case::estimate_503("eth_estimateGas", Fault::HttpError(503), false, true, 0)]
#[case::nonce_503("eth_getTransactionCount", Fault::HttpError(503), false, true, 0)]
#[case::send_503("eth_sendRawTransactionSync", Fault::HttpError(503), false, false, 0)]
#[case::estimate_504("eth_estimateGas", Fault::HttpError(504), true, true, 0)]
#[case::nonce_504("eth_getTransactionCount", Fault::HttpError(504), true, true, 0)]
#[case::send_504("eth_sendRawTransactionSync", Fault::HttpError(504), true, false, 0)]
#[case::unreadable_estimate_html("eth_estimateGas", Fault::RawResponse { status: 200, body: "<html>maintenance GW_BARE_SECRET</html>".into() }, true, true, 0)]
#[case::unreadable_estimate_empty("eth_estimateGas", Fault::RawResponse { status: 200, body: "".into() }, true, true, 0)]
#[case::unreadable_estimate_malformed("eth_estimateGas", Fault::RawResponse { status: 200, body: "{invalid GW_BARE_SECRET".into() }, true, true, 0)]
#[case::unreadable_nonce_html("eth_getTransactionCount", Fault::RawResponse { status: 200, body: "<html>maintenance GW_BARE_SECRET</html>".into() }, true, true, 0)]
#[case::unreadable_nonce_empty("eth_getTransactionCount", Fault::RawResponse { status: 200, body: "".into() }, true, true, 0)]
#[case::unreadable_nonce_malformed("eth_getTransactionCount", Fault::RawResponse { status: 200, body: "{invalid GW_BARE_SECRET".into() }, true, true, 0)]
#[case::unreadable_send_html("eth_sendRawTransactionSync", Fault::RawResponse { status: 200, body: "<html>maintenance GW_BARE_SECRET</html>".into() }, true, true, 0)]
#[case::unreadable_send_empty("eth_sendRawTransactionSync", Fault::RawResponse { status: 200, body: "".into() }, true, true, 0)]
#[case::unreadable_send_malformed("eth_sendRawTransactionSync", Fault::RawResponse { status: 200, body: "{invalid GW_BARE_SECRET".into() }, true, true, 0)]
#[case::long_outage_true("eth_sendRawTransactionSync", Fault::HttpError(503), true, true, 300)]
#[case::long_outage_false("eth_sendRawTransactionSync", Fault::HttpError(503), false, true, 300)]
#[case::estimate_never_answers(
    "eth_estimateGas",
    Fault::DiscardAfter(Duration::from_secs(90)),
    true,
    true,
    0
)]
#[case::send_local_timeout(
    "eth_sendRawTransactionSync",
    Fault::DiscardAfter(Duration::from_secs(10)),
    false,
    false,
    0
)]
#[case::conduit_deadline("eth_sendRawTransactionSync", Fault::RpcError { code: -32000, message: "context deadline exceeded".into() }, true, true, 0)]
#[case::rpc_unavailable("eth_estimateGas", Fault::RpcError { code: -32603, message: "temporarily unavailable".into() }, false, false, 0)]
#[case::connection_drop_estimate("eth_estimateGas", Fault::DisconnectBeforeForward, true, true, 0)]
#[case::connection_drop_send(
    "eth_sendRawTransactionSync",
    Fault::DisconnectBeforeForward,
    false,
    false,
    0
)]
#[tokio::test]
#[ignore = "extended validation campaign"]
#[serial(db)]
async fn transient_failure_preserves_near_exhausted_proof_and_recovers(
    #[case] method: &str,
    #[case] fault: Fault,
    #[case] remove: bool,
    #[case] verified: bool,
    #[case] outage_seconds: u64,
) -> anyhow::Result<()> {
    let conf = ConfigSettings {
        verify_proof_resp_max_retries: 15,
        verify_proof_remove_after_max_retries: remove,
        error_sleep_initial_secs: 1,
        error_sleep_max_secs: 4,
        send_txn_sync_timeout_secs: 1,
        graceful_shutdown_timeout: Duration::from_secs(5),
        ..Default::default()
    };
    let env = TestEnvironment::new_with_config(SignerType::PrivateKey, conf, false).await?;
    let deploy = env.http_provider()?;
    let input = InputVerification::deploy(&deploy, false, false, false, false).await?;
    let ciphertext = CiphertextCommits::deploy(&deploy, false).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let url = proxy.url();
    let inner = ProviderBuilder::default()
        .filler(FillersWithoutNonceManagement::default())
        .wallet(env.wallet.clone())
        .connect_reqwest(gateway_http_client(&url)?, url);
    let provider = NonceManagedProvider::new(inner, Some(env.wallet.default_signer().address()));
    let sender = TransactionSender::new(
        env.db_pool.clone(),
        *input.address(),
        *ciphertext.address(),
        env.signer.clone(),
        provider,
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let proof_id = i64::from(rand::random::<u32>());
    sqlx::query(
        "INSERT INTO verify_proofs
         (zk_proof_id, chain_id, contract_address, user_address, handles, verified, retry_count)
         VALUES ($1, 42, $2, $3, $4, $5, 14)",
    )
    .bind(proof_id)
    .bind(env.contract_address.to_string())
    .bind(env.user_address.to_string())
    .bind(vec![1u8; 64])
    .bind(verified)
    .execute(&env.db_pool)
    .await?;

    sqlx::raw_sql(
        "CREATE TABLE campaign_proof_audit (old_row jsonb, new_row jsonb);
        CREATE FUNCTION audit_campaign_proof() RETURNS trigger LANGUAGE plpgsql AS $$
        BEGIN INSERT INTO campaign_proof_audit VALUES (to_jsonb(OLD), to_jsonb(NEW));
        RETURN NULL; END $$;
        CREATE TRIGGER campaign_proof_audit AFTER UPDATE OR DELETE ON verify_proofs
        FOR EACH ROW EXECUTE FUNCTION audit_campaign_proof();",
    )
    .execute(&env.db_pool)
    .await?;
    proxy.set_fault(method, fault, None);
    let initial_calls = proxy.calls(method);
    let started = tokio::time::Instant::now();
    let run = tokio::spawn(async move { sender.run().await });
    let deadline = started + Duration::from_secs(outage_seconds + 180);
    loop {
        let retries: Option<i32> =
            sqlx::query_scalar("SELECT retry_count FROM verify_proofs WHERE zk_proof_id = $1")
                .bind(proof_id)
                .fetch_optional(&env.db_pool)
                .await?;
        assert_eq!(
            retries,
            Some(14),
            "transient error must not exhaust or delete proof"
        );
        assert!(!run.is_finished());
        if proxy.calls(method) - initial_calls >= 4 && started.elapsed().as_secs() >= outage_seconds
        {
            break;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "fault was not exercised repeatedly"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    // Retain the original minimum backoff bound; per-proof cooldown may
    // increase it. An immediate retry loop must fail this check.
    assert!(started.elapsed() >= Duration::from_secs(7));
    let audit_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM campaign_proof_audit
         WHERE (old_row - 'last_retry_at') IS DISTINCT FROM (new_row - 'last_retry_at')",
    )
    .fetch_one(&env.db_pool)
    .await?;
    assert_eq!(
        audit_count, 0,
        "infrastructure failures changed more than retry scheduling"
    );
    assert!(
        (proxy.calls(method) - initial_calls) as u64 <= 5 + started.elapsed().as_secs() / 3,
        "attempt rate exceeded bounded operation backoff"
    );

    // An already-started estimate may use the remaining 30-second HTTP
    // deadline after restoration; allow that plus backoff and drain time.
    let recovery_secs = if method == "eth_estimateGas" { 50 } else { 20 };
    proxy.clear_all_faults();
    tokio::time::timeout(Duration::from_secs(recovery_secs), async {
        loop {
            let present: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM verify_proofs WHERE zk_proof_id = $1)",
            )
            .bind(proof_id)
            .fetch_one(&env.db_pool)
            .await?;
            if !present {
                break;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        anyhow::Ok(())
    })
    .await??;

    // Deletion alone is not success: require the matching on-chain event.
    if verified {
        let events = input
            .VerifyProofResponse_filter()
            .from_block(0)
            .query()
            .await?;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0.zkProofId, U256::from(proof_id as u64));
        assert_eq!(
            events[0].0.ctHandles,
            vec![alloy::primitives::FixedBytes([1u8; 32]); 2]
        );
    } else {
        let events = input
            .RejectProofResponse_filter()
            .from_block(0)
            .query()
            .await?;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0.zkProofId, U256::from(proof_id as u64));
    }
    env.cancel_token.cancel();
    tokio::time::timeout(Duration::from_secs(10), run).await???;
    Ok(())
}
