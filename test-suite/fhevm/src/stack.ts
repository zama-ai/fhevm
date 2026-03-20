/**
 * Orchestrates fhevm stack lifecycle commands such as up, down, resume, clean, upgrade, status, and logs.
 */
import fs from "node:fs/promises";

import { ensureLockSnapshot, previewBundle, resolveBundle } from "./bundle-store";
import { compatPolicyForState, requiresMultichainAclAddress, validateBundleCompatibility } from "./compat";
import { type ComposeDoc, generatedComposeComponents, loadMergedComposeDoc, resolvedComposeEnv, serviceNameList } from "./render-compose";
import { renderRuntime } from "./render";
import { resolveScenarioForOptions, runtimePlanForState, topologyForState } from "./runtime-plan";
import { effectiveOverrides, hasLocalCoprocessorInstance, listScenarioSummaries } from "./scenario";
import { run, runStreaming, runWithHeartbeat, composeEnv } from "./shell";
import { clearState, loadState, markStep, saveState } from "./state";
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
} from "./errors";
import { describeBundle } from "./resolve";
import {
  ADDRESS_DIR,
  COMPOSE_OUT_DIR,
  COMPONENT_BY_STEP,
  COMPONENTS,
  COPROCESSOR_DB_CONTAINER,
  CRSGEN_ID_SELECTOR,
  ENV_DIR,
  GENERATED_CONFIG_DIR,
  GROUP_BUILD_COMPONENTS,
  GROUP_BUILD_SERVICES,
  GROUP_SERVICE_SUFFIXES,
  KEYGEN_ID_SELECTOR,
  KMS_CORE_CONTAINER,
  LOCK_DIR,
  LOG_TARGETS,
  MINIO_EXTERNAL_URL,
  MINIO_INTERNAL_URL,
  PORTS,
  PROJECT,
  REPO_ROOT,
  SCHEMA_COUPLED_GROUPS,
  STATE_DIR,
  TEST_SUITE_CONTAINER,
  composePath,
  dockerArgs,
  envPath,
  gatewayAddressesPath,
  gatewayAddressesSolidityPath,
  hostAddressesPath,
  hostAddressesSolidityPath,
  paymentBridgingAddressesSolidityPath,
  relayerConfigPath,
  versionsEnvPath,
} from "./layout";
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
} from "./types";
import { STEP_NAMES } from "./types";
import {
  exists,
  hostReachableMaterialUrl,
  hostReachableRpcUrl,
  predictedCrsId,
  predictedKeyId,
  readEnvFile,
  readEnvFileIfExists,
  remove,
  toServiceName,
  withHexPrefix,
  writeJson,
} from "./utils";

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
const UPGRADEABLE_GROUPS = ["coprocessor", "kms-connector", "test-suite"] as const;
type UpgradeGroup = (typeof UPGRADEABLE_GROUPS)[number];
const POST_BOOT_HEALTH_GATE_DELAY_MS = 5_000;
const KMS_CONNECTOR_HEALTH_CONTAINERS = [
  "kms-connector-gw-listener",
  "kms-connector-kms-worker",
  "kms-connector-tx-sender",
];
const NETWORK_TARGETS: ReadonlySet<string> = new Set(["devnet", "testnet", "mainnet"]);

const timed = async <T>(label: string, task: () => Promise<T>) => {
  const started = Date.now();
  const result = await task();
  console.log(`${label} done (${Math.round((Date.now() - started) / 1000)}s)`);
  return result;
};

const stateStepIndex = (step: StepName) => STEP_NAMES.indexOf(step);

const describeOverride = (item: { group: string; services?: string[] }) =>
  `${item.group}${item.services?.length ? `[${item.services.join(",")}]` : ""}`;

const visibleOverrides = (state: Pick<State, "overrides" | "scenario">) =>
  effectiveOverrides(state.overrides, state.scenario);

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

const printBundle = (bundle: VersionBundle, options?: { detailed?: boolean }) => {
  console.log(`[resolve] ${bundle.lockName}`);
  if (options?.detailed) {
    console.log(describeBundle(bundle));
  }
};

const printPlan = (state: Pick<State, "target" | "overrides" | "scenario">, fromStep?: StepName) => {
  const topology = topologyForState(state);
  const overrides = visibleOverrides(state);
  console.log(`[plan] profile=${state.target}`);
  if (overrides.length) {
    console.log(`[plan] overrides=${overrides.map(describeOverride).join(", ")}`);
    for (const warning of overrideWarnings(overrides, state.target)) {
      console.log(`[warn] ${warning}`);
    }
  }
  console.log(`[plan] topology=n${topology.count}/t${topology.threshold}`);
  console.log(`[plan] steps=${STEP_NAMES.slice(stateStepIndex(fromStep ?? STEP_NAMES[0])).join(" -> ")}`);
};

export const shellEscape = (value: string) => `'${value.replaceAll("'", `'\\''`)}'`;

const projectContainers = async (all = false) => {
  const ps = await run(
    ["docker", "ps", ...(all ? ["-a"] : []), "--filter", `label=com.docker.compose.project=${PROJECT}`, "--format", "{{.Names}}"],
    { allowFailure: true },
  );
  if (ps.code !== 0) {
    throw new PreflightError(ps.stderr.trim() || "docker ps failed");
  }
  return ps.stdout.split(/\r?\n/).map((line) => line.trim()).filter(Boolean);
};

const partialSchemaOverrides = (overrides: LocalOverride[]) =>
  overrides.filter(
    (item): item is LocalOverride & { services: string[] } =>
      !!item.services?.length && SCHEMA_COUPLED_GROUPS.includes(item.group),
  );

const minioIp = async () => {
  const result = await run(["docker", "inspect", "fhevm-minio"], { allowFailure: true });
  if (result.code !== 0) {
    throw new PreflightError("Could not determine MinIO IP");
  }
  let inspected: Array<{
    NetworkSettings: { Networks: Record<string, { IPAddress: string }> };
  }>;
  try {
    inspected = JSON.parse(result.stdout) as Array<{
      NetworkSettings: { Networks: Record<string, { IPAddress: string }> };
    }>;
  } catch (error) {
    throw new PreflightError(
      `docker inspect fhevm-minio returned invalid JSON: ${error instanceof Error ? error.message : String(error)}`,
    );
  }
  const ip = inspected[0] ? Object.values(inspected[0].NetworkSettings.Networks)[0]?.IPAddress : "";
  if (!ip) {
    throw new PreflightError("Could not determine MinIO IP");
  }
  return ip;
};

const defaultEndpoints = async () => {
  const ip = await minioIp();
  return {
    gatewayHttp: "http://gateway-node:8546",
    gatewayWs: "ws://gateway-node:8546",
    hostHttp: "http://host-node:8545",
    hostWs: "ws://host-node:8545",
    minioInternal: MINIO_INTERNAL_URL,
    minioExternal: `http://${ip}:9000`,
  };
};

