//! Evaluates ordered instruction-local FHE executions.
//!
//! Two signers cover two authorities, and they are only sometimes the same key: `payer` funds rent
//! for persistent output accounts; `encrypted_value_account_authority` is the default authority
//! that signs for persistent values read and written. Every persistent value an execution touches
//! is admitted by its own authority's signature — found among the default signer and the signing
//! remaining accounts — and nothing else: an application program signs for its PDAs by CPI and
//! forwards a user wallet as `payer`.

use anchor_lang::prelude::*;

use super::common::*;
use super::encrypted_value::{
    append_public_decrypt_leaf, grow_account_if_needed, seal_allow_leaves,
};
use super::input_verification::verify_input_attestation;
use crate::{
    errors::ZamaHostError,
    events::{
        FheExecuteRandomSeed, FheExecuteRandomSeedsEvent, ProducedPublicOutput,
        PublicOutputsProducedEvent,
    },
    state::*,
};

mod account_table;
mod block_cap;
mod event_transport;
mod hcu;
mod preflight;
mod walk;

use account_table::ExecutionAccountTable;
use event_transport::{emit_execution_random_seeds, emit_public_outputs_produced};
use preflight::preflight_execution;
use walk::{walk_steps, ExecutionHandleContext, RandContext};

/// Accounts for one composed, instruction-local fhe_execute.
///
/// Persistent input and output `EncryptedValue` accounts are supplied in
/// `remaining_accounts` and referenced by index from [`FheExecuteArgs`].
#[derive(Accounts)]
#[event_cpi]
pub struct FheExecute<'info> {
    /// Pays rent for any persistent output accounts.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Default authority signer: admits every persistent value read or written that does not name
    /// an authority of its own. An output that sets `authority_index` points at a remaining account
    /// instead, and that account must sign and must equal the authority the output declares; an
    /// operand's authority may likewise be any signing remaining account.
    pub encrypted_value_account_authority: Signer<'info>,
    /// Singleton config PDA. Read-only: the cap is read from here, but the writable per-slot
    /// counter is the separate `hcu_block_meter`, never this singleton — so the hot path takes no
    /// write lock on the config.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// System program used for persistent output creation.
    pub system_program: Program<'info, System>,
    /// Per-application HCU block meter (written once in the execution `charge`). The HCU PDAs
    /// (`hcu_block_meter`, `hcu_trusted_app_record`) key on the `(program, scope)` of the
    /// persistent values the execution touches — an identity the program proved when it created
    /// them, so no caller can rotate a fresh signer to mint a fresh per-slot meter. Untrusted
    /// applications in the metering band MUST supply this meter; trusted applications and the
    /// unrestricted default omit it. An `UncheckedAccount` because it may be uninitialized
    /// (lazy-created) and is validated manually.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// Trust witness (read-only), keyed on `(program, scope)`. Present + program-owned +
    /// `trusted == true` ⇒ bypass the cap; absent (`None`) ⇒ untrusted, fall through to the meter;
    /// present-but-malformed ⇒ reject.
    pub hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
    /// The host's rand nonce, consumed and incremented by an execution that contains a rand step.
    /// Required exactly then, and refused otherwise so no execution write-locks it for nothing.
    #[account(mut, seeds = [RAND_NONCE_SEED], bump = rand_nonce.bump)]
    pub rand_nonce: Option<Account<'info, RandNonce>>,
}

