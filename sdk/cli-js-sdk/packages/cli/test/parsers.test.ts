import { describe, expect, it } from "vitest";

import { daysToSeconds } from "../src/cli/parsers";

describe("daysToSeconds", () => {
  it("converts CLI permit days to the SDK's seconds unit", () => {
    expect(daysToSeconds(1)).toBe(86_400);
    expect(daysToSeconds(7)).toBe(604_800);
  });

  it.each([0, -1, 1.5, Number.MAX_SAFE_INTEGER])(
    "rejects unsupported day count %s",
    (days) => {
      expect(() => daysToSeconds(days)).toThrow(
        "Permit duration in days is outside the supported range",
      );
    },
  );
});
