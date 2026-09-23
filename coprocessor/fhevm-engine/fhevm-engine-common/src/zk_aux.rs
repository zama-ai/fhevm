//! Auxiliary ("metadata") data bound into an input ZK proof.
//!
//! Both the prover and the verifier must assemble these bytes identically, or the proof
//! fails to verify. The layout lives here so the two sides cannot drift:
//!
//! ```text
//! contract_addr (20) || user_addr (20) || acl_contract_addr (20) || chain_id (32 BE)
//! ```
//!
//! Verifier side: `zkproof-worker`'s `auxiliary::ZkData`, built from the `verify_proofs`
//! row plus the chain's ACL address. Prover side: real clients via the relayer SDK, and
//! [`crate::synthetic_input`] for the blue-green dry-run probe.

use std::str::FromStr;

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
}
