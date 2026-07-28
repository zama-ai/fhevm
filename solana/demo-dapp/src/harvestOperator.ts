import { createSolanaRpc, type TransactionSigner } from "@solana/kit";
import { buildHarvestInstruction, getVaultMetrics } from "@fhevm/sdk/solana/vault";

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

export type DemoAuthorizationHeaders = Readonly<{
  authorization: string;
  "x-fhevm-demo-boot-id": string;
}>;

const fundDonation = async (
  keeper: TransactionSigner,
  baseUnits: bigint,
  authorizationHeaders: DemoAuthorizationHeaders,
): Promise<void> => {
  const response = await fetch("http://127.0.0.1:8090/mint-usdc", {
    method: "POST",
    headers: { "content-type": "application/json", origin: "http://127.0.0.1:5173", ...authorizationHeaders },
    body: JSON.stringify({ address: keeper.address, amount: Number(baseUnits) / 1_000_000 }),
  });
  if (!response.ok) {
    const body = (await response.json().catch(() => null)) as { readonly error?: string } | null;
    throw new Error(body?.error ?? `demo faucet failed with HTTP ${response.status}`);
  }
};

/** Adds one year of illustrative 7% yield to the current vault assets. */
export const harvestDemoVault = async (
  config: DemoConfig,
  keeper: TransactionSigner,
  authorizationHeaders: DemoAuthorizationHeaders,
): Promise<{ readonly before: VaultMetrics; readonly after: VaultMetrics }> => {
  const rpc = createSolanaRpc(config.rpcUrl);
  const before = await readDemoVaultMetrics(config);
  const donation = donationForOneYear(before);

  await fundDonation(keeper, donation, authorizationHeaders);
  await sendTransaction(
    config,
    keeper,
    [await buildHarvestInstruction(rpc, { donor: keeper, vault: config.vault, amount: donation })],
    HARVEST_COMPUTE_UNIT_LIMIT,
  );
  return { before, after: await readDemoVaultMetrics(config) };
};