/// Runs one ordered FHE execution, with instruction-local transient outputs.
pub fn fhe_execute<'info>(
    mut ctx: Context<'info, FheExecute<'info>>,
    args: FheExecuteArgs,
) -> Result<()> {
    assert_not_paused(&ctx.accounts.host_config)?;
    require!(
        !args.steps.is_empty() && args.steps.len() <= MAX_FHE_EXECUTION_STEPS,
        ZamaHostError::InvalidFheExecuteOperationCount
    );
    require!(
        usize::from(args.account_count) == ctx.remaining_accounts.len(),
        ZamaHostError::FheExecuteAccountCountMismatch
    );
    let rand_nonce = consume_rand_nonce(&mut ctx, &args)?;
    // The account table owns every remaining-accounts invariant for the execution:
    // duplicate rejection (at construction), the used-account bitmap (marked in
    // preflight, asserted before execution mutates state), persistent-output
    // claims, and output-PDA derivation.
    let mut account_table = ExecutionAccountTable::new(ctx.remaining_accounts)?;
    // Preflight also settles the execution's application identity: the one `(program, scope)`
    // every persistent value it reads or writes belongs to. Metering, the deny list and rand
    // seeds all key on it.
    let app = preflight_execution(&mut account_table, &ctx, &args)?;
    if let Some(app) = app {
        let host_config = &ctx.accounts.host_config;
        let deny_record = account_table.deny_record(host_config.grant_deny_list_enabled, app)?;
        check_scope_not_denied_info(host_config, app, deny_record)?;
    }

    // HCU metering: one pure pass over the execution, enforcing the per-execution total + in-execution depth
    // caps against the canonical host_config limits (u64::MAX = unlimited). The same total then feeds the
    // block-cap charge — reused, never independently recomputed — so both caps trip before
    // execution burns CU or creates any ACL record.
    let host_config = &ctx.accounts.host_config;
    let execution = hcu::meter_execution(
        &args.steps,
        &args.dictionary,
        host_config.max_hcu_per_tx,
        host_config.max_hcu_depth_per_tx,
    )?;

    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash(clock.slot)?;
    let handle_context = ExecutionHandleContext {
        derivation: HandleDerivationContext {
            chain_id: ctx.accounts.host_config.chain_id,
            previous_bank_hash,
            unix_timestamp: clock.unix_timestamp,
        },
        rand: rand_nonce.map(|nonce| RandContext {
            nonce,
            app: app.unwrap_or_default(),
        }),
    };
    let random_seeds = collect_execution_random_seeds(&args, &handle_context);
    block_cap::charge(&ctx, app, execution.total, clock.slot)?;
    // Execution is the single walk: it validates each step as it mutates. A failure mid-execution
    // leaves partial writes behind only until the runtime reverts the transaction, which discards
    // every account write — so no validate-only pre-pass is needed for atomicity. The event CPI
    // stays last so no event describes state that did not commit.
    let created_public_outputs =
        execute_steps(&mut account_table, &ctx, &args, app, &handle_context)?;
    emit_execution_random_seeds(&ctx, random_seeds)?;
    emit_public_outputs_produced(&ctx, created_public_outputs)?;
    Ok(())
}

/// Takes the host's rand nonce for this execution and advances it, when the execution has a rand
/// step. The account is required exactly then: a rand execution without it cannot derive a fresh
/// seed, and a non-rand execution that passes it would serialize on it for nothing.
fn consume_rand_nonce(
    ctx: &mut Context<'_, FheExecute<'_>>,
    args: &FheExecuteArgs,
) -> Result<Option<u64>> {
    let has_rand = args.steps.iter().any(|step| {
        matches!(
            step,
            FheExecuteStep::Rand { .. } | FheExecuteStep::RandBounded { .. }
        )
    });
    match (ctx.accounts.rand_nonce.as_mut(), has_rand) {
        (Some(rand_nonce), true) => {
            let nonce = rand_nonce.nonce;
            rand_nonce.nonce = nonce
                .checked_add(1)
                .ok_or(ZamaHostError::InvalidFheExecuteAccount)?;
            Ok(Some(nonce))
        }
        (None, true) => Err(error!(ZamaHostError::FheExecuteRandNonceMissing)),
        (Some(_), false) => Err(error!(ZamaHostError::InvalidFheExecuteAccount)),
        (None, false) => Ok(None),
    }
}

fn collect_execution_random_seeds(
    args: &FheExecuteArgs,
    handle_context: &ExecutionHandleContext,
) -> Vec<FheExecuteRandomSeed> {
    args.steps
        .iter()
        .enumerate()
        .filter(|(_, step)| {
            matches!(
                step,
                FheExecuteStep::Rand { .. } | FheExecuteStep::RandBounded { .. }
            )
        })
        .map(|(index, _)| FheExecuteRandomSeed {
            step_index: index as u16,
            seed: handle_context.rand_seed(index as u16),
        })
        .collect()
}

#[inline(never)]
fn execute_steps<'a, 'info>(
    table: &mut ExecutionAccountTable<'a, 'info>,
    ctx: &Context<'info, FheExecute<'info>>,
    args: &FheExecuteArgs,
    app: Option<AppScope>,
    handle_context: &ExecutionHandleContext,
) -> Result<Vec<ProducedPublicOutput>> {
    let mut execution = ExecutionState {
        table,
        dictionary: &args.dictionary,
        produced: Vec::with_capacity(args.steps.len()),
        created_public_outputs: Vec::new(),
        app,
        chain_id: handle_context.derivation.chain_id,
        host_config: ctx.accounts.host_config.as_ref(),
    };
    walk_steps(&mut execution, ctx, args, handle_context)?;
    Ok(execution.created_public_outputs)
}

