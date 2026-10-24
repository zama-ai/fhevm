//! Private E2E control: hold ordinary delivery of selected blocks before ingestion.
//! Compiled only for unit tests and explicitly enabled fault-test images.
use serde::Deserialize;
use serde_json::Value;
use std::{
    io,
    io::Write,
    path::Path,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio_util::sync::CancellationToken;

const CONTROL: &str = "/tmp/fhevm-test-catchup";
static RECEIPT_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[derive(Deserialize)]
struct Control {
    block_hashes: Vec<String>,
    until: u64,
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn selected(path: &Path, payload: &Value) -> io::Result<bool> {
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(false)
        }
        Err(error) => return Err(error),
    };
    let control: Control =
        serde_json::from_slice(&bytes).map_err(io::Error::other)?;
    if control.until <= now() || control.until > now().saturating_add(2400) {
        return Err(io::Error::other(
            "catchup isolation control expired or invalid",
        ));
    }
    Ok(payload["block_hash"].as_str().is_some_and(|hash| {
        control.block_hashes.iter().any(|expected| expected == hash)
    }))
}

pub async fn before_ingest(
    payload: &Value,
    cancel: &CancellationToken,
) -> io::Result<()> {
    hold_at(Path::new(CONTROL), payload, cancel).await
}

async fn hold_at(
    path: &Path,
    payload: &Value,
    cancel: &CancellationToken,
) -> io::Result<()> {
    if !matches!(payload["flow"].as_str(), Some("LIVE" | "FINAL" | "REORGED")) {
        return Ok(());
    }
    while selected(path, payload)? {
        tokio::select! {
            _ = cancel.cancelled() => return Err(io::Error::new(io::ErrorKind::Interrupted, "catchup test cancelled")),
            _ = tokio::time::sleep(Duration::from_millis(100)) => {}
        }
    }
    Ok(())
}

pub fn after_ingest(payload: &Value) -> io::Result<()> {
    record_at(Path::new(CONTROL), payload)
}

fn record_at(path: &Path, payload: &Value) -> io::Result<()> {
    if payload["flow"] != "CATCHUP" || !selected(path, payload)? {
        return Ok(());
    }
    // One append holds the complete successful payload, including its request
    // ID and original transaction/log data. Retries may produce duplicates.
    let mut bytes = serde_json::to_vec(payload).map_err(io::Error::other)?;
    bytes.push(b'\n');
    let _lock = RECEIPT_LOCK
        .lock()
        .map_err(|_| io::Error::other("catchup receipt writer panicked"))?;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path.with_extension("receipts"))?;
    file.write_all(&bytes)?;
    file.sync_all()
}

pub fn execute(action: &str) -> io::Result<String> {
    let path = Path::new(CONTROL);
    match action {
        "empty" => {
            if path.try_exists()?
                || path.with_extension("receipts").try_exists()?
            {
                return Err(io::Error::other(
                    "prior catchup control needs recovery",
                ));
            }
            Ok("empty".to_owned())
        }
        "receipts" => std::fs::read_to_string(path.with_extension("receipts")),
        "clear" => {
            for file in [path.to_owned(), path.with_extension("receipts")] {
                match std::fs::remove_file(file) {
                    Err(error) if error.kind() != io::ErrorKind::NotFound => {
                        return Err(error)
                    }
                    _ => {}
                }
            }
            Ok("cleared".to_owned())
        }
        _ => Err(io::Error::other("unknown private catchup control action")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn isolation_holds_live_final_and_reorg_but_allows_and_records_catchup(
    ) {
        let root = std::env::temp_dir()
            .join(format!("catchup-isolation-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let path = root.join("control");
        std::fs::write(
            &path,
            json!({"block_hashes":["selected"],"until":now()+60}).to_string(),
        )
        .unwrap();
        let cancel = CancellationToken::new();
        for flow in ["LIVE", "FINAL", "REORGED"] {
            let selected = json!({"flow":flow,"block_hash":"selected"});
            assert!(tokio::time::timeout(
                Duration::from_millis(20),
                hold_at(&path, &selected, &cancel)
            )
            .await
            .is_err());
            hold_at(
                &path,
                &json!({"flow":flow,"block_hash":"another"}),
                &cancel,
            )
            .await
            .unwrap();
        }
        let replay = json!({"flow":"CATCHUP","block_hash":"selected","catchup_id":"new","transactions":[{"hash":"original"}]});
        hold_at(&path, &replay, &cancel).await.unwrap();
        record_at(&path, &replay).unwrap();
        record_at(&path, &json!({"flow":"LIVE","block_hash":"selected"}))
            .unwrap();
        let receipt: Value = serde_json::from_str(
            &std::fs::read_to_string(path.with_extension("receipts")).unwrap(),
        )
        .unwrap();
        assert_eq!(receipt, replay);
        cancel.cancel();
        assert_eq!(
            hold_at(
                &path,
                &json!({"flow":"FINAL","block_hash":"selected"}),
                &cancel
            )
            .await
            .unwrap_err()
            .kind(),
            io::ErrorKind::Interrupted
        );
        std::fs::write(
            &path,
            json!({"block_hashes":["selected"],"until":now()-1}).to_string(),
        )
        .unwrap();
        assert!(record_at(&path, &replay).is_err());
        assert!(hold_at(
            &path,
            &json!({"flow":"LIVE","block_hash":"selected"}),
            &CancellationToken::new()
        )
        .await
        .is_err());
        std::fs::remove_file(&path).unwrap();
        hold_at(
            &path,
            &json!({"flow":"LIVE","block_hash":"selected"}),
            &CancellationToken::new(),
        )
        .await
        .unwrap();
        std::fs::remove_dir_all(root).unwrap();
    }
}
