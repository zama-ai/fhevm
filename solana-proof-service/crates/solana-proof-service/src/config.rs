//! Service configuration loaded from YAML + `SOLANA_PROOF__*` env overrides.

use std::net::SocketAddr;
use std::path::Path;
use std::time::Duration;

use serde::Deserialize;

const DEFAULT_CONFIG_PATH: &str = "config/app.yaml";
const CONFIG_PATH_ENV: &str = "SOLANA_PROOF_CONFIG_PATH";
const ENV_PREFIX: &str = "SOLANA_PROOF";
const ENV_SEPARATOR: &str = "__";

#[derive(Clone, Debug, Deserialize)]
pub struct ServiceConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub solana: SolanaConfig,
    pub yellowstone: YellowstoneConfig,
    #[serde(default)]
    pub readiness: ReadinessConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub bind_address: SocketAddr,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub connection_string: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

fn default_max_connections() -> u32 {
    10
}

#[derive(Clone, Debug, Deserialize)]
pub struct SolanaConfig {
    /// Base58 host program id.
    pub program_id: String,
    /// Confirmed JSON-RPC URL used only for on-chain peak checks (read-only).
    pub rpc_url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct YellowstoneConfig {
    pub grpc_url: String,
    #[serde(default)]
    pub x_token: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReadinessConfig {
    /// Max silence between ingest progress heartbeats before `source_lagging`.
    #[serde(default = "default_max_ingest_silence_secs")]
    pub max_ingest_silence_secs: u64,
}

fn default_max_ingest_silence_secs() -> u64 {
    60
}

impl Default for ReadinessConfig {
    fn default() -> Self {
        Self {
            max_ingest_silence_secs: default_max_ingest_silence_secs(),
        }
    }
}

impl ReadinessConfig {
    pub fn max_ingest_silence(&self) -> Duration {
        Duration::from_secs(self.max_ingest_silence_secs)
    }
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
        if self.readiness.max_ingest_silence_secs == 0 {
            anyhow::bail!("readiness.max_ingest_silence_secs must be >= 1");
        }
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
            continue;
        }
        set_path(value, &segments, parse_env_value(&raw))?;
    }
    Ok(())
}

fn parse_env_value(raw: &str) -> serde_yaml::Value {
    if let Ok(v) = serde_yaml::from_str::<serde_yaml::Value>(raw) {
        match v {
            serde_yaml::Value::String(_) | serde_yaml::Value::Null => {
                serde_yaml::Value::String(raw.to_owned())
            }
            other => other,
        }
    } else {
        serde_yaml::Value::String(raw.to_owned())
    }
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

    #[test]
    fn loads_minimal_yaml() {
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
        assert_eq!(config.readiness.max_ingest_silence_secs, 60);
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
