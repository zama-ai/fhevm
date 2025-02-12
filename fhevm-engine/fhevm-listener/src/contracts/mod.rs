use alloy::sol;

#[allow(unused_imports)] // required for accessing ::COUNT
use alloy_sol_types::SolEventInterface;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    AclContract,
    "src/contracts/ACL.json.abi"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    TfheContract,
    "src/contracts/TFHEExecutor.json.abi"
);
