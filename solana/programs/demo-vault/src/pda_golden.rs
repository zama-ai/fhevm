use crate::{constants::*, pda_vectors::PdaVectors};
use anchor_lang::prelude::*;

#[test]
fn pda_golden() {
    let vectors = PdaVectors::load();
    let vault = vectors.key("vault");
    let pdas = [
        (
            "vaultAuthority",
            Pubkey::find_program_address(&[VAULT_AUTHORITY_SEED, vault.as_ref()], &crate::ID),
        ),
        (
            "shareMint",
            Pubkey::find_program_address(&[SHARE_MINT_SEED, vault.as_ref()], &crate::ID),
        ),
        (
            "vaultTokenAccount",
            Pubkey::find_program_address(&[VAULT_TOKEN_ACCOUNT_SEED, vault.as_ref()], &crate::ID),
        ),
    ];
    vectors.check("demoVault", crate::ID, &pdas);
}
