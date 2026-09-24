//! Config structs of the aggregator (deserialised by `config.rs`) and their validation.

use std::time::Duration;

use serde::Deserialize;
use url::{Host, Url};

/// Upper bound of every configured duration: keeps deadline arithmetic trivially safe.
pub const MAX_TIMEOUT: Duration = Duration::from_secs(60);

/// Invalid configuration; the message names the offending field with its dotted path.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{0}")]
pub struct ConfigError(pub String);

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KmsAggregatorConfig {
    /// Allow plain http and `auth: none` towards non-loopback hosts. Local/compose only.
    #[serde(default)]
    pub allow_insecure_http: bool,
    /// HTTP calls in flight at once, all aggregations together (one semaphore per process).
    #[serde(default = "default_max_concurrent_calls")]
    pub max_concurrent_calls: usize,
    pub call: CallConfig,
    pub user_decrypt: UserDecryptConfig,
    pub public_decrypt: PublicDecryptConfig,
    pub endpoints: Vec<EndpointConfig>,
}

fn default_max_concurrent_calls() -> usize {
    64
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CallConfig {
    /// The deadline: one aggregation, and every call in it, ends by then.
    #[serde(with = "humantime_serde")]
    pub timeout: Duration,
    #[serde(default)]
    pub retries: RetryConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetryConfig {
    /// Retries after the first attempt, retryable errors only. 0 = one attempt. Any value: the deadline
    /// (`call.timeout`) is the real bound, a retry whose delay does not fit before it is skipped.
    #[serde(default)]
    pub max_retries: u32,
    /// Delay before the first retry; retry `k` waits `min(delay * 2^(k-1), backoff_max)`.
    #[serde(with = "humantime_serde", default = "default_delay")]
    pub delay: Duration,
    /// Cap of the exponential delay, at most `call.timeout`.
    #[serde(with = "humantime_serde", default = "default_backoff_max")]
    pub backoff_max: Duration,
}

fn default_delay() -> Duration {
    Duration::from_millis(500)
}

fn default_backoff_max() -> Duration {
    Duration::from_secs(4)
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 0,
            delay: default_delay(),
            backoff_max: default_backoff_max(),
        }
    }
}

