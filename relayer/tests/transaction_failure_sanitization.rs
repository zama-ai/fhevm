#![cfg(feature = "integration-tests")]

use alloy::primitives::{Address, Bytes};
use ethereum_rpc_mock::{MockConfig, MockServer, Response, UsageLimit};
use fhevm_relayer::{
    config::settings::Settings,
    core::{errors::EventProcessingError, job_id::JobId},
    gateway::arbitrum::transaction::{
        helper::{GatewayTransactionEngine, ReceiptRecordOutcome, TransactionType},
        TransactionHelper, TxClaimOutcome, TxLifecycleHooks, TxResult,
    },
    metrics,
    orchestrator::DispatchGate,
};
use prometheus::Registry;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default)]
struct RecordingHook {
    failure_reason: Mutex<Option<String>>,
}

#[async_trait::async_trait]
impl TxLifecycleHooks for RecordingHook {
    async fn on_tx_in_flight(
        &self,
        _job_id: &JobId,
    ) -> Result<TxClaimOutcome, EventProcessingError> {
        Ok(TxClaimOutcome::Claimed)
    }

    async fn on_receipt_received(
        &self,
        _job_id: &JobId,
        _receipt: &TxResult,
    ) -> Result<ReceiptRecordOutcome, EventProcessingError> {
        panic!("a rejected transaction must not produce a receipt")
    }

    async fn on_failure(
        &self,
        _job_id: &JobId,
        err_reason: &str,
    ) -> Result<(), EventProcessingError> {
        *self.failure_reason.lock().await = Some(err_reason.to_owned());
        Ok(())
    }
}

const NUL_REVERT: &str = "execution reverted\0binary detail";

/// Sends one transaction to a mock RPC that `reject`s it, and returns the reason the
/// failure hook was given.
async fn failure_reason_for(reject: impl FnOnce(&MockServer, Address)) -> String {
    let target = Address::repeat_byte(0x42);
    let mock = MockServer::new(MockConfig {
        port: 0,
        chain_id: 654_321,
        ..MockConfig::new()
    });
    mock.set_code(target, Bytes::from_static(&[0x60, 0x00]));
    reject(&mock, target);
    let server = mock.start().await.expect("start mock RPC server");

    let config_path = format!(
        "{}/tests/relayer-test-config.yaml",
        env!("CARGO_MANIFEST_DIR")
    );
    let mut settings = Settings::new(Some(config_path)).expect("load test settings");
    settings.gateway.blockchain_rpc.http_url = server.url().to_owned();
    settings.gateway.blockchain_rpc.read_http_url = server.url().to_owned();
    settings.gateway.blockchain_rpc.chain_id = 654_321;
    settings.gateway.tx_engine.retry.max_attempts = 1;
    settings.gateway.tx_engine.retry.retry_interval_ms = 0;
    metrics::init_transaction_metrics(&Registry::new(), settings.metrics.clone());

    let engine = GatewayTransactionEngine::new(
        settings.gateway.blockchain_rpc.clone(),
        settings.gateway.tx_engine.clone(),
        DispatchGate::open_for_tests(1),
    )
    .await
    .expect("build transaction engine");
    let helper = TransactionHelper::new(settings.gateway, Arc::new(engine));
    let hook = RecordingHook::default();

    let result = helper
        .send_raw_transaction_sync(
            TransactionType::PublicDecryptRequest,
            JobId::from([7; 32]),
            &hook,
            target,
            Bytes::from_static(&[0xde, 0xad, 0xbe, 0xef]),
        )
        .await;

    assert!(
        result.is_err(),
        "the RPC rejection must fail the transaction"
    );
    server.shutdown().await.expect("stop mock RPC server");
    let reason = hook.failure_reason.lock().await.clone();
    reason.expect("failure hook was called")
}

fn assert_nul_escaped(reason: &str) {
    assert!(
        reason.contains("execution reverted\\0binary detail"),
        "{reason}"
    );
    assert!(
        !reason.contains('\0'),
        "failure reason still contains a NUL byte"
    );
}

#[tokio::test]
async fn transaction_rpc_nul_is_sanitized_before_failure_hook() {
    let reason = failure_reason_for(|mock, target| {
        mock.on_transaction(
            move |params| params.to == Some(target),
            Response::Error(NUL_REVERT.to_owned()),
            UsageLimit::Once,
        );
    })
    .await;
    assert_nul_escaped(&reason);
}

#[tokio::test]
async fn gas_estimation_rpc_nul_is_sanitized_before_failure_hook() {
    let reason = failure_reason_for(|mock, target| {
        mock.on_estimate_gas(
            move |params| params.to == target,
            Response::Error(NUL_REVERT.to_owned()),
            UsageLimit::Once,
        );
    })
    .await;
    assert_nul_escaped(&reason);
}
