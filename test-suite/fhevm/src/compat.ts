import type { RuntimePlan } from "./runtime-plan";
import type { State } from "./types";
import { effectiveOverrides } from "./scenario";

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
  coprocessorDisableHealthcheck: Partial<Record<CompatService, true>>;
  connectorEnv: Record<string, string>;
};

/**
 * Single source of truth for the small amount of compatibility knowledge
 * the CLI must own explicitly.
 *
 * Why this file exists:
 * - the stack is assembled from components that do not all evolve in lockstep
 * - some cross-version pairs are known-invalid and should fail fast at boot
 * - some older supported images still need runtime shims to boot under the
 *   current CLI
 * - modern non-network targets (`latest-main`, `sha`) still need maintained
 *   defaults for non-repo companions such as relayer
 *
 * This file should stay narrow. It is for durable policy-level rules, not for
 * encoding every transient cross-repo break manually.
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
    { key: "COPROCESSOR_GW_LISTENER_VERSION",  below: [0, 12, 0] as CompatSemver, profile: "legacy-gw-listener-no-drift-addresses", unparsed: "legacy" as const },
    { key: "COPROCESSOR_HOST_LISTENER_VERSION", below: [0, 12, 0] as CompatSemver, profile: "legacy-coprocessor-api-keys", unparsed: "modern" as const },
    { key: "COPROCESSOR_TX_SENDER_VERSION",     below: [0, 12, 0] as CompatSemver, profile: "legacy-tx-sender-gateway-flags", unparsed: "modern" as const },
    { key: "COPROCESSOR_TX_SENDER_VERSION",     below: [0, 11, 1] as CompatSemver, profile: "legacy-tx-sender-host-chain-url", unparsed: "modern" as const },
    { key: "CONNECTOR_GW_LISTENER_VERSION",     below: [0, 11, 0] as CompatSemver, profile: "legacy-connector-chain-id", unparsed: "modern" as const },
  ],

  /**
   * Pinned versions for components NOT built from this workspace.
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
  "legacy-gw-listener-no-drift-addresses": {
    coprocessorArgs: {},
    coprocessorDropFlags: {
      "gw-listener": ["--ciphertext-commits-address", "--gateway-config-address"],
    },
    coprocessorDisableHealthcheck: {
      "gw-listener": true,
    },
    connectorEnv: {},
  },
  "legacy-coprocessor-api-keys": {
    coprocessorArgs: {
      "host-listener": [["--coprocessor-api-key", { env: "COPROCESSOR_API_KEY" }]],
      "host-listener-poller": [["--coprocessor-api-key", { env: "COPROCESSOR_API_KEY" }]],
      "sns-worker": [["--tenant-api-key", { env: "TENANT_API_KEY" }]],
    },
    coprocessorDropFlags: {},
    coprocessorDisableHealthcheck: {},
    connectorEnv: {},
  },
  "legacy-connector-chain-id": {
    coprocessorArgs: {},
    coprocessorDropFlags: {},
    coprocessorDisableHealthcheck: {},
    connectorEnv: {
      KMS_CONNECTOR_CHAIN_ID: "KMS_CONNECTOR_GATEWAY_CHAIN_ID",
    },
  },
  "legacy-tx-sender-host-chain-url": {
    coprocessorArgs: {
      "transaction-sender": [["--host-chain-url", { env: "RPC_WS_URL" }]],
    },
    coprocessorDropFlags: {},
    coprocessorDisableHealthcheck: {},
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
    coprocessorDropFlags: {},
    coprocessorDisableHealthcheck: {},
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
 * Unparsable versions (e.g. SHA tags) may be treated as modern or legacy,
 * depending on whether the caller is applying destructive or additive compat.
 */
const versionLt = (
  version: string,
  target: CompatSemver,
  options?: { unparsed?: "modern" | "legacy" },
) => {
  const parsed = parseCompatVersion(version);
  if (!parsed) return options?.unparsed === "legacy";
  for (let index = 0; index < parsed.length; index += 1) {
    if (parsed[index] !== target[index]) return parsed[index] < target[index];
  }
  return false;
};

type CompatState =
  | Pick<State, "versions" | "overrides" | "scenario">
  | Pick<RuntimePlan, "versions" | "overrides" | "coprocessor">;

const effectiveCompatOverrides = (state: CompatState) =>
  effectiveOverrides(
    state.overrides,
    "scenario" in state ? state.scenario : state.coprocessor,
  );

const usesModernWorkspaceProtocol = (state: CompatState) =>
  ["coprocessor", "gateway-contracts", "host-contracts"].every((group) =>
    effectiveCompatOverrides(state).some((override) => override.group === group),
  );

export const requiresMultichainAclAddress = (state: CompatState) =>
  !usesModernWorkspaceProtocol(state) && versionLt(state.versions.env.COPROCESSOR_TX_SENDER_VERSION ?? "", [0, 12, 0]);

export const requiresLegacyRelayerReadinessConfig = (state: Pick<CompatState, "versions">) =>
  versionLt(state.versions.env.RELAYER_VERSION ?? "", [0, 10, 0]);

/** Test-suite SDK < v0.11.0 appends /v1/ to RELAYER_URL; >= v0.11.0 expects the URL to include the version path. */
export const requiresLegacyRelayerUrl = (state: Pick<CompatState, "versions">) =>
  versionLt(state.versions.env.TEST_SUITE_VERSION ?? "", [0, 11, 0]);

export type BundleIncompatibility = { severity: "error"; code: string; message: string };

/**
 * Detect cross-component version incompatibilities that would cause runtime failures.
 * Returns an empty array when the bundle is consistent.
 */
export const validateBundleCompatibility = (
  state: Pick<CompatState, "versions">,
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

export const compatPolicyForState = (state: CompatState): CompatPolicy => {
  const policy: CompatPolicy = {
    coprocessorArgs: {},
    coprocessorDropFlags: {},
    coprocessorDisableHealthcheck: {},
    connectorEnv: {},
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
    for (const [service, disabled] of Object.entries(profile.coprocessorDisableHealthcheck)) {
      if (disabled) {
        policy.coprocessorDisableHealthcheck[service as CompatService] = true;
      }
    }
    Object.assign(policy.connectorEnv, profile.connectorEnv);
  }
  return policy;
};
