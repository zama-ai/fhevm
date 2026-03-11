import type { State } from "./types";

type CompatSemver = readonly [number, number, number];
type CompatService = "host-listener" | "host-listener-poller" | "sns-worker" | "transaction-sender";

export type CompatPolicy = {
  coprocessorArgs: Partial<Record<CompatService, Array<readonly [string, string]>>>;
  connectorEnv: Record<string, string>;
};

const COMPAT_PROFILES = {
  "legacy-coprocessor-api-keys": {
    coprocessorArgs: {
      "host-listener": [["--coprocessor-api-key", "COPROCESSOR_API_KEY"]],
      "host-listener-poller": [["--coprocessor-api-key", "COPROCESSOR_API_KEY"]],
      "sns-worker": [["--tenant-api-key", "TENANT_API_KEY"]],
    },
    connectorEnv: {},
  },
  "legacy-connector-chain-id": {
    coprocessorArgs: {},
    connectorEnv: {
      KMS_CONNECTOR_CHAIN_ID: "KMS_CONNECTOR_GATEWAY_CHAIN_ID",
    },
  },
  "legacy-tx-sender-host-chain-url": {
    coprocessorArgs: {
      "transaction-sender": [["--host-chain-url", "RPC_WS_URL"]],
    },
    connectorEnv: {},
  },
  "legacy-tx-sender-gateway-flags": {
    coprocessorArgs: {
      "transaction-sender": [
        ["--multichain-acl-address", "MULTICHAIN_ACL_ADDRESS"],
        ["--delegation-fallback-polling", "30"],
        ["--delegation-max-retry", "100000"],
        ["--retry-immediately-on-nonce-error", "2"],
      ],
    },
    connectorEnv: {},
  },
} as const satisfies Record<string, CompatPolicy>;

const COMPAT_RULES = {
  coprocessor: [
    { before: [0, 12, 0] as CompatSemver, profile: "legacy-coprocessor-api-keys" },
    { before: [0, 12, 0] as CompatSemver, profile: "legacy-tx-sender-gateway-flags" },
    { before: [0, 11, 1] as CompatSemver, profile: "legacy-tx-sender-host-chain-url" },
  ],
  connector: [{ before: [0, 11, 0] as CompatSemver, profile: "legacy-connector-chain-id" }],
} as const;

const parseCompatVersion = (version: string) => {
  const match = version.match(/^v?(\d+)\.(\d+)\.(\d+)/);
  if (!match) {
    return undefined;
  }
  const [, major, minor, patch] = match;
  return [Number(major), Number(minor), Number(patch)] as const;
};

const versionLt = (version: string, target: CompatSemver) => {
  const parsed = parseCompatVersion(version);
  if (!parsed) {
    return false;
  }
  for (let index = 0; index < parsed.length; index += 1) {
    if (parsed[index] !== target[index]) {
      return parsed[index] < target[index];
    }
  }
  return false;
};

const usesModernWorkspaceProtocol = (state: Pick<State, "overrides">) =>
  ["coprocessor", "gateway-contracts", "host-contracts"].every((group) =>
    state.overrides.some((override) => override.group === group),
  );

export const requiresMultichainAclAddress = (state: Pick<State, "versions" | "overrides">) =>
  !usesModernWorkspaceProtocol(state) && versionLt(state.versions.env.COPROCESSOR_TX_SENDER_VERSION ?? "", [0, 12, 0]);

export const requiresLegacyRelayerReadinessConfig = (state: Pick<State, "versions">) =>
  versionLt(state.versions.env.RELAYER_VERSION ?? "", [0, 10, 0]);

export const compatPolicyForState = (state: State): CompatPolicy => {
  const policy: CompatPolicy = { coprocessorArgs: {}, connectorEnv: {} };
  for (const rule of COMPAT_RULES.coprocessor) {
    if (!versionLt(state.versions.env.COPROCESSOR_HOST_LISTENER_VERSION ?? "", rule.before)) {
      continue;
    }
    const profile = COMPAT_PROFILES[rule.profile];
    for (const [service, args] of Object.entries(profile.coprocessorArgs)) {
      policy.coprocessorArgs[service as CompatService] = [
        ...(policy.coprocessorArgs[service as CompatService] ?? []),
        ...args,
      ];
    }
  }
  for (const rule of COMPAT_RULES.connector) {
    if (!versionLt(state.versions.env.CONNECTOR_GW_LISTENER_VERSION ?? "", rule.before)) {
      continue;
    }
    Object.assign(policy.connectorEnv, COMPAT_PROFILES[rule.profile].connectorEnv);
  }
  return policy;
};
