import { describe, expect, test } from "bun:test";

import {
  assertActiveIdUnchanged,
  eventLogWord,
  firstDataWord,
  parseUintOutput,
  uint256LeHex,
} from "./commands/kms-generation-abort";
import { uint256ToId } from "./utils/fs";

describe("kms-generation-abort parseUintOutput", () => {
  test("parses a plain decimal", () => {
    expect(parseUintOutput("42")).toBe(42n);
  });

  test("strips cast's `[scientific-notation]` annotation (real getKeyCounter output)", () => {
    const raw = "1809251394333065553493296640760748560207343510400633813116524750123642650625 [1.809e75]";
    expect(parseUintOutput(raw)).toBe(1809251394333065553493296640760748560207343510400633813116524750123642650625n);
  });

  test("accepts 0x-prefixed hex output", () => {
    expect(parseUintOutput("0x0400000000000000000000000000000000000000000000000000000000000001")).toBe(
      BigInt("0x0400000000000000000000000000000000000000000000000000000000000001"),
    );
  });

  test("tolerates whitespace / trailing newlines", () => {
    expect(parseUintOutput("  7  \n")).toBe(7n);
  });

  test("throws on non-numeric output (e.g. a revert / error string)", () => {
    expect(() => parseUintOutput("Error: execution reverted")).toThrow(/could not parse/);
  });
});

describe("kms-generation-abort uint256LeHex", () => {
  test("byte-reverses into the little-endian form the connector stores (alloy as_le_slice)", () => {
    // On-chain prepKeygenId 0x0300…0002 is stored as bytea 0200…0003 in prep_keygen_requests.
    const prepKeygenId = (0x03n << 248n) | 2n;
    expect(uint256LeHex(prepKeygenId)).toBe("0200000000000000000000000000000000000000000000000000000000000003");
  });

  test("reverses whole bytes, not hex digits", () => {
    expect(uint256LeHex(0xabcdn)).toBe(`cdab${"0".repeat(60)}`);
    expect(uint256LeHex(0xabcdn)).toHaveLength(64);
  });
});

describe("kms-generation-abort firstDataWord", () => {
  test("reads the leading uint256 of the ABI-encoded event data", () => {
    const id = (0x03n << 248n) | 2n;
    expect(firstDataWord(`0x${uint256ToId(id)}${uint256ToId(1n)}`)).toBe(id);
  });

  test("throws on truncated data", () => {
    expect(() => firstDataWord("0x1234")).toThrow(/too short/);
  });
});

describe("kms-generation-abort assertActiveIdUnchanged", () => {
  test("passes when the active id stayed at baseline", () => {
    expect(() => assertActiveIdUnchanged("key", 1n, 1n, 2n)).not.toThrow();
  });

  test("reports the product failure when the aborted id activated", () => {
    expect(() => assertActiveIdUnchanged("key", 1n, 2n, 2n)).toThrow(/did not prevent activation/);
  });

  test("reports an earlier/concurrent ceremony when an unrelated id activated", () => {
    expect(() => assertActiveIdUnchanged("CRS", 1n, 3n, 2n)).toThrow(/earlier or concurrent run/);
  });
});

describe("kms-generation-abort eventLogWord", () => {
  const topic = "0xAAAA000000000000000000000000000000000000000000000000000000000000";
  const receipt = (topics: string[][]) => ({
    status: "0x1",
    logs: topics.map((entry) => ({ address: "0x1", topics: entry, data: `0x${uint256ToId(9n)}` })),
  });

  test("finds the event by topic0 case-insensitively and returns its leading word", () => {
    expect(eventLogWord(receipt([[topic.toLowerCase()]]), topic, "AbortKeygen")).toBe(9n);
  });

  test("skips unrelated events", () => {
    const other = "0xBBBB000000000000000000000000000000000000000000000000000000000000";
    expect(eventLogWord(receipt([[other], [topic]]), topic, "AbortKeygen")).toBe(9n);
  });

  test("throws with the seen topics when the event is missing", () => {
    expect(() => eventLogWord(receipt([]), topic, "AbortKeygen")).toThrow(/no AbortKeygen event/);
  });
});
