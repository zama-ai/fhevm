import { ensureLockSnapshot } from "../resolve/bundle-store";
import { generateRuntime } from "../generate";
import { requiresMultichainAclAddress } from "../compat/compat";
import { stackSpecForState, topologyForState } from "../stack-spec/stack-spec";
import { PreflightError } from "../errors";
import {
  COMPONENTS,
  GROUP_BUILD_SERVICES,
  gatewayAddressesPath,
  gatewayAddressesSolidityPath,
  paymentBridgingAddressesSolidityPath,
  kmsCoreConfigPath,
  relayerConfigPath,
  versionsEnvPath,
  envPath,
  composePath,
  hostChainAddressesPath,
  hostChainAddressesSolidityPath,
} from "../layout";
import type { State, StepName } from "../types";
import { exists, readEnvFile } from "../utils/fs";
import {
  generatedComposeComponents,
  loadMergedComposeDoc,
  localSourceRevision,
} from "../generate/compose";
import { defaultHostChain, extraHostChains } from "./topology";

/** Validates that a generated address file exists and contains the required keys. */
export const ensureGeneratedAddressFile = async (file: string, producer: string, requiredKeys: string[]) => {
  if (!(await exists(file))) {
    throw new PreflightError(`${producer} completed but did not generate ${file}`);
  }
  const env = await readEnvFile(file);
  const missing = requiredKeys.filter((key) => !env[key]);
  if (missing.length) {
    throw new PreflightError(`${producer} completed but ${file} is missing ${missing.join(", ")}`);
  }
};

/** Validates that a generated address file does NOT contain forbidden keys. */
export const assertGeneratedAddressFileLacks = async (
  file: string,
  producer: string,
  forbiddenKeys: string[],
) => {
  if (!(await exists(file))) {
    return;
  }
  const env = await readEnvFile(file);
  const present = forbiddenKeys.filter((key) => env[key]);
  if (present.length) {
    throw new PreflightError(
      `${producer} generated ${file} with forbidden key(s) ${present.join(", ")} — these belong on the canonical host only`,
    );
  }
};

/** Regenerates runtime artifacts when persisted state outlives generated files. */
export const runtimeArtifactPaths = (state: State) => {
  const topology = topologyForState(state);
  const defaultChain = defaultHostChain(state);
  return [
    versionsEnvPath,
    relayerConfigPath,
    kmsCoreConfigPath,
    ...COMPONENTS.map(envPath),
    ...[...generatedComposeComponents(stackSpecForState(state))].map(composePath),
    ...Array.from({ length: Math.max(0, topology.count - 1) }, (_, index) => envPath(`coprocessor.${index + 1}`)),
    ...(state.discovery
      ? [
          gatewayAddressesPath,
          gatewayAddressesSolidityPath,
          paymentBridgingAddressesSolidityPath,
          ...(defaultChain ? [hostChainAddressesPath(defaultChain.key), hostChainAddressesSolidityPath(defaultChain.key)] : []),
        ]
      : []),
    ...extraHostChains(state).flatMap((chain) => {
      const { node, sc, copro } = chain;
      return [
        envPath(node),
        envPath(sc),
        hostChainAddressesPath(chain.key),
        ...(state.discovery ? [hostChainAddressesSolidityPath(chain.key)] : []),
        composePath(node),
        composePath(sc),
        composePath(copro),
        ...Array.from({ length: topology.count }, (_, index) => envPath(`coprocessor-${chain.key}.${index}`)),
      ];
    }),
  ];
};

/**
 * A persisted local KMS runtime override must rebuild from the source revision
 * that generated it. This is especially important for a retry after a Docker
 * build failure: the original compose file may predate the linked-worktree
 * BUILD_ID fallback even though every runtime artifact still exists.
 */
const kmsConnectorBuildRevisionCurrent = async (state: State) => {
  const selectedRuntimeServices = state.overrides
    .filter((override) => override.group === "kms-connector")
    .flatMap((override) =>
      override.services?.length
        ? override.services.filter((service) => !service.endsWith("-db-migration"))
        : GROUP_BUILD_SERVICES["kms-connector"].filter((service) => !service.endsWith("-db-migration")),
    );
  if (!selectedRuntimeServices.length) {
    return true;
  }
  const compose = await loadMergedComposeDoc("kms-connector");
  const expectedBuildId = localSourceRevision();
  return selectedRuntimeServices.every((service) => {
    const build = compose.services[service]?.build;
    const args = build && typeof build === "object" && !Array.isArray(build)
      ? (build as { args?: unknown }).args
      : undefined;
    return Boolean(
      args &&
        typeof args === "object" &&
        !Array.isArray(args) &&
        (args as Record<string, unknown>).BUILD_ID === expectedBuildId,
    );
  });
};

export type RuntimeArtifactOperations = {
  ensureLockSnapshot: typeof ensureLockSnapshot;
  exists: typeof exists;
  kmsConnectorBuildRevisionCurrent: typeof kmsConnectorBuildRevisionCurrent;
  generateRuntime: typeof generateRuntime;
};

const runtimeArtifactOperations: RuntimeArtifactOperations = {
  ensureLockSnapshot,
  exists,
  kmsConnectorBuildRevisionCurrent,
  generateRuntime,
};

/** Regenerates runtime artifacts when persisted state outlives generated files. */
export const ensureRuntimeArtifacts = async (
  state: State,
  reason: string,
  operations: RuntimeArtifactOperations = runtimeArtifactOperations,
) => {
  await operations.ensureLockSnapshot(state.lockPath, state.versions);
  const allExist = (await Promise.all(runtimeArtifactPaths(state).map((file) => operations.exists(file)))).every(Boolean);
  if (allExist && (await operations.kmsConnectorBuildRevisionCurrent(state))) {
    return;
  }
  console.log(`[regen] restoring runtime artifacts for ${reason}`);
  await operations.generateRuntime(state, stackSpecForState(state));
};

/** Returns multi-chain compose file names and their owning step for the current scenario. */
export const multiChainComposeEntries = (state: Pick<State, "scenario">): Array<[string, StepName]> => {
  const entries: Array<[string, StepName]> = [];
  for (const chain of extraHostChains(state)) {
    const { node, sc, copro } = chain;
    entries.push([node, "base"]);
    entries.push([sc, "host-deploy"]);
    entries.push([copro, "coprocessor"]);
  }
  return entries;
};
