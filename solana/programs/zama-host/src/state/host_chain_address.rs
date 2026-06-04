//! Canonical bytes32 host-chain addresses for the Solana surface (RFC-021).
//!
//! On EVM hosts an address is 20 bytes and the input-proof auxiliary data packs
//! three of them ahead of a 32-byte big-endian chain id, for 92 bytes total
//! (see the coprocessor `zkproof-worker` `ZkData::assemble`). RFC-021 makes the
//! host-chain address canonical at 32 bytes so that EVM and non-EVM hosts share
//! one width: a Solana [`Pubkey`] is already 32 bytes, and a 20-byte EVM address
//! is left-padded with 12 zero bytes (the same shape `abi.encode(address)`
//! produces). The auxiliary layout therefore widens from 92 to 128 bytes:
//! `contract(32) || user(32) || acl(32) || chain_id(32)`.

use super::*;

/// Width of a canonical host-chain address.
pub const HOST_CHAIN_ADDRESS_LEN: usize = 32;
/// Width of an EVM account address, before bytes32 canonicalization.
pub const EVM_ADDRESS_LEN: usize = 20;
/// Width of the bytes32 input-proof auxiliary data:
/// `contract || user || acl_contract || chain_id`.
pub const HOST_CHAIN_AUX_LEN: usize = 4 * HOST_CHAIN_ADDRESS_LEN;

/// A host-chain account address in its RFC-021 canonical bytes32 form.
///
/// This is the single address width shared across the Solana surface. Solana
/// addresses round-trip a [`Pubkey`] verbatim; EVM addresses are carried as the
/// canonical left-padded bytes32 used by `abi.encode(address)`.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct HostChainAddress(pub [u8; HOST_CHAIN_ADDRESS_LEN]);

impl HostChainAddress {
    /// Wraps canonical bytes32 address material.
    pub fn from_bytes(bytes: [u8; HOST_CHAIN_ADDRESS_LEN]) -> Self {
        Self(bytes)
    }

    /// Canonicalizes a Solana [`Pubkey`], which is already 32 bytes.
    pub fn from_pubkey(pubkey: Pubkey) -> Self {
        Self(pubkey.to_bytes())
    }

    /// Canonicalizes a 20-byte EVM address by left-padding with 12 zero bytes,
    /// matching the `abi.encode(address)` bytes32 layout RFC-021 adopts.
    pub fn from_evm_address(evm_address: [u8; EVM_ADDRESS_LEN]) -> Self {
        let mut bytes = [0u8; HOST_CHAIN_ADDRESS_LEN];
        bytes[HOST_CHAIN_ADDRESS_LEN - EVM_ADDRESS_LEN..].copy_from_slice(&evm_address);
        Self(bytes)
    }

    /// Borrows the canonical bytes32 representation.
    pub fn as_bytes(&self) -> &[u8; HOST_CHAIN_ADDRESS_LEN] {
        &self.0
    }

    /// Recovers a Solana [`Pubkey`] from canonical address material.
    pub fn to_pubkey(self) -> Pubkey {
        Pubkey::new_from_array(self.0)
    }

    /// Recovers a 20-byte EVM address, if the canonical form is a left-padded
    /// EVM address (the leading 12 bytes are zero). A Solana address whose first
    /// 12 bytes are nonzero is not an EVM address and yields `None`.
    pub fn to_evm_address(self) -> Option<[u8; EVM_ADDRESS_LEN]> {
        let (prefix, address) = self.0.split_at(HOST_CHAIN_ADDRESS_LEN - EVM_ADDRESS_LEN);
        if prefix.iter().any(|byte| *byte != 0) {
            return None;
        }
        let mut evm_address = [0u8; EVM_ADDRESS_LEN];
        evm_address.copy_from_slice(address);
        Some(evm_address)
    }
}

/// Input-proof auxiliary data over canonical bytes32 host-chain addresses.
///
/// This mirrors the EVM coprocessor `ZkData` (`zkproof-worker/src/auxiliary.rs`)
/// in shape and field order, widened to the RFC-021 bytes32 address surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HostChainAuxData {
    /// App account this input authorization is scoped to.
    pub contract: HostChainAddress,
    /// User the encrypted input belongs to.
    pub user: HostChainAddress,
    /// ACL domain authority the input is bound to.
    pub acl_contract: HostChainAddress,
    /// Host-chain id, encoded as a 32-byte big-endian integer.
    pub chain_id: u64,
}

impl HostChainAuxData {
    /// Assembles the canonical auxiliary bytes:
    /// `contract || user || acl_contract || chain_id` (128 bytes), where the
    /// chain id occupies the trailing 32 bytes big-endian. Mirrors the EVM
    /// `ZkData::assemble`, widened from 92 to 128 bytes per RFC-021.
    pub fn assemble(&self) -> [u8; HOST_CHAIN_AUX_LEN] {
        let mut data = [0u8; HOST_CHAIN_AUX_LEN];
        let mut offset = 0;
        for address in [&self.contract, &self.user, &self.acl_contract] {
            data[offset..offset + HOST_CHAIN_ADDRESS_LEN].copy_from_slice(address.as_bytes());
            offset += HOST_CHAIN_ADDRESS_LEN;
        }
        // Chain id is a 32-byte big-endian integer in the trailing word; the
        // u64 lands in its least-significant 8 bytes.
        let chain_id_word_start = HOST_CHAIN_AUX_LEN - 8;
        data[chain_id_word_start..].copy_from_slice(&self.chain_id.to_be_bytes());
        data
    }
}
