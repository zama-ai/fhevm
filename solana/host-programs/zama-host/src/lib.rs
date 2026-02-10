use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWqkZ4FK6s7vY8J3xA5rJQbSq");

const OP_ADD: u8 = 0;
const OP_SUB: u8 = 1;
const OP_NEG: u8 = 20;
const OP_NOT: u8 = 21;
const OP_CAST: u8 = 23;
const OP_TRIVIAL_ENCRYPT: u8 = 24;
const OP_IF_THEN_ELSE: u8 = 25;
const OP_RAND: u8 = 26;
const OP_RAND_BOUNDED: u8 = 27;

#[program]
pub mod zama_host {
    use super::*;

    pub fn request_add(
        ctx: Context<RequestOp>,
        lhs: [u8; 32],
        rhs: [u8; 32],
        is_scalar: bool,
    ) -> Result<()> {
        let result_handle = derive_result_handle_with_tag(lhs, rhs, is_scalar, OP_ADD);

        emit!(OpRequestedAdd {
            caller: ctx.accounts.caller.key(),
            lhs,
            rhs,
            is_scalar,
            result_handle,
        });

        Ok(())
    }

    pub fn request_sub(
        ctx: Context<RequestOp>,
        lhs: [u8; 32],
        rhs: [u8; 32],
        is_scalar: bool,
    ) -> Result<()> {
        let result_handle = derive_result_handle_with_tag(lhs, rhs, is_scalar, OP_SUB);

        emit!(OpRequestedSub {
            caller: ctx.accounts.caller.key(),
            lhs,
            rhs,
            is_scalar,
            result_handle,
        });

        Ok(())
    }

    pub fn request_binary_op(
        ctx: Context<RequestOp>,
        lhs: [u8; 32],
        rhs: [u8; 32],
        is_scalar: bool,
        opcode: u8,
    ) -> Result<()> {
        validate_binary_opcode(opcode)?;
        let result_handle = derive_result_handle_with_tag(lhs, rhs, is_scalar, opcode);

        emit!(OpRequestedBinary {
            caller: ctx.accounts.caller.key(),
            lhs,
            rhs,
            is_scalar,
            result_handle,
            opcode,
        });

        Ok(())
    }

    pub fn request_unary_op(ctx: Context<RequestOp>, input: [u8; 32], opcode: u8) -> Result<()> {
        validate_unary_opcode(opcode)?;
        let result_handle = derive_unary_result_handle(input, opcode);

        emit!(OpRequestedUnary {
            caller: ctx.accounts.caller.key(),
            input,
            result_handle,
            opcode,
        });

        Ok(())
    }

    pub fn request_if_then_else(
        ctx: Context<RequestOp>,
        control: [u8; 32],
        if_true: [u8; 32],
        if_false: [u8; 32],
    ) -> Result<()> {
        let result_handle =
            derive_ternary_result_handle(control, if_true, if_false, OP_IF_THEN_ELSE);

        emit!(OpRequestedIfThenElse {
            caller: ctx.accounts.caller.key(),
            control,
            if_true,
            if_false,
            result_handle,
        });

        Ok(())
    }

    pub fn request_cast(ctx: Context<RequestOp>, input: [u8; 32], to_type: u8) -> Result<()> {
        let result_handle = derive_unary_result_handle(input, OP_CAST ^ to_type);

        emit!(OpRequestedCast {
            caller: ctx.accounts.caller.key(),
            input,
            to_type,
            result_handle,
        });

        Ok(())
    }

    pub fn request_trivial_encrypt(
        ctx: Context<RequestOp>,
        pt: [u8; 32],
        to_type: u8,
    ) -> Result<()> {
        let result_handle = derive_unary_result_handle(pt, OP_TRIVIAL_ENCRYPT ^ to_type);

        emit!(OpRequestedTrivialEncrypt {
            caller: ctx.accounts.caller.key(),
            pt,
            to_type,
            result_handle,
        });

        Ok(())
    }

    pub fn request_rand(ctx: Context<RequestOp>, rand_type: u8, seed: [u8; 32]) -> Result<()> {
        let result_handle = derive_unary_result_handle(seed, OP_RAND ^ rand_type);

        emit!(OpRequestedRand {
            caller: ctx.accounts.caller.key(),
            rand_type,
            seed,
            result_handle,
        });

        Ok(())
    }

