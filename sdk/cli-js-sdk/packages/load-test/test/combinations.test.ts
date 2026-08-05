import { describe, expect, it } from "vitest";

import {
  binomial,
  minimumCombinationPoolSize,
  unrankCombination,
} from "../src/pool/combinations";

describe("binomial", () => {
  it("matches known values", () => {
    expect(binomial(5, 2)).toBe(10n);
    expect(binomial(10, 3)).toBe(120n);
    expect(binomial(4, 4)).toBe(1n);
    expect(binomial(4, 0)).toBe(1n);
    expect(binomial(3, 5)).toBe(0n);
  });

  it("handles pool-scale inputs without overflow", () => {
    expect(binomial(200, 4)).toBe(64_684_950n);
  });
});

describe("minimumCombinationPoolSize", () => {
  it("finds the smallest pool that satisfies a combination budget", () => {
    expect(minimumCombinationPoolSize(30_240n, 2)).toBe(247);
    expect(minimumCombinationPoolSize(30_240n, 3)).toBe(58);
    expect(minimumCombinationPoolSize(30_240n, 4)).toBe(31);
  });

  it("accounts for already-consumed combinations when callers include cursor position", () => {
    // 39 handles give C(39,2)=741 combinations. If all 741 must be preserved
    // plus 30,240 new requests, 250 handles are needed: C(249,2)=30,876,
    // C(250,2)=31,125.
    expect(minimumCombinationPoolSize(741n + 30_240n, 2)).toBe(250);
  });

  it("rejects invalid combination sizes", () => {
    expect(() => minimumCombinationPoolSize(1n, 0)).toThrow(/positive/);
  });
});

describe("unrankCombination", () => {
  it("enumerates every combination exactly once", () => {
    const n = 7;
    const k = 3;
    const total = binomial(n, k);
    const seen = new Set<string>();
    for (let rank = 0n; rank < total; rank += 1n) {
      const combination = unrankCombination(rank, n, k);
      expect(combination).toHaveLength(k);
      // Strictly ascending, in range.
      for (let i = 0; i < k; i += 1) {
        expect(combination[i]).toBeGreaterThanOrEqual(0);
        expect(combination[i]).toBeLessThan(n);
        if (i > 0) expect(combination[i]).toBeGreaterThan(combination[i - 1] ?? -1);
      }
      seen.add(combination.join(","));
    }
    expect(seen.size).toBe(Number(total));
  });

  it("is deterministic for a given rank", () => {
    expect(unrankCombination(0n, 5, 2)).toEqual(unrankCombination(0n, 5, 2));
    expect(unrankCombination(0n, 5, 2)).toEqual([0, 1]);
  });

  it("rejects out-of-range ranks", () => {
    expect(() => unrankCombination(10n, 5, 2)).toThrow(/out of range/);
    expect(() => unrankCombination(-1n, 5, 2)).toThrow(/out of range/);
  });
});
