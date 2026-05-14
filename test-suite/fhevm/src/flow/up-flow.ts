/**
 * Orchestrates fhevm stack lifecycle commands such as up, down, resume, clean, upgrade, status, and logs.
 */

import path from "node:path";

import { ensureLockSnapshot, previewBundle, resolveBundle } from "../resolve/bundle-store";
import {
  assertSupportedBundleScenario,
  bootstrapUsesHostKmsGeneration,
  requiresGatewayKmsGenerationAddress,
  requiresMultichainAclAddress,
  requiresModernHostAddressArtifacts,
  supportsHostListenerConsumer,
  validateBundleCompatibility,
} from "../compat/compat";
import { driftDatabaseName } from "../drift";
import { serviceNameList } from "../generate/compose";
import { generateRuntime } from "../generate";
import { resolveScenarioForOptions, stackSpecForState, topologyForState } from "../stack-spec/stack-spec";
import { effectiveOverrides, hasLocalCoprocessorInstance, listScenarioSummaries } from "../scenario/resolve";
import { run, runStreaming } from "../utils/process";
import { loadState, markStep, saveState } from "../state/state";
import {
  BootstrapTimeout,
  BuildError,
  ContainerCrashed,
  ContainerStartError,
  GitHubApiError,
  IncompatibleVersions,
  MinioError,
  PreflightError,
  ProbeTimeout,
  ResumeError,
  RpcError,
  SchemaGuardError,
} from "../errors";
import { describeBundle, REPO_KEYS } from "../resolve/target";
import {
  ADDRESS_DIR,
  COMPOSE_OUT_DIR,
  COMPONENT_BY_STEP,
  COMPONENTS,
  COPROCESSOR_DB_CONTAINER,
  DEFAULT_CHAIN_ID,
  CRSGEN_ID_SELECTOR,
  DEFAULT_GATEWAY_RPC_PORT,
  DEFAULT_HOST_RPC_PORT,
  DEFAULT_POSTGRES_PASSWORD,
  DEFAULT_POSTGRES_USER,
  ENV_DIR,
  GENERATED_CONFIG_DIR,
  GROUP_BUILD_COMPONENTS,
  KEYGEN_ID_SELECTOR,
  KMS_CORE_CONTAINER,
  LOCK_DIR,
  LOG_TARGETS,
  MINIO_PORT,
  MINIO_EXTERNAL_URL,
  MINIO_INTERNAL_URL,
  PORTS,
  PROJECT,
  REPO_ROOT,
  SCHEMA_COUPLED_GROUPS,
  STATE_DIR,
  TEST_SUITE_CONTAINER,
  coprocessorHostKey,
  dockerArgs,
  envPath,
  gatewayAddressesPath,
  hostChainAddressesPath,
} from "../layout";
import type {
  BuiltImage,
  CleanOptions,
  Discovery,
  LocalOverride,
  OverrideGroup,
  State,
  StepName,
  UpOptions,
  VersionBundle,
  VersionTarget,
} from "../types";
import { OVERRIDE_GROUPS, STEP_NAMES } from "../types";
import {
  exists,
  hostReachableMaterialUrl,
  hostReachableRpcUrl,
  predictedCrsId,
  predictedKeyId,
  readJson,
  readEnvFile,
  remove,
  withHexPrefix,
  writeJson,
} from "../utils/fs";
import { ensureDiscovery, createDiscovery, defaultEndpoints, discoverContracts, minioIp, validateDiscovery } from "./discovery";
import { defaultHostChain, extraHostChains, hostChainsForState } from "./topology";
import {
  KMS_CONNECTOR_HEALTH_CONTAINERS,
  castBool,
  coprocessorHealthContainers,
  discoverSigner,
  dockerInspect,
  ensureMaterial,
  listenerContainersForChain,
  postBootHealthGate,
  probeBootstrap,
  waitForBootstrap,
  waitForContainer,
  waitForCoprocessor,
  waitForCoprocessorServices,
  waitForKmsConnector,
  waitForLog,
  waitForRpc,
  waitForStableChainListeners,
  waitForTestSuite,
} from "./readiness";
import {
  assertGeneratedAddressFileLacks,
  ensureGeneratedAddressFile,
  ensureRuntimeArtifacts,
  multiChainComposeEntries,
  runtimeArtifactPaths,
} from "./artifacts";
import {
  multiChainCoprocessorUpgradeTargets,
  resolveUpgradePlan,
  resumeRepairStep,
  type UpgradeGroup,
} from "./repair";
import {
  composeDown,
  composeUp,
  inspectImageId,
  maybeBuild,
  multiChainComposeDown,
  multiChainComposeTask,
  multiChainComposeUp,
  projectContainers,
  removeProjectResources,
  resetAfterStep,
  stepComposeTask,
  stepComposeUp,
} from "./runtime-compose";
import { pause, runContractTask, unpause } from "./contracts";

export {
  castBool,
  createDiscovery,
  defaultEndpoints,
  discoverContracts,
  discoverSigner,
  dockerInspect,
  ensureDiscovery,
  ensureMaterial,
  minioIp,
  multiChainCoprocessorUpgradeTargets,
  pause,
  postBootHealthGate,
  probeBootstrap,
  resolveUpgradePlan,
  resumeRepairStep,
  runtimeArtifactPaths,
  unpause,
  validateDiscovery,
  waitForBootstrap,
  waitForContainer,
  waitForLog,
  waitForRpc,
};

const SCHEMA_GUARDS = {
  coprocessor: {
    versionKey: "COPROCESSOR_DB_MIGRATION_VERSION",
    repoPath: "coprocessor/fhevm-engine/db-migration/migrations",
  },
  "kms-connector": {
    versionKey: "CONNECTOR_DB_MIGRATION_VERSION",
    repoPath: "kms-connector/connector-db/migrations",
  },
} as const satisfies Partial<Record<OverrideGroup, { versionKey: string; repoPath: string }>>;

const SCHEMA_GUARD_TARGETS = new Set<VersionBundle["target"]>(["latest-supported", "latest-main", "sha"]);
export const preflightPorts = (state: Pick<State, "scenario">) =>
  [...new Set([...PORTS, ...state.scenario.hostChains.map((chain) => chain.rpcPort)])];

/** Throws if Docker memory is below the scenario minimum: 16 GB standard, 32 GB for multi-chain + multi-coprocessor.
 *  Uses a 1 GB slack to account for VM overhead. */
export const assertDockerMemory = async (scenario: State["scenario"]) => {
  const result = await run(["docker", "info", "--format", "{{.MemTotal}}"], { allowFailure: true });
  if (result.code !== 0) return;
  const memBytes = parseInt(result.stdout.trim(), 10);
  if (isNaN(memBytes)) return;

  const minGb = scenario.hostChains.length > 1 && scenario.topology.count > 1 ? 32 : 16;
  if (memBytes >= (minGb - 1) * 1024 ** 3) return;

  const reportedGb = Math.round((memBytes / 1024 ** 3) * 2) / 2;
  throw new PreflightError(
    `Docker memory is ${reportedGb.toFixed(1)} GB — at least ${minGb} GB required.\nAllocate at least ${minGb} GB of memory to Docker and retry.`,
  );
};
const NETWORK_TARGETS: ReadonlySet<string> = new Set(["devnet", "testnet", "mainnet"]);

const postgresExecOptions = () => ({
  user: process.env.POSTGRES_USER ?? DEFAULT_POSTGRES_USER,
  password: process.env.POSTGRES_PASSWORD ?? DEFAULT_POSTGRES_PASSWORD,
});