const createDiscovery = (endpoints: Discovery["endpoints"]): Discovery => ({
  gateway: {},
  host: {},
  kmsSigner: "",
  fheKeyId: predictedKeyId(),
  crsKeyId: predictedCrsId(),
  endpoints,
});

const ensureDiscovery = async (state: State) => {
  if (!state.discovery) {
    state.discovery = createDiscovery(await defaultEndpoints());
  }
  return state.discovery;
};

const discoverContracts = async () => {
  const gwExists = await exists(gatewayAddressesPath);
  const hostExists = await exists(hostAddressesPath);
  if (!gwExists || !hostExists) {
    throw new PreflightError("Missing generated address files under .fhevm/runtime/addresses");
  }
  return {
    gateway: await readEnvFile(gatewayAddressesPath),
    host: await readEnvFile(hostAddressesPath),
  };
};

const preflight = async (state: State, strictPorts = true, needsGitHub = true) => {
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
    PORTS.map(async (port) => {
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
    const guard = SCHEMA_GUARDS[item.group as keyof typeof SCHEMA_GUARDS];
    if (!guard) {
      continue;
    }
    const ref = bundle.env[guard.versionKey];
    if (!ref) {
      continue;
    }
    const verified = await run(["git", "rev-parse", "-q", "--verify", `${ref}^{commit}`], {
      cwd: REPO_ROOT,
      allowFailure: true,
    });
    if (verified.code !== 0) {
      throw new SchemaGuardError(
        item.group,
        `Cannot compare local ${item.group} migrations against ${ref}; local git ref is missing. Run \`git fetch --tags\` or pass --allow-schema-mismatch.`,
      );
    }
    const untracked = await run(
      ["git", "ls-files", "--others", "--exclude-standard", "--", guard.repoPath],
      { cwd: REPO_ROOT, allowFailure: true },
    );
    if (untracked.code !== 0) {
      throw new SchemaGuardError(item.group, `Failed to inspect local ${item.group} migrations`);
    }
    if (untracked.stdout.trim()) {
      throw new SchemaGuardError(
        item.group,
        `${item.group}: local DB migrations diverge from ${ref}. Use --override ${item.group} or pass --allow-schema-mismatch if you know this service remains compatible.`,
      );
    }
    const diff = await run(["git", "diff", "--quiet", "--exit-code", ref, "--", guard.repoPath], {
      cwd: REPO_ROOT,
      allowFailure: true,
    });
    if (diff.code === 1) {
      throw new SchemaGuardError(
        item.group,
        `${item.group}: local DB migrations diverge from ${ref}. Use --override ${item.group} or pass --allow-schema-mismatch if you know this service remains compatible.`,
      );
    }
    if (diff.code !== 0 && diff.code !== 1) {
      throw new SchemaGuardError(item.group, `Failed to compare local ${item.group} migrations against ${ref}`);
    }
  }
};

const assertUpgradeSchemaStable = async (
  bundle: VersionBundle,
  group: OverrideGroup,
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
    throw new SchemaGuardError(
      group,
      `Cannot compare local ${group} migrations against ${ref}; local git ref is missing. Run \`git fetch --tags\` or do a fresh \`fhevm-cli up\`.`,
    );
  }
  const untracked = await run(
    ["git", "ls-files", "--others", "--exclude-standard", "--", guard.repoPath],
    { cwd: REPO_ROOT, allowFailure: true },
  );
  if (untracked.code !== 0) {
    throw new SchemaGuardError(group, `Failed to inspect local ${group} migrations`);
  }
  const diffMessage =
    `${group}: local DB migrations changed. \`fhevm-cli upgrade ${group}\` only supports runtime rebuilds; do a fresh \`fhevm-cli up\` for schema changes.`;
  if (untracked.stdout.trim()) {
    throw new SchemaGuardError(group, diffMessage);
  }
  const diff = await run(["git", "diff", "--quiet", "--exit-code", ref, "--", guard.repoPath], {
    cwd: REPO_ROOT,
    allowFailure: true,
  });
  if (diff.code === 1) {
    throw new SchemaGuardError(group, diffMessage);
  }
  if (diff.code !== 0 && diff.code !== 1) {
    throw new SchemaGuardError(group, `Failed to compare local ${group} migrations against ${ref}`);
  }
};

const validateDiscovery = (state: Pick<State, "target" | "versions" | "discovery" | "overrides" | "scenario">) => {
  const discovery = state.discovery;
  if (!discovery) {
    throw new PreflightError("Missing discovery state");
  }
  const requiredGateway = [
    "GATEWAY_CONFIG_ADDRESS",
    "KMS_GENERATION_ADDRESS",
    "DECRYPTION_ADDRESS",
    "INPUT_VERIFICATION_ADDRESS",
    "CIPHERTEXT_COMMITS_ADDRESS",
    ...(requiresMultichainAclAddress(state) ? ["MULTICHAIN_ACL_ADDRESS"] : []),
  ];
  const requiredHost = [
    "ACL_CONTRACT_ADDRESS",
    "FHEVM_EXECUTOR_CONTRACT_ADDRESS",
    "KMS_VERIFIER_CONTRACT_ADDRESS",
    "INPUT_VERIFIER_CONTRACT_ADDRESS",
    "PAUSER_SET_CONTRACT_ADDRESS",
  ];
  for (const key of requiredGateway) {
    if (!discovery.gateway[key]) {
      throw new PreflightError(`Missing gateway discovery value ${key}`);
    }
  }
  for (const key of requiredHost) {
    if (!discovery.host[key]) {
      throw new PreflightError(`Missing host discovery value ${key}`);
    }
  }
  if (!discovery.kmsSigner) {
    throw new PreflightError("Missing KMS signer discovery");
  }
  if (!discovery.fheKeyId || !discovery.crsKeyId) {
    throw new PreflightError("Missing predicted key ids");
  }
};

