use crate::core::{
    config::Config,
    event_processor::{RequestCheckError, RequestCheckKind},
};
use alloy::{eips::BlockId, primitives::U256, providers::Provider};
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

/// Outcome of the lookup in the local `kms_context` and `kms_epoch` tables.
enum LocalCheck {
    Valid,
    Destroyed,
    Unknown,
}

/// Validates KMS contexts and epochs against the local `kms_context` and `kms_epoch` tables,
/// falling back to `ProtocolConfig` on Ethereum for pairs not cached locally.
#[derive(Clone)]
pub struct DbContextManager<P> {
    /// The database pool used to query the `kms_context` and `kms_epoch` tables.
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

        match self.check_local_db(context_id, epoch_id).await? {
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
    pub fn new(db_pool: Pool<Postgres>, config: &Config, ethereum_provider: P) -> Self {
        let protocol_config_contract =
            ProtocolConfig::new(config.protocol_config_contract.address, ethereum_provider);
        Self {
            db_pool,
            protocol_config_contract,
        }
    }

    /// Checks the requested context and epoch against the local `kms_context` and `kms_epoch`
    /// tables, in a single query as this lookup is on the requests' hot path.
    async fn check_local_db(
        &self,
        context_id: U256,
        epoch_id: Option<U256>,
    ) -> Result<LocalCheck, RequestCheckError> {
        // No row is returned when the context is not cached, and the `epoch_*` columns are
        // `NULL` when the epoch is not cached.
        let row = sqlx::query!(
            r#"SELECT c.is_valid AS "context_is_valid!", e.is_valid AS "epoch_is_valid?",
                e.context_id AS "epoch_context_id?"
            FROM kms_context c
            LEFT JOIN kms_epoch e ON e.id = $2
            WHERE c.id = $1"#,
            context_id.as_le_slice(),
            epoch_id.as_ref().map(U256::as_le_slice),
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| {
            RequestCheckError::network(anyhow!("Query to check context/epoch validity failed: {e}"))
        })?;

        let Some(row) = row else {
            return Ok(LocalCheck::Unknown); // Context not cached
        };
        if !row.context_is_valid {
            return Ok(LocalCheck::Destroyed);
        }
        if epoch_id.is_none() {
            // v1 extra_data carries no epoch: the context row alone concludes
            return Ok(LocalCheck::Valid);
        }
        match row.epoch_is_valid {
            None => Ok(LocalCheck::Unknown), // Epoch not cached
            Some(false) => Ok(LocalCheck::Destroyed),
            Some(true) if row.epoch_context_id.as_deref() == Some(context_id.as_le_slice()) => {
                Ok(LocalCheck::Valid)
            }
            // In case of mismatch between the cached context and the requested one, double-check
            // on-chain instead of rejecting, and let `cache_valid_pair` fix the row if needed
            Some(true) => {
                warn!("Requested context does not match the one cached for this epoch");
                Ok(LocalCheck::Unknown)
            }
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
    async fn cache_valid_pair(&self, context_id: U256, epoch_id: U256) {
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
