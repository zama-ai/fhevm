//! Shared Solana byte types and the host program's account addresses.
//!
//! Authorization itself lives in [`super::solana`]; this module holds what several of its rules
//! and the public-decrypt path share — the pubkey and handle aliases and the PDA derivations of the two singleton-shaped records the pipeline reads (the
//! host config and a delegation row). The encrypted store's address is derived from its
//! own fields in [`super::solana::encrypted_store`].

use sha2::{Digest, Sha256};
use solana_pubkey::Pubkey;

pub type SolanaPubkeyBytes = [u8; 32];
pub type HandleBytes = [u8; 32];

// The record layout, its decoder and the wildcard sentinel live in the shared crate, so every
// off-chain reader (this connector's authoritative check, the relayer's advisory pre-check)
// decodes the same bytes through one implementation.
pub use zama_solana_acl::WILDCARD_AUTHORITY;
pub use zama_solana_acl::delegation::DELEGATION_SEED;

pub use zama_solana_acl::HOST_CONFIG_SEED;
const ANCHOR_DISCRIMINATOR_LEN: usize = 8;

pub fn host_config_address(host_program_id: SolanaPubkeyBytes) -> (SolanaPubkeyBytes, u8) {
    let host_program_id = Pubkey::new_from_array(host_program_id);
    let (address, bump) = Pubkey::find_program_address(&[HOST_CONFIG_SEED], &host_program_id);
    (address.to_bytes(), bump)
}

pub fn user_decryption_delegation_address(
    host_program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    authority: SolanaPubkeyBytes,
) -> (SolanaPubkeyBytes, u8) {
    let host_program_id = Pubkey::new_from_array(host_program_id);
    let (address, bump) = Pubkey::find_program_address(
        &[
            DELEGATION_SEED,
            delegator.as_ref(),
            delegate.as_ref(),
            authority.as_ref(),
        ],
        &host_program_id,
    );
    (address.to_bytes(), bump)
}

pub fn anchor_account_discriminator(account_name: &str) -> [u8; ANCHOR_DISCRIMINATOR_LEN] {
    let mut hasher = Sha256::new();
    hasher.update(b"account:");
    hasher.update(account_name.as_bytes());
    let digest = hasher.finalize();
    let mut discriminator = [0; ANCHOR_DISCRIMINATOR_LEN];
    discriminator.copy_from_slice(&digest[..ANCHOR_DISCRIMINATOR_LEN]);
    discriminator
}
