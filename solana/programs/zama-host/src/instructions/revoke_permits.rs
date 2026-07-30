//! Revokes a user's permits by raising their invalidation watermark.
//!
//! The instruction writes one number: the later of the stored watermark and the
//! current clock. Everything else about revocation is a reader's rule — a verifier
//! rejects a permit whose validity window starts before this moment, and a missing
//! account reads as zero — so this handler has no list to walk and nothing to
//! enumerate. That is the whole point of the design: one transaction, constant work,
//! however many permits are outstanding. What a raised watermark cannot reach — a
//! permit pre-signed to open in the future — is recorded on [`PermitInvalidation`].
//!
//! Account validation here is manual rather than expressed through typed account
//! wrappers: the program is moving off the framework, and new code does not add to
//! the pile of macro-driven validation that has to be unwound later.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{errors::ZamaHostError, state::*};

/// Accounts for revoking a user's outstanding permits.
///
/// Deliberately small. There is no config account, because pausing the host must not
/// take away a user's ability to revoke — a lever that can be disabled by the operator
/// is not the user's lever. And there is no separate payer: the user pays for their own
/// watermark account, which keeps the signer set to exactly the one identity the
/// watermark is keyed by.
#[derive(Accounts)]
pub struct RevokePermits<'info> {
    /// The user revoking their permits, and the payer for the watermark account.
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: validated manually against the canonical watermark address for `user`,
    /// then created if absent.
    #[account(mut)]
    pub invalidation: UncheckedAccount<'info>,
    /// System program, used when the watermark account has to be created.
    pub system_program: Program<'info, System>,
}

/// Raises the caller's invalidation watermark to the current clock.
pub fn revoke_permits(ctx: Context<RevokePermits>) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;

    let user = ctx.accounts.user.key();
    let invalidation = ctx.accounts.invalidation.to_account_info();

    // The clock is refused rather than coerced when it reads before the epoch. The
    // watermark is unsigned seconds, so a cast would land near the top of the range and
    // permanently kill every permit this user will ever sign — an unrecoverable state
    // produced by a conversion. Failing closed cannot destroy an account.
    let now = u64::try_from(Clock::get()?.unix_timestamp)
        .map_err(|_| error!(ZamaHostError::ClockBeforeEpoch))?;

    // The address is derived from the signer, which is what keys the watermark to an
    // identity: an account belonging to anyone else is simply not at this address, so
    // "move somebody else's watermark" has nowhere to land.
    let (expected_address, bump) = permit_invalidation_address(user);
    require_keys_eq!(
        invalidation.key(),
        expected_address,
        ZamaHostError::PermitInvalidationPdaMismatch
    );
    // Stated here rather than left to the account attribute: every other check in this
    // handler is written by hand because the program is moving off the framework, and a
    // safety property that lives only in a macro disappears silently when the macro does.
    require!(
        invalidation.is_writable,
        ZamaHostError::PermitInvalidationAccountInvalid
    );

    let previous_watermark = if is_absent_watermark(&invalidation)? {
        create_pda_if_needed(
            &ctx.accounts.user.to_account_info(),
            &invalidation,
            &ctx.accounts.system_program.to_account_info(),
            8 + PermitInvalidation::SPACE,
            &[PERMIT_INVALIDATION_SEED, user.as_ref(), &[bump]],
        )?;
        // An absent account is a watermark of zero — the same reading the verifier does,
        // stated in the one place that creates the account.
        0
    } else {
        stored_watermark(&invalidation, user, bump)?
    };

    write_account(
        &invalidation,
        &PermitInvalidation {
            user,
            // Monotonic by construction: the recorded value is a maximum, so a slot whose
            // clock lags cannot resurrect permits this user already killed.
            invalidation_watermark: previous_watermark.max(now),
            bump,
        },
    )
}

/// True when the watermark slot has never been written: system-owned and empty.
///
/// A system-owned empty account cannot be executable, so that combination is refused
/// rather than treated as absent.
fn is_absent_watermark(invalidation: &AccountInfo) -> Result<bool> {
    if invalidation.owner == &System::id() && invalidation.data_is_empty() {
        require!(
            !invalidation.executable,
            ZamaHostError::PermitInvalidationAccountInvalid
        );
        return Ok(true);
    }
    Ok(false)
}

/// Reads the watermark out of an existing record, refusing anything this instruction did
/// not write.
///
/// Four ways an account at the canonical address can fail to be that record: another
/// program owns it, it is the wrong size, it carries another record type's discriminator,
/// or it names a different user. The last one is why the record stores its user at all —
/// the contents are checked against the address rather than trusted because the address
/// looked right.
fn stored_watermark(invalidation: &AccountInfo, user: Pubkey, bump: u8) -> Result<u64> {
    require_keys_eq!(
        *invalidation.owner,
        crate::ID,
        ZamaHostError::PermitInvalidationAccountInvalid
    );
    require!(
        !invalidation.executable,
        ZamaHostError::PermitInvalidationAccountInvalid
    );
    // Exact size, both directions: a shorter account is a truncated or foreign record, and
    // a longer one is a different account type that happens to live here. Neither is
    // reinterpreted.
    require!(
        invalidation.data_len() == 8 + PermitInvalidation::SPACE,
        ZamaHostError::PermitInvalidationAccountInvalid
    );

    let data = invalidation.try_borrow_data()?;
    let mut cursor: &[u8] = &data;
    let record = PermitInvalidation::try_deserialize(&mut cursor)
        .map_err(|_| error!(ZamaHostError::PermitInvalidationAccountInvalid))?;

    require_keys_eq!(
        record.user,
        user,
        ZamaHostError::PermitInvalidationAccountInvalid
    );
    require!(
        record.bump == bump,
        ZamaHostError::PermitInvalidationPdaMismatch
    );

    Ok(record.invalidation_watermark)
}
