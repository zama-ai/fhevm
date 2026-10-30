//! Persisted sender ownership and lifecycle. Callers retain the live-write fence
//! across preparation and broadcast; signed bytes commit in a short inner transaction.
use super::begin_live_write;
use sqlx::{PgPool, Postgres, Transaction};

#[derive(Clone, Debug)]
pub(super) enum Task {
    Proof(i64),
    Ciphertext(Vec<u8>),
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Status {
    RetryWait,
    AwaitingOutcome,
    Completed,
    Stopped,
}

impl Status {
    fn as_str(self) -> &'static str {
        match self {
            Self::RetryWait => "RETRY_WAIT",
            Self::AwaitingOutcome => "AWAITING_OUTCOME",
            Self::Completed => "COMPLETED",
            Self::Stopped => "STOPPED",
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct Attempt {
    pub task: Task,
    pub number: i64,
    pub raw: Option<Vec<u8>>,
}

impl Attempt {
    /// Compare-and-swap the selected generation. A stale selection cannot claim a newer job.
    pub async fn claim(pool: &PgPool, task: Task, selected: i64) -> anyhow::Result<Option<Self>> {
        let Some(mut tx) = begin_live_write(pool).await? else {
            return Ok(None);
        };
        let number = match &task {
            Task::Proof(id) => sqlx::query!(
                "UPDATE verify_proofs SET sender_status = 'SENDING',
                    sender_attempt_no = sender_attempt_no + 1, sender_status_updated_at = NOW(),
                    sender_next_action_at = NULL
                 WHERE zk_proof_id = $1 AND sender_attempt_no = $2
                   AND sender_status IN ('QUEUED', 'RETRY_WAIT', 'AWAITING_OUTCOME')
                   AND (sender_next_action_at IS NULL OR sender_next_action_at <= NOW())
                 RETURNING sender_attempt_no, sender_raw_tx",
                id,
                selected
            )
            .fetch_optional(tx.as_mut())
            .await?
            .map(|row| (row.sender_attempt_no, row.sender_raw_tx)),
            Task::Ciphertext(handle) => sqlx::query!(
                "UPDATE ciphertext_digest SET sender_status = 'SENDING',
                    sender_attempt_no = sender_attempt_no + 1, sender_status_updated_at = NOW(),
                    sender_next_action_at = NULL
                 WHERE handle = $1 AND sender_attempt_no = $2
                   AND sender_status IN ('QUEUED', 'RETRY_WAIT', 'AWAITING_OUTCOME')
                   AND (sender_next_action_at IS NULL OR sender_next_action_at <= NOW())
                 RETURNING sender_attempt_no, sender_raw_tx",
                handle,
                selected
            )
            .fetch_optional(tx.as_mut())
            .await?
            .map(|row| (row.sender_attempt_no, row.sender_raw_tx)),
        };
        tx.commit().await?;
        Ok(number.map(|(number, raw)| Self { task, number, raw }))
    }

    pub fn retry_status(&self) -> Status {
        if self.raw.is_some() {
            Status::AwaitingOutcome
        } else {
            Status::RetryWait
        }
    }

    /// Inner errors are RPC outcomes for the operation's existing retry policy;
    /// outer errors are persistence failures and must not trigger a fresh submission.
    pub async fn submit<P>(
        &mut self,
        pool: &PgPool,
        _acceptance_fence: &mut Transaction<'_, Postgres>,
        provider: &crate::NonceManagedProvider<P>,
        signer: crate::AbstractSigner,
        request: alloy::rpc::types::TransactionRequest,
        conf: &crate::ConfigSettings,
    ) -> anyhow::Result<alloy::transports::TransportResult<alloy::rpc::types::TransactionReceipt>>
    where
        P: alloy::providers::Provider + Clone + 'static,
    {
        use crate::nonce_managed_provider::PreparedTransaction;
        let timeout = std::time::Duration::from_secs(conf.send_txn_sync_timeout_secs.into());
        let prepared = if let Some(raw) = &self.raw {
            let prepared = PreparedTransaction::decode(raw.clone())?;
            match provider
                .inner()
                .get_transaction_receipt(prepared.hash)
                .await
            {
                Ok(Some(receipt)) => {
                    provider.settle_prepared(&prepared, true).await;
                    self.raw = None;
                    return Ok(Ok(receipt));
                }
                Ok(None) => {}
                Err(error) => return Ok(Err(error)),
            }
            prepared
        } else {
            let prepared = match provider
                .prepare_transaction(request, conf.gas_limit_overprovision_percent, signer)
                .await
            {
                Ok(prepared) => prepared,
                Err(error) => return Ok(Err(error)),
            };
            // The caller's fence already protects this write. Acquiring another
            // advisory fence on a second connection could deadlock behind a
            // queued halt waiting for the caller's fence to drain.
            let mut tx = pool.begin().await?;
            let changed = match &self.task {
                Task::Proof(id) => sqlx::query!(
                    "UPDATE verify_proofs SET sender_raw_tx = $3, sender_tx_hash = $4
                     WHERE zk_proof_id = $1 AND sender_attempt_no = $2 AND sender_status = 'SENDING'",
                    id, self.number, &prepared.raw, prepared.hash.as_slice()
                ).execute(tx.as_mut()).await?.rows_affected(),
                Task::Ciphertext(handle) => sqlx::query!(
                    "UPDATE ciphertext_digest SET sender_raw_tx = $3, sender_tx_hash = $4
                     WHERE handle = $1 AND sender_attempt_no = $2 AND sender_status = 'SENDING'",
                    handle, self.number, &prepared.raw, prepared.hash.as_slice()
                ).execute(tx.as_mut()).await?.rows_affected(),
            };
            if changed == 0 {
                provider.settle_prepared(&prepared, false).await;
                anyhow::bail!("sender attempt was superseded before broadcast");
            }
            // If commit's outcome is uncertain, retain the reservation until recovery.
            tx.commit().await?;
            self.raw = Some(prepared.raw.clone());
            prepared
        };
        let result = provider.broadcast_prepared(&prepared, timeout).await;
        if result.is_ok() {
            provider.settle_prepared(&prepared, true).await;
            self.raw = None;
            return Ok(result);
        }
        // An immediate receipt check resolves accepted transactions whose response was lost.
        if let Ok(Some(receipt)) = provider
            .inner()
            .get_transaction_receipt(prepared.hash)
            .await
        {
            provider.settle_prepared(&prepared, true).await;
            self.raw = None;
            return Ok(Ok(receipt));
        }
        let error = result.as_ref().unwrap_err();
        if error.as_error_resp().is_some() && !super::common::is_transient_gateway_error(error) {
            // A nonce rejection can describe an earlier accepted attempt. Keep its reservation
            // while that hash is known to the node, or while reconciliation itself fails.
            if super::common::is_retryable_nonce_error(error) {
                match provider
                    .inner()
                    .get_transaction_by_hash(prepared.hash)
                    .await
                {
                    Ok(None) => {}
                    _ => return Ok(result),
                }
                // Reconcile before releasing: if that RPC fails, keep the original reservation.
                let confirmed = match provider.reconcile_nonce().await {
                    Ok(confirmed) => confirmed,
                    Err(_) => return Ok(result),
                };
                provider
                    .settle_prepared(&prepared, confirmed > prepared.nonce)
                    .await;
            } else {
                provider.settle_prepared(&prepared, false).await;
            }
            self.raw = None;
        }
        Ok(result)
    }

    /// The caller commits its legacy fields and the status in the same guarded transaction.
    /// Returning None means this attempt lost ownership (replay, cleanup, or cutover).
    pub async fn transition(
        &self,
        pool: &PgPool,
        status: Status,
        error: Option<&str>,
        delay_secs: u16,
    ) -> anyhow::Result<Option<Transaction<'static, Postgres>>> {
        let Some(mut tx) = begin_live_write(pool).await? else {
            return Ok(None);
        };
        let status = status.as_str();
        let delay = f64::from(delay_secs);
        let changed =
            match &self.task {
                Task::Proof(id) => {
                    sqlx::query!(
                "UPDATE verify_proofs SET sender_status = CASE WHEN $3 = 'AWAITING_OUTCOME' AND sender_raw_tx IS NULL
                        THEN 'RETRY_WAIT' ELSE $3 END, sender_status_updated_at = NOW(),
                    sender_last_error = $4,
                    sender_raw_tx = CASE WHEN $3 = 'AWAITING_OUTCOME' THEN sender_raw_tx ELSE NULL END,
                    sender_tx_hash = CASE WHEN $3 = 'RETRY_WAIT' THEN NULL ELSE sender_tx_hash END,
                    sender_next_action_at = CASE WHEN $3 IN ('RETRY_WAIT', 'AWAITING_OUTCOME')
                        THEN NOW() + make_interval(secs => $5) ELSE NULL END
                 WHERE zk_proof_id = $1 AND sender_attempt_no = $2 AND sender_status = 'SENDING'",
                id, self.number, status, error, delay
            )
                    .execute(tx.as_mut())
                    .await?
                    .rows_affected()
                }
                Task::Ciphertext(handle) => sqlx::query!(
                "UPDATE ciphertext_digest SET sender_status = CASE WHEN $3 = 'AWAITING_OUTCOME' AND sender_raw_tx IS NULL
                        THEN 'RETRY_WAIT' ELSE $3 END, sender_status_updated_at = NOW(),
                    sender_last_error = $4,
                    sender_raw_tx = CASE WHEN $3 = 'AWAITING_OUTCOME' THEN sender_raw_tx ELSE NULL END,
                    sender_tx_hash = CASE WHEN $3 = 'RETRY_WAIT' THEN NULL ELSE sender_tx_hash END,
                    sender_next_action_at = CASE WHEN $3 IN ('RETRY_WAIT', 'AWAITING_OUTCOME')
                        THEN NOW() + make_interval(secs => $5) ELSE NULL END
                 WHERE handle = $1 AND sender_attempt_no = $2 AND sender_status = 'SENDING'",
                handle, self.number, status, error, delay
            )
                .execute(tx.as_mut())
                .await?
                .rows_affected(),
            };
        if changed == 0 {
            return Ok(None);
        }
        Ok(Some(tx))
    }
}

/// Session ownership makes restart recovery safe against a second live sender.
/// Keep the detached connection alive for the full run; dropping it releases the lock.
pub(crate) async fn recover<P>(
    pool: &PgPool,
    provider: &crate::NonceManagedProvider<P>,
) -> anyhow::Result<sqlx::PgConnection>
where
    P: alloy::providers::Provider + Clone + 'static,
{
    let mut owner = pool.acquire().await?.detach();
    let locked = sqlx::query_scalar!(r#"SELECT pg_try_advisory_lock(7346182640291) AS "locked!""#)
        .fetch_one(&mut owner)
        .await?;
    anyhow::ensure!(locked, "another transaction sender owns this database");
    let Some(mut tx) = begin_live_write(pool).await? else {
        anyhow::bail!("sender retired during recovery")
    };
    sqlx::query!("UPDATE verify_proofs SET
            sender_status = CASE WHEN sender_raw_tx IS NULL THEN 'RETRY_WAIT' ELSE 'AWAITING_OUTCOME' END,
            sender_attempt_no = sender_attempt_no + 1, sender_status_updated_at = NOW(),
            sender_next_action_at = NOW(), sender_last_error = 'previous sender attempt interrupted'
        WHERE sender_status = 'SENDING'").execute(tx.as_mut()).await?;
    sqlx::query!("UPDATE ciphertext_digest SET
            sender_status = CASE WHEN sender_raw_tx IS NULL THEN 'RETRY_WAIT' ELSE 'AWAITING_OUTCOME' END,
            sender_attempt_no = sender_attempt_no + 1, sender_status_updated_at = NOW(),
            sender_next_action_at = NOW(), sender_last_error = 'previous sender attempt interrupted'
        WHERE sender_status = 'SENDING'").execute(tx.as_mut()).await?;
    let transactions = sqlx::query_scalar!(
        r#"SELECT sender_raw_tx AS "raw!" FROM verify_proofs WHERE sender_raw_tx IS NOT NULL
         UNION ALL SELECT sender_raw_tx AS "raw!" FROM ciphertext_digest WHERE sender_raw_tx IS NOT NULL"#
    ).fetch_all(tx.as_mut()).await?;
    tx.commit().await?;
    provider.restore_prepared(&transactions).await?;
    Ok(owner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn concurrent_claim_and_stale_completion_cannot_overwrite_current_attempt(
    ) -> anyhow::Result<()> {
        let database =
            test_harness::instance::setup_test_db(test_harness::instance::ImportMode::None)
                .await
                .map_err(|error| anyhow::anyhow!("{error}"))?;
        let pool = PgPool::connect(database.db_url()).await?;
        sqlx::query("INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, verified, handles)
            VALUES (1, 42, '0x0000000000000000000000000000000000000001',
                '0x0000000000000000000000000000000000000002', TRUE, '')")
            .execute(&pool).await?;
        let (left, right) = tokio::join!(
            Attempt::claim(&pool, Task::Proof(1), 0),
            Attempt::claim(&pool, Task::Proof(1), 0)
        );
        let (left, right) = (left?, right?);
        assert_ne!(left.is_some(), right.is_some());
        let original = left.or(right).unwrap();
        original
            .transition(&pool, Status::RetryWait, Some("temporary error"), 0)
            .await?
            .unwrap()
            .commit()
            .await?;
        let current = Attempt::claim(&pool, Task::Proof(1), original.number)
            .await?
            .unwrap();
        assert!(original
            .transition(&pool, Status::Completed, None, 0)
            .await?
            .is_none());
        let state: (String, i64) = sqlx::query_as(
            "SELECT sender_status, sender_attempt_no FROM verify_proofs WHERE zk_proof_id = 1",
        )
        .fetch_one(&pool)
        .await?;
        assert_eq!(state, ("SENDING".into(), current.number));
        // Gateway finalization/replay can invalidate a claim too.
        sqlx::query("UPDATE verify_proofs SET sender_status = 'COMPLETED', sender_attempt_no = sender_attempt_no + 1 WHERE zk_proof_id = 1")
            .execute(&pool).await?;
        assert!(current
            .transition(&pool, Status::Stopped, Some("late failure"), 0)
            .await?
            .is_none());
        Ok(())
    }
}
