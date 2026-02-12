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

const HCU_METER_SEED: &[u8] = b"hcu_meter";
const HCU_GLOBAL_SEED: &[u8] = b"hcu_global";
const HCU_GLOBAL_LIMIT_PER_TX: u64 = 20_000_000;
const HCU_GLOBAL_LIMIT_PER_WINDOW: u64 = 20_000_000;
const HCU_WINDOW_SIZE_SLOTS: u64 = 1;
const HCU_DEPTH_LIMIT_PER_TX: u64 = 5_000_000;
const HCU_MAX_TRACKED_HANDLES: usize = 128;

#[program]
pub mod zama_host {
    use super::*;

    pub fn begin_hcu_meter(ctx: Context<BeginHcuMeter>, meter_id: [u8; 16]) -> Result<()> {
        let meter = &mut ctx.accounts.meter;
        meter.authority = ctx.accounts.authority.key();
        meter.meter_id = meter_id;
        meter.tx_total_hcu = 0;
        meter.tracked_handles = Vec::new();
        ctx.accounts
            .hcu_global
            .initialize_defaults(current_hcu_window_id()?);
        Ok(())
    }

    pub fn close_hcu_meter(ctx: Context<CloseHcuMeter>, _meter_id: [u8; 16]) -> Result<()> {
        let current_window = current_hcu_window_id()?;
        let global = &mut ctx.accounts.hcu_global;
        global.roll_window_if_needed(current_window);
        global.charge_window(ctx.accounts.meter.tx_total_hcu)?;
        Ok(())
    }

    pub fn request_add(
        ctx: Context<RequestOp>,
        lhs: [u8; 32],
        rhs: [u8; 32],
        is_scalar: bool,
    ) -> Result<()> {
        let result_handle = derive_binary_result_handle(lhs, rhs, is_scalar, OP_ADD);
        let op_hcu = binary_op_hcu(OP_ADD, is_scalar)?;
        if is_scalar {
            charge_hcu(
                &mut ctx.accounts.meter,
                ctx.accounts.caller.key(),
                op_hcu,
                &[lhs],
                result_handle,
            )?;
        } else {
            charge_hcu(
                &mut ctx.accounts.meter,
                ctx.accounts.caller.key(),
                op_hcu,
                &[lhs, rhs],
                result_handle,
            )?;
        }

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
        let result_handle = derive_binary_result_handle(lhs, rhs, is_scalar, OP_SUB);
        let op_hcu = binary_op_hcu(OP_SUB, is_scalar)?;
        if is_scalar {
            charge_hcu(
                &mut ctx.accounts.meter,
                ctx.accounts.caller.key(),
                op_hcu,
                &[lhs],
                result_handle,
            )?;
        } else {
            charge_hcu(
                &mut ctx.accounts.meter,
                ctx.accounts.caller.key(),
                op_hcu,
                &[lhs, rhs],
                result_handle,
            )?;
        }

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

        let op_hcu = binary_op_hcu(opcode, is_scalar)?;
        let result_handle = derive_binary_result_handle(lhs, rhs, is_scalar, opcode);
        if is_scalar {
            charge_hcu(
                &mut ctx.accounts.meter,
                ctx.accounts.caller.key(),
                op_hcu,
                &[lhs],
                result_handle,
            )?;
        } else {
            charge_hcu(
                &mut ctx.accounts.meter,
                ctx.accounts.caller.key(),
                op_hcu,
                &[lhs, rhs],
                result_handle,
            )?;
        }

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

        let op_hcu = unary_op_hcu(opcode)?;
        let result_handle = derive_unary_result_handle(input, opcode);
        charge_hcu(
            &mut ctx.accounts.meter,
            ctx.accounts.caller.key(),
            op_hcu,
            &[input],
            result_handle,
        )?;

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
        let op_hcu = if_then_else_hcu();
        let result_handle =
            derive_ternary_result_handle(control, if_true, if_false, OP_IF_THEN_ELSE);
        charge_hcu(
            &mut ctx.accounts.meter,
            ctx.accounts.caller.key(),
            op_hcu,
            &[control, if_true, if_false],
            result_handle,
        )?;

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
        let op_hcu = cast_hcu();
        let result_handle = derive_unary_result_handle(input, OP_CAST ^ to_type);
        charge_hcu(
            &mut ctx.accounts.meter,
            ctx.accounts.caller.key(),
            op_hcu,
            &[input],
            result_handle,
        )?;

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
        let op_hcu = trivial_encrypt_hcu();
        let result_handle = derive_unary_result_handle(pt, OP_TRIVIAL_ENCRYPT ^ to_type);
        charge_hcu(
            &mut ctx.accounts.meter,
            ctx.accounts.caller.key(),
            op_hcu,
            &[],
            result_handle,
        )?;

        emit!(OpRequestedTrivialEncrypt {
            caller: ctx.accounts.caller.key(),
            pt,
            to_type,
            result_handle,
        });
        Ok(())
    }