    pub fn request_rand_bounded(
        ctx: Context<RequestOp>,
        upper_bound: [u8; 32],
        rand_type: u8,
        seed: [u8; 32],
    ) -> Result<()> {
        let result_handle =
            derive_ternary_result_handle(upper_bound, seed, [rand_type; 32], OP_RAND_BOUNDED);

        emit!(OpRequestedRandBounded {
            caller: ctx.accounts.caller.key(),
            upper_bound,
            rand_type,
            seed,
            result_handle,
        });

        Ok(())
    }

    pub fn allow(ctx: Context<AllowHandle>, handle: [u8; 32], account: Pubkey) -> Result<()> {
        emit!(HandleAllowed {
            caller: ctx.accounts.caller.key(),
            handle,
            account,
        });

        Ok(())
    }

    pub fn request_add_cpi(
        ctx: Context<RequestOpCpi>,
        lhs: [u8; 32],
        rhs: [u8; 32],
        is_scalar: bool,
    ) -> Result<()> {
        let result_handle = derive_result_handle_with_tag(lhs, rhs, is_scalar, OP_ADD);

        emit_cpi!(OpRequestedAdd {
            caller: ctx.accounts.caller.key(),
            lhs,
            rhs,
            is_scalar,
            result_handle,
        });

        Ok(())
    }

    pub fn request_sub_cpi(
        ctx: Context<RequestOpCpi>,
        lhs: [u8; 32],
        rhs: [u8; 32],
        is_scalar: bool,
    ) -> Result<()> {
        let result_handle = derive_result_handle_with_tag(lhs, rhs, is_scalar, OP_SUB);

        emit_cpi!(OpRequestedSub {
            caller: ctx.accounts.caller.key(),
            lhs,
            rhs,
            is_scalar,
            result_handle,
        });

        Ok(())
    }

    pub fn request_binary_op_cpi(
        ctx: Context<RequestOpCpi>,
        lhs: [u8; 32],
        rhs: [u8; 32],
        is_scalar: bool,
        opcode: u8,
    ) -> Result<()> {
        validate_binary_opcode(opcode)?;
        let result_handle = derive_result_handle_with_tag(lhs, rhs, is_scalar, opcode);

        emit_cpi!(OpRequestedBinary {
            caller: ctx.accounts.caller.key(),
            lhs,
            rhs,
            is_scalar,
            result_handle,
            opcode,
        });

        Ok(())
    }

    pub fn request_unary_op_cpi(
        ctx: Context<RequestOpCpi>,
        input: [u8; 32],
        opcode: u8,
    ) -> Result<()> {
        validate_unary_opcode(opcode)?;
        let result_handle = derive_unary_result_handle(input, opcode);

        emit_cpi!(OpRequestedUnary {
            caller: ctx.accounts.caller.key(),
            input,
            result_handle,
            opcode,
        });

        Ok(())
    }

    pub fn request_if_then_else_cpi(
        ctx: Context<RequestOpCpi>,
        control: [u8; 32],
        if_true: [u8; 32],
        if_false: [u8; 32],
    ) -> Result<()> {
        let result_handle =
            derive_ternary_result_handle(control, if_true, if_false, OP_IF_THEN_ELSE);

        emit_cpi!(OpRequestedIfThenElse {
            caller: ctx.accounts.caller.key(),
            control,
            if_true,
            if_false,
            result_handle,
        });

        Ok(())
    }

    pub fn request_cast_cpi(
        ctx: Context<RequestOpCpi>,
        input: [u8; 32],
        to_type: u8,
    ) -> Result<()> {
        let result_handle = derive_unary_result_handle(input, OP_CAST ^ to_type);

        emit_cpi!(OpRequestedCast {
            caller: ctx.accounts.caller.key(),
            input,
            to_type,
            result_handle,
        });

        Ok(())
    }

    pub fn request_trivial_encrypt_cpi(
        ctx: Context<RequestOpCpi>,
        pt: [u8; 32],
        to_type: u8,
    ) -> Result<()> {
        let result_handle = derive_unary_result_handle(pt, OP_TRIVIAL_ENCRYPT ^ to_type);

        emit_cpi!(OpRequestedTrivialEncrypt {
            caller: ctx.accounts.caller.key(),
            pt,
            to_type,
            result_handle,
        });

        Ok(())
    }

    pub fn request_rand_cpi(
        ctx: Context<RequestOpCpi>,
        rand_type: u8,
        seed: [u8; 32],
    ) -> Result<()> {
        let result_handle = derive_unary_result_handle(seed, OP_RAND ^ rand_type);

        emit_cpi!(OpRequestedRand {
            caller: ctx.accounts.caller.key(),
            rand_type,
            seed,
            result_handle,
        });

        Ok(())
    }

