import { describe, expect, test } from "bun:test";

import {
  assertRotationConsistency,
  dataWordAt,
  decodeNewKmsEpoch,
  formatKmsId,
  formatPair,
  parseAddressList,
} from "./protocol-config";
import type { NewKmsEpochEvent } from "./protocol-config";
import type { Receipt } from "../kms-onchain";

/** A 32-byte word of `value`, as it appears in a log's `data` payload. */
const word = (value: bigint) => value.toString(16).padStart(64, "0");

const TOPIC0 = "0xaaaa";

/** A receipt carrying one NewKmsEpoch log: indexed context/epoch, three non-indexed data words. */
const newKmsEpochReceipt = (
  contextId: bigint,
  epochId: bigint,
  previousContextId: bigint,
  previousEpochId: bigint,
  materialBlockNumber: bigint,
): Receipt =>
  ({
    status: "0x1",
    logs: [
      {
        address: "0xprotocolconfig",
        topics: [TOPIC0, `0x${word(contextId)}`, `0x${word(epochId)}`],
        data: `0x${word(previousContextId)}${word(previousEpochId)}${word(materialBlockNumber)}`,
      },
    ],
  }) as unknown as Receipt;

describe("kms-qa protocol-config dataWordAt", () => {
  const data = `0x${word(11n)}${word(22n)}${word(33n)}`;

  test("reads each non-indexed word by index", () => {
    expect(dataWordAt(data, 0)).toBe(11n);
    expect(dataWordAt(data, 1)).toBe(22n);
    expect(dataWordAt(data, 2)).toBe(33n);
  });

  test("accepts data without the 0x prefix", () => {
    expect(dataWordAt(data.slice(2), 1)).toBe(22n);
  });

  test("throws a diagnosable error when the word is past the end", () => {
    expect(() => dataWordAt(data, 3)).toThrow(/data too short for word 3/);
  });

  test("rejects a negative or fractional index", () => {
    expect(() => dataWordAt(data, -1)).toThrow(/non-negative integer/);
    expect(() => dataWordAt(data, 1.5)).toThrow(/non-negative integer/);
  });

  test("handles the large domain-tagged ids the protocol actually uses", () => {
    const tagged = (1n << 251n) + 5n;
    expect(dataWordAt(`0x${word(tagged)}`, 0)).toBe(tagged);
  });
});

describe("kms-qa protocol-config decodeNewKmsEpoch", () => {
  test("maps indexed args to topics and non-indexed args to data words", () => {
    const event = decodeNewKmsEpoch(newKmsEpochReceipt(7n, 9n, 7n, 8n, 1420n), TOPIC0);
    expect(event).toEqual({
      contextId: 7n,
      epochId: 9n,
      previousContextId: 7n,
      previousEpochId: 8n,
      materialBlockNumber: 1420n,
    });
  });

  test("matches the topic case-insensitively", () => {
    expect(decodeNewKmsEpoch(newKmsEpochReceipt(7n, 9n, 7n, 8n, 1n), "0xAAAA").epochId).toBe(9n);
  });

  test("reports the topics it did see when the event is absent", () => {
    const receipt = { status: "0x1", logs: [{ address: "0x0", topics: ["0xbbbb"], data: "0x" }] } as unknown as Receipt;
    expect(() => decodeNewKmsEpoch(receipt, TOPIC0)).toThrow(/no NewKmsEpoch event.*0xbbbb/s);
  });

  test("reports 'none' when the receipt carried no logs at all", () => {
    const receipt = { status: "0x1", logs: [] } as unknown as Receipt;
    expect(() => decodeNewKmsEpoch(receipt, TOPIC0)).toThrow(/topics seen: none/);
  });
});