const postgresExec = async (dbName: string, args: string[]) => {
  const postgres = postgresExecOptions();
  return run(
    [
      "docker",
      "exec",
      "-e",
      `PGPASSWORD=${postgres.password}`,
      COPROCESSOR_DB_CONTAINER,
      "psql",
      "-U",
      postgres.user,
      "-d",
      dbName,
      ...args,
    ],
    { allowFailure: true },
  );
};

const sqlLiteral = (value: string) => `'${value.replaceAll("'", "''")}'`;

/** Logs elapsed time for one stack subtask. */
const timed = async <T>(label: string, task: () => Promise<T>) => {
  const started = Date.now();
  const result = await task();
  console.log(`${label} done (${Math.round((Date.now() - started) / 1000)}s)`);
  return result;
};

/** Returns the pipeline index for a named stack step. */
const stateStepIndex = (step: StepName) => STEP_NAMES.indexOf(step);

/** Formats one override entry for plan and resume logs. */
const describeOverride = (item: { group: string; services?: string[] }) =>
  `${item.group}${item.services?.length ? `[${item.services.join(",")}]` : ""}`;

/** Computes the user-visible override set after scenario expansion. */
const visibleOverrides = (state: Pick<State, "overrides" | "scenario">) =>
  effectiveOverrides(state.overrides, state.scenario);

/** Builds human-readable warnings for risky override combinations. */
const overrideWarnings = (overrides: LocalOverride[], target?: string) => {
  const warnings = overrides.flatMap((item) =>
    item.services?.length && SCHEMA_COUPLED_GROUPS.includes(item.group)
      ? [
          `${item.group}: per-service override with a shared database. If your changes include DB migrations, non-overridden services may fail. Use --override ${item.group} (full group) in that case.`,
        ]
      : [],
  );
  if (target && NETWORK_TARGETS.has(target) && overrides.length) {
    warnings.push(
      `Overriding on network target '${target}': ensure your local code is compatible with ${target}'s DB schema, contract interfaces, and service versions.`,
    );
  }
  return warnings;
};

/** Prints the resolved version bundle in compact or detailed form. */
const fullBuildActive = (overrides: LocalOverride[]) =>
  OVERRIDE_GROUPS.every((group) => overrides.some((item) => item.group === group));

/** Rewrites displayed repo-owned versions to match the effective runtime source under `--build`. */
export const displayedBundle = (
  bundle: VersionBundle,
  overrides: LocalOverride[],
) =>
  !fullBuildActive(overrides)
    ? bundle
    : {
        ...bundle,
        env: Object.fromEntries(
          Object.entries(bundle.env).map(([key, value]) => [key, REPO_KEYS.has(key) ? "LOCAL BUILD" : value]),
        ),
      };

/** Prints the resolved version bundle in compact or detailed form. */
const printBundle = (state: Pick<State, "versions" | "overrides">, options?: { detailed?: boolean }) => {
  const bundle = displayedBundle(state.versions, state.overrides);
  console.log(`[resolve] ${bundle.lockName}`);
  if (options?.detailed) {
    console.log(describeBundle(bundle));
  }
};

/** Prints the resolved execution plan for `up` or `up --dry-run`. */
const printPlan = (state: Pick<State, "target" | "overrides" | "scenario">, fromStep?: StepName) => {
  const topology = topologyForState(state);
  const overrides = visibleOverrides(state);
  const localTestSuite = overrides.some((item) => item.group === "test-suite");
  const isMultiChain = state.scenario.hostChains.length > 1;
  const topologyLabel = `${topology.count} coprocessor${topology.count === 1 ? "" : "s"}, threshold ${topology.threshold}`;
  console.log(`[plan] profile=${state.target}`);
  if (overrides.length) {
    console.log(`[plan] overrides=${overrides.map(describeOverride).join(", ")}`);
    for (const warning of overrideWarnings(overrides, state.target)) {
      console.log(`[warn] ${warning}`);
    }
  }
  console.log(`[plan] test-suite=${localTestSuite ? "local workspace image" : "published image"}`);
  if (!localTestSuite) {
    console.log("[plan] local e2e test changes require --override test-suite or --build");
  }
  console.log(`[plan] topology=n${topology.count}/t${topology.threshold} (${topologyLabel})${isMultiChain ? " multi-chain" : ""}`);
  console.log(`[plan] steps=${STEP_NAMES.slice(stateStepIndex(fromStep ?? STEP_NAMES[0])).join(" -> ")}`);
};

/** Quotes a shell argument for safe inclusion in a `sh -lc` command. */
export const shellEscape = (value: string) => `'${value.replaceAll("'", `'\\''`)}'`;

/** Filters overrides that may diverge from a shared database schema. */
const partialSchemaOverrides = (overrides: LocalOverride[]) =>
  overrides.filter(
    (item): item is LocalOverride & { services: string[] } =>
      !!item.services?.length && SCHEMA_COUPLED_GROUPS.includes(item.group),
  );


/** Verifies required local tooling and port availability before boot. */
export const preflight = async (state: State, strictPorts = true, needsGitHub = true) => {
  const requiredCommands = ["bun", "docker", "cast", ...(needsGitHub ? ["gh"] : [])];
  const whichResults = await Promise.all(
    requiredCommands.map(async (command) => {
      try {
        const result = await run(["which", command], { allowFailure: true });
        return { command, found: result.code === 0 };
      } catch {
        return { command, found: false };
      }
    }),
  );
  for (const { command, found } of whichResults) {
    if (!found) {
      throw new PreflightError(`Required command "${command}" not found`);
    }
  }
  const projectPorts = await run(
    ["docker", "ps", "--filter", `label=com.docker.compose.project=${PROJECT}`, "--format", "{{.Ports}}"],
    { allowFailure: true },
  );
  if (projectPorts.code !== 0) {
    throw new PreflightError(projectPorts.stderr.trim() || "docker ps failed");
  }
  const portResults = await Promise.all(
    preflightPorts(state).map(async (port) => {
      try {
        return {
          port,
          busy: await run(["lsof", "-nP", `-iTCP:${port}`, "-sTCP:LISTEN"], { allowFailure: true }),
        };
      } catch {
        return { port, busy: { stdout: "", stderr: "", code: 1 } };
      }
    }),
  );
  for (const { port, busy } of portResults) {
    if (busy.code === 0 && busy.stdout.trim() && !projectPorts.stdout.includes(`:${port}->`)) {
      const message = `Port ${port} is already in use\n${busy.stdout}`;
      if (strictPorts) {
        throw new PreflightError(message);
      }
      console.log(`[preflight] warning: ${message}`);
    }
  }
};

const assertSchemaRepoStable = async (
  group: OverrideGroup,
  bundle: VersionBundle,
  missingRefMessage: (ref: string) => string,
  mismatchMessage: (ref: string) => string,
) => {
  const guard = SCHEMA_GUARDS[group as keyof typeof SCHEMA_GUARDS];
  if (!guard || !SCHEMA_GUARD_TARGETS.has(bundle.target)) {
    return;
  }
  const ref = bundle.env[guard.versionKey];
  if (!ref) {
    return;
  }
  const verified = await run(["git", "rev-parse", "-q", "--verify", `${ref}^{commit}`], {
    cwd: REPO_ROOT,
    allowFailure: true,
  });
  if (verified.code !== 0) {
    throw new SchemaGuardError(group, missingRefMessage(ref));
  }
  const untracked = await run(
    ["git", "ls-files", "--others", "--exclude-standard", "--", guard.repoPath],
    { cwd: REPO_ROOT, allowFailure: true },
  );
  if (untracked.code !== 0) {
    throw new SchemaGuardError(group, `Failed to inspect local ${group} migrations`);
  }
  if (untracked.stdout.trim()) {
    throw new SchemaGuardError(group, mismatchMessage(ref));
  }
  const diff = await run(["git", "diff", "--quiet", "--exit-code", ref, "--", guard.repoPath], {
    cwd: REPO_ROOT,
    allowFailure: true,
  });
  if (diff.code === 1) {
    throw new SchemaGuardError(group, mismatchMessage(ref));
  }
  if (diff.code !== 0 && diff.code !== 1) {
    throw new SchemaGuardError(group, `Failed to compare local ${group} migrations against ${ref}`);
  }
};

const assertSchemaCompatibility = async (
  bundle: VersionBundle,
  overrides: LocalOverride[],
  scenario: State["scenario"],
  allowSchemaMismatch: boolean,
) => {
  if (allowSchemaMismatch || !SCHEMA_GUARD_TARGETS.has(bundle.target)) {
    return;
  }
  for (const item of partialSchemaOverrides(effectiveOverrides(overrides, scenario))) {
    await assertSchemaRepoStable(
      item.group,
      bundle,
      (ref) => `Cannot compare local ${item.group} migrations against ${ref}; local git ref is missing. Run \`git fetch --tags\` or pass --allow-schema-mismatch.`,
      (ref) =>
        `${item.group}: local DB migrations diverge from ${ref}. Use --override ${item.group} or pass --allow-schema-mismatch if you know this service remains compatible.`,
    );
  }
};

