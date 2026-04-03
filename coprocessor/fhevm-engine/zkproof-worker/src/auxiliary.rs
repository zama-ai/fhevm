use fhevm_engine_common::chain_id::ChainId;
use std::str::FromStr;

const LEGACY_SIZE: usize = 92;
const INPUT_PROOF_EXTRA_DATA_VERSION: u8 = 0x01;
const INPUT_PROOF_IDENTITY_EXTRA_DATA_LENGTH: usize = 65;

/// ZkData is the data that is used to generate the ZKPs
#[derive(Debug, Clone)]
pub(crate) struct ZkData {
    pub contract_address: String,
    pub user_address: String,
    pub acl_contract_address: String,
    pub chain_id: ChainId,
    pub extra_data: Vec<u8>,
}

impl ZkData {
    /// creates the auxiliary data for proving/verifying the input ZKPs from the
    /// individual inputs.
    ///
    /// Legacy format:
    /// `contract_addr(20) || user_addr(20) || acl_addr(20) || chain_id(32)`
    ///
    /// Native identity format:
    /// `contract_id(32) || user_id(32) || acl_id(32) || chain_id(32)`
    pub fn assemble(&self) -> anyhow::Result<Vec<u8>> {
        let chain_id_bytes: [u8; 32] = alloy_primitives::U256::from(self.chain_id.as_u64())
            .to_owned()
            .to_be_bytes();

        if let Some((contract_id, user_id)) = parse_versioned_identities(&self.extra_data)? {
            let acl_identity = parse_acl_identity(&self.acl_contract_address)?;
            let mut data = Vec::with_capacity(128);
            data.extend_from_slice(&contract_id);
            data.extend_from_slice(&user_id);
            data.extend_from_slice(&acl_identity);
            data.extend_from_slice(&chain_id_bytes);
            return Ok(data);
        }

        let contract_bytes =
            alloy_primitives::Address::from_str(&self.contract_address)?.into_array();
        let user_bytes = alloy_primitives::Address::from_str(&self.user_address)?.into_array();
        let acl_bytes =
            alloy_primitives::Address::from_str(&self.acl_contract_address)?.into_array();

        let front: Vec<u8> = [contract_bytes, user_bytes, acl_bytes].concat();
        let mut data = vec![0_u8; LEGACY_SIZE];
        data[..60].copy_from_slice(front.as_slice());
        data[60..].copy_from_slice(&chain_id_bytes);
        Ok(data)
    }

    pub fn acl_identity_bytes_for_handle(&self) -> anyhow::Result<Vec<u8>> {
        if parse_versioned_identities(&self.extra_data)?.is_some() {
            return Ok(parse_acl_identity(&self.acl_contract_address)?.to_vec());
        }

        let acl_bytes =
            alloy_primitives::Address::from_str(&self.acl_contract_address)?.into_array();
        Ok(acl_bytes.to_vec())
    }
}

fn parse_versioned_identities(extra_data: &[u8]) -> anyhow::Result<Option<([u8; 32], [u8; 32])>> {
    if extra_data.len() != INPUT_PROOF_IDENTITY_EXTRA_DATA_LENGTH {
        return Ok(None);
    }
    if extra_data[0] != INPUT_PROOF_EXTRA_DATA_VERSION {
        return Ok(None);
    }

    let contract_id: [u8; 32] = extra_data[1..33]
        .try_into()
        .map_err(|_| anyhow::anyhow!("invalid contract identity bytes in input-proof extraData"))?;
    let user_id: [u8; 32] = extra_data[33..65]
        .try_into()
        .map_err(|_| anyhow::anyhow!("invalid user identity bytes in input-proof extraData"))?;
    Ok(Some((contract_id, user_id)))
}

fn parse_acl_identity(value: &str) -> anyhow::Result<[u8; 32]> {
    if let Ok(address) = alloy_primitives::Address::from_str(value) {
        let mut identity = [0_u8; 32];
        identity[12..].copy_from_slice(address.as_slice());
        return Ok(identity);
    }

    if let Some(stripped) = value.strip_prefix("0x") {
        let raw = hex::decode(stripped)?;
        if raw.len() == 32 {
            return raw
                .try_into()
                .map_err(|_| anyhow::anyhow!("invalid 32-byte ACL identity"));
        }
    }

    let raw = bs58::decode(value).into_vec()?;
    if raw.len() != 32 {
        anyhow::bail!("invalid ACL identity length: expected 32 bytes, got {}", raw.len());
    }
    raw.try_into()
        .map_err(|_| anyhow::anyhow!("invalid 32-byte ACL identity"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::hex;

    #[test]
    fn test_assemble_valid_addresses() {
        let contract_address = "0x1111111111111111111111111111111111111111".to_string();
        let user_address = "0x2222222222222222222222222222222222222222".to_string();
        let acl_contract_address = "0x3333333333333333333333333333333333333333".to_string();
        let chain_id = ChainId::try_from(1_u64).unwrap();

        let zk_data = ZkData {
            contract_address: contract_address.clone(),
            user_address: user_address.clone(),
            acl_contract_address: acl_contract_address.clone(),
            chain_id,
            extra_data: vec![0x00],
        };

        let assembled_hex = hex::encode(zk_data.assemble().expect("Failed to assemble ZkData"));
        let expected_hex = contract_address[2..].to_string()
            + &user_address[2..]
            + &acl_contract_address[2..]
            + "0000000000000000000000000000000000000000000000000000000000000001";

        assert_eq!(assembled_hex.len() / 2, LEGACY_SIZE);
        assert_eq!(assembled_hex, expected_hex);
    }

    #[test]
    fn test_assemble_versioned_identities() {
        let chain_id = ChainId::try_from(1_u64).unwrap();
        let contract_id = [0x11_u8; 32];
        let user_id = [0x22_u8; 32];
        let acl_identity = [0x33_u8; 32];
        let mut extra_data = vec![INPUT_PROOF_EXTRA_DATA_VERSION];
        extra_data.extend_from_slice(&contract_id);
        extra_data.extend_from_slice(&user_id);

        let zk_data = ZkData {
            contract_address: "0x1111111111111111111111111111111111111111".to_string(),
            user_address: "0x2222222222222222222222222222222222222222".to_string(),
            acl_contract_address: format!("0x{}", hex::encode(acl_identity)),
            chain_id,
            extra_data,
        };

        let assembled = zk_data.assemble().expect("Failed to assemble ZkData");
        assert_eq!(assembled.len(), 128);
        assert_eq!(&assembled[0..32], &contract_id);
        assert_eq!(&assembled[32..64], &user_id);
        assert_eq!(&assembled[64..96], &acl_identity);
    }
}
