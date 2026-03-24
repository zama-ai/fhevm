/**
 * Renders generated config files derived from the resolved stack state.
 */
import YAML from "yaml";

import { requiresLegacyRelayerReadinessConfig } from "../compat/compat";
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

/** Appends extra host chains to the relayer config, driven by hostChains topology. */
const appendExtraHostChains = (
  config: Record<string, unknown>,
  state: Pick<State, "discovery">,
  extraChains: HostChainScenario[],
) => {
  const hostChainsList = config.host_chains;
  if (!Array.isArray(hostChainsList)) return config;
  const existing = new Set(
    hostChainsList.map((entry: unknown) =>
      typeof entry === "object" && entry !== null ? (entry as Record<string, unknown>).chain_id : undefined,
    ),
  );
  const primaryAcl = Object.values(state.discovery?.hosts ?? {})[0]?.ACL_CONTRACT_ADDRESS ?? "";
  for (const chain of extraChains) {
    const chainId = Number(chain.chainId);
    if (existing.has(chainId)) continue;
    const container = chain.key.replace(/^host/, "host-node");
    const aclAddress = state.discovery?.hosts[chain.key]?.ACL_CONTRACT_ADDRESS ?? primaryAcl;
    hostChainsList.push({
      chain_id: chainId,
      url: `http://${container}:${chain.rpcPort}`,
      acl_address: aclAddress,
    });
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
  const extraChains = (plan?.hostChains ?? []).slice(1);
  if (extraChains.length > 0) {
    config = appendExtraHostChains(config, state, extraChains);
  }
  return YAML.stringify(config);
};
