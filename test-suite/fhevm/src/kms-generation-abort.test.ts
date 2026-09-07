import { describe, expect, test } from "bun:test";

import { assertActiveIdUnchanged } from "./commands/kms-generation-abort";

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
