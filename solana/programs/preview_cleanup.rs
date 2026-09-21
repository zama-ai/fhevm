//! Preview-only recovery, authorized by this program's upgrade authority.
//! Recover PDA-owned external accounts before erasing the state used to discover them.
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{bpf_loader_upgradeable, system_program};
use anchor_spl::token::{self, Burn, CloseAccount, Token, TokenAccount};

#[derive(Accounts)]
pub struct PreviewAdmin<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        address = bpf_loader_upgradeable::get_program_data_address(&crate::ID),
        constraint = program_data.upgrade_authority_address == Some(admin.key())
    )]
    pub program_data: Account<'info, ProgramData>,
}

pub fn close_owned_accounts<'info>(ctx: Context<'info, PreviewAdmin<'info>>) -> Result<()> {
    let admin = ctx.accounts.admin.to_account_info();
    for target in ctx.remaining_accounts {
        if target.owner != &crate::ID {
            continue;
        }
        let balance = target.lamports();
        **target.try_borrow_mut_lamports()? = 0;
        **admin.try_borrow_mut_lamports()? = admin
            .lamports()
            .checked_add(balance)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        target.resize(0)?;
        target.assign(&system_program::ID);
    }
    Ok(())
}

#[derive(Accounts)]
pub struct PreviewCloseToken<'info> {
    pub authorization: PreviewAdmin<'info>,
    /// CHECK: checked against the caller-supplied PDA seeds before signing.
    pub authority: UncheckedAccount<'info>,
    #[account(mut, constraint = account.owner == authority.key())]
    pub account: Account<'info, TokenAccount>,
    /// CHECK: token program checks the mint; bound to the token account here.
    #[account(mut, address = account.mint)]
    pub mint: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

/// Reset destroys disposable mock tokens; this instruction is absent from non-preview builds.
pub fn close_token(ctx: Context<PreviewCloseToken>, seeds: Vec<Vec<u8>>) -> Result<()> {
    let seeds: Vec<&[u8]> = seeds.iter().map(Vec::as_slice).collect();
    let authority = Pubkey::create_program_address(&seeds, &crate::ID)
        .map_err(|_| ProgramError::InvalidSeeds)?;
    require_keys_eq!(authority, ctx.accounts.authority.key());
    let signer = [&seeds[..]];
    if ctx.accounts.account.amount > 0 {
        token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.key(),
                Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
                &signer,
            ),
            ctx.accounts.account.amount,
        )?;
    }
    token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.key(),
        CloseAccount {
            account: ctx.accounts.account.to_account_info(),
            destination: ctx.accounts.authorization.admin.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
        &signer,
    ))
}

#[derive(Accounts)]
pub struct PreviewDrain<'info> {
    pub authorization: PreviewAdmin<'info>,
    /// CHECK: PDA seeds and system ownership are checked before transferring.
    #[account(mut, owner = system_program::ID)]
    pub authority: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn drain(ctx: Context<PreviewDrain>, seeds: Vec<Vec<u8>>) -> Result<()> {
    let seeds: Vec<&[u8]> = seeds.iter().map(Vec::as_slice).collect();
    require_keys_eq!(
        Pubkey::create_program_address(&seeds, &crate::ID)
            .map_err(|_| ProgramError::InvalidSeeds)?,
        ctx.accounts.authority.key()
    );
    anchor_lang::system_program::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.key(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.authority.to_account_info(),
                to: ctx.accounts.authorization.admin.to_account_info(),
            },
            &[&seeds],
        ),
        ctx.accounts.authority.lamports(),
    )
}
