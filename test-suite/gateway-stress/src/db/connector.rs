use crate::{
    config::DatabaseConfig,
    db::types::{DecryptionRequestDbMetadata, SnsCiphertextMaterialDbItem},
    decryption::types::DecryptionRequest,
};
use fhevm_gateway_bindings::decryption::Decryption::{
    PublicDecryptionRequest, UserDecryptionRequest,
};
use sqlx::{Executor, Pool, Postgres, QueryBuilder, postgres::PgPoolOptions};
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

        for request in requests {
            match request {
                DecryptionRequest::Public(r) => public_decryptions.push(r),
                DecryptionRequest::User(r) => user_decryptions.push(r),
            }
        }

        let mut inserted_requests = vec![];
        if !public_decryptions.is_empty() {
            inserted_requests.extend(self.insert_public_requests(public_decryptions).await?);
        }
        if !user_decryptions.is_empty() {
            inserted_requests.extend(self.insert_user_requests(user_decryptions).await?);
        }
        self.request_sender.send(inserted_requests)?;

        Ok(())
    }

    async fn insert_public_requests(
        &self,
        requests: Vec<PublicDecryptionRequest>,
    ) -> anyhow::Result<Vec<DecryptionRequestDbMetadata>> {
        let mut requests_metadata = vec![];

        for reqs in requests.chunks(self.insertion_chunk_size) {
            let mut query_builder = QueryBuilder::new(
                "INSERT INTO public_decryption_requests(decryption_id, sns_ct_materials, extra_data) ",
            );
            query_builder.push_values(reqs, |mut bind, req| {
                bind.push_bind(req.decryptionId.to_le_bytes_vec())
                    .push_bind(
                        req.snsCtMaterials
                            .iter()
                            .map(SnsCiphertextMaterialDbItem::from)
                            .collect::<Vec<_>>(),
                    )
                    .push_bind(req.extraData.to_vec());
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

        for reqs in requests.chunks(self.insertion_chunk_size) {
            let mut query_builder = QueryBuilder::new("
                INSERT INTO user_decryption_requests(decryption_id, sns_ct_materials, user_address, public_key, extra_data)
            ");
            query_builder.push_values(reqs, |mut bind, req| {
                bind.push_bind(req.decryptionId.to_le_bytes_vec())
                    .push_bind(
                        req.snsCtMaterials
                            .iter()
                            .map(SnsCiphertextMaterialDbItem::from)
                            .collect::<Vec<_>>(),
                    )
                    .push_bind(req.userAddress.to_vec())
                    .push_bind(req.publicKey.to_vec())
                    .push_bind(req.extraData.to_vec());
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
}

impl Display for DbConnector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DbConnector {}", self.name)
    }
}
