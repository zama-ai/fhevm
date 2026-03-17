import YAML from "yaml";

import {
  requiresLegacyRelayerReadinessConfig,
} from "./compat";
import type { State } from "./types";

export const rewriteRelayerConfig = (
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

export const renderRelayerConfig = (
  state: Pick<State, "versions"> & Partial<Pick<State, "overrides">>,
  templateText: string,
) =>
  YAML.stringify(
    rewriteRelayerConfig(
      YAML.parse(templateText) as Record<string, unknown>,
      state,
    ),
  );