const ensureRuntimeArtifacts = async (state: State, reason: string) => {
  const topology = topologyForState(state);
  await ensureLockSnapshot(state.lockPath, state.versions);
  const generatedCompose = [...generatedComposeComponents(runtimePlanForState(state))].map(composePath);
  const requiredPaths = [
    versionsEnvPath,
    relayerConfigPath,
    ...COMPONENTS.map(envPath),
    ...generatedCompose,
    ...Array.from({ length: Math.max(0, topology.count - 1) }, (_, index) => envPath(`coprocessor.${index + 1}`)),
    ...(state.discovery
      ? [
          gatewayAddressesPath,
          gatewayAddressesSolidityPath,
          paymentBridgingAddressesSolidityPath,
          hostAddressesPath,
          hostAddressesSolidityPath,
        ]
      : []),
  ];
  const allExist = (await Promise.all(requiredPaths.map((file) => exists(file)))).every(Boolean);
  if (allExist) {
    return;
  }
  console.log(`[regen] restoring runtime artifacts for ${reason}`);
  await renderRuntime(state, runtimePlanForState(state));
};

const resetAfterStep = async (step: StepName) => {
  const start = stateStepIndex(step);
  const failed: string[] = [];
  for (let index = STEP_NAMES.length - 1; index >= start; index -= 1) {
    for (const component of COMPONENT_BY_STEP[STEP_NAMES[index]]) {
      const ok = await composeDown(component);
      if (!ok) {
        failed.push(component);
      }
    }
  }
  if (failed.length) {
    throw new PreflightError(`Failed to stop components while resetting from ${step}: ${failed.join(", ")}`);
  }
};

export const resolveUpgradePlan = (
  state: Pick<State, "overrides" | "scenario">,
  groupValue: string | undefined,
) => {
  if (!groupValue || !UPGRADEABLE_GROUPS.includes(groupValue as UpgradeGroup)) {
    throw new Error(`upgrade expects one of ${UPGRADEABLE_GROUPS.join(", ")}`);
  }
  const group = groupValue as UpgradeGroup;
  const groupOverrides = state.overrides.filter((item) => item.group === group);
  if (group === "coprocessor" && !hasLocalCoprocessorInstance(state) && !groupOverrides.length) {
    throw new Error("upgrade requires an active local coprocessor instance");
  }
  if (group !== "coprocessor" && !groupOverrides.length) {
    throw new Error(`upgrade requires an active local override for ${group}`);
  }
  const [component] = GROUP_BUILD_COMPONENTS[group];
  if (!component) {
    throw new Error(`No runtime component registered for ${group}`);
  }
  const selectedServices = groupOverrides.flatMap((item) => item.services ?? []);
  const fullGroupServices = groupOverrides.length && !selectedServices.length ? GROUP_BUILD_SERVICES[group] : [];
  const overrideServices = selectedServices.length ? [...new Set(selectedServices)] : fullGroupServices;
  const scenario = state.scenario;
  const plannedServices =
    group === "coprocessor"
      ? scenario.instances.flatMap((instance) => {
          if (instance.source.mode === "registry") {
            return [];
          }
          const selected =
            instance.source.mode === "local"
              ? instance.localServices ?? GROUP_BUILD_SERVICES.coprocessor
              : overrideServices;
          return selected.map((service) =>
            instance.index === 0 ? service : service.replace(/^coprocessor-/, `coprocessor${instance.index}-`),
          );
        })
      : selectedServices.length
        ? [...new Set(selectedServices)]
        : GROUP_BUILD_SERVICES[group];
  const services = [...new Set(plannedServices)];
  const runtimeServices = services.filter((service) => !service.endsWith("-db-migration"));
  if (!runtimeServices.length) {
    throw new Error(`upgrade requires restartable runtime services for ${group}`);
  }
  return {
    component,
    group,
    runtimeServices,
    step: group === "coprocessor" ? "coprocessor" : group,
  } as const;
};

const dockerInspect = async (name: string) => {
  const result = await run(["docker", "inspect", name], { allowFailure: true });
  if (result.code !== 0) {
    const message = (result.stderr || result.stdout).trim();
    if (/no such object|no such container/i.test(message)) {
      return [] as Array<{
        Name: string;
        State: { Status: string; ExitCode: number; Health?: { Status: string } };
        NetworkSettings: { Networks: Record<string, { IPAddress: string }> };
      }>;
    }
    throw new PreflightError(message || `docker inspect ${name} failed`);
  }
  try {
    return JSON.parse(result.stdout) as Array<{
      Name: string;
      State: { Status: string; ExitCode: number; Health?: { Status: string } };
      NetworkSettings: { Networks: Record<string, { IPAddress: string }> };
    }>;
  } catch (error) {
    throw new PreflightError(
      `docker inspect ${name} returned invalid JSON: ${error instanceof Error ? error.message : String(error)}`,
    );
  }
};

const waitForContainer = async (container: string, want: "running" | "healthy" | "complete") => {
  const attempts = 90;
  for (let attempt = 0; attempt <= attempts; attempt += 1) {
    const [inspect] = await dockerInspect(container);
    if (inspect) {
      if (want === "healthy" && inspect.State.Health?.Status === "healthy") {
        return;
      }
      if (want === "running" && inspect.State.Status === "running") {
        return;
      }
      if (want === "complete" && inspect.State.Status === "exited" && inspect.State.ExitCode === 0) {
        return;
      }
      if (inspect.State.Status === "exited" && inspect.State.ExitCode !== 0) {
        const logs = await run(["docker", "logs", container], { allowFailure: true });
        throw new ContainerCrashed(container, inspect.State.ExitCode, (logs.stdout + logs.stderr).trim());
      }
    }
    if (attempt === attempts) {
      throw new ProbeTimeout(container, 180);
    }
    await Bun.sleep(2_000);
  }
};

const waitForLog = async (container: string, pattern: RegExp) => {
  for (let attempt = 0; attempt <= 90; attempt += 1) {
    const logs = await run(["docker", "logs", container], { allowFailure: true });
    const combined = logs.stdout + logs.stderr;
    const match = combined.match(pattern);
    if (match) {
      return match[0];
    }
    if (attempt === 90) {
      throw new ProbeTimeout(container, 180);
    }
    await Bun.sleep(2_000);
  }
};

