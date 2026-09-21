use crate::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

#[derive(Accounts)]
pub struct WithdrawFee<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [OFT_SEED, oft_store.token_escrow.as_ref()],
        bump = oft_store.bump,
        has_one = admin @OFTError::Unauthorized
    )]
    pub oft_store: Account<'info, OFTStore>,
    #[account(
        address = oft_store.token_mint,
        mint::token_program = token_program
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        address = oft_store.token_escrow,
        token::authority = oft_store,
        token::mint = token_mint,
        token::token_program = token_program
    )]
    pub token_escrow: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = token_mint,
        token::token_program = token_program
    )]
    pub token_dest: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl WithdrawFee<'_> {
    pub fn apply(ctx: &mut Context<WithdrawFee>, params: &WithdrawFeeParams) -> Result<()> {
        require!(
            ctx.accounts.token_escrow.amount - ctx.accounts.oft_store.tvl_ld >= params.fee_ld,
            OFTError::InvalidFee
        );
        let seeds: &[&[u8]] = &[
            OFT_SEED,
            &ctx.accounts.token_escrow.key().to_bytes(),
            &[ctx.accounts.oft_store.bump],
        ];
        token_interface::transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.token_escrow.to_account_info(),
                    mint: ctx.accounts.token_mint.to_account_info(),
                    to: ctx.accounts.token_dest.to_account_info(),
                    authority: ctx.accounts.oft_store.to_account_info(),
                },
            )
            .with_signer(&[&seeds]),
            params.fee_ld,
            ctx.accounts.token_mint.decimals,
        )?;
        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawFeeParams {
    pub fee_ld: u64,
}
