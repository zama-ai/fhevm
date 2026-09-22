use crate::*;

#[derive(Accounts)]
#[instruction(params: SetPauseParams)]
pub struct SetPause<'info> {
    /// pauser or unpauser
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [OFT_SEED, oft_store.token_escrow.as_ref()],
        bump = oft_store.bump,
        constraint = is_valid_signer(signer.key(), &oft_store, params.paused) @OFTError::Unauthorized
    )]
    pub oft_store: Account<'info, OFTStore>,
}

impl SetPause<'_> {
    pub fn apply(ctx: &mut Context<SetPause>, params: &SetPauseParams) -> Result<()> {
        ctx.accounts.oft_store.paused = params.paused;
        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SetPauseParams {
    pub paused: bool,
}

fn is_valid_signer(signer: Pubkey, oft_store: &OFTStore, paused: bool) -> bool {
    if paused {
        oft_store.pauser == Some(signer)
    } else {
        oft_store.unpauser == Some(signer)
    }
}