/** Checks whether the coprocessor database already contains seeded runtime data. */
const coprocessorDbSeeded = async (database: string) => {
  const result = await postgresExec(database, ["-tAc", "select 1 from host_chains limit 1"]);
  return result.code === 0 && result.stdout.trim() === "1";
};

const coprocessorDbsSeeded = async (state: Pick<State, "scenario">) =>
  (await Promise.all(
    Array.from({ length: topologyForState(state).count }, (_, index) => driftDatabaseName(index)).map(coprocessorDbSeeded),
  )).every(Boolean);

/** Registers an extra host chain in all coprocessor databases and restarts zkproof-workers. */
const registerExtraChainInCoprocessor = async (state: State, chain: { key: string; chainId: string }) => {
  const plan = stackSpecForState(state);
  const chainHost = state.discovery?.hosts[chain.key] ?? {};
  const aclAddress = chainHost.ACL_CONTRACT_ADDRESS ?? "";
  for (let index = 0; index < plan.topology.count; index += 1) {
    const dbName = driftDatabaseName(index);
    const prefix = index === 0 ? "coprocessor-" : `coprocessor${index}-`;
    console.log(`[multi-chain] registering ${chain.key} in ${dbName}`);
    const result = await postgresExec(dbName, [
      "-c",
      `INSERT INTO host_chains (chain_id, name, acl_contract_address) VALUES (${chain.chainId}, ${sqlLiteral(chain.key)}, ${sqlLiteral(aclAddress)}) ON CONFLICT (chain_id) DO NOTHING;`,
    ]);
    if (result.code !== 0) {
      throw new PreflightError(result.stderr.trim() || result.stdout.trim() || `failed to register ${chain.key} in ${dbName}`);
    }
    console.log(`[multi-chain] restarting ${prefix}zkproof-worker`);
    await run(["docker", "stop", `${prefix}zkproof-worker`]);
    await run(["docker", "start", `${prefix}zkproof-worker`]);
    await waitForContainer(`${prefix}zkproof-worker`, "running");
  }
};


