use std::time::Duration;

use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use alloy::primitives::U256;
use alloy::providers::{Provider, ProviderBuilder, WalletProvider, WsConnect};
use alloy::signers::local::PrivateKeySigner;
use serial_test::serial;
use tokio::time::sleep;

use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::utils::DatabaseURL;
use host_listener::database::tfhe_event_propagate::Database;
use host_listener::poller::{run_poller, PollerConfig};
use test_harness::instance::ImportMode;

mod common;
use alloy::sol_types::SolEvent;
use common::{allowed_request, emit_log_request, RawLog};
use host_listener::contracts::TfheContract::TrivialEncrypt;

#[tokio::test]
#[serial(db)]
async fn poller_state_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let db_instance =
        test_harness::instance::setup_test_db(ImportMode::None).await?;
    let chain_id = ChainId::try_from(42_u64).unwrap();

    let db_url: DatabaseURL = db_instance.db_url.clone();
    let mut db = Database::new(&db_url, chain_id, 128).await?;

    let pool = db.pool.read().await.clone();
    sqlx::query("DELETE FROM host_listener_poller_state WHERE chain_id = $1")
        .bind(chain_id.as_i64())
        .execute(&pool)
        .await?;

    assert_eq!(db.poller_get_last_caught_up_block(chain_id).await?, None);

    db.poller_set_last_caught_up_block(chain_id, 5).await?;
    assert_eq!(db.poller_get_last_caught_up_block(chain_id).await?, Some(5));

    db.reconnect().await;
    db.poller_set_last_caught_up_block(chain_id, 7).await?;
    assert_eq!(db.poller_get_last_caught_up_block(chain_id).await?, Some(7));

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EventKind {
    Tfhe,
    Acl,
}

