use crate::decryption::extract_id_from_receipt;
use alloy::{
    contract::EventPoller,
    primitives::{FixedBytes, U256},
    providers::Provider,
    rpc::types::TransactionReceipt,
    sol_types::SolEvent,
};
use anyhow::anyhow;
use fhevm_gateway_rust_bindings::decryption::Decryption::{
    self, DecryptionInstance, PublicDecryptionResponse,
};
use futures::StreamExt;
use std::{collections::HashSet, time::Duration};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tracing::{debug, error, info};

#[tracing::instrument(skip(decryption_contract, handles, id_sender))]
pub async fn send_public_decryption<P: Provider>(
    index: u64,
    decryption_contract: DecryptionInstance<(), P>,
    handles: Vec<FixedBytes<32>>,
    id_sender: UnboundedSender<U256>,
) {
    if let Err(e) = send_public_decryption_inner(decryption_contract, handles, id_sender).await {
        error!("{e}");
    }
}

async fn send_public_decryption_inner<P: Provider>(
    decryption_contract: DecryptionInstance<(), P>,
    handles: Vec<FixedBytes<32>>,
    id_sender: UnboundedSender<U256>,
) -> anyhow::Result<()> {
    let decryption_call = decryption_contract
        .publicDecryptionRequest(handles)
        .send()
        .await
        .map_err(|e| anyhow!("Failed to send transaction: {e}"))?;
    let receipt = decryption_call
        .get_receipt()
        .await
        .map_err(|e| anyhow!("Failed to get receipt: {e}"))?;
    info!("PublicDecryptionRequest successfully sent!");

    let req_id = extract_public_decryption_id_from_receipt(&receipt)?;
    id_sender.send(req_id)?;

    Ok(())
}

fn extract_public_decryption_id_from_receipt(receipt: &TransactionReceipt) -> anyhow::Result<U256> {
    extract_id_from_receipt(
        receipt,
        Decryption::PublicDecryptionRequest::SIGNATURE_HASH,
        |log| {
            Decryption::PublicDecryptionRequest::decode_log_data(log, true)
                .map(|event| event.decryptionId)
                .map_err(|e| anyhow!("Failed to decode event data {e}"))
        },
    )
}

pub async fn wait_for_response(
    filter: EventPoller<PublicDecryptionResponse>,
    id_receiver: UnboundedReceiver<U256>,
) {
    if let Err(e) = wait_for_response_inner(filter, id_receiver).await {
        error!("{e}");
    }
}

async fn wait_for_response_inner(
    mut filter: EventPoller<PublicDecryptionResponse>,
    mut id_receiver: UnboundedReceiver<U256>,
) -> anyhow::Result<()> {
    filter.poller = filter.poller.with_poll_interval(Duration::from_secs(1));
    let mut responses = filter.into_stream();
    let mut received_ids = HashSet::new();
    loop {
        let Some(id) = id_receiver.recv().await else {
            break;
        };
        info!(
            "PublicDecryptionRequest #{id} was sent. Waiting for PublicDecryptionResponse #{id}..."
        );

        while !received_ids.remove(&id) {
            match responses.next().await {
                Some(Ok((response, _))) => {
                    let response_id = response.decryptionId;
                    debug!("Received PublicDecryptionResponse #{response_id}");
                    received_ids.insert(response_id);
                }
                Some(Err(e)) => return Err(anyhow!("Failed to retrieve next event: {e}")),
                None => return Err(anyhow!("No more events to receive!")),
            }
        }
        info!("PublicDecryptionResponse #{id} was successfully received!");
    }
    Ok(())
}
