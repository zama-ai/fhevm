//! Encrypted-value-ACL lineage CPI helpers.
//!
//! The token→host CPI that creates, rotates, and publicly-marks the single
//! durable ACL lineage backing a rotating value (confidential balance / total
//! supply), plus the wrap-time "credit a public amount into a lineage" eval.
//! Extracted from `common.rs` as one cohesive unit (encrypted-value ACL + MMR
//! PoC, fhevm-internal#1569).

use super::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::{InstructionData, ToAccountInfos, ToAccountMetas};

/// Invokes a zama-host instruction signed by a token PDA, supplying the host
/// program through the account infos only — never the instruction metas — so
/// the callee observes no spurious remaining account. Mirrors the manual-CPI
/// pattern in the `zama_fhe` crate.
fn invoke_zama_host_signed<'info, A>(
    zama_program: &AccountInfo<'info>,
    accounts: A,
    data: Vec<u8>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()>
where
    A: ToAccountMetas + ToAccountInfos<'info>,
{
    let instruction = Instruction {
        program_id: *zama_program.key,
        accounts: accounts.to_account_metas(None),
        data,
    };
    let mut infos = accounts.to_account_infos();
    infos.push(zama_program.clone());
    invoke_signed(&instruction, &infos, signer_seeds)?;
    Ok(())
}

/// Token→host CPI account infos shared by every encrypted-value ACL lineage
/// upsert. Grouped into one struct so the upsert helper stays a few clean
/// arguments rather than a long positional list.
pub(crate) struct LineageCpi<'info> {
    pub(crate) zama_program: AccountInfo<'info>,
    pub(crate) encrypted_value_acl: AccountInfo<'info>,
    pub(crate) payer: AccountInfo<'info>,
    pub(crate) system_program: AccountInfo<'info>,
}

/// The app-authority PDA that signs a lineage upsert as the owning value's
/// authority, plus the deterministic `(app_account, label)` the lineage is keyed
/// by. `balance` is owner-scoped (the token-account PDA); `total_supply` is
/// mint-scoped (the total-supply authority PDA).
pub(crate) struct LineageAuthority<'info> {
    account: AccountInfo<'info>,
    seeds: Vec<Vec<u8>>,
    app_account: Pubkey,
    label: [u8; 32],
}

impl<'info> LineageAuthority<'info> {
    pub(crate) fn balance(token_account: &Account<'info, ConfidentialTokenAccount>) -> Self {
        Self {
            account: token_account.to_account_info(),
            seeds: vec![
                b"token-account".to_vec(),
                token_account.mint.to_bytes().to_vec(),
                token_account.owner.to_bytes().to_vec(),
                vec![token_account.bump],
            ],
            app_account: token_account.key(),
            label: balance_label(),
        }
    }

    pub(crate) fn total_supply(
        authority: &UncheckedAccount<'info>,
        mint: Pubkey,
        bump: u8,
    ) -> Self {
        Self {
            account: authority.to_account_info(),
            seeds: vec![
                b"total-supply".to_vec(),
                mint.to_bytes().to_vec(),
                vec![bump],
            ],
            app_account: authority.key(),
            label: total_supply_label(),
        }
    }

    /// The account that keys this lineage's PDA (the value's app authority).
    pub(crate) fn app_account(&self) -> Pubkey {
        self.app_account
    }
}

/// Asserts `cpi.encrypted_value_acl` is the canonical lineage PDA for this
/// `(domain, app_account, label)` and returns the lineage `value_key`. The sole
/// PDA guard shared by every lineage CPI, so callers can never operate on a
/// substituted ACL account.
fn assert_lineage_pda(
    cpi: &LineageCpi<'_>,
    authority: &LineageAuthority<'_>,
    acl_domain_key: Pubkey,
) -> Result<[u8; 32]> {
    let value_key =
        zama_host::acl_nonce_key(acl_domain_key, authority.app_account, authority.label);
    let (expected_acl, _) = zama_host::encrypted_value_acl_address(value_key);
    require_keys_eq!(
        cpi.encrypted_value_acl.key(),
        expected_acl,
        ConfidentialTokenError::EncryptedValueAclMismatch
    );
    Ok(value_key)
}

