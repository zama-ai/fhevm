//! In-process scripted connector for tests: each node answers per attempt, after a delay, under paused tokio time.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use alloy::primitives::{B256, Bytes as AlloyBytes, b256};
use async_trait::async_trait;
use bytes::Bytes;
use kms_connector_api::{
    ErrorCode, ErrorResponse, PUBLIC_DECRYPTION_ROUTE, PublicDecryptionRequest,
    PublicDecryptionResponse, USER_DECRYPTION_ROUTE, UserDecryptionRequest, UserDecryptionResponse,
};
use serde::Deserialize;
use tokio::time::sleep;
use url::Url;

use super::call::Caller;
use super::client::{AttemptError, ConnectorClient, Endpoint, HttpReply};
use super::config::{
    AuthConfig, CallConfig, EndpointConfig, FlowConfig, KmsAggregatorConfig, RetryConfig,
};

/// From `kms-connector/crates/api/tests/vectors.json`.
pub const USER_REQUEST_JSON: &str = r#"{"handles":[{"handle":"0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","contractAddress":"0x3333333333333333333333333333333333333333","ownerAddress":"0x4444444444444444444444444444444444444444"}],"userAddress":"0x5555555555555555555555555555555555555555","publicKey":"0x20002000","allowedContracts":["0x3333333333333333333333333333333333333333"],"requestValidity":{"startTimestamp":1770000000,"durationSeconds":300},"signature":"0x6666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666","extraData":"0x00"}"#;
pub const USER_REQUEST_ID: B256 =
    b256!("0x0fc9151ca446462aeadbf740c1021d1c089413cb54940664dcb996a031268453");
pub const PUBLIC_REQUEST_JSON: &str = r#"{"ctHandles":["0x1111111111111111111111111111111111111111111111111111111111111111","0x2222222222222222222222222222222222222222222222222222222222222222"],"extraData":"0x00"}"#;
pub const PUBLIC_REQUEST_ID: B256 =
    b256!("0xaaa2956d918458882642374b5ef6fc9117ee708e1930c84510faa0e4afa018b8");

/// A YAML string is either one of our fixed replies or a connector error code (`acl_denied`, `rate_limited`, …).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum Reply {
    Fixed(Fixed),
    Error(ErrorCode),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Fixed {
    /// `200` with a valid response derived from the request (per-node 65-byte signature and share).
    Ok,
    /// Public: a different `decryptedResult`; user: same as `Ok`.
    Divergent,
    /// A 64-byte signature.
    BadSignature,
    /// Never answers (until cancelled).
    Hang,
    /// Connection refused.
    Refused,
}

pub struct MockClient {
    scripts: Vec<Vec<Reply>>,
    delay: Duration,
    attempts: Mutex<Vec<u32>>,
}

impl MockClient {
    /// `scripts[node]` = replies per attempt (the last one repeats). Endpoint names are `node-<i>`.
    pub fn new(scripts: Vec<Vec<Reply>>, delay: Duration) -> Arc<Self> {
        let attempts = Mutex::new(vec![0; scripts.len()]);
        Arc::new(Self {
            scripts,
            delay,
            attempts,
        })
    }

    pub fn attempts(&self, node: usize) -> u32 {
        self.attempts.lock().unwrap()[node]
    }

    /// A validated config for `n` mock nodes (loopback urls, no auth, thresholds 1).
    pub fn config(n: usize, timeout: Duration, max_retries: u32) -> KmsAggregatorConfig {
        KmsAggregatorConfig {
            allow_insecure_http: false,
            max_concurrent_calls: n.max(1),
            call: CallConfig {
                timeout,
                retries: RetryConfig {
                    max_retries,
                    ..RetryConfig::default()
                },
            },
            user_decrypt: FlowConfig { threshold: 1 },
            public_decrypt: FlowConfig { threshold: 1 },
            endpoints: (0..n)
                .map(|i| EndpointConfig {
                    name: format!("node-{i}"),
                    url: Url::parse(&format!("http://localhost:3000/{i}")).unwrap(),
                    auth: AuthConfig::None,
                })
                .collect(),
        }
    }

