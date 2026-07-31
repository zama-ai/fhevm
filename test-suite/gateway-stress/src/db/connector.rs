use crate::{
    config::DatabaseConfig, db::types::DecryptionRequestDbMetadata,
    decryption::types::DecryptionRequest,
};
use anyhow::anyhow;
use fhevm_gateway_bindings::decryption::Decryption::{
    PublicDecryptionRequest_1 as PublicDecryptionRequest,
    UserDecryptionRequest_2 as UserDecryptionRequest,
    UserDecryptionRequest_3 as UserDecryptionRequestV2,
};
use sqlx::{
    Executor, Pool, Postgres, QueryBuilder, postgres::PgPoolOptions, types::time::OffsetDateTime,
};
use std::fmt::Display;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, trace};

#[derive(Clone)]
pub struct DbConnector {
    pub name: String,
    pub db_pool: Pool<Postgres>,
    pub insertion_chunk_size: usize,
    pub request_sender: UnboundedSender<Vec<DecryptionRequestDbMetadata>>,
}

impl DbConnector {
    pub async fn connect(
        db_config: &DatabaseConfig,
        index: usize,
        request_sender: UnboundedSender<Vec<DecryptionRequestDbMetadata>>,
    ) -> anyhow::Result<Self> {
        let url = &db_config.urls[index];
        let name = url
            .split('@')
            .next_back()
            .unwrap_or(&format!("connector_{index}"))
            .to_string();

        debug!("Connecting to database #{index} ({name})");
        let db_pool = PgPoolOptions::new()
            .max_connections(db_config.pool_size)
            .acquire_timeout(db_config.connection_timeout)
            .connect(url)
            .await?;
        debug!("Successfully connected to database #{index} ({name})");

        Ok(Self {
            name,
            db_pool,
            insertion_chunk_size: db_config.insertion_chunk_size,
            request_sender,
        })
    }

    pub async fn health_check(&self) -> anyhow::Result<()> {
        sqlx::query!("SELECT 1 AS health")
            .fetch_one(&self.db_pool)
            .await?;
        Ok(())
    }

    pub async fn clear_tables(&self) -> anyhow::Result<()> {
        debug!("Clearing database tables for connector {}", self.name);

        sqlx::query!("DELETE FROM public_decryption_requests")
            .execute(&self.db_pool)
            .await?;
        sqlx::query!("DELETE FROM user_decryption_requests")
            .execute(&self.db_pool)
            .await?;
        sqlx::query!("DELETE FROM public_decryption_responses")
            .execute(&self.db_pool)
            .await?;
        sqlx::query!("DELETE FROM user_decryption_responses")
            .execute(&self.db_pool)
            .await?;

        debug!("Database tables cleared for connector {}", self.name);
        Ok(())
    }

    #[tracing::instrument(fields(self = %self))]
    pub async fn insert_requests(&self, requests: Vec<DecryptionRequest>) -> anyhow::Result<()> {
        let mut public_decryptions = vec![];
        let mut user_decryptions = vec![];
        let mut user_v2_decryptions = vec![];

        for request in requests {
            match request {
                DecryptionRequest::Public(r) => public_decryptions.push(r),
                DecryptionRequest::User(r) => user_decryptions.push(r),
                DecryptionRequest::UserV2(r) => user_v2_decryptions.push(r),
            }
        }

        let mut inserted_requests = vec![];
        if !public_decryptions.is_empty() {
            inserted_requests.extend(self.insert_public_requests(public_decryptions).await?);
        }
        if !user_decryptions.is_empty() {
            inserted_requests.extend(self.insert_user_requests(user_decryptions).await?);
        }
        if !user_v2_decryptions.is_empty() {
            inserted_requests.extend(self.insert_user_v2_requests(user_v2_decryptions).await?);
        }
        self.request_sender.send(inserted_requests)?;

        Ok(())
    }

    async fn insert_public_requests(
        &self,
        requests: Vec<PublicDecryptionRequest>,
    ) -> anyhow::Result<Vec<DecryptionRequestDbMetadata>> {
        let mut requests_metadata = vec![];

        let created_at = OffsetDateTime::now_utc();
        for reqs in requests.chunks(self.insertion_chunk_size) {
            let mut query_builder = QueryBuilder::new(
                "INSERT INTO public_decryption_requests(
                    decryption_id, ct_handles, extra_data, otlp_context, created_at
                ) ",
            );
            query_builder.push_values(reqs, |mut bind, req| {
                bind.push_bind(req.decryptionId.to_le_bytes_vec())
                    .push_bind(req.ctHandles.iter().map(|h| h.to_vec()).collect::<Vec<_>>())
                    .push_bind(req.extraData.to_vec())
                    .push_bind(alloy::hex::decode(EMPTY_OTLP_CONTEXT_SERIALIZED_HEX).unwrap())
                    .push_bind(created_at);
            });
            query_builder.push(" RETURNING decryption_id, created_at");

            let query = query_builder.build();
            let query_results = self.db_pool.fetch_all(query).await?;
            requests_metadata.extend(query_results.into_iter().map(|r| r.into()));
        }

