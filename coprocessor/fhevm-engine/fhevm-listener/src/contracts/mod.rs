use alloy::sol;

// contracts are compiled in build.rs/build_contract() using hardhat
// json are generated in build.rs/build_contract() using hardhat
sol!(
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    AclContract,
    "./../../../host-contracts/artifacts/contracts/ACL.sol/ACL.json"
);

sol!(
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    TfheContract,
    "./../../../host-contracts/artifacts/contracts/FHEVMExecutor.sol/FHEVMExecutor.json"
);
