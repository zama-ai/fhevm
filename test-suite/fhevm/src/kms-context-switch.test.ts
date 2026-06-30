import { describe, expect, test } from "bun:test";

import { parseContextAndEpoch } from "./commands/kms-context-switch";

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
