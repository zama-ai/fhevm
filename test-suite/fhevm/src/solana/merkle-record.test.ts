import { describe, expect, test } from "bun:test";

import { recordMismatches } from "./merkle-record";

const STORE = "11111111111111111111111111111112";

describe("Merkle record check", () => {
  test("flags a store whose count or peaks differ, or that the record lacks", () => {
    const chain = new Map([[STORE, { leafCount: 3n, peaks: ["aa", "bb"] }]]);
    expect(recordMismatches(chain, new Map([[STORE, { leafCount: 3n, peaks: ["aa", "bb"] }]]))).toEqual([]);
    expect(recordMismatches(chain, new Map([[STORE, { leafCount: 2n, peaks: ["aa"] }]]))).toHaveLength(1);
    expect(recordMismatches(chain, new Map([[STORE, { leafCount: 3n, peaks: ["aa", "cc"] }]]))).toHaveLength(1);
    expect(recordMismatches(chain, new Map())).toHaveLength(1);
  });

  test("accepts a store without leaves that the record has not seen", () => {
    expect(recordMismatches(new Map([[STORE, { leafCount: 0n, peaks: [] }]]), new Map())).toEqual([]);
  });

  test("flags leaves recorded for a store that has none on chain", () => {
    const chain = new Map([[STORE, { leafCount: 0n, peaks: [] }]]);
    expect(recordMismatches(chain, new Map([[STORE, { leafCount: 1n, peaks: ["aa"] }]]))).toHaveLength(1);
  });

  test("flags a store only the record holds", () => {
    expect(recordMismatches(new Map(), new Map([[STORE, { leafCount: 4n, peaks: ["dd"] }]]))).toHaveLength(1);
  });
});
