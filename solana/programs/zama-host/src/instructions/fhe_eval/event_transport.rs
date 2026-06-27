#[cfg(feature = "emit-events")]
use super::event_budget::should_emit_eval_events_as_cpi;
use super::*;

// With `emit-events` off the funnel is a no-op, so these payloads are built (in
// the walk) but never read — expected in that config.
#[cfg_attr(not(feature = "emit-events"), allow(dead_code))]
pub(super) enum EvalEvent {
    Binary(FheBinaryOpEvent),
    Ternary(FheTernaryOpEvent),
    Trivial(TrivialEncryptEvent),
    Rand(FheRandEvent),
    Unary(FheUnaryOpEvent),
    RandBounded(FheRandBoundedEvent),
    Sum(FheSumEvent),
    IsIn(FheIsInEvent),
    MulDiv(FheMulDivEvent),
    AclRecordBound(AclRecordBoundEvent),
    AclAllowed(AclAllowedEvent),
    AclSubjectAllowed(AclSubjectAllowedEvent),
}

/// With `emit-events` disabled, off-chain reconstruction (Yellowstone gRPC) is the
/// sole event source for `fhe_eval`, so this is a no-op.
#[cfg(not(feature = "emit-events"))]
pub(super) fn emit_eval_events<'info>(
    _ctx: &Context<'info, FheEval<'info>>,
    _events: Vec<EvalEvent>,
) -> Result<()> {
    Ok(())
}

#[cfg(feature = "emit-events")]
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
            EvalEvent::Unary(event) => emit_eval_event!(event),
            EvalEvent::RandBounded(event) => emit_eval_event!(event),
            EvalEvent::Sum(event) => emit_eval_event!(event),
            EvalEvent::IsIn(event) => emit_eval_event!(event),
            EvalEvent::MulDiv(event) => emit_eval_event!(event),
            EvalEvent::AclRecordBound(event) => emit!(event),
            EvalEvent::AclAllowed(event) => emit_eval_event!(event),
            EvalEvent::AclSubjectAllowed(event) => emit!(event),
        }
    }
    Ok(())
}
