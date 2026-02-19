use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use tokio::time::sleep;
use tracing::info;

use crate::contracts::FinalizedEventEnvelope;
use crate::database::ingest::map_envelope_to_actions;
use crate::database::solana_event_propagate::Database;

pub mod solana_rpc_source;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cursor {
    pub slot: u64,
    pub tx_index: u32,
    pub op_index: u16,
}

#[derive(Clone, Debug)]
pub struct PollerConfig {
    pub poll_interval: Duration,
    pub max_batch_size: usize,
    pub finalized_only: bool,
}

#[derive(Clone, Debug)]
pub struct SourceBatch {
    pub events: Vec<FinalizedEventEnvelope>,
    pub next_cursor: Cursor,
}

impl SourceBatch {
    pub fn empty(cursor: Cursor) -> Self {
        Self {
            events: Vec::new(),
            next_cursor: cursor,
        }
    }
}

#[async_trait]
pub trait EventSource {
    async fn next_batch(
        &mut self,
        cursor: Cursor,
        max_batch_size: usize,
        finalized_only: bool,
    ) -> Result<SourceBatch>;
}

fn cursor_is_ahead(a: Cursor, b: Cursor) -> bool {
    (a.slot, a.tx_index, a.op_index) > (b.slot, b.tx_index, b.op_index)
}

pub async fn run_poller<S: EventSource>(
    config: &PollerConfig,
    source: &mut S,
    store: &mut Database,
    cursor: &mut Cursor,
) -> Result<()> {
    loop {
        let source_batch = source
            .next_batch(*cursor, config.max_batch_size, config.finalized_only)
            .await?;

        if source_batch.events.is_empty() {
            if cursor_is_ahead(source_batch.next_cursor, *cursor) {
                store
                    .set_cursor(source_batch.next_cursor.slot as i64)
                    .await?;
                *cursor = source_batch.next_cursor;
                info!(slot = cursor.slot, "advanced poller cursor without events");
            }
            sleep(config.poll_interval).await;
            continue;
        }

        for envelope in source_batch.events {
            let actions = map_envelope_to_actions(&envelope, store.tenant_id)?;
            let stats = store.apply_actions(&actions).await?;
            info!(
                slot = envelope.slot,
                tx_index = envelope.tx_index,
                op_index = envelope.op_index,
                ?stats,
                "applied finalized solana envelope"
            );
            let envelope_cursor = Cursor {
                slot: envelope.slot,
                tx_index: envelope.tx_index,
                op_index: envelope.op_index,
            };
            if cursor_is_ahead(envelope_cursor, *cursor) {
                *cursor = envelope_cursor;
            }
        }

        if cursor_is_ahead(source_batch.next_cursor, *cursor) {
            store
                .set_cursor(source_batch.next_cursor.slot as i64)
                .await?;
            *cursor = source_batch.next_cursor;
            info!(
                slot = cursor.slot,
                "advanced poller cursor at batch watermark"
            );
        }
    }
}
