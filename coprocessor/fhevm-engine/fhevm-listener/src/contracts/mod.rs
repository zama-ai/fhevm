use alloy::sol;

#[allow(unused_imports)] // required for accessing ::COUNT
use alloy_sol_types::SolEventInterface;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    AclContract,
    "./../../../host-contracts/artifacts/contracts/ACL.sol/ACL.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    TfheContract,
    "./../../../host-contracts/artifacts/contracts/FHEVMExecutor.sol/FHEVMExecutor.json"
);
