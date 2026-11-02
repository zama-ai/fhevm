//! Development only: mock KMS connectors serving the relayer's temporary `POST /v1/pad-decrypt` from a cleartext
//! fhevm stack on anvil, and a flow driver for `POST /v4/pad-decrypt`. Start with `docs.md`.
//!
//! ```text
//! pad-mock connectors --rpc-url URL --padded-inputter ADDR --listen HOST:PORT --keys KEY,KEY,…
//! pad-mock flow --relayer-url URL --rpc-url URL --padded-inputter ADDR --kms-verifier ADDR
//!               --protocol-config ADDR --user-key KEY [--threshold N] [--extra-data HEX]
//! ```

mod chain;
mod connector;
mod flow;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::process::ExitCode;
use std::str::FromStr;

use alloy::primitives::Address;
use alloy::signers::local::PrivateKeySigner;

use crate::chain::Rpc;
use crate::connector::Node;
use crate::flow::FlowConfig;

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.split_first() {
        Some((command, rest)) if command == "connectors" => connectors(rest).await,
        Some((command, rest)) if command == "flow" => flow(rest).await,
        _ => Err("usage: pad-mock connectors|flow --flag value … (see main.rs)".to_owned()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("pad-mock: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Serves one node per key, on consecutive ports from `--listen`.
async fn connectors(args: &[String]) -> Result<(), String> {
    let flags = Flags::parse(args)?;
    let rpc = Rpc::new(flags.get("rpc-url")?);
    let padded_inputter: Address = flags.get("padded-inputter")?;
    let listen: SocketAddr = flags.get("listen")?;
    let keys = flags.raw("keys")?;
    let mut nodes = Vec::new();
    let mut addresses = Vec::new();
    for (i, key) in keys.split(',').enumerate() {
        let key =
            PrivateKeySigner::from_str(key.trim()).map_err(|e| format!("--keys[{i}]: {e}"))?;
        let port = u16::try_from(usize::from(listen.port()) + i).map_err(|_| "port overflow")?;
        nodes.push(Node {
            name: format!("kms_{}", i + 1),
            key,
            rpc: rpc.clone(),
            padded_inputter,
        });
        addresses.push(SocketAddr::new(listen.ip(), port));
    }
    connector::serve(nodes, addresses)
        .await
        .map_err(|e| e.to_string())
}

/// One pad decryption through the relayer, checked.
async fn flow(args: &[String]) -> Result<(), String> {
    let flags = Flags::parse(args)?;
    let threshold: usize = flags.get_or("threshold", 3)?;
    if threshold == 0 {
        return Err("--threshold must be at least 1".to_owned());
    }
    let config = FlowConfig {
        relayer: flags.get("relayer-url")?,
        rpc: Rpc::new(flags.get("rpc-url")?),
        padded_inputter: flags.get("padded-inputter")?,
        kms_verifier: flags.get("kms-verifier")?,
        protocol_config: flags.get("protocol-config")?,
        user: PrivateKeySigner::from_str(flags.raw("user-key")?)
            .map_err(|e| format!("--user-key: {e}"))?,
        extra_data: flags.get_or("extra-data", chain::default_extra_data())?,
        threshold,
    };
    flow::run(&config).await
}

/// `--name value` pairs.
struct Flags(HashMap<String, String>);

impl Flags {
    fn parse(args: &[String]) -> Result<Self, String> {
        let mut map = HashMap::new();
        let mut it = args.iter();
        while let Some(flag) = it.next() {
            let name = flag
                .strip_prefix("--")
                .ok_or_else(|| format!("expected --flag, got {flag}"))?;
            let value = it.next().ok_or_else(|| format!("--{name} needs a value"))?;
            map.insert(name.to_owned(), value.clone());
        }
        Ok(Self(map))
    }

    fn raw(&self, name: &str) -> Result<&str, String> {
        self.0
            .get(name)
            .map(String::as_str)
            .ok_or_else(|| format!("--{name} is required"))
    }

    fn get<T: FromStr>(&self, name: &str) -> Result<T, String>
    where
        T::Err: std::fmt::Display,
    {
        self.raw(name)?
            .parse()
            .map_err(|e| format!("--{name}: {e}"))
    }

    fn get_or<T: FromStr>(&self, name: &str, default: T) -> Result<T, String>
    where
        T::Err: std::fmt::Display,
    {
        if self.0.contains_key(name) {
            self.get(name)
        } else {
            Ok(default)
        }
    }
}

#[cfg(test)]
mod tests {
    use alloy::primitives::Bytes;

    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn flags_parse_values_defaults_and_errors() {
        let flags = Flags::parse(&args(&["--threshold", "5", "--extra-data", "0x02"])).unwrap();
        assert_eq!(flags.get::<usize>("threshold").unwrap(), 5);
        assert_eq!(flags.get_or::<usize>("missing", 3).unwrap(), 3);
        assert_eq!(flags.get::<Bytes>("extra-data").unwrap().as_ref(), &[2]);
        assert!(
            flags
                .raw("missing")
                .unwrap_err()
                .contains("--missing is required")
        );
        assert!(flags.get::<usize>("extra-data").is_err());
        assert!(Flags::parse(&args(&["threshold", "5"])).is_err());
        assert!(Flags::parse(&args(&["--threshold"])).is_err());
    }
}