describe("kms-qa protocol-config assertRotationConsistency", () => {
  const baseline = { contextId: 7n, epochId: 8n };
  const event = (overrides: Partial<NewKmsEpochEvent> = {}): NewKmsEpochEvent => ({
    contextId: 7n,
    epochId: 9n,
    previousContextId: 7n,
    previousEpochId: 8n,
    materialBlockNumber: 1420n,
    ...overrides,
  });

  test("accepts a well-formed same-context rotation", () => {
    expect(() => assertRotationConsistency(baseline, event())).not.toThrow();
  });

  test("rejects a baseline the contract disagrees with — a concurrent lifecycle op", () => {
    expect(() => assertRotationConsistency(baseline, event({ previousEpochId: 6n }))).toThrow(
      /ran concurrently, so the baseline is ambiguous/s,
    );
  });

  test("rejects a previousContextId that is not the active context", () => {
    expect(() => assertRotationConsistency(baseline, event({ previousContextId: 6n }))).toThrow(
      /must not change the context/,
    );
  });

  test("rejects an epoch opened under a different context", () => {
    expect(() => assertRotationConsistency(baseline, event({ contextId: 8n }))).toThrow(
      /must reuse the active context/,
    );
  });

  test("rejects a non-sequential epoch id, since ids come from ++epochCounter", () => {
    expect(() => assertRotationConsistency(baseline, event({ epochId: 11n }))).toThrow(/expected the rotation to open epoch 9/);
  });
});

describe("kms-qa protocol-config formatPair", () => {
  test("renders both ids for logs and error messages", () => {
    expect(formatPair({ contextId: 7n, epochId: 9n })).toBe("contextId=7 epochId=9");
  });
});

describe("kms-qa protocol-config parseAddressList", () => {
  test("parses the bracketed list cast prints for an address[] return", () => {
    expect(
      parseAddressList("[0x8B67E6d0EfdBa22D88e7b71801B401d2e686F219, 0xB542D5e6505237D24Af1396920657c74D84Ed2ea]"),
    ).toEqual(["0x8b67e6d0efdba22d88e7b71801b401d2e686f219", "0xb542d5e6505237d24af1396920657c74d84ed2ea"]);
  });

  test("lowercases so addresses compare regardless of checksum casing", () => {
    expect(parseAddressList("[0xABCDEFabcdef0123456789ABCDEFabcdef012345]")).toEqual([
      "0xabcdefabcdef0123456789abcdefabcdef012345",
    ]);
  });

  test("returns an empty list for an empty array", () => {
    expect(parseAddressList("[]")).toEqual([]);
    expect(parseAddressList("")).toEqual([]);
  });

  test("ignores surrounding whitespace and newlines", () => {
    expect(parseAddressList("\n  [0x1111111111111111111111111111111111111111]  \n")).toHaveLength(1);
  });
});

describe("kms-qa protocol-config formatKmsId", () => {
  /** The real ids observed on a live stack: tag 0x07 for contexts, 0x08 for epochs. */
  const contextId = 3166189940082864718613269121331309980362851143201109172953918312716374638599n;
  const epochId = 3618502788666131106986593281521497120414687020801267626233049500247285301264n;

  test("renders a context id as ctx#<counter>", () => {
    expect(formatKmsId(contextId)).toBe("ctx#7");
  });

  test("renders an epoch id as epoch#<counter>", () => {
    expect(formatKmsId(epochId)).toBe("epoch#16");
  });

  test("distinguishes ids that differ only in their last digits", () => {
    expect(formatKmsId(contextId)).not.toBe(formatKmsId(contextId - 1n));
    expect(formatKmsId(contextId - 1n)).toBe("ctx#6");
  });

  test("falls back to the full decimal for an unrecognised tag, rather than a misleading short form", () => {
    const untagged = 42n;
    expect(formatKmsId(untagged)).toBe("42");
    const otherTag = (0x09n << 248n) + 3n;
    expect(formatKmsId(otherTag)).toBe(otherTag.toString());
  });

  test("handles a zero counter", () => {
    expect(formatKmsId(0x07n << 248n)).toBe("ctx#0");
  });

  test("is used by formatPair, so error messages stay readable", () => {
    expect(formatPair({ contextId, epochId })).toBe("contextId=ctx#7 epochId=epoch#16");
  });
});
