use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use tracing::info;

use crate::{host::host_chain_id, FollowerConfig, SOLANA_RPC_REQUEST_TIMEOUT};

/// The flags of a binary that follows zama-host on a Solana cluster.
#[derive(clap::Args, Debug, Clone)]
pub struct FollowerArgs {
    /// Solana JSON-RPC endpoint of the live cluster, read at confirmed commitment: HostConfig and
    /// the confirmed slot.
    #[arg(long, default_value = "http://127.0.0.1:8899")]
    url: String,

    /// Yellowstone gRPC endpoint.
    #[arg(long, default_value = "http://127.0.0.1:10000")]
    grpc_url: String,

    /// Solana JSON-RPC endpoint whose ledger history rebuilds, with `getBlock` and
    /// `getTransaction`, the slots Yellowstone can no longer replay, and reads the start block's
    /// hash. It may be another provider's. Defaults to `--url`.
    #[arg(long, env = "SOLANA_ARCHIVE_URL")]
    archive_url: Option<String>,

    /// Optional `x-token` auth metadata for the gRPC endpoint.
    #[arg(long, env = "SOLANA_GRPC_X_TOKEN")]
    grpc_x_token: Option<String>,

    /// `zama-host` program id to follow.
    #[arg(long)]
    program_id: Pubkey,
}

/// What a follower runs with: its configuration and the clients of the live and archive endpoints.
pub struct Follower {
    pub config: FollowerConfig,
    pub live: RpcClient,
    pub archive: RpcClient,
}

impl FollowerArgs {
    /// A client of the live endpoint at confirmed commitment.
    pub fn live_rpc(&self) -> RpcClient {
        RpcClient::new_with_timeout_and_commitment(
            self.url.clone(),
            SOLANA_RPC_REQUEST_TIMEOUT,
            CommitmentConfig::confirmed(),
        )
    }

    /// Reads the chain id from the deployment's confirmed HostConfig. It is the one source of the
    /// chain id: handles are derived with it, so nothing derived from the follower can disagree.
    pub async fn connect(&self) -> Result<Follower> {
        let live = self.live_rpc();
        let chain_id = host_chain_id(&live, &self.program_id).await?;
        info!(chain_id, "read the chain id from the confirmed HostConfig");
        let archive = RpcClient::new_with_timeout(
            self.archive_url.clone().unwrap_or_else(|| self.url.clone()),
            SOLANA_RPC_REQUEST_TIMEOUT,
        );
        Ok(Follower {
            config: FollowerConfig {
                grpc_url: self.grpc_url.clone(),
                x_token: self.grpc_x_token.clone(),
                program_id: self.program_id,
                chain_id,
            },
            live,
            archive,
        })
    }
}
