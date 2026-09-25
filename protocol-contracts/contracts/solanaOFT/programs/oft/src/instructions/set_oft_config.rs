use crate::*;
use oapp::endpoint::instructions::SetDelegateParams;

#[derive(Accounts)]
pub struct SetOFTConfig<'info> {
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [OFT_SEED, oft_store.token_escrow.as_ref()],
        bump = oft_store.bump,
        has_one = admin @OFTError::Unauthorized
    )]
    pub oft_store: Account<'info, OFTStore>,
}

impl SetOFTConfig<'_> {
    pub fn apply(ctx: &mut Context<SetOFTConfig>, params: &SetOFTConfigParams) -> Result<()> {
        match params.clone() {
            SetOFTConfigParams::Admin(admin) => {
                ctx.accounts.oft_store.admin = admin;
            },
            SetOFTConfigParams::Delegate(delegate) => {
                let oft_store_seed = ctx.accounts.oft_store.token_escrow.key();
                let seeds: &[&[u8]] =
                    &[OFT_SEED, &oft_store_seed.to_bytes(), &[ctx.accounts.oft_store.bump]];
                let _ = oapp::endpoint_cpi::set_delegate(
                    ctx.accounts.oft_store.endpoint_program,
                    ctx.accounts.oft_store.key(),
                    &ctx.remaining_accounts,
                    seeds,
                    SetDelegateParams { delegate },
                )?;
            },
            SetOFTConfigParams::DefaultFee(fee_bps) => {
                require!(fee_bps < MAX_FEE_BASIS_POINTS, OFTError::InvalidFee);
                ctx.accounts.oft_store.default_fee_bps = fee_bps;
            },
            SetOFTConfigParams::Paused(paused) => {
                ctx.accounts.oft_store.paused = paused;
            },
            SetOFTConfigParams::Pauser(pauser) => {
                ctx.accounts.oft_store.pauser = pauser;
            },
            SetOFTConfigParams::Unpauser(unpauser) => {
                ctx.accounts.oft_store.unpauser = unpauser;
            },
        }
        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub enum SetOFTConfigParams {
    Admin(Pubkey),
    Delegate(Pubkey), // OApp delegate for the endpoint
    DefaultFee(u16),
    Paused(bool),
    Pauser(Option<Pubkey>),
    Unpauser(Option<Pubkey>),
}
