import { describe, expect, test } from "bun:test";

import { responseVersion } from "./commands/user-decryption-responses";

describe("KMS user-decryption response versions", () => {
  test("recognizes the empty legacy response", () => {
    expect(responseVersion("0x")).toBe("v0");
  });

  test("requires the complete v1 context payload", () => {
    expect(responseVersion(`0x01${"ab".repeat(32)}`)).toBe("v1");
    expect(responseVersion("0x01")).toBeUndefined();
    expect(responseVersion(`0x01${"ab".repeat(31)}`)).toBeUndefined();
    expect(responseVersion(`0x01${"ab".repeat(33)}`)).toBeUndefined();
  });

  test("rejects unknown response versions", () => {
    expect(responseVersion(`0x02${"ab".repeat(32)}`)).toBeUndefined();
  });
});