/// The single walk's state: resolves operands through the shared account table
/// (which preflight already validated for coverage and authority signatures),
/// validates and creates or updates persistent outputs, and buffers
/// produced-public lifecycle records. The operand resolvers driving these
/// methods live with the step match in [`walk`].
struct ExecutionState<'t, 'a, 'info> {
    table: &'t mut ExecutionAccountTable<'a, 'info>,
    /// The execution's interned constant dictionary ([`FheExecuteArgs::dictionary`]).
    dictionary: &'t [[u8; 32]],
    produced: Vec<ProducedValue>,
    created_public_outputs: Vec<ProducedPublicOutput>,
    /// The application every persistent value of the execution belongs to; `None` when the
    /// execution touches none.
    app: Option<AppScope>,
    chain_id: u64,
    host_config: &'t HostConfig,
}

impl<'info> ExecutionState<'_, '_, 'info> {
    fn dictionary_bytes(&self, index: u8) -> Result<[u8; 32]> {
        crate::state::dictionary_bytes(self.dictionary, index)
    }

    #[inline(never)]
    fn resolve_persistent_operand(
        &mut self,
        handle: [u8; 32],
        encrypted_value_index: u16,
    ) -> Result<ResolvedOperand> {
        let chain_id = self.chain_id;
        let value = self
            .table
            .canonical_encrypted_value(encrypted_value_index)?;
        assert_current_handle(value, handle, chain_id)?;
        Ok(ResolvedOperand::encrypted(handle))
    }

    #[inline(never)]
    fn resolve_verified_input_operand(
        &mut self,
        attestation: &CoprocessorInputAttestation,
    ) -> Result<ResolvedOperand> {
        // Authoritative in-execution verification of the coprocessor attestation. No account, no
        // PDA — the "allow" exists only for this instruction's execution (the EVM
        // `allowTransient(input, msg.sender)` analog). The caller-is-contract gate is enforced in
        // `resolve_encrypted_operand`; derived outputs are then unconstrained, exactly like EVM.
        verify_input_attestation(self.host_config, attestation)?;
        Ok(ResolvedOperand::encrypted(attestation.input_handle))
    }

    #[inline(never)]
    fn accept_output(
        &mut self,
        ctx: &Context<'info, FheExecute<'info>>,
        op_index: u16,
        result: [u8; 32],
        output: &FheExecuteOutput,
    ) -> Result<()> {
        let created_public_output = accept_execution_output(
            ctx,
            self.table,
            self.dictionary,
            &mut self.produced,
            result,
            output,
            op_index,
        )?;
        if let Some(record) = created_public_output {
            self.created_public_outputs.push(record);
        }
        Ok(())
    }
}

#[inline(never)]
fn accept_execution_output<'info>(
    ctx: &Context<'info, FheExecute<'info>>,
    table: &mut ExecutionAccountTable<'_, 'info>,
    dictionary: &[[u8; 32]],
    produced: &mut Vec<ProducedValue>,
    result: [u8; 32],
    output: &FheExecuteOutput,
    op_index: u16,
) -> Result<Option<ProducedPublicOutput>> {
    require!(
        !produced.iter().any(|value| value.handle == result),
        ZamaHostError::FheExecuteDuplicateHandle
    );

    let created_public_output = match output {
        FheExecuteOutput::Transient => None,
        FheExecuteOutput::StoredValue {
            output_encrypted_value_index,
            output_authority_index,
            output_program_index,
            output_authority_key_index,
            output_scope_index,
            output_label_index,
            output_authority_seeds,
            output_allow_indexes,
            previous_handle_index,
            make_public,
        } => {
            let declared = DeclaredOutput {
                app: AppScope {
                    program: dictionary_key(dictionary, *output_program_index)?,
                    scope: dictionary_bytes(dictionary, *output_scope_index)?,
                },
                authority: dictionary_key(dictionary, *output_authority_key_index)?,
                label: dictionary_bytes(dictionary, *output_label_index)?,
                authority_seeds: output_authority_seeds,
                allows: resolve_dictionary_keys(dictionary, output_allow_indexes)?,
                previous_handle: previous_handle_index
                    .map(|index| dictionary_bytes(dictionary, index))
                    .transpose()?,
                make_public: *make_public,
            };
            let authority = persistent_output_authority(
                table,
                ctx,
                output_authority_index.map(u16::from),
                declared.authority,
            )?;
            require_keys_eq!(
                authority.key(),
                declared.authority,
                ZamaHostError::EncryptedValueAccountAuthorityMismatch
            );
            let encrypted_value = bind_execution_output(
                ctx,
                table,
                dictionary,
                u16::from(*output_encrypted_value_index),
                result,
                &declared,
            )?;
            make_public.then(|| ProducedPublicOutput {
                step_index: op_index,
                encrypted_value,
                output_handle: result,
            })
        }
    };

    produced.push(ProducedValue { handle: result });
    Ok(created_public_output)
}

