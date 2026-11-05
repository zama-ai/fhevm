/**
 * Encodes legacy runtime shims and incompatibility rules across supported fhevm component version combinations.
 * The oldest supported release is v0.13.0: nothing here may fire only below it.
 */
import { effectiveOverrides } from "../scenario/resolve";
import type { StackSpec } from "../stack-spec/stack-spec";
import type { CoprocessorInstanceSource, ResolvedCoprocessorScenario, State } from "../types";

type CompatSemver = readonly [number, number, number];
type CompatService =
  | "gw-listener"
  | "host-listener"
  | "host-listener-poller"
  | "host-listener-consumer"
  | "sns-worker"
  | "transaction-sender";
type CompatArgValue = { env: string } | { value: string };

export type CompatPolicy = {
  coprocessorArgs: Partial<Record<CompatService, Array<readonly [string, CompatArgValue]>>>;
  coprocessorDropFlags: Partial<Record<CompatService, string[]>>;
  connectorEnv: Record<string, string>;
  composeEnv: Record<string, string>;
};

/** The command-shaping half of a policy — all a coprocessor fleet needs. */
export type CoprocessorArgPolicy = Pick<CompatPolicy, "coprocessorArgs" | "coprocessorDropFlags">;

/** A version pair that breaks at runtime: `left` below its floor together with `right` at or above its floor. */
type CompatIncompatibility = {
  code: string;
  left: { key: string; below: CompatSemver };
  right: { key: string; atOrAbove: CompatSemver };
  message: string;
};

export const COMPAT_MATRIX = {
  // Empty since v0.13.0 became the oldest supported release; kept so a new rule is a one-entry change.
  incompatibilities: [] as CompatIncompatibility[],
  legacyShims: [
    {
      key: "COPROCESSOR_HOST_LISTENER_VERSION",
      below: [0, 14, 0] as CompatSemver,
      profile: "legacy-host-listener-no-protocol-config-address",
      unparsed: "modern" as const,
    },
    {
      key: "COPROCESSOR_HOST_LISTENER_VERSION",
      below: [0, 14, 0] as CompatSemver,
      profile: "legacy-host-listener-no-confidential-bridge-address",
      unparsed: "modern" as const,
    },
    {
      key: "COPROCESSOR_HOST_LISTENER_VERSION",
      below: [0, 13, 2] as CompatSemver,
      profile: "legacy-host-listener-poller-no-seed-start-block",
      unparsed: "modern" as const,
    },
    {
      key: "COPROCESSOR_SNS_WORKER_VERSION",
      below: [0, 14, 0] as CompatSemver,
      profile: "legacy-sns-worker-no-signer-flags",
      unparsed: "modern" as const,
    },
    {
      key: "COPROCESSOR_SNS_WORKER_VERSION",
      below: [0, 15, 0] as CompatSemver,
      profile: "legacy-sns-worker-split-bucket-flags",
      unparsed: "modern" as const,
    },
  ],
  anchors: {
    SIMPLE_ACL_MIN_SHA: "803f1048727eabf6d8b3df618203e3c7dda77890",
  },
} as const;

export const MODERN_RELAYER_IMAGE_REPOSITORY = "ghcr.io/zama-ai/fhevm/relayer";
export const MODERN_RELAYER_MIGRATE_IMAGE_REPOSITORY = "ghcr.io/zama-ai/fhevm/relayer-migrate";
export const LEGACY_KMS_CORE_IMAGE_REPOSITORY = "ghcr.io/zama-ai/kms/core-service";
/** From 0.15 the cores run the insecure build: only it allows no `[threshold.tls]` config. */
export const MODERN_KMS_CORE_IMAGE_REPOSITORY = "ghcr.io/zama-ai/kms/core-service-insecure";
/** KMS core release whose key material a pre-0.15 coprocessor (tfhe-rs 1.6) can no longer deserialize. */
const KMS_CORE_MATERIAL_BREAK: CompatSemver = [0, 15, 0];

