// wallets — the keypairs a scenario generates for itself, and where their SOL goes afterwards.
//
// Zero protocol knowledge: a run wallet is a fresh keypair funded through the environment's
// provisioning context (airdrop locally, deployer transfer on a live cluster). On a live cluster the
// deployer's SOL is finite, so every wallet a run made hands its remainder back at the end instead
// of stranding it on a key nobody keeps. Named actors that persist between runs are personas
// (`personas.ts`), not run wallets.

import { type Address } from "@solana/kit";

import {
  generateSolanaKeypair,
  loadKeypairSigner,
  type GeneratedKeypair,
  type SolanaProvisioningContext,
} from "../../src/solana/provision";
import type { TestEnv } from "./loadEnv";

export type RunWallets = {
  /** A fresh keypair holding `sol` SOL. */
  fresh(sol: number): Promise<GeneratedKeypair>;
  /**
   * Returns every generated wallet's remaining SOL to the deployer wallet, where funding came from
   * it (live clusters). A no-op where a faucet funded them: airdropped SOL is free. Call it as the
   * scenario's last step; a scenario that fails before it leaves that run's wallets funded, which is
   * the funding amount of the environment (`FUNDING_BY_SOURCE`) at most.
   */
  sweep(): Promise<void>;
};

export const openRunWallets = (env: TestEnv, context: SolanaProvisioningContext): RunWallets => {
  const generated: GeneratedKeypair[] = [];
  return {
    async fresh(sol) {
      const wallet = await generateSolanaKeypair();
      await context.fundSol(wallet.signer.address, sol);
      generated.push(wallet);
      return wallet;
    },
    async sweep() {
      if (env.capabilities.faucet) return;
      const deployer: Address = (await loadKeypairSigner(env.roots.deployerKeypairPath)).address;
      for (const wallet of generated.splice(0)) await context.sweepSol(wallet.signer, deployer);
    },
  };
};
