use anyhow::{Context, Result, bail};
use serde_json::Value;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};

const DEFAULT_RPC_URL: &str = "http://127.0.0.1:18999";
const LOCALNET_TIMEOUT: Duration = Duration::from_secs(180);

pub struct SolanaLocalnet {
    repo_root: PathBuf,
    addresses_env: PathBuf,
    process: Option<Child>,
}

impl SolanaLocalnet {
    pub async fn start() -> Result<Self> {
        let repo_root = workspace_root()?;
        let solana_host_root = repo_root.join("solana-host-contracts");
        let addresses_env = solana_host_root.join("addresses/.env.host");

        if std::env::var("SOLANA_E2E_USE_EXISTING_LOCALNET").is_ok()
            || rpc_is_healthy(DEFAULT_RPC_URL).await
        {
            run_shell(
                &repo_root,
                &format!(
                    "source ~/.zshrc && make -C {} localnet-bootstrap",
                    shell_escape(&solana_host_root)
                ),
                false,
            )
            .await
            .context("bootstrap existing Solana localnet")?;

            wait_for_localnet(&addresses_env).await?;

            return Ok(Self {
                repo_root,
                addresses_env,
                process: None,
            });
        }

        let command = format!(
            "source ~/.zshrc && make -C {} localnet",
            shell_escape(&solana_host_root)
        );
        let mut child = Command::new("/bin/zsh")
            .arg("-lc")
            .arg(command)
            .current_dir(&repo_root)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .context("spawn solana localnet")?;

        match wait_for_localnet(&addresses_env).await {
            Ok(()) => Ok(Self {
                repo_root,
                addresses_env,
                process: Some(child),
            }),
            Err(err) => {
                let _ = child.kill();
                let _ = child.wait();
                Err(err)
            }
        }
    }

    pub fn addresses_env_path(&self) -> &Path {
        &self.addresses_env
    }

    pub fn repo_root(&self) -> &Path {
        &self.repo_root
    }

    pub async fn run_scenario(&self, scenario: &str) -> Result<Value> {
        let local_cli_manifest =
            self.repo_root.join("solana-host-contracts/local-cli/Cargo.toml");
        let command = format!(
            "source ~/.zshrc && cargo run --manifest-path {} -- {} --addresses-env {}",
            shell_escape(&local_cli_manifest),
            scenario,
            shell_escape(&self.addresses_env),
        );
        let stdout = run_shell(&self.repo_root, &command, true)
            .await
            .with_context(|| format!("run solana scenario {scenario}"))?;
        serde_json::from_str(stdout.trim()).context("parse local-cli scenario JSON")
    }

    pub fn load_addresses(&self) -> Result<HashMap<String, String>> {
        load_env_file(&self.addresses_env)
    }
}

impl Drop for SolanaLocalnet {
    fn drop(&mut self) {
        if let Some(child) = &mut self.process {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

async fn wait_for_localnet(addresses_env: &Path) -> Result<()> {
    let started = Instant::now();
    loop {
        if started.elapsed() > LOCALNET_TIMEOUT {
            bail!(
                "timed out waiting for Solana localnet bootstrap ({})",
                addresses_env.display()
            );
        }

        if let Ok(values) = load_env_file(addresses_env) {
            if let Some(rpc_url) = values.get("SOLANA_HOST_RPC_URL") {
                if rpc_is_healthy(rpc_url).await {
                    return Ok(());
                }
            }
        } else if rpc_is_healthy(DEFAULT_RPC_URL).await {
            return Ok(());
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

async fn rpc_is_healthy(rpc_url: &str) -> bool {
    let client = reqwest::Client::new();
    let response = client
        .post(rpc_url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getHealth",
            "params": [],
        }))
        .send()
        .await;

    match response {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

async fn run_shell(repo_root: &Path, command: &str, capture_stdout: bool) -> Result<String> {
    let mut cmd = Command::new("/bin/zsh");
    cmd.arg("-lc").arg(command).current_dir(repo_root);

    if capture_stdout {
        cmd.stdout(Stdio::piped()).stderr(Stdio::inherit());
    } else {
        cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    }

    let output = cmd.output().context("run shell command")?;
    if !output.status.success() {
        bail!("command failed: {command}");
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn load_env_file(path: &Path) -> Result<HashMap<String, String>> {
    let contents =
        fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    Ok(parse_env_contents(&contents))
}

fn parse_env_contents(contents: &str) -> HashMap<String, String> {
    let mut values = HashMap::new();
    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        values.insert(
            key.trim().to_owned(),
            value.trim().trim_matches('"').to_owned(),
        );
    }
    values
}

fn workspace_root() -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .nth(3)
        .map(PathBuf::from)
        .context("resolve workspace root")
}

fn shell_escape(path: &Path) -> String {
    let value = path.display().to_string();
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}
