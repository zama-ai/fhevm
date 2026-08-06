import type { Address, FetchAccountConfig, Instruction, Rpc, SolanaRpcApi, TransactionSigner } from '@solana/kit';

import { fetchVault } from './internal/generated/demoVault/accounts/vault.js';
import { getHarvestInstruction } from './internal/generated/demoVault/instructions/harvest.js';
import { associatedTokenAddress } from './internal/tokenValueAccount.js';

type SolanaRpc = Rpc<SolanaRpcApi>;

export type SolanaVaultHarvestParameters = {
  /** Donor and transfer authority over `donorUnderlying`. */
  readonly donor: TransactionSigner;
  /** Vault receiving the simulated yield. */
  readonly vault: Address;
  /** Underlying base units donated without minting shares. */
  readonly amount: bigint;
};

/** Public assets/share supply used to present the demo vault's live share price. */
export type SolanaVaultMetrics = {
  readonly underlyingMint: Address;
  readonly shareMint: Address;
  readonly vaultTokenAccount: Address;
  readonly totalAssets: bigint;
  readonly totalShares: bigint;
};

/**
 * Builds the demo vault's permissionless harvest instruction. The SDK reads the vault account so
 * callers supply only semantic roots; underlying/share/token-account wiring is never reconstructed
 * in an app.
 */
export async function buildHarvestInstruction(
  rpc: SolanaRpc,
  parameters: SolanaVaultHarvestParameters,
  config?: FetchAccountConfig,
): Promise<Instruction> {
  const vault = await fetchVault(rpc, parameters.vault, config);
  return getHarvestInstruction({
    donor: parameters.donor,
    vault: parameters.vault,
    underlyingMint: vault.data.underlyingMint,
    donorUnderlying: await associatedTokenAddress(parameters.donor.address, vault.data.underlyingMint),
    vaultTokenAccount: vault.data.vaultTokenAccount,
    amount: parameters.amount,
  });
}

/** Reads the public vault totals whose ratio determines its live share price. */
export async function getVaultMetrics(
  rpc: SolanaRpc,
  vaultAddress: Address,
  config?: FetchAccountConfig,
): Promise<SolanaVaultMetrics> {
  const vault = await fetchVault(rpc, vaultAddress, config);
  const commitment = config?.commitment;
  const rpcConfig = commitment === undefined ? {} : { commitment };
  const [assets, shares] = await Promise.all([
    rpc.getTokenAccountBalance(vault.data.vaultTokenAccount, rpcConfig).send(),
    rpc.getTokenSupply(vault.data.shareMint, rpcConfig).send(),
  ]);
  return {
    underlyingMint: vault.data.underlyingMint,
    shareMint: vault.data.shareMint,
    vaultTokenAccount: vault.data.vaultTokenAccount,
    totalAssets: BigInt(assets.value.amount),
    totalShares: BigInt(shares.value.amount),
  };
}