        trace!(
            "Inserted {} public decryption requests",
            requests_metadata.len(),
        );
        Ok(requests_metadata)
    }

    async fn insert_user_requests(
        &self,
        requests: Vec<UserDecryptionRequest>,
    ) -> anyhow::Result<Vec<DecryptionRequestDbMetadata>> {
        let mut requests_metadata = vec![];

        let created_at = OffsetDateTime::now_utc();
        for reqs in requests.chunks(self.insertion_chunk_size) {
            let mut query_builder = QueryBuilder::new(
                "INSERT INTO user_decryption_requests(
                    decryption_id, ct_handles, user_address, public_key, extra_data, otlp_context, created_at
                ) ");
            query_builder.push_values(reqs, |mut bind, req| {
                bind.push_bind(req.decryptionId.to_le_bytes_vec())
                    .push_bind(req.ctHandles.iter().map(|h| h.to_vec()).collect::<Vec<_>>())
                    .push_bind(req.userAddress.to_vec())
                    .push_bind(req.publicKey.to_vec())
                    .push_bind(req.extraData.to_vec())
                    .push_bind(alloy::hex::decode(EMPTY_OTLP_CONTEXT_SERIALIZED_HEX).unwrap())
                    .push_bind(created_at);
            });
            query_builder.push(" RETURNING decryption_id, created_at");

            let query = query_builder.build();
            let query_results = self.db_pool.fetch_all(query).await?;
            requests_metadata.extend(query_results.into_iter().map(|r| r.into()));
        }

        trace!(
            "Inserted {} user decryption requests",
            requests_metadata.len(),
        );
        Ok(requests_metadata)
    }

    async fn insert_user_v2_requests(
        &self,
        requests: Vec<UserDecryptionRequestV2>,
    ) -> anyhow::Result<Vec<DecryptionRequestDbMetadata>> {
        let mut requests_metadata = vec![];

        for reqs in requests.chunks(self.insertion_chunk_size) {
            let prepared = reqs
                .iter()
                .map(|req| {
                    let start: i64 = req
                        .payload
                        .requestValidity
                        .startTimestamp
                        .try_into()
                        .map_err(|_| anyhow!("RFC016 startTimestamp does not fit in i64"))?;
                    let duration: i64 = req
                        .payload
                        .requestValidity
                        .durationSeconds
                        .try_into()
                        .map_err(|_| anyhow!("RFC016 durationSeconds does not fit in i64"))?;
                    Ok::<_, anyhow::Error>((req, start, duration))
                })
                .collect::<Result<Vec<_>, _>>()?;

            let created_at = OffsetDateTime::now_utc();
            let mut query_builder = QueryBuilder::new(
                "INSERT INTO user_decryption_requests(
                    decryption_id, ct_handles, user_address, public_key, extra_data, otlp_context,
                    handle_owner_addresses, handle_contract_addresses, allowed_contracts,
                    start_timestamp, duration_seconds, signature, created_at
                )
            ",
            );
            query_builder.push_values(prepared, |mut bind, (req, start, duration)| {
                let payload = &req.payload;
                bind.push_bind(req.decryptionId.to_le_bytes_vec())
                    .push_bind(
                        req.handles
                            .iter()
                            .map(|h| h.handle.to_vec())
                            .collect::<Vec<_>>(),
                    )
                    .push_bind(payload.userAddress.to_vec())
                    .push_bind(payload.publicKey.to_vec())
                    .push_bind(payload.extraData.to_vec())
                    .push_bind(alloy::hex::decode(EMPTY_OTLP_CONTEXT_SERIALIZED_HEX).unwrap())
                    .push_bind(
                        req.handles
                            .iter()
                            .map(|h| h.ownerAddress.to_vec())
                            .collect::<Vec<_>>(),
                    )
                    .push_bind(
                        req.handles
                            .iter()
                            .map(|h| h.contractAddress.to_vec())
                            .collect::<Vec<_>>(),
                    )
                    .push_bind(
                        payload
                            .allowedContracts
                            .iter()
                            .map(|c| c.to_vec())
                            .collect::<Vec<_>>(),
                    )
                    .push_bind(start)
                    .push_bind(duration)
                    .push_bind(payload.signature.to_vec())
                    .push_bind(created_at);
            });
            query_builder.push(" RETURNING decryption_id, created_at");

            let query = query_builder.build();
            let query_results = self.db_pool.fetch_all(query).await?;
            requests_metadata.extend(query_results.into_iter().map(|r| r.into()));
        }

        trace!(
            "Inserted {} user decryption V2 requests",
            requests_metadata.len(),
        );
        Ok(requests_metadata)
    }
}

impl Display for DbConnector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DbConnector {}", self.name)
    }
}

const EMPTY_OTLP_CONTEXT_SERIALIZED_HEX: &str = "0000000000000000";