export const runStep = async (state: State, step: StepName) => {
  const stepIndex = stateStepIndex(step) + 1;
  console.log(`[step ${stepIndex}/${STEP_NAMES.length}] ${step}`);
  const stepStarted = Date.now();

  switch (step) {
    case "preflight":
      await preflight(state, true, state.requiresGitHub ?? true);
      break;
    case "resolve":
      printBundle(state, { detailed: true });
      break;
    case "generate":
      await generateRuntime(state, stackSpecForState(state));
      break;
    case "base": {
      await run(
        ["docker", "network", "create", "--label", `com.docker.compose.project=${PROJECT}`, "--label", "com.docker.compose.network=default", `${PROJECT}_default`],
        { allowFailure: true },
      );
      await stepComposeUp("minio", state);
      await waitForContainer("fhevm-minio", "healthy");
      await waitForContainer("fhevm-minio-setup", "complete");
      await stepComposeUp("core", state);
      await waitForLog(KMS_CORE_CONTAINER, /KMS Server service socket address/);
      await stepComposeUp("database", state);
      await waitForContainer(COPROCESSOR_DB_CONTAINER, "healthy");
      const defaultChain = defaultHostChain(state);
      await stepComposeUp("host-node", state);
      if (!defaultChain) {
        throw new PreflightError("Missing default host chain");
      }
      await waitForRpc(`http://localhost:${defaultChain.rpcPort}`);
      // Fund the kms-connector tx-sender on the host chain. The wallet is derived from the gateway
      // mnemonic so anvil pre-funds it there, but not on the host chain (different mnemonic).
      const kmsConnectorEnv = await readEnvFile(envPath("kms-connector"));
      const txSenderKey = kmsConnectorEnv.KMS_CONNECTOR_PRIVATE_KEY;
      if (txSenderKey) {
        const txSenderAddress = (await run(["cast", "wallet", "address", txSenderKey])).stdout.trim();
        await run([
          "cast", "rpc", "anvil_setBalance", txSenderAddress, "0x56BC75E2D63100000", // 100 ETH
          "--rpc-url", `http://localhost:${defaultChain.rpcPort}`,
        ]);
      }
      await stepComposeUp("gateway-node", state);
      await waitForRpc(`http://localhost:${DEFAULT_GATEWAY_RPC_PORT}`);
      const plan = stackSpecForState(state);
      const endpoints = await defaultEndpoints();
      endpoints.hosts[defaultChain.key] = {
        http: `http://${defaultChain.node}:${defaultChain.rpcPort}`,
        ws: `ws://${defaultChain.node}:${defaultChain.rpcPort}`,
      };
      state.discovery = createDiscovery(endpoints);
      for (const chain of extraHostChains(state)) {
        state.discovery.endpoints.hosts[chain.key] = {
          http: `http://${chain.node}:${chain.rpcPort}`,
          ws: `ws://${chain.node}:${chain.rpcPort}`,
        };
      }
      await generateRuntime(state, stackSpecForState(state));
      for (const chain of extraHostChains(state)) {
        await multiChainComposeUp(chain.node);
        await waitForRpc(`http://localhost:${chain.rpcPort}`);
      }
      break;
    }
    case "kms-signer": {
      const discovery = await ensureDiscovery(state);
      const signer = await discoverSigner();
      discovery.kmsSigner = signer.address;
      discovery.minioKeyPrefix = signer.minioKeyPrefix;
      await generateRuntime(state, stackSpecForState(state));
      break;
    }
    case "gateway-deploy":
      await stepComposeTask("gateway-mocked-payment", state, ["gateway-deploy-mocked-zama-oft"]);
      await waitForContainer("gateway-deploy-mocked-zama-oft", "complete");
      await stepComposeTask("gateway-sc", state, ["gateway-sc-deploy"]);
      await waitForContainer("gateway-sc-deploy", "complete");
      await ensureGeneratedAddressFile(gatewayAddressesPath, "gateway-sc-deploy", [
        "GATEWAY_CONFIG_ADDRESS",
        "INPUT_VERIFICATION_ADDRESS",
        "CIPHERTEXT_COMMITS_ADDRESS",
        "DECRYPTION_ADDRESS",
        ...(requiresGatewayKmsGenerationAddress(state) ? ["KMS_GENERATION_ADDRESS"] : []),
      ]);
      (await ensureDiscovery(state)).gateway = await readEnvFile(gatewayAddressesPath);
      await generateRuntime(state, stackSpecForState(state));
      await stepComposeTask("gateway-mocked-payment", state, ["gateway-set-relayer-mocked-payment"], { noDeps: true });
      await waitForContainer("gateway-set-relayer-mocked-payment", "complete");
      break;
    case "host-deploy":
      if (!defaultHostChain(state)) {
        throw new PreflightError("Missing default host chain");
      }
      await stepComposeTask("host-sc", state, ["host-sc-deploy"]);
      await waitForContainer("host-sc-deploy", "complete");
      await ensureGeneratedAddressFile(hostChainAddressesPath(defaultHostChain(state)!.key), "host-sc-deploy", [
        "ACL_CONTRACT_ADDRESS",
        "FHEVM_EXECUTOR_CONTRACT_ADDRESS",
        "KMS_VERIFIER_CONTRACT_ADDRESS",
        "INPUT_VERIFIER_CONTRACT_ADDRESS",
        "HCU_LIMIT_CONTRACT_ADDRESS",
        ...(requiresModernHostAddressArtifacts(state) ? ["PROTOCOL_CONFIG_CONTRACT_ADDRESS", "KMS_GENERATION_CONTRACT_ADDRESS"] : []),
      ]);
      for (const chain of extraHostChains(state)) {
        const scKey = chain.sc;
        await timed(`[multi-chain] ${scKey}-deploy`, async () => {
          await multiChainComposeTask(scKey, [`${scKey}-deploy`]);
          await waitForContainer(`${scKey}-deploy`, "complete");
          await ensureGeneratedAddressFile(hostChainAddressesPath(chain.key), `${scKey}-deploy`, [
            "ACL_CONTRACT_ADDRESS",
            "FHEVM_EXECUTOR_CONTRACT_ADDRESS",
            "KMS_VERIFIER_CONTRACT_ADDRESS",
            "INPUT_VERIFIER_CONTRACT_ADDRESS",
            "HCU_LIMIT_CONTRACT_ADDRESS",
            ...(requiresModernHostAddressArtifacts(state) ? ["PROTOCOL_CONFIG_CONTRACT_ADDRESS"] : []),
          ]);
          await assertGeneratedAddressFileLacks(hostChainAddressesPath(chain.key), `${scKey}-deploy`, [
            "KMS_GENERATION_CONTRACT_ADDRESS",
          ]);
        });
        await timed(`[multi-chain] ${scKey}-add-pausers`, async () => {
          await multiChainComposeTask(scKey, [`${scKey}-add-pausers`]);
          await waitForContainer(`${scKey}-add-pausers`, "complete");
        });
      }
      break;
    case "discover": {
      const contracts = await discoverContracts(state);
      const discovery = await ensureDiscovery(state);
      discovery.gateway = contracts.gateway;
      discovery.hosts = { ...discovery.hosts, ...contracts.hosts };
      break;
    }
    case "regenerate":
      await generateRuntime(state, stackSpecForState(state));
      break;
    case "validate": {
      validateDiscovery(state);
      const incompatibilities = validateBundleCompatibility(state);
      if (incompatibilities.length) {
        throw new IncompatibleVersions(incompatibilities.map((item) => item.message));
      }
      break;
    }
    case "listener-core":
      if (!supportsHostListenerConsumer(state)) {
        break;
      }
      await postgresExec("", ["-c", "CREATE DATABASE listener;"]);
      await stepComposeUp("listener-core", state,
        ["listener-redis"]
      );
      await waitForContainer("listener-redis", "running");
      await stepComposeUp("listener-core", state,
        ["listener-publisher-for-anvil"]
      );
      await waitForContainer("listener-publisher-for-anvil", "running");
      break;
    case "coprocessor": {
      const skipMigration = await coprocessorDbsSeeded(state);
      const services = skipMigration ? coprocessorHealthContainers(state) : serviceNameList(state, "coprocessor");
      await stepComposeUp("coprocessor", state, services, { noDeps: skipMigration });
      await waitForCoprocessorServices(state, skipMigration);
      await postBootHealthGate(coprocessorHealthContainers(state));
      for (const chain of extraHostChains(state)) {
        const suffix = chain.suffix;
        await timed(`[multi-chain] register ${chain.key} in coprocessor DBs`, () =>
          registerExtraChainInCoprocessor(state, chain),
        );
        await timed(`[multi-chain] start host-listener${suffix} services`, async () => {
          await multiChainComposeUp(coprocessorHostKey(chain.key));
          await waitForStableChainListeners(state, chain.key);
        });
      }
      break;
    }
    case "kms-connector":
      await stepComposeUp("kms-connector", state);
      await waitForKmsConnector();
      await postBootHealthGate(KMS_CONNECTOR_HEALTH_CONTAINERS);
      break;
    case "bootstrap": {
      await ensureRuntimeArtifacts(state, "bootstrap");
      const bootstrapDone = await probeBootstrap(state).catch((error) => (error instanceof MinioError ? null : Promise.reject(error)));
      if (bootstrapDone) {
        state.discovery!.actualFheKeyId = bootstrapDone.actualFheKeyId;
        state.discovery!.actualCrsKeyId = bootstrapDone.actualCrsKeyId;
        await generateRuntime(state, stackSpecForState(state));
        break;
      }
      const [hostEnv, gatewayEnv] = await Promise.all([readEnvFile(envPath("host-sc")), readEnvFile(envPath("gateway-sc"))]);
      const hostChainsRegistered = (
        await Promise.all(
          Array.from({ length: Number(gatewayEnv.NUM_HOST_CHAINS ?? "0") }, (_, index) => gatewayEnv[`HOST_CHAIN_CHAIN_ID_${index}`])
            .filter(Boolean)
            .map((chainId) =>
              castBool(
                state.discovery!.endpoints.gateway.http,
                withHexPrefix(state.discovery!.gateway.GATEWAY_CONFIG_ADDRESS),
                "isHostChainRegistered(uint256)(bool)",
                chainId,
              ).catch(() => false),
            ),
        )
      ).every(Boolean);
      if (!hostChainsRegistered) {
        await timed("[bootstrap] add-network", () =>
          stepComposeTask("gateway-sc", state, ["gateway-sc-add-network"], { noDeps: true }),
        );
        await waitForContainer("gateway-sc-add-network", "complete");
      }
      const bootstrapReady = await probeBootstrap(state).catch((error) => (error instanceof MinioError ? null : Promise.reject(error)));
      if (bootstrapReady) {
        state.discovery!.actualFheKeyId = bootstrapReady.actualFheKeyId;
        state.discovery!.actualCrsKeyId = bootstrapReady.actualCrsKeyId;
        await generateRuntime(state, stackSpecForState(state));
        break;
      }
      const bootstrapHost = defaultHostChain(state);
      if (!bootstrapHost) {
        throw new PreflightError("Missing default host chain");
      }
      const hostPauserRegistered = await castBool(
        state.discovery!.endpoints.hosts[bootstrapHost.key].http,
        withHexPrefix(state.discovery!.hosts[bootstrapHost.key].PAUSER_SET_CONTRACT_ADDRESS),
        "isPauser(address)(bool)",
        withHexPrefix(hostEnv.PAUSER_ADDRESS_0),
      ).catch(() => false);
      if (!hostPauserRegistered) {
        await timed("[bootstrap] add-host-pausers", () =>
          stepComposeTask("host-sc", state, ["host-sc-add-pausers"], { noDeps: true }),
        );
        await waitForContainer("host-sc-add-pausers", "complete");
      }
      const gatewayPauserRegistered = await castBool(
        state.discovery!.endpoints.gateway.http,
        withHexPrefix(gatewayEnv.PAUSER_SET_ADDRESS),
        "isPauser(address)(bool)",
        withHexPrefix(gatewayEnv.PAUSER_ADDRESS_0),
      ).catch(() => false);
      if (!gatewayPauserRegistered) {
        await timed("[bootstrap] add-gateway-pausers", () =>
          stepComposeTask("gateway-sc", state, ["gateway-sc-add-pausers"], { noDeps: true }),
        );
        await waitForContainer("gateway-sc-add-pausers", "complete");
      }
      if (bootstrapUsesHostKmsGeneration(state)) {
        await timed("[bootstrap] trigger-keygen", () =>
          stepComposeTask("host-sc", state, ["host-sc-trigger-keygen"], { noDeps: true }),
        );
        await waitForContainer("host-sc-trigger-keygen", "complete");
        await timed("[bootstrap] trigger-crsgen", () =>
          stepComposeTask("host-sc", state, ["host-sc-trigger-crsgen"], { noDeps: true }),
        );
        await waitForContainer("host-sc-trigger-crsgen", "complete");
      } else {
        await timed("[bootstrap] trigger-keygen", () =>
          stepComposeTask("gateway-sc", state, ["gateway-sc-trigger-keygen"], { noDeps: true }),
        );
        await waitForContainer("gateway-sc-trigger-keygen", "complete");
        await timed("[bootstrap] trigger-crsgen", () =>
          stepComposeTask("gateway-sc", state, ["gateway-sc-trigger-crsgen"], { noDeps: true }),
        );
        await waitForContainer("gateway-sc-trigger-crsgen", "complete");
      }
      await timed("[bootstrap] wait-for-materials", () => waitForBootstrap(state));
      await generateRuntime(state, stackSpecForState(state));
      break;
    }
    case "relayer":
      await stepComposeUp("relayer", state);
      await waitForContainer("fhevm-relayer-db", "healthy");
      await waitForContainer("fhevm-relayer", "running");
      await waitForLog("fhevm-relayer", /All servers are ready and responding/);
      break;
    case "test-suite":
      await stepComposeUp("test-suite", state);
      await waitForTestSuite();
      break;
  }

  await markStep(state, step);
  console.log(`[step ${stepIndex}/${STEP_NAMES.length}] ${step} done (${Math.round((Date.now() - stepStarted) / 1000)}s)`);
};

