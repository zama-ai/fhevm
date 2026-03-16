import type { State } from "./types";

type CompatSemver = readonly [number, number, number];
type CompatService = "host-listener" | "host-listener-poller" | "sns-worker" | "transaction-sender";
type CompatArgValue = { env: string } | { value: string };

export type CompatPolicy = {
  coprocessorArgs: Partial<Record<CompatService, Array<readonly [string, CompatArgValue]>>>;
  connectorEnv: Record<string, string>;
};

/**
 * Single source of truth for all version compatibility knowledge.
 *
 * CI workflows read externalDefaults and anchors via `./fhevm-cli compat-defaults`
 * instead of hardcoding values, so updating this matrix propagates everywhere.
 */
export const COMPAT_MATRIX = {
  /**
   * Cross-component version pairs that cause runtime failures.
   * The CLI validates these at boot and rejects incompatible bundles.
   *
   * To add a new incompatibility:
   *   1. Add an entry here with a unique `code`
   *   2. Run `bun test` — validateBundleCompatibility tests will cover it
   */
  incompatibilities: [
    {
      code: "relayer-v1-vs-test-suite-v2",
      left:  { key: "RELAYER_VERSION",    below: [0, 10, 0] as CompatSemver },
      right: { key: "TEST_SUITE_VERSION", atOrAbove: [0, 11, 0] as CompatSemver },
      message: "Relayer only serves /v1 API, but test-suite expects /v2. Upgrade relayer to >= v0.10.0 or pin test-suite below v0.11.0.",
    },
  ],

  /**
   * Components below a version threshold need legacy CLI flags or env var rewrites.
   * Each entry references a profile from SHIM_PROFILES.
   *
   * To add a new shim:
   *   1. Add the profile to SHIM_PROFILES (what flags/env to inject)
   *   2. Add an entry here (when to inject them)
   *   3. Run `bun test` — compatPolicyForState tests will cover it
   */
  legacyShims: [
    { key: "COPROCESSOR_HOST_LISTENER_VERSION", below: [0, 12, 0] as CompatSemver, profile: "legacy-coprocessor-api-keys" },
    { key: "COPROCESSOR_TX_SENDER_VERSION",     below: [0, 12, 0] as CompatSemver, profile: "legacy-tx-sender-gateway-flags" },
    { key: "COPROCESSOR_TX_SENDER_VERSION",     below: [0, 11, 1] as CompatSemver, profile: "legacy-tx-sender-host-chain-url" },
    { key: "CONNECTOR_GW_LISTENER_VERSION",     below: [0, 11, 0] as CompatSemver, profile: "legacy-connector-chain-id" },
  ],

  /**
   * Pinned versions for components NOT built from this workspace.
   * CI reads these via `./fhevm-cli compat-defaults` instead of hardcoding.
   *
   * To bump the relayer pin:
   *   1. Update the SHA here
   *   2. Run `bun test` — done. CI picks it up automatically.
   */
  externalDefaults: {
    RELAYER_VERSION: "sha-29b0750",
    RELAYER_MIGRATE_VERSION: "sha-29b0750",
  },

  /**
   * Git history anchors used for target resolution.
   * The simple-ACL cutover is the oldest commit the CLI will accept for
   * SHA targets and latest-main resolution.
   */
  anchors: {
    SIMPLE_ACL_MIN_SHA: "803f1048727eabf6d8b3df618203e3c7dda77890",
  },
} as const;

const SHIM_PROFILES = {
  "legacy-coprocessor-api-keys": {
    coprocessorArgs: {
      "host-listener": [["--coprocessor-api-key", { env: "COPROCESSOR_API_KEY" }]],
      "host-listener-poller": [["--coprocessor-api-key", { env: "COPROCESSOR_API_KEY" }]],
      "sns-worker": [["--tenant-api-key", { env: "TENANT_API_KEY" }]],
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
      "transaction-sender": [["--host-chain-url", { env: "RPC_WS_URL" }]],
    },
    connectorEnv: {},
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
    connectorEnv: {},
  },
} as const satisfies Record<string, CompatPolicy>;

const parseCompatVersion = (version: string) => {
  const match = version.match(/^v?(\d+)\.(\d+)\.(\d+)/);
  if (!match) {
    return undefined;
  }
  const [, major, minor, patch] = match;
  return [Number(major), Number(minor), Number(patch)] as const;
};

/**
 * Return true when `version` is older than `target`.
 * Unparsable versions (e.g. SHA tags) are treated as modern (returns false).
 * The preset and CI systems already treat SHA targets as modern — compat agrees.
 */
const versionLt = (version: string, target: CompatSemver) => {
  const parsed = parseCompatVersion(version);
  if (!parsed) return false;
  for (let index = 0; index < parsed.length; index += 1) {
    if (parsed[index] !== target[index]) return parsed[index] < target[index];
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

/** Test-suite SDK < v0.11.0 appends /v1/ to RELAYER_URL; >= v0.11.0 expects the URL to include the version path. */
export const requiresLegacyRelayerUrl = (state: Pick<State, "versions">) =>
  versionLt(state.versions.env.TEST_SUITE_VERSION ?? "", [0, 11, 0]);

export type BundleIncompatibility = { severity: "error"; code: string; message: string };

/**
 * Detect cross-component version incompatibilities that would cause runtime failures.
 * Returns an empty array when the bundle is consistent.
 */
export const validateBundleCompatibility = (
  state: Pick<State, "versions">,
): BundleIncompatibility[] => {
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

export const compatPolicyForState = (state: State): CompatPolicy => {
  const policy: CompatPolicy = { coprocessorArgs: {}, connectorEnv: {} };
  for (const shim of COMPAT_MATRIX.legacyShims) {
    if (!versionLt(state.versions.env[shim.key] ?? "", shim.below)) {
      continue;
    }
    const profile = SHIM_PROFILES[shim.profile];
    for (const [service, args] of Object.entries(profile.coprocessorArgs)) {
      policy.coprocessorArgs[service as CompatService] = [
        ...(policy.coprocessorArgs[service as CompatService] ?? []),
        ...args,
      ];
    }
    Object.assign(policy.connectorEnv, profile.connectorEnv);
  }
  return policy;
};
