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

    /// The instruction a maximum-size emission carries, built without a runtime; the emission
    /// itself goes through `crate::event_cpi`, like every other event.
    #[test]
    fn maximum_random_seeds_event_has_one_signed_readonly_event_authority_and_fits_cpi_data() {
        let seeds = (0..MAX_FHE_EXECUTION_STEPS)
            .map(|index| FheExecuteRandomSeed {
                step_index: index as u16,
                seed: [index as u8; 16],
            })
            .collect();
        let instruction = event_cpi_instruction(&FheExecuteRandomSeedsEvent {
            version: EVENT_VERSION,
            seeds,
        });

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
        // Solana's CPI instruction-data limit.
        assert!(instruction.data.len() <= 10_240);
    }
}
