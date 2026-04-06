use anyhow::{Context, Result, bail};
use serde_json::Value;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::{Child, Command, Output, Stdio},
    time::{Duration, Instant},
};

const DEFAULT_RPC_URL: &str = "http://127.0.0.1:18999";
const LOCALNET_TIMEOUT: Duration = Duration::from_secs(180);
const SOLANA_HOST_PROGRAM_MANIFEST: &str =
    "solana-host-contracts/programs/solana-host-contracts/Cargo.toml";
const SOLANA_TEST_INPUT_PROGRAM_MANIFEST: &str =
    "solana-host-contracts/programs/solana-test-input-program/Cargo.toml";
const SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_MANIFEST: &str =
    "solana-host-contracts/programs/solana-confidential-token-program/Cargo.toml";
const SOLANA_LOCAL_CLI_MANIFEST: &str = "solana-host-contracts/local-cli/Cargo.toml";

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
            bootstrap_localnet(&repo_root).context("bootstrap existing Solana localnet")?;

            wait_for_localnet(&addresses_env).await?;

            return Ok(Self {
                repo_root,
                addresses_env,
                process: None,
            });
        }

        ensure_programs_built(&repo_root).context("build Solana SBF programs")?;

        let mut child = Command::new("anchor")
            .arg("localnet")
            .arg("--skip-build")
            .env("NO_DNA", "1")
            .current_dir(&solana_host_root)
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
        let manifest_path = self.repo_root.join(SOLANA_LOCAL_CLI_MANIFEST);
        let mut command = Command::new("cargo");
        command
            .arg("run")
            .arg("--manifest-path")
            .arg(&manifest_path)
            .arg("--")
            .arg(scenario)
            .arg("--addresses-env")
            .arg(&self.addresses_env)
            .current_dir(&self.repo_root);
        let stdout = run_command_capture(command)
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

fn bootstrap_localnet(repo_root: &Path) -> Result<()> {
    let manifest_path = repo_root.join(SOLANA_LOCAL_CLI_MANIFEST);
    let mut command = Command::new("cargo");
    command
        .arg("run")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(manifest_path)
        .arg("--")
        .arg("bootstrap-localnet")
        .current_dir(repo_root);
    run_command(command).map(|_| ())
}

fn ensure_programs_built(repo_root: &Path) -> Result<()> {
    for manifest in [
        SOLANA_HOST_PROGRAM_MANIFEST,
        SOLANA_TEST_INPUT_PROGRAM_MANIFEST,
        SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_MANIFEST,
    ] {
        let manifest_path = repo_root.join(manifest);
        let output = Command::new("cargo")
            .arg("build-sbf")
            .arg("--manifest-path")
            .arg(&manifest_path)
            .current_dir(repo_root)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .with_context(|| format!("run cargo build-sbf for {}", manifest_path.display()))?;
        ensure_success(
            output,
            &format!("cargo build-sbf --manifest-path {}", manifest_path.display()),
        )?;
    }
    Ok(())
}

fn run_command(mut command: Command) -> Result<Output> {
    command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let output = command.output().context("run command")?;
    ensure_success(output, "command")
}

fn run_command_capture(mut command: Command) -> Result<String> {
    command.stdout(Stdio::piped()).stderr(Stdio::inherit());
    let output = command.output().context("run command")?;
    let output = ensure_success(output, "command")?;
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn ensure_success(output: Output, description: &str) -> Result<Output> {
    if !output.status.success() {
        bail!("{description} failed with status {}", output.status);
    }
    Ok(output)
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
