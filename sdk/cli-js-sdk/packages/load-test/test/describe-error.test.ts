import { describe, expect, it } from "vitest";

import { describeError } from "../src/cli/program";

describe("describeError", () => {
  it("prints a plain error message", () => {
    expect(describeError(new Error("boom"))).toEqual(["boom"]);
  });

  it("prints non-Error values", () => {
    expect(describeError("boom")).toEqual(["boom"]);
  });

  it("flattens AggregateError members", () => {
    const error = new AggregateError(
      [new Error("first failure"), new Error("second failure")],
      "Handle generation and pool commit both failed",
    );
    expect(describeError(error)).toEqual([
      "Handle generation and pool commit both failed",
      "  first failure",
      "  second failure",
    ]);
  });

  it("follows cause chains, including within aggregates", () => {
    const inner = new Error("heartbeat failed", { cause: new Error("EPERM") });
    const error = new AggregateError([inner], "commit failed");
    expect(describeError(error)).toEqual([
      "commit failed",
      "  heartbeat failed",
      "    caused by:",
      "      EPERM",
    ]);
  });
});
