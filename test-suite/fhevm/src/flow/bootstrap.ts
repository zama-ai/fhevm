/**
 * Blue-green `bootstrap`: boot the components that exist before the keys do at a previous
 * release, then upgrade them to the resolved bundle once the operators have ingested the keys.
 */
import { PreflightError } from "../errors";
import { kmsPartyIds } from "../kms-party";
import type { BlueGreenBootstrap, LocalOverride, OverrideGroup, State, VersionBundle } from "../types";

export const CONTRACT_VERSION_KEYS = ["GATEWAY_VERSION", "HOST_VERSION"] as const;
export const LISTENER_CORE_VERSION_KEYS = ["LISTENER_CORE_VERSION"] as const;
export const CONNECTOR_VERSION_KEYS = [
  "CONNECTOR_DB_MIGRATION_VERSION",
  "CONNECTOR_GW_LISTENER_VERSION",
  "CONNECTOR_KMS_WORKER_VERSION",
  "CONNECTOR_TX_SENDER_VERSION",
] as const;
/** Connector services the previous release did not ship; they join with the connector upgrade. */
export const CONNECTOR_OPTIONAL_VERSION_KEYS = ["CONNECTOR_ENDPOINT_VERSION", "CONNECTOR_PROXY_VERSION"] as const;
/** Pinned only when the test runs the upgrade itself; the default bootstrap boots the target relayer. */
export const RELAYER_VERSION_KEYS = ["RELAYER_VERSION", "RELAYER_MIGRATE_VERSION"] as const;

/** Local overrides that would replace a pinned release image; suspended until the upgrade. */
export const bootstrapSuspendedGroups = (bootstrap: BlueGreenBootstrap): readonly OverrideGroup[] => [
  "gateway-contracts",
  "host-contracts",
  "kms-connector",
  "listener-core",
  ...(bootstrap.upgrade === "deferred" ? (["relayer"] as const) : []),
];

/** The bundle with every bootstrapped component pinned to the release. */
export const bootstrapBootVersions = (bundle: VersionBundle, bootstrap: BlueGreenBootstrap): VersionBundle => {
  const env = { ...bundle.env };
  const pinned = [...CONTRACT_VERSION_KEYS, ...LISTENER_CORE_VERSION_KEYS, ...CONNECTOR_VERSION_KEYS, "CORE_VERSION"];
  for (const key of bootstrap.upgrade === "deferred" ? [...pinned, ...RELAYER_VERSION_KEYS] : pinned) {
    env[key] = bootstrap.tag;
  }
  for (const key of CONNECTOR_OPTIONAL_VERSION_KEYS) {
    delete env[key];
  }
  return { ...bundle, env, sources: [...bundle.sources, `bootstrap=${bootstrap.tag}`] };
};

export const bootstrapBootOverrides = (overrides: LocalOverride[], bootstrap: BlueGreenBootstrap) =>
  overrides.filter((override) => !bootstrapSuspendedGroups(bootstrap).includes(override.group));

/** True when the scenario boots at its `bootstrap` release and leaves the upgrade to the test. */
export const isDeferredBootstrap = (scenario: State["scenario"]) =>
  scenario.kind === "blue-green" && scenario.bootstrap?.upgrade === "deferred";

/** One unit of the bootstrap upgrade; a test that defers the upgrade runs them itself. */
export type UpgradeStep = "contracts" | "relayer" | `kms-operator:${number}` | "listener-core" | "green";

/** The production order. The relayer is a step only when deferred; otherwise it boots at the bundle. */
export const upgradeOrder = (scenario: State["scenario"]): UpgradeStep[] => [
  "contracts",
  ...(isDeferredBootstrap(scenario) ? (["relayer"] as const) : []),
  ...kmsPartyIds(scenario.kms.parties).map((id) => `kms-operator:${id}` as const),
  "listener-core",
  "green",
];

/** Rejects the whole list before any step runs, so a typo cannot leave the stack half upgraded. */
export const checkUpgradeSteps = (
  order: readonly string[],
  completed: readonly string[],
  steps: readonly string[],
) => {
  const done = new Set(completed);
  for (const step of steps) {
    if (!order.includes(step)) {
      throw new PreflightError(`unknown upgrade step ${step}; expected one of ${order.join(", ")}`);
    }
    if (done.has(step)) {
      throw new PreflightError(`upgrade step ${step} already ran or is listed twice`);
    }
    if (step.startsWith("kms-operator:") && !done.has("contracts")) {
      throw new PreflightError(`upgrade step ${step} needs the contracts step first`);
    }
    const missing = step === "green" ? order.find((other) => other !== "green" && !done.has(other)) : undefined;
    if (missing) {
      throw new PreflightError(`upgrade step green needs ${missing} first`);
    }
    done.add(step);
  }
};

/** `env` with the `keys` the target defines taken from it. */
export const bootstrapPhaseEnv = (
  env: Record<string, string>,
  target: Record<string, string>,
  keys: readonly string[],
) => ({
  ...env,
  ...Object.fromEntries(keys.filter((key) => target[key] !== undefined).map((key) => [key, target[key]])),
});

/**
 * Upgrades a proxy from the snapshotted previous sources to the current ones, or exits cleanly
 * when the contract's reinitializer did not move: the task would otherwise revert, and the
 * plans below can stay a list of every upgradeable contract rather than a snapshot of one diff.
 */
export const contractUpgradeCommand = (task: string, contract: string) => {
  const reinitializer = (file: string) =>
    `$(grep -m1 -oE 'REINITIALIZER_VERSION = [0-9]+' ${file} | grep -oE '[0-9]+' || true)`;
  return [
    `prev=${reinitializer(`previous-contracts/${contract}.sol`)}`,
    `next=${reinitializer(`contracts/${contract}.sol`)}`,
    `if [ -n "$prev" ] && [ "$prev" = "$next" ]; then`,
    `  echo "[bootstrap] ${contract}: reinitializer $prev unchanged since the snapshot; nothing to upgrade"; exit 0`,
    "fi",
    [
      `npx hardhat ${task}`,
      `--current-implementation previous-contracts/${contract}.sol:${contract}`,
      `--new-implementation contracts/${contract}.sol:${contract}`,
      "--verify-contract false",
      "--use-internal-proxy-address true",
    ].join(" "),
  ].join("\n");
};

/** Every gateway contract with an upgrade task, consumers before the contract they read. */
export const GATEWAY_CONTRACT_UPGRADES = [
  ["task:upgradeDecryption", "Decryption"],
  ["task:upgradeCiphertextCommits", "CiphertextCommits"],
  ["task:upgradeInputVerification", "InputVerification"],
  ["task:upgradeGatewayConfig", "GatewayConfig"],
  ["task:upgradeKMSGeneration", "KMSGeneration"],
] as const;

/** Deployed on the canonical host chain only. */
export const CANONICAL_HOST_CONTRACT_UPGRADES = [["task:upgradeKMSGeneration", "KMSGeneration"]] as const;

/** Every host contract with an upgrade task that is deployed on each host chain. */
export const HOST_CONTRACT_UPGRADES = [
  ["task:upgradeFHEVMExecutor", "FHEVMExecutor"],
  ["task:upgradeACL", "ACL"],
  ["task:upgradeHCULimit", "HCULimit"],
  ["task:upgradeInputVerifier", "InputVerifier"],
  ["task:upgradeKMSVerifier", "KMSVerifier"],
  ["task:upgradeProtocolConfig", "ProtocolConfig"],
] as const;
