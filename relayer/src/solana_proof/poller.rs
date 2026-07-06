//! Background poll loop wiring `ingest::poll_once` on a timer, seeding the
//! cursor from config on first run.

use std::sync::Arc;
use std::time::Duration;

use tracing::{error, info};

use crate::solana_proof::chain::ChainFetcher;
use crate::solana_proof::config::SolanaProofConfig;
use crate::solana_proof::ingest::poll_once;
use crate::solana_proof::store::{Cursor, LeafStore};

/// Runs `ingest::poll_once` on a `poll_interval_secs` timer until the process
/// exits. Intended to be spawned once at startup (see `startup.rs` for the
/// analogous pattern used by the gateway's listener pool); not itself spawned
/// here since this module does not own the relayer's task-spawning conventions.
pub async fn run_poll_loop<C: ChainFetcher, S: LeafStore>(
    fetcher: Arc<C>,
    store: Arc<S>,
    config: SolanaProofConfig,
) {
    let program_id = match config.program_id_bytes() {
        Ok(id) => id,
        Err(e) => {
            error!("solana_proof: invalid program_id, poll loop not started: {e}");
            return;
        }
    };

    // Seed the cursor from config on first run so ingestion doesn't have to
    // scan from the program's genesis signature every time.
    if store.get_cursor().await.ok().flatten().is_none() {
        if let Some(start_signature) = &config.start_signature {
            if let Err(e) = store
                .set_cursor(Cursor {
                    last_signature: Some(start_signature.clone()),
                    last_slot: 0,
                })
                .await
            {
                error!("solana_proof: failed to seed poll cursor: {e}");
                return;
            }
        }
    }

    let interval = Duration::from_secs(config.poll_interval_secs);
    info!(
        poll_interval_secs = config.poll_interval_secs,
        poll_signature_limit = config.poll_signature_limit,
        "solana_proof: poll loop started"
    );

    loop {
        match poll_once(
            fetcher.as_ref(),
            store.as_ref(),
            program_id,
            config.poll_signature_limit,
        )
        .await
        {
            Ok(processed) if processed > 0 => {
                info!("solana_proof: ingested {processed} new transaction(s)");
            }
            Ok(_) => {}
            Err(e) => error!("solana_proof: poll cycle failed: {e}"),
        }
        tokio::time::sleep(interval).await;
    }
}