const waitForRpc = async (url: string) => {
  for (let attempt = 0; attempt <= 60; attempt += 1) {
    try {
      const response = await fetch(url, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ jsonrpc: "2.0", id: 1, method: "eth_chainId", params: [] }),
      });
      if (response.ok) {
        return;
      }
    } catch {
      // retry
    }
    if (attempt === 60) {
      throw new ProbeTimeout(url, 60);
    }
    await Bun.sleep(1_000);
  }
};

const postBootHealthGate = async (containers: string[], delayMs = POST_BOOT_HEALTH_GATE_DELAY_MS) => {
  if (delayMs > 0) {
    await Bun.sleep(delayMs);
  }
  const crashed: { name: string; exitCode: number; logs: string }[] = [];
  for (const name of containers) {
    const [inspect] = await dockerInspect(name);
    if (!inspect) {
      crashed.push({ name, exitCode: -1, logs: "(container not found)" });
      continue;
    }
    if (inspect.State.Status === "exited" && inspect.State.ExitCode !== 0) {
      const result = await run(["docker", "logs", "--tail", "30", name], { allowFailure: true });
      crashed.push({ name, exitCode: inspect.State.ExitCode, logs: (result.stdout + result.stderr).trim() });
    }
  }
  if (crashed.length) {
    const first = crashed[0];
    const details = crashed
      .map((item) => `  ${item.name} (exit ${item.exitCode}):\n    ${item.logs.split("\n").join("\n    ")}`)
      .join("\n");
    throw new ContainerCrashed(first.name, first.exitCode, `Post-boot health gate: ${crashed.length} container(s) crashed:\n${details}`);
  }
};

const discoverSigner = async () => {
  for (let attempt = 0; attempt <= 60; attempt += 1) {
    const logs = await run(["docker", "logs", KMS_CORE_CONTAINER], { allowFailure: true });
    const match = logs.stdout.match(/handle ([a-zA-Z0-9]+)/) ?? logs.stderr.match(/handle ([a-zA-Z0-9]+)/);
    if (match) {
      const handle = match[1];
      for (const prefix of ["PUB/PUB", "PUB"]) {
        try {
          const response = await fetch(`${MINIO_EXTERNAL_URL}/kms-public/${prefix}/VerfAddress/${handle}`);
          if (response.ok) {
            return {
              address: (await response.text()).trim(),
              minioKeyPrefix: prefix,
            };
          }
        } catch {
          // retry
        }
      }
    }
    if (attempt === 60) {
      throw new MinioError("Could not discover KMS signer after 60 attempts");
    }
    await Bun.sleep(1_000);
  }
  throw new MinioError("Could not discover KMS signer after 60 attempts");
};

const ensureMaterial = async (url: string) => {
  for (let attempt = 0; attempt <= 30; attempt += 1) {
    try {
      const response = await fetch(hostReachableMaterialUrl(url), { method: "HEAD" });
      if (response.ok) {
        return;
      }
    } catch {
      // retry
    }
    if (attempt === 30) {
      throw new MinioError(`Material not ready: ${url}`);
    }
    await Bun.sleep(1_000);
  }
};

const castBool = async (rpcUrl: string, to: string, signature: string, ...args: string[]) => {
  try {
    const result = await run(["cast", "call", to, signature, ...args, "--rpc-url", hostReachableRpcUrl(rpcUrl)]);
    const stdout = result.stdout.trim();
    return stdout === "true" || stdout === "0x1" || stdout === "0x0000000000000000000000000000000000000000000000000000000000000001";
  } catch (error) {
    throw new RpcError(rpcUrl, error instanceof Error ? error.message : String(error));
  }
};

const probeBootstrap = async (state: State) => {
  const discovery = state.discovery!;
  const keyPrefix = discovery.minioKeyPrefix ?? "PUB";
  try {
    const ethCallRaw = async (data: string) => {
      const rpcUrl = hostReachableRpcUrl(discovery.endpoints.gatewayHttp);
      const response = await fetch(rpcUrl, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({
          jsonrpc: "2.0",
          id: 1,
          method: "eth_call",
          params: [{ to: withHexPrefix(discovery.gateway.KMS_GENERATION_ADDRESS), data }, "latest"],
        }),
      });
      if (!response.ok) return 0n;
      const payload = (await response.json()) as { result?: string };
      if (!payload.result) {
        return 0n;
      }
      try {
        return BigInt(payload.result);
      } catch {
        throw new RpcError(rpcUrl, `eth_call returned malformed bigint result: ${payload.result}`);
      }
    };
    const actualKey = await ethCallRaw(KEYGEN_ID_SELECTOR);
    const actualCrs = await ethCallRaw(CRSGEN_ID_SELECTOR);
    if (actualKey === 0n || actualCrs === 0n) {
      return null;
    }
    const actualFheKeyId = actualKey.toString(16).padStart(64, "0");
    const actualCrsKeyId = actualCrs.toString(16).padStart(64, "0");
    await Promise.all([
      ensureMaterial(`${discovery.endpoints.minioExternal}/kms-public/${keyPrefix}/PublicKey/${actualFheKeyId}`),
      ensureMaterial(`${discovery.endpoints.minioExternal}/kms-public/${keyPrefix}/CRS/${actualCrsKeyId}`),
    ]);
    if (discovery.fheKeyId !== actualFheKeyId || discovery.crsKeyId !== actualCrsKeyId) {
      throw new PreflightError(
        `Predicted bootstrap ids drifted: expected ${discovery.fheKeyId}/${discovery.crsKeyId}, got ${actualFheKeyId}/${actualCrsKeyId}`,
      );
    }
    return { actualFheKeyId, actualCrsKeyId };
  } catch (error) {
    if (error instanceof MinioError || error instanceof PreflightError) {
      throw error;
    }
    return null;
  }
};

const waitForBootstrap = async (state: State, attempts = 120) => {
  for (let attempt = 0; attempt < attempts; attempt += 1) {
    const result = await probeBootstrap(state);
    if (result) {
      state.discovery!.actualFheKeyId = result.actualFheKeyId;
      state.discovery!.actualCrsKeyId = result.actualCrsKeyId;
      return result;
    }
    if (attempt < attempts - 1) {
      if (attempt === 0 || (attempt + 1) % 5 === 0) {
        console.log(`[wait] bootstrap materials (${(attempt + 1) * 2}s elapsed)`);
      }
      await Bun.sleep(2_000);
    }
  }
  throw new BootstrapTimeout(attempts * 2);
};

