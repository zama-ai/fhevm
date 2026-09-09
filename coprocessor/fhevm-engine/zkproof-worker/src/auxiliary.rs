use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::zk_aux::{assemble_aux_data, ZK_AUX_DATA_SIZE};

const SIZE: usize = ZK_AUX_DATA_SIZE;

/// ZkData is the data that is used to generate the ZKPs
#[derive(Debug, Clone)]
pub(crate) struct ZkData {
    pub contract_address: String,
    pub user_address: String,
    pub acl_contract_address: String,
    pub chain_id: ChainId,
}

impl ZkData {
    /// creates the auxiliary data for proving/verifying the input ZKPs from the
    /// individual inputs
    ///
    /// `contract_addr || user_addr  || acl_contract_addr || chain_id` i.e. 92
    /// bytes since chain ID is encoded as a 32 byte big endian integer
    pub fn assemble(&self) -> anyhow::Result<[u8; SIZE]> {
        assemble_aux_data(
            &self.contract_address,
            &self.user_address,
            &self.acl_contract_address,
            self.chain_id,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::hex;

    #[test]
    fn test_assemble_valid_addresses() {
        // Define  20-byte addresses
        let contract_address = "0x1111111111111111111111111111111111111111".to_string();
        let user_address = "0x2222222222222222222222222222222222222222".to_string();
        let acl_contract_address = "0x3333333333333333333333333333333333333333".to_string();
        let chain_id = ChainId::try_from(1_u64).unwrap();

        let zk_data = ZkData {
            contract_address: contract_address.clone(),
            user_address: user_address.clone(),
            acl_contract_address: acl_contract_address.clone(),
            chain_id,
        };

        let assembled_hex = hex::encode(zk_data.assemble().expect("Failed to assemble ZkData"));
        // concatenate the addresses
        let expected_hex = contract_address[2..].to_string()
            + &user_address[2..]
            + &acl_contract_address[2..]
            + "0000000000000000000000000000000000000000000000000000000000000001";

        assert_eq!(assembled_hex.len() / 2, SIZE);
        assert_eq!(assembled_hex, expected_hex);
    }
}
