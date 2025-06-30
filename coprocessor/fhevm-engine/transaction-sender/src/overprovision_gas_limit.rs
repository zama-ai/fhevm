use alloy::network::{Ethereum, TransactionBuilder};
use alloy_provider::Provider;
use alloy_rpc_types::TransactionRequest;
use tracing::{debug, warn};

// If `txn_request.gas` is set, overprovision it by the given percent.
// If `txn_request.gas` is not set, estimate the gas limit and then overprovision it by the given percent.
// If the percent is less than 100, code will assert.
// If the gas estimation fails, it will not set the gas limit for overprovisioning and will log a warning.
pub async fn try_overprovision_gas_limit<T: Provider<Ethereum>>(
    txn_request: impl Into<TransactionRequest>,
    provider: &T,
    percent: u32,
) -> TransactionRequest {
    assert!(percent >= 100, "Overprovision percent must be at least 100");

    let overprovision = |gas: u64| (gas as u128 * percent as u128 / 100) as u64;

    let mut txn: TransactionRequest = txn_request.into();

    let new_gas = match txn.gas {
        Some(existing_gas) => Some(existing_gas),
        None => match provider.estimate_gas(txn.clone()).await {
            Ok(estimated_gas) => Some(estimated_gas),
            Err(err) => {
                warn!(error = %err, gas_limit_overprovision_percent = percent, "Failed to estimate gas for overprovisioning, not setting gas limit for overprovisioning");
                None
            }
        },
    }.map(overprovision);

    if let Some(gas) = new_gas {
        debug!(
            gas_limit = gas,
            gas_limit_overprovision_percent = percent,
            "Overprovisioned gas limit"
        );
        txn.set_gas_limit(gas);
    }

    txn
}