const SHIM_PROFILES = {
  "legacy-host-listener-no-protocol-config-address": {
    coprocessorArgs: {},
    coprocessorDropFlags: {
      "host-listener": ["--protocol-config-address"],
      "host-listener-poller": ["--protocol-config-address"],
      "host-listener-consumer": ["--protocol-config-address"],
    },
    connectorEnv: {},
    composeEnv: {},
  },
  "legacy-host-listener-no-confidential-bridge-address": {
    coprocessorArgs: {},
    coprocessorDropFlags: {
      "host-listener": ["--confidential-bridge-address"],
      "host-listener-poller": ["--confidential-bridge-address"],
      "host-listener-consumer": ["--confidential-bridge-address"],
    },
    connectorEnv: {},
    composeEnv: {},
  },
  // --seed-start-block landed on main and was backported to release/0.13.x
  // (first shipped in v0.13.2-0) and release/0.14.x (first shipped in
  // v0.14.0-4). Build suffixes within a release family are not comparable, so
  // the floor is the 0.13.2 family; 0.14.x builds all resolve as supported.
  "legacy-host-listener-poller-no-seed-start-block": {
    coprocessorArgs: {},
    coprocessorDropFlags: {
      "host-listener-poller": ["--seed-start-block"],
    },
    connectorEnv: {},
    composeEnv: {},
  },
  "legacy-sns-worker-no-signer-flags": {
    coprocessorArgs: {},
    coprocessorDropFlags: {
      "sns-worker": ["--signer-type", "--private-key"],
    },
    connectorEnv: {},
    composeEnv: {},
  },
  // v0.15.0 replaced the split --bucket-name-ct64/--bucket-name-ct128 pair with
  // a single required --bucket-name, since both artifact kinds now live in one
  // bucket under their own key prefixes (#3372). Legacy images still expect both
  // flags, and the standalone `ct64`/`ct128` buckets are gone, so point
  // both at the per-coprocessor bucket.
  //
  // Collapsing two buckets into one is only safe for images that carry #3372,
  // i.e. that key ct128/ct64 as `ct128/{handle}/{ctx}` and `ct64/{handle}/{ctx}`.
  // Before it, BOTH artifacts used the bare `{handle}/{ctx}` key and the separate
  // buckets were the only thing keeping them apart — sharing a bucket would make
  // them overwrite each other. On the 0.14 line #3372 first shipped in v0.14.0-7,
  // which is what the blue-green scenarios pin; earlier 0.14.0-N prereleases must
  // not be used as a BCS baseline against this stack.
  "legacy-sns-worker-split-bucket-flags": {
    coprocessorArgs: {
      "sns-worker": [
        ["--bucket-name-ct128", { env: "BUCKET_NAME" }],
        ["--bucket-name-ct64", { env: "BUCKET_NAME" }],
      ],
    },
    coprocessorDropFlags: {
      "sns-worker": ["--bucket-name"],
    },
    connectorEnv: {},
    composeEnv: {},
  },
} as const satisfies Record<string, CompatPolicy>;

/** Parses a semver-like version string into comparable numeric parts. */
const parseCompatVersion = (version: string) => {
  const match = version.match(/^v?(\d+)\.(\d+)\.(\d+)(?:([-+])(.+))?$/);
  if (!match) {
    return undefined;
  }
  const [, major, minor, patch, suffixType, suffix = ""] = match;
  return {
    parts: [Number(major), Number(minor), Number(patch)] as const,
    prerelease: suffixType === "-" && !/^\d+$/.test(suffix),
    build: suffixType === "-" && /^\d+$/.test(suffix) ? Number(suffix) : undefined,
  };
};

/** Keeps a release command shape when only the registry image tag changes to a commit SHA. */
export const replaceRegistrySourceTag = (
  source: CoprocessorInstanceSource,
  tag: string,
  compatTag?: string,
): CoprocessorInstanceSource => {
  if (parseCompatVersion(tag)) {
    return { mode: "registry", tag };
  }
  const effectiveCompatTag = compatTag ?? (source.mode === "registry"
    ? source.compatTag ?? (parseCompatVersion(source.tag) ? source.tag : undefined)
    : undefined);
  return { mode: "registry", tag, ...(effectiveCompatTag ? { compatTag: effectiveCompatTag } : {}) };
};

const compatVersionGte = (
  version: string,
  target: CompatSemver,
  options?: { unparsed?: "modern" | "legacy" },
) => {
  const parsed = parseCompatVersion(version);
  if (!parsed) {
    return options?.unparsed === "modern";
  }
  for (let index = 0; index < parsed.parts.length; index += 1) {
    if (parsed.parts[index] !== target[index]) {
      return parsed.parts[index] > target[index];
    }
  }
  return !parsed.prerelease;
};

