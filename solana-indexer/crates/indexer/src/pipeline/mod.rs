//! Carbon pipeline assembly: RPC-transaction-crawler datasource (finalized) ->
//! hand-written EV-ACL decoder -> reconstructing processor.
//!
//! The crawler is seeded with the persisted cursor's `last_signature` as its
//! `until` bound, so a restart resumes without reprocessing. Carbon persists
//! nothing itself; the resume point lives in `indexer_cursor`.

pub mod processor;

use std::str::FromStr;
use std::sync::Arc;

use carbon_core::pipeline::{Pipeline, ShutdownStrategy};
use carbon_rpc_transaction_crawler_datasource::{
    ConnectionConfig, Filters, RetryConfig, RpcTransactionCrawler,
};
use solana_pubkey::Pubkey;
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::config::settings::SolanaConfig;
use crate::decoder::EvAclDecoder;
use crate::metrics::Metrics;
use crate::pipeline::processor::EvAclProcessor;
use crate::store::repositories::lineage_repo::LineageRepo;

/// Builds and runs the Carbon pipeline until `cancel` fires. Resolves the program
/// id, seeds the crawler `until` from the DB cursor, and registers the decoder +
/// processor. The crawler crawls `getSignaturesForAddress(program_id)`.
pub async fn run(
    solana: &SolanaConfig,
    repo: LineageRepo,
    metrics: Arc<Metrics>,
    cancel: CancellationToken,
) -> anyhow::Result<()> {
    let program_id = Pubkey::from_str(&solana.program_id)
        .map_err(|e| anyhow::anyhow!("invalid program_id {}: {e}", solana.program_id))?;
    let commitment = solana.commitment_config();

    // Resume from the persisted cursor: its last_signature is the crawler's
    // `until` bound (stop crawling once we reach an already-processed signature).
    let (last_signature, last_slot) = repo.get_cursor().await?;
    let until_signature = if last_signature.is_empty() {
        None
    } else {
        solana_signature_from_str(&last_signature)
    };
    info!(
        program_id = %program_id,
        commitment = %solana.commitment,
        last_slot,
        resume = until_signature.is_some(),
        "starting Carbon pipeline"
    );

    let connection_config = ConnectionConfig::new(
        solana.backfill_batch,
        solana.poll_interval,
        // max_concurrent_requests = 1: serialize transaction fetching. The crawler emits
        // signatures and `buffer_unordered(max_concurrent_requests)`-fetches their transactions,
        // forwarding them in COMPLETION order. With >1 in flight a newer-slot transaction can be
        // processed (advancing the resume cursor) while an older one from the same page is still
        // pending; a crash there would persist a cursor past the unprocessed older transaction and
        // permanently skip it (the cursor's `until` bound never re-crawls behind itself). Serial
        // fetching removes that race — at most one transaction is in flight, so the cursor never
        // advances past an unprocessed sibling. (Re-processing on restart is idempotent: events are
        // UNIQUE(pda, event_index) and the lineage reconstruction is deterministic.)
        1,
        RetryConfig::default(),
        None,
        None,
        false,
    );
    let filters = Filters::new(None, None, until_signature);

    // SEAM: to move to a Geyser feed, swap this RpcTransactionCrawler for a
    // carbon_yellowstone_grpc_datasource::YellowstoneGrpcGeyserClient here; the
    // decoder + processor downstream are unchanged.
    let datasource = RpcTransactionCrawler::new(
        solana.rpc_url.clone(),
        program_id,
        connection_config,
        filters,
        Some(commitment),
    );

    let decoder = EvAclDecoder::new(program_id);
    let processor = EvAclProcessor::new(repo, metrics);

    let mut pipeline = Pipeline::builder()
        .datasource(datasource)
        .instruction(decoder, processor)
        .datasource_cancellation_token(cancel.clone())
        .shutdown_strategy(ShutdownStrategy::ProcessPending)
        .build()
        .map_err(|e| anyhow::anyhow!("failed to build Carbon pipeline: {e:?}"))?;

    pipeline
        .run()
        .await
        .map_err(|e| anyhow::anyhow!("Carbon pipeline error: {e:?}"))?;
    Ok(())
}

/// Parses a base58 signature string into the crawler's `Signature` type.
/// Returns `None` on a malformed cursor (treated as a fresh start).
fn solana_signature_from_str(raw: &str) -> Option<solana_signature::Signature> {
    solana_signature::Signature::from_str(raw).ok()
}
