use crate::{
    db::types::{DecryptionRequestDbMetadata, DecryptionResponseDbMetadata},
    decryption::BurstResult,
};
use alloy::primitives::U256;
use anyhow::anyhow;
use sqlx::{Pool, Postgres};
use std::{fmt::Display, time::Duration};
use tokio::{sync::mpsc::UnboundedReceiver, time::interval};
use tracing::{debug, trace};

const RESPONSE_POLLING: Duration = Duration::from_millis(500);

pub struct ResponseTracker {
    name: String,
    request_receiver: UnboundedReceiver<Vec<DecryptionRequestDbMetadata>>,
    db_pool: Pool<Postgres>,
}

impl ResponseTracker {
    pub fn new(
        name: String,
        request_receiver: UnboundedReceiver<Vec<DecryptionRequestDbMetadata>>,
        db_pool: Pool<Postgres>,
    ) -> Self {
        Self {
            name,
            request_receiver,
            db_pool,
        }
    }

    #[tracing::instrument(fields(self = %self))]
    pub async fn wait_responses_of_next_burst(&mut self) -> anyhow::Result<BurstResult> {
        let requests = match self.request_receiver.recv().await {
            None => return Err(anyhow!("Request receiver channel was closed unexpectedly")),
            Some(requests) => requests,
        };

        let mut responses = Vec::new();
        if requests.is_empty() {
            return Err(anyhow!("Empty request burst"));
        }

        let burst_start = requests.iter().map(|r| r.created_at).min().unwrap();
        let mut interval = interval(RESPONSE_POLLING);
        let mut ids_to_process: Vec<Vec<u8>> = requests
            .into_iter()
            .map(|r| r.id.to_le_bytes_vec())
            .collect();

        while !ids_to_process.is_empty() {
            interval.tick().await;

            let resps = self.fetch_responses_with_ids(&ids_to_process).await?;
            ids_to_process.retain(|id| !resps.iter().any(|r| r.id.to_le_bytes_vec() == *id));

            responses.extend(resps);
        }

        let burst_end = responses.iter().map(|r| r.created_at).max().unwrap();
        let latency = (burst_end - burst_start).as_seconds_f64();

        let result = BurstResult {
            latency,
            throughput: responses.len() as f64 / latency,
        };
        debug!(
            latency = result.latency,
            throughput = result.throughput,
            "Burst successfully processed!"
        );

        Ok(result)
    }

    async fn fetch_responses_with_ids(
        &self,
        ids: &[Vec<u8>],
    ) -> anyhow::Result<Vec<DecryptionResponseDbMetadata>> {
        trace!("Fetching responses for {} ids", ids.len());

        let mut responses = Vec::new();
        let public_rows = sqlx::query!(
                "SELECT decryption_id, created_at FROM public_decryption_responses WHERE decryption_id = ANY($1::bytea[])",
                ids,
            )
            .fetch_all(&self.db_pool)
            .await?;
        for row in public_rows {
            responses.push(DecryptionResponseDbMetadata {
                id: U256::from_le_slice(&row.decryption_id),
                created_at: row.created_at,
            });
        }

        let user_rows = sqlx::query!(
                "SELECT decryption_id, created_at FROM user_decryption_responses WHERE decryption_id = ANY($1::bytea[])",
                ids,
            )
            .fetch_all(&self.db_pool)
            .await?;
        for row in user_rows {
            responses.push(DecryptionResponseDbMetadata {
                id: U256::from_le_slice(&row.decryption_id),
                created_at: row.created_at,
            });
        }

        trace!("Fetched {} responses", responses.len());
        Ok(responses)
    }
}

impl Display for ResponseTracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResponseTracker {}", self.name)
    }
}
