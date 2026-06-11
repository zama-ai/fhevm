use super::event_budget::should_emit_eval_events_as_cpi;
use super::*;

pub(super) enum EvalEvent {
    Binary(FheBinaryOpEvent),
    Ternary(FheTernaryOpEvent),
    Trivial(TrivialEncryptEvent),
    Rand(FheRandEvent),
    AclRecordBound(AclRecordBoundEvent),
    AclAllowed(AclAllowedEvent),
    AclSubjectAllowed(AclSubjectAllowedEvent),
}

pub(super) fn emit_eval_events<'info>(
    ctx: &Context<'info, FheEval<'info>>,
    events: Vec<EvalEvent>,
) -> Result<()> {
    let emit_cpi_events = should_emit_eval_events_as_cpi(events.len());
    macro_rules! emit_eval_event {
        ($event:expr) => {
            if emit_cpi_events {
                emit_cpi!($event)
            } else {
                emit!($event)
            }
        };
    }
    for event in events {
        match event {
            EvalEvent::Binary(event) => emit_eval_event!(event),
            EvalEvent::Ternary(event) => emit_eval_event!(event),
            EvalEvent::Trivial(event) => emit_eval_event!(event),
            EvalEvent::Rand(event) => emit_eval_event!(event),
            EvalEvent::AclRecordBound(event) => emit!(event),
            EvalEvent::AclAllowed(event) => emit_eval_event!(event),
            EvalEvent::AclSubjectAllowed(event) => emit!(event),
        }
    }
    Ok(())
}
