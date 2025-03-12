use alloy_sol_types::sol;

pub use acl::ACLManager;

// As the ACLManager uses IDecryptionManager internally, the parsing of the ACLManager.abi with
// the `sol!` macro will expose a `IDecryptionManager` module. Thus, we do this operation in a
// dedicated `acl` module, to avoid conflict with the `IDecryptionManager` module that will be
// exposed by the parsing of the DecryptionManager.abi with the `sol!` macro.
pub mod acl {
    use super::sol;

    sol!(
        #[sol(rpc)]
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        ACLManager,
        "abi/ACLManager.abi"
    );
}

sol!(
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    CiphertextManager,
    "abi/CiphertextManager.abi"
);

sol!(
    #[allow(clippy::too_many_arguments)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    DecryptionManager,
    "abi/DecryptionManager.abi"
);

sol!(
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    HTTPZ,
    "abi/HTTPZ.abi"
);

sol!(
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    KeyManager,
    "abi/KeyManager.abi"
);

sol!(
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    ZKPoKManager,
    "abi/ZKPoKManager.abi"
);
