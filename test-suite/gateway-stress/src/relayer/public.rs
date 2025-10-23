use crate::{bench::BurstResult, config::Config};
use alloy::{primitives::FixedBytes, transports::http::reqwest};
use indicatif::ProgressBar;
use std::time::Instant;
use tokio::task::JoinSet;
use tracing::{Instrument, debug, error, trace};

/// Sends a burst of PublicDecryptionRequest to the Relayer.
#[tracing::instrument(skip(config, relayer_client, progress_bar))]
pub async fn public_decryption_burst(
    burst_index: usize,
    config: Config,
    relayer_client: reqwest::Client,
    progress_bar: ProgressBar,
) -> anyhow::Result<BurstResult> {
    debug!("Start of the burst...");
    let burst_start = Instant::now();

    let mut requests_tasks = JoinSet::new();
    for index in 0..config.parallel_requests {
        requests_tasks.spawn(
            send_public_decryption(
                index,
                relayer_client.clone(),
                config.relayer.as_ref().unwrap().url.clone(),
                config.public_ct.iter().map(|ct| ct.handle).collect(),
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

/// Sends a PublicDecryptionRequest transaction to the Relayer.
#[tracing::instrument(skip(relayer_client, relayer_url, handles))]
async fn send_public_decryption(
    index: u32,
    relayer_client: reqwest::Client,
    relayer_url: String,
    handles: Vec<FixedBytes<32>>,
) {
    let body = serde_json::json!({
        "ciphertextHandles": handles,
        "extraData": "0x00",
    });
    let url = format!("{relayer_url}/v1/public-decrypt");
    match relayer_client.post(url).json(&body).send().await {
        Ok(response) => {
            let response_status = response.status();
            let response_str = format!("{response:?}");
            let body = response.text().await;
            if response_status.is_success() {
                debug!("User decryption response received!");
                trace!("Response: {response_str}, {body:?}");
            } else {
                error!("Unexpected response: {response_str}, {body:?}");
            }
        }
        Err(e) => error!("{e}"),
    }
}
