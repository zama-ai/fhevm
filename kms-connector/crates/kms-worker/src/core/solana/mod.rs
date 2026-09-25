//! Solana account authorization, using the shared permit and ACL crates.

pub mod delegation;
pub mod encrypted_store;
pub mod failure;
pub mod handle_binding;
pub mod pipeline;
pub mod proof;
pub mod public_decrypt;
pub mod snapshot;
pub mod verifier;
pub mod watermark;

pub use verifier::SolanaDecryptionVerifier;

use proof::CoprocessorProofClient;
use snapshot::SolanaRpcClient;
use solana_pubkey::Pubkey;
use zama_solana_acl::{PERMIT_INVALIDATION_SEED, WILDCARD_APP, delegation_seeds};

/// The readers both Solana decryption paths authorize through, for one host chain.
#[derive(Clone, Debug)]
pub struct SolanaHost {
    pub program_id: Pubkey,
    pub reader: SolanaRpcClient,
    pub proofs: CoprocessorProofClient,
}

pub fn permit_invalidation_address(program_id: Pubkey, user: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PERMIT_INVALIDATION_SEED, user.as_ref()], &program_id)
}

/// The delegation row of `delegator → delegate` in the application `(app_program, app_scope)`.
pub fn delegation_address(
    program_id: Pubkey,
    delegator: Pubkey,
    delegate: Pubkey,
    app_program: Pubkey,
    app_scope: Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &delegation_seeds(
            delegator.as_array(),
            delegate.as_array(),
            app_program.as_array(),
            app_scope.as_array(),
        ),
        &program_id,
    )
}

/// The delegation row of `delegator → delegate` that covers every application.
pub fn wildcard_delegation_address(
    program_id: Pubkey,
    delegator: Pubkey,
    delegate: Pubkey,
) -> (Pubkey, u8) {
    let wildcard = Pubkey::new_from_array(WILDCARD_APP);
    delegation_address(program_id, delegator, delegate, wildcard, wildcard)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The seed order, pinned to the address the host program derives for the same inputs
    /// (`sdk_fixture_delegation_address_and_instruction_bytes` in the Mollusk suite).
    #[test]
    fn delegation_rows_derive_the_host_programs_addresses() {
        let program_id = Pubkey::from_str_const("DPq5y89RDZPq9NcMh9X1NgjBWgYmSXg3QoipSBV3ZMzQ");
        let address = |(key, _): (Pubkey, u8)| key.to_string();
        let key = |byte| Pubkey::new_from_array([byte; 32]);
        assert_eq!(
            address(delegation_address(
                program_id,
                key(0x11),
                key(0x22),
                key(0x33),
                key(0x44),
            )),
            "GkmqVNMzqxopBjPkSkZvuLuDE6Jze3iA3Mq5ZHr6SrtJ"
        );
        assert_eq!(
            address(wildcard_delegation_address(
                program_id,
                key(0x11),
                key(0x22),
            )),
            "J4BMamYLJvJroFATJp48L6AeQJDQqv86YAQyPqvBcKq1"
        );
    }
}
