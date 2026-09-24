//! Solana account authorization, using the shared permit and ACL crates.

pub mod delegation;
pub mod encrypted_store;
pub mod failure;
pub mod handle_binding;
pub mod pipeline;
pub mod proof;
pub mod public_decrypt;
pub mod scope;
pub mod snapshot;
pub mod watermark;

use proof::CoprocessorProofClient;
use snapshot::SolanaRpcClient;
use solana_pubkey::Pubkey;
use zama_solana_acl::{DELEGATION_SEED, PERMIT_INVALIDATION_SEED, WILDCARD_AUTHORITY};

pub type SolanaPubkeyBytes = [u8; 32];
pub type HandleBytes = [u8; 32];

/// The readers both Solana decryption paths authorize through, for one host chain.
#[derive(Clone, Debug)]
pub struct SolanaHost {
    pub program_id: SolanaPubkeyBytes,
    pub reader: SolanaRpcClient,
    pub proofs: CoprocessorProofClient,
}

pub fn permit_invalidation_address(
    program_id: SolanaPubkeyBytes,
    user: SolanaPubkeyBytes,
) -> (SolanaPubkeyBytes, u8) {
    find_address(program_id, &[PERMIT_INVALIDATION_SEED, &user])
}

pub fn delegation_address(
    program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    authority: SolanaPubkeyBytes,
) -> (SolanaPubkeyBytes, u8) {
    find_address(
        program_id,
        &[DELEGATION_SEED, &delegator, &delegate, &authority],
    )
}

/// The delegation row of `(delegator, delegate)` that covers every authority.
pub fn wildcard_delegation_address(
    program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
) -> (SolanaPubkeyBytes, u8) {
    delegation_address(program_id, delegator, delegate, WILDCARD_AUTHORITY)
}

fn find_address(program_id: SolanaPubkeyBytes, seeds: &[&[u8]]) -> (SolanaPubkeyBytes, u8) {
    let (address, bump) = Pubkey::find_program_address(seeds, &Pubkey::new_from_array(program_id));
    (address.to_bytes(), bump)
}
