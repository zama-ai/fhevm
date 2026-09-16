import { describe, expect, test } from "bun:test";

import { isAcceptableEcho } from "./relayer-db";

const CONTEXT = `0x07${"0".repeat(62)}`.slice(0, 66);
const V1 = `0x01${CONTEXT.slice(2)}`;
const V2 = `0x02${CONTEXT.slice(2)}${"08".padEnd(64, "0")}`;

describe("kms-qa relayer-db isAcceptableEcho", () => {
  test("accepts a byte-exact echo of a versioned payload", () => {
    expect(isAcceptableEcho(V1, V1)).toBe(true);
    expect(isAcceptableEcho(V2, V2)).toBe(true);
  });

  test("accepts the legacy 0x00 marker coming back empty", () => {
    // The connector normalises the marker before the KMS core sees it; both mean "no context".
    expect(isAcceptableEcho("0x00", "0x")).toBe(true);
  });

  test("accepts 0x00 echoed unchanged, should the normalisation ever be removed", () => {
    expect(isAcceptableEcho("0x00", "0x00")).toBe(true);
  });

  test("rejects a v0 request answered with a versioned payload — that is regeneration", () => {
    // The whole reason v0 is in the case: a value rebuilt from the active context cannot be empty.
    expect(isAcceptableEcho("0x00", V1)).toBe(false);
    expect(isAcceptableEcho("0x00", V2)).toBe(false);
  });

  test("rejects a v1 request answered with a v2 payload — the regeneration this case hunts", () => {
    expect(isAcceptableEcho(V1, V2)).toBe(false);
  });

  test("does not extend the exception to versioned payloads coming back empty", () => {
    expect(isAcceptableEcho(V1, "0x")).toBe(false);
    expect(isAcceptableEcho(V2, "0x")).toBe(false);
  });

  test("rejects a differing payload of the same length", () => {
    const other = `0x01${"f".repeat(64)}`;
    expect(isAcceptableEcho(V1, other)).toBe(false);
  });
});
