import { describe, expect, test } from "bun:test";

import { eventLogWord, eventTopicWord, firstDataWord, parseUintOutput, uint256LeHex } from "./kms-onchain";
import { uint256ToId } from "./utils/fs";

describe("kms-onchain parseUintOutput", () => {
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

describe("kms-onchain uint256LeHex", () => {
  test("byte-reverses into the little-endian form the connector stores (alloy as_le_slice)", () => {
    // On-chain prepKeygenId 0x0300…0002 is stored as bytea 0200…0003 in prep_keygen_requests.
    const prepKeygenId = (0x03n << 248n) | 2n;
    expect(uint256LeHex(prepKeygenId)).toBe("0200000000000000000000000000000000000000000000000000000000000003");
  });

  test("byte-reverses a domain-tagged context id (0x07…) the way the connector stores it", () => {
    // Context ids are tagged 0x07 << 248; epoch ids 0x08 << 248 (utils/src/types/mod.rs).
    const contextId = (0x07n << 248n) | 1n;
    expect(uint256LeHex(contextId)).toBe("0100000000000000000000000000000000000000000000000000000000000007");
  });

  test("reverses whole bytes, not hex digits", () => {
    expect(uint256LeHex(0xabcdn)).toBe(`cdab${"0".repeat(60)}`);
    expect(uint256LeHex(0xabcdn)).toHaveLength(64);
  });
});

describe("kms-onchain firstDataWord", () => {
  test("reads the leading uint256 of the ABI-encoded event data", () => {
    const id = (0x03n << 248n) | 2n;
    expect(firstDataWord(`0x${uint256ToId(id)}${uint256ToId(1n)}`)).toBe(id);
  });

  test("throws on truncated data", () => {
    expect(() => firstDataWord("0x1234")).toThrow(/too short/);
  });
});

describe("kms-onchain eventLogWord", () => {
  const topic = "0xAAAA000000000000000000000000000000000000000000000000000000000000";
  const receipt = (topics: string[][]) => ({
    status: "0x1",
    logs: topics.map((entry) => ({ address: "0x1", topics: entry, data: `0x${uint256ToId(9n)}` })),
  });

  test("finds the event by topic0 case-insensitively and returns its leading data word", () => {
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

describe("kms-onchain eventTopicWord", () => {
  const topic0 = "0xCCCC000000000000000000000000000000000000000000000000000000000000";
  // ProtocolConfig's KmsContextDestroyed(uint256 indexed) carries the id in topics[1], not data.
  const receipt = (id: bigint) => ({
    status: "0x1",
    logs: [{ address: "0x1", topics: [topic0, `0x${uint256ToId(id)}`], data: "0x" }],
  });

  test("reads the indexed id from topics[1]", () => {
    const contextId = (0x07n << 248n) | 3n;
    expect(eventTopicWord(receipt(contextId), topic0, 1, "KmsContextDestroyed")).toBe(contextId);
  });

  test("throws when the event is missing", () => {
    expect(() => eventTopicWord({ status: "0x1", logs: [] }, topic0, 1, "KmsContextDestroyed")).toThrow(
      /no KmsContextDestroyed event/,
    );
  });

  test("throws when the indexed topic is absent at the given position", () => {
    const noTopic = { status: "0x1", logs: [{ address: "0x1", topics: [topic0], data: "0x" }] };
    expect(() => eventTopicWord(noTopic, topic0, 1, "KmsContextDestroyed")).toThrow(/no indexed topic/);
  });
});
