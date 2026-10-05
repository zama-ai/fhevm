//! Reads a KMS context's `NewKmsContext` event from the canonical `ProtocolConfig`. The event is the
//! only record of a context's nodes, thresholds and software; `getKmsContextAnchor` names the block
//! that emitted it and the hash it must match.

use alloy::{
    eips::BlockId,
    primitives::{B256, U256, keccak256},
    providers::Provider,
    rpc::types::Filter,
    sol_types::{SolEvent, SolValue},
    transports::TransportError,
};
use fhevm_host_bindings::protocol_config::ProtocolConfig::{NewKmsContext, ProtocolConfigInstance};

#[derive(Debug, thiserror::Error)]
pub enum KmsContextReadError {
    #[error("getKmsContextAnchor({context_id})")]
    Anchor {
        context_id: U256,
        #[source]
        source: alloy::contract::Error,
    },
    #[error("NewKmsContext({context_id}) logs at block {block}")]
    Logs {
        context_id: U256,
        block: u64,
        #[source]
        source: TransportError,
    },
    #[error("{count} NewKmsContext({context_id}) events at its anchor block {block}")]
    EventCount {
        context_id: U256,
        block: u64,
        count: usize,
    },
    #[error("decode NewKmsContext({context_id})")]
    Decode {
        context_id: U256,
        #[source]
        source: alloy::sol_types::Error,
    },
    #[error("NewKmsContext({context_id}) hashes to {computed}, its anchor commits to {anchored}")]
    HashMismatch {
        context_id: U256,
        computed: B256,
        anchored: B256,
    },
}

/// The context's `NewKmsContext` event, read at the block its anchor names and checked against the
/// anchor's `contextInfoHash`. The anchor is read at the finalized block. Unlike
/// `getKmsNodesForContext`, this also reads a Pending or Created context.
pub async fn read_kms_context<P: Provider>(
    protocol_config: &ProtocolConfigInstance<P>,
    context_id: U256,
) -> Result<NewKmsContext, KmsContextReadError> {
    let anchor = protocol_config
        .getKmsContextAnchor(context_id)
        .block(BlockId::finalized())
        .call()
        .await
        .map_err(|source| KmsContextReadError::Anchor { context_id, source })?;
    let block: u64 = anchor.emissionBlockNumber.saturating_to();
    let filter = Filter::new()
        .address(*protocol_config.address())
        .event_signature(NewKmsContext::SIGNATURE_HASH)
        .topic1(B256::from(context_id))
        .from_block(block)
        .to_block(block);
    let logs = protocol_config
        .provider()
        .get_logs(&filter)
        .await
        .map_err(|source| KmsContextReadError::Logs {
            context_id,
            block,
            source,
        })?;
    let [log] = logs.as_slice() else {
        return Err(KmsContextReadError::EventCount {
            context_id,
            block,
            count: logs.len(),
        });
    };
    let event = NewKmsContext::decode_log(&log.inner)
        .map_err(|source| KmsContextReadError::Decode { context_id, source })?
        .data;
    let computed = context_info_hash(&event);
    if computed != anchor.contextInfoHash {
        return Err(KmsContextReadError::HashMismatch {
            context_id,
            computed,
            anchored: anchor.contextInfoHash,
        });
    }
    Ok(event)
}

/// The `contextInfoHash` `ProtocolConfig` stores in a context's anchor:
/// `keccak256(abi.encode(kmsNodeParams, thresholds, softwareVersion, pcrValues))`.
pub fn context_info_hash(event: &NewKmsContext) -> B256 {
    keccak256(
        (
            event.kmsNodeParams.as_slice(),
            event.thresholds.clone(),
            &event.softwareVersion,
            event.pcrValues.as_slice(),
        )
            .abi_encode_sequence(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::{Address, Bytes, Log as PrimitiveLog},
        providers::{ProviderBuilder, mock::Asserter},
        rpc::types::Log,
    };
    use fhevm_host_bindings::protocol_config::ProtocolConfig::{self, KmsNodeParams};

    const CONTRACT: Address = Address::repeat_byte(0xC0);

    fn context_id() -> U256 {
        U256::from(7)
    }

    fn event(tx_sender: Address) -> NewKmsContext {
        NewKmsContext {
            contextId: context_id(),
            kmsNodeParams: vec![KmsNodeParams {
                txSenderAddress: tx_sender,
                ..Default::default()
            }],
            ..Default::default()
        }
    }

    fn log(event: &NewKmsContext) -> Log {
        Log {
            inner: PrimitiveLog {
                address: CONTRACT,
                data: event.encode_log_data(),
            },
            ..Default::default()
        }
    }

    fn anchor(asserter: &Asserter, hash: B256) {
        asserter.push_success(&Bytes::from((U256::from(20), hash).abi_encode_sequence()));
    }

    async fn read(asserter: &Asserter) -> Result<NewKmsContext, KmsContextReadError> {
        let provider = ProviderBuilder::new().connect_mocked_client(asserter.clone());
        read_kms_context(&ProtocolConfig::new(CONTRACT, provider), context_id()).await
    }

    #[tokio::test]
    async fn reads_the_event_its_anchor_commits_to() {
        let expected = event(Address::repeat_byte(0xA1));
        let asserter = Asserter::new();
        anchor(&asserter, context_info_hash(&expected));
        asserter.push_success(&vec![log(&expected)]);
        assert_eq!(read(&asserter).await.expect("read"), expected);
    }

    #[tokio::test]
    async fn refuses_an_anchor_without_its_event() {
        let asserter = Asserter::new();
        anchor(&asserter, B256::ZERO);
        asserter.push_success(&Vec::<Log>::new());
        assert!(matches!(
            read(&asserter).await,
            Err(KmsContextReadError::EventCount { count: 0, .. })
        ));
    }

    #[tokio::test]
    async fn refuses_two_events_at_the_anchor_block() {
        let first = event(Address::repeat_byte(0xA1));
        let asserter = Asserter::new();
        anchor(&asserter, context_info_hash(&first));
        asserter.push_success(&vec![log(&first), log(&event(Address::repeat_byte(0xB2)))]);
        assert!(matches!(
            read(&asserter).await,
            Err(KmsContextReadError::EventCount { count: 2, .. })
        ));
    }

    #[tokio::test]
    async fn refuses_an_event_its_anchor_does_not_commit_to() {
        let anchored = event(Address::repeat_byte(0xA1));
        let asserter = Asserter::new();
        anchor(&asserter, context_info_hash(&anchored));
        asserter.push_success(&vec![log(&event(Address::repeat_byte(0xB2)))]);
        assert!(matches!(
            read(&asserter).await,
            Err(KmsContextReadError::HashMismatch { .. })
        ));
    }
}
