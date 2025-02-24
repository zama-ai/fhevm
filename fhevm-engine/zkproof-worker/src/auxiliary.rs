use std::str::FromStr;

const SIZE: usize = 92;

/// ZkData is the data that is used to generate the ZKPs
#[derive(Debug, Clone)]
pub(crate) struct ZkData {
    pub contract_address: String,
    pub user_address: String,
    pub acl_contract_address: String,
    pub chain_id: i32,
}

impl ZkData {
    /// creates the auxiliary data for proving/verifying the input ZKPs from the individual inputs
    ///
    /// `contract_addr || user_addr  || acl_contract_addr || chain_id` i.e. 92 bytes since chain ID is encoded as a 32 byte big endian integer
    pub fn assemble(&self) -> anyhow::Result<[u8; SIZE]> {
        let contract_bytes =
            alloy_primitives::Address::from_str(&self.contract_address)?.into_array();
        let user_bytes = alloy_primitives::Address::from_str(&self.user_address)?.into_array();
        let acl_bytes =
            alloy_primitives::Address::from_str(&self.acl_contract_address)?.into_array();
        let chain_id_bytes: [u8; 32] = alloy_primitives::U256::from(self.chain_id)
            .to_owned()
            .to_be_bytes();

        // Copy contract address into the first 20 bytes
        let front: Vec<u8> = [contract_bytes, user_bytes, acl_bytes].concat();
        let mut data = [0_u8; SIZE];
        data[..60].copy_from_slice(front.as_slice());
        data[60..].copy_from_slice(&chain_id_bytes);
        Ok(data)
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
        let chain_id = 1;

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
