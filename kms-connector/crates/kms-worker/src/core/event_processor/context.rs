use crate::core::{
    config::Config,
    event_processor::{RequestCheckError, RequestCheckKind},
};
use alloy::{primitives::U256, providers::Provider};
use anyhow::anyhow;
use connector_utils::types::extra_data::ExtraData;
use fhevm_host_bindings::protocol_config::ProtocolConfig::{self, ProtocolConfigInstance};
use sqlx::{Pool, Postgres, types::chrono::Utc};
use tracing::{info, warn};

pub trait ContextManager: Send + Sync {
    /// Validates the KMS context and epoch referenced by a request's parsed `extra_data`.
    fn validate_context(
        &self,
        extra_data: &ExtraData,
    ) -> impl Future<Output = Result<(), RequestCheckError>> + Send;
}

/// Outcome of the lookup in the local `kms_context` table.
enum LocalCheck {
    Valid,
    Destroyed,
    Unknown,
}

/// Validates KMS contexts and epochs against the local `kms_context` table, falling back to
/// `ProtocolConfig` on Ethereum for pairs not cached locally.
#[derive(Clone)]
pub struct DbContextManager<P> {
    /// The database pool used to query the `kms_context` table.
    db_pool: Pool<Postgres>,

    /// The `ProtocolConfig` contract instance on Ethereum, source of truth for context and
    /// epoch validity.
    protocol_config_contract: ProtocolConfigInstance<P>,
}

impl<P: Provider> ContextManager for DbContextManager<P> {
    #[tracing::instrument(skip(self))]
    async fn validate_context(&self, extra_data: &ExtraData) -> Result<(), RequestCheckError> {
        let Some(context_id) = extra_data.context_id else {
            // Accepting request with no context for backwards compatibility with the relayer-sdk.
            // TODO: Remove once https://github.com/zama-ai/fhevm-internal/issues/1506 is resolved.
            return Ok(());
        };
        let epoch_id = extra_data.epoch_id;

        match self.check_local_db(context_id, epoch_id).await? {
            LocalCheck::Valid => Ok(()),
            LocalCheck::Destroyed => Err(RequestCheckError::irrecoverable(
                RequestCheckKind::KmsContext,
                anyhow!("Context #{context_id} has been destroyed"),
            )),
            LocalCheck::Unknown => self.validate_on_chain(context_id, epoch_id).await,
        }
    }
}

impl<P: Provider> DbContextManager<P> {
    pub fn new(db_pool: Pool<Postgres>, config: &Config, ethereum_provider: P) -> Self {
        let protocol_config_contract =
            ProtocolConfig::new(config.protocol_config_contract.address, ethereum_provider);
        Self {
            db_pool,
            protocol_config_contract,
        }
    }

    /// Checks the requested context and epoch against the local `kms_context` table.
    async fn check_local_db(
        &self,
        context_id: U256,
        epoch_id: Option<U256>,
    ) -> Result<LocalCheck, RequestCheckError> {
        let rows = sqlx::query!(
            "SELECT epoch_id, is_valid FROM kms_context WHERE id = $1",
            context_id.as_le_slice()
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| {
            RequestCheckError::network(anyhow!("Query to check context #{context_id} failed: {e}"))
        })?;

        // `is_valid = false` is only ever written by `KmsContextDestroyed`, which invalidates the
        // whole context at once.
        if rows.iter().any(|r| !r.is_valid) {
            return Ok(LocalCheck::Destroyed);
        }

        let known = match epoch_id {
            // v1 extra_data carries no epoch: any valid row proves the context exists
            None => !rows.is_empty(),
            Some(epoch_id) => rows.iter().any(|r| r.epoch_id == epoch_id.as_le_slice()),
        };
        if known {
            Ok(LocalCheck::Valid)
        } else {
            Ok(LocalCheck::Unknown)
        }
    }

    /// Validates a pair not cached locally against `ProtocolConfig`, caching it on success.
    async fn validate_on_chain(
        &self,
        context_id: U256,
        epoch_id: Option<U256>,
    ) -> Result<(), RequestCheckError> {
        info!("Context not found in DB, validating against ProtocolConfig...");

        let context_valid = self
            .protocol_config_contract
            .isValidKmsContext(context_id)
            .call()
            .await
            .map_err(|e| {
                RequestCheckError::network(anyhow!(
                    "isValidKmsContext(#{context_id}) call failed: {e}"
                ))
            })?;
        if !context_valid {
            return Err(RequestCheckError::recoverable(
                RequestCheckKind::KmsContext,
                anyhow!("Context #{context_id} is not valid on-chain (yet?)"),
            ));
        }

        let Some(epoch_id) = epoch_id else {
            // v1 extra_data: no epoch to validate, and nothing to cache without one
            return Ok(());
        };

        let epoch_valid = self
            .protocol_config_contract
            .isValidEpochForContext(context_id, epoch_id)
            .call()
            .await
            .map_err(|e| {
                RequestCheckError::network(anyhow!(
                    "isValidEpochForContext(#{context_id}, #{epoch_id}) call failed: {e}"
                ))
            })?;
        if !epoch_valid {
            return Err(RequestCheckError::recoverable(
                RequestCheckKind::KmsContext,
                anyhow!("Epoch #{epoch_id} of context #{context_id} is not active on-chain (yet?)"),
            ));
        }

        self.cache_valid_pair(context_id, epoch_id).await;
        Ok(())
    }

    /// Caches a pair confirmed valid on-chain, so subsequent requests skip the RPC calls.
    async fn cache_valid_pair(&self, context_id: U256, epoch_id: U256) {
        let now = Utc::now();
        let query_result = sqlx::query!(
            "INSERT INTO kms_context(id, epoch_id, is_valid, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
            context_id.as_le_slice(),
            epoch_id.as_le_slice(),
            true,
            now,
            now,
        )
        .execute(&self.db_pool)
        .await;

        match query_result {
            Ok(_) => info!("Context cached as valid in DB"),
            Err(e) => warn!("Failed to cache context: {e}"),
        }
    }
}
