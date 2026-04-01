/**
 * Encodes legacy runtime shims and incompatibility rules across supported fhevm component version combinations.
 */
import type { State } from "../types";
import type { StackSpec } from "../stack-spec/stack-spec";
import { effectiveOverrides } from "../scenario/resolve";

type CompatSemver = readonly [number, number, number];
type CompatService =
  | "gw-listener"
  | "host-listener"
  | "host-listener-poller"
  | "sns-worker"
  | "transaction-sender";
type CompatArgValue = { env: string } | { value: string };

export type CompatPolicy = {
  coprocessorArgs: Partial<Record<CompatService, Array<readonly [string, CompatArgValue]>>>;
  coprocessorDropFlags: Partial<Record<CompatService, string[]>>;
  connectorEnv: Record<string, string>;
  composeEnv: Record<string, string>;
};

export const COMPAT_MATRIX = {
  incompatibilities: [
    {
      code: "relayer-v1-vs-test-suite-v2",
      left: { key: "RELAYER_VERSION", below: [0, 10, 0] as CompatSemver },
      right: { key: "TEST_SUITE_VERSION", atOrAbove: [0, 11, 0] as CompatSemver },
      message:
        "Relayer only serves /v1 API, but test-suite expects /v2. Upgrade relayer to >= v0.10.0 or pin test-suite below v0.11.0.",
    },
  ],
  legacyShims: [
    { key: "COPROCESSOR_GW_LISTENER_VERSION", below: [0, 12, 0] as CompatSemver, profile: "legacy-gw-listener-no-drift-addresses", unparsed: "modern" as const },
    { key: "COPROCESSOR_HOST_LISTENER_VERSION", below: [0, 12, 0] as CompatSemver, profile: "legacy-coprocessor-api-keys", unparsed: "modern" as const },
    { key: "COPROCESSOR_TX_SENDER_VERSION", below: [0, 12, 0] as CompatSemver, profile: "legacy-tx-sender-gateway-flags", unparsed: "modern" as const },
    { key: "COPROCESSOR_TX_SENDER_VERSION", below: [0, 11, 1] as CompatSemver, profile: "legacy-tx-sender-host-chain-url", unparsed: "modern" as const },
    { key: "CONNECTOR_GW_LISTENER_VERSION", below: [0, 11, 0] as CompatSemver, profile: "legacy-connector-chain-id", unparsed: "modern" as const },
  ],
  anchors: {
    SIMPLE_ACL_MIN_SHA: "803f1048727eabf6d8b3df618203e3c7dda77890",
  },
} as const;

export const LEGACY_RELAYER_IMAGE_REPOSITORY = "ghcr.io/zama-ai/console/relayer";
export const LEGACY_RELAYER_MIGRATE_IMAGE_REPOSITORY = "ghcr.io/zama-ai/console/relayer-migrate";
export const MODERN_RELAYER_IMAGE_REPOSITORY = "ghcr.io/zama-ai/fhevm/relayer";
export const MODERN_RELAYER_MIGRATE_IMAGE_REPOSITORY = "ghcr.io/zama-ai/fhevm/relayer-migrate";

const SHIM_PROFILES = {
  "legacy-gw-listener-no-drift-addresses": {
    coprocessorArgs: {},
    coprocessorDropFlags: {
      "gw-listener": ["--ciphertext-commits-address", "--gateway-config-address"],
    },
    connectorEnv: {},
    composeEnv: {},
  },
  "legacy-coprocessor-api-keys": {
    coprocessorArgs: {
      "host-listener": [["--coprocessor-api-key", { env: "COPROCESSOR_API_KEY" }]],
      "host-listener-poller": [["--coprocessor-api-key", { env: "COPROCESSOR_API_KEY" }]],
      "sns-worker": [["--tenant-api-key", { env: "TENANT_API_KEY" }]],
    },
    coprocessorDropFlags: {},
    connectorEnv: {},
    composeEnv: {},
  },
  "legacy-connector-chain-id": {
    coprocessorArgs: {},
    coprocessorDropFlags: {},
    connectorEnv: { KMS_CONNECTOR_CHAIN_ID: "KMS_CONNECTOR_GATEWAY_CHAIN_ID" },
    composeEnv: {},
  },
  "legacy-tx-sender-host-chain-url": {
    coprocessorArgs: {
      "transaction-sender": [["--host-chain-url", { env: "RPC_WS_URL" }]],
    },
    coprocessorDropFlags: {},
    connectorEnv: {},
    composeEnv: {},
  },
  "legacy-tx-sender-gateway-flags": {
    coprocessorArgs: {
      "transaction-sender": [
        ["--multichain-acl-address", { env: "MULTICHAIN_ACL_ADDRESS" }],
        ["--delegation-fallback-polling", { value: "30" }],
        ["--delegation-max-retry", { value: "100000" }],
        ["--retry-immediately-on-nonce-error", { value: "2" }],
      ],
    },
    coprocessorDropFlags: {},
    connectorEnv: {},
    composeEnv: {},
  },
} as const satisfies Record<string, CompatPolicy>;

