import path from "node:path";

import { ensureLockSnapshot } from "../resolve/bundle-store";
import { generateRuntime } from "../generate";
import { requiresMultichainAclAddress } from "../compat/compat";
import { stackSpecForState, topologyForState } from "../stack-spec/stack-spec";
import { PreflightError } from "../errors";
import {
  COMPONENTS,
  GENERATED_CONFIG_DIR,
  gatewayAddressesPath,
  gatewayAddressesSolidityPath,
  paymentBridgingAddressesSolidityPath,
  kmsCoreConfigPath,
  kmsGenKeysConfigPath,
  relayerConfigPath,
  versionsEnvPath,
  envPath,
  composePath,
  hostChainAddressesPath,
  hostChainAddressesSolidityPath,
  isExternalNode,
} from "../layout";
import type { State, StepName } from "../types";
import { exists, readEnvFile } from "../utils/fs";
import { generatedComposeComponents } from "../generate/compose";
import { KMS_THRESHOLD_CONFIG_NAME, kmsThresholdGenKeysConfigName } from "../generate/kms-core";
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
  const plan = stackSpecForState(state);
  const thresholdConfigPaths =
    plan.kms.mode === "threshold"
      ? [
          path.join(GENERATED_CONFIG_DIR, KMS_THRESHOLD_CONFIG_NAME),
          ...Array.from({ length: plan.kms.parties }, (_, index) =>
            path.join(GENERATED_CONFIG_DIR, kmsThresholdGenKeysConfigName(index + 1)),
          ),
        ]
      : [];
  return [
    versionsEnvPath,
    relayerConfigPath,
    kmsCoreConfigPath,
    kmsGenKeysConfigPath,
    ...thresholdConfigPaths,
    ...COMPONENTS.map(envPath),
    ...[...generatedComposeComponents(plan)].map(composePath),
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

/** Regenerates runtime artifacts when persisted state outlives generated files. */
export const ensureRuntimeArtifacts = async (state: State, reason: string) => {
  await ensureLockSnapshot(state.lockPath, state.versions);
  const allExist = (await Promise.all(runtimeArtifactPaths(state).map((file) => exists(file)))).every(Boolean);
  if (allExist) {
    return;
  }
  console.log(`[regen] restoring runtime artifacts for ${reason}`);
  await generateRuntime(state, stackSpecForState(state));
};

/** Returns multi-chain compose file names and their owning step for the current scenario. */
export const multiChainComposeEntries = (state: Pick<State, "scenario">): Array<[string, StepName]> => {
  const entries: Array<[string, StepName]> = [];
  // Externally-provisioned hosts (Solana) have no fhevm-cli node/sc/coprocessor compose, so there
  // is nothing to bring up or tear down here.
  for (const chain of extraHostChains(state).filter((c) => !isExternalNode(c))) {
    const { node, sc, copro } = chain;
    entries.push([node, "base"]);
    entries.push([sc, "host-deploy"]);
    entries.push([copro, "coprocessor"]);
  }
  return entries;
};
