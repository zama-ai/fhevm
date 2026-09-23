use super::*;
use crate::event_cpi::emit_event_cpi;

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

    /// The instruction a maximum-size `FheExecuteRandomSeedsEvent` emission would carry. Built here
    /// so the CPI-data bound below can be asserted without a runtime; the emission itself goes
    /// through `crate::event_cpi`, same as every other event.
    fn random_seeds_event_instruction(seeds: Vec<FheExecuteRandomSeed>) -> Instruction {
        event_cpi_instruction(&FheExecuteRandomSeedsEvent {
            version: EVENT_VERSION,
            seeds,
        })
    }

    #[test]
    fn maximum_batch_has_one_signed_readonly_event_authority_and_fits_cpi_data() {
        let seeds = (0..MAX_FHE_EXECUTION_STEPS)
            .map(|index| FheExecuteRandomSeed {
                step_index: index as u16,
                seed: [index as u8; 16],
            })
            .collect();
        let instruction = random_seeds_event_instruction(seeds);

        assert_eq!(instruction.program_id, crate::ID);
        assert_eq!(instruction.accounts.len(), 1);
        assert_eq!(
            instruction.accounts[0].pubkey,
            crate::EVENT_AUTHORITY_AND_BUMP.0
        );
        assert!(instruction.accounts[0].is_signer);
        assert!(!instruction.accounts[0].is_writable);
        // 21 bytes of framing (ix tag + event discriminator + version + vec length) plus
        // 18 bytes per record (u16 step index + 16-byte seed).
        assert_eq!(instruction.data.len(), 21 + MAX_FHE_EXECUTION_STEPS * 18);
        assert_eq!(instruction.data.len(), 597);
        // The cap itself, asserted rather than left in prose: the two lines above are a
        // change-detector (raise MAX_FHE_EXECUTION_STEPS and they fail with the new number), but
        // neither of them says what the number has to be under. DD-038 recorded the 10,240-byte CPI
        // instruction-data limit, so this is the assertion that actually encodes the headroom.
        assert!(instruction.data.len() <= 10_240);
    }
}