    /// A `Caller` over this mock, one endpoint per script.
    pub fn caller(self: Arc<Self>, timeout: Duration, max_retries: u32) -> Arc<Caller> {
        let cfg = Self::config(self.scripts.len(), timeout, max_retries);
        Arc::new(Caller::new(&cfg, self).unwrap())
    }
}

#[async_trait]
impl ConnectorClient for MockClient {
    async fn post(
        &self,
        endpoint: &Endpoint,
        route: &str,
        body: Bytes,
    ) -> Result<HttpReply, AttemptError> {
        let node: usize = endpoint
            .name
            .trim_start_matches("node-")
            .parse()
            .expect("mock endpoint name");
        let attempt = {
            let mut attempts = self.attempts.lock().unwrap();
            attempts[node] += 1;
            attempts[node] - 1
        };
        let script = &self.scripts[node];
        let reply = script
            .get(attempt as usize)
            .or(script.last())
            .copied()
            .unwrap_or(Reply::Fixed(Fixed::Refused));
        sleep(self.delay).await;
        match reply {
            Reply::Fixed(Fixed::Hang) => std::future::pending().await,
            Reply::Fixed(Fixed::Refused) => Err(AttemptError::Transport(
                "mock: connection refused".to_owned(),
            )),
            Reply::Error(code) => Ok(error_reply(route, &body, code)),
            Reply::Fixed(fixed) => Ok(ok_reply(route, &body, node, fixed)),
        }
    }
}

/// A 65-byte signature (`r ‖ s ‖ v`, `v = 27`), distinct per node. Nothing verifies it.
pub fn signature(node: usize) -> AlloyBytes {
    let mut bytes = vec![0x50 ^ node as u8; 65];
    bytes[64] = 27;
    AlloyBytes::from(bytes)
}

/// A node's opaque share blob: 96 bytes, distinct per node.
pub fn share(node: usize) -> AlloyBytes {
    AlloyBytes::from(vec![0xA0 ^ node as u8; 96])
}

/// An ABI-encoded public result (two `uint256`): 64 bytes; `variant` changes the first value.
pub fn result(variant: u8) -> AlloyBytes {
    let mut bytes = vec![0u8; 64];
    bytes[31] = 42 + variant;
    bytes[63] = 7;
    AlloyBytes::from(bytes)
}

/// The request's derived id and echoed `extraData`, by route.
fn request_facts(route: &str, body: &[u8]) -> (B256, AlloyBytes) {
    if route == USER_DECRYPTION_ROUTE {
        let request: UserDecryptionRequest = serde_json::from_slice(body).expect("mock: user body");
        (request.id(), request.extraData)
    } else {
        assert_eq!(route, PUBLIC_DECRYPTION_ROUTE, "mock: unknown route");
        let request: PublicDecryptionRequest =
            serde_json::from_slice(body).expect("mock: public body");
        (request.id(), request.extraData)
    }
}

/// A `200` reply for `route`, derived from the request body so the id matches what was sent.
pub fn ok_reply(route: &str, body: &[u8], node: usize, fixed: Fixed) -> HttpReply {
    let (decryption_id, extra_data) = request_facts(route, body);
    let mut signature = signature(node);
    if fixed == Fixed::BadSignature {
        signature = AlloyBytes::copy_from_slice(&signature[..64]);
    }
    let body = if route == USER_DECRYPTION_ROUTE {
        serde_json::to_vec(&UserDecryptionResponse {
            decryption_id,
            user_decrypted_shares: share(node),
            signature,
            extra_data,
        })
    } else {
        serde_json::to_vec(&PublicDecryptionResponse {
            decryption_id,
            decrypted_result: result(u8::from(fixed == Fixed::Divergent)),
            signature,
            extra_data,
        })
    };
    HttpReply {
        status: 200,
        body: Bytes::from(body.expect("mock: response serialises")),
    }
}

