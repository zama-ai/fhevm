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
