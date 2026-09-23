use std::time::Duration;

use thiserror::Error;
use tracing::{error, info};

use primitives::utils::saturating_u64_to_i64;

use crate::config::config::BlockchainConfig;
use crate::store::repositories::{BlockRepository, FinalBlockRepository};

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
    final_blocks: FinalBlockRepository,
    active: bool,
    /// `cleaner.active && finality_active`: the final-blocks cleaning only
    /// runs when the finality flow itself is enabled, so a disabled finality
    /// flow never generates per-chain cron queries.
    final_active: bool,
    blocks_to_keep: u64,
    cron_secs: u64,
}

impl Cleaner {
    pub fn new(
        blocks: BlockRepository,
        final_blocks: FinalBlockRepository,
        blockchain: &BlockchainConfig,
    ) -> Self {
        Self {
            blocks,
            final_blocks,
            active: blockchain.cleaner.active,
            final_active: blockchain.cleaner.active && blockchain.finality_active,
            blocks_to_keep: blockchain.cleaner.blocks_to_keep,
            cron_secs: blockchain.cleaner.cron_secs,
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

    /// Run one final-blocks cleanup iteration: delete old final blocks, then
    /// wait `cron_secs`.
    ///
    /// Returns whether the loop should be rescheduled (`false` when the
    /// cleaner or the finality flow is inactive — the loop deliberately
    /// ends). The next-iteration publish lives in
    /// [`FinalCleanerHandler`](crate::core::workers::FinalCleanerHandler)
    /// so it can happen after the flow lock is released.
    ///
    /// All deletion queries are chain-scoped: the repository bakes the
    /// chain_id in at construction, so one chain's cleaner never touches
    /// another chain's final blocks.
    pub async fn run_final(&self) -> bool {
        if !self.final_active {
            info!("FinalCleaner: inactive — skipping cleanup and not re-triggering");
            return false;
        }

        match self
            .final_blocks
            .delete_blocks_keeping_latest(saturating_u64_to_i64(self.blocks_to_keep))
            .await
        {
            Ok(deleted) => {
                if deleted > 0 {
                    match self.final_blocks.get_min_block_number().await {
                        Ok(Some(min_block)) => {
                            info!(
                                deleted,
                                min_block_kept = min_block,
                                blocks_to_keep = self.blocks_to_keep,
                                "FinalCleaner: removed {deleted} final blocks, blocks below {min_block} were deleted"
                            );
                        }
                        _ => {
                            info!(
                                deleted,
                                blocks_to_keep = self.blocks_to_keep,
                                "FinalCleaner: removed {deleted} final blocks"
                            );
                        }
                    }
                } else {
                    info!(
                        blocks_to_keep = self.blocks_to_keep,
                        "FinalCleaner: no final blocks to clean up"
                    );
                }
            }
            Err(e) => {
                error!(
                    error = %e,
                    blocks_to_keep = self.blocks_to_keep,
                    "FinalCleaner: failed to delete old final blocks, skipping this iteration"
                );
            }
        }

        tokio::time::sleep(Duration::from_secs(self.cron_secs)).await;

        true
    }
}
