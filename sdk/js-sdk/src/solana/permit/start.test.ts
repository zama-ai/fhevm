// Where the validity window starts.
//
// Three rules in one value, and the pins below separate them: the clock margin, rounding down to the
// minute, and the floor the signer's invalidation watermark puts under it. The interesting cases are the ones where
// the two disagree — a permit created seconds after a revocation is exactly the case plain rounding
// gets wrong, and the case a test that only checked "is a multiple of 60" would call correct.

import { describe, expect, it } from 'vitest';
import {
  PERMIT_START_CLOCK_MARGIN_SECONDS,
  PERMIT_START_GRANULARITY_SECONDS,
  normalizeSolanaPermitStart,
} from './index.js';

const startOf = (now: bigint, invalidationWatermark: bigint): bigint =>
  normalizeSolanaPermitStart({ now, invalidationWatermark });

describe('the normalized start, when no revocation constrains it', () => {
  it('sets the current time back by the margin, then rounds it down to the minute', () => {
    expect(startOf(1_767_229_439n, 0n)).toBe(1_767_229_320n);
    expect(startOf(1_767_229_441n, 0n)).toBe(1_767_229_380n);
  });

  // The case the margin is for: at the top of a minute, rounding alone leaves no room at all for a
  // host clock that trails wall time.
  it('starts a full margin before a time on a minute boundary', () => {
    expect(startOf(1_767_229_380n, 0n)).toBe(1_767_229_380n - PERMIT_START_CLOCK_MARGIN_SECONDS);
  });

  it('reads a missing watermark as no revocation', () => {
    // A user who has never revoked has no on-chain account, which reads as zero.
    expect(startOf(1_767_229_439n, 0n)).toBe(1_767_229_320n);
  });

  it('starts at the epoch within the first margin of it', () => {
    expect(startOf(119n, 0n)).toBe(0n);
    expect(startOf(59n, 0n)).toBe(0n);
    expect(startOf(0n, 0n)).toBe(0n);
  });
});

describe('the normalized start, when a revocation constrains it', () => {
  // The case plain rounding gets wrong: the revocation landed mid-minute, and rounding down would
  // put the start before it, so the permit would start already invalidated.
  it('does not fall below a watermark set later in the same minute', () => {
    expect(startOf(1_767_229_439n, 1_767_229_400n)).toBe(1_767_229_400n);
  });

  // The margin does not reach below a watermark either: the host wrote it with the clock the
  // Connector reads, so it is never ahead of that clock.
  it('does not fall below a watermark set within the margin', () => {
    expect(startOf(1_767_229_439n, 1_767_229_420n)).toBe(1_767_229_420n);
  });

  // And the watermark is used as it stands. Rounding it down would undo exactly what it is for.
  it('keeps the watermark unrounded, even off a minute boundary', () => {
    const start = startOf(1_767_229_439n, 1_767_229_401n);
    expect(start).toBe(1_767_229_401n);
    expect(start % PERMIT_START_GRANULARITY_SECONDS).not.toBe(0n);
  });

  it('takes the rounded time when the watermark is older', () => {
    expect(startOf(1_767_229_439n, 1_767_000_000n)).toBe(1_767_229_320n);
  });

  it('takes either when the two agree', () => {
    expect(startOf(1_767_229_439n, 1_767_229_320n)).toBe(1_767_229_320n);
  });
});

describe('the normalized start, as a function', () => {
  // Sampled across a whole minute at three watermark offsets: before the minute, inside it, and at
  // the current second. Every case must land in the window both rules leave open.
  const seconds = Array.from({ length: 61 }, (_, offset) => 1_767_229_380n + BigInt(offset));

  it('lands at or above the watermark, and a full margin before now unless the watermark is later', () => {
    for (const now of seconds) {
      for (const watermark of [0n, now - 90n, now - 30n, now]) {
        const start = startOf(now, watermark);
        expect(start, `now ${now}, watermark ${watermark}`).toBeGreaterThanOrEqual(watermark);
        expect(start, `now ${now}, watermark ${watermark}`).toBeLessThanOrEqual(
          watermark > now - PERMIT_START_CLOCK_MARGIN_SECONDS ? watermark : now - PERMIT_START_CLOCK_MARGIN_SECONDS,
        );
      }
    }
  });

  it('never moves backwards as time passes', () => {
    for (const watermark of [0n, 1_767_229_400n]) {
      let previous = 0n;
      for (const now of seconds) {
        const start = startOf(now, watermark);
        expect(start, `now ${now}, watermark ${watermark}`).toBeGreaterThanOrEqual(previous);
        previous = start;
      }
    }
  });

  // Totality, mirroring the renderer's: a watermark above the current time has no usable start, and
  // this function still answers — the rules that refuse it are evaluated elsewhere, and refusing
  // here would turn one rejection into two with different wording.
  it('answers even when no usable start exists', () => {
    expect(startOf(1_767_229_380n, 1_800_000_000n)).toBe(1_800_000_000n);
  });
});
