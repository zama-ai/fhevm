use crate::{pda_vectors::PdaVectors, state::*};
use anchor_lang::prelude::*;

#[test]
fn pda_golden() {
    let vectors = PdaVectors::load();
    let pdas = [
        (
            "tokenAccount",
            token_account_address(vectors.key("mint"), vectors.key("owner")),
        ),
        (
            "totalSupplyAuthority",
            total_supply_authority_address(vectors.key("mint")),
        ),
        (
            "vaultAuthority",
            vault_authority_address(vectors.key("mint")),
        ),
        (
            "pendingBurn",
            pending_burn_address(vectors.key("mint"), vectors.key("tokenAccount")),
        ),
        (
            "eventAuthority",
            Pubkey::find_program_address(&[b"__event_authority"], &crate::ID),
        ),
    ];
    vectors.check("confidentialToken", crate::ID, &pdas);
}
