//! Test-only interruption after a real successful handler and before Redis XACK.
use serde::{Deserialize, Serialize};
use std::{
    io::Write,
    path::Path,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Deserialize)]
struct Control {
    block_hash: String,
    until: u64,
}
#[derive(Serialize, Deserialize)]
struct Receipt {
    queue: String,
    message_id: String,
    delivery_count: u64,
    payload: serde_json::Value,
    redelivered: bool,
}

pub(crate) async fn before_ack(payload: &[u8], queue: &str, message_id: &str, delivery_count: u64) {
    at(
        Path::new("/tmp/fhevm-test-broker-ack"),
        payload,
        queue,
        message_id,
        delivery_count,
    )
    .await;
}
async fn at(path: &Path, payload: &[u8], queue: &str, message_id: &str, delivery_count: u64) {
    let Ok(control) = std::fs::read(path)
        .and_then(|bytes| serde_json::from_slice::<Control>(&bytes).map_err(std::io::Error::other))
    else {
        return;
    };
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    if control.until <= now || control.until > now.saturating_add(600) {
        return;
    }
    let Ok(payload) = serde_json::from_slice::<serde_json::Value>(payload) else {
        return;
    };
    if payload.get("block_hash").and_then(|value| value.as_str())
        != Some(control.block_hash.as_str())
        || payload.get("flow").and_then(|value| value.as_str()) != Some("LIVE")
    {
        return;
    }
    let redelivered = delivery_count > 1;
    let receipt = Receipt {
        queue: queue.to_owned(),
        message_id: message_id.to_owned(),
        delivery_count,
        payload,
        redelivered,
    };
    let observed = path.with_extension("observed");
    match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&observed)
    {
        Ok(mut file) => {
            if file
                .write_all(&serde_json::to_vec(&receipt).unwrap())
                .and_then(|_| file.sync_all())
                .is_err()
            {
                return;
            }
            while path.exists()
                && SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    < control.until
            {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
        Err(_) => {
            let Ok(first) = std::fs::read(&observed).and_then(|bytes| {
                serde_json::from_slice::<Receipt>(&bytes).map_err(std::io::Error::other)
            }) else {
                return;
            };
            // The runner also queries XPENDING independently: production delivery
            // metadata has a fallback and cannot alone establish redelivery.
            if !first.redelivered
                && redelivered
                && first.queue == receipt.queue
                && first.message_id == receipt.message_id
                && first.payload == receipt.payload
            {
                let _ = std::fs::write(
                    path.with_extension("redelivered"),
                    serde_json::to_vec(&receipt).unwrap(),
                );
                while path.exists()
                    && SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs()
                        < control.until
                {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn only_identical_actual_redelivery_can_complete_the_control() {
        let root = std::env::temp_dir().join(format!("broker-ack-control-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let path = root.join("control");
        let hash = format!("0x{}", "ab".repeat(32));
        let payload = serde_json::to_vec(
            &serde_json::json!({"flow":"LIVE","block_hash":hash,"block_number":17}),
        )
        .unwrap();
        let until = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 20;
        std::fs::write(
            &path,
            serde_json::json!({"block_hash":hash,"until":until}).to_string(),
        )
        .unwrap();
        let held = tokio::spawn({
            let path = path.clone();
            let payload = payload.clone();
            async move {
                at(&path, &payload, "selected", "100-0", 1).await;
            }
        });
        tokio::time::timeout(Duration::from_secs(2), async {
            while !path.with_extension("observed").exists() {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .unwrap();
        assert!(!held.is_finished());
        held.abort(); // Models the missing ACK; the durable receipt survives.
        let _ = held.await;
        at(&path, &payload, "selected", "100-0", 1).await;
        at(&path, &payload, "another-queue", "100-0", 2).await;
        at(&path, b"{\"flow\":\"FINAL\"}", "selected", "100-0", 2).await;
        assert!(!path.with_extension("redelivered").exists());
        let replay = tokio::spawn({
            let path = path.clone();
            let payload = payload.clone();
            async move {
                at(&path, &payload, "selected", "100-0", 2).await;
            }
        });
        tokio::time::timeout(Duration::from_secs(2), async {
            while !path.with_extension("redelivered").exists() {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .unwrap();
        std::fs::remove_file(&path).unwrap();
        replay.await.unwrap();
        std::fs::remove_dir_all(root).unwrap();
    }
}
