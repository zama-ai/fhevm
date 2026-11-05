/**
 * Renders generated config files derived from the resolved stack state.
 */
import YAML from "yaml";

import { requiresLegacyRelayerKeyUrlConfig } from "../compat/compat";
import { hostChainRuntimes, OBJECT_STORE_INTERNAL_URL, RELAYER_HTTP_PORT } from "../layout";
import type { StackSpec } from "../stack-spec/stack-spec";
import type { HostChainScenario, State } from "../types";
import { hostReachableMaterialUrl, predictedCrsId, predictedKeyId } from "../utils/fs";

/** Rewrites keyurl config into the static schema expected by released relayers. */
const rewriteRelayerKeyUrlConfig = (
  config: Record<string, unknown>,
  state: Pick<State, "versions"> & Partial<Pick<State, "discovery">>,
) => {
  if (!requiresLegacyRelayerKeyUrlConfig(state)) {
    return config;
  }
  const keyPrefix = state.discovery?.objectStoreKeyPrefix ?? "PUB";
  const materialBase = `${(
    state.discovery?.endpoints?.objectStoreExternal ??
    state.discovery?.endpoints?.objectStoreInternal ??
    OBJECT_STORE_INTERNAL_URL
  ).replace(/\/$/, "")}/kms-public/${keyPrefix}`;
  const fheKeyId = state.discovery?.actualFheKeyId ?? state.discovery?.fheKeyId ?? predictedKeyId();
  const crsKeyId = state.discovery?.actualCrsKeyId ?? state.discovery?.crsKeyId ?? predictedCrsId();
  config.keyurl = {
    fhe_public_key: {
      data_id: fheKeyId,
      url: hostReachableMaterialUrl(`${materialBase}/PublicKey/${fheKeyId}`),
    },
    crs: {
      data_id: crsKeyId,
      url: hostReachableMaterialUrl(`${materialBase}/CRS/${crsKeyId}`),
    },
  };
  return config;
};

/** Rebuilds relayer host chain config from the active topology plan. */
const rewriteHostChains = (
  config: Record<string, unknown>,
  state: Pick<State, "discovery">,
  chains: HostChainScenario[],
) => {
  if (!Array.isArray(config.host_chains)) return config;
  config.host_chains = hostChainRuntimes(chains).map((chain) => {
    const chainId = Number(chain.chainId);
    const aclAddress = state.discovery?.hosts[chain.key]?.ACL_CONTRACT_ADDRESS ?? "";
    return {
      chain_id: chainId,
      url: `http://${chain.node}:${chain.rpcPort}`,
      acl_address: aclAddress,
    };
  });
  return config;
};

/** Rewrites protocol_config with the discovered ProtocolConfig address and host chain RPC. */
const rewriteProtocolConfig = (
  config: Record<string, unknown>,
  state: Pick<State, "discovery">,
  chains: HostChainScenario[],
) => {
  const pc = config.protocol_config as Record<string, unknown> | undefined;
  if (!pc) return config;
  const runtimes = hostChainRuntimes(chains);
  const defaultChain = runtimes[0];
  if (defaultChain) {
    pc.ethereum_http_rpc_url = `http://${defaultChain.node}:${defaultChain.rpcPort}`;
    const hostAddresses = state.discovery?.hosts[defaultChain.key];
    if (hostAddresses?.PROTOCOL_CONFIG_CONTRACT_ADDRESS) {
      pc.address = hostAddresses.PROTOCOL_CONFIG_CONTRACT_ADDRESS;
    }
  }
  return config;
};

/** Renders the relayer config file from the template and compatibility policy. */
export const renderRelayerConfig = (
  state: Pick<State, "versions" | "discovery"> & Partial<Pick<State, "overrides">>,
  templateText: string,
  plan?: Pick<StackSpec, "hostChains">,
) => {
  let config = YAML.parse(templateText) as Record<string, unknown>;
  config = rewriteRelayerKeyUrlConfig(config, state);
  const chains = plan?.hostChains ?? [];
  if (chains.length) {
    config = rewriteHostChains(config, state, chains);
    config = rewriteProtocolConfig(config, state, chains);
  }
  return YAML.stringify(config);
};

/**
 * Renders the relayer-http config: the KMS proxies and audiences the e2e suite gets
 * (KMS_CONNECTOR_ENDPOINT_URLS / KMS_CONNECTOR_TX_SENDER_ADDRESSES), thresholds from the MPC
 * threshold t (public t+1, user 2t+1), the host chains' ids, and the discovered ProtocolConfig.
 */
export const renderRelayerHttpConfig = (
  state: Pick<State, "discovery">,
  templateText: string,
  plan: Pick<StackSpec, "hostChains" | "kms">,
  envs: Record<string, Record<string, string>>,
) => {
  const config = YAML.parse(templateText) as {
    http: Record<string, unknown>;
    kms_aggregator: {
      public_decrypt: Record<string, unknown>;
      user_decrypt: Record<string, unknown>;
      auth: Record<string, unknown>;
      endpoints: unknown[];
    };
  };
  const chains = hostChainRuntimes(plan.hostChains);
  const defaultChain = chains[0];
  if (!defaultChain) throw new Error("Missing default host chain");
  const t = plan.kms.threshold;
  const aggregator = config.kms_aggregator;
  config.http.endpoint = `0.0.0.0:${RELAYER_HTTP_PORT}`;
  config.http.supported_chain_ids = chains.map((chain) => Number(chain.chainId));
  aggregator.public_decrypt.threshold = t + 1;
  aggregator.user_decrypt.threshold = 2 * t + 1;
  aggregator.auth.chain_id = Number(defaultChain.chainId);
  aggregator.auth.protocol_config =
    state.discovery?.hosts[defaultChain.key]?.PROTOCOL_CONFIG_CONTRACT_ADDRESS ?? aggregator.auth.protocol_config;
  aggregator.auth.address = envs["kms-connector"]?.KMS_CONNECTOR_ALLOWED_SENDERS ?? "";
  const list = (value?: string) => (value ? value.split(",") : []);
  const audiences = list(envs["test-suite"]?.KMS_CONNECTOR_TX_SENDER_ADDRESSES);
  aggregator.endpoints = list(envs["test-suite"]?.KMS_CONNECTOR_ENDPOINT_URLS).map((url, index) => ({
    name: `kms_${index + 1}`,
    url,
    audience: audiences[index] ?? "",
  }));
  return YAML.stringify(config);
};
