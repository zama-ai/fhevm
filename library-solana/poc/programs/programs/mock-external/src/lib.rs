use acl::cpi::accounts::{Allow as AclAllow, InitHandle as AclInitHandle};
use acl::program::Acl;
use acl::state::Config as AclConfig;
use anchor_lang::prelude::*;

declare_id!("GA6G4AuaU95aeu47j2tGkm8UzCqQus9ztPssnQt9PcJ9");

pub const AUTHORITY_SEED: &[u8] = b"external_input_authority";
pub const PERMISSION_LIST_SEED: &[u8] = b"permission_list";

#[program]
pub mod mock_external {
    use super::*;

    pub fn allow_external_input(
        ctx: Context<AllowExternalInput>,
        handle: [u8; 32],
        app: Pubkey,
    ) -> Result<()> {
        let auth_bump = ctx.bumps.authority;
        let pl_bump = ctx.bumps.permission_list;
        let auth_seeds: &[&[u8]] = &[AUTHORITY_SEED, &[auth_bump]];
        let pl_seeds: &[&[u8]] = &[PERMISSION_LIST_SEED, &handle, &[pl_bump]];

        let user = ctx.accounts.payer.key();

        let acl_program = ctx.accounts.acl_program.to_account_info();
        let payer = ctx.accounts.payer.to_account_info();
        let authority = ctx.accounts.authority.to_account_info();
        let permission_list = ctx.accounts.permission_list.to_account_info();
        let acl_config = ctx.accounts.acl_config.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        // Init the handle. Fails with `already in use` if the PDA exists.
        // Sign with both PDAs: `authority` (required by ACL config) and
        // `permission_list` (the new account being created via system CPI).
        let init_signer_seeds: &[&[&[u8]]] = &[auth_seeds, pl_seeds];
        let cpi_ctx = CpiContext::new_with_signer(
            acl_program.clone(),
            AclInitHandle {
                payer: payer.clone(),
                authority: authority.clone(),
                permission_list: permission_list.clone(),
                acl_config: acl_config.clone(),
                system_program: system_program.clone(),
            },
            init_signer_seeds,
        );
        acl::cpi::init_handle(cpi_ctx, handle)?;

        // Allow the handle for the user (signer) and the specified app.
        // Only the `authority` PDA needs to sign; the permission_list is now
        // owned by ACL and modified through its program logic.
        let allow_signer_seeds: &[&[&[u8]]] = &[auth_seeds];
        for context_key in [user, app] {
            let cpi_ctx = CpiContext::new_with_signer(
                acl_program.clone(),
                AclAllow {
                    payer: payer.clone(),
                    authority: authority.clone(),
                    permission_list: permission_list.clone(),
                    acl_config: acl_config.clone(),
                    system_program: system_program.clone(),
                },
                allow_signer_seeds,
            );
            acl::cpi::allow(cpi_ctx, handle, context_key)?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(handle: [u8; 32])]
pub struct AllowExternalInput<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: PDA registered as `external_input_authority` in the ACL config;
    /// only this program can sign for it via `invoke_signed`.
    #[account(seeds = [AUTHORITY_SEED], bump)]
    pub authority: UncheckedAccount<'info>,

    /// CHECK: PDA derived from the handle under this program. Starts uninitialized
    /// and is created by ACL's `init_handle` CPI (signed for here via PDA seeds).
    /// After init, ACL owns it and the `allow` CPI mutates it.
    #[account(
        mut,
        seeds = [PERMISSION_LIST_SEED, handle.as_ref()],
        bump,
    )]
    pub permission_list: UncheckedAccount<'info>,

    pub acl_config: Account<'info, AclConfig>,

    pub acl_program: Program<'info, Acl>,
    pub system_program: Program<'info, System>,
}
