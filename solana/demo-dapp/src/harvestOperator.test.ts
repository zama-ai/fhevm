import { describe, expect, test } from "vitest";

import { donationForOneYear } from "./harvestOperator";

describe("donationForOneYear", () => {
  test("adds 7% to the current vault assets", () => {
    expect(donationForOneYear({ totalAssets: 100_000_000n, totalShares: 100_000_000n })).toBe(7_000_000n);
  });

  test("compounds from the current assets on every call", () => {
    expect(donationForOneYear({ totalAssets: 107_000_000n, totalShares: 100_000_000n })).toBe(7_490_000n);
    expect(donationForOneYear({ totalAssets: 130_000_000n, totalShares: 100_000_000n })).toBe(9_100_000n);
  });

  test("rounds up so a non-empty vault always advances", () => {
    expect(donationForOneYear({ totalAssets: 1n, totalShares: 1n })).toBe(1n);
  });
});