/// The connector's error body at `code.http_status()`; `malformed` carries no id, like the real endpoint.
pub fn error_reply(route: &str, body: &[u8], code: ErrorCode) -> HttpReply {
    let id = (code != ErrorCode::Malformed).then(|| request_facts(route, body).0);
    let error = ErrorResponse::new(code, format!("mock {}", code.as_str()), id);
    HttpReply {
        status: code.http_status(),
        body: Bytes::from(serde_json::to_vec(&error).expect("mock: error serialises")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_ids_match_the_api_vectors() {
        let user: UserDecryptionRequest = serde_json::from_str(USER_REQUEST_JSON).unwrap();
        assert_eq!(user.id(), USER_REQUEST_ID);
        let public: PublicDecryptionRequest = serde_json::from_str(PUBLIC_REQUEST_JSON).unwrap();
        assert_eq!(public.id(), PUBLIC_REQUEST_ID);
    }

    #[test]
    fn ok_reply_decodes_to_the_dto_with_the_request_id() {
        let reply = ok_reply(
            PUBLIC_DECRYPTION_ROUTE,
            PUBLIC_REQUEST_JSON.as_bytes(),
            2,
            Fixed::Ok,
        );
        let response: PublicDecryptionResponse = serde_json::from_slice(&reply.body).unwrap();
        assert_eq!(reply.status, 200);
        assert_eq!(response.decryption_id, PUBLIC_REQUEST_ID);
        assert_eq!(response.signature, signature(2));
        assert_eq!(response.decrypted_result, result(0));
        assert_eq!(response.extra_data.as_ref(), &[0u8]);

        let divergent = ok_reply(
            PUBLIC_DECRYPTION_ROUTE,
            PUBLIC_REQUEST_JSON.as_bytes(),
            2,
            Fixed::Divergent,
        );
        let response: PublicDecryptionResponse = serde_json::from_slice(&divergent.body).unwrap();
        assert_eq!(response.decrypted_result, result(1));

        let short = ok_reply(
            USER_DECRYPTION_ROUTE,
            USER_REQUEST_JSON.as_bytes(),
            1,
            Fixed::BadSignature,
        );
        let response: UserDecryptionResponse = serde_json::from_slice(&short.body).unwrap();
        assert_eq!(response.signature.len(), 64);
        assert_eq!(response.user_decrypted_shares, share(1));
    }

    #[test]
    fn error_reply_uses_the_connector_status_and_flag() {
        let reply = error_reply(
            USER_DECRYPTION_ROUTE,
            USER_REQUEST_JSON.as_bytes(),
            ErrorCode::Overloaded,
        );
        assert_eq!(reply.status, 503);
        let error: ErrorResponse = serde_json::from_slice(&reply.body).unwrap();
        assert_eq!(error.code, ErrorCode::Overloaded);
        assert!(error.retryable);
        assert_eq!(error.decryption_id, Some(USER_REQUEST_ID));

        let malformed = error_reply(
            USER_DECRYPTION_ROUTE,
            USER_REQUEST_JSON.as_bytes(),
            ErrorCode::Malformed,
        );
        let error: ErrorResponse = serde_json::from_slice(&malformed.body).unwrap();
        assert_eq!((malformed.status, error.decryption_id), (400, None));
    }

    #[test]
    fn replies_deserialize_from_bare_strings() {
        let replies: Vec<Reply> =
            serde_json::from_str(r#"["ok","hang","acl_denied","rate_limited"]"#).unwrap();
        assert_eq!(
            replies,
            vec![
                Reply::Fixed(Fixed::Ok),
                Reply::Fixed(Fixed::Hang),
                Reply::Error(ErrorCode::AclDenied),
                Reply::Error(ErrorCode::RateLimited),
            ]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn scripts_repeat_their_last_reply_and_count_attempts() {
        let mock = MockClient::new(
            vec![vec![
                Reply::Error(ErrorCode::RateLimited),
                Reply::Fixed(Fixed::Ok),
            ]],
            Duration::from_millis(5),
        );
        let endpoint = Endpoint {
            name: "node-0".into(),
            url: Url::parse("http://localhost:3000/0").unwrap(),
            auth: None,
        };
        let body = Bytes::from_static(USER_REQUEST_JSON.as_bytes());
        let mut seen = Vec::new();
        for _ in 0..3 {
            let reply = mock
                .post(&endpoint, USER_DECRYPTION_ROUTE, body.clone())
                .await
                .unwrap();
            seen.push(reply.status);
        }
        assert_eq!(seen, vec![429, 200, 200]);
        assert_eq!(mock.attempts(0), 3);
    }
}
