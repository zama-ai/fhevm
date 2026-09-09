//! `EncryptedValue` leaf sealing. Persistent handle creation and update happen only through
//! `fhe_execute` output provenance, which allows keys inline on the write; the one instruction
//! here seals an existing value's current handle public. Event-free by design — indexers
//! reconstruct MMR leaves from instruction data, using the shared `zama_solana_acl` crate, not
//! from emitted events.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

use super::common::*;
use crate::{errors::ZamaHostError, state::*};

/// Appends one historical-access leaf per allowed key on `handle`, in list order, at the
/// value's next leaf indexes. Used by `fhe_execute`'s persistent output binding right after the
/// handle is written.
pub(super) fn seal_allow_leaves(
    info: &AccountInfo,
    value: &mut EncryptedValue,
    handle: [u8; 32],
    keys: &[Pubkey],
) -> Result<()> {
    let account_key = info.key().to_bytes();
    for key in keys {
        let commitment = zama_solana_acl::historical_access_leaf_commitment(
            account_key,
            value.leaf_count,
            handle,
            key.to_bytes(),
        );
        zama_solana_acl::mmr_append(&mut value.peaks, &mut value.leaf_count, commitment)
            .map_err(map_mmr_append_error)?;
    }
    Ok(())
}

/// Appends a public-decrypt leaf for `handle` at the encrypted value account's next leaf index.
/// Shared by `make_handle_public` and by `fhe_execute`'s public output binding so both produce a
/// byte-identical public-decrypt commitment.
pub(super) fn append_public_decrypt_leaf(
    info: &AccountInfo,
    value: &mut EncryptedValue,
    handle: [u8; 32],
) -> Result<()> {
    let account_key = info.key().to_bytes();
    let commitment =
        zama_solana_acl::public_decrypt_leaf_commitment(account_key, value.leaf_count, handle);
    zama_solana_acl::mmr_append(&mut value.peaks, &mut value.leaf_count, commitment)
        .map_err(map_mmr_append_error)
}

/// Accounts for `make_handle_public`.
#[derive(Accounts)]
pub struct MakeEncryptedValueHandlePublic<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Must equal `EncryptedValue.encrypted_value_account_authority`.
    pub authority: Signer<'info>,
    /// CHECK: layout and ownership are validated inside the handler via `read_canonical_encrypted_value`.
    #[account(mut)]
    pub encrypted_value: UncheckedAccount<'info>,
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// The value's application deny record, required while the deny list is enabled: making a
    /// handle public is an allow, and a denied application cannot allow.
    pub deny_scope_record: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
}

/// Seals `handle` — the value's current handle — as publicly decryptable by appending a
/// public-decrypt leaf.
///
/// Sealing a handle that is already sealed is accepted, not rejected. The second leaf commits to
/// the same `(account, handle)` fact as the first, so it authorizes nothing new, and what it costs
/// is bounded by the MMR shape: peaks are one per set bit of `leaf_count`, capped at
/// `MAX_MMR_PEAKS`, and the caller's own payer funds every growth step. Rejecting it would need the
/// account to remember which handle is already sealed — new `EncryptedValue` state in the shared
/// crate, the listener, the coprocessor, and the IDL — while a state-free guard could only read
/// back the last leaf when `leaf_count` is odd, accepting or rejecting the same call by parity.
/// Recorded as INVARIANTS #53 (fhevm-internal#1859 §6c).
pub fn make_handle_public(
    ctx: Context<MakeEncryptedValueHandlePublic>,
    handle: [u8; 32],
) -> Result<()> {
    assert_not_paused(&ctx.accounts.host_config)?;
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let info = ctx.accounts.encrypted_value.to_account_info();
    let mut value = read_canonical_encrypted_value(&info)?;
    require_keys_eq!(
        ctx.accounts.authority.key(),
        value.encrypted_value_account_authority,
        ZamaHostError::EncryptedValueAccountAuthorityMismatch
    );
    require!(
        handle == value.current_handle,
        ZamaHostError::EncryptedValuePublicHandleMismatch
    );
    check_scope_not_denied(
        &ctx.accounts.host_config,
        AppScope {
            program: value.program,
            scope: value.scope,
        },
        ctx.accounts.deny_scope_record.as_ref(),
    )?;

    append_public_decrypt_leaf(&info, &mut value, handle)?;

    let space = zama_solana_acl::EncryptedValue::account_size(value.peaks.len());
    grow_account_if_needed(
        &ctx.accounts.payer.to_account_info(),
        &info,
        &ctx.accounts.system_program.to_account_info(),
        space,
    )?;
    write_account(&info, &value)?;
    Ok(())
}

