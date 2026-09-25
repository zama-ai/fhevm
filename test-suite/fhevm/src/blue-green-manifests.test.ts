import { describe, expect, test } from "bun:test";
import { rangeGaps } from "./commands/blue-green-manifests";

describe("blue-green manifest range continuity", () => {
  test("contiguous ranges in any order have no gap", () => {
    expect(rangeGaps([
      { publicationBlock: 20, first: 11, last: 20 },
      { publicationBlock: 10, first: 5, last: 10 },
      { publicationBlock: 21, first: 21, last: 21 },
    ])).toEqual([]);
  });

  test("a skipped block is reported", () => {
    expect(rangeGaps([
      { publicationBlock: 10, first: 5, last: 10 },
      { publicationBlock: 20, first: 12, last: 20 },
    ])).toEqual(["manifest at 20 starts at 12, previous ended at 10"]);
  });

  test("an overlapping range is reported", () => {
    expect(rangeGaps([
      { publicationBlock: 10, first: 5, last: 10 },
      { publicationBlock: 12, first: 9, last: 12 },
    ])).toHaveLength(1);
  });
});
