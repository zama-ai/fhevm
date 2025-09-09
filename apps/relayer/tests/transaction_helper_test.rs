use std::sync::Arc;
use alloy::primitives::{Bytes, FixedBytes, U256};
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::Signer;

#[tokio::test]
async fn test_tx_helper() {
    use fhevm_relayer::transaction::{TransactionService, TxConfig};
    
    let node_rpc_url = "ws://localhost:8756";
    let private_key = std::env::var("TEST_PRIVATE_KEY").unwrap_or_else(|_| {
        "34aacca926bab195601bcf5702786d35cab968159b718ae671b226de11b9afee".to_string()
    });
    let mut local_signer: PrivateKeySigner = private_key.parse().unwrap();
    local_signer.set_chain_id(Some(123456));

    let tx_service = TransactionService::new(node_rpc_url, Arc::new(local_signer.clone()))
        .await
        .unwrap();

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

    // TODO: return tx-nonce used
    // Most generally we should return information about the tx itself
    // since lots of important stuff will come from the provider fillers
    let result = tx_service
        .submit_and_wait(
            local_signer.address(),
            Bytes::from(FixedBytes::from(U256::ZERO)),
            TxConfig::default(),
        )
        .await;

    println!(
        "Canceling transaction from {:?} resulted in {:?}",
        local_signer.address(),
        result
    );
}