//! The `fhe_execute` CPIs an instruction issued, decoded from its inner instructions and checked
//! against the runtime sysvars.
//!
//! Values are not tracked here: the cleartext host build records them in the accounts
//! ([`crate::cleartext`]).

use mollusk_svm::result::InstructionResult;

use crate::{decode_fhe_execute_args, Ctx};

/// What one instruction's host calls covered.
pub struct RecordedExecutions {
    /// Distinct `fhe_execute` CPIs decoded from the inner instructions.
    pub executions: usize,
    /// Effects that wrote a store slot.
    pub persistent_outputs: usize,
}

/// Counts every `fhe_execute` the instruction issued, and checks each execution's event against
/// the runtime sysvars.
pub fn record(context: &Ctx, result: &InstructionResult) -> RecordedExecutions {
    let message = result
        .message
        .as_ref()
        .expect("Mollusk result must include its compiled message");
    let mut executions = 0;
    let mut persistent_outputs = 0;
    for (index, inner) in result.inner_instructions.iter().enumerate() {
        let program = message
            .account_keys()
            .get(inner.instruction.program_id_index as usize)
            .copied();
        if program != Some(zama_host::id()) {
            continue;
        }
        let Some(args) = decode_fhe_execute_args(&inner.instruction.data) else {
            continue;
        };
        let mut events = result.inner_instructions[index + 1..]
            .iter()
            .take_while(|child| child.stack_height > inner.stack_height)
            .filter(|child| {
                message.account_keys()[child.instruction.program_id_index as usize] == zama_host::ID
            })
            .filter_map(|child| {
                crate::decode_anchor_event::<zama_host::FheExecutedEvent>(&child.instruction.data)
            });
        let event = events
            .next()
            .expect("every execution emits FheExecutedEvent");
        assert!(events.next().is_none(), "one executed event per execution");
        assert_event_matches_runtime(context, &args, &event);
        executions += 1;
        persistent_outputs += args
            .effects
            .iter()
            .filter(|effect| effect.slot.is_some())
            .count();
    }
    RecordedExecutions {
        executions,
        persistent_outputs,
    }
}

fn assert_event_matches_runtime(
    context: &Ctx,
    args: &zama_host::FheExecuteArgs,
    event: &zama_host::FheExecutedEvent,
) {
    assert_eq!(event.version, zama_host::EVENT_VERSION);
    let slot = context.mollusk.sysvars.clock.slot;
    let previous_bank_hash = context
        .mollusk
        .sysvars
        .slot_hashes
        .iter()
        .find(|(candidate, _)| *candidate < slot)
        .map(|(_, hash)| hash.to_bytes())
        .expect("test runtime must contain a previous bank hash");
    assert_eq!(event.previous_bank_hash, previous_bank_hash);
    assert_eq!(
        event.unix_timestamp,
        context.mollusk.sysvars.clock.unix_timestamp
    );
    assert_eq!(event.results.len(), args.steps.len(), "one result per step");
}
