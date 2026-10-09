import { afterEach } from "bun:test";
// vertical — the per-test setup the decrypt scenarios share: gate on a healthy stack, open a
// provisioning context, fund one fresh wallet, and bind the decrypt config to the live chain.
//
// Every scenario starts from exactly this bundle, so it lives in the harness rather than being
// copied into each scenario file. One wallet per test keeps scenarios fully isolated: the specimen
// programs key their state on the owner, so a fresh wallet means a fresh counter and chain, and the
// token scenarios mint fresh.

import { getAddressEncoder } from "@solana/kit";

import { bytes32HexFromId } from "../../../src/solana/addresses";
import type { FheVerticalConfig } from "../../../src/solana/fhe-vertical";
import { readHostChainId, type GeneratedKeypair, type SolanaProvisioningContext } from "../../../src/solana/provision";
import { readDecryptTrustInputs } from "../../../src/solana/target";
import { loadEnv, type TestEnv } from "../loadEnv";
import { openRunWallets, type RunWallets } from "../wallets";
import { openProvisioning } from "./provisioning";
import { ensureUp, type SolanaStack } from "./stack";

const activeWallets = new Set<RunWallets>();
afterEach(async () => {
  for (const wallets of activeWallets) await wallets.sweep();
  activeWallets.clear();
});

export type VerticalTestSetup = {
  readonly env: TestEnv;
  readonly stack: SolanaStack;
  readonly context: SolanaProvisioningContext;
  /** Every wallet this test generates, `wallet` included; the test sweeps them as its last step. */
  readonly wallets: RunWallets;
  readonly wallet: GeneratedKeypair;
  readonly config: FheVerticalConfig;
  /** The wallet's 32-byte ed25519 seed, 0x-hex — the user-decrypt signing secret. */
  readonly secretKey: string;
  /** The wallet pubkey as bytes32 hex — the attested user of the scenarios' input proofs. */
  readonly walletHex: `0x${string}`;
};

/** Healthy stack + provisioning context + one funded fresh wallet + live decrypt config. */
export const verticalSetup = async (): Promise<VerticalTestSetup> => {
  const env = loadEnv();
  const stack = await ensureUp(env);
  const context = await openProvisioning(env);
  const wallets = openRunWallets(env, context);
  activeWallets.add(wallets);
  const wallet = await wallets.fresh(env.funding.primarySol);
  // The permit path's trust inputs (party ids follow the signer registry order). The local stack
  // runs the test FHE parameter set.
  const trust = await readDecryptTrustInputs(env);
  const config: FheVerticalConfig = {
    rpcUrl: env.rpcUrl,
    relayerUrl: env.relayerUrl,
    // From the live HostConfig account, not the env: the decrypts must bind the chain id the
    // deployed host actually signs for.
    chainId: await readHostChainId(context),
    userDecryptContextId: env.userDecryptContextId ?? trust.kmsContextId.toString(),
    verifyingProgramId: env.aclProgram,
    kmsSigners: trust.kmsSigners,
    kmsEpochId: bytes32HexFromId(trust.kmsEpochId),
    fheParameter: "test",
    gatewayChainId: trust.gatewayChainId.toString(),
    gatewayDecryptionContract: trust.decryptionContract,
  };
  const secretKey = `0x${Buffer.from(wallet.bytes.subarray(0, 32)).toString("hex")}`;
  const walletHex = `0x${Buffer.from(getAddressEncoder().encode(wallet.signer.address)).toString("hex")}` as const;
  return { env, stack, context, wallets, wallet, config, secretKey, walletHex };
};
