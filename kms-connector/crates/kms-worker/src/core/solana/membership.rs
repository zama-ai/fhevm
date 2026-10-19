//! Whether this connector's node is a node of a request's KMS context, read from `ProtocolConfig`
//! on Ethereum (Solana DD-076). A coprocessor answers a tx-sender it does not know with a retryable
//! error, so a node outside the request's context is refused here, before any proof read.

use crate::core::event_processor::{RequestCheckError, RequestCheckKind};
use alloy::{
    eips::BlockId,
    primitives::{Address, U256},
    providers::DynProvider,
};
use anyhow::anyhow;
use fhevm_host_bindings::protocol_config::ProtocolConfig::{self, ProtocolConfigInstance};
use kms_connector_api::ErrorCode;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

/// Reads whether a tx-sender is a node of a KMS context, and keeps both answers: a context's nodes
/// are set when it is created and never change.
#[derive(Clone)]
pub struct KmsContextMembers {
    protocol_config: ProtocolConfigInstance<DynProvider>,
    answers: Arc<Mutex<HashMap<(U256, Address), bool>>>,
}

impl KmsContextMembers {
    /// Reads the `ProtocolConfig` at `protocol_config` through `ethereum_provider`.
    pub fn new(protocol_config: Address, ethereum_provider: DynProvider) -> Self {
        Self {
            protocol_config: ProtocolConfig::new(protocol_config, ethereum_provider),
            answers: Arc::default(),
        }
    }

    /// Refuses for good a `tx_sender` that is not a node of `context_id` at the finalized block. A
    /// failed read is recoverable, and nothing is kept from it: the request's next attempt reads
    /// again.
    pub async fn check(
        &self,
        context_id: U256,
        tx_sender: Address,
    ) -> Result<(), RequestCheckError> {
        let kept = self
            .answers
            .lock()
            .unwrap()
            .get(&(context_id, tx_sender))
            .copied();
        let is_member = match kept {
            Some(is_member) => is_member,
            None => {
                let is_member = self
                    .protocol_config
                    .isKmsTxSenderForContext(context_id, tx_sender)
                    .block(BlockId::finalized())
                    .call()
                    .await
                    .map_err(|e| {
                        RequestCheckError::network(anyhow!(
                            "isKmsTxSenderForContext(#{context_id:#066x}, {tx_sender}) call failed: {e}"
                        ))
                    })?;
                self.answers
                    .lock()
                    .unwrap()
                    .insert((context_id, tx_sender), is_member);
                is_member
            }
        };
        if !is_member {
            return Err(RequestCheckError::irrecoverable(
                RequestCheckKind::KmsContext,
                ErrorCode::Unprocessable,
                anyhow!("{tx_sender} is not a node of KMS context #{context_id:#066x}"),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event_processor::ProcessingErrorKind;
    use alloy::{
        providers::{Provider, ProviderBuilder, mock::Asserter},
        sol_types::SolValue,
    };

    fn members(asserter: Asserter) -> KmsContextMembers {
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        KmsContextMembers::new(Address::repeat_byte(0xC0), provider.erased())
    }

    /// A tx-sender that is not a node of the context is refused for good as `unprocessable`: the
    /// node holds no share for the request.
    #[tokio::test]
    async fn a_node_outside_the_context_is_refused_for_good() {
        let asserter = Asserter::new();
        asserter.push_success(&false.abi_encode());

        let error = members(asserter)
            .check(U256::from(33), Address::repeat_byte(0x01))
            .await
            .unwrap_err()
            .record();

        assert_eq!(
            (error.kind, error.code),
            (ProcessingErrorKind::Irrecoverable, ErrorCode::Unprocessable),
            "{error}"
        );
    }

    /// Both answers are read once per context and tx-sender: the asserter holds one answer for
    /// each, so a second read of either would fail.
    #[tokio::test]
    async fn each_answer_is_read_once() {
        let asserter = Asserter::new();
        asserter.push_success(&true.abi_encode()); // (33, member)
        asserter.push_success(&false.abi_encode()); // (33, outsider)
        let members = members(asserter);

        let context_id = U256::from(33);
        for _ in 0..2 {
            members
                .check(context_id, Address::repeat_byte(0x01))
                .await
                .unwrap();
            let error = members
                .check(context_id, Address::repeat_byte(0x02))
                .await
                .unwrap_err()
                .record();
            assert_eq!(error.kind, ProcessingErrorKind::Irrecoverable, "{error}");
        }
    }

    /// A failed read is recoverable, and nothing is kept: the next check reads again.
    #[tokio::test]
    async fn a_failed_read_is_retried() {
        let asserter = Asserter::new();
        asserter.push_failure_msg("execution reverted");
        asserter.push_success(&true.abi_encode());
        let members = members(asserter);

        let (context_id, tx_sender) = (U256::from(33), Address::repeat_byte(0x01));
        let error = members
            .check(context_id, tx_sender)
            .await
            .unwrap_err()
            .record();
        assert_eq!(
            (error.kind, error.code),
            (
                ProcessingErrorKind::Recoverable,
                ErrorCode::UpstreamTransient
            ),
            "{error}"
        );
        members.check(context_id, tx_sender).await.unwrap();
    }
}