const sameCompatBase = (version: string, target: CompatSemver) => {
  const parsed = parseCompatVersion(version);
  return !!parsed && parsed.parts.every((part, index) => part === target[index]);
};

export const kmsCoreImageRepository = (version: string) =>
  versionBeforeReleaseFamily(version, KMS_CORE_MATERIAL_BREAK)
    ? LEGACY_KMS_CORE_IMAGE_REPOSITORY
    : MODERN_KMS_CORE_IMAGE_REPOSITORY;

/** Detects a KMS core upgrade across the 0.15 material break, which needs an epoch/context migration. */
export const kmsCoreUpgradeCrossesMaterialBreak = (from: string, to: string) =>
  versionBeforeReleaseFamily(from, KMS_CORE_MATERIAL_BREAK) && !versionBeforeReleaseFamily(to, KMS_CORE_MATERIAL_BREAK);

/**
 * A registry-pinned pre-0.15 Blue cannot read material generated by a 0.15+ KMS core, so a
 * blue-green stack must boot at a pre-0.15 `bootstrap.tag` and upgrade after ingest.
 * A SHA pin is judged by its `compatTag`, the release family `--bcs-tag` runs it as.
 */
export const assertBlueGreenKmsCompatibility = (
  scenario: State["scenario"],
  versions: Pick<State["versions"], "env">,
) => {
  if (scenario.kind !== "blue-green" || scenario.bcs.source.mode !== "registry") return;
  const blue = scenario.bcs.source.compatTag ?? scenario.bcs.source.tag;
  const core = versions.env.CORE_VERSION ?? "";
  if (!versionBeforeReleaseFamily(blue, KMS_CORE_MATERIAL_BREAK) || versionBeforeReleaseFamily(core, KMS_CORE_MATERIAL_BREAK)) {
    return;
  }
  throw new Error(
    `Blue ${blue} cannot deserialize key material from KMS core ${core}; set bootstrap.tag in the scenario so the keys are generated by a pre-0.15 stack before it is upgraded.`,
  );
};

