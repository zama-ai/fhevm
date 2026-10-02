//! Expiring, key-scoped transport routing for isolated migration tests.
//! This changes only the HTTP endpoint; the real S3 client, digest validation,
//! parser and activation transaction remain in use.
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
struct Route {
    key: String,
    until: u64,
    endpoint: String,
}

pub fn endpoint(key_suffix: &str) -> Option<String> {
    let text = std::fs::read_to_string("/tmp/fhevm-test-key-download").ok()?;
    route(
        &text,
        key_suffix,
        SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs(),
    )
}

fn route(text: &str, key_suffix: &str, now: u64) -> Option<String> {
    let control: Route = serde_json::from_str(text).ok()?;
    if control.key.len() != 64
        || !control.key.bytes().all(|byte| byte.is_ascii_hexdigit())
        || key_suffix
            != format!("{}/{}", super::XOF_KEY_SET_S3_PREFIX, control.key)
        || control.until <= now
        || control.until > now.saturating_add(3600)
    {
        return None;
    }
    let url = url::Url::parse(&control.endpoint).ok()?;
    if url.scheme() != "http"
        || url.host_str().is_none()
        || url.port().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.path() != "/"
    {
        return None;
    }
    Some(control.endpoint)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transport_control_cannot_capture_other_keys_or_outlive_its_budget() {
        let key = "ab".repeat(32);
        let selected = format!("{}/{key}", super::super::XOF_KEY_SET_S3_PREFIX);
        let config = serde_json::json!({"key":key,"until":120,"endpoint":"http://proxy:1234"}).to_string();
        assert_eq!(
            route(&config, &selected, 100).as_deref(),
            Some("http://proxy:1234")
        );
        assert!(route(&config, &selected, 120).is_none());
        assert!(route(&config, "/ServerKey/ab", 100).is_none());
        assert!(route(
            &config,
            &format!("/CompressedXofKeySet/{}", "cd".repeat(32)),
            100
        )
        .is_none());
        for endpoint in [
            "https://proxy:1234",
            "http://proxy",
            "http://proxy:1234/path",
            "http://user@proxy:1234",
        ] {
            let config =
                serde_json::json!({"key":key,"until":120,"endpoint":endpoint})
                    .to_string();
            assert!(route(&config, &selected, 100).is_none());
        }
        let config = serde_json::json!({"key":key,"until":3701,"endpoint":"http://proxy:1234"}).to_string();
        assert!(route(&config, &selected, 100).is_none());
    }
}
