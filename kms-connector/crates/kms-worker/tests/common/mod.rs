use alloy::{
    hex, primitives::FixedBytes, providers::Provider, rpc::types::Transaction, sol_types::SolCall,
    transports::http::reqwest,
};
use connector_utils::tests::rand::rand_address;
use fhevm_gateway_bindings::decryption::Decryption::{
    CtHandleContractPair, userDecryptionRequestCall,
};
use fhevm_host_bindings::acl::ACL::ACLInstance;
use kms_worker::core::{
    Config, DbEventPicker, DbKmsResponsePublisher, KmsWorker,
    event_processor::{
        DbEventProcessor, DecryptionProcessor, KMSGenerationProcessor, KmsClient, s3::S3Service,
    },
};
use sqlx::{Pool, Postgres};
use std::collections::HashMap;

pub async fn init_kms_worker<GP: Provider + Clone + 'static, HP: Provider + Clone + 'static>(
    config: Config,
    gateway_provider: GP,
    acl_contracts_mock: HashMap<u64, ACLInstance<HP>>,
    db: &Pool<Postgres>,
) -> anyhow::Result<KmsWorker<DbEventPicker, DbEventProcessor<GP, HP>>> {
    let kms_client = KmsClient::connect(&config).await?;
    let s3_client = reqwest::Client::new();
    let event_picker = DbEventPicker::connect(db.clone(), &config).await?;

    let s3_service = S3Service::new(&config, gateway_provider.clone(), s3_client);
    let decryption_processor = DecryptionProcessor::new(
        &config,
        gateway_provider.clone(),
        acl_contracts_mock,
        s3_service,
    );
    let kms_generation_processor = KMSGenerationProcessor::new(&config);
    let event_processor = DbEventProcessor::new(
        kms_client.clone(),
        decryption_processor,
        kms_generation_processor,
        config.max_decryption_attempts,
        db.clone(),
    );
    let response_publisher = DbKmsResponsePublisher::new(db.clone());
    let kms_worker = KmsWorker::new(event_picker, event_processor, response_publisher);
    Ok(kms_worker)
}

pub fn create_mock_user_decryption_request_tx(
    tx_hash: FixedBytes<32>,
    handle: FixedBytes<32>,
) -> Result<Transaction, serde_json::Error> {
    // Create the calldata for the userDecryptionRequest
    let calldata = userDecryptionRequestCall {
        ctHandleContractPairs: vec![CtHandleContractPair {
            ctHandle: handle,
            contractAddress: rand_address(),
        }],
        ..Default::default()
    }
    .abi_encode();

    // Mock get_transaction_by_hash response
    serde_json::from_value(serde_json::json!({
        "hash": hex::encode(tx_hash.as_slice()),
        "nonce": "0x0",
        "blockHash": null,
        "blockNumber": null,
        "transactionIndex": null,
        "from": "0x0000000000000000000000000000000000000000",
        "to": "0x0000000000000000000000000000000000000000",
        "value": "0x0",
        "gasPrice": "0x0",
        "gas": "0x0",
        "input": format!("0x{}", alloy::hex::encode(&calldata)),
        "v": "0x1b",
        "r": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "s": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "type": "0x0"
    }))
}
