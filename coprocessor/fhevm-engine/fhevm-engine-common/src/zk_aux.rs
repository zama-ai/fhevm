//! Auxiliary ("metadata") data bound into an input ZK proof.
//!
//! Both the prover and the verifier must assemble these bytes identically, or the proof
//! fails to verify. The layout lives here so the two sides cannot drift:
//!
//! EVM hosts (20-byte addresses, 92 bytes total):
//!
//! ```text
//! contract_addr (20) || user_addr (20) || acl_contract_addr (20) || chain_id (32 BE)
//! ```
//!
//! Solana hosts (RFC-021 bytes32 identities, 128 bytes total):
//!
//! ```text
//! contract (32) || user (32) || acl (32) || chain_id (32 BE, chain-type high bit set)
//! ```
//!
//! Verifier side: `zkproof-worker`'s `auxiliary::ZkData`, built from the `verify_proofs`
//! row plus the chain's ACL address. Prover side: real clients via the relayer SDK, and
//! [`crate::synthetic_input`] for the blue-green dry-run probe.

use std::str::FromStr;

use anyhow::anyhow;

use crate::chain_id::ChainId;

/// Length of the assembled auxiliary data. The chain id is a 32-byte big-endian integer,
/// so 20 + 20 + 20 + 32.
pub const ZK_AUX_DATA_SIZE: usize = 92;

/// Assemble the auxiliary data bound into an input proof.
///
/// Addresses are parsed from their `0x`-prefixed hex form, so a malformed address is an
/// error rather than silently-wrong metadata (which would only surface later as an
/// unverifiable proof).
pub fn assemble_aux_data(
    contract_address: &str,
    user_address: &str,
    acl_contract_address: &str,
    chain_id: ChainId,
) -> anyhow::Result<[u8; ZK_AUX_DATA_SIZE]> {
    let contract_bytes = alloy::primitives::Address::from_str(contract_address)?.into_array();
    let user_bytes = alloy::primitives::Address::from_str(user_address)?.into_array();
    let acl_bytes = alloy::primitives::Address::from_str(acl_contract_address)?.into_array();
    let chain_id_bytes: [u8; 32] = alloy::primitives::U256::from(chain_id.as_u64()).to_be_bytes();

    let mut data = [0_u8; ZK_AUX_DATA_SIZE];
    data[..20].copy_from_slice(&contract_bytes);
    data[20..40].copy_from_slice(&user_bytes);
    data[40..60].copy_from_slice(&acl_bytes);
    data[60..].copy_from_slice(&chain_id_bytes);
    Ok(data)
}

/// Length of the assembled Solana auxiliary data: 3 x 32-byte identity + 32-byte chain id.
pub const SOLANA_ZK_AUX_DATA_SIZE: usize = 128;

/// Assemble the auxiliary data bound into an input proof for a Solana host.
///
/// The three identities are RFC-021 bytes32 host addresses, accepted in either encoding
/// they appear in (see [`parse_bytes32`]). The chain id keeps its chain-type high bit.
pub fn assemble_solana_aux_data(
    contract_identity: &str,
    user_identity: &str,
    acl_identity: &str,
    chain_id: ChainId,
) -> anyhow::Result<[u8; SOLANA_ZK_AUX_DATA_SIZE]> {
    let chain_id_bytes: [u8; 32] = alloy::primitives::U256::from(chain_id.as_u64()).to_be_bytes();

    let mut data = [0_u8; SOLANA_ZK_AUX_DATA_SIZE];
    data[..32].copy_from_slice(&parse_bytes32(contract_identity)?);
    data[32..64].copy_from_slice(&parse_bytes32(user_identity)?);
    data[64..96].copy_from_slice(&parse_bytes32(acl_identity)?);
    data[96..].copy_from_slice(&chain_id_bytes);
    Ok(data)
}

/// Parses a Solana `bytes32` host identity from either encoding it appears in:
/// the `0x`-prefixed hex form carried verbatim by gateway events (contract and
/// user identities), or the base58 form an on-chain Solana program id is stored
/// as in `host_chains` (the ACL identity). Both encode the same 32 bytes; the
/// `0x` prefix is the discriminator.
pub fn parse_bytes32(value: &str) -> anyhow::Result<[u8; 32]> {
    let bytes = match value.strip_prefix("0x") {
        Some(hex_str) => alloy::primitives::hex::decode(hex_str)?,
        None => bs58::decode(value).into_vec()?,
    };
    <[u8; 32]>::try_from(bytes.as_slice())
        .map_err(|_| anyhow!("expected a 32-byte identity, got {} bytes", bytes.len()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::hex;

    #[test]
    fn layout_is_addresses_then_big_endian_chain_id() {
        let contract_address = "0x1111111111111111111111111111111111111111";
        let user_address = "0x2222222222222222222222222222222222222222";
        let acl_contract_address = "0x3333333333333333333333333333333333333333";

        let assembled = assemble_aux_data(
            contract_address,
            user_address,
            acl_contract_address,
            ChainId::try_from(1_u64).unwrap(),
        )
        .expect("assemble");

        let expected = contract_address[2..].to_string()
            + &user_address[2..]
            + &acl_contract_address[2..]
            + "0000000000000000000000000000000000000000000000000000000000000001";

        assert_eq!(hex::encode(assembled), expected);
    }

    #[test]
    fn malformed_address_is_an_error() {
        assert!(assemble_aux_data(
            "not-an-address",
            "0x2222222222222222222222222222222222222222",
            "0x3333333333333333333333333333333333333333",
            ChainId::try_from(1_u64).unwrap(),
        )
        .is_err());
    }

    #[test]
    fn solana_layout_is_bytes32_identities_then_big_endian_chain_id() {
        use crate::chain_id::SOLANA_CHAIN_TYPE_BIT;
        let assembled = assemble_solana_aux_data(
            &format!("0x{}", "11".repeat(32)),
            &format!("0x{}", "22".repeat(32)),
            &format!("0x{}", "33".repeat(32)),
            ChainId::from_canonical_u64(SOLANA_CHAIN_TYPE_BIT | 12345),
        )
        .expect("assemble");
        assert_eq!(&assembled[0..32], &[0x11; 32]);
        assert_eq!(&assembled[32..64], &[0x22; 32]);
        assert_eq!(&assembled[64..96], &[0x33; 32]);
        let expected_chain_id =
            alloy::primitives::U256::from(SOLANA_CHAIN_TYPE_BIT | 12345).to_be_bytes::<32>();
        assert_eq!(&assembled[96..128], &expected_chain_id);
    }

    #[test]
    fn parse_bytes32_accepts_hex_and_base58_for_same_identity() {
        let hex = "0x9c7da263cccb5084844e292a2ce0db0e51bbf310100656aa4572b83dfe35fca5";
        let base58 = "BXsiKq6Jg4vgdBqSd75NbMbKaB7WFKK48NVXx4zoeLsW";
        assert_eq!(parse_bytes32(hex).unwrap(), parse_bytes32(base58).unwrap());
    }
}
