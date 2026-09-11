mod common;
mod support;

use alloy::providers::ProviderBuilder;
use common::{CiphertextCommits, InputVerification, SignerType, TestEnvironment};
use std::{
    io::Write,
    sync::{Arc, Mutex},
    time::Duration,
};
use support::{Fault, FaultProxy};
use test_harness::db_utils::{insert_ciphertext_digest, insert_random_keys_and_host_chain};
use tracing_subscriber::prelude::*;
use transaction_sender::{
    diagnostics::gateway_tracing_filter, gateway_http_client, get_chain_id, ConfigSettings,
    FillersWithoutNonceManagement, NonceManagedProvider, TransactionSender,
};

#[derive(Clone, Default)]
struct Logs(Arc<Mutex<Vec<u8>>>);
impl Write for Logs {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for Logs {
    type Writer = Self;
    fn make_writer(&'a self) -> Self {
        self.clone()
    }
}

/// Capture TRACE-level logs while exercising actual providers and DB writes.
/// A Gateway can echo a bare token as well as its full URL, including in a
/// malformed response; both must be absent from every diagnostic sink.
#[tokio::test]
async fn gateway_credentials_stay_out_of_logs_health_and_database() -> anyhow::Result<()> {
    let logs = Logs::default();
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::filter_fn(
            gateway_tracing_filter,
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_writer(logs.clone()),
        )
        .try_init()?;
    let conf = ConfigSettings {
        verify_proof_resp_max_retries: 50,
        verify_proof_remove_after_max_retries: false,
        add_ciphertexts_max_retries: 50,
        error_sleep_initial_secs: 1,
        error_sleep_max_secs: 1,
        graceful_shutdown_timeout: Duration::from_secs(5),
        ..Default::default()
    };
    let env = TestEnvironment::new_with_config(SignerType::PrivateKey, conf, false).await?;
    let deploy = env.http_provider()?;
    let input = InputVerification::deploy(&deploy, false, false, false, false).await?;
    let commits = CiphertextCommits::deploy(&deploy, false).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let mut url = proxy.url();
    url.set_username("GW_USER_SECRET").unwrap();
    url.set_password(Some("GW_PASSWORD_SECRET")).unwrap();
    url.set_path("/GW_PATH_SECRET");
    url.set_query(Some("token=GW_QUERY_SECRET"));
    let inner = ProviderBuilder::default()
        .filler(FillersWithoutNonceManagement::default())
        .wallet(env.wallet.clone())
        .connect_reqwest(gateway_http_client(&url)?, url.clone());
    let provider = NonceManagedProvider::new(inner, Some(env.wallet.default_signer().address()));
    let sender = TransactionSender::new(
        env.db_pool.clone(),
        *input.address(),
        *commits.address(),
        env.signer.clone(),
        provider,
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;
    let (host_chain_id, key_id) = insert_random_keys_and_host_chain(&env.db_pool).await?;
    insert_ciphertext_digest(
        &env.db_pool,
        host_chain_id,
        key_id,
        &[1u8; 32],
        &[2u8; 32],
        &[3u8; 32],
        1,
    )
    .await?;
    sqlx::query(
        "INSERT INTO verify_proofs
        (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
        VALUES (123, 42, $1, $2, $3, true)",
    )
    .bind(env.contract_address.to_string())
    .bind(env.user_address.to_string())
    .bind(vec![1u8; 64])
    .execute(&env.db_pool)
    .await?;

    let echoed = format!("upstream URL {url}, bare token GW_BARE_SECRET");
    proxy.set_fault(
        "eth_estimateGas",
        Fault::RawResponse {
            status: 400,
            body: echoed.clone(),
        },
        None,
    );
    let run_sender = sender.clone();
    let run = tokio::spawn(async move { run_sender.run().await });
    let cases = [
        (400, echoed.clone(), "HTTP error 400", "HTTP error 400"),
        // Proof budget is preserved, but add-ciphertext's existing unlimited
        // bucket still writes an error. Cover that DB sink too.
        (429, echoed.clone(), "HTTP error 400", "HTTP error 429"),
        (
            200,
            serde_json::json!({"jsonrpc":"2.0", "id":0,
            "error":{"code":-32000,"message":echoed,"data":"GW_BARE_SECRET"}})
            .to_string(),
            "JSON-RPC error -32000",
            "JSON-RPC error -32000",
        ),
        (
            200,
            format!("<html>{echoed}</html>"),
            "invalid JSON response",
            "invalid JSON response",
        ),
    ];
    for (status, body, proof_expected, ciphertext_expected) in cases {
        // JSON-RPC responses must echo the actual request ID (see proxy).
        proxy.set_fault("eth_estimateGas", Fault::RawResponse { status, body }, None);
        tokio::time::timeout(Duration::from_secs(15), async {
            loop {
                let proof: Option<String> = sqlx::query_scalar(
                    "SELECT last_error FROM verify_proofs WHERE zk_proof_id = 123",
                )
                .fetch_one(&env.db_pool)
                .await?;
                let ciphertext: Option<String> =
                    sqlx::query_scalar("SELECT txn_last_error FROM ciphertext_digest LIMIT 1")
                        .fetch_one(&env.db_pool)
                        .await?;
                for value in [&proof, &ciphertext].into_iter().flatten() {
                    assert!(!value.contains("SECRET"), "credential leaked into DB error");
                    assert!(
                        !value.contains(url.as_str()),
                        "Gateway URL leaked into DB error"
                    );
                }
                if proof.as_deref().is_some_and(|v| v.contains(proof_expected))
                    && ciphertext
                        .as_deref()
                        .is_some_and(|v| v.contains(ciphertext_expected))
                {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
            anyhow::Ok(())
        })
        .await??;
    }

    for method in ["eth_blockNumber", "eth_chainId"] {
        proxy.set_fault(
            method,
            Fault::RawResponse {
                status: 400,
                body: echoed.clone(),
            },
            None,
        );
    }
    let health = sender.health_check().await;
    assert!(!health.healthy);
    assert!(!health.details.unwrap().contains("SECRET"));
    let calls = proxy.calls("eth_chainId");
    assert!(tokio::time::timeout(
        Duration::from_millis(300),
        get_chain_id(url, Duration::from_secs(1))
    )
    .await
    .is_err());
    assert!(
        proxy.calls("eth_chainId") > calls,
        "startup probe must encounter an error"
    );
    env.cancel_token.cancel();
    tokio::time::timeout(Duration::from_secs(10), run).await???;
    // TLS handshake diagnostics can expose endpoint names via ClientHello.
    tracing::trace!(target: "rustls::client::hs", "GW_TLS_SECRET");
    tracing::info!("gateway diagnostics regression completed");
    let output = String::from_utf8(logs.0.lock().unwrap().clone())?;
    assert!(
        output.contains("gateway diagnostics regression completed"),
        "log capture must be active"
    );
    assert!(
        !output.contains("SECRET"),
        "credential leaked into logs or span fields"
    );
    Ok(())
}
