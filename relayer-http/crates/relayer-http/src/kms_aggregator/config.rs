//! Config structs of the aggregator (deserialised by `config.rs`) and their validation.

use std::time::Duration;

use alloy::primitives::Address;
use fhevm_sig_auth::DEFAULT_MAX_AUTH_VALIDITY;
use serde::Deserialize;
use url::{Host, Url};

use crate::metrics::MAX_KMS_NODES;

/// Upper bound of every configured duration: keeps deadline arithmetic trivially safe.
pub const MAX_TIMEOUT: Duration = Duration::from_secs(60);

/// Lower bound of the deadline: below one second no retry list is useful and the deadline becomes noise.
pub const MIN_TIMEOUT: Duration = Duration::from_secs(1);

/// Invalid configuration; the message names the offending field with its dotted path.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{0}")]
pub struct ConfigError(pub String);

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KmsAggregatorConfig {
    /// Allow plain http towards non-loopback hosts. Local/compose only.
    #[serde(default)]
    pub allow_insecure_http: bool,
    /// Accept any TLS certificate from the connectors: no CA, expiry or hostname check. Local/devnet only.
    #[serde(default)]
    pub skip_tls_verification: bool,
    /// HTTP calls in flight at once, all aggregations together (one semaphore per process).
    #[serde(default = "default_max_concurrent_calls")]
    pub max_concurrent_calls: usize,
    #[serde(default)]
    pub call: CallConfig,
    pub user_decrypt: UserDecryptConfig,
    pub public_decrypt: PublicDecryptConfig,
    pub auth: AuthConfig,
    pub endpoints: Vec<EndpointConfig>,
}

/// Sized for ~200 decryptions/s with 13 nodes: λ × Σ(per-node hold time) × 1.5, a hold time being the node's
/// latency (~1 s) or the deadline for a hung node. See `kms_aggregator/docs.md` §9.
fn default_max_concurrent_calls() -> usize {
    4096
}

/// The deadline and the two retry budgets of one node call. Every field has a default: an empty `call:` is valid.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CallConfig {
    /// The deadline: one aggregation, and every call in it, ends by then.
    #[serde(with = "humantime_serde")]
    pub timeout: Duration,
    /// Retryable errors other than a rate limit: an explicit list of waits.
    pub retries: RetryConfig,
    /// `rate_limited` and bare 429: exponential waits from a floor.
    pub rate_limit: RateLimitConfig,
}

impl Default for CallConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
            retries: RetryConfig::default(),
            rate_limit: RateLimitConfig::default(),
        }
    }
}

/// The transient retry budget: the wait before each retry, in order. The list is the budget: `[]` is a single
/// attempt, `waits[k-1]` is the wait before retry `k`. Short waits first catch a jitter, longer ones after reach the
/// deadline (`docs.md` section 6); the deadline (`call.timeout`) stays the real bound, a wait that does not fit before
/// it is skipped.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RetryConfig {
    #[serde(deserialize_with = "durations")]
    pub waits: Vec<Duration>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            waits: [200, 300, 500, 800, 1000]
                .into_iter()
                .map(Duration::from_millis)
                .collect(),
        }
    }
}

/// A list of durations with units (`[200ms, 1s]`; the `humantime_serde` adapter covers one duration only).
/// Entries are trimmed and empty ones dropped, so the one value an environment override can carry
/// (`200ms, 500ms`, `200ms,`, or nothing at all for a single attempt) parses like the YAML list.
fn durations<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Vec<Duration>, D::Error> {
    use serde::de::{Error, IntoDeserializer};
    let entries: Vec<String> = Deserialize::deserialize(deserializer)?;
    entries
        .iter()
        .map(|entry| entry.trim())
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            humantime_serde::deserialize::<Duration, _>(entry.into_deserializer())
                .map_err(|e: serde::de::value::Error| D::Error::custom(format!("{entry:?}: {e}")))
        })
        .collect()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RateLimitConfig {
    /// Retries after a rate limit. 0 = give up at the first 429. Default 1: one window, then the client retries.
    pub max_retries: u32,
    /// Retry `k` waits `max(initial_delay, Retry-After) * 2^(k-1)`. Default 1 s: the connector proxy's window.
    #[serde(with = "humantime_serde")]
    pub initial_delay: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_retries: 1,
            initial_delay: Duration::from_secs(1),
        }
    }
}

