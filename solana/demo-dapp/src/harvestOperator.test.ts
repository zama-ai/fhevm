import { describe, expect, test } from "vitest";

import { donationToTargetPrice } from "./harvestOperator";

describe("donationToTargetPrice", () => {
  test("donates exactly enough to move a 1.00 share price to 1.25", () => {
    expect(donationToTargetPrice({ totalAssets: 100_000_000n, totalShares: 100_000_000n })).toBe(25_000_000n);
  });

  test("is idempotent once the target has been reached", () => {
    expect(donationToTargetPrice({ totalAssets: 125_000_000n, totalShares: 100_000_000n })).toBe(0n);
    expect(donationToTargetPrice({ totalAssets: 130_000_000n, totalShares: 100_000_000n })).toBe(0n);
  });

  test("rounds the target up so it never undershoots 1.25", () => {
    expect(donationToTargetPrice({ totalAssets: 1n, totalShares: 1n })).toBe(1n);
  });
});
