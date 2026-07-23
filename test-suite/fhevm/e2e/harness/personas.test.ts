import { afterEach, describe, expect, test } from "bun:test";
import crypto from "node:crypto";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { loadPersonas } from "./personas";
import type { TestEnv } from "./loadEnv";

// A real ed25519 keypair on disk in Solana's 64-byte `[seed(32) | pubkey(32)]` layout, so the SDK's
// `createKeyPairSignerFromBytes` validates it (random bytes would fail the seed→pubkey check).
const writeKeypairFile = async (dir: string, name: string): Promise<string> => {
  const { publicKey, privateKey } = crypto.generateKeyPairSync("ed25519");
  const seed = privateKey.export({ type: "pkcs8", format: "der" }).subarray(-32);
  const pub = publicKey.export({ type: "spki", format: "der" }).subarray(-32);
  const secretKey = new Uint8Array(64);
  secretKey.set(seed, 0);
  secretKey.set(pub, 32);
  const file = path.join(dir, `${name}.json`);
  await fs.writeFile(file, JSON.stringify([...secretKey]), "utf8");
  return file;
};

const tmpDirs: string[] = [];
const makeDir = async (): Promise<string> => {
  const dir = await fs.mkdtemp(path.join(os.tmpdir(), "personas-"));
  tmpDirs.push(dir);
  return dir;
};
afterEach(async () => {
  for (const d of tmpDirs.splice(0)) await fs.rm(d, { recursive: true, force: true });
});

const envWith = (deployerKeypairPath: string, faucet = true): TestEnv =>
  ({
    source: "demo-config",
    roots: { deployerKeypairPath },
    capabilities: { faucet, freshMints: false, fastSlots: true },
    rpcUrl: "http://127.0.0.1:8899",
  }) as unknown as TestEnv;

describe("loadPersonas roles", () => {
  test("loads extra named actors (the demo keeper) from disk alongside the deployer", async () => {
    const dir = await makeDir();
    const deployerPath = await writeKeypairFile(dir, "deployer");
    const keeperPath = await writeKeypairFile(dir, "keeper");

    const personas = await loadPersonas(envWith(deployerPath), { keeper: keeperPath });
    expect(personas.roles.keeper).toBeDefined();
    expect(personas.roles.keeper!.name).toBe("keeper");
    // The keeper is a distinct actor from the deployer, and its address derives from its own key.
    expect(personas.roles.keeper!.address).not.toBe(personas.deployer.address);
    expect(personas.roles.keeper!.address).toMatch(/^[1-9A-HJ-NP-Za-km-z]{32,44}$/);
  });

  test("with no extra roles the map is empty and only the deployer loads", async () => {
    const dir = await makeDir();
    const deployerPath = await writeKeypairFile(dir, "deployer");
    const personas = await loadPersonas(envWith(deployerPath));
    expect(personas.roles).toEqual({});
  });

  test("rejects a keypair file that is not 64 bytes", async () => {
    const dir = await makeDir();
    const deployerPath = await writeKeypairFile(dir, "deployer");
    const badPath = path.join(dir, "bad.json");
    await fs.writeFile(badPath, JSON.stringify([1, 2, 3]), "utf8");
    await expect(loadPersonas(envWith(deployerPath), { keeper: badPath })).rejects.toThrow(/64-byte/);
  });
});
