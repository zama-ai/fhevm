// TODO: Remove this file when fhevm-gateway is public
// by direct import fhevm_gateway_rust_bindings

use alloy::sol;

sol! {
    #[allow(clippy::too_many_arguments)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    IDecryption,
    "./src/gw_contracts/abi/IDecryption.abi"
}

sol! {
    #[allow(clippy::too_many_arguments)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    GatewayConfig,
    "./src/gw_contracts/abi/GatewayConfig.abi"
}

sol! {
    #[allow(clippy::too_many_arguments)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    IKmsManagement,
    "./src/gw_contracts/abi/IKmsManagement.abi"
}
