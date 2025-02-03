use alloy::primitives::keccak256;

/// Events related to the Decryption Oracle
#[derive(Debug)]
pub struct DecryptionOracleEvents;

impl DecryptionOracleEvents {
    pub const EVENT_SIGNATURE: &'static str =
        "DecryptionRequest(uint256,uint256,uint256[],address,bytes4)";

    pub const TOPIC_SIGNATURE: &'static str =
        "0x2139fe1716d177355181c45bfba01280a9ce6d0a226dec18bb5808867a812179";

    pub const LEGACY_EVENT_SIGNATURE: &'static str =
        "EventDecryption(uint256,uint256[],address,bytes4,uint256,uint256,bool)";

    pub const LEGACY_TOPIC_SIGNATURE: &'static str =
        "0x2dc9f5cb271a872eb89f488a1d216ded8a5b96226ca01f8d3128e028ae5459f8";

    pub fn validate_topics() -> bool {
        let computed_topic = format!("0x{:x}", keccak256(Self::EVENT_SIGNATURE));
        computed_topic == Self::TOPIC_SIGNATURE
            && format!("0x{:x}", keccak256(Self::LEGACY_EVENT_SIGNATURE))
                == Self::LEGACY_TOPIC_SIGNATURE
    }
}

#[derive(Debug)]
pub struct TfheExecutorEvents;

impl TfheExecutorEvents {
    pub const FHE_ADD_SIGNATURE: &'static str = "FheAdd(address,uint256,uint256,bytes1,uint256)";

    pub const FHE_SUB_SIGNATURE: &'static str = "FheSub(address,uint256,uint256,bytes1,uint256)";

    pub fn all_signatures() -> Vec<&'static str> {
        vec![Self::FHE_ADD_SIGNATURE, Self::FHE_SUB_SIGNATURE]
    }

    pub fn validate_signatures() -> bool {
        Self::all_signatures()
            .iter()
            .all(|sig| sig.contains('(') && sig.contains(')') && sig.contains(','))
    }
}
