use alloy::{
    hex,
    primitives::{Address, FixedBytes, U256},
    providers::{Provider, mock::Asserter},
    rpc::types::Transaction,
    sol_types::{SolCall, SolValue},
    transports::http::reqwest,
};
use connector_utils::tests::{
    rand::rand_address,
    setup::{S3_CT_RFC023_BUCKET, s3_ct_attestation_signer},
};
use fhevm_gateway_bindings::{
    decryption::Decryption::{
        CtHandleContractPair, userDecryptionRequest_1Call as userDecryptionRequestCall,
    },
    gateway_config::GatewayConfig::Coprocessor,
};
use fhevm_host_bindings::acl::ACL::ACLInstance;
use kms_worker::core::{
    Config, CtAttestationConfig, DbEventPicker, DbKmsResponsePublisher, KmsWorker,
    event_processor::{
        CiphertextManager, DbContextManager, DbEventProcessor, DecryptionProcessor,
        KMSGenerationProcessor, KmsClient,
    },
};
use sqlx::{Pool, Postgres};
use std::{collections::HashMap, time::Duration};
use tokio_util::sync::CancellationToken;

/// Mocks the Gateway RPC responses for the initial Coprocessor registry load of the
/// `CiphertextManager`.
pub fn mock_copro_registry_load(asserter: &Asserter, s3_url: &str) -> Address {
    let copro_tx_sender = rand_address();
    asserter.push_success(&vec![s3_ct_attestation_signer().address()].abi_encode());
    asserter.push_success(&vec![copro_tx_sender].abi_encode());
    asserter.push_success(&U256::ONE.abi_encode());
    let coprocessor = Coprocessor {
        s3BucketUrl: format!("{s3_url}/{S3_CT_RFC023_BUCKET}"),
        ..Default::default()
    };
    asserter.push_success(&coprocessor.abi_encode());
    copro_tx_sender
}

pub async fn init_kms_worker<GP, HP>(
    config: Config,
    gateway_provider: GP,
    acl_contracts_mock: HashMap<u64, ACLInstance<HP>>,
    db: &Pool<Postgres>,
) -> anyhow::Result<KmsWorker<DbEventPicker, DbEventProcessor<GP, HP, DbContextManager>>>
where
    GP: Provider + Clone + 'static,
    HP: Provider + Clone + 'static,
{
    // The 24h refresh interval (see `testing_ct_attestation_config`) means the refresh task
    // never fires during a test, so a throwaway token is fine here.
    let ciphertext_manager = CiphertextManager::connect(
        gateway_provider.clone(),
        reqwest::Client::new(),
        &config,
        CancellationToken::new(),
    )
    .await?;

    let kms_client = KmsClient::connect(&config).await?;
    let event_picker = DbEventPicker::connect(db.clone(), &config).await?;

    let context_manager = DbContextManager::new(db.clone());
    let decryption_processor = DecryptionProcessor::new(
        &config,
        context_manager.clone(),
        gateway_provider.clone(),
        acl_contracts_mock,
        ciphertext_manager,
    );
    let kms_generation_processor =
        KMSGenerationProcessor::new(&config, context_manager, db.clone());
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

pub fn testing_ct_attestation_config(enabled: bool) -> CtAttestationConfig {
    CtAttestationConfig {
        enabled,
        registry_refresh: Duration::from_hours(24), // Avoid refreshing the registry during test
        ..Default::default()
    }
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
