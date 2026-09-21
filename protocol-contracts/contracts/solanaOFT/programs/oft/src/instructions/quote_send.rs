use crate::*;
use oapp::endpoint::{instructions::QuoteParams, MessagingFee};

use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_fee::{TransferFee, TransferFeeConfig},
            BaseStateWithExtensions, StateWithExtensions,
        },
        state::Mint as MintState,
    },
    token_interface::Mint,
};

#[derive(Accounts)]
#[instruction(params: QuoteSendParams)]
pub struct QuoteSend<'info> {
    #[account(
        seeds = [OFT_SEED, oft_store.token_escrow.as_ref()],
        bump = oft_store.bump
    )]
    pub oft_store: Account<'info, OFTStore>,
    #[account(
        seeds = [
            PEER_SEED,
            oft_store.key().as_ref(),
            &params.dst_eid.to_be_bytes()
        ],
        bump = peer.bump
    )]
    pub peer: Account<'info, PeerConfig>,
    #[account(address = oft_store.token_mint)]
    pub token_mint: InterfaceAccount<'info, Mint>,
}

impl QuoteSend<'_> {
    pub fn apply(ctx: &Context<QuoteSend>, params: &QuoteSendParams) -> Result<MessagingFee> {
        require!(!ctx.accounts.oft_store.paused, OFTError::Paused);

        let (_, amount_received_ld, _) = compute_fee_and_adjust_amount(
            params.amount_ld,
            &ctx.accounts.oft_store,
            &ctx.accounts.token_mint,
            ctx.accounts.peer.fee_bps,
        )?;
        require!(amount_received_ld >= params.min_amount_ld, OFTError::SlippageExceeded);

        // calling endpoint cpi
        oapp::endpoint_cpi::quote(
            ctx.accounts.oft_store.endpoint_program,
            ctx.remaining_accounts,
            QuoteParams {
                sender: ctx.accounts.oft_store.key(),
                dst_eid: params.dst_eid,
                receiver: ctx.accounts.peer.peer_address,
                message: msg_codec::encode(
                    params.to,
                    amount_received_ld,
                    Pubkey::default(),
                    &params.compose_msg,
                ),
                pay_in_lz_token: params.pay_in_lz_token,
                options: ctx
                    .accounts
                    .peer
                    .enforced_options
                    .combine_options(&params.compose_msg, &params.options)?,
            },
        )
    }
}

pub fn compute_fee_and_adjust_amount(
    amount_ld: u64,
    oft_store: &OFTStore,
    token_mint: &InterfaceAccount<Mint>,
    fee_bps: Option<u16>,
) -> Result<(u64, u64, u64)> {
    let (amount_sent_ld, amount_received_ld, oft_fee_ld) = if OFTType::Adapter == oft_store.oft_type
    {
        let mut amount_received_ld =
            oft_store.remove_dust(get_post_fee_amount_ld(token_mint, amount_ld)?);
        let amount_sent_ld = get_pre_fee_amount_ld(token_mint, amount_received_ld)?;

        // remove the oft fee from the amount_received_ld
        let oft_fee_ld = oft_store.remove_dust(calculate_fee(
            amount_received_ld,
            oft_store.default_fee_bps,
            fee_bps,
        ));
        amount_received_ld -= oft_fee_ld;
        (amount_sent_ld, amount_received_ld, oft_fee_ld)
    } else {
        // if it is Native OFT, there is no transfer fee
        let amount_sent_ld = oft_store.remove_dust(amount_ld);
        let oft_fee_ld = oft_store.remove_dust(calculate_fee(
            amount_sent_ld,
            oft_store.default_fee_bps,
            fee_bps,
        ));
        let amount_received_ld = amount_sent_ld - oft_fee_ld;
        (amount_sent_ld, amount_received_ld, oft_fee_ld)
    };
    Ok((amount_sent_ld, amount_received_ld, oft_fee_ld))
}

