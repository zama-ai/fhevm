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

export const normalizeRelayerUrl = (value: string): string => {
  const withProtocol = /^https?:\/\//i.test(value) ? value : `http://${value}`;
  return withProtocol.replace(/\/+$/, "").replace(/\/v[12]$/i, "");
};

export const resolveChain = (options: ClientOptions): FhevmChain =>
  withRelayerUrl(
    resolveNetworkConfig(options.network).fhevmChain,
    options.relayerUrl,
  );

export const resolveContractAddress = (options: ClientOptions): Hex =>
  options.contractAddress ?? resolveNetworkConfig(options.network).fheTestAddress;

export const resolveRpcUrl = (options: ClientOptions): string => {
  const config = resolveNetworkConfig(options.network);
  return rpcUrlFromOptions(options.rpcUrl, config);
};

const rpcUrlFromOptions = (
  rpcUrl: string | undefined,
  config: NetworkConfig,
): string => rpcUrl ?? process.env[config.envRpcUrl] ?? config.defaultRpcUrl;
