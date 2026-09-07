import { describe, expect, test } from "bun:test";
import { mkdtemp, readFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";

import { writeSolanaAddressArtifact } from "./artifact";
import { parseKeypairBytes, resolveKeypairPath, writeKeypairJson } from "./keypair";

describe("writeSolanaAddressArtifact", () => {
  test("writes chart-scrapable _ADDRESS keys", async () => {
    const dir = await mkdtemp(path.join(tmpdir(), "solana-addresses-"));
    const file = await writeSolanaAddressArtifact(dir, {
      zamaHostId: "Host111111111111111111111111111111111111111",
      confidentialTokenId: "Token11111111111111111111111111111111111111",
      bootstrapSlot: "42",
    });
    expect(path.basename(file)).toBe(".env.solana");
    const body = await readFile(file, "utf8");
    expect(body).toBe(
      [
        "ZAMA_HOST_ADDRESS=Host111111111111111111111111111111111111111",
        "CONFIDENTIAL_TOKEN_ADDRESS=Token11111111111111111111111111111111111111",
        "BOOTSTRAP_SLOT_ADDRESS=42",
        "",
      ].join("\n"),
    );
  });
});

describe("parseKeypairBytes", () => {
  const valid = JSON.stringify(Array.from({ length: 64 }, (_, i) => i));

  test("accepts a 64-byte array", () => {
    expect(parseKeypairBytes(valid)).toHaveLength(64);
  });

  test("rejects short, non-integer, and non-JSON inputs", () => {
    expect(() => parseKeypairBytes("[1,2]")).toThrow("64 bytes");
    expect(() => parseKeypairBytes(JSON.stringify(Array.from({ length: 64 }, () => 1.5)))).toThrow("64 bytes");
    expect(() => parseKeypairBytes("not-json")).toThrow("invalid Solana keypair JSON");
  });
});

describe("resolveKeypairPath", () => {
  test("prefers inline JSON over a path env", async () => {
    const dir = await mkdtemp(path.join(tmpdir(), "solana-keypairs-"));
    const writePath = path.join(dir, "deployer.json");
    const json = JSON.stringify(Array.from({ length: 64 }, (_, i) => i));
    const resolved = await resolveKeypairPath({
      pathEnv: "/unused.json",
      jsonEnv: json,
      fallbackPath: "/fallback.json",
      writePath,
    });
    expect(resolved).toBe(writePath);
    expect(await readFile(writePath, "utf8")).toBe(json);
  });

  test("falls back to path env, then fallbackPath", async () => {
    expect(
      await resolveKeypairPath({
        pathEnv: "/from-path.json",
        jsonEnv: undefined,
        fallbackPath: "/fallback.json",
        writePath: "/tmp/unused.json",
      }),
    ).toBe("/from-path.json");
    expect(
      await resolveKeypairPath({
        pathEnv: undefined,
        jsonEnv: undefined,
        fallbackPath: "/fallback.json",
        writePath: "/tmp/unused.json",
      }),
    ).toBe("/fallback.json");
  });
});

describe("writeKeypairJson", () => {
  test("refuses invalid JSON before writing", async () => {
    const dir = await mkdtemp(path.join(tmpdir(), "solana-keypairs-"));
    await expect(writeKeypairJson(path.join(dir, "bad.json"), "[1]")).rejects.toThrow("64 bytes");
  });
});
