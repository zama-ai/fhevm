import { describe, expect, test } from "bun:test";
import crypto from "node:crypto";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import type { SolanaProvisioningContext } from "../../src/solana/provision";
import type { TestEnv } from "./loadEnv";
import { openRunWallets } from "./wallets";

const writeKeypairFile = async (): Promise<string> => {
  const { publicKey, privateKey } = crypto.generateKeyPairSync("ed25519");
  const secretKey = new Uint8Array(64);
  secretKey.set(privateKey.export({ type: "pkcs8", format: "der" }).subarray(-32), 0);
  secretKey.set(publicKey.export({ type: "spki", format: "der" }).subarray(-32), 32);
  const dir = await fs.mkdtemp(path.join(os.tmpdir(), "run-wallets-"));
  const file = path.join(dir, "deployer.json");
  await fs.writeFile(file, JSON.stringify([...secretKey]), "utf8");
  return file;
};

const envWith = (deployerKeypairPath: string, faucet: boolean): TestEnv =>
  ({ roots: { deployerKeypairPath }, capabilities: { faucet, freshMints: true, fastSlots: true } }) as unknown as TestEnv;

const stubContext = () => {
  const funded: [string, number][] = [];
  const swept: [string, string][] = [];
  const context = {
    async fundSol(recipient: string, sol: number) {
      funded.push([recipient, sol]);
      return null;
    },
    async sweepSol(from: { address: string }, to: string) {
      swept.push([from.address, to]);
      return "sig";
    },
  } as unknown as SolanaProvisioningContext;
  return { context, funded, swept };
};

describe("run wallets", () => {
  test("funds each fresh wallet to the requested amount and sweeps them all to the deployer", async () => {
    const deployerPath = await writeKeypairFile();
    const { context, funded, swept } = stubContext();
    const wallets = openRunWallets(envWith(deployerPath, false), context);

    const first = await wallets.fresh(0.2);
    const second = await wallets.fresh(0.05);
    expect(funded).toEqual([
      [first.signer.address, 0.2],
      [second.signer.address, 0.05],
    ]);

    await wallets.sweep();
    expect(swept.map(([from]) => from)).toEqual([first.signer.address, second.signer.address]);
    expect(new Set(swept.map(([, to]) => to)).size).toBe(1);
    // Swept wallets leave the book: a second sweep has nothing to do.
    await wallets.sweep();
    expect(swept).toHaveLength(2);
  });

  test("with a faucet the wallets keep their airdropped SOL", async () => {
    const deployerPath = await writeKeypairFile();
    const { context, swept } = stubContext();
    const wallets = openRunWallets(envWith(deployerPath, true), context);
    await wallets.fresh(1);
    await wallets.sweep();
    expect(swept).toEqual([]);
  });
});
