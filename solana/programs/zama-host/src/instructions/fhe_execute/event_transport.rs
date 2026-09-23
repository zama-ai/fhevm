use super::*;
use crate::event_cpi::emit_event_cpi;

pub(super) fn emit_executed_event<'info>(
    ctx: &Context<'info, FheExecute<'info>>,
    derivation: &HandleDerivationContext,
    transient_store: &TransientStore,
    call_start: usize,
    step_count: usize,
    seeds: Vec<FheExecuteRandomSeed>,
) -> Result<()> {
    let results = (call_start..call_start + step_count)
        .map(|index| {
            transient_store
                .result(index)
                .map(|result| result.handle)
                .ok_or(ZamaHostError::InvalidReturnSelection)
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;
    emit_event_cpi(
        &ctx.accounts.event_authority,
        &FheExecutedEvent {
            version: EVENT_VERSION,
            previous_bank_hash: derivation.previous_bank_hash,
            unix_timestamp: derivation.unix_timestamp,
            results,
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
    fn maximum_executed_event_has_one_signed_readonly_event_authority_and_fits_cpi_data() {
        let event = FheExecutedEvent {
            version: EVENT_VERSION,
            previous_bank_hash: [7; 32],
            unix_timestamp: 1_700_000_000,
            results: vec![[9; 32]; MAX_FHE_EXECUTION_STEPS],
            seeds: (0..MAX_FHE_EXECUTION_STEPS)
                .map(|index| FheExecuteRandomSeed {
                    step_index: index as u16,
                    seed: [index as u8; 16],
                })
                .collect(),
        };
        let instruction = event_cpi_instruction(&event);

        assert_eq!(instruction.program_id, crate::ID);
        assert_eq!(instruction.accounts.len(), 1);
        assert_eq!(
            instruction.accounts[0].pubkey,
            crate::EVENT_AUTHORITY_AND_BUMP.0
        );
        assert!(instruction.accounts[0].is_signer);
        assert!(!instruction.accounts[0].is_writable);
        // The bytes `emit_cpi!` would send.
        assert_eq!(
            instruction.data,
            [
                anchor_lang::event::EVENT_IX_TAG_LE,
                &anchor_lang::Event::data(&event)
            ]
            .concat()
        );
        // 57 bytes of framing (ix tag + event discriminator + version + bank hash + timestamp) and
        // two vec lengths, then 32 bytes per result and 18 per seed (u16 step index + 16-byte seed).
        assert_eq!(
            instruction.data.len(),
            57 + 2 * 4 + MAX_FHE_EXECUTION_STEPS * (32 + 18)
        );
        // Solana's CPI instruction-data limit.
        assert!(instruction.data.len() <= 10_240);
    }
}
