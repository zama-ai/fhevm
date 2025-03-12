use alloy_sol_types::sol;

pub use acl::IACLManager;

// As the IACLManager uses IDecryptionManager internally, the parsing of the IACLManager.abi with
// the `sol!` macro will expose a `IDecryptionManager` module. Thus, we do this operation in a
// dedicated `acl` module, to avoid conflict with the `IDecryptionManager` module that will be
// exposed by the parsing of the IDecryptionManager.abi with the `sol!` macro.
pub mod acl {
    use super::sol;

    sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        IACLManager,
        "abi/IACLManager.abi"
    );
}

sol!(
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    ICiphertextManager,
    "abi/ICiphertextManager.abi"
);

sol!(
    #[allow(clippy::too_many_arguments)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    IDecryptionManager,
    "abi/IDecryptionManager.abi"
);

sol!(
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    IHTTPZ,
    "abi/IHTTPZ.abi"
);

sol!(
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    IKeyManager,
    "abi/IKeyManager.abi"
);

sol!(
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    IZKPoKManager,
    "abi/IZKPoKManager.abi"
);
