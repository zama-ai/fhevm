use alloy::{
    hex,
    primitives::{Address, FixedBytes, U256},
    providers::{Provider, mock::Asserter},
    rpc::types::Transaction,
    sol_types::{SolCall, SolValue},
};
use connector_utils::tests::{
    rand::rand_address,
    setup::{S3_CT_BUCKET, s3_ct_attestation_signer},
};
use fhevm_gateway_bindings::{
    decryption::Decryption::{
        CtHandleContractPair, userDecryptionRequest_1Call as userDecryptionRequestCall,
    },
    gateway_config::GatewayConfig::Coprocessor,
};
use fhevm_host_bindings::acl::ACL::ACLInstance;
use kms_worker::core::{
    Config, DbEventPicker, DbKmsResponsePublisher, KmsWorker,
    event_processor::{
        CiphertextManager, DbContextManager, DbEventProcessor, DecryptionProcessor, HostRpcClient,
        KMSGenerationProcessor, KmsClient, ProtocolConfigProcessor,
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
        s3BucketUrl: format!("{s3_url}/{S3_CT_BUCKET}"),
        ..Default::default()
    };
    asserter.push_success(&coprocessor.abi_encode());
    copro_tx_sender
}

pub async fn init_kms_worker<P>(
    config: Config,
    provider: P,
    acl_contracts_mock: HashMap<u64, ACLInstance<P>>,
    db: &Pool<Postgres>,
) -> anyhow::Result<KmsWorker<DbEventPicker, DbEventProcessor<P, P, DbContextManager<P>>>>
where
    P: Provider + Clone + 'static,
{
    let ciphertext_manager =
        CiphertextManager::connect(provider.clone(), &config, CancellationToken::new()).await?;

    let kms_client = KmsClient::connect(&config).await?;
    let event_picker = DbEventPicker::connect(db.clone(), &config).await?;

    let context_manager = DbContextManager::connect(
        db.clone(),
        &config,
        provider.clone(),
        CancellationToken::new(),
    )
    .await?;
    let host_clients = acl_contracts_mock
        .into_iter()
        .map(|(chain_id, acl)| (chain_id, HostRpcClient::new(chain_id, acl)))
        .collect();
    let decryption_processor =
        DecryptionProcessor::new(&config, provider.clone(), host_clients, ciphertext_manager);
    let kms_generation_processor = KMSGenerationProcessor::new(&config);
    let protocol_config_processor = ProtocolConfigProcessor::new(&config, provider.clone());
    let event_processor = DbEventProcessor::new(
        kms_client.clone(),
        context_manager,
        decryption_processor,
        kms_generation_processor,
        protocol_config_processor,
        config.max_decryption_attempts,
        db.clone(),
    );
    let response_publisher = DbKmsResponsePublisher::new(db.clone());
    let kms_worker = KmsWorker::new(event_picker, event_processor, response_publisher);
    Ok(kms_worker)
}

/// Registry refresh interval used by the tests: long enough that the refresh task never fires.
pub const TEST_COPRO_REGISTRY_REFRESH: Duration = Duration::from_hours(24);

/// Context cache refresh interval used by the tests: long enough that the refresh task never
/// fires, so validations conclude from the initial snapshot and the on-chain fallback alone.
pub const TEST_KMS_CONTEXT_CACHE_REFRESH: Duration = Duration::from_hours(24);

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

    // Mock get_transaction_by_hash response. `to` is the default Decryption contract address so
    // the direct-target hardening check in `fetch_calldata` passes.
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
