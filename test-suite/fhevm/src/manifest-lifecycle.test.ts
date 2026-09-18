import { describe, expect, test } from "bun:test";
import { flippedDigest, validateFixture, waitForManifestCondition } from "./commands/manifest-lifecycle";

const fixture = { chainId: 12345, rootBlock: 42, rootBlockHash: `0x${"42".repeat(32)}`, root: `0x${"51".repeat(32)}`, child: `0x${"52".repeat(32)}` };

describe("manifest lifecycle oracle", () => {
  test("rejects incomplete or unsafe fixture identities before composing SQL", () => {
    expect(validateFixture(fixture)).toEqual(fixture);
    expect(() => validateFixture({ ...fixture, root: "0x'" })).toThrow();
    expect(() => validateFixture({ ...fixture, chainId: NaN })).toThrow();
    expect(() => validateFixture({ ...fixture, rootBlock: 1.5 })).toThrow();
    expect(() => validateFixture({ ...fixture, child: fixture.root })).toThrow();
    expect(() => validateFixture({ ...fixture, root: undefined as unknown as string })).toThrow();
  });

  test("expected signed fault differs by exactly one bit", () => {
    const digest = `0x${"64".repeat(32)}`;
    expect(flippedDigest(digest)).toBe(`0x65${"64".repeat(31)}`);
    expect(flippedDigest(flippedDigest(digest))).toBe(digest);
    expect(() => flippedDigest("0x01")).toThrow();
  });

  test("polling reports the last durable observation on timeout", async () => {
    await expect(waitForManifestCondition("publication", async () => ({ state: "pending" }), () => false, 0))
      .rejects.toThrow('publication timed out; last observation: {"state":"pending"}');
  });

  test("query failure is not swallowed as eventual consistency", async () => {
    await expect(waitForManifestCondition("publication", async () => { throw new Error("SQL failed"); }, () => true))
      .rejects.toThrow("SQL failed");
  });
});
