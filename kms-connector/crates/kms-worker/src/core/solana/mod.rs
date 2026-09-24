//! Solana account authorization, using the shared permit and ACL crates.

pub mod delegation;
pub mod encrypted_store;
pub mod failure;
pub mod handle_binding;
pub mod pipeline;
pub mod proof;
pub mod public_decrypt;
pub mod snapshot;
pub mod watermark;

use proof::CoprocessorProofClient;
use snapshot::SolanaRpcClient;
use solana_pubkey::Pubkey;
use zama_solana_acl::{PERMIT_INVALIDATION_SEED, WILDCARD_APP, delegation_seeds};

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

/// The delegation row of `delegator → delegate` in the application `(app_program, app_scope)`.
pub fn delegation_address(
    program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    app_program: SolanaPubkeyBytes,
    app_scope: SolanaPubkeyBytes,
) -> (SolanaPubkeyBytes, u8) {
    find_address(
        program_id,
        &delegation_seeds(&delegator, &delegate, &app_program, &app_scope),
    )
}

/// The delegation row of `delegator → delegate` that covers every application.
pub fn wildcard_delegation_address(
    program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
) -> (SolanaPubkeyBytes, u8) {
    delegation_address(program_id, delegator, delegate, WILDCARD_APP, WILDCARD_APP)
}

fn find_address(program_id: SolanaPubkeyBytes, seeds: &[&[u8]]) -> (SolanaPubkeyBytes, u8) {
    let (address, bump) = Pubkey::find_program_address(seeds, &Pubkey::new_from_array(program_id));
    (address.to_bytes(), bump)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The seed order, pinned to the address the host program derives for the same inputs
    /// (`sdk_fixture_delegation_address_and_instruction_bytes` in the Mollusk suite).
    #[test]
    fn delegation_rows_derive_the_host_programs_addresses() {
        let program_id = Pubkey::from_str_const("DPq5y89RDZPq9NcMh9X1NgjBWgYmSXg3QoipSBV3ZMzQ");
        let address = |(key, _)| Pubkey::new_from_array(key).to_string();
        assert_eq!(
            address(delegation_address(
                program_id.to_bytes(),
                [0x11; 32],
                [0x22; 32],
                [0x33; 32],
                [0x44; 32],
            )),
            "GkmqVNMzqxopBjPkSkZvuLuDE6Jze3iA3Mq5ZHr6SrtJ"
        );
        assert_eq!(
            address(wildcard_delegation_address(
                program_id.to_bytes(),
                [0x11; 32],
                [0x22; 32],
            )),
            "J4BMamYLJvJroFATJp48L6AeQJDQqv86YAQyPqvBcKq1"
        );
    }
}