const imageRefsFromDoc = (doc: ComposeDoc, services: string[]) => {
  const selected = services.length ? services : Object.keys(doc.services);
  return [
    ...new Set(
      selected
        .map((name) => doc.services[name]?.image)
        .filter((value): value is string => typeof value === "string" && value.length > 0),
    ),
  ];
};

const coprocessorInstanceIndex = (service: string) => {
  const match = /^coprocessor(?:(\d+))?-/.exec(service);
  if (!match) {
    return undefined;
  }
  return match[1] ? Number(match[1]) : 0;
};

const inspectImageId = async (ref: string) => {
  const result = await run(["docker", "image", "inspect", ref, "--format", "{{.Id}}"], { allowFailure: true });
  return result.code === 0 ? result.stdout.trim() : "";
};

const saveBuiltImages = async (
  state: State,
  refs: Array<{ ref: string; group: BuiltImage["group"]; instanceIndex?: number }>,
) => {
  const current = new Map((state.builtImages ?? []).map((item) => [item.ref, item] as const));
  for (const entry of refs) {
    const id = await inspectImageId(entry.ref);
    if (!id) {
      continue;
    }
    current.set(entry.ref, {
      ref: entry.ref,
      id,
      group: entry.group,
      instanceIndex: entry.instanceIndex,
    });
  }
  state.builtImages = [...current.values()].sort((a, b) => a.ref.localeCompare(b.ref));
  await saveState(state);
};

const refsAlreadyBuilt = async (state: State, refs: string[]) =>
  (await Promise.all(
    refs.map(async (ref) => {
      const id = await inspectImageId(ref);
      return !!id && (state.builtImages ?? []).some((image) => image.ref === ref && image.id === id);
    }),
  )).every(Boolean);

const composeUp = async (
  component: string,
  services: string[] = [],
  options: { noDeps?: boolean; env?: Record<string, string> } = {},
) => {
  try {
    await runStreaming(
      [...dockerArgs(component), "up", "-d", ...(options.noDeps ? ["--no-deps"] : []), ...services],
      { env: await composeEnv(component, options.env) },
    );
  } catch (error) {
    throw new ContainerStartError(component, error instanceof Error ? error.message : String(error));
  }
};

const composeDown = async (component: string) => {
  try {
    const code = await runStreaming([...dockerArgs(component), "down", "-v"], {
      env: await composeEnv(component),
      allowFailure: true,
    });
    if (code !== 0) {
      console.log(`[warn] compose down failed for ${component} (${code})`);
      return false;
    }
    return true;
  } catch {
    console.log(`[warn] compose down failed for ${component}`);
    return false;
  }
};

const composeBuild = async (component: string, services: string[], env?: Record<string, string>) => {
  try {
    await runStreaming([...dockerArgs(component), "build", ...services], {
      env: await composeEnv(component, env),
    });
  } catch (error) {
    throw new ContainerStartError(component, error instanceof Error ? error.message : String(error));
  }
};

const maybeBuild = async (component: string, state: State, options: { force?: boolean } = {}) => {
  try {
    if (component === "coprocessor") {
      const doc = await loadMergedComposeDoc(component);
      const services = Object.entries(doc.services)
        .filter(([, service]) => !!service.build)
        .map(([name]) => name);
      if (!services.length) {
        return;
      }
      const refs = imageRefsFromDoc(doc, services);
      if (!options.force && (await refsAlreadyBuilt(state, refs))) {
        return;
      }
      console.log("[build] coprocessor");
      for (const ref of refs) {
        await run(["docker", "image", "rm", "-f", ref], { allowFailure: true });
      }
      for (const service of services) {
        await timed(`[build] ${service}`, () => composeBuild(component, [service]));
      }
      await saveBuiltImages(
        state,
        refs.map((ref) => ({
          ref,
          group: "coprocessor" as const,
          instanceIndex: coprocessorInstanceIndex(
            services.find((service) => doc.services[service]?.image === ref) ?? "",
          ),
        })),
      );
      return;
    }

    for (const override of state.overrides) {
      if (!GROUP_BUILD_COMPONENTS[override.group].includes(component)) {
        continue;
      }
      const doc = await loadMergedComposeDoc(component);
      const available = new Set(Object.keys(doc.services));
      const candidates = override.services?.length ? override.services : GROUP_BUILD_SERVICES[override.group];
      const services = candidates.filter((service) => available.has(service));
      if (!services.length) {
        continue;
      }
      console.log(`[build] ${override.group} (${component})`);
      const refs = imageRefsFromDoc(doc, services);
      if (!options.force && (await refsAlreadyBuilt(state, refs))) {
        continue;
      }
      for (const ref of refs) {
        await run(["docker", "image", "rm", "-f", ref], { allowFailure: true });
      }
      const seen = new Set<string>();
      const deduped = services.filter((service) => {
        const image = doc.services[service]?.image;
        if (typeof image !== "string" || seen.has(image)) {
          return false;
        }
        seen.add(image);
        return true;
      });
      const buildBatches = override.group === "coprocessor" ? deduped.map((service) => [service]) : [deduped];
      for (const batch of buildBatches) {
        await timed(`[build] ${batch.join(",")}`, () => composeBuild(component, batch));
      }
      await saveBuiltImages(
        state,
        imageRefsFromDoc(doc, services).map((ref) => ({ ref, group: override.group })),
      );
    }
  } catch (error) {
    if (error instanceof ContainerStartError) {
      throw new BuildError(error.component, error.stderr);
    }
    throw new BuildError(component, error instanceof Error ? error.message : String(error));
  }
};

const stepComposeUp = async (
  component: string,
  state: State,
  services?: string[],
  options?: { noDeps?: boolean; env?: Record<string, string> },
) => {
  await maybeBuild(component, state);
  await composeUp(component, services, options);
};

const coprocessorHealthContainers = (state: Pick<State, "scenario">) => {
  const topology = topologyForState(state);
  const suffixes = GROUP_SERVICE_SUFFIXES.coprocessor.filter((suffix) => !suffix.includes("migration"));
  const names: string[] = [];
  for (let index = 0; index < topology.count; index += 1) {
    for (const suffix of suffixes) {
      names.push(toServiceName(suffix, index));
    }
  }
  return names;
};