impl RetryConfig {
    /// Delay before retry number `retry` (1-based): `min(delay * 2^(retry-1), backoff_max)`, saturating.
    pub fn delay_for(&self, retry: u32) -> Duration {
        let factor = 2u32.saturating_pow(retry.saturating_sub(1));
        self.delay.saturating_mul(factor).min(self.backoff_max)
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
    pub auth: AuthConfig,
}

/// The secret itself is never in the config, only the name of the env var holding it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum AuthConfig {
    None,
    ApiKey { value_env: String },
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
            match &ep.auth {
                AuthConfig::None if !local && !self.allow_insecure_http => {
                    return fail(format!(
                        "{at}.auth is `none` to a non-loopback host: configure an api_key or set allow_insecure_http (local only)"
                    ));
                }
                AuthConfig::ApiKey { value_env } if value_env.trim().is_empty() => {
                    return fail(format!("{at}.auth.value_env must not be empty"));
                }
                _ => {}
            }
        }
        if self.max_concurrent_calls < n {
            return fail(format!(
                "kms_aggregator.max_concurrent_calls ({}) must be >= number of endpoints ({n})",
                self.max_concurrent_calls
            ));
        }
        let call = &self.call;
        if call.timeout.is_zero() || call.timeout > MAX_TIMEOUT {
            return fail(format!(
                "kms_aggregator.call.timeout must be within 1ms..={MAX_TIMEOUT:?}"
            ));
        }
        let retries = &call.retries;
        if retries.max_retries > 0
            && (retries.delay.is_zero()
                || retries.delay > retries.backoff_max
                || retries.backoff_max > call.timeout)
        {
            return fail(
                "kms_aggregator.call.retries must satisfy 0 < delay <= backoff_max <= call.timeout"
                    .to_owned(),
            );
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

    /// `n` https endpoints with API keys, thresholds 1, single attempt.
    pub(crate) fn valid(n: usize) -> KmsAggregatorConfig {
        KmsAggregatorConfig {
            allow_insecure_http: false,
            max_concurrent_calls: 64.max(n),
            call: CallConfig {
                timeout: Duration::from_secs(5),
                retries: RetryConfig::default(),
            },
            user_decrypt: UserDecryptConfig {
                threshold: 1,
                checks: UserChecks::default(),
            },
            public_decrypt: PublicDecryptConfig { threshold: 1 },
            endpoints: (0..n)
                .map(|i| EndpointConfig {
                    name: format!("kms_{i:02}"),
                    url: url(&format!("https://kms-{i:02}.example.net:8443")),
                    auth: AuthConfig::ApiKey {
                        value_env: format!("KMS_{i:02}_API_KEY"),
                    },
                })
                .collect(),
        }
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
    fn plain_http_and_no_auth_allowed_on_loopback_or_when_opted_in() {
        for host in ["localhost", "127.0.0.1", "[::1]", "LOCALHOST"] {
            let mut cfg = valid(1);
            cfg.endpoints[0].url = url(&format!("http://{host}:3002"));
            cfg.endpoints[0].auth = AuthConfig::None;
            cfg.validate().unwrap_or_else(|e| panic!("{host}: {e}"));
        }
        let mut cfg = valid(1);
        cfg.endpoints[0].url = url("http://kms-connector-1:8080");
        cfg.endpoints[0].auth = AuthConfig::None;
        assert!(message(&cfg).contains("plain http to a non-loopback host"));
        cfg.allow_insecure_http = true;
        cfg.validate().unwrap();
    }

    #[test]
    fn no_auth_to_remote_https_host_is_rejected() {
        let mut cfg = valid(1);
        cfg.endpoints[0].auth = AuthConfig::None;
        assert!(message(&cfg).contains("auth is `none` to a non-loopback host"));
    }

    #[test]
    fn api_key_env_name_must_not_be_empty() {
        let mut cfg = valid(1);
        cfg.endpoints[0].auth = AuthConfig::ApiKey {
            value_env: String::new(),
        };
        assert!(message(&cfg).contains("auth.value_env must not be empty"));
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
        assert!(message(&cfg).contains("call.timeout must be within"));
        cfg.call.timeout = MAX_TIMEOUT + Duration::from_millis(1);
        assert!(message(&cfg).contains("call.timeout must be within"));
        cfg.call.timeout = MAX_TIMEOUT;
        cfg.validate().unwrap();
    }

    #[test]
    fn retry_rules() {
        let mut cfg = valid(1);
        // No ceiling on the count: the deadline bounds the retries that actually run.
        cfg.call.retries.max_retries = u32::MAX;
        cfg.validate().unwrap();
        cfg.call.retries.max_retries = 2;
        cfg.call.retries.delay = Duration::ZERO;
        assert!(message(&cfg).contains("0 < delay <= backoff_max <= call.timeout"));
        cfg.call.retries.delay = Duration::from_secs(5);
        cfg.call.retries.backoff_max = Duration::from_secs(4);
        assert!(message(&cfg).contains("0 < delay <= backoff_max <= call.timeout"));
        // backoff_max beyond the deadline can never take effect: refused.
        cfg.call.retries.delay = Duration::from_millis(500);
        cfg.call.retries.backoff_max = cfg.call.timeout + Duration::from_secs(1);
        assert!(message(&cfg).contains("0 < delay <= backoff_max <= call.timeout"));
        cfg.call.retries.backoff_max = cfg.call.timeout;
        cfg.validate().unwrap();
        // With no retries the delays are irrelevant.
        cfg.call.retries.max_retries = 0;
        cfg.call.retries.delay = Duration::ZERO;
        cfg.call.retries.backoff_max = Duration::from_secs(600);
        cfg.validate().unwrap();
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
    fn delay_for_doubles_and_caps() {
        let retries = RetryConfig::default();
        let ms = |v| Duration::from_millis(v);
        assert_eq!(retries.delay_for(1), ms(500));
        assert_eq!(retries.delay_for(2), ms(1000));
        assert_eq!(retries.delay_for(3), ms(2000));
        assert_eq!(retries.delay_for(4), ms(4000));
        assert_eq!(retries.delay_for(5), ms(4000));
        assert_eq!(retries.delay_for(0), ms(500));
        assert_eq!(retries.delay_for(u32::MAX), ms(4000));
    }

    #[test]
    fn retry_config_default() {
        let retries = RetryConfig::default();
        assert_eq!(retries.max_retries, 0);
        assert_eq!(retries.delay, Duration::from_millis(500));
        assert_eq!(retries.backoff_max, Duration::from_secs(4));
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
    fn debug_output_never_contains_a_secret() {
        let cfg = valid(2);
        let printed = format!("{cfg:?}");
        assert!(printed.contains("KMS_00_API_KEY"));
        assert!(!printed.contains("Bearer"));
    }
}