/** Compares a version string against a compatibility floor. */
const versionLt = (version: string, target: CompatSemver, options?: { unparsed?: "modern" | "legacy" }) => {
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

export const versionBeforeReleaseFamily = (
  version: string,
  target: CompatSemver,
  options?: { unparsed?: "modern" | "legacy" },
) => versionLt(version, target, options) && !sameCompatBase(version, target);

type CompatState =
  | Pick<State, "versions" | "overrides" | "scenario">
  | Pick<StackSpec, "versions" | "overrides" | "coprocessor">;

/** Computes the effective override set used for compatibility decisions. */
const effectiveCompatOverrides = (state: CompatState) => {
  const scenario = "scenario" in state ? state.scenario : state.coprocessor;
  // Blue-green has no instance-based sources for `effectiveOverrides` to expand.
  if (!scenario || scenario.kind !== "coprocessor-consensus") {
    return state.overrides;
  }
  return effectiveOverrides(state.overrides, scenario);
};

/** Detects when relayer keyurl config must stay on the pre-host-poller static shape. */
export const requiresLegacyRelayerKeyUrlConfig = (state: Pick<CompatState, "versions">) =>
  versionBeforeReleaseFamily(state.versions.env.RELAYER_VERSION ?? "", [0, 14, 0], { unparsed: "modern" });

/**
 * Detects when the test harness must seed host_chains for legacy coprocessor images.
 *
 * Below v0.13.1 the runtime does not reliably seed host_chains before zkproof
 * caches it, so the harness seeds it manually. Declarative seeding was added to
 * initialize_db.sh after v0.13.0 was cut, so all v0.13.0-N Docker builds still
 * need the manual shim; v0.13.1+ images self-seed.
 */
export const requiresLegacyHostChainSeedShim = (state: Pick<CompatState, "versions">) =>
  versionBeforeReleaseFamily(state.versions.env.COPROCESSOR_DB_MIGRATION_VERSION ?? "", [0, 13, 1], {
    unparsed: "modern",
  }) ||
  versionBeforeReleaseFamily(state.versions.env.COPROCESSOR_ZKPROOF_WORKER_VERSION ?? "", [0, 13, 1], {
    unparsed: "modern",
  });

/** Detects when the resolved coprocessor bundle includes the consensus-detector service. */
export const supportsConsensusDetector = (state: Pick<CompatState, "versions">) => {
  const version = state.versions.env.COPROCESSOR_CONSENSUS_DETECTOR_VERSION;
  if (!version) return false;
  return !versionBeforeReleaseFamily(version, [0, 14, 0], { unparsed: "modern" });
};

/** Detects when the resolved coprocessor bundle includes the upgrade-controller service. */
export const supportsUpgradeController = (state: Pick<CompatState, "versions">) => {
  const version = state.versions.env.COPROCESSOR_UPGRADE_CONTROLLER_VERSION;
  if (!version) return false;
  return !versionBeforeReleaseFamily(version, [0, 14, 0], { unparsed: "modern" });
};

/** Detects when the resolved kms-connector bundle includes the HTTP decryption path. */
export const supportsConnectorHttp = (state: Pick<CompatState, "versions" | "overrides">) => {
  const available = (service: string, versionKey: string) =>
    Boolean(state.versions.env[versionKey]) ||
    state.overrides.some(
      (override) =>
        override.group === "kms-connector" && (!override.services?.length || override.services.includes(service)),
    );
  return (
    available("kms-connector-endpoint", "CONNECTOR_ENDPOINT_VERSION") &&
    available("kms-connector-proxy", "CONNECTOR_PROXY_VERSION")
  );
};

/** Detects when the resolved coprocessor bundle includes coprocessor-proxy. */
export const supportsCoprocessorProxy = (state: Pick<CompatState, "versions" | "overrides">) =>
  Boolean(state.versions.env.COPROCESSOR_PROXY_VERSION) ||
  state.overrides.some(
    (override) =>
      override.group === "coprocessor" && (!override.services?.length || override.services.includes("coprocessor-proxy")),
  );

/**
 * Whether a stack runs coprocessor-proxy: its bundle includes the proxy, and the stack is not
 * blue-green, where one proxy per fleet is a follow-up. Takes a state (`scenario`) or a stack spec
 * (`blueGreen`).
 */
export const runsCoprocessorProxy = (
  state: Pick<CompatState, "versions" | "overrides"> & { scenario?: { kind: string }; blueGreen?: unknown },
) => state.scenario?.kind !== "blue-green" && !state.blueGreen && supportsCoprocessorProxy(state);

/** Detects KMS cores whose bootstrap preprocessing needs a much larger wait budget.
 * Pre-v0.13.20 cores can spend more than 20 minutes on two four-party preprocessing
 * sessions; from v0.13.20 the standard threshold budget is enough. */
export const requiresLegacyKmsBootstrapBudget = (coreVersion: string) =>
  versionBeforeReleaseFamily(coreVersion, [0, 13, 20], { unparsed: "modern" });

/**
 * Detects when host images ship canonical ProtocolConfig seeding at all (0.13.1+), so non-canonical
 * chains can seed ProtocolConfig from the canonical chain like production instead of "fresh".
 * This only answers "does the image have the task". Use `canonicalProtocolConfigSeedingUsesEnv` to
 * pick which input the task accepts.
 */
export const supportsCanonicalProtocolConfigSeeding = (state: CompatState) =>
  effectiveCompatOverrides(state).some((override) => override.group === "host-contracts") ||
  !versionLt(state.versions.env.HOST_VERSION ?? "", [0, 13, 1], { unparsed: "modern" });

/**
 * First host build whose canonical seeding tasks read the reviewed snapshot from `CANONICAL_*`
 * environment variables. Every tag from v0.13.1 through v0.14.0-9 declares `--canonical-rpc-url` /
 * `--canonical-protocol-config-address` instead. The floor therefore sits inside the v0.14.0 family.
 * A three-part floor cannot express it. [0, 14, 0] would wrongly claim v0.14.0-0..9 read environment
 * variables. [0, 14, 1] would wrongly claim the release carrying the migration reads command-line
 * flags. The separate build floor below closes that gap. Confirm the build number against the tag
 * that ships the migration when that tag is cut.
 */
const CANONICAL_ENV_SEEDING_FLOOR: CompatSemver = [0, 14, 0];
const CANONICAL_ENV_SEEDING_FLOOR_BUILD = 10;

/**
 * Detects when the host image's canonical seeding tasks take the exported snapshot as `CANONICAL_*`
 * environment variables instead of command-line flags. Older images take the same input as
 * command-line flags. Those images read the canonical chain live over RPC and need no exported
 * snapshot.
 */
export const canonicalProtocolConfigSeedingUsesEnv = (state: CompatState) => {
  if (effectiveCompatOverrides(state).some((override) => override.group === "host-contracts")) {
    return true;
  }
  const version = state.versions.env.HOST_VERSION ?? "";
  const parsed = parseCompatVersion(version);
  // Same base family as the floor, with a numeric `-N` suffix, so compare build numbers.
  if (parsed?.build !== undefined && sameCompatBase(version, CANONICAL_ENV_SEEDING_FLOOR)) {
    return parsed.build >= CANONICAL_ENV_SEEDING_FLOOR_BUILD;
  }
  return compatVersionGte(version, CANONICAL_ENV_SEEDING_FLOOR, { unparsed: "modern" });
};

type BundleIncompatibility = { severity: "error"; code: string; message: string };

/** Evaluates compatibility rules against a resolved bundle. */
export const validateBundleCompatibility = (state: Pick<CompatState, "versions">): BundleIncompatibility[] => {
  const issues: BundleIncompatibility[] = [];
  for (const rule of COMPAT_MATRIX.incompatibilities) {
    const leftVersion = state.versions.env[rule.left.key] ?? "";
    const rightVersion = state.versions.env[rule.right.key] ?? "";
    // Incompatibility rules deliberately use raw `versionLt` (not
    // `versionBeforeReleaseFamily`): a prerelease must count as below its own
    // final tag here, so a `vX.Y.0-rc1` left side still fires a rule whose
    // floor is `vX.Y.0`. The "prerelease counts as older" behavior at the bottom
    // of `versionLt` is pinned by the incompatibility-rule test in compat.test.ts.
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

/**
 * Rejects a multi-node consensus scenario whose nodes would not all run the
 * same coprocessor revision.
 *
 * Byte-consensus compares persisted ciphertext bytes across coprocessors,
 * which is only meaningful when every node runs the same software revision.
 * Two shapes break that and are refused: a service-scoped coprocessor
 * override, which mixes locally built services with published ones; and
 * instances whose effective sources differ (one local, one published, or two
 * different published tags). A single published tag for every node, or a full
 * local build for every node, both satisfy the premise and pass -- the
 * orchestrated CI path runs the published bundle with no override at all.
 */
const assertSingleCoprocessorRevision = (
  state: CompatState,
  scenario: Pick<ResolvedCoprocessorScenario, "instances">,
) => {
  const overrides = effectiveCompatOverrides(state);
  const partial = overrides.find((override) => override.group === "coprocessor" && override.services?.length);
  if (partial) {
    throw new Error(
      "Consensus scenarios need every coprocessor service on every node from one revision; " +
        `a service-scoped coprocessor override (${(partial.services ?? []).join(", ")}) mixes locally built ` +
        "services with published ones. Use --override coprocessor (or --build) for a full local build, or no " +
        "coprocessor override to run the bundle's published images.",
    );
  }
  // An inherited instance builds locally only under an EXPLICIT full
  // coprocessor override; scenario-owned sources (`mode: local` on some
  // instance) never change how a sibling `inherit` instance sources its
  // images, and `assertScenarioOverrideCompatibility` already refuses
  // explicit overrides alongside scenario-owned sources.
  const fullLocal = state.overrides.some((override) => override.group === "coprocessor" && !override.services?.length);
  const bundleTag = state.versions.env.COPROCESSOR_TFHE_WORKER_VERSION ?? "";
  const revisionOf = (source: CoprocessorInstanceSource) => {
    switch (source.mode) {
      case "local":
        return "local";
      case "registry":
        return source.tag === bundleTag ? "bundle" : `registry:${source.tag}`;
      default:
        return fullLocal ? "local" : "bundle";
    }
  };
  const nodesByRevision = new Map<string, number[]>();
  scenario.instances.forEach((instance, position) => {
    const revision = revisionOf(instance.source);
    nodesByRevision.set(revision, [...(nodesByRevision.get(revision) ?? []), instance.index ?? position]);
  });
  if (nodesByRevision.size > 1) {
    const detail = [...nodesByRevision]
      .map(([revision, indexes]) => `${revision} (instance ${indexes.join(", ")})`)
      .join(" vs ");
    throw new Error(
      `Consensus scenarios need every node on one coprocessor revision; instance sources differ: ${detail}. ` +
        "Align the instance sources, or use --override coprocessor to build every node locally.",
    );
  }
};

/** Rejects multi-node consensus scenarios whose nodes would not all run one coprocessor revision. */
export const assertSupportedBundleScenario = (state: CompatState) => {
  const scenario = "scenario" in state ? state.scenario : state.coprocessor;
  if (!scenario) return; // blue-green stack spec — handled by its own compat path
  if (scenario.kind === "coprocessor-consensus" && (scenario.topology?.count ?? 1) > 1) {
    assertSingleCoprocessorRevision(state, scenario);
  }
};

/** Folds one shim profile's coprocessor arg adjustments into an accumulating policy. */
const mergeShimArgs = (policy: CoprocessorArgPolicy, profile: CompatPolicy) => {
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
};

// HTTP was backported in 0.13.5 and after 0.14.1 (#3916, #3923).
// Unversioned images retain the existing current/main compatibility policy.
const transactionSenderGatewayArg = (version: string): readonly [string, CompatArgValue] => {
  const http = compatVersionGte(version, [0, 14, 2], { unparsed: "modern" }) ||
    (compatVersionGte(version, [0, 13, 5]) && versionBeforeReleaseFamily(version, [0, 14, 0]));
  return ["--gateway-url", { env: http ? "GATEWAY_URL" : "GATEWAY_WS_URL" }];
};

/**
 * Builds the coprocessor arg policy for a fleet pinned to one published image tag.
 *
 * `compatPolicyForState` reads the resolved version bundle, i.e. the images the
 * target/lock picked. A scenario instance with `source.mode: registry` ignores
 * that bundle and runs the tag it names instead — blue-green's BCS fleet being
 * the case that matters, since it deliberately runs the *previous* release while
 * the bundle describes HEAD. Its shims therefore have to be evaluated against
 * the pinned tag, or those older binaries are handed the modern flag contract.
 *
 * Only `COPROCESSOR_*` shims are consulted: a pinned coprocessor image tag says
 * nothing about the connector or the contracts.
 */
export const compatArgPolicyForPinnedTag = (tag: string): CoprocessorArgPolicy => {
  const policy: CoprocessorArgPolicy = { coprocessorArgs: {}, coprocessorDropFlags: {} };
  for (const shim of COMPAT_MATRIX.legacyShims) {
    if (!shim.key.startsWith("COPROCESSOR_")) {
      continue;
    }
    if (!versionBeforeReleaseFamily(tag, shim.below, { unparsed: shim.unparsed })) {
      continue;
    }
    mergeShimArgs(policy, SHIM_PROFILES[shim.profile]);
  }
  policy.coprocessorArgs["transaction-sender"] = [
    ...(policy.coprocessorArgs["transaction-sender"] ?? []),
    transactionSenderGatewayArg(tag),
  ];
  return policy;
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
    if (!versionBeforeReleaseFamily(state.versions.env[shim.key] ?? "", shim.below, { unparsed: shim.unparsed })) {
      continue;
    }
    const profile = SHIM_PROFILES[shim.profile];
    mergeShimArgs(policy, profile);
    Object.assign(policy.connectorEnv, profile.connectorEnv);
  }
  policy.coprocessorArgs["transaction-sender"] = [
    ...(policy.coprocessorArgs["transaction-sender"] ?? []),
    transactionSenderGatewayArg(state.versions.env.COPROCESSOR_TX_SENDER_VERSION ?? ""),
  ];
  if (state.versions.env.CORE_VERSION) {
    policy.composeEnv.CORE_IMAGE_REPOSITORY = kmsCoreImageRepository(state.versions.env.CORE_VERSION);
  }
  if (state.versions.env.RELAYER_VERSION) {
    policy.composeEnv.RELAYER_IMAGE_REPOSITORY = MODERN_RELAYER_IMAGE_REPOSITORY;
  }
  if (state.versions.env.RELAYER_MIGRATE_VERSION) {
    policy.composeEnv.RELAYER_MIGRATE_IMAGE_REPOSITORY = MODERN_RELAYER_MIGRATE_IMAGE_REPOSITORY;
  }
  return policy;
};
