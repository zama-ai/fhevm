import { describe, expect, test } from "bun:test";

import { parseRecordRows, recordMismatches } from "./merkle-record";

const STORE = "11111111111111111111111111111112";
const STORE_HEX = `${"00".repeat(31)}01`;

describe("Merkle record check", () => {
  test("parses rows with and without peaks", () => {
    const rows = parseRecordRows(`${STORE_HEX} 3 ${"aa".repeat(32)},${"bb".repeat(32)}\n${"00".repeat(32)} 0 \n`);
    expect(rows.get(STORE)).toEqual({ leafCount: 3n, peaks: ["aa".repeat(32), "bb".repeat(32)] });
    expect(rows.get("11111111111111111111111111111111")).toEqual({ leafCount: 0n, peaks: [] });
  });

  test("flags a store whose count or peaks differ, or that the record lacks", () => {
    const chain = new Map([[STORE, { leafCount: 3n, peaks: ["aa", "bb"] }]]);
    expect(recordMismatches(chain, new Map([[STORE, { leafCount: 3n, peaks: ["aa", "bb"] }]]))).toEqual([]);
    expect(recordMismatches(chain, new Map([[STORE, { leafCount: 2n, peaks: ["aa"] }]]))).toHaveLength(1);
    expect(recordMismatches(chain, new Map([[STORE, { leafCount: 3n, peaks: ["aa", "cc"] }]]))).toHaveLength(1);
    expect(recordMismatches(chain, new Map())).toHaveLength(1);
  });

  test("ignores stores without leaves and stores only the record holds", () => {
    const chain = new Map([[STORE, { leafCount: 0n, peaks: [] }]]);
    expect(recordMismatches(chain, new Map([["closed", { leafCount: 4n, peaks: ["dd"] }]]))).toEqual([]);
  });
});
