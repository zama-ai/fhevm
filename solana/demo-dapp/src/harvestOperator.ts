import { createSolanaRpc, type Address, type TransactionSigner } from "@solana/kit";
import { buildHarvestInstruction, getVaultMetrics } from "./vault/index.js";

import type { DemoConfig } from "./demoConfig";
import { sendTransaction } from "./sendTransaction";
import type { VaultMetrics } from "./batchTypes";
import {
  DEMO_YEAR_GROWTH_DENOMINATOR,
  DEMO_YEAR_GROWTH_NUMERATOR,
} from "./yieldPolicy";

const HARVEST_COMPUTE_UNIT_LIMIT = 200_000;

export const donationForOneYear = (metrics: VaultMetrics): bigint => {
  if (metrics.totalShares === 0n) throw new Error("the vault has no shares to accrue yield to");
  const targetAssets =
    (metrics.totalAssets * DEMO_YEAR_GROWTH_NUMERATOR + DEMO_YEAR_GROWTH_DENOMINATOR - 1n) /
    DEMO_YEAR_GROWTH_DENOMINATOR;
  return targetAssets - metrics.totalAssets;
};

export const readDemoVaultMetrics = async (config: DemoConfig): Promise<VaultMetrics> => {
  const metrics = await getVaultMetrics(createSolanaRpc(config.rpcUrl), config.vault, {
    commitment: "confirmed",
  });
  return { totalAssets: metrics.totalAssets, totalShares: metrics.totalShares };
};

/** Mints `baseUnits` of the vault's underlying to `recipient` (the operator's mock-USDC minter). */
export type UnderlyingMinter = (recipient: Address, baseUnits: bigint) => Promise<string>;

/** Adds one year of illustrative 7% yield to the current vault assets. */
export const harvestDemoVault = async (
  config: DemoConfig,
  keeper: TransactionSigner,
  mintUnderlying: UnderlyingMinter,
): Promise<{ readonly before: VaultMetrics; readonly after: VaultMetrics }> => {
  const rpc = createSolanaRpc(config.rpcUrl);
  const before = await readDemoVaultMetrics(config);
  const donation = donationForOneYear(before);

  await mintUnderlying(keeper.address, donation);
  await sendTransaction(
    config,
    keeper,
    [await buildHarvestInstruction(rpc, { donor: keeper, vault: config.vault, amount: donation })],
    HARVEST_COMPUTE_UNIT_LIMIT,
  );
  return { before, after: await readDemoVaultMetrics(config) };
};
