import { describe, expect, test } from "bun:test";

import { contextIdToConnectorHex, responseVersion } from "./commands/user-decryption-responses";

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

describe("KMS connector context IDs", () => {
  test("encodes the on-wire big-endian context as connector uint256 bytes", () => {
    expect(contextIdToConnectorHex(`0x${"01".repeat(29)}020304`)).toBe(`040302${"01".repeat(29)}`);
  });

  test("rejects values that are not 32-byte hex context IDs", () => {
    expect(() => contextIdToConnectorHex("0x0102")).toThrow("32-byte hex");
  });
});
