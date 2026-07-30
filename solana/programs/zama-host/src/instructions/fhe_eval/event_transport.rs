use super::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
};

pub(super) fn emit_public_outputs_produced<'info>(
    ctx: &Context<'info, FheEval<'info>>,
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

fn public_outputs_produced_event_instruction(outputs: Vec<ProducedPublicOutput>) -> Instruction {
    let event = PublicOutputsProducedEvent {
        version: PUBLIC_OUTPUTS_PRODUCED_EVENT_VERSION,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_batch_has_one_signed_readonly_event_authority_and_fits_cpi_data() {
        let outputs = (0..MAX_FHE_EVAL_OPS)
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
        // 66 bytes per record (u16 step index + encrypted value key + output handle);
        // one batch stays far below the 10,240-byte CPI instruction-data cap (DD-038).
        assert_eq!(instruction.data.len(), 21 + MAX_FHE_EVAL_OPS * 66);
        assert_eq!(instruction.data.len(), 2_133);
    }
}
