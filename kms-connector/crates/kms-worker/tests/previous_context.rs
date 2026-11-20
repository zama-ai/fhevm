use alloy::primitives::U256;
use connector_utils::{
    tests::setup::{BlockchainInstance, define_kms_context_v014, wait_for_finalized},
    types::{KmsGrpcRequest, u256_to_request_id},
};
use fhevm_host_bindings::protocol_config::ProtocolConfig::NewKmsContext;
use kms_worker::core::{Config, event_processor::ProtocolConfigProcessor};
use rstest::rstest;
use std::time::Duration;

#[rstest]
#[timeout(Duration::from_secs(90))]
#[tokio::test]
async fn previous_context_defined_before_connector_url_is_verified_and_used() -> anyhow::Result<()>
{
    let blockchain = BlockchainInstance::setup().await?;
    let (previous, block) = define_kms_context_v014(&blockchain.protocol_config_contract).await?;
    wait_for_finalized(&blockchain.provider, block).await?;

    let mut config = Config::default();
    config.protocol_config_contract.address = *blockchain.protocol_config_contract.address();
    let processor = ProtocolConfigProcessor::new(&config, blockchain.provider.clone());
    let event = NewKmsContext {
        contextId: previous.contextId + U256::ONE,
        previousContextId: previous.contextId,
        ..Default::default()
    };

    let KmsGrpcRequest::NewMpcContext { old, .. } =
        processor.prepare_new_kms_context_request(&event).await?
    else {
        panic!("expected a NewMpcContext request");
    };

    let old_context = old.new_context.expect("old request has a context");
    assert_eq!(
        old_context.context_id,
        Some(u256_to_request_id(previous.contextId))
    );
    assert_eq!(old_context.software_version, previous.softwareVersion);
    let node = &old_context.mpc_nodes[0];
    let previous_node = &previous.kmsNodeParams[0];
    assert_eq!(node.mpc_identity, previous_node.mpcIdentity);
    assert_eq!(node.party_id, previous_node.partyId);
    assert_eq!(node.external_url, previous_node.ipAddress);
    assert_eq!(node.ca_cert, Some(previous_node.caCert.to_vec()));
    Ok(())
}