const waitForCoprocessorServices = async (state: State, skipMigration: boolean) => {
  const topology = topologyForState(state);
  for (let index = 0; index < topology.count; index += 1) {
    if (!skipMigration) {
      await waitForContainer(toServiceName("db-migration", index), "complete");
    }
    await waitForContainer(toServiceName("host-listener", index), "running");
    await waitForContainer(toServiceName("host-listener-poller", index), "running");
    await waitForContainer(toServiceName("gw-listener", index), "running");
    await waitForContainer(toServiceName("tfhe-worker", index), "running");
    await waitForContainer(toServiceName("zkproof-worker", index), "running");
    await waitForContainer(toServiceName("sns-worker", index), "running");
    await waitForContainer(toServiceName("transaction-sender", index), "running");
  }
};

const waitForCoprocessor = async (state: State) => waitForCoprocessorServices(state, false);

const coprocessorDbSeeded = async (database: string) => {
  const result = await run(
    ["docker", "exec", COPROCESSOR_DB_CONTAINER, "psql", "-U", "postgres", "-d", database, "-tAc", "select 1 from host_chains limit 1"],
    { allowFailure: true },
  );
  return result.code === 0 && result.stdout.trim() === "1";
};

const coprocessorDbsSeeded = async (state: Pick<State, "scenario">) =>
  (await Promise.all(
    Array.from({ length: topologyForState(state).count }, (_, index) => (index === 0 ? "coprocessor" : `coprocessor_${index}`)).map(coprocessorDbSeeded),
  )).every(Boolean);

const waitForKmsConnector = async () => {
  await waitForContainer("kms-connector-db-migration", "complete");
  await waitForContainer("kms-connector-gw-listener", "running");
  await waitForContainer("kms-connector-kms-worker", "running");
  await waitForContainer("kms-connector-tx-sender", "running");
};

const waitForTestSuite = async () => waitForContainer(TEST_SUITE_CONTAINER, "running");

const runContractTask = async (
  component: "host-sc" | "gateway-sc",
  service: "host-sc-deploy" | "gateway-sc-deploy",
  command: string,
) => {
  const state = await loadState();
  if (!state) {
    throw new PreflightError("Stack is not running; run `fhevm-cli up` first");
  }
  await ensureRuntimeArtifacts(state, "contract task");
  await runStreaming(
    [...dockerArgs(component), "run", "--rm", "--no-deps", "--entrypoint", "sh", service, "-lc", command],
    { env: { ...resolvedComposeEnv(state), ...(await readEnvFileIfExists(envPath(component))) } },
  );
};

export const pause = async (scope: string | undefined) => {
  if (scope === "host") {
    await runContractTask("host-sc", "host-sc-deploy", "npx hardhat compile && npx hardhat task:pauseACL");
    return;
  }
  if (scope === "gateway") {
    await runContractTask("gateway-sc", "gateway-sc-deploy", "npx hardhat compile && npx hardhat task:pauseAllGatewayContracts");
    return;
  }
  throw new PreflightError("pause expects `host` or `gateway`");
};

export const unpause = async (scope: string | undefined) => {
  if (scope === "host") {
    await runContractTask("host-sc", "host-sc-deploy", "npx hardhat compile && npx hardhat task:unpauseACL");
    return;
  }
  if (scope === "gateway") {
    await runContractTask("gateway-sc", "gateway-sc-deploy", "npx hardhat compile && npx hardhat task:unpauseAllGatewayContracts");
    return;
  }
  throw new PreflightError("unpause expects `host` or `gateway`");
};

