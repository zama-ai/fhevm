//! Service configuration loaded from YAML + `SOLANA_PROOF__*` env overrides.

use std::net::SocketAddr;
use std::path::Path;

use serde::Deserialize;

const DEFAULT_CONFIG_PATH: &str = "config/app.yaml";
const CONFIG_PATH_ENV: &str = "SOLANA_PROOF_CONFIG_PATH";
const ENV_PREFIX: &str = "SOLANA_PROOF";
const ENV_SEPARATOR: &str = "__";

/// Known `SOLANA_PROOF__*` override paths (lowercase segments). Typos fail fast.
const KNOWN_ENV_PATHS: &[&[&str]] = &[
    &["server", "bind_address"],
    &["database", "connection_string"],
    &["database", "max_connections"],
    &["solana", "program_id"],
    &["solana", "rpc_url"],
    &["yellowstone", "grpc_url"],
    &["yellowstone", "x_token"],
];

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub solana: SolanaConfig,
    pub yellowstone: YellowstoneConfig,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerConfig {
    pub bind_address: SocketAddr,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseConfig {
    pub connection_string: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

fn default_max_connections() -> u32 {
    10
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SolanaConfig {
    /// Base58 host program id.
    pub program_id: String,
    /// Confirmed JSON-RPC URL used only for on-chain peak checks (read-only).
    pub rpc_url: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct YellowstoneConfig {
    pub grpc_url: String,
    #[serde(default)]
    pub x_token: Option<String>,
}

impl ServiceConfig {
    pub fn load() -> anyhow::Result<Self> {
        let path =
            std::env::var(CONFIG_PATH_ENV).unwrap_or_else(|_| DEFAULT_CONFIG_PATH.to_owned());
        Self::load_from_path(path)
    }

    pub fn load_from_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let raw = std::fs::read_to_string(path)
            .map_err(|err| anyhow::anyhow!("failed to read config {}: {err}", path.display()))?;
        let mut value: serde_yaml::Value = serde_yaml::from_str(&raw)
            .map_err(|err| anyhow::anyhow!("failed to parse config {}: {err}", path.display()))?;
        apply_env_overrides(&mut value)?;
        let config: Self = serde_yaml::from_value(value)
            .map_err(|err| anyhow::anyhow!("invalid config {}: {err}", path.display()))?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.database.connection_string.trim().is_empty() {
            anyhow::bail!("database.connection_string must not be empty");
        }
        if self.database.max_connections == 0 {
            anyhow::bail!("database.max_connections must be >= 1");
        }
        if self.solana.rpc_url.trim().is_empty() {
            anyhow::bail!("solana.rpc_url must not be empty");
        }
        if self.yellowstone.grpc_url.trim().is_empty() {
            anyhow::bail!("yellowstone.grpc_url must not be empty");
        }
        decode_program_id(&self.solana.program_id)?;
        Ok(())
    }

    pub fn program_id_bytes(&self) -> anyhow::Result<[u8; 32]> {
        decode_program_id(&self.solana.program_id)
    }
}

pub fn decode_program_id(encoded: &str) -> anyhow::Result<[u8; 32]> {
    let bytes = bs58::decode(encoded)
        .into_vec()
        .map_err(|err| anyhow::anyhow!("invalid program_id base58: {err}"))?;
    let bytes: [u8; 32] = bytes.try_into().map_err(|bytes: Vec<u8>| {
        anyhow::anyhow!("program_id must decode to 32 bytes, got {}", bytes.len())
    })?;
    Ok(bytes)
}

fn apply_env_overrides(value: &mut serde_yaml::Value) -> anyhow::Result<()> {
    for (key, raw) in std::env::vars() {
        let Some(path) = key.strip_prefix(&format!("{ENV_PREFIX}{ENV_SEPARATOR}")) else {
            continue;
        };
        let segments: Vec<&str> = path.split(ENV_SEPARATOR).collect();
        if segments.is_empty() || segments.iter().any(|s| s.is_empty()) {
            anyhow::bail!("invalid {ENV_PREFIX}{ENV_SEPARATOR}* path `{key}`");
        }
        if !is_known_env_path(&segments) {
            anyhow::bail!(
                "unknown config env override `{key}`; expected one of {}",
                known_env_override_names().join(", ")
            );
        }
        set_path(value, &segments, parse_env_value(&segments, &raw))?;
    }
    Ok(())
}

fn is_known_env_path(segments: &[&str]) -> bool {
    let lower: Vec<String> = segments.iter().map(|s| s.to_ascii_lowercase()).collect();
    KNOWN_ENV_PATHS.iter().any(|known| {
        known.len() == lower.len() && known.iter().zip(lower.iter()).all(|(a, b)| a == b)
    })
}

fn known_env_override_names() -> Vec<String> {
    KNOWN_ENV_PATHS
        .iter()
        .map(|parts| {
            format!(
                "{ENV_PREFIX}{ENV_SEPARATOR}{}",
                parts
                    .iter()
                    .map(|p| p.to_ascii_uppercase())
                    .collect::<Vec<_>>()
                    .join(ENV_SEPARATOR)
            )
        })
        .collect()
}

/// Env values for string fields stay strings (tokens like `12345` / `true` must
/// not become YAML integers/bools). Only known numeric fields are typed.
fn parse_env_value(segments: &[&str], raw: &str) -> serde_yaml::Value {
    if is_numeric_env_path(segments) {
        if let Ok(v) = serde_yaml::from_str::<serde_yaml::Value>(raw) {
            if matches!(v, serde_yaml::Value::Number(_)) {
                return v;
            }
        }
    }
    serde_yaml::Value::String(raw.to_owned())
}

fn is_numeric_env_path(segments: &[&str]) -> bool {
    let lower: Vec<String> = segments.iter().map(|s| s.to_ascii_lowercase()).collect();
    matches!(
        lower.as_slice(),
        [a, b] if a == "database" && b == "max_connections"
    )
}

fn set_path(
    root: &mut serde_yaml::Value,
    segments: &[&str],
    leaf: serde_yaml::Value,
) -> anyhow::Result<()> {
    let mut cursor = root;
    for (idx, segment) in segments.iter().enumerate() {
        let key = segment.to_ascii_lowercase();
        if idx + 1 == segments.len() {
            match cursor {
                serde_yaml::Value::Mapping(map) => {
                    map.insert(serde_yaml::Value::String(key), leaf);
                    return Ok(());
                }
                other => {
                    let mut map = serde_yaml::Mapping::new();
                    map.insert(serde_yaml::Value::String(key), leaf);
                    *other = serde_yaml::Value::Mapping(map);
                    return Ok(());
                }
            }
        }
        if !cursor.is_mapping() {
            *cursor = serde_yaml::Value::Mapping(serde_yaml::Mapping::new());
        }
        let map = cursor.as_mapping_mut().expect("mapping just ensured");
        let entry = map
            .entry(serde_yaml::Value::String(key))
            .or_insert_with(|| serde_yaml::Value::Mapping(serde_yaml::Mapping::new()));
        cursor = entry;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn clear_test_env_overrides() {
        std::env::remove_var("SOLANA_PROOF__READINESS__TYPO_SECS");
        std::env::remove_var("SOLANA_PROOF__DATABASE__MAX_CONNECTIONS");
        std::env::remove_var("SOLANA_PROOF__YELLOWSTONE__X_TOKEN");
        std::env::remove_var("SOLANA_PROOF__CONFIG_PATH");
    }

    #[test]
    fn loads_minimal_yaml() {
        let _guard = env_lock().lock().unwrap();
        clear_test_env_overrides();
        let file = tempfile_config(
            r#"
server:
  bind_address: 127.0.0.1:8080
database:
  connection_string: postgres://localhost/solana_proof
solana:
  program_id: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
  rpc_url: http://127.0.0.1:8899
yellowstone:
  grpc_url: http://127.0.0.1:10000
"#,
        );
        let config = ServiceConfig::load_from_path(file.path()).unwrap();
        assert_eq!(config.server.bind_address.port(), 8080);
        file.close().ok();
    }

    #[test]
    fn string_shaped_env_secrets_stay_strings() {
        let _guard = env_lock().lock().unwrap();
        clear_test_env_overrides();
        let file = tempfile_config(
            r#"
server:
  bind_address: 127.0.0.1:8080
database:
  connection_string: postgres://localhost/solana_proof
solana:
  program_id: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
  rpc_url: http://127.0.0.1:8899
yellowstone:
  grpc_url: http://127.0.0.1:10000
"#,
        );
        // SAFETY: test-only process-local env; serialized by env_lock.
        std::env::set_var("SOLANA_PROOF__YELLOWSTONE__X_TOKEN", "12345");
        std::env::set_var("SOLANA_PROOF__DATABASE__MAX_CONNECTIONS", "7");
        let config = ServiceConfig::load_from_path(file.path()).unwrap();
        clear_test_env_overrides();
        assert_eq!(config.yellowstone.x_token.as_deref(), Some("12345"));
        assert_eq!(config.database.max_connections, 7);
        file.close().ok();
    }

    #[test]
    fn boolean_looking_string_env_stays_string() {
        let _guard = env_lock().lock().unwrap();
        clear_test_env_overrides();
        let file = tempfile_config(
            r#"
server:
  bind_address: 127.0.0.1:8080
database:
  connection_string: postgres://localhost/solana_proof
solana:
  program_id: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
  rpc_url: http://127.0.0.1:8899
yellowstone:
  grpc_url: http://127.0.0.1:10000
"#,
        );
        std::env::set_var("SOLANA_PROOF__YELLOWSTONE__X_TOKEN", "true");
        let config = ServiceConfig::load_from_path(file.path()).unwrap();
        clear_test_env_overrides();
        assert_eq!(config.yellowstone.x_token.as_deref(), Some("true"));
        file.close().ok();
    }

    #[test]
    fn rejects_unknown_yaml_fields() {
        let _guard = env_lock().lock().unwrap();
        clear_test_env_overrides();
        let file = tempfile_config(
            r#"
server:
  bind_address: 127.0.0.1:8080
  typo_field: true
database:
  connection_string: postgres://localhost/solana_proof
solana:
  program_id: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
  rpc_url: http://127.0.0.1:8899
yellowstone:
  grpc_url: http://127.0.0.1:10000
"#,
        );
        let err = ServiceConfig::load_from_path(file.path()).unwrap_err();
        assert!(
            err.to_string().contains("unknown field") || err.to_string().contains("typo_field"),
            "unexpected error: {err}"
        );
        file.close().ok();
    }

    #[test]
    fn rejects_unknown_env_override_paths() {
        let _guard = env_lock().lock().unwrap();
        clear_test_env_overrides();
        let file = tempfile_config(
            r#"
server:
  bind_address: 127.0.0.1:8080
database:
  connection_string: postgres://localhost/solana_proof
solana:
  program_id: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
  rpc_url: http://127.0.0.1:8899
yellowstone:
  grpc_url: http://127.0.0.1:10000
"#,
        );
        // SAFETY: test-only process-local env; serialized by env_lock.
        std::env::set_var("SOLANA_PROOF__READINESS__TYPO_SECS", "5");
        let err = ServiceConfig::load_from_path(file.path()).unwrap_err();
        clear_test_env_overrides();
        assert!(
            err.to_string().contains("unknown config env override"),
            "unexpected error: {err}"
        );
        file.close().ok();
    }

    #[test]
    fn rejects_misspelled_config_path_double_underscore_env() {
        let _guard = env_lock().lock().unwrap();
        clear_test_env_overrides();
        let file = tempfile_config(
            r#"
server:
  bind_address: 127.0.0.1:8080
database:
  connection_string: postgres://localhost/solana_proof
solana:
  program_id: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
  rpc_url: http://127.0.0.1:8899
yellowstone:
  grpc_url: http://127.0.0.1:10000
"#,
        );
        // Valid path env is SOLANA_PROOF_CONFIG_PATH (single underscore after PROOF).
        std::env::set_var("SOLANA_PROOF__CONFIG_PATH", "/tmp/nope.yaml");
        let err = ServiceConfig::load_from_path(file.path()).unwrap_err();
        clear_test_env_overrides();
        assert!(
            err.to_string().contains("unknown config env override"),
            "unexpected error: {err}"
        );
        file.close().ok();
    }

    fn tempfile_config(contents: &str) -> NamedTemp {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "solana-proof-config-{}-{}.yaml",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
        NamedTemp { path }
    }

    struct NamedTemp {
        path: std::path::PathBuf,
    }

    impl NamedTemp {
        fn path(&self) -> &Path {
            &self.path
        }

        fn close(self) -> std::io::Result<()> {
            std::fs::remove_file(&self.path)
        }
    }

    impl Drop for NamedTemp {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}
