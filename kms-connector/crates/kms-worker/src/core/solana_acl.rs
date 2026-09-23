//! Solana byte aliases and the addresses of the host program's singleton-shaped records.

use solana_pubkey::Pubkey;
use zama_solana_acl::{DELEGATION_SEED, HOST_CONFIG_SEED};

pub type SolanaPubkeyBytes = [u8; 32];
pub type HandleBytes = [u8; 32];

pub fn host_config_address(host_program_id: SolanaPubkeyBytes) -> (SolanaPubkeyBytes, u8) {
    find_address(host_program_id, &[HOST_CONFIG_SEED])
}

pub fn user_decryption_delegation_address(
    host_program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    authority: SolanaPubkeyBytes,
) -> (SolanaPubkeyBytes, u8) {
    find_address(
        host_program_id,
        &[DELEGATION_SEED, &delegator, &delegate, &authority],
    )
}

fn find_address(program_id: SolanaPubkeyBytes, seeds: &[&[u8]]) -> (SolanaPubkeyBytes, u8) {
    let (address, bump) = Pubkey::find_program_address(seeds, &Pubkey::new_from_array(program_id));
    (address.to_bytes(), bump)
}
