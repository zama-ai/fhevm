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

import { normalizeApiPrefix } from "./relayer/api-prefix";

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
  const relayerApiPrefix =
    overrides.relayerApiPrefix ?? process.env.LOAD_TEST_RELAYER_API_PREFIX;
  const relayerBUrlRaw =
    overrides.relayerBUrl ?? process.env.LOAD_TEST_RELAYER_B_URL;
  const relayerBApiPrefixRaw =
    overrides.relayerBApiPrefix ?? process.env.LOAD_TEST_RELAYER_B_API_PREFIX;
  const relayerBConfigPathRaw =
    overrides.relayerBConfigPath ?? process.env.LOAD_TEST_RELAYER_B_CONFIG;

  if (!relayerBUrlRaw) {
    if (relayerBApiPrefixRaw !== undefined) {
      throw new Error(
        "--relayer-b-api-prefix (or LOAD_TEST_RELAYER_B_API_PREFIX) requires --relayer-b-url " +
          "(or LOAD_TEST_RELAYER_B_URL) to also be set.",
      );
    }
    if (relayerBConfigPathRaw !== undefined) {
      throw new Error(
        "--relayer-b-config (or LOAD_TEST_RELAYER_B_CONFIG) requires --relayer-b-url " +
          "(or LOAD_TEST_RELAYER_B_URL) to also be set.",
      );
    }
  }

  const relayerBUrl = relayerBUrlRaw ? normalizeRelayerUrl(relayerBUrlRaw) : undefined;
  // Reject only when A and B are effectively the SAME target: identical
  // normalized URL AND identical effective API prefix (B falls back to A's
  // prefix when unset, mirroring how the client resolves it). Path-routed
  // deployments — one host serving A and B under distinct paths or API
  // prefixes (e.g. /v1 vs /v2) — are legitimate and must be accepted.
  if (relayerBUrl) {
    const sameUrl = new URL(relayerBUrl).href === new URL(relayerUrl).href;
    const samePrefix =
      normalizeApiPrefix(relayerApiPrefix) ===
      normalizeApiPrefix(relayerBApiPrefixRaw ?? relayerApiPrefix);
    if (sameUrl && samePrefix) {
      throw new Error(
        `--relayer-b-url (${relayerBUrl}) resolves to the same target as the primary relayer ` +
          `(${relayerUrl}) with the same API prefix; A/B comparison needs two distinct targets. ` +
          `Use different hosts, paths, or API prefixes (e.g. --relayer-api-prefix /v1 vs ` +
          `--relayer-b-api-prefix /v2).`,
      );
    }
  }

  return {
    network,
    relayerUrl,
    relayerApiPrefix,
    relayerBUrl,
    relayerBApiPrefix: relayerBApiPrefixRaw,
    rpcUrl: overrides.rpcUrl ?? process.env[networkConfig.envRpcUrl] ?? undefined,
    contractAddress: overrides.contractAddress as Hex | undefined,
    contractChainId: networkConfig.hostChain.id,
    dataDir: overrides.dataDir ?? process.env.LOAD_TEST_DATA_DIR ?? ".load-test",
    relayerConfigPath:
      overrides.relayerConfigPath ?? process.env.LOAD_TEST_RELAYER_CONFIG,
    relayerBConfigPath: relayerBConfigPathRaw,
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
