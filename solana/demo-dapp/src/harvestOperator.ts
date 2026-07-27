import { createSolanaRpc, type TransactionSigner } from "@solana/kit";
import { buildHarvestInstruction, getVaultMetrics } from "@fhevm/sdk/solana/vault";

import type { DemoConfig } from "./demoConfig";
import { sendTransaction } from "./sendTransaction";
import type { VaultMetrics } from "./batchTypes";

const TARGET_PRICE_NUMERATOR = 5n;
const TARGET_PRICE_DENOMINATOR = 4n;
const HARVEST_COMPUTE_UNIT_LIMIT = 200_000;

export const donationToTargetPrice = (metrics: VaultMetrics): bigint => {
  if (metrics.totalShares === 0n) throw new Error("the vault has no shares to accrue yield to");
  const targetAssets =
    (metrics.totalShares * TARGET_PRICE_NUMERATOR + TARGET_PRICE_DENOMINATOR - 1n) /
    TARGET_PRICE_DENOMINATOR;
  return targetAssets > metrics.totalAssets ? targetAssets - metrics.totalAssets : 0n;
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

/** Raises the live share price to at least 1.25 once; repeat calls are chain-state idempotent. */
export const harvestDemoVault = async (
  config: DemoConfig,
  keeper: TransactionSigner,
  authorizationHeaders: DemoAuthorizationHeaders,
): Promise<{ readonly before: VaultMetrics; readonly after: VaultMetrics }> => {
  const rpc = createSolanaRpc(config.rpcUrl);
  const before = await readDemoVaultMetrics(config);
  const donation = donationToTargetPrice(before);
  if (donation === 0n) return { before, after: before };

  await fundDonation(keeper, donation, authorizationHeaders);
  await sendTransaction(
    config,
    keeper,
    [await buildHarvestInstruction(rpc, { donor: keeper, vault: config.vault, amount: donation })],
    HARVEST_COMPUTE_UNIT_LIMIT,
  );
  return { before, after: await readDemoVaultMetrics(config) };
};
