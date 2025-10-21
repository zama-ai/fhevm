pub mod public;
pub mod types;
pub mod user;

pub use public::{init_public_decryption_response_listener, public_decryption_burst};
pub use user::{init_user_decryption_response_listener, user_decryption_burst};

use alloy::{
    primitives::{B256, LogData, U256},
    providers::Provider,
    rpc::types::{TransactionReceipt, TransactionRequest},
};
use anyhow::anyhow;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, trace, warn};

use crate::blockchain::manager::AppProvider;

pub const EVENT_LISTENER_POLLING: Duration = Duration::from_millis(500);

fn extract_id_from_receipt<F>(
    receipt: &TransactionReceipt,
    event_hash: B256,
    decode_fn: F,
) -> anyhow::Result<U256>
where
    F: Fn(&LogData) -> anyhow::Result<U256>,
{
    trace!("Receipt details: {receipt:?}");

    for log in receipt.inner.logs().iter() {
        if let Some(first_topic) = log.topics().first()
            && first_topic == &event_hash
        {
            let decryption_id = decode_fn(log.data())?;
            debug!(
                ?receipt.transaction_hash,
                ?receipt.block_number,
                "Decryption #{decryption_id} has been accepted on the Gateway!"
            );
            return Ok(decryption_id);
        }
    }

    Err(anyhow!("Event not found in logs"))
}

const TX_RETRIES: usize = 50;
const TX_GAS_INCREASE_PERCENT: u128 = 105;

async fn send_tx_with_retries<F>(
    provider: &AppProvider,
    mut decryption_call: TransactionRequest,
    id_sender: UnboundedSender<U256>,
    extract_id_fn: F,
) -> Result<(), anyhow::Error>
where
    F: Fn(&TransactionReceipt) -> anyhow::Result<U256>,
{
    let mut last_error = String::new();
    for i in 1..=TX_RETRIES {
        overprovision_gas(provider, &mut decryption_call).await;

        trace!("Sending transaction to the Gateway");
        match provider
            .send_transaction_sync(decryption_call.clone())
            .await
        {
            Ok(receipt) => {
                let id = extract_id_fn(&receipt)?;
                id_sender.send(id)?;
                return Ok(());
            }
            Err(e) => {
                debug!("WARN: Transaction attempt #{i}/{TX_RETRIES} failed: {e}",);
                last_error = e.to_string();
            }
        }
    }
    Err(anyhow!(
        "All transactions attempt failed. Last error: {last_error}"
    ))
}

async fn overprovision_gas<P: Provider>(provider: &P, call: &mut TransactionRequest) {
    let current_gas = match call.gas {
        Some(gas) => gas,
        None => match provider.estimate_gas(call.clone()).await {
            Ok(estimation) => estimation,
            Err(e) => return warn!("Failed to estimate gas for the tx: {e}"),
        },
    };
    let new_gas = (current_gas as u128 * TX_GAS_INCREASE_PERCENT / 100) as u64;
    call.gas = Some(new_gas);
    trace!("Initial gas estimation for the tx: {current_gas}. Increased to {new_gas}");
}

pub struct BurstResult {
    pub latency: f64,
    pub throughput: f64,
}
