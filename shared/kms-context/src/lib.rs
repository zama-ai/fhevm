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

/// Mirrors `KMS_CONTEXT_COUNTER_BASE` from `host-contracts/contracts/shared/Constants.sol`:
/// `0x07 << 248`, the top byte of the highest limb.
pub const KMS_CONTEXT_COUNTER_BASE: U256 = U256::from_limbs([0, 0, 0, 0x07 << 56]);

#[derive(Debug, thiserror::Error)]
pub enum KmsContextReadError {
    #[error("getKmsContextAnchor({context_id})")]
    Anchor {
        context_id: U256,
        #[source]
        source: Box<alloy::contract::Error>,
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
        .map_err(|source| KmsContextReadError::Anchor {
            context_id,
            source: Box::new(source),
        })?;
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
        primitives::{Address, Bytes, Log as PrimitiveLog, b256, bytes},
        providers::{ProviderBuilder, RootProvider, mock::Asserter},
        rpc::types::Log,
        transports::TransportResult,
    };
    use fhevm_host_bindings::protocol_config::{
        IProtocolConfig::KmsThresholds,
        ProtocolConfig::{self, KmsNodeParams, PcrValues},
    };

    const CONTRACT: Address = Address::repeat_byte(0xC0);
    const ANCHOR_BLOCK: u64 = 20;

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
            block_number: Some(ANCHOR_BLOCK),
            block_hash: Some(B256::repeat_byte(0x20)),
            ..Default::default()
        }
    }

    /// A node holding `logs`: it answers `eth_getLogs` with the ones the request's filter
    /// selects, as a node does, and every other call from the mocked client.
    struct Node {
        root: RootProvider,
        logs: Vec<Log>,
    }

    #[async_trait::async_trait]
    impl Provider for Node {
        fn root(&self) -> &RootProvider {
            &self.root
        }

        async fn get_logs(&self, filter: &Filter) -> TransportResult<Vec<Log>> {
            Ok(self
                .logs
                .iter()
                .filter(|log| filter.rpc_matches(log))
                .cloned()
                .collect())
        }
    }

    /// Reads context 7 from a node whose anchor names [`ANCHOR_BLOCK`] and `anchored`.
    async fn read(anchored: B256, logs: Vec<Log>) -> Result<NewKmsContext, KmsContextReadError> {
        let asserter = Asserter::new();
        asserter.push_success(&Bytes::from(
            (U256::from(ANCHOR_BLOCK), anchored).abi_encode_sequence(),
        ));
        let root = ProviderBuilder::default().connect_mocked_client(asserter);
        read_kms_context(
            &ProtocolConfig::new(CONTRACT, Node { root, logs }),
            context_id(),
        )
        .await
    }

    #[tokio::test]
    async fn reads_the_event_its_anchor_commits_to() {
        let expected = event(Address::repeat_byte(0xA1));
        let read = read(context_info_hash(&expected), vec![log(&expected)]).await;
        assert_eq!(read.expect("read"), expected);
    }

    /// Another context's event in the anchor block, as when governance destroys a Pending
    /// context and defines a new one in the same block, is not this context's.
    #[tokio::test]
    async fn reads_its_own_event_among_other_contexts_at_the_anchor_block() {
        let expected = event(Address::repeat_byte(0xA1));
        let other = NewKmsContext {
            contextId: U256::from(6),
            ..event(Address::repeat_byte(0xB2))
        };
        let read = read(
            context_info_hash(&expected),
            vec![log(&other), log(&expected)],
        )
        .await;
        assert_eq!(read.expect("read"), expected);
    }

    #[tokio::test]
    async fn refuses_an_anchor_without_its_event() {
        assert!(matches!(
            read(B256::ZERO, Vec::new()).await,
            Err(KmsContextReadError::EventCount { count: 0, .. })
        ));
    }

    #[tokio::test]
    async fn refuses_two_events_at_the_anchor_block() {
        let first = event(Address::repeat_byte(0xA1));
        let second = event(Address::repeat_byte(0xB2));
        assert!(matches!(
            read(context_info_hash(&first), vec![log(&first), log(&second)]).await,
            Err(KmsContextReadError::EventCount { count: 2, .. })
        ));
    }

    #[tokio::test]
    async fn refuses_an_event_its_anchor_does_not_commit_to() {
        let anchored = event(Address::repeat_byte(0xA1));
        let emitted = event(Address::repeat_byte(0xB2));
        assert!(matches!(
            read(context_info_hash(&anchored), vec![log(&emitted)]).await,
            Err(KmsContextReadError::HashMismatch { .. })
        ));
    }

    /// The expected hash is Foundry's, not this crate's: `cast keccak` of
    /// `cast abi-encode "f((address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])"
    /// "[(0xa1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1,0xb2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2,10.0.0.1,s3://kms,1,kms-1,0xcafe,p1)]"
    /// "(1,2,3,4)" "v0.13.0" "[(0x01,0x02,0x03)]"`, with cast 1.7.1.
    #[test]
    fn context_info_hash_is_the_solidity_abi_encoding() {
        let event = NewKmsContext {
            contextId: context_id(),
            previousContextId: U256::from(6),
            kmsNodeParams: vec![KmsNodeParams {
                txSenderAddress: Address::repeat_byte(0xA1),
                signerAddress: Address::repeat_byte(0xB2),
                ipAddress: "10.0.0.1".into(),
                storageUrl: "s3://kms".into(),
                partyId: 1,
                mpcIdentity: "kms-1".into(),
                caCert: bytes!("cafe"),
                storagePrefix: "p1".into(),
            }],
            thresholds: KmsThresholds {
                publicDecryption: U256::from(1),
                userDecryption: U256::from(2),
                kmsGen: U256::from(3),
                mpc: U256::from(4),
            },
            softwareVersion: "v0.13.0".into(),
            pcrValues: vec![PcrValues {
                pcr0: bytes!("01"),
                pcr1: bytes!("02"),
                pcr2: bytes!("03"),
            }],
        };
        assert_eq!(
            context_info_hash(&event),
            b256!("0x0cdf51f41c1e7d35d7f8f252e70333ad756c3b345e027a42a6c9292ff1ad612e")
        );
    }
}
