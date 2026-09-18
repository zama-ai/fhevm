// provisioning — opens the provisioning context for a TestEnv, with the funding source the
// environment implies: a validator airdrop where a faucet exists, otherwise System transfers from
// the deployer wallet. Every harness path that creates actors funds them through this context.

import {
  createProvisioningContext,
  loadKeypairSigner,
  type ProvisioningContextOptions,
  type SolanaProvisioningContext,
} from "../../../src/solana/provision";
import type { TestEnv } from "../loadEnv";

export const openProvisioning = async (
  env: TestEnv,
  options: Omit<ProvisioningContextOptions, "funder"> = {},
): Promise<SolanaProvisioningContext> => {
  const funder = env.capabilities.faucet ? undefined : await loadKeypairSigner(env.roots.deployerKeypairPath);
  return createProvisioningContext(env.rpcUrl, env.wsUrl, { ...options, funder });
};