/// A persistent output's declaration, resolved out of the dictionary.
struct DeclaredOutput<'a> {
    app: AppScope,
    authority: Pubkey,
    label: [u8; 32],
    authority_seeds: &'a [PdaSeed],
    allows: Vec<Pubkey>,
    previous_handle: Option<[u8; 32]>,
    make_public: bool,
}

fn resolve_dictionary_keys(dictionary: &[[u8; 32]], indexes: &[u8]) -> Result<Vec<Pubkey>> {
    indexes
        .iter()
        .map(|index| dictionary_key(dictionary, *index))
        .collect()
}

/// The signer that admits a persistent output: the named remaining account, or the default
/// authority signer.
fn persistent_output_authority<'info>(
    table: &ExecutionAccountTable<'_, 'info>,
    ctx: &Context<'info, FheExecute<'info>>,
    authority_index: Option<u16>,
    output_authority: Pubkey,
) -> Result<AccountInfo<'info>> {
    let authority = match authority_index {
        Some(index) => {
            let authority = table.account(index)?;
            require!(authority.is_signer, ZamaHostError::InvalidFheExecuteAccount);
            require_keys_eq!(
                authority.key(),
                output_authority,
                ZamaHostError::EncryptedValueAccountAuthorityMismatch
            );
            authority.clone()
        }
        None => ctx
            .accounts
            .encrypted_value_account_authority
            .to_account_info(),
    };
    Ok(authority)
}

#[derive(Clone)]
pub(super) struct ProducedValue {
    handle: [u8; 32],
}

#[derive(Clone)]
pub(super) struct ResolvedOperand {
    pub(super) handle: [u8; 32],
    pub(super) scalar: bool,
}

impl ResolvedOperand {
    fn encrypted(handle: [u8; 32]) -> Self {
        Self {
            handle,
            scalar: false,
        }
    }

    fn scalar(handle: [u8; 32]) -> Self {
        Self {
            handle,
            scalar: true,
        }
    }

    fn from_produced(value: &ProducedValue) -> Self {
        Self {
            handle: value.handle,
            scalar: false,
        }
    }
}

#[inline(never)]
fn bind_execution_output<'info>(
    ctx: &Context<'info, FheExecute<'info>>,
    table: &mut ExecutionAccountTable<'_, 'info>,
    dictionary: &[[u8; 32]],
    output_encrypted_value_index: u16,
    result: [u8; 32],
    declared: &DeclaredOutput<'_>,
) -> Result<Pubkey> {
    assert_allow_keys(&declared.allows)?;

    let output_info = table.account(output_encrypted_value_index)?;
    let output_pda = table.expected_output_pda(declared.app, declared.authority, declared.label);
    require_keys_eq!(
        output_info.key(),
        output_pda.key,
        ZamaHostError::EncryptedValuePdaMismatch
    );
    // One write per account per execution: the read-after-write operand rule relies on it.
    table.claim_persistent_output(output_info.key())?;
    // Explicit on the update path; `create_pda_strict` enforces it on create.
    require!(
        output_info.is_writable,
        ZamaHostError::InvalidFheExecuteAccount
    );

    let mut value = if output_info.owner == &crate::ID {
        // Update: the declared previous handle must be the stored one, so an execution built on
        // stale state fails instead of overwriting a newer handle, and indexers replay the write
        // from instruction data alone. The stored program was proven on create and the canonical
        // address check above binds the declared identity to it, so seeds are not re-proven.
        require!(
            declared.authority_seeds.is_empty(),
            ZamaHostError::InvalidFheExecuteAccount
        );
        let mut value = table.take_canonical_encrypted_value(output_encrypted_value_index)?;
        require!(
            declared.previous_handle == Some(value.current_handle),
            ZamaHostError::PreviousStateMismatch
        );
        value.current_handle = result;
        value
    } else {
        // Create: nothing to replace, and the authority must be proven a PDA of the declared
        // program, or another program could claim this `(program, scope)` for its own values.
        require!(
            declared.previous_handle.is_none(),
            ZamaHostError::PreviousStateMismatch
        );
        assert_authority_is_program_pda(
            dictionary,
            declared.authority_seeds,
            declared.app.program,
            declared.authority,
        )?;
        EncryptedValue {
            program: declared.app.program,
            encrypted_value_account_authority: declared.authority,
            scope: declared.app.scope,
            label: declared.label,
            current_handle: result,
            leaf_count: 0,
            peaks: Vec::new(),
            bump: output_pda.bump,
        }
    };
    // Leaf order: one historical-access leaf per allowed key on the NEW handle, in declared
    // order, then the public leaf last. Byte-identical to what the coprocessor recomputes from the
    // instruction and to `make_handle_public`.
    seal_allow_leaves(output_info, &mut value, result, &declared.allows)?;
    if declared.make_public {
        append_public_decrypt_leaf(output_info, &mut value, result)?;
    }
    let space = 8 + EncryptedValue::space(value.peaks.len());
    if output_info.owner == &crate::ID {
        grow_account_if_needed(
            &ctx.accounts.payer.to_account_info(),
            output_info,
            &ctx.accounts.system_program.to_account_info(),
            space,
        )?;
    } else {
        create_pda_strict(
            &ctx.accounts.payer.to_account_info(),
            output_info,
            &ctx.accounts.system_program.to_account_info(),
            space,
            &[
                zama_solana_acl::ENCRYPTED_VALUE_SEED,
                declared.app.program.as_ref(),
                declared.authority.as_ref(),
                &declared.app.scope,
                &declared.label,
                &[output_pda.bump],
            ],
        )?;
    }
    write_account(output_info, &value)?;
    Ok(output_info.key())
}