/** Parses a semver-like version string into comparable numeric parts. */
const parseCompatVersion = (version: string) => {
  const match = version.match(/^v?(\d+)\.(\d+)\.(\d+)(?:([-+]).*)?$/);
  if (!match) {
    return undefined;
  }
  const [, major, minor, patch, suffixType] = match;
  return {
    parts: [Number(major), Number(minor), Number(patch)] as const,
    prerelease: suffixType === "-",
  };
};

const usesModernRelayerRepository = (version: string) => !parseCompatVersion(version);

export const relayerImageRepository = (version: string) =>
  usesModernRelayerRepository(version) ? MODERN_RELAYER_IMAGE_REPOSITORY : LEGACY_RELAYER_IMAGE_REPOSITORY;

export const relayerMigrateImageRepository = (version: string) =>
  usesModernRelayerRepository(version) ? MODERN_RELAYER_MIGRATE_IMAGE_REPOSITORY : LEGACY_RELAYER_MIGRATE_IMAGE_REPOSITORY;

/** Compares a version string against a compatibility floor. */
const versionLt = (
  version: string,
  target: CompatSemver,
  options?: { unparsed?: "modern" | "legacy" },
) => {
  const parsed = parseCompatVersion(version);
  if (!parsed) {
    return options?.unparsed === "legacy";
  }
  for (let index = 0; index < parsed.parts.length; index += 1) {
    if (parsed.parts[index] !== target[index]) {
      return parsed.parts[index] < target[index];
    }
  }
  return parsed.prerelease;
};

type CompatState =
  | Pick<State, "versions" | "overrides" | "scenario">
  | Pick<StackSpec, "versions" | "overrides" | "coprocessor">;

/** Computes the effective override set used for compatibility decisions. */
const effectiveCompatOverrides = (state: CompatState) =>
  effectiveOverrides(state.overrides, "scenario" in state ? state.scenario : state.coprocessor);

/** Detects when local workspace overrides imply the modern runtime protocol. */
const usesModernWorkspaceProtocol = (state: CompatState) =>
  ["coprocessor", "gateway-contracts", "host-contracts"].every((group) =>
    effectiveCompatOverrides(state).some((override) => override.group === group),
  );

export const requiresMultichainAclAddress = (state: CompatState) =>
  !usesModernWorkspaceProtocol(state) &&
  versionLt(state.versions.env.COPROCESSOR_TX_SENDER_VERSION ?? "", [0, 12, 0]);

/** Detects when relayer readiness config must stay on the legacy shape. */
export const requiresLegacyRelayerReadinessConfig = (state: Pick<CompatState, "versions">) =>
  versionLt(state.versions.env.RELAYER_VERSION ?? "", [0, 10, 0]);

/** Detects when kms-core still expects the legacy config schema. */
export const requiresLegacyKmsCoreConfig = (state: Pick<CompatState, "versions">) =>
  versionLt(state.versions.env.CORE_VERSION ?? "", [0, 13, 10]);

/** Detects when test-suite should use the legacy relayer base URL. */
export const requiresLegacyRelayerUrl = (state: Pick<CompatState, "versions">) =>
  versionLt(state.versions.env.TEST_SUITE_VERSION ?? "", [0, 11, 0]);

/** Detects when contract tasks still expect the legacy internal PauserSet flag name. */
const requiresLegacyPauserTaskFlag = (version: string) =>
  versionLt(version, [0, 12, 0], { unparsed: "modern" });

