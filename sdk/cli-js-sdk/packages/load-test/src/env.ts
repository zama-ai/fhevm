import {
  DEFAULT_NETWORK,
  NETWORKS,
  normalizeRelayerUrl,
  resolveNetworkConfig,
  type NetworkName,
} from "@cli-fhevm-sdk/toolkit";
import { join } from "node:path";
import type { Hex } from "viem";
import { mnemonicToAccount, privateKeyToAccount, type Account } from "viem/accounts";

/** Resolved tool environment shared by pool, run, and report commands. */
export type LoadTestEnv = Readonly<{
  network: NetworkName;
  /** Relayer origin used for HTTP and SDK calls. */
  relayerUrl: string;
  /** API route prefix for the primary relayer. */
  relayerApiPrefix?: string;
  /** Optional candidate relayer origin for paired A/B dispatch. */
  relayerBUrl?: string;
  /** API route prefix for the candidate relayer. */
  relayerBApiPrefix?: string;
  rpcUrl?: string;
  contractAddress?: Hex;
  /** Host-chain id used in request bodies. */
  contractChainId: number;
  /** Root for pools and run artifacts. */
  dataDir: string;
  /** Optional path to the relayer config file snapshotted into reports. */
  relayerConfigPath?: string;
  /** Optional path to the candidate relayer config file snapshotted into reports. */
  relayerBConfigPath?: string;
}>;

export type EnvOverrides = Readonly<{
  network?: string;
  relayerUrl?: string;
  relayerApiPrefix?: string;
  relayerBUrl?: string;
  relayerBApiPrefix?: string;
  rpcUrl?: string;
  contractAddress?: string;
  dataDir?: string;
  relayerConfigPath?: string;
  relayerBConfigPath?: string;
}>;

export const resolveEnv = (overrides: EnvOverrides = {}): LoadTestEnv => {
  const network = (overrides.network ??
    process.env.LOAD_TEST_NETWORK ??
    DEFAULT_NETWORK) as NetworkName;
  if (!NETWORKS.includes(network)) {
    throw new Error(`Unknown network "${network}". Expected one of: ${NETWORKS.join(", ")}.`);
  }
  const networkConfig = resolveNetworkConfig(network);
  const relayerUrl = normalizeRelayerUrl(
    overrides.relayerUrl ??
      process.env.LOAD_TEST_RELAYER_URL ??
      networkConfig.fhevmChain.fhevm.relayerUrl,
  );
  const relayerBUrlRaw =
    overrides.relayerBUrl ?? process.env.LOAD_TEST_RELAYER_B_URL;

  return {
    network,
    relayerUrl,
    relayerApiPrefix:
      overrides.relayerApiPrefix ?? process.env.LOAD_TEST_RELAYER_API_PREFIX,
    relayerBUrl: relayerBUrlRaw ? normalizeRelayerUrl(relayerBUrlRaw) : undefined,
    relayerBApiPrefix:
      overrides.relayerBApiPrefix ?? process.env.LOAD_TEST_RELAYER_B_API_PREFIX,
    rpcUrl: overrides.rpcUrl ?? process.env[networkConfig.envRpcUrl] ?? undefined,
    contractAddress: overrides.contractAddress as Hex | undefined,
    contractChainId: networkConfig.hostChain.id,
    dataDir: overrides.dataDir ?? process.env.LOAD_TEST_DATA_DIR ?? ".load-test",
    relayerConfigPath:
      overrides.relayerConfigPath ?? process.env.LOAD_TEST_RELAYER_CONFIG,
    relayerBConfigPath:
      overrides.relayerBConfigPath ?? process.env.LOAD_TEST_RELAYER_B_CONFIG,
  };
};

export const poolDir = (env: LoadTestEnv, name: string): string =>
  join(env.dataDir, "pools", env.network, name);

export const runsDir = (env: LoadTestEnv): string => join(env.dataDir, "runs");

/** Sentinel lane index for the bare `PRIVATE_KEY` account. */
export const PRIVATE_KEY_LANE = -1;
/** Sentinel lane index for the sign-only `DELEGATE_PRIVATE_KEY` account. */
export const DELEGATE_KEY_LANE = -2;

/**
 * Wallet lanes: HD accounts derived from `MNEMONIC` at sequential address
 * indices, so handle pools can be created in parallel without nonce races.
 * Negative indices are sentinels for explicit private-key accounts.
 */
export const laneAccount = (index: number): Account => {
  if (index === PRIVATE_KEY_LANE) {
    const privateKey = process.env.PRIVATE_KEY as Hex | undefined;
    if (!privateKey) throw new Error("Pool references PRIVATE_KEY but it is not set.");
    return privateKeyToAccount(privateKey);
  }
  if (index === DELEGATE_KEY_LANE) {
    const privateKey = process.env.DELEGATE_PRIVATE_KEY as Hex | undefined;
    if (!privateKey) {
      throw new Error("Pool references DELEGATE_PRIVATE_KEY but it is not set.");
    }
    return privateKeyToAccount(privateKey);
  }
  const mnemonic = process.env.MNEMONIC;
  if (!mnemonic) {
    throw new Error(`Pool references mnemonic lane ${index.toString()} but MNEMONIC is not set.`);
  }
  return mnemonicToAccount(mnemonic, { addressIndex: index });
};

/** Lane indices for `count` lanes given available credentials. */
export const laneIndices = (count: number): number[] => {
  if (process.env.MNEMONIC) return Array.from({ length: count }, (_, i) => i);
  if (process.env.PRIVATE_KEY) return [PRIVATE_KEY_LANE];
  throw new Error("Provide MNEMONIC (preferred, enables parallel lanes) or PRIVATE_KEY.");
};

/**
 * Delegate account lane for delegated-user-decrypt pools. The delegate only
 * signs permits (never funds transactions): a high HD index when a mnemonic
 * is available, otherwise the dedicated `DELEGATE_PRIVATE_KEY`.
 */
export const delegateLaneIndex = (mnemonicDelegateIndex: number): number => {
  if (process.env.MNEMONIC) return mnemonicDelegateIndex;
  if (process.env.DELEGATE_PRIVATE_KEY) return DELEGATE_KEY_LANE;
  throw new Error(
    "Delegated pools need a distinct delegate account: set MNEMONIC or DELEGATE_PRIVATE_KEY.",
  );
};
