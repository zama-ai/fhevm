use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWqkZ4FK6s7vY8J3xA5rJQbSq");

#[program]
pub mod zama_host_v0 {
    use super::*;

    pub fn request_add(
        ctx: Context<RequestAdd>,
        lhs: [u8; 32],
        rhs: [u8; 32],
        is_scalar: bool,
    ) -> Result<()> {
        // Deterministic placeholder handle derivation for scaffold purposes.
        // Final implementation should match gateway/handle semantics.
        let result_handle = derive_result_handle(lhs, rhs, is_scalar);

        emit!(OpRequestedAddV1 {
            caller: ctx.accounts.caller.key(),
            lhs,
            rhs,
            is_scalar,
            result_handle,
        });

        Ok(())
    }

    pub fn allow(ctx: Context<AllowHandle>, handle: [u8; 32], account: Pubkey) -> Result<()> {
        emit!(HandleAllowedV1 {
            caller: ctx.accounts.caller.key(),
            handle,
            account,
        });

        Ok(())
    }

    pub fn request_add_cpi(
        ctx: Context<RequestAddCpi>,
        lhs: [u8; 32],
        rhs: [u8; 32],
        is_scalar: bool,
    ) -> Result<()> {
        let result_handle = derive_result_handle(lhs, rhs, is_scalar);

        emit_cpi!(OpRequestedAddV1 {
            caller: ctx.accounts.caller.key(),
            lhs,
            rhs,
            is_scalar,
            result_handle,
        });

        Ok(())
    }

    pub fn allow_cpi(
        ctx: Context<AllowHandleCpi>,
        handle: [u8; 32],
        account: Pubkey,
    ) -> Result<()> {
        emit_cpi!(HandleAllowedV1 {
            caller: ctx.accounts.caller.key(),
            handle,
            account,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RequestAdd<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,
}

#[derive(Accounts)]
pub struct AllowHandle<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct RequestAddCpi<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct AllowHandleCpi<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,
}

#[event]
pub struct OpRequestedAddV1 {
    pub caller: Pubkey,
    pub lhs: [u8; 32],
    pub rhs: [u8; 32],
    pub is_scalar: bool,
    pub result_handle: [u8; 32],
}

#[event]
pub struct HandleAllowedV1 {
    pub caller: Pubkey,
    pub handle: [u8; 32],
    pub account: Pubkey,
}

fn derive_result_handle(lhs: [u8; 32], rhs: [u8; 32], is_scalar: bool) -> [u8; 32] {
    let mut output = [0u8; 32];
    for i in 0..32 {
        output[i] = lhs[i] ^ rhs[i];
    }
    if is_scalar {
        output[31] ^= 0x01;
    }
    output
}