/** Summarizes persisted stack state for resume-related error messages. */
const describeResumeState = (state: State) => {
  const topology = topologyForState(state);
  const overrides = visibleOverrides(state);
  return [
    `profile=${state.target}`,
    `topology=${topology.count}/${topology.threshold}`,
    ...(state.scenario.hostChains.length > 1 ? ["multi-chain"] : []),
    ...(state.scenario.origin !== "default"
      ? [`scenario=${state.scenario.origin}${state.scenario.sourcePath ? `:${state.scenario.sourcePath}` : ""}`]
      : []),
    ...(overrides.length ? [`overrides=${overrides.map(describeOverride).join(", ")}`] : []),
  ].join(" ");
};

/** Lists the explicit options that conflict with resuming persisted state. */
export const resumeOptionConflicts = (
  state: State,
  options: Pick<
    UpOptions,
    "requestedTarget" | "sha" | "lockFile" | "scenarioPath" | "overrides" | "allowSchemaMismatch" | "reset"
  >,
) => {
  const mismatches: string[] = [];
  if (options.requestedTarget) mismatches.push(`target=${options.requestedTarget}`);
  if (options.sha) mismatches.push(`sha=${options.sha}`);
  if (options.lockFile) mismatches.push(`lock-file=${options.lockFile}`);
  if (options.scenarioPath) mismatches.push(`scenario=${options.scenarioPath}`);
  if (options.overrides.length) mismatches.push(`overrides=${options.overrides.map(describeOverride).join(", ")}`);
  if (options.allowSchemaMismatch) mismatches.push("--allow-schema-mismatch");
  if (options.reset) mismatches.push("--reset");
  return mismatches;
};

/** Rejects fresh-stack options when `--resume` is requested. */
const ensureResumeOptions = (
  state: State,
  options: Pick<
    UpOptions,
    "requestedTarget" | "sha" | "lockFile" | "scenarioPath" | "overrides" | "allowSchemaMismatch" | "reset"
  >,
) => {
  const mismatches = resumeOptionConflicts(state, options);
  if (mismatches.length) {
    throw new ResumeError(
      `--resume uses the persisted stack configuration; remove ${mismatches.join(", ")} or start a fresh stack. Persisted state: ${describeResumeState(state)}`,
    );
  }
};

/** Chooses the first pipeline step that should run for the current boot mode. */
const startStep = (state: State, options: Pick<UpOptions, "resume" | "fromStep">): StepName => {
  if (options.fromStep) {
    return options.fromStep;
  }
  if (!options.resume || !state.completedSteps.length) {
    return STEP_NAMES[0];
  }
  return STEP_NAMES.find((step) => !state.completedSteps.includes(step)) ?? STEP_NAMES[STEP_NAMES.length - 1];
};

/** Determines whether the chosen target requires live GitHub resolution. */
const targetNeedsGitHub = (options: Pick<UpOptions, "target" | "lockFile">) =>
  !options.lockFile && options.target !== "latest-supported";

/** Rejects topology combinations that are not yet supported for a selected target. */
const assertSupportedTargetScenario = (target: VersionTarget, scenario: State["scenario"]) => {
  if (NETWORK_TARGETS.has(target) && scenario.hostChains.length > 1) {
    throw new PreflightError(
      `--target ${target} does not currently support multi-chain scenarios; rerun without --scenario multi-chain or use latest-main`,
    );
  }
};

/** Builds a synthetic state object for dry-run previews. */
export const previewStateFromBundle = (
  options: Pick<UpOptions, "overrides" | "lockFile">,
  bundle: VersionBundle,
  scenario: State["scenario"],
): State => {
  assertSupportedTargetScenario(bundle.target, scenario);
  assertSupportedBundleScenario({ versions: bundle, overrides: options.overrides, scenario });
  return {
    target: bundle.target,
    lockPath: "",
    requiresGitHub: targetNeedsGitHub({ target: bundle.target, lockFile: options.lockFile }),
    versions: bundle,
    overrides: options.overrides,
    scenario,
    scenarioSourcePath: scenario.sourcePath,
    completedSteps: [],
    updatedAt: new Date().toISOString(),
  };
};

const bootstrapState = async (options: UpOptions) => {
  console.log(`[up] target=${options.target}`);
  const scenario = await resolveScenarioForOptions(options);
  await assertDockerMemory(scenario);
  const resolveStarted = Date.now();
  const resolved = await resolveBundle(options, process.env);
  console.log(`[resolve] bundle ready (${Math.round((Date.now() - resolveStarted) / 1000)}s)`);
  assertSupportedTargetScenario(resolved.bundle.target, scenario);
  assertSupportedBundleScenario({ versions: resolved.bundle, overrides: options.overrides, scenario });
  await assertSchemaCompatibility(resolved.bundle, options.overrides, scenario, options.allowSchemaMismatch);
  await ensureLockSnapshot(resolved.lockPath, resolved.bundle);
  return {
    target: resolved.bundle.target,
    lockPath: resolved.lockPath,
    requiresGitHub: targetNeedsGitHub({ target: resolved.bundle.target, lockFile: options.lockFile }),
    versions: resolved.bundle,
    overrides: options.overrides,
    scenario,
    scenarioSourcePath: scenario.sourcePath,
    completedSteps: [],
    updatedAt: new Date().toISOString(),
  } satisfies State;
};

