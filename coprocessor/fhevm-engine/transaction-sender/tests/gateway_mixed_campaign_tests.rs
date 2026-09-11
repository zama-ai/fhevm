mod common;
mod support;

use alloy::providers::ProviderBuilder;
use common::{CiphertextCommits, InputVerification, SignerType, TestEnvironment};
use std::time::Duration;
use support::FaultProxy;
use test_harness::db_utils::{insert_ciphertext_digest, insert_random_keys_and_host_chain};
use transaction_sender::{
    gateway_http_client, ConfigSettings, FillersWithoutNonceManagement, NonceManagedProvider,
    TransactionSender,
};

#[tokio::test]
#[ignore = "extended validation campaign"]
async fn mixed_work_progresses_through_selective_outage_and_restarts() -> anyhow::Result<()> {
    let conf = ConfigSettings {
        verify_proof_resp_batch_limit: 10,
        add_ciphertexts_batch_limit: 10,
        verify_proof_resp_max_retries: 15,
        verify_proof_remove_after_max_retries: true,
        graceful_shutdown_timeout: Duration::from_secs(12),
        ..Default::default()
    };
    let mut env = TestEnvironment::new_with_config(SignerType::PrivateKey, conf, false).await?;
    let deploy = env.http_provider()?;
    let input = InputVerification::deploy(&deploy, false, false, false, false).await?;
    let commits = CiphertextCommits::deploy(&deploy, false).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    proxy.set_early_proof_fault(10);
    sqlx::raw_sql(
        "CREATE TABLE mixed_audit (old_row jsonb, new_row jsonb);
        CREATE FUNCTION audit_mixed() RETURNS trigger LANGUAGE plpgsql AS $$
        BEGIN INSERT INTO mixed_audit VALUES (to_jsonb(OLD), to_jsonb(NEW)); RETURN NULL; END $$;
        CREATE TRIGGER mixed_audit AFTER UPDATE OR DELETE ON verify_proofs
        FOR EACH ROW EXECUTE FUNCTION audit_mixed();",
    )
    .execute(&env.db_pool)
    .await?;
    let (host, key) = insert_random_keys_and_host_chain(&env.db_pool).await?;
    let mut next_id = 1i64;
    let mut later_completed = Vec::new();
    for cycle in 0..3 {
        for n in 0..10u8 {
            insert_ciphertext_digest(
                &env.db_pool,
                host,
                key,
                &[40 + cycle * 10 + n; 32],
                &[2u8; 32],
                &[3u8; 32],
                1,
            )
            .await?;
        }
        let url = proxy.url();
        let inner = ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .connect_reqwest(gateway_http_client(&url)?, url);
        let provider =
            NonceManagedProvider::new(inner, Some(env.wallet.default_signer().address()));
        let sender = TransactionSender::new(
            env.db_pool.clone(),
            *input.address(),
            *commits.address(),
            env.signer.clone(),
            provider,
            env.cancel_token.clone(),
            env.conf.clone(),
            None,
        )
        .await?;
        let calls = proxy.calls("eth_estimateGas");
        let run = tokio::spawn(async move { sender.run().await });
        // More than two proof batches initially; new work arrives on each restart.
        let count = if cycle == 0 { 30 } else { 10 };
        for _ in 0..count {
            sqlx::query("INSERT INTO verify_proofs
                (zk_proof_id, chain_id, contract_address, user_address, handles, verified, retry_count)
                VALUES ($1,42,$2,$3,$4,$5,14)")
                .bind(next_id).bind(env.contract_address.to_string()).bind(env.user_address.to_string())
                .bind(vec![1u8;64]).bind(next_id % 2 == 0).execute(&env.db_pool).await?;
            next_id += 1;
            // Keep new work arriving while deferred early proofs become due.
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
        // Require observed retries, rather than accepting an idle sender.
        tokio::time::timeout(Duration::from_secs(30), async {
            while proxy.calls("eth_estimateGas") < calls + 40 {
                assert!(!run.is_finished());
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        })
        .await?;
        let early: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM verify_proofs WHERE zk_proof_id <= 10 AND retry_count=14",
        )
        .fetch_one(&env.db_pool)
        .await?;
        assert_eq!(early, 10, "early work lost eligibility");
        tokio::time::timeout(Duration::from_secs(30), async {
            loop {
                let remaining: i64 =
                    sqlx::query_scalar("SELECT COUNT(*) FROM verify_proofs WHERE zk_proof_id > 10")
                        .fetch_one(&env.db_pool)
                        .await?;
                if remaining == 0 {
                    break;
                }
                assert!(!run.is_finished());
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            anyhow::Ok(())
        })
        .await??;
        let remaining = 0;
        later_completed.push(next_id - 11 - remaining);
        let mutations: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM mixed_audit WHERE (old_row->>'zk_proof_id')::bigint <= 10
             AND (old_row - 'last_retry_at') IS DISTINCT FROM (new_row - 'last_retry_at')",
        )
        .fetch_one(&env.db_pool)
        .await?;
        assert_eq!(
            mutations, 0,
            "transient failures changed more than retry scheduling"
        );
        tokio::time::timeout(Duration::from_secs(30), async {
            loop {
                let remaining: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM ciphertext_digest WHERE txn_is_sent = false",
                )
                .fetch_one(&env.db_pool)
                .await?;
                if remaining == 0 {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            anyhow::Ok(())
        })
        .await??;
        let retried: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM mixed_audit
             WHERE (old_row->>'zk_proof_id')::bigint <= 10
               AND old_row->>'last_retry_at' IS NOT NULL
               AND new_row->>'last_retry_at' IS DISTINCT FROM old_row->>'last_retry_at'",
        )
        .fetch_one(&env.db_pool)
        .await?;
        assert!(retried > 0, "new arrivals starved deferred retries");
        if cycle == 2 {
            proxy.clear_all_faults();
            tokio::time::timeout(Duration::from_secs(60), async {
                loop {
                    let remaining: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM verify_proofs")
                        .fetch_one(&env.db_pool)
                        .await?;
                    if remaining == 0 {
                        break;
                    }
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
                anyhow::Ok(())
            })
            .await??;
        }
        env.cancel_token.cancel();
        tokio::time::timeout(Duration::from_secs(15), run).await???;
        env.cancel_token = tokio_util::sync::CancellationToken::new();
    }
    let verified = input
        .VerifyProofResponse_filter()
        .from_block(0)
        .query()
        .await?;
    let rejected = input
        .RejectProofResponse_filter()
        .from_block(0)
        .query()
        .await?;
    let mut ids: Vec<_> = verified
        .iter()
        .map(|x| x.0.zkProofId)
        .chain(rejected.iter().map(|x| x.0.zkProofId))
        .collect();
    ids.sort();
    ids.dedup();
    assert_eq!(
        ids.len(),
        50,
        "every logical proof must have contract evidence"
    );
    println!("All 50 proofs recovered after two restarts; ciphertexts drained during outage; later proof completions per cycle: {later_completed:?}");
    assert!(
        later_completed.iter().all(|n| *n > 0),
        "selective failures starved later healthy proof batches"
    );
    Ok(())
}
