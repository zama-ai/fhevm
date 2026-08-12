// vertical — the per-test setup the fhe_execute scenarios share: gate on a healthy stack, open a
// provisioning context, fund one fresh wallet, and bind the decrypt config to the live chain.
//
// Every fhe-vertical/operator test starts from exactly this bundle, so it lives in the harness
// rather than being copied into each scenario file. One wallet per test keeps scenarios fully
// isolated: scenario-owned encrypted values derive from `(domain=wallet, account=wallet, label)`,
// so a fresh wallet means fresh PDAs no matter what labels other tests used.

import { getAddressEncoder } from "@solana/kit";

import { SOLANA_DEFAULT_PUBLIC_DECRYPT_CONTEXT } from "../../../src/layout";
import type { FheVerticalConfig } from "../../../src/solana/fhe-vertical";
import {
  createProvisioningContext,
  generateSolanaKeypair,
  readHostChainId,
  type GeneratedKeypair,
  type SolanaProvisioningContext,
} from "../../../src/solana/provision";
import { loadEnv, type TestEnv } from "../loadEnv";
import { ensureUp, type SolanaStack } from "./stack";

export type VerticalTestSetup = {
  readonly env: TestEnv;
  readonly stack: SolanaStack;
  readonly context: SolanaProvisioningContext;
  readonly wallet: GeneratedKeypair;
  readonly config: FheVerticalConfig;
  /** The wallet's 32-byte ed25519 seed, 0x-hex — the user-decrypt signing secret. */
  readonly secretKey: string;
  /** The wallet pubkey as bytes32 hex — the ACL domain key for scenario-owned values. */
  readonly walletHex: `0x${string}`;
};

/** Healthy stack + provisioning context + one funded fresh wallet + live decrypt config. */
export const verticalSetup = async (): Promise<VerticalTestSetup> => {
  const env = loadEnv();
  const stack = await ensureUp(env);
  const context = createProvisioningContext(env.rpcUrl, env.wsUrl);
  const wallet = await generateSolanaKeypair();
  await context.airdropSol(wallet.signer.address, 10n);
  const config: FheVerticalConfig = {
    relayerUrl: env.relayerUrl,
    proofServiceUrl: env.proofServiceUrl,
    // From the live HostConfig account, not the env: the decrypts must bind the chain id the
    // deployed host actually signs for.
    chainId: await readHostChainId(context),
    publicDecryptContextId: SOLANA_DEFAULT_PUBLIC_DECRYPT_CONTEXT,
    userDecryptContextId: env.userDecryptContextId,
  };
  const secretKey = `0x${Buffer.from(wallet.bytes.subarray(0, 32)).toString("hex")}`;
  const walletHex = `0x${Buffer.from(getAddressEncoder().encode(wallet.signer.address)).toString("hex")}` as const;
  return { env, stack, context, wallet, config, secretKey, walletHex };
};