/** Boots or resumes the stack and runs the remaining pipeline steps. */
export const up = async (options: UpOptions) => {
  const started = Date.now();
  const persistedState = await loadState();
  let state = options.resume ? persistedState : undefined;
  let fromStep = options.fromStep;
  if (options.resume && !state) {
    throw new ResumeError("No .fhevm/state/state.json to resume from");
  }
  if (!state) {
    const nextState = await bootstrapState(options);
    if (persistedState || (await projectContainers(true)).length) {
      console.log("[up] cleaning previous run");
      await down();
    }
    state = nextState;
    await saveState(state);
  }
  if (options.resume) {
    state.requiresGitHub = false;
    state.scenarioSourcePath ??= state.scenario?.sourcePath;
    ensureResumeOptions(state, options);
    await ensureRuntimeArtifacts(state, "resume");
    const running = await projectContainers();
    if (!running.length) {
      console.log(
        fromStep
          ? `[resume] stack is stopped; ignoring --from-step ${fromStep} and restarting from base`
          : "[resume] stack is stopped; restarting from base",
      );
      state.completedSteps = [];
      fromStep = undefined;
      await saveState(state);
    } else if (!fromStep) {
      const names = await projectContainers(true);
      const inspected = await Promise.all(names.map(async (name) => (await dockerInspect(name))[0]));
      const repairFrom = resumeRepairStep(
        state,
        new Map(
          names.map((name, index) => [
            name,
            {
              status: inspected[index]?.State.Status ?? "missing",
              health: inspected[index]?.State.Health?.Status,
            },
          ]),
        ),
      );
      if (repairFrom) {
        console.log(`[resume] detected degraded stack; repairing from ${repairFrom}`);
        state.completedSteps = state.completedSteps.filter((step) => stateStepIndex(step) < stateStepIndex(repairFrom));
        await saveState(state);
      } else if (STEP_NAMES.every((step) => state!.completedSteps.includes(step))) {
        console.log("[resume] nothing to do");
        return;
      }
    }
  }
  for (const warning of overrideWarnings(visibleOverrides(state), state.target)) {
    console.log(`[warn] ${warning}`);
  }
  if (options.resume && fromStep) {
    await resetAfterStep(fromStep, STEP_NAMES, COMPONENT_BY_STEP, stateStepIndex, state);
    state.completedSteps = state.completedSteps.filter((step) => stateStepIndex(step) < stateStepIndex(fromStep));
    await saveState(state);
  }
  const from = startStep(state, { ...options, fromStep });
  for (const step of STEP_NAMES.slice(stateStepIndex(from))) {
    if (options.resume && state.completedSteps.includes(step) && !options.fromStep) {
      continue;
    }
    await runStep(state, step);
  }
  console.log(`[done] stack ready in ${Math.round((Date.now() - started) / 1000)}s`);
};

/** Resolves and prints the stack plan without mutating state or containers. */
export const upDryRun = async (options: Omit<UpOptions, "dryRun">) => {
  if (options.resume) {
    const state = await loadState();
    if (!state) {
      throw new ResumeError("No .fhevm/state/state.json to resume from");
    }
    state.requiresGitHub = false;
    state.scenarioSourcePath ??= state.scenario?.sourcePath;
    ensureResumeOptions(state, options);
    await preflight(state, false, state.requiresGitHub);
    printBundle(state, { detailed: true });
    printPlan(state, options.fromStep ?? startStep(state, options));
    console.log("[dry-run] resume preview uses persisted state only; no runtime state or containers were changed");
    return;
  }
  console.log(`[up] target=${options.target}`);
  const scenario = await resolveScenarioForOptions(options);
  await assertDockerMemory(scenario);
  const bundle = await previewBundle(options, process.env);
  await assertSchemaCompatibility(bundle, options.overrides, scenario, options.allowSchemaMismatch);
  const state = previewStateFromBundle(options, bundle, scenario);
  await preflight(state, false, state.requiresGitHub);
  printBundle(state, { detailed: true });
  printPlan(state, options.fromStep);
  console.log("[dry-run] preflight passed; no runtime state or containers were changed");
};

/** Deletes generated runtime artifacts while keeping persisted stack state. */
const pruneGeneratedRuntimeArtifacts = async () => {
  const targets = [ENV_DIR, COMPOSE_OUT_DIR, GENERATED_CONFIG_DIR, ADDRESS_DIR];
  await Promise.all(targets.map(async (target) => {
    if (await exists(target)) {
      await remove(target);
    }
  }));
};

/** Stops the running stack and removes generated runtime artifacts. */
export const down = async () => {
  const state = await loadState();
  const existing = await projectContainers(true);
  if (!existing.length) {
    console.log("[down] nothing to stop");
    await removeProjectResources("volume", "{{.Name}}");
    await removeProjectResources("network", "{{.Name}}");
    await pruneGeneratedRuntimeArtifacts();
    return;
  }
  if (!state) {
    await removeProjectResources("container", "{{.ID}}");
    await removeProjectResources("volume", "{{.Name}}");
    await removeProjectResources("network", "{{.Name}}");
    await pruneGeneratedRuntimeArtifacts();
    return;
  }
  await ensureRuntimeArtifacts(state, "teardown");
  const failed: string[] = [];
  if (state && state.scenario?.hostChains?.length > 1) {
    const toTearDown = multiChainComposeEntries(state).map(([name]) => name);
    const results = await Promise.all(
      toTearDown.map(async (name) => ({ name, ok: await multiChainComposeDown(name) })),
    );
    for (const { name, ok } of results) {
      if (!ok) failed.push(name);
    }
  }
  for (const component of [...COMPONENTS].reverse()) {
    console.log(`[down] ${component}`);
    const ok = await composeDown(component);
    if (!ok) {
      failed.push(component);
    }
  }
  if (failed.length) {
    throw new PreflightError(`Failed to stop components: ${failed.join(", ")}`);
  }
  const leftovers = await run(
    ["docker", "ps", "-a", "--filter", `label=com.docker.compose.project=${PROJECT}`, "--format", "{{.ID}}"],
    { allowFailure: true },
  );
  const ids = leftovers.stdout.split(/\r?\n/).map((line) => line.trim()).filter(Boolean);
  if (ids.length) {
    console.log(`[down] removing ${ids.length} stale project container${ids.length === 1 ? "" : "s"}`);
    await run(["docker", "rm", "-fv", ...ids], { allowFailure: true });
  }
  await removeProjectResources("volume", "{{.Name}}");
  await removeProjectResources("network", "{{.Name}}");
  await pruneGeneratedRuntimeArtifacts();
};

/** Stops the stack, removes owned images unless asked to keep them, and deletes persisted state. */
export const clean = async (options: CleanOptions) => {
  console.log("[clean] start");
  const state = await loadState();
  await down();
  if (!options.keepImages && state?.builtImages?.length) {
    console.log(`[clean] removing ${state.builtImages.length} owned image${state.builtImages.length === 1 ? "" : "s"}`);
    const failures: string[] = [];
    for (const image of state.builtImages) {
      const currentId = await inspectImageId(image.ref);
      if (!currentId || currentId !== image.id) {
        continue;
      }
      console.log(`[image] ${image.ref}`);
      const result = await run(["docker", "image", "rm", image.ref], { allowFailure: true });
      if (result.code !== 0) {
        failures.push(`${image.ref}: ${result.stderr.trim() || "docker image rm failed"}`);
      }
    }
    if (failures.length) {
      throw new PreflightError(`Failed to remove owned images:\n${failures.join("\n")}`);
    }
  }
  if (await exists(STATE_DIR)) {
    console.log(`[clean] removing ${STATE_DIR}`);
  } else {
    console.log("[clean] no runtime state");
  }
  await remove(STATE_DIR);
  console.log("[clean] done");
};