/// The retry budget a failure consumes (`AttemptError::retry_class`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryClass {
    /// Every retryable error except a rate limit.
    Transient,
    /// `rate_limited`, or a bare 429.
    RateLimit,
}

impl RetryClass {
    /// The configuration section's name, as logged.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Transient => "transient",
            Self::RateLimit => "rate_limit",
        }
    }
}

impl CallConfig {
    /// The retry budget of `class`: the length of the transient list, the rate-limit count.
    pub fn max_retries(&self, class: RetryClass) -> u32 {
        match class {
            RetryClass::Transient => u32::try_from(self.retries.waits.len()).unwrap_or(u32::MAX),
            RetryClass::RateLimit => self.rate_limit.max_retries,
        }
    }

    /// The wait before retry `k` (1-based) of `class`. Transient: the k-th wait of the list (zero past its end: the
    /// budget is checked first, so that value is never slept). Rate limit: exponential from the floor, saturating;
    /// `retry_after` only raises that floor.
    pub fn delay_for(&self, class: RetryClass, k: u32, retry_after: Option<Duration>) -> Duration {
        match class {
            RetryClass::Transient => {
                let index = usize::try_from(k.saturating_sub(1)).unwrap_or(usize::MAX);
                self.retries.waits.get(index).copied().unwrap_or_default()
            }
            RetryClass::RateLimit => self
                .rate_limit
                .initial_delay
                .max(retry_after.unwrap_or_default())
                .saturating_mul(2u32.saturating_pow(k.saturating_sub(1))),
        }
    }

