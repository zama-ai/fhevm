use crate::*;
use anchor_spl::token_interface::Mint;

#[derive(Accounts)]
#[instruction(params: QuoteOFTParams)]
pub struct QuoteOFT<'info> {
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

impl QuoteOFT<'_> {
    pub fn apply(ctx: &Context<QuoteOFT>, params: &QuoteOFTParams) -> Result<QuoteOFTResult> {
        require!(!ctx.accounts.oft_store.paused, OFTError::Paused);

        let (amount_sent_ld, amount_received_ld, oft_fee_ld) = compute_fee_and_adjust_amount(
            params.amount_ld,
            &ctx.accounts.oft_store,
            &ctx.accounts.token_mint,
            ctx.accounts.peer.fee_bps,
        )?;
        require!(amount_received_ld >= params.min_amount_ld, OFTError::SlippageExceeded);

        let oft_limits = OFTLimits { min_amount_ld: 0, max_amount_ld: 0xffffffffffffffff };
        let mut oft_fee_details = if amount_received_ld + oft_fee_ld < amount_sent_ld {
            vec![OFTFeeDetail {
                fee_amount_ld: amount_sent_ld - oft_fee_ld - amount_received_ld,
                description: "Token2022 Transfer Fee".to_string(),
            }]
        } else {
            vec![]
        };
        // cross chain fee
        if oft_fee_ld > 0 {
            oft_fee_details.push(OFTFeeDetail {
                fee_amount_ld: oft_fee_ld,
                description: "Cross Chain Fee".to_string(),
            });
        }
        let oft_receipt = OFTReceipt { amount_sent_ld, amount_received_ld };
        Ok(QuoteOFTResult { oft_limits, oft_fee_details, oft_receipt })
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct QuoteOFTParams {
    pub dst_eid: u32,
    pub to: [u8; 32],
    pub amount_ld: u64,
    pub min_amount_ld: u64,
    pub options: Vec<u8>,
    pub compose_msg: Option<Vec<u8>>,
    pub pay_in_lz_token: bool,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct QuoteOFTResult {
    pub oft_limits: OFTLimits,
    pub oft_fee_details: Vec<OFTFeeDetail>,
    pub oft_receipt: OFTReceipt,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct OFTFeeDetail {
    pub fee_amount_ld: u64,
    pub description: String,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct OFTReceipt {
    pub amount_sent_ld: u64,
    pub amount_received_ld: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct OFTLimits {
    pub min_amount_ld: u64,
    pub max_amount_ld: u64,
}
