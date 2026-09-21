use crate::*;
use anchor_lang::solana_program;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::spl_token_2022::{self, solana_program::program_option::COption},
    token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use oapp::endpoint::{
    cpi::accounts::Clear,
    instructions::{ClearParams, SendComposeParams},
    ConstructCPIContext,
};

#[event_cpi]
#[derive(Accounts)]
#[instruction(params: LzReceiveParams)]
pub struct LzReceive<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [
            PEER_SEED,
            oft_store.key().as_ref(),
            &params.src_eid.to_be_bytes()
        ],
        bump = peer.bump,
        constraint = peer.peer_address == params.sender @OFTError::InvalidSender
    )]
    pub peer: Account<'info, PeerConfig>,
    #[account(
        mut,
        seeds = [OFT_SEED, oft_store.token_escrow.as_ref()],
        bump = oft_store.bump
    )]
    pub oft_store: Account<'info, OFTStore>,
    #[account(
        mut,
        address = oft_store.token_escrow,
        token::authority = oft_store,
        token::mint = token_mint,
        token::token_program = token_program
    )]
    pub token_escrow: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: the wallet address to receive the token
    #[account(address = Pubkey::from(msg_codec::send_to(&params.message)) @OFTError::InvalidTokenDest)]
    pub to_address: AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = to_address,
        associated_token::token_program = token_program
    )]
    pub token_dest: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        address = oft_store.token_mint,
        mint::token_program = token_program
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,
    // Only used for native mint, the mint authority can be:
    //      1. a spl-token multisig account with oft_store as one of the signers, and the quorum **MUST** be 1-of-n. (recommended)
    //      2. or the mint_authority is oft_store itself.
    #[account(constraint = token_mint.mint_authority == COption::Some(mint_authority.key()) @OFTError::InvalidMintAuthority)]
    pub mint_authority: Option<AccountInfo<'info>>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl LzReceive<'_> {
    pub fn apply(ctx: &mut Context<LzReceive>, params: &LzReceiveParams) -> Result<()> {
        require!(!ctx.accounts.oft_store.paused, OFTError::Paused);

        let oft_store_seed = ctx.accounts.token_escrow.key();
        let seeds: &[&[u8]] = &[OFT_SEED, oft_store_seed.as_ref(), &[ctx.accounts.oft_store.bump]];

        // Validate and clear the payload
        let accounts_for_clear = &ctx.remaining_accounts[0..Clear::MIN_ACCOUNTS_LEN];
        let _ = oapp::endpoint_cpi::clear(
            ctx.accounts.oft_store.endpoint_program,
            ctx.accounts.oft_store.key(),
            accounts_for_clear,
            seeds,
            ClearParams {
                receiver: ctx.accounts.oft_store.key(),
                src_eid: params.src_eid,
                sender: params.sender,
                nonce: params.nonce,
                guid: params.guid,
                message: params.message.clone(),
            },
        )?;

        // Convert the amount from sd to ld
        let amount_sd = msg_codec::amount_sd(&params.message);
        let mut amount_received_ld = ctx.accounts.oft_store.sd2ld(amount_sd);

        // Consume the inbound rate limiter
        if let Some(rate_limiter) = ctx.accounts.peer.inbound_rate_limiter.as_mut() {
            rate_limiter.try_consume(amount_received_ld)?;
        }
        // Refill the outbound rate limiter
        if let Some(rate_limiter) = ctx.accounts.peer.outbound_rate_limiter.as_mut() {
            rate_limiter.refill(amount_received_ld)?;
        }

        if ctx.accounts.oft_store.oft_type == OFTType::Adapter {
            // unlock from escrow
            ctx.accounts.oft_store.tvl_ld -= amount_received_ld;
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
                amount_received_ld,
                ctx.accounts.token_mint.decimals,
            )?;

            // update the amount_received_ld with the post transfer fee amount
            amount_received_ld =
                get_post_fee_amount_ld(&ctx.accounts.token_mint, amount_received_ld)?
        } else if let Some(mint_authority) = &ctx.accounts.mint_authority {
            // Native type
            // mint
            let ix = spl_token_2022::instruction::mint_to(
                ctx.accounts.token_program.key,
                &ctx.accounts.token_mint.key(),
                &ctx.accounts.token_dest.key(),
                mint_authority.key,
                &[&ctx.accounts.oft_store.key()],
                amount_received_ld,
            )?;
            solana_program::program::invoke_signed(
                &ix,
                &[
                    ctx.accounts.token_dest.to_account_info(),
                    ctx.accounts.token_mint.to_account_info(),
                    mint_authority.to_account_info(),
                    ctx.accounts.oft_store.to_account_info(),
                ],
                &[&seeds],
            )?;
        } else {
            return Err(OFTError::InvalidMintAuthority.into());
        }

        if let Some(message) = msg_codec::compose_msg(&params.message) {
            oapp::endpoint_cpi::send_compose(
                ctx.accounts.oft_store.endpoint_program,
                ctx.accounts.oft_store.key(),
                &ctx.remaining_accounts[Clear::MIN_ACCOUNTS_LEN..],
                seeds,
                SendComposeParams {
                    to: ctx.accounts.to_address.key(),
                    guid: params.guid,
                    index: 0, // only 1 compose msg per lzReceive
                    message: compose_msg_codec::encode(
                        params.nonce,
                        params.src_eid,
                        amount_received_ld,
                        &message,
                    ),
                },
            )?;
        }

        emit_cpi!(OFTReceived {
            guid: params.guid,
            src_eid: params.src_eid,
            to: ctx.accounts.to_address.key(),
            amount_received_ld,
        });
        Ok(())
    }
}
