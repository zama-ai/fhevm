use crate::{
    bench::BurstResult,
    blockchain::user::{DURATION_DAYS, RAND_PUBLIC_KEY, generate_eip712},
    config::Config,
};
use alloy::{hex, transports::http::reqwest};
use anyhow::anyhow;
use gateway_sdk::FhevmSdk;
use indicatif::ProgressBar;
use serde_json::Value;
use std::{
    sync::Arc,
    time::{Instant, SystemTime},
};
use tokio::task::JoinSet;
use tracing::{Instrument, debug, error, trace};

/// Sends a burst of UserDecryptionRequest to the Relayer.
#[tracing::instrument(skip(config, relayer_client, sdk, progress_bar))]
pub async fn user_decryption_burst(
    burst_index: usize,
    config: Config,
    relayer_client: reqwest::Client,
    sdk: Arc<FhevmSdk>,
    progress_bar: ProgressBar,
) -> anyhow::Result<BurstResult> {
    debug!("Start of the burst...");
    let burst_start = Instant::now();

    let mut requests_tasks = JoinSet::new();
    for index in 0..config.parallel_requests {
        requests_tasks.spawn(
            send_user_decryption(
                index,
                relayer_client.clone(),
                config.clone(),
                Arc::clone(&sdk),
            )
            .in_current_span(),
        );
    }
    debug!("All requests of the burst have been sent! Waiting for responses...");

    for _ in 0..config.parallel_requests {
        requests_tasks.join_next().await;
        progress_bar.inc(1);
    }
    debug!("Successfully received all responses of the burst!");

    let latency = burst_start.elapsed().as_secs_f64();
    let res = BurstResult {
        latency,
        throughput: config.parallel_requests as f64 / latency,
    };
    progress_bar.finish_with_message(format!(
        "Handled burst #{} of {} in {:.2}s. Throughput: {:.2} tps",
        burst_index, config.parallel_requests, res.latency, res.throughput
    ));

    Ok(res)
}

/// Sends a UserDecryptionRequest transaction to the Relayer.
#[tracing::instrument(skip(relayer_client, config, sdk))]
async fn send_user_decryption(
    index: u32,
    relayer_client: reqwest::Client,
    config: Config,
    sdk: Arc<FhevmSdk>,
) {
    if let Err(e) = send_user_decryption_inner(relayer_client, config, sdk).await {
        error!("{e}");
    }
}

async fn send_user_decryption_inner(
    relayer_client: reqwest::Client,
    config: Config,
    sdk: Arc<FhevmSdk>,
) -> anyhow::Result<()> {
    let relayer_config = config
        .relayer
        .expect("Missing [relayer] section in config file"); // Should be unreachable
    let blockchain_config = config
        .blockchain
        .as_ref()
        .expect("Missing [blockchain] section in config file"); // Should be unreachable

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let eip712 = generate_eip712(
        sdk,
        config.allowed_contract,
        blockchain_config.private_key.clone(),
        timestamp,
    )?;

    let handle_contract_pairs: Vec<Value> = config
        .user_ct
        .iter()
        .map(|ct| {
            serde_json::json!({
                "handle": ct.handle,
                "contractAddress": config.allowed_contract,
            })
        })
        .collect();
    let body = serde_json::json!({
        "handleContractPairs": handle_contract_pairs,
        "requestValidity": {
            "startTimestamp": timestamp.to_string(),
            "durationDays": DURATION_DAYS.to_string(),
        },
        "contractsChainId": blockchain_config.host_chain_id.to_string(),
        "contractAddresses": vec![config.allowed_contract],
        "userAddress": relayer_config.user_address,
        "signature": hex::encode(eip712.signature.unwrap()),
        "publicKey": RAND_PUBLIC_KEY,
        "extraData": "0x00",
    });
    debug!("{}", body.clone().to_string());
    let url = format!("{}/v1/user-decrypt", relayer_config.url);

    let response = relayer_client.post(url).json(&body).send().await?;
    let response_status = response.status();
    let response_str = format!("{response:?}");
    let body = response.text().await;

    if response_status.is_success() {
        debug!("User decryption response received!");
        trace!("Response: {response_str}, {body:?}");
        Ok(())
    } else {
        Err(anyhow!("Unexpected response: {response_str}, {body:?}"))
    }
}
