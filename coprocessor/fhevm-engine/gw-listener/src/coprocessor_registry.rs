use alloy::{
    eips::BlockId,
    network::Ethereum,
    primitives::{Address, B256},
    providers::Provider,
    rpc::types::BlockNumberOrTag,
};
use anyhow::{bail, Context};
use fhevm_engine_common::utils::to_hex;
use fhevm_gateway_bindings::gateway_config::GatewayConfig;
use sqlx::{Pool, Postgres};
use tracing::{info, warn};

const REGISTRY_REFRESH_ADVISORY_LOCK: i64 = 0x0047_5743_4f50_524f;
pub(crate) const EVENT_GATEWAY_CONFIG_COPROCESSORS_UPDATED: &str =
    "event_gateway_config_coprocessors_updated";

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CoprocessorRegistryEntry {
    pub tx_sender_address: Address,
    pub signer_address: Address,
    pub s3_bucket_url: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CoprocessorRegistrySnapshot {
    pub gateway_chain_id: i64,
    pub gateway_config_address: Address,
    pub snapshot_block_number: i64,
    pub snapshot_block_hash: B256,
    pub coprocessor_threshold: i64,
    pub coprocessors: Vec<CoprocessorRegistryEntry>,
}

pub(crate) async fn refresh_coprocessor_registry<P>(
    provider: &P,
    gateway_config_address: Address,
    db_pool: &Pool<Postgres>,
) -> anyhow::Result<()>
where
    P: Provider<Ethereum>,
{
    let snapshot = fetch_coprocessor_registry(provider, gateway_config_address).await?;
    if persist_coprocessor_registry(db_pool, &snapshot).await? {
        info!(
            gateway_chain_id = snapshot.gateway_chain_id,
            gateway_config_address = %snapshot.gateway_config_address,
            snapshot_block_number = snapshot.snapshot_block_number,
            snapshot_block_hash = %snapshot.snapshot_block_hash,
            coprocessor_count = snapshot.coprocessors.len(),
            coprocessor_threshold = snapshot.coprocessor_threshold,
            "Persisted GatewayConfig coprocessor registry snapshot"
        );
    }
    Ok(())
}

async fn fetch_coprocessor_registry<P>(
    provider: &P,
    gateway_config_address: Address,
) -> anyhow::Result<CoprocessorRegistrySnapshot>
where
    P: Provider<Ethereum>,
{
    let gateway_chain_id = i64::try_from(provider.get_chain_id().await?)
        .context("Gateway chain ID exceeds PostgreSQL BIGINT")?;
    let snapshot_block_number_u64 = provider.get_block_number().await?;
    let snapshot_block = provider
        .get_block_by_number(BlockNumberOrTag::Number(snapshot_block_number_u64))
        .await?
        .context("latest Gateway block disappeared while reading GatewayConfig")?;
    let snapshot_block_number = i64::try_from(snapshot_block_number_u64)
        .context("Gateway block number exceeds PostgreSQL BIGINT")?;
    let at_block = BlockId::hash_canonical(snapshot_block.header.hash);
    let gateway_config = GatewayConfig::new(gateway_config_address, provider);

    let tx_senders = gateway_config
        .getCoprocessorTxSenders()
        .block(at_block)
        .call()
        .await
        .context("GatewayConfig.getCoprocessorTxSenders failed")?;
    if tx_senders.is_empty() {
        bail!("GatewayConfig returned no coprocessor transaction senders");
    }

    let threshold = gateway_config
        .getCoprocessorMajorityThreshold()
        .block(at_block)
        .call()
        .await
        .context("GatewayConfig.getCoprocessorMajorityThreshold failed")?;
    let coprocessor_threshold = i64::try_from(threshold)
        .context("GatewayConfig coprocessor threshold exceeds PostgreSQL BIGINT")?;

    let mut coprocessors = Vec::with_capacity(tx_senders.len());
    for tx_sender_address in tx_senders {
        let coprocessor = gateway_config
            .getCoprocessor(tx_sender_address)
            .block(at_block)
            .call()
            .await
            .with_context(|| format!("GatewayConfig.getCoprocessor({tx_sender_address}) failed"))?;
        if coprocessor.txSenderAddress != tx_sender_address {
            bail!(
                "GatewayConfig returned transaction sender {} for registry key {}",
                coprocessor.txSenderAddress,
                tx_sender_address,
            );
        }
        coprocessors.push(CoprocessorRegistryEntry {
            tx_sender_address,
            signer_address: coprocessor.signerAddress,
            s3_bucket_url: coprocessor.s3BucketUrl,
        });
    }

    let snapshot = CoprocessorRegistrySnapshot {
        gateway_chain_id,
        gateway_config_address,
        snapshot_block_number,
        snapshot_block_hash: snapshot_block.header.hash,
        coprocessor_threshold,
        coprocessors,
    };
    validate_snapshot(&snapshot)?;
    Ok(snapshot)
}

pub(crate) async fn persist_coprocessor_registry(
    db_pool: &Pool<Postgres>,
    snapshot: &CoprocessorRegistrySnapshot,
) -> anyhow::Result<bool> {
    validate_snapshot(snapshot)?;
    let mut transaction = db_pool.begin().await?;
    sqlx::query!(
        "SELECT pg_advisory_xact_lock($1)",
        REGISTRY_REFRESH_ADVISORY_LOCK,
    )
    .execute(transaction.as_mut())
    .await?;

    let current = sqlx::query!(
        r#"
        SELECT gateway_chain_id,
               gateway_config_address,
               snapshot_block_number,
               snapshot_block_hash
          FROM public.gateway_config_coprocessors
         LIMIT 1
        "#,
    )
    .fetch_optional(transaction.as_mut())
    .await?;
    if current.as_ref().is_some_and(|current| {
        current.gateway_chain_id == snapshot.gateway_chain_id
            && current.gateway_config_address.as_slice()
                == snapshot.gateway_config_address.as_slice()
            && current.snapshot_block_number > snapshot.snapshot_block_number
    }) {
        let current = current.expect("checked present");
        warn!(
            current_block_number = current.snapshot_block_number,
            current_block_hash = %to_hex(&current.snapshot_block_hash),
            attempted_block_number = snapshot.snapshot_block_number,
            attempted_block_hash = %snapshot.snapshot_block_hash,
            "Ignoring older GatewayConfig coprocessor registry snapshot"
        );
        transaction.commit().await?;
        return Ok(false);
    }

    sqlx::query!("DELETE FROM public.gateway_config_coprocessors")
        .execute(transaction.as_mut())
        .await?;
    for coprocessor in &snapshot.coprocessors {
        sqlx::query!(
            r#"
            INSERT INTO public.gateway_config_coprocessors (
                tx_sender_address,
                signer_address,
                s3_bucket_url,
                coprocessor_threshold,
                gateway_chain_id,
                gateway_config_address,
                snapshot_block_number,
                snapshot_block_hash
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            coprocessor.tx_sender_address.as_slice(),
            coprocessor.signer_address.as_slice(),
            coprocessor.s3_bucket_url,
            snapshot.coprocessor_threshold,
            snapshot.gateway_chain_id,
            snapshot.gateway_config_address.as_slice(),
            snapshot.snapshot_block_number,
            snapshot.snapshot_block_hash.as_slice(),
        )
        .execute(transaction.as_mut())
        .await?;
    }
    sqlx::query!(
        "SELECT pg_notify($1, '')",
        EVENT_GATEWAY_CONFIG_COPROCESSORS_UPDATED,
    )
    .execute(transaction.as_mut())
    .await?;
    transaction.commit().await?;
    Ok(true)
}

fn validate_snapshot(snapshot: &CoprocessorRegistrySnapshot) -> anyhow::Result<()> {
    if snapshot.gateway_chain_id < 0 {
        bail!("Gateway chain ID must be non-negative");
    }
    if snapshot.snapshot_block_number < 0 {
        bail!("Gateway snapshot block number must be non-negative");
    }
    if snapshot.coprocessors.is_empty() {
        bail!("GatewayConfig coprocessor registry must not be empty");
    }
    let coprocessor_count = i64::try_from(snapshot.coprocessors.len())
        .context("coprocessor count exceeds PostgreSQL BIGINT")?;
    if snapshot.coprocessor_threshold <= 0 || snapshot.coprocessor_threshold > coprocessor_count {
        bail!(
            "invalid coprocessor threshold {} for {} registered coprocessors",
            snapshot.coprocessor_threshold,
            coprocessor_count,
        );
    }
    for coprocessor in &snapshot.coprocessors {
        if coprocessor.s3_bucket_url.trim().is_empty() {
            bail!(
                "GatewayConfig coprocessor {} has an empty S3 bucket URL",
                coprocessor.tx_sender_address,
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use sqlx::postgres::PgPoolOptions;
    use test_harness::instance::ImportMode;

    fn snapshot() -> CoprocessorRegistrySnapshot {
        CoprocessorRegistrySnapshot {
            gateway_chain_id: 54321,
            gateway_config_address: Address::repeat_byte(0x10),
            snapshot_block_number: 100,
            snapshot_block_hash: B256::repeat_byte(0x20),
            coprocessor_threshold: 2,
            coprocessors: vec![
                CoprocessorRegistryEntry {
                    tx_sender_address: Address::repeat_byte(0x30),
                    signer_address: Address::repeat_byte(0x31),
                    s3_bucket_url: "https://operator-1.example".into(),
                },
                CoprocessorRegistryEntry {
                    tx_sender_address: Address::repeat_byte(0x40),
                    signer_address: Address::repeat_byte(0x41),
                    s3_bucket_url: "https://operator-2.example".into(),
                },
            ],
        }
    }

    #[test]
    fn validates_complete_snapshot() {
        validate_snapshot(&snapshot()).unwrap();
    }

    #[test]
    fn rejects_invalid_threshold_or_empty_bucket_url() {
        let mut invalid = snapshot();
        invalid.coprocessor_threshold = 3;
        assert!(validate_snapshot(&invalid).is_err());

        invalid = snapshot();
        invalid.coprocessors[0].s3_bucket_url = "  ".into();
        assert!(validate_snapshot(&invalid).is_err());
    }

    #[tokio::test]
    #[serial(db)]
    async fn atomically_replaces_snapshot_and_rejects_older_refresh() -> anyhow::Result<()> {
        let database = test_harness::instance::setup_test_db(ImportMode::None)
            .await
            .expect("valid test database");
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(database.db_url.as_str())
            .await?;
        sqlx::query!("TRUNCATE public.gateway_config_coprocessors")
            .execute(&pool)
            .await?;

        let initial = snapshot();
        assert!(persist_coprocessor_registry(&pool, &initial).await?);
        let rows = sqlx::query!(
            r#"
            SELECT tx_sender_address,
                   signer_address,
                   s3_bucket_url,
                   coprocessor_threshold,
                   snapshot_block_number,
                   snapshot_block_hash
              FROM public.gateway_config_coprocessors
             ORDER BY tx_sender_address
            "#,
        )
        .fetch_all(&pool)
        .await?;
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].coprocessor_threshold, 2);
        assert_eq!(rows[0].snapshot_block_number, 100);
        assert_eq!(rows[0].snapshot_block_hash, vec![0x20; 32]);

        let mut older = initial.clone();
        older.snapshot_block_number = 99;
        older.snapshot_block_hash = B256::repeat_byte(0x19);
        older.coprocessors[0].s3_bucket_url = "https://stale.example".into();
        assert!(!persist_coprocessor_registry(&pool, &older).await?);
        let urls = sqlx::query_scalar!(
            "SELECT s3_bucket_url FROM public.gateway_config_coprocessors ORDER BY tx_sender_address",
        )
        .fetch_all(&pool)
        .await?;
        assert_eq!(
            urls,
            vec![
                "https://operator-1.example".to_owned(),
                "https://operator-2.example".to_owned(),
            ],
        );

        let mut newer = initial;
        newer.snapshot_block_number = 101;
        newer.snapshot_block_hash = B256::repeat_byte(0x21);
        newer.coprocessor_threshold = 1;
        newer.coprocessors.truncate(1);
        assert!(persist_coprocessor_registry(&pool, &newer).await?);
        let current = sqlx::query!(
            r#"
            SELECT COUNT(*) AS "count!",
                   MIN(coprocessor_threshold) AS "threshold!",
                   MIN(snapshot_block_number) AS "block_number!"
              FROM public.gateway_config_coprocessors
            "#,
        )
        .fetch_one(&pool)
        .await?;
        assert_eq!(current.count, 1);
        assert_eq!(current.threshold, 1);
        assert_eq!(current.block_number, 101);
        Ok(())
    }
}
