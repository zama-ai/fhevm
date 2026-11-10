use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use alloy::primitives::U256;
use alloy::providers::{Provider, ProviderBuilder, WalletProvider, WsConnect};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol_types::SolEvent;
use host_listener::contracts::TfheContract::TrivialEncrypt;
use serial_test::serial;
use std::time::Duration;
use test_harness::instance::ImportMode;
mod common;
use common::{emit_log_request, RawLog};

/// Exercise the deployed websocket entry point, not just its admission helper.
/// A malformed known event must terminate it before a later valid block lands.
#[tokio::test]
#[serial(db)]
async fn websocket_rejected_block_exits_without_advancing_past_it(
) -> anyhow::Result<()> {
    use clap::Parser;
    use host_listener::cmd::{main, Args};
    let instance = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    let pool = sqlx::PgPool::connect(instance.db_url()).await?;
    let anvil = Anvil::new().chain_id(42).spawn();
    let signer: PrivateKeySigner = anvil.first_key().clone().into();
    let provider = ProviderBuilder::new()
        .wallet(EthereumWallet::new(signer))
        .connect_ws(WsConnect::new(anvil.ws_endpoint()))
        .await?;
    let emitter = RawLog::deploy(provider.clone()).await?;
    let caller = provider.signer_addresses().next().unwrap();
    let bad = provider
        .send_transaction(emit_log_request(
            &emitter,
            alloy::primitives::LogData::new_unchecked(
                vec![TrivialEncrypt::SIGNATURE_HASH],
                Default::default(),
            ),
        ))
        .await?
        .get_receipt()
        .await?;
    let mut handle = [0_u8; 32];
    handle[0] = 7;
    handle[21] = 0xff;
    handle[22..30].copy_from_slice(&42_u64.to_be_bytes());
    handle[30] = 4;
    let good = provider
        .send_transaction(emit_log_request(
            &emitter,
            TrivialEncrypt {
                caller,
                pt: U256::from(2),
                toType: 4,
                result: handle.into(),
            }
            .encode_log_data(),
        ))
        .await?
        .get_receipt()
        .await?;
    let mut args = Args::try_parse_from([
        "host-listener",
        "--acl-contract-address",
        &alloy::primitives::Address::ZERO.to_string(),
        "--tfhe-contract-address",
        &emitter.address().to_string(),
    ])?;
    args.url = anvil.ws_endpoint();
    args.database_url = instance.db_url.clone();
    args.start_at_block = Some(0);
    args.end_at_block = Some(good.block_number.unwrap() as i64);
    args.catchup_margin = 0;
    args.catchup_finalization_in_blocks = 0;
    args.health_port = 0;
    args.disable_synthetic_ops = true;
    let result =
        tokio::time::timeout(Duration::from_secs(15), main(args)).await?;
    let error = result
        .expect_err("websocket must propagate rejection instead of skipping");
    assert!(error.to_string().contains("Rejected block"), "{error:#}");
    let latest: Option<i64> = sqlx::query_scalar("SELECT MAX(block_number) FROM host_chain_blocks_valid WHERE chain_id=42")
        .fetch_one(&pool).await?;
    assert!(latest.is_none_or(|n| n < bad.block_number.unwrap() as i64));
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM computations")
            .fetch_one(&pool)
            .await?,
        0
    );
    Ok(())
}
