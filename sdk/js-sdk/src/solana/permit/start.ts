// Where a permit's validity window starts.
//
// Three rules meet in one value. A start is set back by a clock margin first: the Connector judges
// the window against the host's `Clock` at `finalized`, which trails wall time by the finalization
// lag (about 13 s) plus whatever the cluster clock drifts, and a start ahead of that clock is
// refused as not yet valid, a refusal the decryption relayer reports as a rejected signature. Then
// it is rounded down to the minute, so that permits signed within the same minute carry the same
// start and a wallet screen shows a round number rather than the second a click landed on. The
// margin shortens the usable window by up to two minutes. And a start is never below the signer's invalidation watermark, because
// a permit that starts before the signer's last revocation is dead the moment it is signed — plain
// rounding alone would kill a permit created immediately after a revocation, in the seconds between
// the watermark and the next minute boundary.
//
// The watermark wins as it stands, unrounded: rounding it down would put the start back below the
// revocation it is there to clear.

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
