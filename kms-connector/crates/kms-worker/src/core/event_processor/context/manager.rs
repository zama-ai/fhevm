use super::KmsContextCache;
use crate::core::{
    config::Config,
    event_processor::{RequestCheckError, RequestCheckKind, context::cache::LocalCheck},
};
use alloy::{eips::BlockId, primitives::U256, providers::Provider};
use anyhow::anyhow;
use connector_utils::types::extra_data::ExtraData;
use fhevm_host_bindings::protocol_config::ProtocolConfig::{self, ProtocolConfigInstance};
use sqlx::{Pool, Postgres, types::chrono::Utc};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

pub trait ContextManager: Send + Sync {
    /// Validates the KMS context and epoch referenced by a request's parsed `extra_data`.
    fn validate_context(
        &self,
        extra_data: &ExtraData,
    ) -> impl Future<Output = Result<(), RequestCheckError>> + Send;
}

/// Validates KMS contexts and epochs against the in-memory cache of the `kms_context` and
/// `kms_epoch` tables, falling back to `ProtocolConfig` on Ethereum for pairs not cached.
#[derive(Clone)]
pub struct DbContextManager<P> {
    /// The in-memory cache of the context and epoch DB tables.
    context_cache: KmsContextCache,

    /// The database pool used to persist pairs validated on-chain.
    db_pool: Pool<Postgres>,

    /// The `ProtocolConfig` contract instance on Ethereum, source of truth for context and
    /// epoch validity.
    protocol_config_contract: ProtocolConfigInstance<P>,
}

impl<P: Provider> ContextManager for DbContextManager<P> {
    #[tracing::instrument(skip_all, fields(context_id = ?extra_data.context_id, epoch_id = ?extra_data.epoch_id))]
    async fn validate_context(&self, extra_data: &ExtraData) -> Result<(), RequestCheckError> {
        let Some(context_id) = extra_data.context_id else {
            // Accepting request with no context for backwards compatibility with the relayer-sdk.
            // TODO: Remove once https://github.com/zama-ai/fhevm-internal/issues/1506 is resolved.
            return Ok(());
        };
        let epoch_id = extra_data.epoch_id;

        match self.context_cache.snapshot().check(context_id, epoch_id) {
            LocalCheck::Valid => Ok(()),
            LocalCheck::Destroyed => Err(RequestCheckError::irrecoverable(
                RequestCheckKind::KmsContext,
                anyhow!(
                    "Context #{context_id}{} has been destroyed",
                    epoch_id
                        .map(|id| format!(" or epoch #{id}"))
                        .unwrap_or_default()
                ),
            )),
            LocalCheck::Unknown => self.validate_on_chain(context_id, epoch_id).await,
        }
    }
}

impl<P: Provider> DbContextManager<P> {
    /// Creates a new `DbContextManager`, loading the initial context cache snapshot and
    /// spawning its background refresh task.
    pub async fn connect(
        db_pool: Pool<Postgres>,
        config: &Config,
        ethereum_provider: P,
        cancel_token: CancellationToken,
    ) -> anyhow::Result<Self> {
        let protocol_config_contract =
            ProtocolConfig::new(config.protocol_config_contract.address, ethereum_provider);
        let context_cache = KmsContextCache::connect(
            db_pool.clone(),
            config.kms_context_cache_refresh,
            cancel_token,
        )
        .await?;
        Ok(Self {
            context_cache,
            db_pool,
            protocol_config_contract,
        })
    }

    /// Validates a pair not cached locally against `ProtocolConfig`, caching it on success.
    async fn validate_on_chain(
        &self,
        context_id: U256,
        epoch_id: Option<U256>,
    ) -> Result<(), RequestCheckError> {
        info!("Context not found in cache, validating against ProtocolConfig...");

        let context_valid = self
            .protocol_config_contract
            .isValidKmsContext(context_id)
            .block(BlockId::finalized())
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
            .block(BlockId::finalized())
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
    ///
    /// The pair is registered in memory (effective immediately) and persisted in the DB (to
    /// survive restarts, and as the source the in-memory cache refreshes from).
    async fn cache_valid_pair(&self, context_id: U256, epoch_id: U256) {
        self.context_cache.insert_valid_pair(context_id, epoch_id);

        let now = Utc::now();
        let query_result = async {
            let mut tx = self.db_pool.begin().await?;
            sqlx::query!(
                "INSERT INTO kms_context(id, is_valid, created_at, updated_at)
                VALUES ($1, TRUE, $2, $2) ON CONFLICT DO NOTHING",
                context_id.as_le_slice(),
                now,
            )
            .execute(&mut *tx)
            .await?;
            // Only the context association is upserted, never `is_valid`: a destruction event
            // processed between the on-chain read and this write must not be overridden
            sqlx::query!(
                "INSERT INTO kms_epoch(id, context_id, is_valid, created_at, updated_at)
                VALUES ($1, $2, TRUE, $3, $3)
                ON CONFLICT (id) DO UPDATE SET context_id = $2, updated_at = $3",
                epoch_id.as_le_slice(),
                context_id.as_le_slice(),
                now,
            )
            .execute(&mut *tx)
            .await?;
            tx.commit().await
        }
        .await;

        match query_result {
            Ok(()) => info!("Context cached as valid in DB"),
            Err(e) => warn!("Failed to cache context: {e}"),
        }
    }
}