fn calculate_fee(pre_fee_amount: u64, default_fee_bps: u16, fee_bps: Option<u16>) -> u64 {
    let final_fee_bps = if let Some(bps) = fee_bps { bps as u128 } else { default_fee_bps as u128 };
    if final_fee_bps == 0 || pre_fee_amount == 0 {
        0
    } else {
        // pre_fee_amount * final_fee_bps / ONE_IN_BASIS_POINTS
        let fee = (pre_fee_amount as u128) * final_fee_bps;
        (fee / ONE_IN_BASIS_POINTS) as u64
    }
}

pub fn get_post_fee_amount_ld(token_mint: &InterfaceAccount<Mint>, amount_ld: u64) -> Result<u64> {
    let token_mint_info = token_mint.to_account_info();
    let token_mint_data = token_mint_info.try_borrow_data()?;
    let token_mint_ext = StateWithExtensions::<MintState>::unpack(&token_mint_data)?;
    let post_amount_ld =
        if let Ok(transfer_fee_config) = token_mint_ext.get_extension::<TransferFeeConfig>() {
            transfer_fee_config
                .get_epoch_fee(Clock::get()?.epoch)
                .calculate_post_fee_amount(amount_ld)
                .ok_or(ProgramError::InvalidArgument)?
        } else {
            amount_ld
        };
    Ok(post_amount_ld)
}

// Calculate the amount_sent_ld necessary to receive amount_received_ld
// Does *not* de-dust any inputs or outputs.
fn get_pre_fee_amount_ld(token_mint: &InterfaceAccount<Mint>, amount_ld: u64) -> Result<u64> {
    let token_mint_info = token_mint.to_account_info();
    let token_mint_data = token_mint_info.try_borrow_data()?;
    let token_mint_ext = StateWithExtensions::<MintState>::unpack(&token_mint_data)?;
    let pre_amount_ld =
        if let Ok(transfer_fee) = token_mint_ext.get_extension::<TransferFeeConfig>() {
            calculate_pre_fee_amount(transfer_fee.get_epoch_fee(Clock::get()?.epoch), amount_ld)
                .ok_or(ProgramError::InvalidArgument)?
        } else {
            amount_ld
        };
    Ok(pre_amount_ld)
}

// DO NOT CHANGE THIS CODE!!!
// bug reported on token2022: https://github.com/solana-labs/solana-program-library/pull/6704/files
// copy code over as fix has not been published
pub const MAX_FEE_BASIS_POINTS: u16 = 10_000;
const ONE_IN_BASIS_POINTS: u128 = MAX_FEE_BASIS_POINTS as u128;
fn calculate_pre_fee_amount(fee: &TransferFee, post_fee_amount: u64) -> Option<u64> {
    let maximum_fee = u64::from(fee.maximum_fee);
    let transfer_fee_basis_points = u16::from(fee.transfer_fee_basis_points) as u128;
    match (transfer_fee_basis_points, post_fee_amount) {
        // no fee, same amount
        (0, _) => Some(post_fee_amount),
        // 0 zero out, 0 in
        (_, 0) => Some(0),
        // 100%, cap at max fee
        (ONE_IN_BASIS_POINTS, _) => maximum_fee.checked_add(post_fee_amount),
        _ => {
            let numerator = (post_fee_amount as u128).checked_mul(ONE_IN_BASIS_POINTS)?;
            let denominator = ONE_IN_BASIS_POINTS.checked_sub(transfer_fee_basis_points)?;
            let raw_pre_fee_amount = ceil_div(numerator, denominator)?;

            if raw_pre_fee_amount.checked_sub(post_fee_amount as u128)? >= maximum_fee as u128 {
                post_fee_amount.checked_add(maximum_fee)
            } else {
                // should return `None` if `pre_fee_amount` overflows
                u64::try_from(raw_pre_fee_amount).ok()
            }
        },
    }
}

fn ceil_div(numerator: u128, denominator: u128) -> Option<u128> {
    numerator.checked_add(denominator)?.checked_sub(1)?.checked_div(denominator)
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct QuoteSendParams {
    pub dst_eid: u32,
    pub to: [u8; 32],
    pub amount_ld: u64,
    pub min_amount_ld: u64,
    pub options: Vec<u8>,
    pub compose_msg: Option<Vec<u8>>,
    pub pay_in_lz_token: bool,
}