    pub fn request_rand_bounded_cpi(
        ctx: Context<RequestOpCpi>,
        upper_bound: [u8; 32],
        rand_type: u8,
        seed: [u8; 32],
    ) -> Result<()> {
        let result_handle =
            derive_ternary_result_handle(upper_bound, seed, [rand_type; 32], OP_RAND_BOUNDED);

        emit_cpi!(OpRequestedRandBounded {
            caller: ctx.accounts.caller.key(),
            upper_bound,
            rand_type,
            seed,
            result_handle,
        });

        Ok(())
    }

    pub fn allow_cpi(
        ctx: Context<AllowHandleCpi>,
        handle: [u8; 32],
        account: Pubkey,
    ) -> Result<()> {
        emit_cpi!(HandleAllowed {
            caller: ctx.accounts.caller.key(),
            handle,
            account,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RequestOp<'info> {
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
pub struct RequestOpCpi<'info> {
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
pub struct OpRequestedAdd {
    pub caller: Pubkey,
    pub lhs: [u8; 32],
    pub rhs: [u8; 32],
    pub is_scalar: bool,
    pub result_handle: [u8; 32],
}

#[event]
pub struct OpRequestedSub {
    pub caller: Pubkey,
    pub lhs: [u8; 32],
    pub rhs: [u8; 32],
    pub is_scalar: bool,
    pub result_handle: [u8; 32],
}

#[event]
pub struct OpRequestedBinary {
    pub caller: Pubkey,
    pub lhs: [u8; 32],
    pub rhs: [u8; 32],
    pub is_scalar: bool,
    pub result_handle: [u8; 32],
    pub opcode: u8,
}

#[event]
pub struct OpRequestedUnary {
    pub caller: Pubkey,
    pub input: [u8; 32],
    pub result_handle: [u8; 32],
    pub opcode: u8,
}

#[event]
pub struct OpRequestedIfThenElse {
    pub caller: Pubkey,
    pub control: [u8; 32],
    pub if_true: [u8; 32],
    pub if_false: [u8; 32],
    pub result_handle: [u8; 32],
}

#[event]
pub struct OpRequestedCast {
    pub caller: Pubkey,
    pub input: [u8; 32],
    pub to_type: u8,
    pub result_handle: [u8; 32],
}

#[event]
pub struct OpRequestedTrivialEncrypt {
    pub caller: Pubkey,
    pub pt: [u8; 32],
    pub to_type: u8,
    pub result_handle: [u8; 32],
}

#[event]
pub struct OpRequestedRand {
    pub caller: Pubkey,
    pub rand_type: u8,
    pub seed: [u8; 32],
    pub result_handle: [u8; 32],
}

#[event]
pub struct OpRequestedRandBounded {
    pub caller: Pubkey,
    pub upper_bound: [u8; 32],
    pub rand_type: u8,
    pub seed: [u8; 32],
    pub result_handle: [u8; 32],
}

#[event]
pub struct HandleAllowed {
    pub caller: Pubkey,
    pub handle: [u8; 32],
    pub account: Pubkey,
}

#[error_code]
pub enum HostError {
    #[msg("invalid binary opcode")]
    InvalidBinaryOpcode,
    #[msg("invalid unary opcode")]
    InvalidUnaryOpcode,
}

fn validate_binary_opcode(opcode: u8) -> Result<()> {
    require!(opcode <= 19, HostError::InvalidBinaryOpcode);
    Ok(())
}

fn validate_unary_opcode(opcode: u8) -> Result<()> {
    require!(
        opcode == OP_NEG || opcode == OP_NOT,
        HostError::InvalidUnaryOpcode
    );
    Ok(())
}

fn derive_result_handle_with_tag(
    lhs: [u8; 32],
    rhs: [u8; 32],
    is_scalar: bool,
    tag: u8,
) -> [u8; 32] {
    let mut output = [0u8; 32];
    for i in 0..32 {
        output[i] = lhs[i] ^ rhs[i];
    }
    output[29] ^= tag;
    if is_scalar {
        output[31] ^= 0x01;
    }
    output
}

fn derive_unary_result_handle(input: [u8; 32], tag: u8) -> [u8; 32] {
    let mut output = input;
    output[30] ^= tag;
    output
}

fn derive_ternary_result_handle(
    first: [u8; 32],
    second: [u8; 32],
    third: [u8; 32],
    tag: u8,
) -> [u8; 32] {
    let mut output = [0u8; 32];
    for i in 0..32 {
        output[i] = first[i] ^ second[i] ^ third[i];
    }
    output[28] ^= tag;
    output
}
