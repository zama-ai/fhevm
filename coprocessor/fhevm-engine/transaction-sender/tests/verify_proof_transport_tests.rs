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
#[case::estimate_503("eth_estimateGas", Fault::HttpError(503), true, true)]
#[case::nonce_429("eth_getTransactionCount", Fault::HttpError(429), false, true)]
#[case::send_deadline("eth_sendRawTransactionSync", Fault::RpcError {
    code: -32000, message: "context deadline exceeded".into()
}, true, false)]
#[case::send_timeout(
    "eth_sendRawTransactionSync",
    Fault::DiscardAfter(Duration::from_secs(2)),
    true,
    true
)]
#[tokio::test]
#[serial(db)]
async fn transient_failure_preserves_near_exhausted_proof_and_recovers(
    #[case] method: &str,
    #[case] fault: Fault,
    #[case] remove: bool,
    #[case] verified: bool,
) -> anyhow::Result<()> {
    let conf = ConfigSettings {
        verify_proof_resp_max_retries: 3,
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
         VALUES ($1, 42, $2, $3, $4, $5, 2)",
    )
    .bind(proof_id)
    .bind(env.contract_address.to_string())
    .bind(env.user_address.to_string())
    .bind(vec![1u8; 64])
    .bind(verified)
    .execute(&env.db_pool)
    .await?;

    proxy.set_fault(method, fault, None);
    let initial_calls = proxy.calls(method);
    let started = tokio::time::Instant::now();
    let run = tokio::spawn(async move { sender.run().await });
    let deadline = started + Duration::from_secs(30);
    loop {
        let retries: Option<i32> =
            sqlx::query_scalar("SELECT retry_count FROM verify_proofs WHERE zk_proof_id = $1")
                .bind(proof_id)
                .fetch_optional(&env.db_pool)
                .await?;
        assert_eq!(
            retries,
            Some(2),
            "transient error must not exhaust or delete proof"
        );
        assert!(!run.is_finished());
        if proxy.calls(method) - initial_calls >= 4 {
            break;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "fault was not exercised repeatedly"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    // Four attempts require 1 + 2 + 4 seconds of backoff, even for
    // immediate failures. A success/continue busy loop must fail this check.
    assert!(started.elapsed() >= Duration::from_secs(7));
    // Wait for the fourth local timeout too, then observe the row again.
    tokio::time::sleep(Duration::from_millis(1200)).await;
    assert_eq!(
        proxy.calls(method) - initial_calls,
        4,
        "retry rate exceeded backoff"
    );
    let retries: Option<i32> =
        sqlx::query_scalar("SELECT retry_count FROM verify_proofs WHERE zk_proof_id = $1")
            .bind(proof_id)
            .fetch_optional(&env.db_pool)
            .await?;
    assert_eq!(retries, Some(2));

    proxy.clear_all_faults();
    tokio::time::timeout(Duration::from_secs(20), async {
        loop {
            let present: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM verify_proofs WHERE zk_proof_id = $1
                   AND (verified = TRUE OR (verified = FALSE AND handles IS NOT NULL)))",
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

    // Queue removal alone is not success: require the matching on-chain event.
    // Main retains successful rejections with handles cleared until finalization.
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
