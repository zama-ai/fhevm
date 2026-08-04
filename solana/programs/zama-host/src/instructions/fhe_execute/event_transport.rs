use super::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
};

pub(super) fn emit_public_outputs_produced<'info>(
    ctx: &Context<'info, FheExecute<'info>>,
    outputs: Vec<ProducedPublicOutput>,
) -> Result<()> {
    if outputs.is_empty() {
        return Ok(());
    }
    let instruction = public_outputs_produced_event_instruction(outputs);
    invoke_signed(
        &instruction,
        &[ctx.accounts.event_authority.to_account_info()],
        &[&[b"__event_authority", &[crate::EVENT_AUTHORITY_AND_BUMP.1]]],
    )?;
    Ok(())
}

pub(super) fn emit_execution_random_seeds<'info>(
    ctx: &Context<'info, FheExecute<'info>>,
    seeds: Vec<FheExecuteRandomSeed>,
) -> Result<()> {
    if seeds.is_empty() {
        return Ok(());
    }
    let event = FheExecuteRandomSeedsEvent {
        version: EVENT_VERSION,
        seeds,
    };
    emit_event(ctx, &event)
}

fn public_outputs_produced_event_instruction(outputs: Vec<ProducedPublicOutput>) -> Instruction {
    let event = PublicOutputsProducedEvent {
        version: EVENT_VERSION,
        outputs,
    };
    let data = anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(anchor_lang::Event::data(&event))
        .collect::<Vec<_>>();
    Instruction::new_with_bytes(
        crate::ID,
        &data,
        vec![AccountMeta::new_readonly(
            crate::EVENT_AUTHORITY_AND_BUMP.0,
            true,
        )],
    )
}

/// Hand-rolled `emit_cpi!`. The reference implementation is anchor-attribute-event's `emit_cpi`
/// macro, and this is a faithful expansion of it: same `EVENT_IX_TAG_LE`, same `Event::data`
/// payload, same single readonly-signer event authority, same `invoke_signed` seeds. It is expanded
/// by hand only so the `Instruction` exists as a value a host unit test can assert on — the macro
/// builds it inline, so the CPI-data-size bound below could not otherwise be checked without a
/// runtime.
///
/// Only the *assembly* is ours; the tag and the payload encoding are taken from anchor-lang, so
/// they track upstream automatically. What would not track upstream is a change to the shape
/// itself (an extra account, a different order). That is caught at runtime rather than by the
/// compiler: the self-CPI re-enters this program through Anchor's generated `__event_dispatch`,
/// which checks the tag and the authority signer, and `host_mollusk.rs` exercises both emitting
/// paths (a `make_public: true` output and a `Rand` step) against the real SBF artifact. Keep it
/// that way — if those two cases ever stop being covered, this becomes an unchecked copy of an
/// upstream wire format.
fn emit_event<'info, T: anchor_lang::Event>(
    ctx: &Context<'info, FheExecute<'info>>,
    event: &T,
) -> Result<()> {
    let data = anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(anchor_lang::Event::data(event))
        .collect::<Vec<_>>();
    let instruction = Instruction::new_with_bytes(
        crate::ID,
        &data,
        vec![AccountMeta::new_readonly(
            crate::EVENT_AUTHORITY_AND_BUMP.0,
            true,
        )],
    );
    invoke_signed(
        &instruction,
        &[ctx.accounts.event_authority.to_account_info()],
        &[&[b"__event_authority", &[crate::EVENT_AUTHORITY_AND_BUMP.1]]],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_batch_has_one_signed_readonly_event_authority_and_fits_cpi_data() {
        let outputs = (0..MAX_FHE_EXECUTION_STEPS)
            .map(|index| ProducedPublicOutput {
                step_index: index as u16,
                encrypted_value: Pubkey::new_unique(),
                output_handle: [index as u8; 32],
            })
            .collect();
        let instruction = public_outputs_produced_event_instruction(outputs);

        assert_eq!(instruction.program_id, crate::ID);
        assert_eq!(instruction.accounts.len(), 1);
        assert_eq!(
            instruction.accounts[0].pubkey,
            crate::EVENT_AUTHORITY_AND_BUMP.0
        );
        assert!(instruction.accounts[0].is_signer);
        assert!(!instruction.accounts[0].is_writable);
        // 21 bytes of framing (ix tag + event discriminator + version + vec length) plus
        // 66 bytes per record (u16 step index + encrypted value account pubkey + output handle).
        assert_eq!(instruction.data.len(), 21 + MAX_FHE_EXECUTION_STEPS * 66);
        assert_eq!(instruction.data.len(), 2_133);
        // The cap itself, asserted rather than left in prose: the two lines above are a
        // change-detector (raise MAX_FHE_EXECUTION_STEPS and they fail with the new number), but
        // neither of them says what the number has to be under. DD-038 is the 10,240-byte CPI
        // instruction-data limit, so this is the assertion that actually encodes the headroom.
        assert!(instruction.data.len() <= 10_240);
    }
}
