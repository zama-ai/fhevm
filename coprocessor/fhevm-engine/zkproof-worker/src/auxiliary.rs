use anyhow::anyhow;
use fhevm_engine_common::chain_id::ChainId;
use std::str::FromStr;

/// EVM auxiliary layout: 3 x 20-byte address + 32-byte chain id.
const SIZE_EVM: usize = 92;
/// Solana (RFC-021) auxiliary layout: 3 x 32-byte bytes32 identity + 32-byte chain id.
const SIZE_SOLANA: usize = 128;

/// ZkData is the data that is used to generate the ZKPs
#[derive(Debug, Clone)]
pub(crate) struct ZkData {
    pub contract_address: String,
    pub user_address: String,
    pub acl_contract_address: String,
    pub chain_id: ChainId,
}

impl ZkData {
    /// Assembles the auxiliary data the input ZK proof is bound to.
    ///
    /// The prover (client SDK) and this verifier must agree on the layout byte
    /// for byte, or proof verification fails. The host chain type selects it:
    /// EVM hosts use 20-byte addresses (92 bytes total), Solana hosts use RFC-021
    /// bytes32 identities (128 bytes total). The chain id is always the trailing
    /// 32-byte big-endian word and carries the chain-type high bit verbatim.
    pub fn assemble(&self) -> anyhow::Result<Vec<u8>> {
        if self.chain_id.is_solana_host() {
            self.assemble_solana()
        } else {
            self.assemble_evm()
        }
    }

    /// `contract_addr(20) || user_addr(20) || acl_contract_addr(20) || chain_id(32)`.
    fn assemble_evm(&self) -> anyhow::Result<Vec<u8>> {
        let mut data = Vec::with_capacity(SIZE_EVM);
        data.extend_from_slice(
            alloy_primitives::Address::from_str(&self.contract_address)?.as_slice(),
        );
        data.extend_from_slice(alloy_primitives::Address::from_str(&self.user_address)?.as_slice());
        data.extend_from_slice(
            alloy_primitives::Address::from_str(&self.acl_contract_address)?.as_slice(),
        );
        data.extend_from_slice(&self.chain_id_word());
        debug_assert_eq!(data.len(), SIZE_EVM);
        Ok(data)
    }

    /// `contract(32) || user(32) || acl(32) || chain_id(32)`, where the three
    /// identities are RFC-021 bytes32 host addresses (0x-prefixed 32-byte hex).
    fn assemble_solana(&self) -> anyhow::Result<Vec<u8>> {
        let mut data = Vec::with_capacity(SIZE_SOLANA);
        data.extend_from_slice(&parse_bytes32(&self.contract_address)?);
        data.extend_from_slice(&parse_bytes32(&self.user_address)?);
        data.extend_from_slice(&parse_bytes32(&self.acl_contract_address)?);
        data.extend_from_slice(&self.chain_id_word());
        debug_assert_eq!(data.len(), SIZE_SOLANA);
        Ok(data)
    }

    /// Chain id as a 32-byte big-endian word, preserving the chain-type high bit.
    fn chain_id_word(&self) -> [u8; 32] {
        alloy_primitives::U256::from(self.chain_id.as_u64()).to_be_bytes()
    }
}

