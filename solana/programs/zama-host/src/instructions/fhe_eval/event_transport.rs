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
    RandBounded(FheRandBoundedEvent),
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
    // `emit_cpi!` only (no `emit!` log fallback — no consumer reads logs). A frame
    // with more events than a CPI transport can hold on the 32KiB heap carries no
    // event; born-public outputs are kept out of such frames by
    // `assert_born_public_frame_transportable`, and every other durable handle
    // reconstructs from instruction data.
    if !should_emit_eval_events_as_cpi(events.len()) {
        return Ok(());
    }
    for event in events {
        match event {
            EvalEvent::Binary(event) => emit_cpi!(event),
            EvalEvent::Ternary(event) => emit_cpi!(event),
            EvalEvent::Trivial(event) => emit_cpi!(event),
            EvalEvent::Rand(event) => emit_cpi!(event),
            EvalEvent::RandBounded(event) => emit_cpi!(event),
        }
    }
    Ok(())
}
