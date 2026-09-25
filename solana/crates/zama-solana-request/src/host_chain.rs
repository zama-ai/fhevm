//! Which kind of host chain a chain id names (DD-052). The id's high byte is the chain type:
//! `0x00` for EVM, whose ids zero-extend into it, and `0x01` for Solana, whose low seven bytes are
//! the published cluster tag. A handle carries its chain id in bytes 22..30.

pub const EVM_CHAIN_TYPE: u8 = 0x00;
pub const SOLANA_CHAIN_TYPE: u8 = 0x01;
const CHAIN_TYPE_SHIFT: u32 = 56;
const CLUSTER_TAG_MASK: u64 = 0x00ff_ffff_ffff_ffff;

pub const fn chain_type_byte(chain_id: u64) -> u8 {
    (chain_id >> CHAIN_TYPE_SHIFT) as u8
}

pub const fn is_evm_host_chain_id(chain_id: u64) -> bool {
    chain_type_byte(chain_id) == EVM_CHAIN_TYPE
}

pub const fn is_solana_host_chain_id(chain_id: u64) -> bool {
    chain_type_byte(chain_id) == SOLANA_CHAIN_TYPE
}

/// A Solana host chain id: type byte `0x01` plus a 56-bit cluster tag.
pub const fn solana_host_chain_id(cluster_tag: u64) -> u64 {
    ((SOLANA_CHAIN_TYPE as u64) << CHAIN_TYPE_SHIFT) | (cluster_tag & CLUSTER_TAG_MASK)
}

/// The chain id a handle embeds, big-endian in bytes 22..30 (`HandleOps.sol`).
pub fn handle_chain_id(handle: &[u8; 32]) -> u64 {
    let mut chain = [0u8; 8];
    chain.copy_from_slice(&handle[22..30]);
    u64::from_be_bytes(chain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_chain_id_is_its_type_byte_and_cluster_tag() {
        let solana = solana_host_chain_id(0x0102_0304);
        assert_eq!(solana, 0x0100_0000_0102_0304);
        assert!(is_solana_host_chain_id(solana) && !is_evm_host_chain_id(solana));
        assert!(is_evm_host_chain_id(31_337) && !is_solana_host_chain_id(31_337));
        assert_eq!(chain_type_byte(0x0200_0000_0000_0001), 0x02);
        assert_eq!(solana_host_chain_id(u64::MAX), 0x01ff_ffff_ffff_ffff);

        let mut handle = [0xaa; 32];
        handle[22..30].copy_from_slice(&solana.to_be_bytes());
        assert_eq!(handle_chain_id(&handle), solana);
    }
}