type BundleIncompatibility = { severity: "error"; code: string; message: string };

/** Detects whether the resolved bundle supports multi-chain listener/database topology. */
const requiresLegacySingleChainCoprocessor = (state: CompatState) =>
  versionLt(state.versions.env.COPROCESSOR_HOST_LISTENER_VERSION ?? "", [0, 12, 0], { unparsed: "modern" }) ||
  versionLt(state.versions.env.COPROCESSOR_HOST_LISTENER_POLLER_VERSION ?? "", [0, 12, 0], { unparsed: "modern" });

/** Evaluates the compatibility matrix against a resolved bundle. */
export const validateBundleCompatibility = (state: Pick<CompatState, "versions">): BundleIncompatibility[] => {
  const issues: BundleIncompatibility[] = [];
  for (const rule of COMPAT_MATRIX.incompatibilities) {
    const leftVersion = state.versions.env[rule.left.key] ?? "";
    const rightVersion = state.versions.env[rule.right.key] ?? "";
    if (versionLt(leftVersion, rule.left.below) && !versionLt(rightVersion, rule.right.atOrAbove)) {
      issues.push({
        severity: "error",
        code: rule.code,
        message: `${rule.left.key} ${leftVersion} / ${rule.right.key} ${rightVersion}: ${rule.message}`,
      });
    }
  }
  return issues;
};

/** Rejects multi-chain scenarios when the resolved coprocessor bundle predates multi-chain support. */
export const assertSupportedBundleScenario = (state: CompatState) => {
  const hostChains = "scenario" in state ? state.scenario.hostChains : state.coprocessor.hostChains;
  if (hostChains.length <= 1 || !requiresLegacySingleChainCoprocessor(state)) {
    return;
  }
  const hostListener = state.versions.env.COPROCESSOR_HOST_LISTENER_VERSION ?? "";
  throw new Error(
    `Multi-chain scenarios require coprocessor runtime >= v0.12.0; resolved COPROCESSOR_HOST_LISTENER_VERSION=${hostListener || "unknown"}.`,
  );
};

/** Builds the compatibility policy that rendering and runtime should apply. */
export const compatPolicyForState = (state: CompatState): CompatPolicy => {
  const policy: CompatPolicy = {
    coprocessorArgs: {},
    coprocessorDropFlags: {},
    connectorEnv: {},
    composeEnv: {},
  };
  for (const shim of COMPAT_MATRIX.legacyShims) {
    if (!versionLt(state.versions.env[shim.key] ?? "", shim.below, { unparsed: shim.unparsed })) {
      continue;
    }
    const profile = SHIM_PROFILES[shim.profile];
    for (const [service, args] of Object.entries(profile.coprocessorArgs)) {
      policy.coprocessorArgs[service as CompatService] = [
        ...(policy.coprocessorArgs[service as CompatService] ?? []),
        ...args,
      ];
    }
    for (const [service, flags] of Object.entries(profile.coprocessorDropFlags)) {
      policy.coprocessorDropFlags[service as CompatService] = [
        ...(policy.coprocessorDropFlags[service as CompatService] ?? []),
        ...flags,
      ];
    }
    Object.assign(policy.connectorEnv, profile.connectorEnv);
  }
  policy.composeEnv.HOST_ADD_PAUSERS_INTERNAL_FLAG = requiresLegacyPauserTaskFlag(
    state.versions.env.HOST_VERSION ?? "",
  )
    ? "--use-internal-pauser-set-address"
    : "--use-internal-proxy-address";
  policy.composeEnv.GATEWAY_ADD_PAUSERS_INTERNAL_FLAG = requiresLegacyPauserTaskFlag(
    state.versions.env.GATEWAY_VERSION ?? "",
  )
    ? "--use-internal-pauser-set-address"
    : "--use-internal-proxy-address";
  policy.composeEnv.RELAYER_IMAGE_REPOSITORY = relayerImageRepository(state.versions.env.RELAYER_VERSION ?? "");
  policy.composeEnv.RELAYER_MIGRATE_IMAGE_REPOSITORY = relayerMigrateImageRepository(
    state.versions.env.RELAYER_MIGRATE_VERSION ?? "",
  );
  return policy;
};
