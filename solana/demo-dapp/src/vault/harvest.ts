import { findAssociatedTokenPda, TOKEN_PROGRAM_ADDRESS } from '@solana-program/token';
import type { Address, Instruction, Rpc, SolanaRpcApi, TransactionSigner } from '@solana/kit';

import { fetchVault } from './internal/generated/demoVault/accounts/vault.js';
import { getHarvestInstruction } from './internal/generated/demoVault/instructions/harvest.js';

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
): Promise<Instruction> {
  const vault = await fetchVault(rpc, parameters.vault);
  return getHarvestInstruction({
    donor: parameters.donor,
    vault: parameters.vault,
    underlyingMint: vault.data.underlyingMint,
    donorUnderlying: (await findAssociatedTokenPda({
      owner: parameters.donor.address,
      tokenProgram: TOKEN_PROGRAM_ADDRESS,
      mint: vault.data.underlyingMint,
    }))[0],
    vaultTokenAccount: vault.data.vaultTokenAccount,
    amount: parameters.amount,
  });
}

/** Reads the public vault totals whose ratio determines its live share price. */
export async function getVaultMetrics(
  rpc: SolanaRpc,
  vaultAddress: Address,
): Promise<SolanaVaultMetrics> {
  const vault = await fetchVault(rpc, vaultAddress);
  const [assets, shares] = await Promise.all([
    rpc.getTokenAccountBalance(vault.data.vaultTokenAccount).send(),
    rpc.getTokenSupply(vault.data.shareMint).send(),
  ]);
  return {
    underlyingMint: vault.data.underlyingMint,
    shareMint: vault.data.shareMint,
    vaultTokenAccount: vault.data.vaultTokenAccount,
    totalAssets: BigInt(assets.value.amount),
    totalShares: BigInt(shares.value.amount),
  };
}
