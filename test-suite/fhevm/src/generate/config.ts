/**
 * Renders generated config files derived from the resolved stack state.
 */
import YAML from "yaml";

import {
  requiresLegacyKmsCoreConfig,
  requiresLegacyRelayerReadinessConfig,
} from "../compat/compat";
import { hostChainRuntimes } from "../layout";
import { solanaProgramId, solanaValidatorUrl } from "./solana";
import type { StackSpec } from "../stack-spec/stack-spec";
import type { HostChainScenario, State } from "../types";

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

/** Rebuilds relayer host chain config from the active topology plan. */
const rewriteHostChains = (
  config: Record<string, unknown>,
  state: Pick<State, "discovery">,
  chains: HostChainScenario[],
) => {
  if (!Array.isArray(config.host_chains)) return config;
  config.host_chains = hostChainRuntimes(chains).map((chain) => {
    if (chain.type === "solana") {
      // RFC-021 ids exceed i64::MAX; the relayer config parser accepts them only as strings (YAML
      // quotes a JS string). acl_address = zama-host program (base58) — the relayer uses it only as
      // a Solana-host discriminant and never connects (Solana ACL is KMS-enforced), so `url` is
      // inert config; it points at the host-native validator for consistency.
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
  const chains = plan?.hostChains ?? [];
  if (chains.length) {
    config = rewriteHostChains(config, state, chains);
    config = rewriteProtocolConfig(config, state, chains);
  }
  return YAML.stringify(config);
};

/** Selects the kms-core config template that matches the requested core image schema. */
export const renderKmsCoreConfig = (
  state: Pick<State, "versions">,
  legacyTemplateText: string,
  modernTemplateText: string,
) => (requiresLegacyKmsCoreConfig(state) ? legacyTemplateText : modernTemplateText);
