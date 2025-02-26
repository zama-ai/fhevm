use alloy::sol;

// Old version in fhevm-devops

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    GatewayContract,
    "./artifacts/GatewayContract.abi"
);

// New version of DecryptionOracle

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    DecryptionOracle,
    "./artifacts/DecryptionOracle.json"
);

// TFHE EXecutor

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    TFHEExecutor,
    "./artifacts/TFHEExecutor.json"
);

// Decryption Manager (Rollup)

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    DecyptionManager,
    "./artifacts/DecryptionManager.json"
);

// ZKPoK  Manager (Rollup)

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    ZKPoKManager,
    "./artifacts/ZKPoKManager.json"
);

// Define the Transfer event structure using alloy_sol_types
sol! {
    #[derive(Debug)]
    event Transfer(address indexed from, address indexed to, uint256 value);
}
