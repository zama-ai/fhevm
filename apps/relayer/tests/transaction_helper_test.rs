mod common;

use alloy::primitives::{Address, Bytes, FixedBytes, U256};
use fhevm_relayer::core::job_id::JobId;
use fhevm_relayer::gateway::arbitrum::transaction::helper::GatewayTransactionEngine;
use std::str::FromStr;

#[tokio::test]
async fn test_tx_helper() {
    let setup = common::utils::TestSetup::new()
        .await
        .expect("Failed to create test setup");

    // let node_rpc_url = &setup.settings.gateway.blockchain_rpc.http_url;
    // let private_key = std::env::var("TEST_PRIVATE_KEY").unwrap_or_else(|_| {
    //     "34aacca926bab195601bcf5702786d35cab968159b718ae671b226de11b9afee".to_string()
    // });
    // let mut local_signer: PrivateKeySigner = private_key.parse().unwrap();
    // local_signer.set_chain_id(Some(123456));

    let tx_service = GatewayTransactionEngine::new(
        setup.settings.gateway.blockchain_rpc.clone(),
        setup.settings.gateway.tx_engine.clone(),
    );

    // let tx_config = TxConfig::default();
    // let tx_helper = TransactionHelper::new(tx_service, tx_config);
    // println!("{:?}", tx_helper);

    // Now let's do some stuff with this tx-helper
    // What we should test:
    // - catch nonce issue
    // - catch revert
    // - catch stuck tx
    // To do so we could probably create another wallet with no protections that would emit
    // failing transactions
    //

    setup.shutdown().await;

    let random_target_address =
        Address::from_str("0xB8Ae44365c45A7C5256b14F607CaE23BC040c354").unwrap();

    // TODO: return tx-nonce used
    // Most generally we should return information about the tx itself
    // since lots of important stuff will come from the provider fillers
    let tx_request = match tx_service
        .prepare_transaction(
            &JobId::ZERO,
            random_target_address,
            Bytes::from(FixedBytes::from(U256::ZERO)),
            None,
        )
        .await
    {
        Ok(rec) => rec,
        Err(_error) => return,
    };

    let result = tx_service
        .send_raw_transaction_sync_with_retries(&JobId::ZERO, tx_request)
        .await;

    println!(
        "Canceling transaction from {:?} resulted in {:?}",
        random_target_address, result
    );
}
