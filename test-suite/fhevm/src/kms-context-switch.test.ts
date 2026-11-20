import { describe, expect, test } from "bun:test";

import { parseContextAndEpoch, parseReplayCalldata } from "./commands/kms-context-switch";

describe("kms-context-switch parseContextAndEpoch", () => {
  test("parses the two newline-separated uint values cast prints", () => {
    expect(parseContextAndEpoch("2\n5")).toEqual({ contextId: 2n, epochId: 5n });
  });

  test("strips cast's `[scientific-notation]` annotation (real getCurrentKmsContextAndEpoch output)", () => {
    const raw =
      "3166189940082864718613269121331309980362851143201109172953918312716374638593 [3.166e75]\n" +
      "3618502788666131106986593281521497120414687020801267626233049500247285301249 [3.618e75]";
    expect(parseContextAndEpoch(raw)).toEqual({
      contextId: 3166189940082864718613269121331309980362851143201109172953918312716374638593n,
      epochId: 3618502788666131106986593281521497120414687020801267626233049500247285301249n,
    });
  });

  test("tolerates whitespace / trailing newlines", () => {
    expect(parseContextAndEpoch("  10   7  \n")).toEqual({ contextId: 10n, epochId: 7n });
  });

  test("handles large uint256 values", () => {
    const big = (2n ** 200n).toString();
    expect(parseContextAndEpoch(`${big} ${big}`)).toEqual({ contextId: 2n ** 200n, epochId: 2n ** 200n });
  });

  test("throws on missing second value", () => {
    expect(() => parseContextAndEpoch("3")).toThrow(/getCurrentKmsContextAndEpoch/);
  });

  test("throws on non-numeric output (e.g. a revert / error string)", () => {
    expect(() => parseContextAndEpoch("Error: execution reverted")).toThrow(/getCurrentKmsContextAndEpoch/);
  });

  test("throws on empty output", () => {
    expect(() => parseContextAndEpoch("")).toThrow(/getCurrentKmsContextAndEpoch/);
  });
});

describe("kms-context-switch parseReplayCalldata", () => {
  test("returns the calldata of each call in replay order", () => {
    const output = [
      "Nothing to compile",
      "ProtocolConfigReplica replay",
      "  target: 0x5FbDB2315678afecb367f032d93F642f64180aa3",
      "  scanned canonical blocks: 0..812",
      "",
      "1. submitKmsContextAndEpoch",
      '  decodedArgs: [{"contextId":"7"}]',
      "  calldata: 0xabc123",
      "",
      "2. submitKmsContextDestruction",
      '  decodedArgs: ["6",["0x01"],"0x00"]',
      "  calldata: 0xDEF456",
      "",
      "Skipped destructions. The owner can apply one with task:mirrorKmsContextDestruction or task:mirrorKmsEpochDestruction:",
      "  - Destruction of epoch 9: the task cannot read the state of this epoch on the replica.",
    ].join("\r\n");
    expect(parseReplayCalldata(output)).toEqual(["0xabc123", "0xDEF456"]);
  });

  test("returns no calldata when there is nothing to replay", () => {
    expect(parseReplayCalldata("ProtocolConfigReplica replay\n  scanned canonical blocks: 0..40\n\nNothing to replay.\n")).toEqual([]);
  });
});