/// Creates or rotates the encrypted-value ACL lineage that is the sole durable
/// ACL for a rotating value (balance or total supply; encrypted-value ACL + MMR
/// PoC, fhevm-internal#1569).
///
/// The value's app-authority PDA signs. On the first handle replacement the
/// lineage is lazily initialized; subsequent replacements rotate it — appending
/// a historical-access MMR leaf per durable subject for the OLD handle before
/// recording the new one. This is the handle-replacement rotation seam: the
/// lineage is reused across rotations instead of minting a fresh keyed-nonce ACL
/// record each time.
pub(crate) fn upsert_value_acl<'info>(
    cpi: &LineageCpi<'info>,
    authority: LineageAuthority<'info>,
    acl_domain_key: Pubkey,
    new_handle: [u8; 32],
    subjects: Vec<Pubkey>,
) -> Result<()> {
    let value_key = assert_lineage_pda(cpi, &authority, acl_domain_key)?;

    let seed_slices: Vec<&[u8]> = authority.seeds.iter().map(Vec::as_slice).collect();
    let signer_seeds: &[&[&[u8]]] = &[&seed_slices];

    if cpi.encrypted_value_acl.owner == &anchor_lang::solana_program::system_program::ID {
        invoke_zama_host_signed(
            &cpi.zama_program,
            zama_host::cpi::accounts::InitializeEncryptedValueAcl {
                payer: cpi.payer.clone(),
                app_account_authority: authority.account.clone(),
                encrypted_value_acl: cpi.encrypted_value_acl.clone(),
                system_program: cpi.system_program.clone(),
            },
            zama_host::instruction::InitializeEncryptedValueAcl {
                value_key,
                acl_domain_key,
                encrypted_value_label: authority.label,
                handle: new_handle,
                subjects,
            }
            .data(),
            signer_seeds,
        )
    } else {
        invoke_zama_host_signed(
            &cpi.zama_program,
            zama_host::cpi::accounts::UpdateEncryptedValueAcl {
                payer: cpi.payer.clone(),
                app_account_authority: authority.account.clone(),
                encrypted_value_acl: cpi.encrypted_value_acl.clone(),
                system_program: cpi.system_program.clone(),
            },
            zama_host::instruction::RotateEncryptedValue {
                new_handle,
                new_subjects: subjects,
            }
            .data(),
            signer_seeds,
        )
    }
}

/// Records an exact public-decrypt MMR leaf on an existing lineage value, marking
/// its CURRENT handle publicly decryptable. The value's app-authority PDA signs
/// (the same authority a rotation uses), so only the owning program can disclose.
/// The lineage must already exist — a value that was never minted has no handle to
/// disclose. This is the on-chain trigger the KMS public-decrypt path consumes.
pub(crate) fn mark_value_acl_public<'info>(
    cpi: &LineageCpi<'info>,
    authority: LineageAuthority<'info>,
    acl_domain_key: Pubkey,
) -> Result<()> {
    assert_lineage_pda(cpi, &authority, acl_domain_key)?;
    require!(
        cpi.encrypted_value_acl.owner != &anchor_lang::solana_program::system_program::ID,
        ConfidentialTokenError::EncryptedValueAclMismatch
    );

    let seed_slices: Vec<&[u8]> = authority.seeds.iter().map(Vec::as_slice).collect();
    let signer_seeds: &[&[&[u8]]] = &[&seed_slices];
    invoke_zama_host_signed(
        &cpi.zama_program,
        zama_host::cpi::accounts::UpdateEncryptedValueAcl {
            payer: cpi.payer.clone(),
            app_account_authority: authority.account.clone(),
            encrypted_value_acl: cpi.encrypted_value_acl.clone(),
            system_program: cpi.system_program.clone(),
        },
        zama_host::instruction::MarkEncryptedValuePublic {}.data(),
        signer_seeds,
    )
}

/// One single-domain "add a public amount to a lineage value" operation: a
/// transient eval that reads the lineage's current handle, adds `amount`, and
/// binds the returned handle back into the lineage. Used for both legs of a wrap
/// (balance and total supply), which live in different ACL domains.
pub(crate) struct CreditLineageByAmount<'a, 'info> {
    pub(crate) context: fhe::EvalContext<'a, 'info>,
    pub(crate) eval_authority: fhe::OutputAuthority<'info>,
    pub(crate) cpi: LineageCpi<'info>,
    pub(crate) lineage: LineageAuthority<'info>,
    pub(crate) acl_domain_key: Pubkey,
    pub(crate) tag: &'a [u8],
    pub(crate) old_handle: [u8; 32],
    pub(crate) nonce_sequence: u64,
    pub(crate) subjects: Vec<Pubkey>,
}

pub(crate) fn credit_lineage_by_amount(
    req: CreditLineageByAmount<'_, '_>,
    amount: u64,
) -> Result<[u8; 32]> {
    let mut amount_context = [0u8; 32];
    amount_context[24..].copy_from_slice(&amount.to_be_bytes());
    let app_account = req.lineage.app_account();
    let context_id = transfer_eval_context(
        req.tag,
        req.acl_domain_key,
        app_account,
        app_account,
        amount_context,
        req.nonce_sequence,
        req.nonce_sequence,
    )?;
    let current =
        zama_fhe::Uint64Handle::durable_at(req.old_handle, req.cpi.encrypted_value_acl.key())
            .map_err(invalid_eval_plan)?;
    let mut builder =
        zama_fhe::EvalBuilder::new(context_id, zama_fhe::EvalAppAuthority::new(app_account));
    let encrypted_amount = builder
        .trivial_encrypt_u64(amount, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let new_value = builder
        .add(current, encrypted_amount, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let new_value_index = new_value
        .producer_index()
        .ok_or(error!(ConfidentialTokenError::InvalidFheEvalPlan))?;
    let plan = builder.finish().map_err(invalid_eval_plan)?;
    fhe::eval_transient(
        req.context,
        req.eval_authority,
        [req.cpi.encrypted_value_acl.clone()],
        plan,
    )?;
    let new_handle = fhe::read_eval_output_handle(new_value_index)?;
    upsert_value_acl(
        &req.cpi,
        req.lineage,
        req.acl_domain_key,
        new_handle,
        req.subjects,
    )?;
    Ok(new_handle)
}