const runStep = async (state: State, step: StepName) => {
  const stepIndex = stateStepIndex(step) + 1;
  console.log(`[step ${stepIndex}/${STEP_NAMES.length}] ${step}`);
  const stepStarted = Date.now();

  switch (step) {
    case "preflight":
      await preflight(state, true, state.requiresGitHub ?? true);
      break;
    case "resolve":
      printBundle(state.versions);
      break;
    case "generate":
      await renderRuntime(state, runtimePlanForState(state));
      break;
    case "base":
      await stepComposeUp("minio", state);
      await waitForContainer("fhevm-minio", "healthy");
      await waitForContainer("fhevm-minio-setup", "complete");
      await stepComposeUp("core", state);
      await waitForLog(KMS_CORE_CONTAINER, /KMS Server service socket address/);
      await stepComposeUp("database", state);
      await waitForContainer(COPROCESSOR_DB_CONTAINER, "healthy");
      await stepComposeUp("host-node", state);
      await waitForRpc("http://localhost:8545");
      await stepComposeUp("gateway-node", state);
      await waitForRpc("http://localhost:8546");
      state.discovery = createDiscovery(await defaultEndpoints());
      await renderRuntime(state, runtimePlanForState(state));
      break;
    case "kms-signer": {
      const discovery = await ensureDiscovery(state);
      const signer = await discoverSigner();
      discovery.kmsSigner = signer.address;
      discovery.minioKeyPrefix = signer.minioKeyPrefix;
      await renderRuntime(state, runtimePlanForState(state));
      break;
    }
    case "gateway-deploy":
      await stepComposeUp("gateway-mocked-payment", state, ["gateway-deploy-mocked-zama-oft"]);
      await waitForContainer("gateway-deploy-mocked-zama-oft", "complete");
      await stepComposeUp("gateway-sc", state, ["gateway-sc-deploy"]);
      await waitForContainer("gateway-sc-deploy", "complete");
      (await ensureDiscovery(state)).gateway = await readEnvFile(gatewayAddressesPath);
      await renderRuntime(state, runtimePlanForState(state));
      await stepComposeUp("gateway-mocked-payment", state, ["gateway-set-relayer-mocked-payment"], { noDeps: true });
      await waitForContainer("gateway-set-relayer-mocked-payment", "complete");
      break;
    case "host-deploy":
      await stepComposeUp("host-sc", state, ["host-sc-deploy"]);
      await waitForContainer("host-sc-deploy", "complete");
      break;
    case "discover": {
      const contracts = await discoverContracts();
      const discovery = await ensureDiscovery(state);
      discovery.gateway = contracts.gateway;
      discovery.host = contracts.host;
      break;
    }
    case "regenerate":
      await renderRuntime(state, runtimePlanForState(state));
      break;
    case "validate": {
      validateDiscovery(state);
      const incompatibilities = validateBundleCompatibility(state);
      if (incompatibilities.length) {
        throw new IncompatibleVersions(incompatibilities.map((item) => item.message));
      }
      break;
    }
    case "coprocessor": {
      const skipMigration = await coprocessorDbsSeeded(state);
      const services = skipMigration ? coprocessorHealthContainers(state) : serviceNameList(state, "coprocessor");
      await stepComposeUp("coprocessor", state, services, { noDeps: skipMigration });
      await waitForCoprocessorServices(state, skipMigration);
      await postBootHealthGate(coprocessorHealthContainers(state));
      break;
    }
    case "kms-connector":
      await stepComposeUp("kms-connector", state);
      await waitForKmsConnector();
      await postBootHealthGate(KMS_CONNECTOR_HEALTH_CONTAINERS);
      break;
    case "bootstrap": {
      const bootstrapDone = await probeBootstrap(state).catch((error) => (error instanceof MinioError ? null : Promise.reject(error)));
      if (bootstrapDone) {
        state.discovery!.actualFheKeyId = bootstrapDone.actualFheKeyId;
        state.discovery!.actualCrsKeyId = bootstrapDone.actualCrsKeyId;
        await renderRuntime(state, runtimePlanForState(state));
        break;
      }
      const [hostEnv, gatewayEnv] = await Promise.all([readEnvFile(envPath("host-sc")), readEnvFile(envPath("gateway-sc"))]);
      const hostChainsRegistered = (
        await Promise.all(
          Array.from({ length: Number(gatewayEnv.NUM_HOST_CHAINS ?? "0") }, (_, index) => gatewayEnv[`HOST_CHAIN_CHAIN_ID_${index}`])
            .filter(Boolean)
            .map((chainId) =>
              castBool(
                state.discovery!.endpoints.gatewayHttp,
                withHexPrefix(state.discovery!.gateway.GATEWAY_CONFIG_ADDRESS),
                "isHostChainRegistered(uint256)(bool)",
                chainId,
              ).catch(() => false),
            ),
        )
      ).every(Boolean);
      if (!hostChainsRegistered) {
        await timed("[bootstrap] add-network", () =>
          stepComposeUp("gateway-sc", state, ["gateway-sc-add-network"], { noDeps: true }),
        );
        await waitForContainer("gateway-sc-add-network", "complete");
      }
      const bootstrapReady = await probeBootstrap(state).catch((error) => (error instanceof MinioError ? null : Promise.reject(error)));
      if (bootstrapReady) {
        state.discovery!.actualFheKeyId = bootstrapReady.actualFheKeyId;
        state.discovery!.actualCrsKeyId = bootstrapReady.actualCrsKeyId;
        await renderRuntime(state, runtimePlanForState(state));
        break;
      }
      const hostPauserRegistered = await castBool(
        state.discovery!.endpoints.hostHttp,
        withHexPrefix(state.discovery!.host.PAUSER_SET_CONTRACT_ADDRESS),
        "isPauser(address)(bool)",
        withHexPrefix(hostEnv.PAUSER_ADDRESS_0),
      ).catch(() => false);
      if (!hostPauserRegistered) {
        await timed("[bootstrap] add-host-pausers", () =>
          stepComposeUp("host-sc", state, ["host-sc-add-pausers"], { noDeps: true }),
        );
        await waitForContainer("host-sc-add-pausers", "complete");
      }
      const gatewayPauserRegistered = await castBool(
        state.discovery!.endpoints.gatewayHttp,
        withHexPrefix(gatewayEnv.PAUSER_SET_ADDRESS),
        "isPauser(address)(bool)",
        withHexPrefix(gatewayEnv.PAUSER_ADDRESS_0),
      ).catch(() => false);
      if (!gatewayPauserRegistered) {
        await timed("[bootstrap] add-gateway-pausers", () =>
          stepComposeUp("gateway-sc", state, ["gateway-sc-add-pausers"], { noDeps: true }),
        );
        await waitForContainer("gateway-sc-add-pausers", "complete");
      }
      await timed("[bootstrap] trigger-keygen", () =>
        stepComposeUp("gateway-sc", state, ["gateway-sc-trigger-keygen"], { noDeps: true }),
      );
      await waitForContainer("gateway-sc-trigger-keygen", "complete");
      await timed("[bootstrap] trigger-crsgen", () =>
        stepComposeUp("gateway-sc", state, ["gateway-sc-trigger-crsgen"], { noDeps: true }),
      );
      await waitForContainer("gateway-sc-trigger-crsgen", "complete");
      await timed("[bootstrap] wait-for-materials", () => waitForBootstrap(state));
      await renderRuntime(state, runtimePlanForState(state));
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

const describeResumeState = (state: State) => {
  const topology = topologyForState(state);
  const overrides = visibleOverrides(state);
  return [
    `profile=${state.target}`,
    `topology=${topology.count}/${topology.threshold}`,
    ...(state.scenario.origin !== "default"
      ? [`scenario=${state.scenario.origin}${state.scenario.sourcePath ? `:${state.scenario.sourcePath}` : ""}`]
      : []),
    ...(overrides.length ? [`overrides=${overrides.map(describeOverride).join(", ")}`] : []),
  ].join(" ");
};

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

const startStep = (state: State, options: Pick<UpOptions, "resume" | "fromStep">): StepName => {
  if (options.fromStep) {
    return options.fromStep;
  }
  if (!options.resume || !state.completedSteps.length) {
    return STEP_NAMES[0];
  }
  return STEP_NAMES.find((step) => !state.completedSteps.includes(step)) ?? STEP_NAMES[STEP_NAMES.length - 1];
};

const targetNeedsGitHub = (options: Pick<UpOptions, "target" | "lockFile">) =>
  !options.lockFile && options.target !== "latest-supported";

export const previewStateFromBundle = (
  options: Pick<UpOptions, "overrides" | "lockFile">,
  bundle: VersionBundle,
  scenario: State["scenario"],
): State => ({
  target: bundle.target,
  lockPath: "",
  requiresGitHub: targetNeedsGitHub({ target: bundle.target, lockFile: options.lockFile }),
  versions: bundle,
  overrides: options.overrides,
  scenario,
  scenarioSourcePath: scenario.sourcePath,
  completedSteps: [],
  updatedAt: new Date().toISOString(),
});

const bootstrapState = async (options: UpOptions) => {
  console.log(`[up] target=${options.target}`);
  const scenario = await resolveScenarioForOptions(options);
  const resolveStarted = Date.now();
  const resolved = await resolveBundle(options, process.env);
  console.log(`[resolve] bundle ready (${Math.round((Date.now() - resolveStarted) / 1000)}s)`);
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

export const up = async (options: UpOptions) => {
  const started = Date.now();
  const persistedState = await loadState();
  let state = options.resume ? persistedState : undefined;
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
    state.requiresGitHub ??= state.target !== "latest-supported";
    state.scenarioSourcePath ??= state.scenario?.sourcePath;
    ensureResumeOptions(state, options);
    await ensureRuntimeArtifacts(state, "resume");
    const running = await projectContainers();
    if (!running.length && !options.fromStep) {
      console.log("[resume] stack is stopped; restarting from base");
      state.completedSteps = [];
      await saveState(state);
    } else if (!options.fromStep && STEP_NAMES.every((step) => state!.completedSteps.includes(step))) {
      console.log("[resume] nothing to do");
      return;
    }
  }
  for (const warning of overrideWarnings(visibleOverrides(state), state.target)) {
    console.log(`[warn] ${warning}`);
  }
  if (options.resume && options.fromStep) {
    await resetAfterStep(options.fromStep);
    state.completedSteps = state.completedSteps.filter((step) => stateStepIndex(step) < stateStepIndex(options.fromStep!));
    await saveState(state);
  }
  const from = startStep(state, options);
  for (const step of STEP_NAMES.slice(stateStepIndex(from))) {
    if (options.resume && state.completedSteps.includes(step) && !options.fromStep) {
      continue;
    }
    await runStep(state, step);
  }
  console.log(`[done] stack ready in ${Math.round((Date.now() - started) / 1000)}s`);
};

export const upDryRun = async (options: Omit<UpOptions, "dryRun">) => {
  if (options.resume) {
    const state = await loadState();
    if (!state) {
      throw new ResumeError("No .fhevm/state/state.json to resume from");
    }
    state.requiresGitHub ??= state.target !== "latest-supported";
    state.scenarioSourcePath ??= state.scenario?.sourcePath;
    ensureResumeOptions(state, options);
    await preflight(state, false, state.requiresGitHub);
    printBundle(state.versions, { detailed: true });
    printPlan(state, options.fromStep ?? startStep(state, options));
    console.log("[dry-run] resume preview uses persisted state only; no state or containers were changed");
    return;
  }
  console.log(`[up] target=${options.target}`);
  const scenario = await resolveScenarioForOptions(options);
  const bundle = await previewBundle(options, process.env);
  await assertSchemaCompatibility(bundle, options.overrides, scenario, options.allowSchemaMismatch);
  const state = previewStateFromBundle(options, bundle, scenario);
  await preflight(state, true, state.requiresGitHub);
  printBundle(state.versions, { detailed: true });
  printPlan(state, options.fromStep);
  console.log("[dry-run] preflight passed; no state or containers were changed");
};

const pruneGeneratedRuntimeArtifacts = async () => {
  const targets = [ENV_DIR, COMPOSE_OUT_DIR, GENERATED_CONFIG_DIR, ADDRESS_DIR];
  await Promise.all(targets.map(async (target) => {
    if (await exists(target)) {
      await remove(target);
    }
  }));
};

export const down = async () => {
  const state = await loadState();
  const existing = await projectContainers(true);
  if (!existing.length) {
    console.log("[down] nothing to stop");
    if (state) {
      await pruneGeneratedRuntimeArtifacts();
    }
    return;
  }
  if (state) {
    await ensureRuntimeArtifacts(state, "teardown");
  }
  const failed: string[] = [];
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
  await pruneGeneratedRuntimeArtifacts();
};

export const clean = async (options: CleanOptions) => {
  console.log("[clean] start");
  const state = await loadState();
  await down();
  if (options.images && state?.builtImages?.length) {
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
    console.log(`[topology] n=${topology.count} t=${topology.threshold}`);
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
    throw new PreflightError(ps.stderr.trim() || "docker ps failed");
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
    throw new PreflightError(running.stderr.trim() || "docker ps failed");
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
      throw new PreflightError(all.stderr.trim() || "docker ps -a failed");
    }
    containers = pickContainers(all.stdout);
  }
  if (!containers.length) {
    throw new PreflightError(`No containers match ${service ?? "fhevm"}`);
  }
  const exactMatch = requested
    ? containers.find((item) => item === requested) ?? containers.find((item) => item.endsWith(`-${requested}`))
    : undefined;
  if (requested && !exactMatch && containers.length > 1) {
    throw new PreflightError(`Multiple containers match ${service}: ${containers.join(", ")}`);
  }
  const container = !requested ? containers[0] : exactMatch ?? containers[0];
  await runStreaming(["docker", "logs", ...(options.follow ? ["--follow"] : []), "--tail", "200", container]);
};

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

export const upgrade = async (groupValue: string | undefined) => {
  const state = await loadState();
  if (!state || !(await projectContainers()).length) {
    throw new PreflightError(
      "Stack is not running; start one with `fhevm-cli up --override ...` or `fhevm-cli up --scenario ...` first",
    );
  }
  await ensureRuntimeArtifacts(state, "upgrade");
  const { component, group, runtimeServices, step } = resolveUpgradePlan(state, groupValue);
  if (!state.completedSteps.includes(step)) {
    throw new PreflightError(`upgrade requires a stack that has completed the ${step} step`);
  }
  await assertSchemaCompatibility(state.versions, state.overrides, state.scenario, false);
  await assertUpgradeSchemaStable(state.versions, group);
  console.log(`[upgrade] ${group}`);
  await renderRuntime(state, runtimePlanForState(state));
  await maybeBuild(component, state, { force: true });
  await composeUp(component, runtimeServices, { noDeps: true });
  if (group === "coprocessor") {
    await waitForCoprocessor(state);
  } else if (group === "kms-connector") {
    await waitForKmsConnector();
  } else {
    await waitForContainer(TEST_SUITE_CONTAINER, "running");
  }
  await markStep(state, step);
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

export const shouldShowResumeHint = (rawArgs: string[]) =>
  !rawArgs.includes("--dry-run") && !rawArgs.some((arg) => RESUME_HINT_BLOCKERS.has(arg));

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
