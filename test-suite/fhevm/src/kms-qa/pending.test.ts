import { describe, expect, test } from "bun:test";

import { pickStallParty, quorumShortfallAfterStall } from "./pending";

describe("kms-qa pending pickStallParty", () => {
  test("takes the last member, leaving the low party ids alone", () => {
    expect(pickStallParty([1, 2, 3, 4])).toBe(4);
  });

  test("picks from the live committee, not from a contiguous range", () => {
    // A node-swapped stack serves {1,2,3,5}: party 4 was dropped and the spare promoted. Stalling
    // party 4 would withhold nothing, and the rotation would activate mid-probe.
    expect(pickStallParty([1, 2, 3, 5])).toBe(5);
  });

  test("tolerates a single-member committee", () => {
    expect(pickStallParty([2])).toBe(2);
  });

  test("refuses an empty committee rather than stalling an arbitrary party", () => {
    expect(() => pickStallParty([])).toThrow(/resolved to no parties/);
  });
});

describe("kms-qa pending quorumShortfallAfterStall", () => {
  test("accepts the five-party-swap topology: 4 committee members, threshold 3", () => {
    expect(quorumShortfallAfterStall(4, 3n, 4)).toBeUndefined();
  });

  test("accepts a committee with room to spare", () => {
    expect(quorumShortfallAfterStall(7, 3n, 7)).toBeUndefined();
  });

  test("rejects a committee that only just reaches the threshold", () => {
    // 3 members, threshold 3: removing one leaves 2, and the decryption the scenario requires to
    // succeed could never complete.
    expect(quorumShortfallAfterStall(3, 3n, 3)).toMatch(/leaves 2 of 3 committee member\(s\).*needs 3/);
  });

  test("names the committee size that would work", () => {
    expect(quorumShortfallAfterStall(3, 3n, 3)).toMatch(/at least 4 members/);
  });

  test("rejects a single-member committee, where stalling withholds everything", () => {
    expect(quorumShortfallAfterStall(1, 1n, 1)).toMatch(/leaves 0 of 1 committee member\(s\)/);
  });
});