#[tokio::test]
#[serial(db)]
async fn poller_restart_repairs_omitted_acl_events_at_unchanged_safe_tip(
) -> Result<(), Box<dyn std::error::Error>> {
    let db_instance =
        test_harness::instance::setup_test_db(ImportMode::None).await?;
    let chain_id = ChainId::try_from(42_u64).unwrap();

    let db_url: DatabaseURL = db_instance.db_url.clone();
    let db = Database::new(&db_url, chain_id, 128).await?;
    let pool = db.pool.read().await.clone();
    sqlx::query("DELETE FROM host_listener_poller_state WHERE chain_id = $1")
        .bind(chain_id.as_i64())
        .execute(&pool)
        .await?;
    sqlx::query("DELETE FROM host_chain_blocks_valid WHERE chain_id = $1")
        .bind(chain_id.as_i64())
        .execute(&pool)
        .await?;
    sqlx::query("DELETE FROM computations")
        .execute(&pool)
        .await?;
    sqlx::query("DELETE FROM allowed_handles")
        .execute(&pool)
        .await?;

    // Spin up a local chain and emit events so the poller starts behind the head.
    let anvil = Anvil::new().chain_id(chain_id.as_u64()).spawn();
    let ws_url = anvil.ws_endpoint();
    let http_url = anvil.endpoint();

    let signer: PrivateKeySigner = anvil.first_key().clone().into();
    let wallet = EthereumWallet::new(signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet.clone())
        .connect_ws(WsConnect::new(ws_url.clone()))
        .await?;

    let tfhe_contract = RawLog::deploy(provider.clone()).await?;
    let acl_contract = RawLog::deploy(provider.clone()).await?;
    let kms_generation_contract = RawLog::deploy(provider.clone()).await?;
    let signer_address = provider
        .signer_addresses()
        .next()
        .expect("anvil provides at least one signer");

    let mut receipts: Vec<(u64, EventKind)> = Vec::new();
    for i in 0..3u64 {
        let mut handle = [0_u8; 32];
        handle[0] = (i + 1) as u8;
        handle[21] = 0xff;
        handle[22..30].copy_from_slice(&42_u64.to_be_bytes());
        handle[30] = 4;
        let tfhe_txn_req = emit_log_request(
            &tfhe_contract,
            TrivialEncrypt {
                caller: signer_address,
                pt: U256::from(i + 1),
                toType: 4,
                result: handle.into(),
            }
            .encode_log_data(),
        );
        let tfhe_receipt = provider
            .send_transaction(tfhe_txn_req)
            .await?
            .get_receipt()
            .await?;
        assert!(tfhe_receipt.status());
        receipts.push((
            tfhe_receipt
                .block_number
                .expect("trivialEncrypt block number"),
            EventKind::Tfhe,
        ));

        let acl_txn_req = allowed_request(
            &acl_contract,
            signer_address,
            signer_address,
            handle.into(),
        );
        let acl_receipt = provider
            .send_transaction(acl_txn_req)
            .await?
            .get_receipt()
            .await?;
        assert!(acl_receipt.status());
        receipts.push((
            acl_receipt.block_number.expect("allow block number"),
            EventKind::Acl,
        ));
    }

    let latest_block = provider.get_block_number().await?;
    let finality_lag = 2u64;
    let safe_tip = latest_block.saturating_sub(finality_lag);

    let expected_tfhe = receipts
        .iter()
        .filter(|(block, kind)| *block <= safe_tip && *kind == EventKind::Tfhe)
        .count() as i64;
    let expected_acl = receipts
        .iter()
        .filter(|(block, kind)| *block <= safe_tip && *kind == EventKind::Acl)
        .count() as i64;
    assert!(expected_tfhe > 0, "no finalized TFHE events to ingest");
    assert!(expected_acl > 0, "no finalized ACL events to ingest");

    let config = PollerConfig {
        replay_window: 64,
        url: http_url,
        // Model an omitted event family in the initial ingestion filter. The
        // restart restores the actual contract without changing chain history.
        acl_address: alloy::primitives::Address::ZERO,
        tfhe_address: *tfhe_contract.address(),
        kms_generation_address: Some(*kms_generation_contract.address()),
        protocol_config_address: Some(alloy::primitives::Address::ZERO),
        confidential_bridge_address: None,
        database_url: db_url.clone(),
        finality_lag,
        batch_size: 2,
        poll_interval: Duration::from_millis(200),
        retry_interval: Duration::from_millis(200),
        service_name: String::new(),
        max_http_retries: 0,
        rpc_compute_units_per_second: 1000,
        health_port: 0,
        seed_start_block: Some(0),
        dependence_cache_size: 10_000,
        dependence_by_connexity: false,
        dependence_cross_block: false,
        dependent_ops_max_per_chain: 0,
        gcs_mode: false,
        disable_synthetic_ops: false,
        canonical_protocol_config_chain_id: Some(chain_id.as_u64()),
    };

    let poller_handle = tokio::spawn(run_poller(config.clone()));

    // Wait for the poller to advance to the safe tip.
    let mut attempts = 0;
    loop {
        let anchor = sqlx::query_scalar::<_, i64>(
            "SELECT last_caught_up_block FROM host_listener_poller_state \
             WHERE chain_id = $1",
        )
        .bind(chain_id.as_i64())
        .fetch_optional(&pool)
        .await?;

        if anchor.map(|a| a as u64) == Some(safe_tip) {
            break;
        }

        attempts += 1;
        if attempts > 100 {
            poller_handle.abort();
            panic!(
                "host listener poller did not reach safe tip {safe_tip} (latest block \
                 {latest_block})"
            );
        }

        sleep(Duration::from_millis(100)).await;
    }

    // Allow the last ingest transaction to complete before stopping the task.
    sleep(Duration::from_millis(200)).await;
    poller_handle.abort();
    let _ = poller_handle.await;

    let computations_count =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM computations")
            .fetch_one(&pool)
            .await?;
    let allowed_count =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM allowed_handles")
            .fetch_one(&pool)
            .await?;
    let last_valid_block = sqlx::query_scalar::<_, Option<i64>>(
        "SELECT MAX(block_number) FROM host_chain_blocks_valid \
         WHERE chain_id = $1",
    )
    .bind(chain_id.as_i64())
    .fetch_one(&pool)
    .await?
    .unwrap_or_default();

    assert_eq!(computations_count, expected_tfhe);
    assert_eq!(allowed_count, 0, "initial ingestion omitted all ACL events");
    let mut repaired = config;
    repaired.acl_address = *acl_contract.address();
    let restart = tokio::spawn(run_poller(repaired));
    tokio::time::timeout(Duration::from_secs(20), async {
        loop {
            assert!(!restart.is_finished(), "restart must remain live");
            let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM allowed_handles").fetch_one(&pool).await?;
            let replay_done: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM public.listener_replay_state WHERE chain_id=42 AND NOT pending)")
                .fetch_one(&pool).await?;
            if count == expected_acl && replay_done { break; }
            sleep(Duration::from_millis(50)).await;
        }
        Ok::<_,sqlx::Error>(())
    }).await??;
    restart.abort();
    let _ = restart.await;
    let cursor: i64 = sqlx::query_scalar("SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id=42").fetch_one(&pool).await?;
    assert_eq!(
        cursor as u64, safe_tip,
        "repair must not rewind durable progress"
    );
    assert!(!fhevm_engine_common::chain_recovery::is_halted(&pool).await?);
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM computations")
            .fetch_one(&pool)
            .await?,
        expected_tfhe,
        "replay must not duplicate computations"
    );
    assert_eq!(last_valid_block as u64, safe_tip);

    Ok(())
}
