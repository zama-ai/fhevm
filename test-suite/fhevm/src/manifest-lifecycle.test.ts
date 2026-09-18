import { describe, expect, test } from "bun:test";
import { assertRestoredMaterial, publicationReady, flippedDigest, validateFixture, waitForManifestCondition } from "./commands/manifest-lifecycle";

const fixture = { chainId: 12345, rootBlock: 42, rootBlockHash: `0x${"42".repeat(32)}`, root: `0x${"51".repeat(32)}`, child: `0x${"52".repeat(32)}` };

describe("manifest lifecycle oracle", () => {
  test("readiness distinguishes publication from registry and epoch prerequisites", () => {
    const ready = { published: true, registryCount: 3, minThreshold: 2, maxThreshold: 2,
      epoch: "legacy", manifestRows: 1, lastPublicationError: null };
    expect(publicationReady(ready)).toBe(true);
    for (const patch of [{ published: false }, { registryCount: 0 }, { minThreshold: null },
      { maxThreshold: 3 }, { epoch: "new-epoch" }]) {
      expect(publicationReady({ ...ready, ...patch })).toBe(false);
    }
  });
  test("rejects incomplete or unsafe fixture identities before composing SQL", () => {
    expect(validateFixture(fixture)).toEqual(fixture);
    expect(() => validateFixture({ ...fixture, root: "0x'" })).toThrow();
    expect(() => validateFixture({ ...fixture, chainId: NaN })).toThrow();
    expect(() => validateFixture({ ...fixture, rootBlock: 1.5 })).toThrow();
    expect(() => validateFixture({ ...fixture, child: fixture.root })).toThrow();
    expect(() => validateFixture({ ...fixture, root: undefined as unknown as string })).toThrow();
  });

  test("single-root fixtures omit descendants but still validate their identities", () => {
    const { child, ...rootOnly } = fixture;
    expect(validateFixture(rootOnly)).toEqual(rootOnly);
    expect(() => validateFixture({ ...rootOnly, child: "0x'" })).toThrow();
  });

  test("healing requires exact bytes, not just matching digest metadata or a new row version", () => {
    const original = { bytes: "00112233", digest: `0x${"64".repeat(32)}`, version: "100" };
    expect(() => assertRestoredMaterial(original, { ...original, version: "102" })).not.toThrow();
    expect(() => assertRestoredMaterial(original, { ...original, bytes: "00112333", version: "102" }))
      .toThrow("exact original ct64 bytes");
    expect(() => assertRestoredMaterial(original, null)).toThrow("missing");
    expect(() => assertRestoredMaterial(original, { ...original, digest: flippedDigest(original.digest) }))
      .toThrow("expected ct64 digest");
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