    pub fn request_rand(ctx: Context<RequestOp>, rand_type: u8, seed: [u8; 32]) -> Result<()> {
        let op_hcu = rand_hcu(rand_type)?;
        let result_handle = derive_unary_result_handle(seed, OP_RAND ^ rand_type);
        charge_hcu(
            &mut ctx.accounts.meter,
            ctx.accounts.caller.key(),
            op_hcu,
            &[seed],
            result_handle,
        )?;

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
        let op_hcu = rand_bounded_hcu(rand_type)?;
        let result_handle =
            derive_ternary_result_handle(upper_bound, seed, [rand_type; 32], OP_RAND_BOUNDED);
        charge_hcu(
            &mut ctx.accounts.meter,
            ctx.accounts.caller.key(),
            op_hcu,
            &[upper_bound, seed],
            result_handle,
        )?;

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
}

#[derive(Accounts)]
#[instruction(meter_id: [u8; 16])]
pub struct BeginHcuMeter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + HcuMeter::LEN,
        seeds = [HCU_METER_SEED, authority.key().as_ref(), meter_id.as_ref()],
        bump,
    )]
    pub meter: Account<'info, HcuMeter>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + HcuGlobal::LEN,
        seeds = [HCU_GLOBAL_SEED],
        bump,
    )]
    pub hcu_global: Account<'info, HcuGlobal>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(meter_id: [u8; 16])]
pub struct CloseHcuMeter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    #[account(
        mut,
        close = payer,
        seeds = [HCU_METER_SEED, authority.key().as_ref(), meter_id.as_ref()],
        bump,
        constraint = meter.authority == authority.key() @ HostError::InvalidHcuAuthority,
        constraint = meter.meter_id == meter_id @ HostError::InvalidHcuMeter,
    )]
    pub meter: Account<'info, HcuMeter>,
    #[account(
        mut,
        seeds = [HCU_GLOBAL_SEED],
        bump,
    )]
    pub hcu_global: Account<'info, HcuGlobal>,
}

#[derive(Accounts)]
pub struct RequestOp<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,
    #[account(
        mut,
        constraint = meter.authority == caller.key() @ HostError::InvalidHcuAuthority,
    )]
    pub meter: Account<'info, HcuMeter>,
}

#[derive(Accounts)]
pub struct AllowHandle<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,
}

#[account]
pub struct HcuMeter {
    pub authority: Pubkey,
    pub meter_id: [u8; 16],
    pub tx_total_hcu: u64,
    pub tracked_handles: Vec<HandleDepth>,
}

impl HcuMeter {
    const LEN: usize = 32 + 16 + 8 + 4 + (HCU_MAX_TRACKED_HANDLES * HandleDepth::LEN);

    fn charge(&mut self, op_hcu: u64, dependencies: &[[u8; 32]], result: [u8; 32]) -> Result<()> {
        let next_tx_total = self
            .tx_total_hcu
            .checked_add(op_hcu)
            .ok_or(HostError::HcuArithmeticOverflow)?;
        require!(
            next_tx_total < HCU_GLOBAL_LIMIT_PER_TX,
            HostError::HcuTransactionLimitExceeded
        );
        self.tx_total_hcu = next_tx_total;

        let parent_depth = dependencies
            .iter()
            .map(|dependency| self.depth_for_handle(dependency))
            .max()
            .unwrap_or(0);
        let next_depth = parent_depth
            .checked_add(op_hcu)
            .ok_or(HostError::HcuArithmeticOverflow)?;
        require!(
            next_depth < HCU_DEPTH_LIMIT_PER_TX,
            HostError::HcuTransactionDepthLimitExceeded
        );

        self.set_handle_depth(result, next_depth)
    }

    fn depth_for_handle(&self, handle: &[u8; 32]) -> u64 {
        self.tracked_handles
            .iter()
            .rev()
            .find(|entry| &entry.handle == handle)
            .map(|entry| entry.depth_hcu)
            .unwrap_or(0)
    }

    fn set_handle_depth(&mut self, handle: [u8; 32], depth_hcu: u64) -> Result<()> {
        if let Some(existing) = self
            .tracked_handles
            .iter_mut()
            .find(|entry| entry.handle == handle)
        {
            if depth_hcu > existing.depth_hcu {
                existing.depth_hcu = depth_hcu;
            }
            return Ok(());
        }

        require!(
            self.tracked_handles.len() < HCU_MAX_TRACKED_HANDLES,
            HostError::HcuTrackedHandleLimitExceeded
        );
        self.tracked_handles.push(HandleDepth { handle, depth_hcu });
        Ok(())
    }
}

#[account]
pub struct HcuGlobal {
    pub last_seen_window_id: u64,
    pub used_hcu_in_window: u64,
    pub limit_hcu_per_window: u64,
}

impl HcuGlobal {
    const LEN: usize = 8 + 8 + 8;

    fn initialize_defaults(&mut self, window_id: u64) {
        if self.limit_hcu_per_window == 0 {
            self.limit_hcu_per_window = HCU_GLOBAL_LIMIT_PER_WINDOW;
            self.last_seen_window_id = window_id;
            self.used_hcu_in_window = 0;
        }
    }

