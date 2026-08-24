use std::time::Duration;

use thiserror::Error;
use tracing::{error, info};

use primitives::utils::saturating_u64_to_i64;

use crate::config::config::CleanerConfig;
use crate::store::repositories::BlockRepository;

/// Errors surfaced by the clean-blocks handler (the cleanup itself swallows
/// DB errors and skips the iteration).
#[derive(Error, Debug)]
pub enum CleanerError {
    #[error("Broker publish error: {message}")]
    BrokerPublishError { message: String },
    #[error("Advisory lock error: {message}")]
    AdvisoryLockError { message: String },
}

#[derive(Clone)]
pub struct Cleaner {
    blocks: BlockRepository,
    active: bool,
    blocks_to_keep: u64,
    cron_secs: u64,
}

impl Cleaner {
    pub fn new(blocks: BlockRepository, config: &CleanerConfig) -> Self {
        Self {
            blocks,
            active: config.active,
            blocks_to_keep: config.blocks_to_keep,
            cron_secs: config.cron_secs,
        }
    }

    /// Run one cleanup iteration: delete old blocks, then wait `cron_secs`.
    ///
    /// Returns whether the loop should be rescheduled (`false` when the
    /// cleaner is inactive — the loop deliberately ends). The next-iteration
    /// publish lives in [`CleanerHandler`](crate::core::workers::CleanerHandler)
    /// so it can happen after the flow lock is released.
    pub async fn run(&self) -> bool {
        if !self.active {
            info!("Cleaner: inactive — skipping cleanup and not re-triggering");
            return false;
        }

        match self
            .blocks
            .delete_blocks_keeping_latest(saturating_u64_to_i64(self.blocks_to_keep))
            .await
        {
            Ok(deleted) => {
                if deleted > 0 {
                    match self.blocks.get_min_block_number().await {
                        Ok(Some(min_block)) => {
                            info!(
                                deleted,
                                min_block_kept = min_block,
                                blocks_to_keep = self.blocks_to_keep,
                                "Cleaner: removed {deleted} blocks, blocks below {min_block} were deleted"
                            );
                        }
                        _ => {
                            info!(
                                deleted,
                                blocks_to_keep = self.blocks_to_keep,
                                "Cleaner: removed {deleted} blocks"
                            );
                        }
                    }
                } else {
                    info!(
                        blocks_to_keep = self.blocks_to_keep,
                        "Cleaner: no blocks to clean up"
                    );
                }
            }
            Err(e) => {
                error!(
                    error = %e,
                    blocks_to_keep = self.blocks_to_keep,
                    "Cleaner: failed to delete old blocks, skipping this iteration"
                );
            }
        }

        tokio::time::sleep(Duration::from_secs(self.cron_secs)).await;

        true
    }
}