/** Prints persisted state and currently running stack containers. */
export const status = async () => {
  const state = await loadState();
  if (state) {
    const topology = topologyForState(state);
    const overrides = visibleOverrides(state);
    console.log(`[target] ${state.target}`);
    if (overrides.length) {
      console.log(`[overrides] ${overrides.map(describeOverride).join(", ")}`);
      for (const warning of overrideWarnings(overrides, state.target)) {
        console.log(`[warn] ${warning}`);
      }
    }
    console.log(
      `[topology] n=${topology.count} t=${topology.threshold} (${topology.count} coprocessor${topology.count === 1 ? "" : "s"}, threshold ${topology.threshold})${state.scenario.hostChains.length > 1 ? " multi-chain" : ""}`,
    );
    if (state.scenario.origin !== "default") {
      console.log(`[scenario] ${state.scenario.origin}${state.scenario.sourcePath ? ` ${state.scenario.sourcePath}` : ""}`);
      for (const instance of state.scenario.instances) {
        const source = instance.source.mode === "registry" ? `registry:${instance.source.tag}` : instance.source.mode;
        console.log(`[coprocessor-${instance.index}] ${source}`);
      }
    }
    console.log(`[steps] ${state.completedSteps.join(", ") || "none"}`);
    console.log(`[updated] ${state.updatedAt}`);
    if (state.builtImages?.length) {
      console.log(`[owned-images] ${state.builtImages.length}`);
      for (const image of state.builtImages) {
        console.log(`  ${image.ref} (${image.group})`);
      }
    }
  }
  const ps = await run(
    ["docker", "ps", "--filter", `label=com.docker.compose.project=${PROJECT}`, "--format", "{{.Names}}\t{{.Status}}"],
    { allowFailure: true },
  );
  if (ps.code !== 0) {
    throw new PreflightError(`Docker unavailable: ${ps.stderr.trim() || "docker ps failed"}`);
  }
  if (!ps.stdout.trim()) {
    if (state) {
      console.log("[warn] persisted state exists but the stack is stopped; run `fhevm-cli up --resume` to restart it");
    }
    console.log("No fhevm containers");
    return;
  }
  console.log(ps.stdout.trim());
};

/** Streams logs for the requested stack container, or the first running one. */
export const logs = async (service: string | undefined, options: { follow: boolean } = { follow: true }) => {
  const requested = service ? LOG_TARGETS[service] ?? service : undefined;
  const matchesRequested = (item: string) => !requested || item === requested || item.endsWith(`-${requested}`);
  const list = async (includeExited: boolean) =>
    run(
      ["docker", "ps", ...(includeExited ? ["-a"] : []), "--filter", `label=com.docker.compose.project=${PROJECT}`, "--format", "{{.Names}}"],
      { allowFailure: true },
    );
  const running = await list(false);
  if (running.code !== 0) {
    throw new PreflightError(`Docker unavailable: ${running.stderr.trim() || "docker ps failed"}`);
  }
  const pickContainers = (stdout: string) =>
    stdout
      .split("\n")
      .map((item) => item.trim())
      .filter(Boolean)
      .filter(matchesRequested);
  let containers = pickContainers(running.stdout);
  const hasRequestedMatch = () =>
    requested ? containers.some((item) => item === requested || item.endsWith(`-${requested}`)) : containers.length > 0;
  if (requested && !hasRequestedMatch()) {
    const all = await list(true);
    if (all.code !== 0) {
      throw new PreflightError(`Docker unavailable: ${all.stderr.trim() || "docker ps -a failed"}`);
    }
    containers = pickContainers(all.stdout);
  }
  if (!containers.length) {
    throw new PreflightError(`No containers match ${service ?? "fhevm"}; run \`fhevm-cli status\` for current stack state`);
  }
  const exactMatch = requested
    ? containers.find((item) => item === requested) ?? containers.find((item) => item.endsWith(`-${requested}`))
    : undefined;
  if (requested && !exactMatch && containers.length > 1) {
    throw new PreflightError(`Multiple containers match ${service}: ${containers.join(", ")}`);
  }
  const container = !requested ? containers[0] : exactMatch ?? containers[0];
  if (!requested) {
    console.log(`[logs] ${options.follow ? "following" : "showing recent logs from"} ${container}`);
  }
  if (!options.follow) {
    const result = await run(["docker", "logs", "--tail", "200", container], { allowFailure: true });
    if (result.code !== 0) {
      throw new PreflightError(result.stderr.trim() || `docker logs failed for ${container}`);
    }
    const output = result.stdout + result.stderr;
    if (output) {
      process.stdout.write(output);
    }
    return;
  }
  await runStreaming(["docker", "logs", "--follow", "--tail", "200", container]);
};

/** Prints the bundled scenario catalog for operator discovery. */
export const listScenarios = async () => {
  const scenarios = await listScenarioSummaries();
  if (!scenarios.length) {
    console.log("No bundled scenarios found.");
    return;
  }
  for (const scenario of scenarios) {
    const header = scenario.name && scenario.name !== scenario.key ? `${scenario.key} - ${scenario.name}` : scenario.key;
    console.log(header);
    if (scenario.description) {
      console.log(`  ${scenario.description}`);
    }
  }
};

type UpgradeOptions = {
  lockFile?: string;
};

export const changedVersionKeys = (current: VersionBundle, next: VersionBundle) =>
  [...new Set([...Object.keys(current.env), ...Object.keys(next.env)])]
    .filter((key) => (current.env[key] ?? "") !== (next.env[key] ?? ""))
    .sort();

export const assertVersionLockChanges = (label: string, allowedVersionKeys: readonly string[], changedKeys: string[]) => {
  const disallowed = changedKeys.filter((key) => !allowedVersionKeys.includes(key));
  if (disallowed.length) {
    throw new PreflightError(
      `${label} lock changes unrelated version keys: ${disallowed.join(", ")}. Allowed: ${allowedVersionKeys.join(", ")}`,
    );
  }
};

const isOverrideGroup = (group: string): group is OverrideGroup =>
  OVERRIDE_GROUPS.includes(group as OverrideGroup);

export const runtimeUpgradeOverrideGroups = (group: UpgradeGroup): OverrideGroup[] => {
  if (group === "kms") {
    return ["kms-connector"];
  }
  if (group === "kms-core") {
    return [];
  }
  return isOverrideGroup(group) ? [group] : [];
};

export const removeRuntimeUpgradeOverrides = (overrides: LocalOverride[], group: UpgradeGroup) => {
  const upgraded = new Set(runtimeUpgradeOverrideGroups(group));
  return overrides.filter((override) => !upgraded.has(override.group));
};

const applyRuntimeUpgradeLock = async (
  state: State,
  group: UpgradeGroup,
  allowedVersionKeys: readonly string[],
  lockFile: string,
) => {
  const lockPath = path.resolve(lockFile);
  const next = await readJson<VersionBundle>(lockPath);
  if (!next.env || typeof next.env !== "object") {
    throw new PreflightError(`Invalid upgrade lock ${lockFile}: missing env map`);
  }
  const changedKeys = changedVersionKeys(state.versions, next);
  assertVersionLockChanges(`upgrade ${group}`, allowedVersionKeys, changedKeys);
  const nextState = {
    ...state,
    target: next.target,
    lockPath,
    requiresGitHub: targetNeedsGitHub({ target: next.target, lockFile }),
    overrides: removeRuntimeUpgradeOverrides(state.overrides, group),
    versions: next,
    updatedAt: new Date().toISOString(),
  } satisfies State;
  assertSupportedTargetScenario(nextState.target, nextState.scenario);
  assertSupportedBundleScenario({ versions: nextState.versions, overrides: nextState.overrides, scenario: nextState.scenario });
  const incompatibilities = validateBundleCompatibility(nextState);
  if (incompatibilities.length) {
    throw new IncompatibleVersions(incompatibilities.map((item) => item.message));
  }
  return { changedKeys, state: nextState };
};

