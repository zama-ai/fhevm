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

// Each ABI re-declares `IKMSGeneration`; scope them into submodules to avoid
// the name colliding in this module.
pub mod kms_generation {
    use alloy::sol;
    sol!(
        #[sol(rpc)]
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        KMSGeneration,
        "./../../../host-contracts/artifacts/contracts/KMSGeneration.sol/KMSGeneration.json"
    );
}
pub use kms_generation::KMSGeneration;

pub mod protocol_config {
    use alloy::sol;
    sol!(
        #[sol(rpc)]
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        ProtocolConfig,
        "./../../../host-contracts/artifacts/contracts/ProtocolConfig.sol/ProtocolConfig.json"
    );
}
pub use protocol_config::ProtocolConfig;
