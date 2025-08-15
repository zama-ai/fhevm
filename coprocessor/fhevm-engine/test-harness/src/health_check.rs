use std::time::Duration;

use tracing::{info, warn};

pub async fn wait_url_success(url: &str, retry: u64, delay: u64) -> bool {
    for step in 1..=retry {
        let response = reqwest::get(url);
        let response_or_timeout = tokio::time::timeout(Duration::from_secs(7), response);
        let Ok(response) = response_or_timeout.await else {
            warn!("Listener timeout");
            continue;
        };
        if response.is_ok() && response.as_ref().unwrap().status().is_success() {
            info!("Listener ok after {} seconds", step * delay);
            return true;
        } else {
            warn!(
                "Listener not ready yet, retry {}/{} in {} seconds, {:?}",
                step, retry, delay, response
            );
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
    }
    false
}

pub async fn wait_alive(url: &str, retry: u64, delay: u64) -> bool {
    let alive_url = format!("{}/liveness", url);
    wait_url_success(&alive_url, retry, delay).await
}

pub async fn wait_healthy(url: &str, retry: u64, delay: u64) -> bool {
    let healthz_url = format!("{}/healthz", url);
    wait_url_success(&healthz_url, retry, delay).await
}
