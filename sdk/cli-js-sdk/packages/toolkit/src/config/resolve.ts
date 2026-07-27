import type { FhevmChain } from "@fhevm/sdk/chains";
import type { Hex } from "viem";

import { resolveNetworkConfig } from "./networks";
import type { ClientOptions, NetworkConfig } from "./types";

const withRelayerUrl = (chain: FhevmChain, relayerUrl?: string): FhevmChain => {
  if (!relayerUrl) return chain;

  return {
    ...chain,
    fhevm: {
      ...chain.fhevm,
      relayerUrl: normalizeRelayerUrl(relayerUrl),
    },
  };
};

/**
 * Normalizes relayer overrides for SDK consumption.
 *
 * The SDK expects the relayer origin, so user-provided `/v1` or `/v2` suffixes
 * are stripped.
 */
export const normalizeRelayerUrl = (value: string): string => {
  const withProtocol = /^https?:\/\//i.test(value) ? value : `http://${value}`;
  return withProtocol.replace(/\/+$/, "").replace(/\/v[12]$/i, "");
};

/** Resolves the FHEVM chain, including optional relayer URL override. */
export const resolveChain = (options: ClientOptions): FhevmChain =>
  withRelayerUrl(
    resolveNetworkConfig(options.network).fhevmChain,
    options.relayerUrl,
  );

/** Resolves the FHETest contract address for a command invocation. */
export const resolveContractAddress = (options: ClientOptions): Hex =>
  options.contractAddress ?? resolveNetworkConfig(options.network).fheTestAddress;

/** Resolves RPC URL from CLI option, network-specific env var, then default. */
export const resolveRpcUrl = (options: ClientOptions): string => {
  const config = resolveNetworkConfig(options.network);
  return rpcUrlFromOptions(options.rpcUrl, config);
};

const rpcUrlFromOptions = (
  rpcUrl: string | undefined,
  config: NetworkConfig,
): string => rpcUrl?.trim() || process.env[config.envRpcUrl]?.trim() || config.defaultRpcUrl;
