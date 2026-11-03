//! Resolves the coprocessors of one upgrade attempt from the on-chain
//! `GatewayConfig` contract, at the block that the attempt names.

use std::time::Duration;

use alloy::{
    eips::BlockId,
    network::Ethereum,
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
    transports::http::reqwest::Url,
};
use anyhow::anyhow;
use fhevm_gateway_bindings::gateway_config::GatewayConfig::{self, GatewayConfigInstance};

/// Reader for the coprocessor registrations in `GatewayConfig`.
///
/// It holds where to reach the Gateway, not a connection. The Gateway is read
/// at startup and once per upgrade attempt, so a held websocket would sit idle
/// between reads, time out and log reconnect errors. Each read opens its own
/// connection and drops it on return, so no connection outlives its read.
#[derive(Clone)]
pub struct S3Service {
    gw_url: Url,
    gateway_config_address: Address,
    max_retries: u32,
    retry_interval: Duration,
}

impl S3Service {
    pub fn new(
        gw_url: Url,
        gateway_config_address: Address,
        max_retries: u32,
        retry_interval: Duration,
    ) -> Self {
        Self {
            gw_url,
            gateway_config_address,
            max_retries,
            retry_interval,
        }
    }

    async fn connect(&self) -> anyhow::Result<impl Provider<Ethereum>> {
        ProviderBuilder::new()
            .connect_ws(
                WsConnect::new(self.gw_url.clone())
                    .with_max_retries(self.max_retries)
                    .with_retry_interval(self.retry_interval),
            )
            .await
            .map_err(|e| anyhow!("connecting to the Gateway failed: {e}"))
    }

    /// The Gateway chain id (`eth_chainId`).
    pub async fn chain_id(&self) -> anyhow::Result<u64> {
        let provider = self.connect().await?;
        provider
            .get_chain_id()
            .await
            .map_err(|e| anyhow!("eth_chainId failed: {e}"))
    }

    /// The members as of one block, read over a connection of their own.
    pub async fn get_members_at(&self, block: u64) -> anyhow::Result<Vec<(Address, String)>> {
        let contract = GatewayConfig::new(self.gateway_config_address, self.connect().await?);
        members_at(&contract, block).await
    }
}

/// The members as of one block: each coprocessor's signer with the bucket it
/// registered. `GatewayConfig` keys `coprocessors` by tx-sender address, so
/// this walks the tx senders and takes the signer from each registration.
/// Both reads name the one block, so every operator resolves one set, and a
/// coprocessor dropped later keeps the bucket it used.
async fn members_at<P: Provider<Ethereum>>(
    contract: &GatewayConfigInstance<P>,
    block: u64,
) -> anyhow::Result<Vec<(Address, String)>> {
    let at = BlockId::number(block);
    let tx_senders = contract
        .getCoprocessorTxSenders()
        .block(at)
        .call()
        .await
        .map_err(|e| anyhow!("getCoprocessorTxSenders at block {block} failed: {e}"))?;

    let mut members = Vec::with_capacity(tx_senders.len());
    for tx_sender in tx_senders {
        let coprocessor = contract
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Bytes;
    use alloy::providers::mock::Asserter;
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

        let contract = GatewayConfig::new(
            Address::ZERO,
            ProviderBuilder::default().connect_mocked_client(asserter),
        );

        assert_eq!(
            members_at(&contract, 7).await.expect("members"),
            vec![(signer, "http://bucket".to_owned())]
        );
    }
}
