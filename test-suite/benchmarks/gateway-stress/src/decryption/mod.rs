mod public;
pub mod user;

pub use public::{init_public_decryption_response_listener, public_decryption_burst};

use alloy::{
    primitives::{B256, LogData, U256},
    rpc::types::TransactionReceipt,
};
use anyhow::anyhow;
use tracing::{debug, trace};

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
            let event_id = decode_fn(log.data())?;
            debug!(
                ?receipt.transaction_hash,
                ?event_id,
                "Found decryption ID from event"
            );
            return Ok(event_id);
        }
    }

    Err(anyhow!("Event not found in logs"))
}
