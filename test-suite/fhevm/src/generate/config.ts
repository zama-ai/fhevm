/**
 * Renders generated config files derived from the resolved stack state.
 */
import YAML from "yaml";

import {
  requiresLegacyRelayerKeyUrlConfig,
  requiresLegacyRelayerReadinessConfig,
} from "../compat/compat";
import { kmsPartyIds, relayerHttpNodeName } from "../kms-party";
import { hostChainRuntimes, OBJECT_STORE_INTERNAL_URL, RELAYER_HTTP_PORT } from "../layout";
import { kmsConnectorProxyUrl } from "./env";
import { solanaProgramId, solanaValidatorUrl } from "./solana";
import type { StackSpec } from "../stack-spec/stack-spec";
import type { HostChainScenario, State } from "../types";
import { hostReachableMaterialUrl, predictedCrsId, predictedKeyId } from "../utils/fs";

/** Rewrites relayer readiness config into the legacy shape when required. */
const rewriteRelayerConfig = (
  config: Record<string, unknown>,
  state: Pick<State, "versions"> & Partial<Pick<State, "overrides">>,
) => {
  if (!requiresLegacyRelayerReadinessConfig(state)) {
    return config;
  }
  const gateway = config.gateway;
  if (!gateway || typeof gateway !== "object") {
    return config;
  }
  const readiness = (gateway as Record<string, unknown>).readiness_checker;
  if (!readiness || typeof readiness !== "object") {
    return config;
  }
  const current = readiness as Record<string, unknown>;
  (gateway as Record<string, unknown>).readiness_checker = Object.fromEntries(
    Object.entries({
      retry:
        current.retry ??
        (current.gw_ciphertext_check as Record<string, unknown> | undefined)?.retry ??
        (current.host_acl_check as Record<string, unknown> | undefined)?.retry,
      public_decrypt: current.public_decrypt,
      user_decrypt: current.user_decrypt,
      delegated_user_decrypt: current.delegated_user_decrypt,
    }).filter(([, value]) => value !== undefined),
  );
  return config;
};

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
    if (chain.type === "solana") {
      // Solana chain ids (DD-052) exceed Number.MAX_SAFE_INTEGER, so the id stays a string, which YAML quotes;
      // the relayer accepts a quoted chain_id. The relayer serves Solana input proofs only and
      // reads no Solana account: it checks that acl_address, the zama-host program, is base58,
      // and never calls `url`.
      return {
        chain_id: chain.chainId,
        url: solanaValidatorUrl(chain),
        acl_address: solanaProgramId(state.discovery, chain.key),
      };
    }
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
  let config = rewriteRelayerConfig(YAML.parse(templateText) as Record<string, unknown>, state);
  config = rewriteRelayerKeyUrlConfig(config, state);
  const chains = plan?.hostChains ?? [];
  if (chains.length) {
    config = rewriteHostChains(config, state, chains);
    config = rewriteProtocolConfig(config, state, chains);
  }
  return YAML.stringify(config);
};

/**
 * Renders relayer-http's config from its checked-in example: every host chain in
 * supported_chain_ids, as strings since a Solana chain id exceeds Number.MAX_SAFE_INTEGER, and one
 * endpoint per kms-connector proxy, named after its party, whose `audience` is that party's KMS
 * tx-sender. The FhevmSig domain is the one the proxies verify: the connectors' Ethereum chain id
 * and ProtocolConfig. Every party's proxy is listed, spares included, so a node swap needs no new
 * config or restart: a party outside the active context only adds a refused call, which the
 * kms-context-switch leg checks ends at its first attempt. A production deployment lists its
 * current nodes and updates the list on a node change.
 */
export const renderRelayerHttpConfig = (
  baseText: string,
  plan: Pick<StackSpec, "hostChains" | "kms">,
  envs: Record<string, Record<string, string>>,
) => {
  const config = YAML.parse(baseText) as {
    log: Record<string, unknown>;
    http: Record<string, unknown>;
    kms_aggregator: {
      public_decrypt: { threshold: number };
      user_decrypt: { threshold: number };
      auth: { chain_id: number; protocol_config: string };
      endpoints: unknown[];
    };
  };
  // JSON lines, as in the preview: the e2e reads a request's lines back by its request id.
  config.log.format = "json";
  config.http.endpoint = `0.0.0.0:${RELAYER_HTTP_PORT}`;
  config.http.supported_chain_ids = plan.hostChains.map((chain) => chain.chainId);
  config.kms_aggregator.public_decrypt.threshold = Number(envs["gateway-sc"].PUBLIC_DECRYPTION_THRESHOLD);
  config.kms_aggregator.user_decrypt.threshold = Number(envs["gateway-sc"].USER_DECRYPTION_THRESHOLD);
  config.kms_aggregator.auth.chain_id = Number(envs["kms-connector"].KMS_CONNECTOR_ETHEREUM_CHAIN_ID);
  config.kms_aggregator.auth.protocol_config = envs["kms-connector"].KMS_CONNECTOR_PROTOCOL_CONFIG_CONTRACT__ADDRESS;
  config.kms_aggregator.endpoints = kmsPartyIds(plan.kms.parties).map((party) => ({
    name: relayerHttpNodeName(party),
    url: kmsConnectorProxyUrl(envs, party),
    audience: envs["gateway-sc"][`KMS_TX_SENDER_ADDRESS_${party - 1}`],
  }));
  return YAML.stringify(config);
};
