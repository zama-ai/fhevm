// TODO: Remove this file when gateway-l2 is public
// by direct import httpz_gateway_rust_bindings

use alloy_sol_types::sol;

sol! {
    #[allow(clippy::too_many_arguments)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    IDecryptionManager,
    "./src/gwl2_contracts/abi/IDecryptionManager.abi"
}

sol! {
    #[allow(clippy::too_many_arguments)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    HTTPZ,
    "./src/gwl2_contracts/abi/HTTPZ.abi"
}

sol! {
    #[allow(clippy::too_many_arguments)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    IKeyManager,
    "./src/gwl2_contracts/abi/IKeyManager.abi"
}
