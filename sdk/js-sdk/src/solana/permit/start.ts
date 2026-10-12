// Where a permit's validity window starts.
//
// Three rules, applied in this order:
//
// 1. Set back by a clock margin. The Connector judges the window against the host's `Clock` at
//    `finalized`, which trails wall time, and refuses a start ahead of it as a rejected signature.
// 2. Rounded down to the minute, so permits signed within one minute carry the same start and a
//    wallet shows a round number.
// 3. Never below the signer's invalidation watermark, which wins unrounded: a permit that starts
//    before the signer's last revocation is dead the moment it is signed.
//
// Rules 1 and 2 shorten the usable window by up to two minutes.

/** The width a start is rounded down to. */
export const PERMIT_START_GRANULARITY_SECONDS = 60n;

/** How far a start is set back from the current time before rounding: the host clock's allowed lag. */
export const PERMIT_START_CLOCK_MARGIN_SECONDS = 60n;

/**
 * The start timestamp to sign, from the current time and the signer's watermark.
 *
 * The watermark floor can put the start back above the margin. That start is still no later than the
 * Connector's clock: the watermark was written by the host's own `Clock`, and is read here at the
 * same `finalized` commitment the Connector reads it at.
 *
 * Total over every pair of non-negative values, including a watermark in the future — whether the
 * result is *usable* is a separate question, decided by rules this function does not evaluate: a
 * start above `now` is refused as a future start, and one below the watermark as invalidated. This
 * function's job is to produce the one start that satisfies both whenever such a start exists.
 *
 * @param options.now - Current unix seconds.
 * @param options.invalidationWatermark - The signer's watermark; `0n` when no revocation was ever
 *   recorded, which is also how a missing on-chain account reads.
 */
export function normalizeSolanaPermitStart(options: {
  readonly now: bigint;
  readonly invalidationWatermark: bigint;
}): bigint {
  const earliest =
    options.now > PERMIT_START_CLOCK_MARGIN_SECONDS ? options.now - PERMIT_START_CLOCK_MARGIN_SECONDS : 0n;
  const rounded = earliest - (earliest % PERMIT_START_GRANULARITY_SECONDS);
  return rounded >= options.invalidationWatermark ? rounded : options.invalidationWatermark;
}