fn map_mmr_append_error(error: zama_solana_acl::AclError) -> anchor_lang::error::Error {
    match error {
        zama_solana_acl::AclError::MmrPeakCapacityExceeded => {
            error!(ZamaHostError::EncryptedValueMmrPeakCapacityExceeded)
        }
        _ => error!(ZamaHostError::EncryptedValueMmrInconsistent),
    }
}

/// Reallocs the account and tops up rent when `target_space` grows past the
/// account's current data length. Never shrinks — the leaf count is monotonic.
pub(super) fn grow_account_if_needed<'info>(
    payer: &AccountInfo<'info>,
    account: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    target_space: usize,
) -> Result<()> {
    if account.data_len() >= target_space {
        return Ok(());
    }
    let rent = Rent::get()?.minimum_balance(target_space);
    if account.lamports() < rent {
        let top_up = rent - account.lamports();
        invoke_signed(
            &system_instruction::transfer(payer.key, account.key, top_up),
            &[payer.clone(), account.clone(), system_program.clone()],
            &[],
        )?;
    }
    account.resize(target_space)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use zama_solana_acl::encrypted_value_account::{reconstruct, EncryptedValueAccountEvent};

    fn value(handle: [u8; 32]) -> EncryptedValue {
        EncryptedValue {
            program: Pubkey::default(),
            encrypted_value_account_authority: Pubkey::default(),
            scope: [0; 32],
            label: [0; 32],
            current_handle: handle,
            leaf_count: 0,
            peaks: Vec::new(),
            bump: 0,
        }
    }

    fn account_key() -> Pubkey {
        Pubkey::new_from_array([9u8; 32])
    }

    fn dummy_info(key: &Pubkey) -> AccountInfo<'_> {
        // The sealing helpers only read `info.key()`; lamports/data/owner are unused, so a
        // minimal system-owned stub suffices.
        static mut LAMPORTS: u64 = 0;
        static OWNER: Pubkey = Pubkey::new_from_array([0; 32]);
        #[allow(static_mut_refs)]
        AccountInfo::new(
            key,
            false,
            false,
            unsafe { &mut LAMPORTS },
            &mut [],
            &OWNER,
            false,
        )
    }

    /// A write that allows two keys and goes public, then an update that allows one: the on-chain
    /// appends reproduce byte-for-byte the peaks the coprocessor derives from the equivalent
    /// `Allowed`/`MarkedPublic` event log.
    #[test]
    fn sealing_matches_shared_value_account_reconstruction() {
        let owner = Pubkey::new_unique();
        let other = Pubkey::new_unique();
        let key = account_key();
        let info = dummy_info(&key);
        let mut v = value([1; 32]);

        seal_allow_leaves(&info, &mut v, [1; 32], &[owner, other]).unwrap();
        append_public_decrypt_leaf(&info, &mut v, [1; 32]).unwrap();
        v.current_handle = [2; 32];
        seal_allow_leaves(&info, &mut v, [2; 32], &[owner]).unwrap();
        assert_eq!(v.leaf_count, 4);

        let events = [
            EncryptedValueAccountEvent::Allowed {
                handle: [1; 32],
                key: owner.to_bytes(),
            },
            EncryptedValueAccountEvent::Allowed {
                handle: [1; 32],
                key: other.to_bytes(),
            },
            EncryptedValueAccountEvent::MarkedPublic { handle: [1; 32] },
            EncryptedValueAccountEvent::Allowed {
                handle: [2; 32],
                key: owner.to_bytes(),
            },
        ];
        let reconstructed = reconstruct(key.to_bytes(), &events);
        assert!(reconstructed.peaks_match(&v.peaks, v.leaf_count));
    }

    #[test]
    fn a_write_with_no_allows_seals_nothing() {
        let key = account_key();
        let info = dummy_info(&key);
        let mut v = value([1; 32]);
        seal_allow_leaves(&info, &mut v, [1; 32], &[]).unwrap();
        assert_eq!(v.leaf_count, 0);
        assert!(v.peaks.is_empty());
    }
}