/// Proves `authority` is a PDA of `program`: the declared seeds (bump last) must derive it. This
/// is what makes `(program, scope)` unforgeable — only `program` can sign for such an authority.
fn assert_authority_is_program_pda(
    dictionary: &[[u8; 32]],
    seeds: &[PdaSeed],
    program: Pubkey,
    authority: Pubkey,
) -> Result<()> {
    let mut resolved: Vec<&[u8]> = Vec::with_capacity(seeds.len());
    for seed in seeds {
        resolved.push(match seed {
            PdaSeed::Interned { index } => dictionary
                .get(*index as usize)
                .ok_or_else(|| error!(ZamaHostError::FheExecuteDictionaryIndexOutOfBounds))?,
            PdaSeed::Literal { bytes } => bytes,
        });
    }
    let derived = Pubkey::create_program_address(&resolved, &program)
        .map_err(|_| error!(ZamaHostError::EncryptedValueAuthorityNotProgramPda))?;
    require_keys_eq!(
        derived,
        authority,
        ZamaHostError::EncryptedValueAuthorityNotProgramPda
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Doc-sync guard (the `resource_bounds_match_liveness_doc` pattern): EVM_PARITY.md's
    /// FHEVMExecutor row quotes `MAX_FHE_EXECUTION_STEPS=32`; a change here must update that row in
    /// the same PR.
    #[test]
    fn batch_ops_bound_matches_evm_parity_doc() {
        assert_eq!(
            MAX_FHE_EXECUTION_STEPS, 32,
            "EVM_PARITY.md FHEVMExecutor row"
        );
    }

    #[test]
    fn authority_pda_proof_accepts_the_programs_pda_and_nothing_else() {
        let program = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let (authority, bump) =
            Pubkey::find_program_address(&[b"token-account", mint.as_ref()], &program);
        let dictionary = [mint.to_bytes()];
        let seeds = [
            PdaSeed::Literal {
                bytes: b"token-account".to_vec(),
            },
            PdaSeed::Interned { index: 0 },
            PdaSeed::Literal { bytes: vec![bump] },
        ];
        assert!(assert_authority_is_program_pda(&dictionary, &seeds, program, authority).is_ok());

        // The same seeds under another program derive another address: the authority is not
        // that program's PDA.
        assert_eq!(
            assert_authority_is_program_pda(&dictionary, &seeds, Pubkey::new_unique(), authority)
                .unwrap_err(),
            error!(ZamaHostError::EncryptedValueAuthorityNotProgramPda)
        );
        // A wallet with no seeds at all is not a PDA of anything.
        assert!(
            assert_authority_is_program_pda(&dictionary, &[], program, Pubkey::new_unique())
                .is_err()
        );
        // An interned seed past the dictionary fails as a dictionary error.
        assert_eq!(
            assert_authority_is_program_pda(
                &dictionary,
                &[PdaSeed::Interned { index: 1 }],
                program,
                authority
            )
            .unwrap_err(),
            error!(ZamaHostError::FheExecuteDictionaryIndexOutOfBounds)
        );
    }
}