/// Parses a Solana `bytes32` host identity from either encoding it appears in:
/// the `0x`-prefixed hex form carried verbatim by gateway events (contract and
/// user identities), or the base58 form an on-chain Solana program id is stored
/// as in `host_chains` (the ACL identity). Both encode the same 32 bytes; the
/// `0x` prefix is the discriminator.
pub(crate) fn parse_bytes32(value: &str) -> anyhow::Result<[u8; 32]> {
    let bytes = match value.strip_prefix("0x") {
        Some(hex_str) => alloy_primitives::hex::decode(hex_str)?,
        None => bs58::decode(value).into_vec()?,
    };
    <[u8; 32]>::try_from(bytes.as_slice())
        .map_err(|_| anyhow!("expected a 32-byte identity, got {} bytes", bytes.len()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::hex;
    use fhevm_engine_common::chain_id::SOLANA_CHAIN_TYPE_BIT;

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

        assert_eq!(assembled_hex.len() / 2, SIZE_EVM);
        assert_eq!(assembled_hex, expected_hex);
    }

    #[test]
    fn assembles_solana_bytes32_aux() {
        let contract = format!("0x{}", "11".repeat(32));
        let user = format!("0x{}", "22".repeat(32));
        let acl = format!("0x{}", "33".repeat(32));
        let chain_id = ChainId::from_canonical_u64(SOLANA_CHAIN_TYPE_BIT | 12345);

        let zk_data = ZkData {
            contract_address: contract,
            user_address: user,
            acl_contract_address: acl,
            chain_id,
        };

        let assembled = zk_data.assemble().expect("assemble solana aux");
        assert_eq!(assembled.len(), SIZE_SOLANA);

        // contract(32) || user(32) || acl(32) || chain_id(32 BE, high bit set).
        assert_eq!(&assembled[0..32], &[0x11; 32]);
        assert_eq!(&assembled[32..64], &[0x22; 32]);
        assert_eq!(&assembled[64..96], &[0x33; 32]);
        let expected_chain_id =
            alloy_primitives::U256::from(SOLANA_CHAIN_TYPE_BIT | 12345).to_be_bytes::<32>();
        assert_eq!(&assembled[96..128], &expected_chain_id);
    }

    #[test]
    fn solana_aux_rejects_short_identity() {
        let zk_data = ZkData {
            contract_address: "0x1111111111111111111111111111111111111111".to_string(),
            user_address: format!("0x{}", "22".repeat(32)),
            acl_contract_address: format!("0x{}", "33".repeat(32)),
            chain_id: ChainId::from_canonical_u64(SOLANA_CHAIN_TYPE_BIT | 1),
        };
        // The 20-byte contract address is not a valid bytes32 identity.
        assert!(zk_data.assemble().is_err());
    }

    #[test]
    fn parse_bytes32_accepts_hex_and_base58_for_same_identity() {
        // A real Solana program id in both encodings: 0x-hex (as gateway events
        // carry it) and base58 (as host_chains stores it) decode to identical bytes.
        let hex = "0x9c7da263cccb5084844e292a2ce0db0e51bbf310100656aa4572b83dfe35fca5";
        let base58 = "BXsiKq6Jg4vgdBqSd75NbMbKaB7WFKK48NVXx4zoeLsW";
        assert_eq!(parse_bytes32(hex).unwrap(), parse_bytes32(base58).unwrap());
    }

    #[test]
    fn solana_aux_matches_when_acl_is_base58() {
        // The ACL identity arrives base58 (from host_chains) while contract/user
        // arrive 0x-hex (from the event); the assembled aux must be identical to
        // the all-hex form so the prover and verifier agree byte for byte.
        let contract = format!("0x{}", "11".repeat(32));
        let user = format!("0x{}", "22".repeat(32));
        let acl_hex = "0x9c7da263cccb5084844e292a2ce0db0e51bbf310100656aa4572b83dfe35fca5";
        let acl_b58 = "BXsiKq6Jg4vgdBqSd75NbMbKaB7WFKK48NVXx4zoeLsW";
        let chain_id = ChainId::from_canonical_u64(SOLANA_CHAIN_TYPE_BIT | 12345);

        let with_hex = ZkData {
            contract_address: contract.clone(),
            user_address: user.clone(),
            acl_contract_address: acl_hex.to_string(),
            chain_id,
        }
        .assemble()
        .unwrap();
        let with_b58 = ZkData {
            contract_address: contract,
            user_address: user,
            acl_contract_address: acl_b58.to_string(),
            chain_id,
        }
        .assemble()
        .unwrap();
        assert_eq!(with_hex, with_b58);
    }
}
