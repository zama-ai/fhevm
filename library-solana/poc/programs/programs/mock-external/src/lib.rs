use acl::cpi::accounts::{Allow as AclAllow, InitHandle as AclInitHandle};
use acl::program::Acl;
use acl::state::{Config as AclConfig, DISCRIMINATOR};
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
        output_index: u128,
    ) -> Result<()> {
        let auth_bump = ctx.bumps.authority;
        let auth_seeds: &[&[u8]] = &[AUTHORITY_SEED, &[auth_bump]];

        let user = ctx.accounts.payer.key();

        let acl_program = ctx.accounts.acl_program.to_account_info();
        let payer = ctx.accounts.payer.to_account_info();
        let authority = ctx.accounts.authority.to_account_info();
        let permission_list = ctx.accounts.permission_list.to_account_info();
        let acl_config = ctx.accounts.acl_config.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        // The handle is currently not checked. We will get to it when we approach this design.

        // Init the handle. Only the `authority` PDA needs to sign — the
        // permission_list is a PDA owned by ACL, and ACL's `init_if_needed`
        // signs for its own create_account CPI from within its program
        // context.
        let init_signer_seeds: &[&[&[u8]]] = &[auth_seeds];
        let cpi_ctx = CpiContext::new_with_signer(
            acl_program.clone(),
            AclInitHandle {
                payer: payer.clone(),
                authority: authority.clone(),
                permission_list: permission_list.clone(),
                acl_config: acl_config.clone(),
                system_program: system_program.clone(),
                program: ctx.accounts.acl_program.to_account_info(),
                event_authority: ctx.accounts.acl_event_authority.to_account_info(),
            },
            init_signer_seeds,
        );
        acl::cpi::init_handle(cpi_ctx, handle, app, output_index)?;

        // Allow the handle for the user (signer) and the specified app.
        // Only the `authority` PDA needs to sign; the permission_list is now
        // owned by ACL and modified through its program logic.
        let allow_signer_seeds: &[&[&[u8]]] = &[auth_seeds];
        for context_key in [user, app] {
            let cpi_ctx = CpiContext::new_with_signer(
                acl_program.clone(),
                AclAllow {
                    authority: authority.clone(),
                    permission_list: permission_list.clone(),
                    acl_config: acl_config.clone(),
                    program: ctx.accounts.acl_program.to_account_info(),
                    event_authority: ctx.accounts.acl_event_authority.to_account_info(),
                },
                allow_signer_seeds,
            );
            acl::cpi::allow(cpi_ctx, handle, context_key, app, output_index)?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(handle: [u8; 32], app: Pubkey, output_index: u128)]
pub struct AllowExternalInput<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// This PDA is init_if_needed only for the mock case.
    /// In real example it should be initialized with a separate instruction
    #[account(init_if_needed, payer = payer, space = DISCRIMINATOR + ExternalInputAuthority::INIT_SPACE, seeds = [AUTHORITY_SEED], bump)]
    pub authority: Account<'info, ExternalInputAuthority>,

    /// CHECK: this account does not belong to the mock, it belongs to the ACL.
    /// For the mock purposes we can enforce its derivation check there,
    /// but in a real external input it may stay different.
    /// `seeds::program = acl_program.key()` is required because the PDA is
    /// owned by ACL — without it Anchor derives the expected address under
    /// the mock's program id and the seeds check fails.
    #[account(
        mut,
        seeds = [PERMISSION_LIST_SEED, app.as_ref(), &output_index.to_le_bytes()],
        bump,
        seeds::program = acl_program.key(),
    )]
    pub permission_list: UncheckedAccount<'info>,

    pub acl_config: Account<'info, AclConfig>,

    /// CHECK: this account does not belong to the mock, it belongs to the ACL.
    /// This is an event account and for easier UX we can check the derivation there
    #[account(
        seeds = [b"__event_authority"],
        seeds::program = acl_program.key(),
        bump,
    )]
    pub acl_event_authority: UncheckedAccount<'info>,

    pub acl_program: Program<'info, Acl>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct ExternalInputAuthority();
