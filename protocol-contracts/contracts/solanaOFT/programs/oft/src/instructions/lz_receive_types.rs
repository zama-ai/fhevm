use crate::*;
use anchor_lang::solana_program;
use anchor_spl::{
    associated_token::{get_associated_token_address_with_program_id, ID as ASSOCIATED_TOKEN_ID},
    token_2022::spl_token_2022::solana_program::program_option::COption,
    token_interface::Mint,
};
use oapp::endpoint_cpi::LzAccount;

#[derive(Accounts)]
pub struct LzReceiveTypes<'info> {
    #[account(
        seeds = [OFT_SEED, oft_store.token_escrow.as_ref()],
        bump = oft_store.bump
    )]
    pub oft_store: Account<'info, OFTStore>,
    #[account(address = oft_store.token_mint)]
    pub token_mint: InterfaceAccount<'info, Mint>,
}

// account structure
// account 0 - payer (executor)
// account 1 - peer
// account 2 - oft store
// account 3 - token escrow
// account 4 - to address / wallet address
// account 5 - token dest
// account 6 - token mint
// account 7 - mint authority (optional)
// account 8 - token program
// account 9 - associated token program
// account 10 - system program
// account 11 - event authority
// account 12 - this program
// account remaining accounts
//      0..9 - accounts for clear
//      9..16 - accounts for compose
impl LzReceiveTypes<'_> {
    pub fn apply(
        ctx: &Context<LzReceiveTypes>,
        params: &LzReceiveParams,
    ) -> Result<Vec<LzAccount>> {
        let (peer, _) = Pubkey::find_program_address(
            &[PEER_SEED, ctx.accounts.oft_store.key().as_ref(), &params.src_eid.to_be_bytes()],
            ctx.program_id,
        );

        // account 0..3
        let mut accounts = vec![
            LzAccount { pubkey: Pubkey::default(), is_signer: true, is_writable: true }, // 0
            LzAccount { pubkey: peer, is_signer: false, is_writable: true },             // 1
            LzAccount { pubkey: ctx.accounts.oft_store.key(), is_signer: false, is_writable: true }, // 2
            LzAccount {
                pubkey: ctx.accounts.oft_store.token_escrow.key(),
                is_signer: false,
                is_writable: true,
            }, // 3
        ];

        // account 4..9
        let to_address = Pubkey::from(msg_codec::send_to(&params.message));
        let token_program = ctx.accounts.token_mint.to_account_info().owner;
        let token_dest = get_associated_token_address_with_program_id(
            &to_address,
            &ctx.accounts.oft_store.token_mint,
            token_program,
        );
        let mint_authority =
            if let COption::Some(mint_authority) = ctx.accounts.token_mint.mint_authority {
                mint_authority
            } else {
                ctx.program_id.key()
            };
        accounts.extend_from_slice(&[
            LzAccount { pubkey: to_address, is_signer: false, is_writable: false }, // 4
            LzAccount { pubkey: token_dest, is_signer: false, is_writable: true },  // 5
            LzAccount {
                pubkey: ctx.accounts.token_mint.key(),
                is_signer: false,
                is_writable: true,
            }, // 6
            LzAccount { pubkey: mint_authority, is_signer: false, is_writable: false }, // 7
            LzAccount { pubkey: *token_program, is_signer: false, is_writable: false }, // 8
            LzAccount { pubkey: ASSOCIATED_TOKEN_ID, is_signer: false, is_writable: false }, // 9
        ]);

        // account 10..12
        let (event_authority_account, _) =
            Pubkey::find_program_address(&[oapp::endpoint_cpi::EVENT_SEED], &ctx.program_id);
        accounts.extend_from_slice(&[
            LzAccount {
                pubkey: solana_program::system_program::ID,
                is_signer: false,
                is_writable: false,
            }, // 10
            LzAccount { pubkey: event_authority_account, is_signer: false, is_writable: false }, // 11
            LzAccount { pubkey: ctx.program_id.key(), is_signer: false, is_writable: false }, // 12
        ]);

        let endpoint_program = ctx.accounts.oft_store.endpoint_program;
        // remaining accounts 0..9
        let accounts_for_clear = oapp::endpoint_cpi::get_accounts_for_clear(
            endpoint_program,
            &ctx.accounts.oft_store.key(),
            params.src_eid,
            &params.sender,
            params.nonce,
        );
        accounts.extend(accounts_for_clear);

        // remaining accounts 9..16
        if let Some(message) = msg_codec::compose_msg(&params.message) {
            let amount_sd = msg_codec::amount_sd(&params.message);
            let amount_ld = ctx.accounts.oft_store.sd2ld(amount_sd);
            let amount_received_ld = if ctx.accounts.oft_store.oft_type == OFTType::Native {
                amount_ld
            } else {
                get_post_fee_amount_ld(&ctx.accounts.token_mint, amount_ld)?
            };

            let accounts_for_composing = oapp::endpoint_cpi::get_accounts_for_send_compose(
                endpoint_program,
                &ctx.accounts.oft_store.key(),
                &to_address,
                &params.guid,
                0,
                &compose_msg_codec::encode(
                    params.nonce,
                    params.src_eid,
                    amount_received_ld,
                    &message,
                ),
            );
            accounts.extend(accounts_for_composing);
        }

        Ok(accounts)
    }
}
