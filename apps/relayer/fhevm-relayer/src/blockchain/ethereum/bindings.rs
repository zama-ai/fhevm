use alloy::sol;

// Old version in fhevm-devops

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    GatewayContract,
    "./artifacts/contract-abis/GatewayContract.abi"
);

// New version of DecryptionOracle

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    DecryptionOracle,
    "./artifacts/contract-abis/DecryptionOracle.json"
);

// TFHE EXecutor

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    TFHEExecutor,
    "./artifacts/contract-abis/TFHEExecutor.json"
);

// Decryption Manager (Rollup)

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    DecyptionManager,
    "./artifacts/contract-abis/DecryptionManager.json"
);

// ZKPoK  Manager (Rollup)

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    ZKPoKManager,
    "./artifacts/contract-abis/ZKPoKManager.json"
);

// Define the Transfer event structure using alloy_sol_types
sol! {
    #[derive(Debug)]
    event Transfer(address indexed from, address indexed to, uint256 value);
}
