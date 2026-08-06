use super::*;
use crate::event_cpi::emit_event_cpi;

pub(super) fn emit_public_outputs_produced<'info>(
    ctx: &Context<'info, FheExecute<'info>>,
    outputs: Vec<ProducedPublicOutput>,
) -> Result<()> {
    if outputs.is_empty() {
        return Ok(());
    }
    emit_event_cpi(
        &ctx.accounts.event_authority,
        &PublicOutputsProducedEvent {
            version: EVENT_VERSION,
            outputs,
        },
    )
}

pub(super) fn emit_execution_random_seeds<'info>(
    ctx: &Context<'info, FheExecute<'info>>,
    seeds: Vec<FheExecuteRandomSeed>,
) -> Result<()> {
    if seeds.is_empty() {
        return Ok(());
    }
    emit_event_cpi(
        &ctx.accounts.event_authority,
        &FheExecuteRandomSeedsEvent {
            version: EVENT_VERSION,
            seeds,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_cpi::event_cpi_instruction;
    use anchor_lang::solana_program::instruction::Instruction;

    /// The instruction a maximum-size `PublicOutputsProducedEvent` emission would carry. Built here
    /// so the CPI-data bound below can be asserted without a runtime; the emission itself goes
    /// through `crate::event_cpi`, same as every other event.
    fn public_outputs_produced_event_instruction(
        outputs: Vec<ProducedPublicOutput>,
    ) -> Instruction {
        event_cpi_instruction(&PublicOutputsProducedEvent {
            version: EVENT_VERSION,
            outputs,
        })
    }

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
