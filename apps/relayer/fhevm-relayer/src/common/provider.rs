use alloy::sol;

// Old version in fhevm-devops

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    GatewayContract,
    "./artifacts/GatewayContract.abi"
);

pub const DECRYPTION_EVENT_CONTRACT_ADDRESS: &str = "0x67aa98a03CC4559E1e98e7b4Ed071C35c40b588d";

pub const DECRYPTION_EVENT_SIGNATURE: &str =
    "EventDecryption(uint256,uint256[],address,bytes4,uint256,uint256,bool)";
pub const TOPIC_DECRYPTION_EVENT_SIGNATURE: &str =
    "0x2dc9f5cb271a872eb89f488a1d216ded8a5b96226ca01f8d3128e028ae5459f8";

// New version of DecryptionOracle

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    DecryptionOracle,
    "./artifacts/DecryptionOracle.json"
);

pub const DECRYPTION_ORACLE_EVENT_CONTRACT_ADDRESS: &str =
    "0x67aa98a03CC4559E1e98e7b4Ed071C35c40b588d";

pub const DECRYPTION_ORACLE_EVENT_SIGNATURE: &str =
    "DecryptionRequest(uint256,uint256,uint256[],address,bytes4)";
pub const TOPIC_DECRYPTION_ORACLE_EVENT_SIGNATURE: &str =
    "0x2139fe1716d177355181c45bfba01280a9ce6d0a226dec18bb5808867a812179";

// TFHE EXecutor

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    TFHEExecutor,
    "./artifacts/TFHEExecutor.json"
);

pub const TFHE_EXECUTOR_EVENT_CONTRACT_ADDRESS: &str = "0x4e142887e3Dc6e414a9b260a1034D20C9B4Eb11F";

pub const TFHE_EXECUTOR_FHE_ADD_EVENT_SIGNATURE: &str =
    "FheAdd(address,uint256,uint256,bytes1,uint256)";

// pub const TOPIC_DECRYPTION_ORACLE_EVENT_SIGNATURE: &str =
//     "0x2139fe1716d177355181c45bfba01280a9ce6d0a226dec18bb5808867a812179";
