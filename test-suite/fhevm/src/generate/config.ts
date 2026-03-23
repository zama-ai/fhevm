/**
 * Renders generated config files derived from the resolved stack state.
 */
import YAML from "yaml";

import { requiresLegacyRelayerReadinessConfig } from "../compat/compat";
import { CHAIN_B_ID, CHAIN_B_PORT } from "../layout";
import type { StackSpec } from "../stack-spec/stack-spec";
import type { State } from "../types";

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

const appendChainB = (
  config: Record<string, unknown>,
  state: Pick<State, "discovery">,
) => {
  const hostChains = config.host_chains;
  if (!Array.isArray(hostChains)) return config;
  const alreadyHasChainB = hostChains.some(
    (entry: unknown) => typeof entry === "object" && entry !== null && (entry as Record<string, unknown>).chain_id === Number(CHAIN_B_ID),
  );
  if (alreadyHasChainB) return config;
  const aclAddress =
    state.discovery?.hostB?.ACL_CONTRACT_ADDRESS ??
    state.discovery?.host?.ACL_CONTRACT_ADDRESS ??
    "";
  hostChains.push({
    chain_id: Number(CHAIN_B_ID),
    url: `http://host-node-b:${CHAIN_B_PORT}`,
    acl_address: aclAddress,
  });
  return config;
};

/** Renders the relayer config file from the template and compatibility policy. */
export const renderRelayerConfig = (
  state: Pick<State, "versions" | "discovery"> & Partial<Pick<State, "overrides">>,
  templateText: string,
  plan?: Pick<StackSpec, "multiChain">,
) => {
  let config = rewriteRelayerConfig(YAML.parse(templateText) as Record<string, unknown>, state);
  if (plan?.multiChain) {
    config = appendChainB(config, state);
  }
  return YAML.stringify(config);
};