const mergeLocalOverrides = (current: LocalOverride[], additions: LocalOverride[] = []) => {
  const seen = new Set(current.map((override) => JSON.stringify(override)));
  const next = [...current];
  for (const override of additions) {
    const key = JSON.stringify(override);
    if (!seen.has(key)) {
      seen.add(key);
      next.push(override);
    }
  }
  return next;
};

/** Applies an ordered rollout version lock without restarting runtime services. */
export const applyVersionLock = async (
  label: string,
  lockFile: string,
  allowedVersionKeys: readonly string[],
  options: { overrides?: LocalOverride[] } = {},
) => {
  const state = await loadState();
  if (!state || !(await projectContainers()).length) {
    throw new PreflightError("Stack is not running; start one with `fhevm-cli up` first");
  }
  await ensureRuntimeArtifacts(state, "rollout version lock");
  const lockPath = path.resolve(lockFile);
  const next = await readJson<VersionBundle>(lockPath);
  if (!next.env || typeof next.env !== "object") {
    throw new PreflightError(`Invalid rollout lock ${lockFile}: missing env map`);
  }
  const changedKeys = changedVersionKeys(state.versions, next);
  assertVersionLockChanges(label, allowedVersionKeys, changedKeys);
  const nextState = {
    ...state,
    target: next.target,
    lockPath,
    requiresGitHub: targetNeedsGitHub({ target: next.target, lockFile }),
    overrides: mergeLocalOverrides(state.overrides, options.overrides),
    versions: next,
    updatedAt: new Date().toISOString(),
  } satisfies State;
  assertSupportedTargetScenario(nextState.target, nextState.scenario);
  assertSupportedBundleScenario({ versions: nextState.versions, overrides: nextState.overrides, scenario: nextState.scenario });
  const incompatibilities = validateBundleCompatibility(nextState);
  if (incompatibilities.length) {
    throw new IncompatibleVersions(incompatibilities.map((item) => item.message));
  }
  await assertSchemaCompatibility(nextState.versions, nextState.overrides, nextState.scenario, false);
  await saveState(nextState);
  await generateRuntime(nextState, stackSpecForState(nextState));
  console.log(`[rollout] ${label} versions=${changedKeys.join(", ") || "(none)"}`);
};

/** Re-reads deployed contract addresses after contract runbook tasks mutate proxies. */
export const refreshDiscovery = async () => {
  const state = await loadState();
  if (!state || !(await projectContainers()).length) {
    throw new PreflightError("Stack is not running; start one with `fhevm-cli up` first");
  }
  await ensureRuntimeArtifacts(state, "rollout discovery");
  const contracts = await discoverContracts(state);
  const discovery = await ensureDiscovery(state);
  discovery.gateway = contracts.gateway;
  discovery.hosts = { ...discovery.hosts, ...contracts.hosts };
  validateDiscovery(state);
  await saveState(state);
  await generateRuntime(state, stackSpecForState(state));
  console.log("[rollout] discovery refreshed");
};

const waitForRelayer = async () => {
  await waitForContainer("fhevm-relayer-db", "healthy");
  await waitForContainer("fhevm-relayer", "running");
  await waitForLog("fhevm-relayer", /All servers are ready and responding/);
};

const waitForUpgrade = async (state: State, group: UpgradeGroup, runtimeServices: string[]) => {
  if (group === "coprocessor") {
    const extraTargets = multiChainCoprocessorUpgradeTargets(state, runtimeServices);
    for (const target of extraTargets) {
      if (target.services.length) {
        await multiChainComposeUp(target.compose, target.services);
      }
      await waitForStableChainListeners(state, target.chainKey);
    }
    await waitForCoprocessor(state);
    await postBootHealthGate([...coprocessorHealthContainers(state), ...extraTargets.flatMap((target) => target.services)]);
    return;
  }
  if (group === "kms-connector" || group === "kms") {
    await waitForContainer(KMS_CORE_CONTAINER, "running");
    await waitForKmsConnector();
    await postBootHealthGate(KMS_CONNECTOR_HEALTH_CONTAINERS);
    return;
  }
  if (group === "kms-core") {
    await waitForContainer(KMS_CORE_CONTAINER, "running");
    return;
  }
  if (group === "listener-core") {
    await waitForContainer("listener-redis", "running");
    await waitForContainer("listener-publisher-for-anvil", "running");
    return;
  }
  if (group === "relayer") {
    await waitForRelayer();
    return;
  }
  await waitForTestSuite();
};

/** Upgrades one runtime group in place, including allowed migrations and optional version-lock application. */
export const upgradeRuntimeGroup = async (groupValue: string | undefined, options: UpgradeOptions = {}) => {
  const state = await loadState();
  if (!state || !(await projectContainers()).length) {
    throw new PreflightError(
      "Stack is not running; start one with `fhevm-cli up --override ...` or `fhevm-cli up --scenario ...` first",
    );
  }
  await ensureRuntimeArtifacts(state, "upgrade");
  const plan = resolveUpgradePlan(state, groupValue, { lockFile: !!options.lockFile });
  for (const step of plan.steps) {
    if (!state.completedSteps.includes(step)) {
      throw new PreflightError(`upgrade requires a stack that has completed the ${step} step`);
    }
  }
  const nextState = options.lockFile
    ? (await applyRuntimeUpgradeLock(state, plan.group, plan.versionKeys, options.lockFile)).state
    : state;
  await assertSchemaCompatibility(nextState.versions, nextState.overrides, nextState.scenario, false);
  console.log(`[upgrade] ${plan.group}`);
  await saveState(nextState);
  await generateRuntime(nextState, stackSpecForState(nextState));
  if (plan.group === "listener-core") {
    await postgresExec("", ["-c", "CREATE DATABASE listener;"]);
  }
  for (const component of plan.components) {
    await maybeBuild(component.component, nextState, { force: true });
  }
  for (const component of plan.components) {
    if (!component.migrationServices.length) {
      continue;
    }
    await composeUp(component.component, component.migrationServices, { forceRecreate: true });
    for (const service of component.migrationServices) {
      await waitForContainer(service, "complete");
    }
  }
  for (const component of plan.components) {
    await composeUp(component.component, component.runtimeServices, { noDeps: true });
  }
  await waitForUpgrade(nextState, plan.group, plan.runtimeServices);
  for (const step of plan.steps) {
    await markStep(nextState, step);
  }
};

const RESUME_HINT_BLOCKERS = new Set([
  "--target",
  "--sha",
  "--lock-file",
  "--scenario",
  "--override",
  "--build",
  "--reset",
  "--from-step",
  "--allow-schema-mismatch",
]);

/** Decides whether a failing command should print the `--resume` hint. */
export const shouldShowResumeHint = (rawArgs: string[]) =>
  !rawArgs.some((arg) => arg === "--dry-run" || arg.startsWith("--dry-run=")) &&
  !rawArgs.some((arg) => [...RESUME_HINT_BLOCKERS].some((flag) => arg === flag || arg.startsWith(`${flag}=`)));

/** Prints a resume hint after eligible failed `up` or `deploy` commands. */
export const showResumeHint = async (rawArgs: string[]) => {
  if (!shouldShowResumeHint(rawArgs)) {
    return;
  }
  const command = rawArgs[0];
  if (command !== "up" && command !== "deploy") {
    return;
  }
  const state = await loadState();
  if (state?.completedSteps.length) {
    console.error("Hint: run with --resume to continue, or without to start fresh.");
  }
};