    fn roll_window_if_needed(&mut self, window_id: u64) {
        if self.last_seen_window_id != window_id {
            self.last_seen_window_id = window_id;
            self.used_hcu_in_window = 0;
        }
    }

    fn charge_window(&mut self, tx_hcu: u64) -> Result<()> {
        let next_window_hcu = self
            .used_hcu_in_window
            .checked_add(tx_hcu)
            .ok_or(HostError::HcuArithmeticOverflow)?;
        require!(
            next_window_hcu <= self.limit_hcu_per_window,
            HostError::HcuWindowLimitExceeded
        );
        self.used_hcu_in_window = next_window_hcu;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct HandleDepth {
    pub handle: [u8; 32],
    pub depth_hcu: u64,
}

impl HandleDepth {
    const LEN: usize = 32 + 8;
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
    #[msg("invalid hcu authority")]
    InvalidHcuAuthority,
    #[msg("invalid hcu meter account")]
    InvalidHcuMeter,
    #[msg("operation supports scalar mode only")]
    OnlyScalarOperationsAreSupported,
    #[msg("hcu transaction limit exceeded")]
    HcuTransactionLimitExceeded,
    #[msg("hcu transaction depth limit exceeded")]
    HcuTransactionDepthLimitExceeded,
    #[msg("hcu arithmetic overflow")]
    HcuArithmeticOverflow,
    #[msg("hcu tracked handle capacity exceeded")]
    HcuTrackedHandleLimitExceeded,
    #[msg("hcu global window limit exceeded")]
    HcuWindowLimitExceeded,
}

fn current_hcu_window_id() -> Result<u64> {
    Ok(Clock::get()?.slot / HCU_WINDOW_SIZE_SLOTS)
}

fn charge_hcu(
    meter: &mut Account<HcuMeter>,
    caller: Pubkey,
    op_hcu: u64,
    dependencies: &[[u8; 32]],
    result: [u8; 32],
) -> Result<()> {
    require!(meter.authority == caller, HostError::InvalidHcuAuthority);
    meter.charge(op_hcu, dependencies, result)
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

fn binary_op_hcu(opcode: u8, is_scalar: bool) -> Result<u64> {
    let hcu = match opcode {
        0 => {
            if is_scalar {
                84_000
            } else {
                88_000
            }
        }
        1 => {
            if is_scalar {
                84_000
            } else {
                91_000
            }
        }
        2 => {
            if is_scalar {
                122_000
            } else {
                150_000
            }
        }
        3 => {
            require!(is_scalar, HostError::OnlyScalarOperationsAreSupported);
            210_000
        }
        4 => {
            require!(is_scalar, HostError::OnlyScalarOperationsAreSupported);
            440_000
        }
        5 => {
            if is_scalar {
                31_000
            } else {
                31_000
            }
        }
        6 => {
            if is_scalar {
                30_000
            } else {
                30_000
            }
        }
        7 => {
            if is_scalar {
                31_000
            } else {
                31_000
            }
        }
        8 => {
            if is_scalar {
                32_000
            } else {
                92_000
            }
        }
        9 => {
            if is_scalar {
                32_000
            } else {
                91_000
            }
        }
        10 => {
            if is_scalar {
                31_000
            } else {
                91_000
            }
        }
        11 => {
            if is_scalar {
                31_000
            } else {
                93_000
            }
        }
        12 => {
            if is_scalar {
                55_000
            } else {
                55_000
            }
        }
        13 => {
            if is_scalar {
                55_000
            } else {
                55_000
            }
        }
        14 => {
            if is_scalar {
                52_000
            } else {
                63_000
            }
        }
        15 => {
            if is_scalar {
                52_000
            } else {
                59_000
            }
        }
        16 => {
            if is_scalar {
                58_000
            } else {
                58_000
            }
        }
        17 => {
            if is_scalar {
                52_000
            } else {
                59_000
            }
        }
        18 => {
            if is_scalar {
                84_000
            } else {
                119_000
            }
        }
        19 => {
            if is_scalar {
                89_000
            } else {
                121_000
            }
        }
        _ => return Err(HostError::InvalidBinaryOpcode.into()),
    };
    Ok(hcu)
}

fn unary_op_hcu(opcode: u8) -> Result<u64> {
    let hcu = match opcode {
        OP_NEG => 79_000,
        OP_NOT => 9,
        _ => return Err(HostError::InvalidUnaryOpcode.into()),
    };
    Ok(hcu)
}

fn if_then_else_hcu() -> u64 {
    55_000
}

fn cast_hcu() -> u64 {
    32
}

fn trivial_encrypt_hcu() -> u64 {
    32
}

fn rand_hcu(_rand_type: u8) -> Result<u64> {
    Ok(23_000)
}

fn rand_bounded_hcu(_rand_type: u8) -> Result<u64> {
    Ok(23_000)
}

fn derive_binary_result_handle(lhs: [u8; 32], rhs: [u8; 32], is_scalar: bool, tag: u8) -> [u8; 32] {
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
