//! Resolves the coprocessors of one upgrade attempt from the on-chain
//! `GatewayConfig` contract, at the block that the attempt names.

use alloy::{eips::BlockId, network::Ethereum, primitives::Address, providers::Provider};
use anyhow::anyhow;
use fhevm_gateway_bindings::gateway_config::GatewayConfig::{self, GatewayConfigInstance};

/// Reader for the coprocessor registrations in `GatewayConfig`.
#[derive(Clone)]
pub struct S3Service<P: Provider<Ethereum>> {
    gateway_config_contract: GatewayConfigInstance<P>,
}

impl<P> S3Service<P>
where
    P: Provider<Ethereum>,
{
    pub fn new(provider: P, gateway_config_address: Address) -> Self {
        let gateway_config_contract = GatewayConfig::new(gateway_config_address, provider);
        Self {
            gateway_config_contract,
        }
    }

    /// The members as of one block: each coprocessor's signer with the bucket it
    /// registered. `GatewayConfig` keys `coprocessors` by tx-sender address, so
    /// this walks the tx senders and takes the signer from each registration.
    /// Both reads name the one block, so every operator resolves one set, and a
    /// coprocessor dropped later keeps the bucket it used.
    pub async fn get_members_at(&self, block: u64) -> anyhow::Result<Vec<(Address, String)>> {
        let at = BlockId::number(block);
        let tx_senders = self
            .gateway_config_contract
            .getCoprocessorTxSenders()
            .block(at)
            .call()
            .await
            .map_err(|e| anyhow!("getCoprocessorTxSenders at block {block} failed: {e}"))?;

        let mut members = Vec::with_capacity(tx_senders.len());
        for tx_sender in tx_senders {
            let coprocessor = self
                .gateway_config_contract
                .getCoprocessor(tx_sender)
                .block(at)
                .call()
                .await
                .map_err(|e| anyhow!("getCoprocessor({tx_sender}) at block {block} failed: {e}"))?;
            if coprocessor.s3BucketUrl.is_empty() {
                return Err(anyhow!(
                    "no S3 bucket URL for coprocessor {tx_sender} at block {block}"
                ));
            }
            members.push((coprocessor.signerAddress, coprocessor.s3BucketUrl));
        }
        Ok(members)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Bytes;
    use alloy::providers::{mock::Asserter, ProviderBuilder};
    use alloy::sol_types::SolCall;
    use fhevm_gateway_bindings::gateway_config::GatewayConfig::{
        getCoprocessorCall, getCoprocessorTxSendersCall, Coprocessor,
    };

    /// `GatewayConfig` keys `coprocessors` by tx sender, and each registration
    /// names the signer. A member read by signer address resolves to nothing,
    /// and this operator then waits for a member it cannot reach.
    #[tokio::test]
    async fn members_come_from_the_tx_sender_registration() {
        let tx_sender = Address::from([0x01; 20]);
        let signer = Address::from([0x02; 20]);
        let asserter = Asserter::new();
        asserter.push_success(&Bytes::from(
            getCoprocessorTxSendersCall::abi_encode_returns(&vec![tx_sender]),
        ));
        asserter.push_success(&Bytes::from(getCoprocessorCall::abi_encode_returns(
            &Coprocessor {
                txSenderAddress: tx_sender,
                signerAddress: signer,
                s3BucketUrl: "http://bucket".to_owned(),
            },
        )));

        let service = S3Service::new(
            ProviderBuilder::default().connect_mocked_client(asserter),
            Address::ZERO,
        );

        assert_eq!(
            service.get_members_at(7).await.expect("members"),
            vec![(signer, "http://bucket".to_owned())]
        );
    }
}
