import { createSolanaRpc, type TransactionSigner } from "@solana/kit";
import { buildHarvestInstruction, getVaultMetrics } from "@fhevm/sdk/solana/vault";

import type { DemoConfig } from "./demoSession";
import { sendTransaction } from "./sendTransaction";

const TARGET_PRICE_NUMERATOR = 5n;
const TARGET_PRICE_DENOMINATOR = 4n;
const HARVEST_COMPUTE_UNIT_LIMIT = 200_000;

export type DemoVaultMetrics = {
  readonly totalAssets: bigint;
  readonly totalShares: bigint;
};

export const donationToTargetPrice = (metrics: DemoVaultMetrics): bigint => {
  if (metrics.totalShares === 0n) throw new Error("the vault has no shares to accrue yield to");
  const targetAssets =
    (metrics.totalShares * TARGET_PRICE_NUMERATOR + TARGET_PRICE_DENOMINATOR - 1n) /
    TARGET_PRICE_DENOMINATOR;
  return targetAssets > metrics.totalAssets ? targetAssets - metrics.totalAssets : 0n;
};

export const readDemoVaultMetrics = async (config: DemoConfig): Promise<DemoVaultMetrics> => {
  const metrics = await getVaultMetrics(createSolanaRpc(config.rpcUrl), config.vault, {
    commitment: "confirmed",
  });
  return { totalAssets: metrics.totalAssets, totalShares: metrics.totalShares };
};

const fundDonation = async (keeper: TransactionSigner, baseUnits: bigint): Promise<void> => {
  const response = await fetch("http://127.0.0.1:8090/mint-usdc", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ address: keeper.address, amount: Number(baseUnits) / 1_000_000 }),
  });
  if (!response.ok) {
    const body = (await response.json().catch(() => null)) as { readonly error?: string } | null;
    throw new Error(body?.error ?? `demo faucet failed with HTTP ${response.status}`);
  }
};

/** Raises the live share price to at least 1.25 once; repeat calls are chain-state idempotent. */
export const harvestDemoVault = async (
  config: DemoConfig,
  keeper: TransactionSigner,
): Promise<{ readonly before: DemoVaultMetrics; readonly after: DemoVaultMetrics }> => {
  const rpc = createSolanaRpc(config.rpcUrl);
  const before = await readDemoVaultMetrics(config);
  const donation = donationToTargetPrice(before);
  if (donation === 0n) return { before, after: before };

  await fundDonation(keeper, donation);
  await sendTransaction(
    config,
    keeper,
    [await buildHarvestInstruction(rpc, { donor: keeper, vault: config.vault, amount: donation })],
    HARVEST_COMPUTE_UNIT_LIMIT,
  );
  return { before, after: await readDemoVaultMetrics(config) };
};