    /// The first retry of `class` whose waits alone reach the deadline, with its start. Stops at that retry, so a
    /// huge rate-limit `max_retries` is cheap to check.
    fn first_retry_past_deadline(&self, class: RetryClass) -> Option<(u32, Duration)> {
        let mut start = Duration::ZERO;
        for k in 1..=self.max_retries(class) {
            start = start.saturating_add(self.delay_for(class, k, None));
            if start >= self.timeout {
                return Some((k, start));
            }
        }
        None
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserDecryptConfig {
    /// Distinct shares that must be accepted before answering.
    pub threshold: usize,
    #[serde(default)]
    pub checks: UserChecks,
}

/// Optional checks on user-decrypt responses. All off by default.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct UserChecks {
    /// Reject a response whose `decryptionId` is not the request's content hash.
    pub decryption_id_match: bool,
    /// Count and return only the shares that carry the majority `decryptionId`.
    pub decryption_id_majority: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicDecryptConfig {
    /// Identical `(decryptedResult, extraData)` answers that must be accepted before answering.
    pub threshold: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EndpointConfig {
    pub name: String,
    pub url: Url,
    /// The audience this connector accepts (today its KMS tx-sender address).
    pub audience: Address,
}

/// `FhevmSig` signing of connector calls (RFC-038).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthConfig {
    /// EIP-712 domain: the Ethereum chain id.
    pub chain_id: u64,
    /// EIP-712 domain: the `ProtocolConfig` address.
    pub protocol_config: Address,
    /// Signature lifetime: longer than `call.timeout`, at most 5 min.
    #[serde(with = "humantime_serde", default = "default_validity")]
    pub validity: Duration,
    /// The signing key's address; startup fails on a mismatch.
    pub address: Address,
    pub signer: SignerConfig,
}

fn default_validity() -> Duration {
    Duration::from_secs(120)
}

/// The signing key; the config names an env var, never holds a secret.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum SignerConfig {
    /// A hex private key in the env var `value_env` (local dev, CI).
    PrivateKey { value_env: String },
    /// An AWS KMS key (deployments).
    AwsKms {
        /// Key id or ARN; region and credentials come from the AWS environment.
        key_id: String,
    },
}

fn fail(message: String) -> Result<(), ConfigError> {
    Err(ConfigError(message))
}

impl KmsAggregatorConfig {
    /// Every rule names the offending field with its dotted path.
    pub fn validate(&self) -> Result<(), ConfigError> {
        let n = self.endpoints.len();
        if n == 0 {
            return fail("kms_aggregator.endpoints must not be empty".to_owned());
        }
        // The bound of the `kms_node` label: the series budget is computed from it.
        if n > MAX_KMS_NODES {
            return fail(format!(
                "kms_aggregator.endpoints must have at most {MAX_KMS_NODES} entries ({n} given)"
            ));
        }
        for (i, ep) in self.endpoints.iter().enumerate() {
            let at = format!("kms_aggregator.endpoints[{i}]");
            if ep.name.trim().is_empty() {
                return fail(format!("{at}.name must not be empty"));
            }
            let earlier = || self.endpoints.iter().take(i);
            if earlier().any(|o| o.name == ep.name) {
                return fail(format!("{at}.name '{}' must be unique", ep.name));
            }
            if earlier().any(|o| o.url == ep.url) {
                return fail(format!("{at}.url must be unique (same node counted twice)"));
            }
            let secure = ep.url.scheme() == "https";
            if !secure && ep.url.scheme() != "http" {
                return fail(format!("{at}.url must use http or https"));
            }
            if ep.url.host().is_none() {
                return fail(format!("{at}.url must have a host"));
            }
            if !ep.url.username().is_empty() || ep.url.password().is_some() {
                return fail(format!("{at}.url must not carry credentials"));
            }
            if ep.url.query().is_some() || ep.url.fragment().is_some() {
                return fail(format!("{at}.url must not carry a query or a fragment"));
            }
            let local = is_loopback(&ep.url);
            if !secure && !local && !self.allow_insecure_http {
                return fail(format!(
                    "{at}.url is plain http to a non-loopback host: use https or set allow_insecure_http (local only)"
                ));
            }
            if ep.audience.is_zero() {
                return fail(format!("{at}.audience must not be the zero address"));
            }
            if earlier().any(|o| o.audience == ep.audience) {
                return fail(format!("{at}.audience must be unique"));
            }
        }
        if self.max_concurrent_calls < n {
            return fail(format!(
                "kms_aggregator.max_concurrent_calls ({}) must be >= number of endpoints ({n})",
                self.max_concurrent_calls
            ));
        }
        let call = &self.call;
        if call.timeout < MIN_TIMEOUT || call.timeout > MAX_TIMEOUT {
            return fail(format!(
                "kms_aggregator.call.timeout must be within {MIN_TIMEOUT:?}..={MAX_TIMEOUT:?}"
            ));
        }
        // The waits of each class must all fit before the deadline, otherwise the last retries can never run.
        let past_deadline =
            |at: &str, class: RetryClass| match call.first_retry_past_deadline(class) {
                Some((k, start)) => fail(format!(
                    "{at}: retry {k} would start at {start:?}, not before call.timeout ({:?})",
                    call.timeout
                )),
                None => Ok(()),
            };
        let at = "kms_aggregator.call.retries.waits";
        if let Some(i) = call.retries.waits.iter().position(Duration::is_zero) {
            return fail(format!("{at}[{i}] must be > 0"));
        }
        past_deadline(at, RetryClass::Transient)?;
        let at = "kms_aggregator.call.rate_limit.initial_delay";
        if call.rate_limit.max_retries > 0 {
            if call.rate_limit.initial_delay.is_zero() {
                return fail(format!("{at} must be > 0 when max_retries > 0"));
            }
            past_deadline(at, RetryClass::RateLimit)?;
        }
        let auth = &self.auth;
        if auth.validity <= call.timeout || auth.validity > DEFAULT_MAX_AUTH_VALIDITY {
            return fail(format!(
                "kms_aggregator.auth.validity must be > call.timeout and <= {DEFAULT_MAX_AUTH_VALIDITY:?}"
            ));
        }
        if auth.address.is_zero() {
            return fail("kms_aggregator.auth.address must not be the zero address".to_owned());
        }
        match &auth.signer {
            SignerConfig::PrivateKey { value_env } if value_env.trim().is_empty() => {
                return fail("kms_aggregator.auth.signer.value_env must not be empty".to_owned());
            }
            SignerConfig::AwsKms { key_id, .. } if key_id.trim().is_empty() => {
                return fail("kms_aggregator.auth.signer.key_id must not be empty".to_owned());
            }
            _ => {}
        }
        for (name, threshold) in [
            ("user_decrypt", self.user_decrypt.threshold),
            ("public_decrypt", self.public_decrypt.threshold),
        ] {
            if threshold == 0 || threshold > n {
                return fail(format!(
                    "kms_aggregator.{name}.threshold ({threshold}) must be within 1..={n}"
                ));
            }
        }
        Ok(())
    }
}

/// `localhost`, `127.0.0.0/8` and `::1` only.
pub(crate) fn is_loopback(url: &Url) -> bool {
    match url.host() {
        Some(Host::Domain(domain)) => domain.eq_ignore_ascii_case("localhost"),
        Some(Host::Ipv4(ip)) => ip.is_loopback(),
        Some(Host::Ipv6(ip)) => ip.is_loopback(),
        None => false,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    fn url(s: &str) -> Url {
        Url::parse(s).unwrap()
    }

    /// `n` https endpoints with distinct audiences, a local signer, thresholds 1, the default call config.
    pub(crate) fn valid(n: usize) -> KmsAggregatorConfig {
        KmsAggregatorConfig {
            allow_insecure_http: false,
            skip_tls_verification: false,
            max_concurrent_calls: 64.max(n),
            call: CallConfig::default(),
            user_decrypt: UserDecryptConfig {
                threshold: 1,
                checks: UserChecks::default(),
            },
            public_decrypt: PublicDecryptConfig { threshold: 1 },
            auth: AuthConfig {
                chain_id: 1,
                protocol_config: Address::repeat_byte(0xc0),
                validity: default_validity(),
                address: Address::repeat_byte(0xee),
                signer: SignerConfig::PrivateKey {
                    value_env: "RELAYER_SIGNER_KEY".to_owned(),
                },
            },
            endpoints: (0..n)
                .map(|i| EndpointConfig {
                    name: format!("kms_{i:02}"),
                    url: url(&format!("https://kms-{i:02}.example.net:8443")),
                    audience: audience(i),
                })
                .collect(),
        }
    }

    /// The audience of endpoint `i`: `0x0101…01` for 0, `0x0202…02` for 1, ...
    pub(crate) fn audience(i: usize) -> Address {
        Address::repeat_byte(u8::try_from(i + 1).unwrap())
    }

    fn message(cfg: &KmsAggregatorConfig) -> String {
        cfg.validate().expect_err("expected a validation error").0
    }

    #[test]
    fn valid_config_passes() {
        valid(13).validate().unwrap();
    }

    #[test]
    fn endpoints_must_not_be_empty() {
        assert!(message(&valid(0)).contains("endpoints must not be empty"));
    }

    #[test]
    fn endpoints_are_bounded_by_the_kms_node_label() {
        valid(MAX_KMS_NODES).validate().unwrap();
        assert!(
            message(&valid(MAX_KMS_NODES + 1))
                .contains("kms_aggregator.endpoints must have at most 13 entries (14 given)")
        );
    }

    #[test]
    fn endpoint_name_rules() {
        let mut cfg = valid(2);
        cfg.endpoints[1].name = "  ".into();
        assert!(message(&cfg).contains("endpoints[1].name must not be empty"));
        cfg.endpoints[1].name = "kms_00".into();
        assert!(message(&cfg).contains("endpoints[1].name 'kms_00' must be unique"));
    }

    #[test]
    fn endpoint_url_must_be_unique() {
        let mut cfg = valid(2);
        cfg.endpoints[1].url = cfg.endpoints[0].url.clone();
        assert!(message(&cfg).contains("endpoints[1].url must be unique"));
    }

    #[test]
    fn endpoint_url_rules() {
        for (bad, expected) in [
            ("ftp://kms.example.net", "must use http or https"),
            (
                "https://user:pw@kms.example.net",
                "must not carry credentials",
            ),
            ("https://kms.example.net/?x=1", "must not carry a query"),
            ("https://kms.example.net/#frag", "must not carry a query"),
            (
                "http://kms.example.net",
                "plain http to a non-loopback host",
            ),
        ] {
            let mut cfg = valid(1);
            cfg.endpoints[0].url = url(bad);
            assert!(message(&cfg).contains(expected), "{bad}: {}", message(&cfg));
        }
    }

    #[test]
    fn url_without_host_is_rejected() {
        let mut cfg = valid(1);
        cfg.endpoints[0].url = url("http:///path");
        // `url` parses `http:///path` as a URL with an empty host for special schemes.
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn plain_http_allowed_on_loopback_or_when_opted_in() {
        for host in ["localhost", "127.0.0.1", "[::1]", "LOCALHOST"] {
            let mut cfg = valid(1);
            cfg.endpoints[0].url = url(&format!("http://{host}:3002"));
            cfg.validate().unwrap_or_else(|e| panic!("{host}: {e}"));
        }
        let mut cfg = valid(1);
        cfg.endpoints[0].url = url("http://kms-connector-1:8080");
        assert!(message(&cfg).contains("plain http to a non-loopback host"));
        cfg.allow_insecure_http = true;
        cfg.validate().unwrap();
    }

    #[test]
    fn audience_rules() {
        let mut cfg = valid(2);
        cfg.endpoints[1].audience = Address::ZERO;
        assert!(message(&cfg).contains("endpoints[1].audience must not be the zero address"));
        cfg.endpoints[1].audience = cfg.endpoints[0].audience;
        assert!(message(&cfg).contains("endpoints[1].audience must be unique"));
    }

    #[test]
    fn auth_validity_covers_a_call_and_stays_accepted() {
        let mut cfg = valid(1);
        cfg.auth.validity = cfg.call.timeout;
        assert!(message(&cfg).contains("auth.validity must be > call.timeout"));
        cfg.auth.validity = DEFAULT_MAX_AUTH_VALIDITY + Duration::from_secs(1);
        assert!(message(&cfg).contains("auth.validity must be > call.timeout"));
        cfg.auth.validity = DEFAULT_MAX_AUTH_VALIDITY;
        cfg.validate().unwrap();
    }

    #[test]
    fn auth_address_must_not_be_zero() {
        let mut cfg = valid(1);
        cfg.auth.address = Address::ZERO;
        assert!(message(&cfg).contains("kms_aggregator.auth.address must not be the zero address"));
    }

    #[test]
    fn signer_fields_must_not_be_empty() {
        let mut cfg = valid(1);
        cfg.auth.signer = SignerConfig::PrivateKey {
            value_env: " ".to_owned(),
        };
        assert!(message(&cfg).contains("auth.signer.value_env must not be empty"));
        cfg.auth.signer = SignerConfig::AwsKms {
            key_id: " ".to_owned(),
        };
        assert!(message(&cfg).contains("auth.signer.key_id must not be empty"));
    }

    #[test]
    fn max_concurrent_calls_covers_one_fan_out() {
        let mut cfg = valid(13);
        cfg.max_concurrent_calls = 12;
        assert!(
            message(&cfg).contains("max_concurrent_calls (12) must be >= number of endpoints (13)")
        );
    }

    #[test]
    fn call_timeout_bounds() {
        let mut cfg = valid(1);
        cfg.call.timeout = Duration::ZERO;
        assert!(message(&cfg).contains("call.timeout must be within 1s..=60s"));
        cfg.call.timeout = Duration::from_millis(999);
        assert!(message(&cfg).contains("call.timeout must be within 1s..=60s"));
        cfg.call.timeout = MAX_TIMEOUT + Duration::from_millis(1);
        assert!(message(&cfg).contains("call.timeout must be within 1s..=60s"));
        cfg.call.timeout = MAX_TIMEOUT;
        cfg.validate().unwrap();
        // The bound alone: the default retry lists need about 3 s, which their own rule says.
        cfg.call.timeout = MIN_TIMEOUT;
        cfg.call.retries = waits(&[]);
        cfg.call.rate_limit.max_retries = 0;
        cfg.validate().unwrap();
    }

    #[test]
    fn waits_are_trimmed_and_empty_entries_dropped() {
        let parsed = |json: &str| serde_json::from_str::<CallConfig>(json).map(|c| c.retries.waits);
        assert_eq!(
            parsed(r#"{"retries": {"waits": ["200ms", " 1s ", ""]}}"#).unwrap(),
            waits(&[200, 1000]).waits
        );
        assert!(
            parsed(r#"{"retries": {"waits": [""]}}"#)
                .unwrap()
                .is_empty()
        );
        let err = parsed(r#"{"retries": {"waits": ["200ms", "nope"]}}"#)
            .unwrap_err()
            .to_string();
        assert!(err.contains("\"nope\""), "{err}");
    }

    /// A transient budget of `ms` waits.
    fn waits(ms: &[u64]) -> RetryConfig {
        RetryConfig {
            waits: ms.iter().copied().map(Duration::from_millis).collect(),
        }
    }

    #[test]
    fn transient_waits_must_fit_before_the_deadline() {
        let mut cfg = valid(1);
        cfg.call.timeout = Duration::from_secs(1);
        // Only the transient class under test: the default 1 s rate-limit wait would not fit either.
        cfg.call.rate_limit.max_retries = 0;
        // Retry 3 would start after 400 + 400 + 400 ms of waits: never before a 1 s deadline.
        cfg.call.retries = waits(&[400, 400, 400, 400]);
        assert_eq!(
            message(&cfg),
            "kms_aggregator.call.retries.waits: retry 3 would start at 1.2s, not before call.timeout (1s)"
        );
        cfg.call.retries = waits(&[400, 400]);
        cfg.validate().unwrap();
        // The message names the zero entry.
        cfg.call.retries = waits(&[200, 0, 200]);
        assert_eq!(
            message(&cfg),
            "kms_aggregator.call.retries.waits[1] must be > 0"
        );
        // The last wait of a long list is the one that reaches the deadline.
        cfg.call.retries = waits(&[1; 1000]);
        assert!(message(&cfg).contains("retries.waits: retry 1000 would start at 1s"));
        // An empty list is a single attempt and nothing to check.
        cfg.call.retries = waits(&[]);
        cfg.validate().unwrap();
    }

    #[test]
    fn rate_limit_waits_must_fit_before_the_deadline() {
        let mut cfg = valid(1);
        // 1 + 2 + 4 = 7 s > 5 s: retry 3 can never run.
        cfg.call.rate_limit.max_retries = 3;
        assert_eq!(
            message(&cfg),
            "kms_aggregator.call.rate_limit.initial_delay: retry 3 would start at 7s, not before call.timeout (5s)"
        );
        cfg.call.rate_limit.max_retries = 2;
        cfg.validate().unwrap();
        cfg.call.rate_limit.initial_delay = Duration::ZERO;
        assert!(
            message(&cfg).contains("rate_limit.initial_delay must be > 0 when max_retries > 0")
        );
        cfg.call.rate_limit = RateLimitConfig {
            max_retries: 0,
            initial_delay: Duration::ZERO,
        };
        cfg.validate().unwrap();
    }

    #[test]
    fn call_section_defaults_and_rejects_the_old_keys() {
        let call: CallConfig = serde_json::from_str("{}").unwrap();
        assert_eq!(call.timeout, Duration::from_secs(5));
        assert_eq!(call.retries.waits, waits(&[200, 300, 500, 800, 1000]).waits);
        assert_eq!(
            (call.rate_limit.max_retries, call.rate_limit.initial_delay),
            (1, Duration::from_secs(1))
        );
        let mut cfg = valid(1);
        cfg.call = call;
        cfg.validate().unwrap();
        // The waits carry units; an empty list is a single attempt.
        let listed: CallConfig =
            serde_json::from_str(r#"{"retries": {"waits": ["200ms", "1s"]}}"#).unwrap();
        assert_eq!(listed.retries.waits, waits(&[200, 1000]).waits);
        let none: CallConfig = serde_json::from_str(r#"{"retries": {"waits": []}}"#).unwrap();
        assert!(none.retries.waits.is_empty());
        // The curve keys of earlier versions are refused, so a stale file fails at startup.
        for (old, key) in [
            (
                r#"{"retries": {"max_retries": 1, "delay": "200ms", "backoff_max": "4s"}}"#,
                "max_retries",
            ),
            (r#"{"retries": {"delay": "200ms"}}"#, "delay"),
        ] {
            let err = serde_json::from_str::<CallConfig>(old)
                .unwrap_err()
                .to_string();
            assert!(err.contains(key), "{err}");
        }
    }

    #[test]
    fn threshold_within_one_and_n() {
        let mut cfg = valid(3);
        cfg.user_decrypt.threshold = 0;
        assert!(message(&cfg).contains("user_decrypt.threshold (0) must be within 1..=3"));
        cfg.user_decrypt.threshold = 3;
        cfg.public_decrypt.threshold = 4;
        assert!(message(&cfg).contains("public_decrypt.threshold (4) must be within 1..=3"));
        cfg.public_decrypt.threshold = 3;
        cfg.validate().unwrap();
    }

    #[test]
    fn user_checks_are_off_by_default_and_strict() {
        assert_eq!(
            UserChecks::default(),
            UserChecks {
                decryption_id_match: false,
                decryption_id_majority: false
            }
        );
        let parsed: UserChecks = serde_json::from_str("{}").unwrap();
        assert_eq!(parsed, UserChecks::default());
        let parsed: UserChecks =
            serde_json::from_str(r#"{"decryption_id_majority": true}"#).unwrap();
        assert!(parsed.decryption_id_majority && !parsed.decryption_id_match);
        assert!(serde_json::from_str::<UserChecks>(r#"{"decryption_id": true}"#).is_err());
        let flow: UserDecryptConfig = serde_json::from_str(r#"{"threshold": 9}"#).unwrap();
        assert_eq!((flow.threshold, flow.checks), (9, UserChecks::default()));
    }

    #[test]
    fn transient_waits_are_read_from_the_list() {
        let call = CallConfig::default();
        let ms = |v| Duration::from_millis(v);
        let delay = |k| call.delay_for(RetryClass::Transient, k, None);
        assert_eq!(
            (delay(1), delay(2), delay(3), delay(4), delay(5)),
            (ms(200), ms(300), ms(500), ms(800), ms(1000))
        );
        assert_eq!(delay(0), ms(200));
        // Past the list: zero, never slept (the budget is exhausted first).
        assert_eq!(
            (delay(6), delay(u32::MAX)),
            (Duration::ZERO, Duration::ZERO)
        );
        // Retry-After never applies to the transient class.
        assert_eq!(
            call.delay_for(RetryClass::Transient, 1, Some(Duration::from_secs(9))),
            ms(200)
        );
    }

    #[test]
    fn rate_limit_delay_doubles_from_the_larger_of_floor_and_header() {
        let call = CallConfig::default();
        let s = Duration::from_secs;
        let delay = |k, after| call.delay_for(RetryClass::RateLimit, k, after);
        assert_eq!(
            (delay(1, None), delay(2, None), delay(3, None)),
            (s(1), s(2), s(4))
        );
        assert_eq!(delay(0, None), s(1));
        // The header raises the base; a header below the floor is ignored.
        assert_eq!((delay(1, Some(s(2))), delay(2, Some(s(2)))), (s(2), s(4)));
        assert_eq!(delay(1, Some(Duration::ZERO)), s(1));
        assert_eq!(delay(u32::MAX, None), s(1).saturating_mul(u32::MAX));
        assert_eq!(
            (
                call.max_retries(RetryClass::Transient),
                call.max_retries(RetryClass::RateLimit)
            ),
            (5, 1)
        );
    }

    #[test]
    fn is_loopback_cases() {
        for (u, expected) in [
            ("http://localhost:1", true),
            ("http://LocalHost", true),
            ("http://127.0.0.1", true),
            ("http://127.1.2.3", true),
            ("http://[::1]", true),
            ("http://0.0.0.0", false),
            ("http://localhost.evil.com", false),
            ("http://10.0.0.1", false),
            ("https://kms.example.net", false),
        ] {
            assert_eq!(is_loopback(&url(u)), expected, "{u}");
        }
    }

    #[test]
    fn signer_config_parses_both_variants() {
        let local: SignerConfig =
            serde_json::from_str(r#"{"type": "private_key", "value_env": "KEY"}"#).unwrap();
        assert_eq!(
            local,
            SignerConfig::PrivateKey {
                value_env: "KEY".to_owned()
            }
        );
        let aws: SignerConfig =
            serde_json::from_str(r#"{"type": "aws_kms", "key_id": "k"}"#).unwrap();
        assert_eq!(
            aws,
            SignerConfig::AwsKms {
                key_id: "k".to_owned()
            }
        );
        assert!(
            serde_json::from_str::<SignerConfig>(r#"{"type": "private_key", "key": "0x01"}"#)
                .is_err()
        );
    }
}
