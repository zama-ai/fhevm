import type { VaultDemoRoots } from "@fhevm/sdk/solana/vault";

import type { DemoConfig } from "./demoSession";

export type VaultDirection = "deposit" | "redeem";

export const vaultRoots = (config: DemoConfig, direction: VaultDirection): VaultDemoRoots => {
  const redeem = direction === "redeem";
  return {
    batcherProgram: config.programs.batcher,
    tokenProgram: config.programs.token,
    vaultProgram: config.programs.vault,
    hostProgram: config.programs.host,
    batcher: config.batchers[direction].batcher,
    joinConfidentialMint: redeem ? config.mints.payoutConfidential : config.mints.joinConfidential,
    payoutConfidentialMint: redeem ? config.mints.joinConfidential : config.mints.payoutConfidential,
    joinUnderlyingMint: redeem ? config.mints.payoutUnderlying : config.mints.joinUnderlying,
    payoutUnderlyingMint: redeem ? config.mints.joinUnderlying : config.mints.payoutUnderlying,
    vault: config.vault,
    hostConfig: config.hostConfig,
    kmsContext: config.kmsContext,
  };
};
