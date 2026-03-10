import path from "node:path";
import { parseArgs } from "node:util";

import { requiresMultichainAclAddress } from "./compat";
import type {
  Discovery,
  InstanceOverride,
  LocalOverride,
  OverrideGroup,
  State,
  StepName,
  Topology,
  VersionBundle,
} from "./types";
import { OVERRIDE_GROUPS, STEP_NAMES, TARGETS } from "./types";
import { composeDown, composeUp, inspectImageId, regen, resolvedComposeEnv, serviceNameList } from "./artifacts";
import {
  COMPONENT_BY_STEP,
  COMPONENTS,
  GROUP_BUILD_COMPONENTS,
  GROUP_BUILD_SERVICES,
  GROUP_SERVICE_SUFFIXES,
  SCHEMA_COUPLED_GROUPS,
  LOCK_DIR,
  LOG_TARGETS,
  PORTS,
  PROJECT,
  STATE_DIR,
  STATE_FILE,
  TEST_GREP,
  composePath,
  dockerArgs,
  envPath,
  gatewayAddressesPath,
  hostAddressesPath,
  resolveServiceOverrides,
} from "./layout";
import type { Runner } from "./utils";
import {
  exists,
  predictedCrsId,
  predictedKeyId,
  readEnvFile,
  readJson,
  remove,
  run,
  runLive,
  sleep,
  toServiceName,
  toError,
  withHexPrefix,
  writeJson,
} from "./utils";
import { applyVersionEnvOverrides, createGitHubClient, describeBundle, resolveTarget } from "./versions";

type UpOptions = {
  target: (typeof TARGETS)[number];
  sha?: string;
  overrides: LocalOverride[];
  topology: Topology;
  fromStep?: StepName;
  lockFile?: string;
  resume: boolean;
  dryRun: boolean;
};

type CleanOptions = {
  images: boolean;
};

type RuntimeDeps = {
  runner: Runner;
  liveRunner: typeof runLive;
  now: () => string;
  fetch: typeof globalThis.fetch;
  env: Record<string, string | undefined>;
};

const defaultDeps: RuntimeDeps = {
  runner: run,
  liveRunner: runLive,
  now: () => new Date().toISOString(),
  fetch: globalThis.fetch,
  env: process.env,
};

const UPGRADEABLE_GROUPS = ["coprocessor", "kms-connector", "test-suite"] as const;
type UpgradeGroup = (typeof UPGRADEABLE_GROUPS)[number];

const dockerInspect = async (runner: Runner, name: string) => {
  const result = await runner(["docker", "inspect", name], {
    allowFailure: true,
  });
  try {
    return JSON.parse(result.stdout || "[]") as Array<{
      Name: string;
      State: { Status: string; ExitCode: number; Health?: { Status: string } };
      NetworkSettings: { Networks: Record<string, { IPAddress: string }> };
    }>;
  } catch (error) {
    throw new Error(`docker inspect ${name} returned invalid JSON: ${toError(error).message}\n${result.stdout || result.stderr || "<empty>"}`);
  }
};

const loadState = async () => (await exists(STATE_FILE) ? readJson<State>(STATE_FILE) : undefined);
const saveState = async (state: State) => writeJson(STATE_FILE, state);

const log = (value: string) => console.log(value);

const createTopology = (count: number, threshold?: number, instances?: Record<string, InstanceOverride>): Topology => ({
  count,
  threshold: threshold ?? count,
  instances: instances ?? {},
});

const parseLocalOverride = (value: string): LocalOverride => {
  const colonIdx = value.indexOf(":");
  if (colonIdx < 0) {
    if (!OVERRIDE_GROUPS.includes(value as OverrideGroup)) {
      throw new Error(`Unsupported override ${value}`);
    }
    return { group: value as OverrideGroup };
  }
  const group = value.slice(0, colonIdx);
  const rest = value.slice(colonIdx + 1);
  if (!OVERRIDE_GROUPS.includes(group as OverrideGroup)) {
    throw new Error(`Unsupported override group "${group}"`);
  }
  const overrideGroup = group as OverrideGroup;
  const parts = rest
    .split(",")
    .map((part) => part.trim())
    .filter(Boolean);
  if (!parts.length) {
    throw new Error(`Expected at least one service name in override "${value}"`);
  }
  return { group: overrideGroup, services: resolveServiceOverrides(overrideGroup, parts) };
};

const parseKeyValue = (value: string) => {
  const idx = value.indexOf("=");
  if (idx < 0) {
    throw new Error(`Expected KEY=VALUE, got ${value}`);
  }
  return [value.slice(0, idx), value.slice(idx + 1)] as const;
};

const parseInstanceKey = (value: string) => {
  const idx = value.indexOf(":");
  if (idx < 0) {
    throw new Error(`Expected INDEX:VALUE, got ${value}`);
  }
  const index = Number(value.slice(0, idx));
  if (!Number.isInteger(index) || index < 0) {
    throw new Error(`Invalid instance index in ${value}`);
  }
  return [index, value.slice(idx + 1)] as const;
};

const parseInstanceEnv = (values: string[]) => {
  const out: Record<string, InstanceOverride> = {};
  for (const value of values) {
    const [index, payload] = parseInstanceKey(value);
    const [key, envValue] = parseKeyValue(payload);
    const name = `coprocessor-${index}`;
    out[name] ??= { env: {}, args: {} };
    out[name].env[key] = envValue;
  }
  return out;
};

const parseInstanceArgs = (values: string[]) => {
  const out: Record<string, InstanceOverride> = {};
  for (const value of values) {
    const [index, payload] = parseInstanceKey(value);
    const [service, arg] = parseKeyValue(payload);
    const name = `coprocessor-${index}`;
    out[name] ??= { env: {}, args: {} };
    out[name].args[service] ??= [];
    out[name].args[service].push(arg);
  }
  return out;
};

const mergeInstanceOverrides = (...items: Record<string, InstanceOverride>[]) => {
  const out: Record<string, InstanceOverride> = {};
  for (const item of items) {
    for (const [name, override] of Object.entries(item)) {
      out[name] ??= { env: {}, args: {} };
      Object.assign(out[name].env, override.env);
      for (const [service, args] of Object.entries(override.args)) {
        out[name].args[service] = [...(out[name].args[service] ?? []), ...args];
      }
    }
  }
  return out;
};

const parseCli = (argv: string[]) => {
  const command = argv[2];
  const parsed = parseArgs({
    args: argv.slice(3),
    allowPositionals: true,
    options: {
      target: { type: "string", default: "latest-release" },
      sha: { type: "string" },
      override: { type: "string", multiple: true, default: [] },
      coprocessors: { type: "string", default: "1" },
      threshold: { type: "string" },
      "from-step": { type: "string" },
      "lock-file": { type: "string" },
      resume: { type: "boolean", default: false },
      "dry-run": { type: "boolean", default: false },
      images: { type: "boolean", default: false },
      grep: { type: "string" },
      network: { type: "string", default: "staging" },
      verbose: { type: "boolean", default: false },
      "instance-env": { type: "string", multiple: true, default: [] },
      "instance-arg": { type: "string", multiple: true, default: [] },
    },
  });
  const target = parsed.values.target as string;
  if (!TARGETS.includes(target as (typeof TARGETS)[number])) {
    throw new Error(`Unsupported target ${target}`);
  }
  const sha = parsed.values.sha as string | undefined;
  if (target === "sha" && !sha) {
    throw new Error("--target sha requires --sha");
  }
  if (target !== "sha" && sha) {
    throw new Error("--sha requires --target sha");
  }
  if (sha && parsed.values["lock-file"]) {
    throw new Error("--sha cannot be used with --lock-file");
  }
  const count = Number(parsed.values.coprocessors);
  const threshold = parsed.values.threshold ? Number(parsed.values.threshold) : undefined;
  if (!Number.isInteger(count) || count < 1 || count > 5) {
    throw new Error("--coprocessors must be between 1 and 5");
  }
  if (threshold !== undefined && (!Number.isInteger(threshold) || threshold < 1 || threshold > count)) {
    throw new Error("--threshold must be between 1 and --coprocessors");
  }
  const overrides = (parsed.values.override as string[]).map(parseLocalOverride);
  const topology = createTopology(
    count,
    threshold,
    mergeInstanceOverrides(
      parseInstanceEnv(parsed.values["instance-env"] as string[]),
      parseInstanceArgs(parsed.values["instance-arg"] as string[]),
    ),
  );
  return { command, parsed, target: target as UpOptions["target"], sha, overrides, topology };
};

const stateStepIndex = (step: StepName) => STEP_NAMES.indexOf(step);

const ensureStep = (value?: string) => {
  if (!value) {
    return undefined;
  }
  if (!STEP_NAMES.includes(value as StepName)) {
    throw new Error(`Unknown step ${value}`);
  }
  return value as StepName;
};

const writeLock = async (bundle: VersionBundle) => {
  const file = path.join(LOCK_DIR, bundle.lockName);
  await writeJson(file, bundle);
  return file;
};

const bundleFromFile = async (target: UpOptions["target"], lockFile: string) => {
  const bundle = await readJson<VersionBundle>(path.resolve(lockFile));
  if (bundle.target && bundle.target !== target) {
    throw new Error(`Lock file target mismatch: bundle=${bundle.target}, requested=${target}`);
  }
  return { ...bundle, target };
};

const resolveBundle = async (options: Pick<UpOptions, "target" | "sha" | "lockFile">, deps: RuntimeDeps) => {
  const bundle = options.lockFile
    ? await bundleFromFile(options.target, options.lockFile)
    : await resolveTarget(options.target, createGitHubClient(deps.runner), { sha: options.sha });
  const resolved = applyVersionEnvOverrides(bundle, deps.env);
  const lockPath = await writeLock(resolved);
  return { bundle: resolved, lockPath };
};

const previewBundle = (options: Pick<UpOptions, "target" | "sha" | "lockFile">, deps: RuntimeDeps) =>
  (options.lockFile
    ? bundleFromFile(options.target, options.lockFile)
    : resolveTarget(options.target, createGitHubClient(deps.runner), { sha: options.sha })).then((bundle) =>
    applyVersionEnvOverrides(bundle, deps.env),
  );

const markStep = async (state: State, step: StepName, deps: RuntimeDeps) => {
  if (!state.completedSteps.includes(step)) {
    state.completedSteps.push(step);
  }
  state.updatedAt = deps.now();
  await saveState(state);
};

const waitForContainer = async (
  deps: RuntimeDeps,
  container: string,
  want: "running" | "healthy" | "complete",
  marker?: string,
) => {
  for (let attempt = 0; attempt < 90; attempt += 1) {
    const [inspect] = await dockerInspect(deps.runner, container);
    if (inspect) {
      if (attempt === 2 || (attempt > 0 && attempt % 10 === 0)) {
        const health = inspect.State.Health?.Status ? `/${inspect.State.Health.Status}` : "";
        const suffix = marker && inspect.State.Status === "running" ? ` waiting for ${marker}` : "";
        log(`[wait] ${container} ${inspect.State.Status}${health}${suffix}`);
      }
      if (want === "running" && inspect.State.Status === "running") {
        return;
      }
      if (want === "healthy" && inspect.State.Health?.Status === "healthy") {
        return;
      }
      if (want === "complete" && inspect.State.Status === "exited" && inspect.State.ExitCode === 0) {
        return;
      }
      if (want === "complete" && inspect.State.Status === "running" && marker) {
        const logs = await deps.runner(["docker", "logs", container], { allowFailure: true });
        if (logs.stdout.includes(marker) || logs.stderr.includes(marker)) {
          await deps.runner(["docker", "stop", container], { allowFailure: true });
          return;
        }
      }
      if (inspect.State.Status === "exited" && inspect.State.ExitCode !== 0) {
        const logs = await deps.runner(["docker", "logs", container], { allowFailure: true });
        throw new Error(`${container} failed\n${logs.stdout}${logs.stderr}`);
      }
    }
    await sleep(2000);
  }
  throw new Error(`Timed out waiting for ${container} (${want})`);
};

const waitForRpc = async (deps: RuntimeDeps, url: string) => {
  for (let attempt = 0; attempt < 60; attempt += 1) {
    try {
      const response = await deps.fetch(url, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ jsonrpc: "2.0", id: 1, method: "eth_chainId", params: [] }),
      });
      if (response.ok) {
        return;
      }
    } catch {}
    if (attempt === 2 || (attempt > 0 && attempt % 10 === 0)) {
      log(`[wait] rpc ${url}`);
    }
    await sleep(1000);
  }
  throw new Error(`RPC not ready: ${url}`);
};

const minioIp = async (deps: RuntimeDeps) => {
  const [inspect] = await dockerInspect(deps.runner, "fhevm-minio");
  const ip = inspect ? Object.values(inspect.NetworkSettings.Networks)[0]?.IPAddress : "";
  if (!ip) {
    throw new Error("Could not determine MinIO IP");
  }
  return ip;
};

const discoverSigner = async (deps: RuntimeDeps) => {
  const logs = await deps.runner(["docker", "logs", "kms-core"], { allowFailure: true });
  const match = logs.stdout.match(/handle ([a-zA-Z0-9]+)/) ?? logs.stderr.match(/handle ([a-zA-Z0-9]+)/);
  if (!match) {
    throw new Error("Could not extract KMS signer handle");
  }
  const response = await deps.fetch(`http://localhost:9000/kms-public/PUB/VerfAddress/${match[1]}`);
  if (!response.ok) {
    throw new Error("Could not fetch KMS signer address");
  }
  return (await response.text()).trim();
};

const waitForKmsCore = async (deps: RuntimeDeps) => {
  for (let attempt = 0; attempt < 90; attempt += 1) {
    const [inspect] = await dockerInspect(deps.runner, "kms-core");
    if (inspect?.State.Status === "exited") {
      const logs = await deps.runner(["docker", "logs", "kms-core"], { allowFailure: true });
      throw new Error(`kms-core failed\n${logs.stdout}${logs.stderr}`);
    }
    const logs = await deps.runner(["docker", "logs", "kms-core"], { allowFailure: true });
    if ((logs.stdout + logs.stderr).includes("KMS Server service socket address")) {
      return;
    }
    if (attempt === 2 || (attempt > 0 && attempt % 10 === 0)) {
      log("[wait] kms-core");
    }
    await sleep(1000);
  }
  throw new Error("Timed out waiting for kms-core readiness");
};

const discoverContracts = async (deps: RuntimeDeps): Promise<Pick<Discovery, "gateway" | "host">> => {
  if (!(await exists(gatewayAddressesPath)) || !(await exists(hostAddressesPath))) {
    throw new Error("Missing generated address files under .fhevm/addresses");
  }
  return {
    gateway: await readEnvFile(gatewayAddressesPath),
    host: await readEnvFile(hostAddressesPath),
  };
};

const validateDiscovery = (state: Pick<State, "target" | "versions" | "discovery">) => {
  const discovery = state.discovery;
  if (!discovery) {
    throw new Error("Missing discovery state");
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
      throw new Error(`Missing gateway discovery value ${key}`);
    }
  }
  for (const key of requiredHost) {
    if (!discovery.host[key]) {
      throw new Error(`Missing host discovery value ${key}`);
    }
  }
  if (!discovery.kmsSigner) {
    throw new Error("Missing KMS signer discovery");
  }
  if (!discovery.fheKeyId || !discovery.crsKeyId) {
    throw new Error("Missing predicted key ids");
  }
};

const ethCallId = async (deps: RuntimeDeps, url: string, to: string, data: string) => {
  const rpcUrl = hostReachableUrl(url);
  const response = await deps.fetch(rpcUrl, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      jsonrpc: "2.0",
      id: 1,
      method: "eth_call",
      params: [{ to, data }, "latest"],
    }),
  });
  if (!response.ok) {
    throw new Error(`eth_call failed for ${data}`);
  }
  const payload = (await response.json()) as { result?: string };
  if (!payload.result) {
    throw new Error(`Missing eth_call result for ${data}`);
  }
  return BigInt(payload.result);
};

const hostReachableUrl = (url: string) =>
  url.replace("http://gateway-node:8546", "http://localhost:8546").replace("http://host-node:8545", "http://localhost:8545");

const hostReachableMaterialUrl = (url: string) =>
  url.replace(/http:\/\/[^/]+:9000/, "http://localhost:9000").replace("http://minio:9000", "http://localhost:9000");

const shellEscape = (value: string) => `'${value.replaceAll("'", `'\\''`)}'`;

const probeBootstrap = async (state: State, deps: RuntimeDeps) => {
  const gateway = withHexPrefix(state.discovery!.gateway.KMS_GENERATION_ADDRESS);
  let actualKey = 0n;
  let actualCrs = 0n;
  try {
    actualKey = await ethCallId(deps, state.discovery!.endpoints.gatewayHttp, gateway, "0xd52f10eb");
    actualCrs = await ethCallId(deps, state.discovery!.endpoints.gatewayHttp, gateway, "0xbaff211e");
  } catch {
    return false;
  }
  if (actualKey === 0n || actualCrs === 0n) {
    return false;
  }
  const actualFheKeyId = actualKey.toString(16).padStart(64, "0");
  const actualCrsKeyId = actualCrs.toString(16).padStart(64, "0");
  await ensureMaterialUrl(
    deps,
    hostReachableMaterialUrl(`${state.discovery!.endpoints.minioExternal}/kms-public/PUB/PublicKey/${actualFheKeyId}`),
  );
  await ensureMaterialUrl(
    deps,
    hostReachableMaterialUrl(`${state.discovery!.endpoints.minioExternal}/kms-public/PUB/CRS/${actualCrsKeyId}`),
  );
  state.discovery!.actualFheKeyId = actualFheKeyId;
  state.discovery!.actualCrsKeyId = actualCrsKeyId;
  if (state.discovery!.fheKeyId !== actualFheKeyId || state.discovery!.crsKeyId !== actualCrsKeyId) {
    throw new Error(
      `Predicted bootstrap ids drifted: expected ${state.discovery!.fheKeyId}/${state.discovery!.crsKeyId}, got ${actualFheKeyId}/${actualCrsKeyId}`,
    );
  }
  return true;
};

const waitForBootstrap = async (state: State, deps: RuntimeDeps, attempts = 120) => {
  for (let attempt = 0; attempt < attempts; attempt += 1) {
    if (await probeBootstrap(state, deps)) {
      return;
    }
    if (attempt === 2 || (attempt > 0 && attempt % 10 === 0)) {
      log("[wait] bootstrap materials");
    }
    await sleep(2000);
  }
  throw new Error("Bootstrap did not activate FHE key and CRS in time");
};

const castBool = async (runner: Runner, rpcUrl: string, to: string, signature: string, ...args: string[]) => {
  const result = (await runner(["cast", "call", to, signature, ...args, "--rpc-url", hostReachableUrl(rpcUrl)])).stdout.trim();
  return result === "true" || result === "0x1" || result === "0x0000000000000000000000000000000000000000000000000000000000000001";
};

const pauserRegistered = async (deps: RuntimeDeps, rpcUrl: string, contract: string, account: string, signature: string) =>
  castBool(deps.runner, rpcUrl, withHexPrefix(contract), signature, withHexPrefix(account));

const ensureMaterialUrl = async (deps: RuntimeDeps, url: string) => {
  for (let attempt = 0; attempt < 30; attempt += 1) {
    const response = await deps.fetch(url, { method: "HEAD" });
    if (response.ok) {
      return;
    }
    await sleep(1000);
  }
  throw new Error(`Material endpoint not ready: ${url}`);
};

const waitForRelayer = async (deps: RuntimeDeps) => {
  for (let attempt = 0; attempt < 60; attempt += 1) {
    const logs = await deps.runner(["docker", "logs", "fhevm-relayer"], { allowFailure: true });
    if ((logs.stdout + logs.stderr).includes("All servers are ready and responding")) {
      return;
    }
    if (attempt === 2 || (attempt > 0 && attempt % 10 === 0)) {
      log("[wait] fhevm-relayer");
    }
    await sleep(2000);
  }
  throw new Error("Relayer did not report ready");
};

const resetAfterStep = async (step: StepName, deps: RuntimeDeps) => {
  const start = stateStepIndex(step);
  for (let index = STEP_NAMES.length - 1; index >= start; index -= 1) {
    for (const component of COMPONENT_BY_STEP[STEP_NAMES[index]]) {
      await composeDown(component, deps);
    }
  }
};

const preflight = async (state: State, deps: RuntimeDeps, strictPorts = true, needsGitHub = true) => {
  for (const cmd of ["bun", "docker", ...(needsGitHub ? ["gh"] : [])]) {
    await deps.runner(["which", cmd]);
  }
  if (state.topology.count > 1) {
    await deps.runner(["which", "cast"]);
  }
  const projectPorts = await deps.runner(
    ["docker", "ps", "--filter", `label=com.docker.compose.project=${PROJECT}`, "--format", "{{.Ports}}"],
    { allowFailure: true },
  );
  for (const port of PORTS) {
    const busy = await deps.runner(["lsof", "-nP", `-iTCP:${port}`, "-sTCP:LISTEN"], { allowFailure: true });
    if (busy.code === 0 && busy.stdout.trim() && !projectPorts.stdout.includes(`:${port}->`)) {
      const message = `Port ${port} is already in use\n${busy.stdout}`;
      if (strictPorts) {
        throw new Error(message);
      }
      log(`[preflight] warning: ${message}`);
    }
  }
};

const bootstrapState = async (options: UpOptions, deps: RuntimeDeps) => {
  const resolved = await resolveBundle(options, deps);
  const state: State = {
    target: options.target,
    lockPath: resolved.lockPath,
    versions: resolved.bundle,
    overrides: options.overrides,
    topology: options.topology,
    completedSteps: [],
    updatedAt: deps.now(),
  };
  await saveState(state);
  return state;
};

const printBundle = (bundle: VersionBundle) => {
  log(`[resolve] ${bundle.lockName}`);
  log(describeBundle(bundle));
};

const describeOverride = (item: LocalOverride) =>
  `${item.group}${item.services?.length ? `[${item.services.join(",")}]` : ""}`;

export const overrideWarnings = (overrides: LocalOverride[]) =>
  overrides.flatMap((item) =>
    item.services?.length && SCHEMA_COUPLED_GROUPS.includes(item.group)
      ? [
          `${item.group}: per-service override with a shared database. ` +
            `If your changes include DB migrations, non-overridden services may fail. ` +
            `Use --override ${item.group} (full group) in that case.`,
        ]
      : [],
  );

const logOverrideWarnings = (overrides: LocalOverride[]) => {
  for (const warning of overrideWarnings(overrides)) {
    log(`[warn] ${warning}`);
  }
};

const printPlan = (state: Pick<State, "target" | "overrides" | "topology">, fromStep?: StepName) => {
  log(`[plan] target=${state.target}`);
  if (state.overrides.length) {
    log(`[plan] overrides=${state.overrides.map(describeOverride).join(", ")}`);
    logOverrideWarnings(state.overrides);
  }
  log(`[plan] topology=n${state.topology.count}/t${state.topology.threshold}`);
  log(`[plan] steps=${STEP_NAMES.slice(stateStepIndex(fromStep ?? STEP_NAMES[0])).join(" -> ")}`);
};

const waitForCoprocessor = async (state: State, deps: RuntimeDeps) => {
  for (let index = 0; index < state.topology.count; index += 1) {
    await waitForContainer(deps, toServiceName("db-migration", index), "complete");
    await waitForContainer(deps, toServiceName("host-listener", index), "running");
    await waitForContainer(deps, toServiceName("gw-listener", index), "running");
    await waitForContainer(deps, toServiceName("tfhe-worker", index), "running");
    await waitForContainer(deps, toServiceName("zkproof-worker", index), "running");
    await waitForContainer(deps, toServiceName("sns-worker", index), "running");
    await waitForContainer(deps, toServiceName("transaction-sender", index), "running");
  }
};

const waitForKmsConnector = async (deps: RuntimeDeps) => {
  await waitForContainer(deps, "kms-connector-db-migration", "complete");
  await waitForContainer(deps, "kms-connector-gw-listener", "running");
  await waitForContainer(deps, "kms-connector-kms-worker", "running");
  await waitForContainer(deps, "kms-connector-tx-sender", "running");
};

const waitForTestSuite = async (deps: RuntimeDeps) => {
  await waitForContainer(deps, "fhevm-test-suite-e2e-debug", "running");
};

const coprocessorServicesForOverrides = (state: Pick<State, "topology">, services?: string[]) => {
  if (!services?.length) {
    return serviceNameList(state, "coprocessor");
  }
  const suffixes = [...new Set(services.map((service) => service.replace(/^coprocessor-/, "")))];
  const names: string[] = [];
  for (let index = 0; index < state.topology.count; index += 1) {
    for (const suffix of suffixes) {
      names.push(toServiceName(suffix, index));
    }
  }
  return names;
};

const runStep = async (state: State, step: StepName, deps: RuntimeDeps) => {
  log(`[step] ${step}`);
  switch (step) {
    case "preflight":
      await preflight(state, deps);
      break;
    case "resolve":
      printBundle(state.versions);
      break;
    case "generate":
      await regen(state, deps);
      break;
    case "base":
      await composeUp("minio", state, deps, saveState, log);
      await waitForContainer(deps, "fhevm-minio", "healthy");
      await waitForContainer(deps, "fhevm-minio-setup", "complete");
      await composeUp("core", state, deps, saveState, log);
      await waitForKmsCore(deps);
      await composeUp("database", state, deps, saveState, log);
      await waitForContainer(deps, "coprocessor-and-kms-db", "healthy");
      await composeUp("host-node", state, deps, saveState, log);
      await waitForRpc(deps, "http://localhost:8545");
      await composeUp("gateway-node", state, deps, saveState, log);
      await waitForRpc(deps, "http://localhost:8546");
      state.discovery = {
        gateway: {},
        host: {},
        kmsSigner: "",
        fheKeyId: predictedKeyId(),
        crsKeyId: predictedCrsId(),
        endpoints: {
          gatewayHttp: "http://gateway-node:8546",
          gatewayWs: "ws://gateway-node:8546",
          hostHttp: "http://host-node:8545",
          hostWs: "ws://host-node:8545",
          minioInternal: "http://minio:9000",
          minioExternal: `http://${await minioIp(deps)}:9000`,
        },
      };
      await regen(state, deps);
      break;
    case "kms-signer":
      state.discovery ??= {
        gateway: {},
        host: {},
        kmsSigner: "",
        fheKeyId: predictedKeyId(),
        crsKeyId: predictedCrsId(),
        endpoints: {
          gatewayHttp: "http://gateway-node:8546",
          gatewayWs: "ws://gateway-node:8546",
          hostHttp: "http://host-node:8545",
          hostWs: "ws://host-node:8545",
          minioInternal: "http://minio:9000",
          minioExternal: `http://${await minioIp(deps)}:9000`,
        },
      };
      state.discovery.kmsSigner = await discoverSigner(deps);
      await regen(state, deps);
      break;
    case "gateway-deploy":
      await composeUp("gateway-mocked-payment", state, deps, saveState, log, [
        "gateway-deploy-mocked-zama-oft",
        "gateway-set-relayer-mocked-payment",
      ]);
      await waitForContainer(deps, "gateway-deploy-mocked-zama-oft", "complete");
      await waitForContainer(deps, "gateway-set-relayer-mocked-payment", "complete");
      await composeUp("gateway-sc", state, deps, saveState, log, ["gateway-sc-deploy"]);
      await waitForContainer(deps, "gateway-sc-deploy", "complete", "Contract deployment done!");
      state.discovery = {
        gateway: await readEnvFile(gatewayAddressesPath),
        host: state.discovery?.host ?? {},
        kmsSigner: state.discovery?.kmsSigner ?? "",
        fheKeyId: state.discovery?.fheKeyId ?? predictedKeyId(),
        crsKeyId: state.discovery?.crsKeyId ?? predictedCrsId(),
        actualFheKeyId: state.discovery?.actualFheKeyId,
        actualCrsKeyId: state.discovery?.actualCrsKeyId,
        endpoints: state.discovery?.endpoints ?? {
          gatewayHttp: "http://gateway-node:8546",
          gatewayWs: "ws://gateway-node:8546",
          hostHttp: "http://host-node:8545",
          hostWs: "ws://host-node:8545",
          minioInternal: "http://minio:9000",
          minioExternal: `http://${await minioIp(deps)}:9000`,
        },
      };
      await regen(state, deps);
      break;
    case "host-deploy":
      await composeUp("host-sc", state, deps, saveState, log, ["host-sc-deploy"]);
      await waitForContainer(deps, "host-sc-deploy", "complete", "Contract deployment done!");
      break;
    case "discover": {
      const contracts = await discoverContracts(deps);
      state.discovery = {
        gateway: contracts.gateway,
        host: contracts.host,
        kmsSigner: state.discovery?.kmsSigner ?? "",
        fheKeyId: state.discovery?.fheKeyId ?? predictedKeyId(),
        crsKeyId: state.discovery?.crsKeyId ?? predictedCrsId(),
        actualFheKeyId: state.discovery?.actualFheKeyId,
        actualCrsKeyId: state.discovery?.actualCrsKeyId,
        endpoints: state.discovery?.endpoints ?? {
          gatewayHttp: "http://gateway-node:8546",
          gatewayWs: "ws://gateway-node:8546",
          hostHttp: "http://host-node:8545",
          hostWs: "ws://host-node:8545",
          minioInternal: "http://minio:9000",
          minioExternal: `http://${await minioIp(deps)}:9000`,
        },
      };
      break;
    }
    case "regenerate":
      await regen(state, deps);
      break;
    case "validate":
      validateDiscovery(state);
      break;
    case "coprocessor":
      await composeUp("coprocessor", state, deps, saveState, log, serviceNameList(state, "coprocessor"));
      await waitForCoprocessor(state, deps);
      break;
    case "kms-connector":
      await composeUp("kms-connector", state, deps, saveState, log);
      await waitForKmsConnector(deps);
      break;
    case "bootstrap":
      await composeUp("gateway-sc", state, deps, saveState, log, ["gateway-sc-add-network"], { noDeps: true });
      await waitForContainer(deps, "gateway-sc-add-network", "complete");
      if (await probeBootstrap(state, deps)) {
        await regen(state, deps);
        break;
      }
      const hostEnv = await readEnvFile(envPath("host-sc"));
      const gatewayEnv = await readEnvFile(envPath("gateway-sc"));
      if (
        !(await pauserRegistered(
          deps,
          state.discovery!.endpoints.hostHttp,
          state.discovery!.host.PAUSER_SET_CONTRACT_ADDRESS,
          hostEnv.PAUSER_ADDRESS_0,
          "isPauser(address)(bool)",
        ))
      ) {
        await composeUp("host-sc", state, deps, saveState, log, ["host-sc-add-pausers"], { noDeps: true });
        await waitForContainer(deps, "host-sc-add-pausers", "complete");
      }
      if (
        !(await pauserRegistered(
          deps,
          state.discovery!.endpoints.gatewayHttp,
          gatewayEnv.PAUSER_SET_ADDRESS,
          gatewayEnv.PAUSER_ADDRESS_0,
          "isPauser(address)(bool)",
        ))
      ) {
        await composeUp("gateway-sc", state, deps, saveState, log, ["gateway-sc-add-pausers"], { noDeps: true });
        await waitForContainer(deps, "gateway-sc-add-pausers", "complete");
      }
      await composeUp("gateway-sc", state, deps, saveState, log, ["gateway-sc-trigger-keygen"], { noDeps: true });
      await waitForContainer(deps, "gateway-sc-trigger-keygen", "complete");
      await composeUp("gateway-sc", state, deps, saveState, log, ["gateway-sc-trigger-crsgen"], { noDeps: true });
      await waitForContainer(deps, "gateway-sc-trigger-crsgen", "complete");
      await waitForBootstrap(state, deps);
      await regen(state, deps);
      break;
    case "relayer":
      await composeUp("relayer", state, deps, saveState, log);
      await waitForContainer(deps, "fhevm-relayer-db", "healthy");
      await waitForContainer(deps, "fhevm-relayer", "running");
      await waitForRelayer(deps);
      break;
    case "test-suite":
      await composeUp("test-suite", state, deps, saveState, log);
      await waitForTestSuite(deps);
      break;
  }
  await markStep(state, step, deps);
};

const startStep = (state: State, options: Pick<UpOptions, "resume" | "fromStep">) => {
  if (options.fromStep) {
    return options.fromStep;
  }
  if (!options.resume || !state.completedSteps.length) {
    return STEP_NAMES[0];
  }
  const remaining = STEP_NAMES.find((step) => !state.completedSteps.includes(step));
  return remaining ?? STEP_NAMES[STEP_NAMES.length - 1];
};

const runUp = async (options: UpOptions, deps: RuntimeDeps) => {
  let state = options.resume ? await loadState() : undefined;
  if (options.resume && !state) {
    throw new Error("No .fhevm/state.json to resume from");
  }
  if (!state) {
    state = await bootstrapState(options, deps);
  }
  if (options.resume && state.target !== options.target) {
    throw new Error(`Resume target mismatch: state=${state.target}, requested=${options.target}`);
  }
  logOverrideWarnings(state.overrides);
  if (options.resume && options.fromStep) {
    await resetAfterStep(options.fromStep, deps);
    state.completedSteps = state.completedSteps.filter(
      (step) => stateStepIndex(step) < stateStepIndex(options.fromStep!),
    );
    await saveState(state);
  }
  const from = startStep(state, options);
  for (const step of STEP_NAMES.slice(stateStepIndex(from))) {
    if (options.resume && state.completedSteps.includes(step) && !options.fromStep) {
      continue;
    }
    await runStep(state, step, deps);
  }
};

const runUpDry = async (options: Omit<UpOptions, "resume" | "dryRun">, deps: RuntimeDeps) => {
  const bundle = await previewBundle(options, deps);
  const state = {
    target: options.target,
    versions: bundle,
    overrides: options.overrides,
    topology: options.topology,
  };
  await preflight(
    {
      target: state.target,
      lockPath: "",
      versions: state.versions,
      overrides: state.overrides,
      topology: state.topology,
      completedSteps: [],
      updatedAt: deps.now(),
    },
    deps,
    true,
    !options.lockFile,
  );
  printBundle(state.versions);
  printPlan(state, options.fromStep);
  log("[dry-run] preflight passed; no state or containers were changed");
};

export const resolveUpgradePlan = (state: Pick<State, "overrides" | "topology">, groupValue: string | undefined) => {
  if (!groupValue || !UPGRADEABLE_GROUPS.includes(groupValue as UpgradeGroup)) {
    throw new Error(`upgrade expects one of ${UPGRADEABLE_GROUPS.join(", ")}`);
  }
  const group = groupValue as UpgradeGroup;
  if (!state.overrides.some((item) => item.group === group)) {
    throw new Error(`upgrade requires an active local override for ${group}`);
  }
  const [component] = GROUP_BUILD_COMPONENTS[group];
  if (!component) {
    throw new Error(`No runtime component registered for ${group}`);
  }
  const groupOverrides = state.overrides.filter((item) => item.group === group);
  const selectedServices = groupOverrides.flatMap((item) => item.services ?? []);
  return {
    component,
    group,
    services:
      group === "coprocessor"
        ? coprocessorServicesForOverrides(state, selectedServices)
        : selectedServices.length
          ? [...new Set(selectedServices)]
          : GROUP_BUILD_SERVICES[group],
    step: group === "coprocessor" ? "coprocessor" : group,
  } as const;
};

const runUpgrade = async (groupValue: string | undefined, deps: RuntimeDeps) => {
  const state = await loadState();
  if (!state) {
    throw new Error("Stack is not running; run `fhevm-cli up --override ...` first");
  }
  const { component, group, services, step } = resolveUpgradePlan(state, groupValue);
  log(`[upgrade] ${group}`);
  await regen(state, deps);
  await composeUp(component, state, deps, saveState, log, services, { noDeps: true });
  if (group === "coprocessor") {
    await waitForCoprocessor(state, deps);
  } else if (group === "kms-connector") {
    await waitForKmsConnector(deps);
  } else {
    await waitForTestSuite(deps);
  }
  await markStep(state, step, deps);
};

const runContractTask = async (
  component: "host-sc" | "gateway-sc",
  service: "host-sc-deploy" | "gateway-sc-deploy",
  command: string,
  deps: RuntimeDeps,
) => {
  const state = await loadState();
  if (!state) {
    throw new Error("Stack is not running; run `fhevm-cli up` first");
  }
  await deps.liveRunner(
    [...dockerArgs(component), "run", "--rm", "--no-deps", "--entrypoint", "sh", service, "-lc", command],
    {
      env: state ? resolvedComposeEnv(state) : undefined,
    },
  );
};

const runPause = async (scope: string | undefined, deps: RuntimeDeps) => {
  if (scope === "host") {
    await runContractTask("host-sc", "host-sc-deploy", "npx hardhat compile && npx hardhat task:pauseACL", deps);
    return;
  }
  if (scope === "gateway") {
    await runContractTask(
      "gateway-sc",
      "gateway-sc-deploy",
      "npx hardhat compile && npx hardhat task:pauseAllGatewayContracts",
      deps,
    );
    return;
  }
  throw new Error("pause expects `host` or `gateway`");
};

const runUnpause = async (scope: string | undefined, deps: RuntimeDeps) => {
  if (scope === "host") {
    await runContractTask("host-sc", "host-sc-deploy", "npx hardhat compile && npx hardhat task:unpauseACL", deps);
    return;
  }
  if (scope === "gateway") {
    await runContractTask(
      "gateway-sc",
      "gateway-sc-deploy",
      "npx hardhat compile && npx hardhat task:unpauseAllGatewayContracts",
      deps,
    );
    return;
  }
  throw new Error("unpause expects `host` or `gateway`");
};

const runDown = async (deps: RuntimeDeps) => {
  let stopped = false;
  for (const component of [...COMPONENTS].reverse()) {
    if (!(await exists(composePath(component)))) {
      continue;
    }
    stopped = true;
    log(`[down] ${component}`);
    await composeDown(component, deps);
  }
  if (!stopped) {
    log("[down] nothing to stop");
  }
};

const runStatus = async (deps: RuntimeDeps) => {
  const state = await loadState();
  if (state) {
    log(`[target] ${state.target}`);
    if (state.overrides.length) {
      log(`[overrides] ${state.overrides.map(describeOverride).join(", ")}`);
      logOverrideWarnings(state.overrides);
    }
    log(`[topology] n=${state.topology.count} t=${state.topology.threshold}`);
    log(`[steps] ${state.completedSteps.join(", ") || "none"}`);
    if (state.builtImages?.length) {
      log(`[owned-images] ${state.builtImages.length}`);
      for (const image of state.builtImages) {
        log(`  ${image.ref} (${image.group})`);
      }
    }
  }
  const ps = await deps.runner(
    ["docker", "ps", "--filter", `label=com.docker.compose.project=${PROJECT}`, "--format", "{{.Names}}\t{{.Status}}"],
    { allowFailure: true },
  );
  log(ps.stdout.trim() || "No fhevm containers");
};

const runLogs = async (service: string | undefined, deps: RuntimeDeps) => {
  const args = [
    "docker",
    "ps",
    "--filter",
    `label=com.docker.compose.project=${PROJECT}`,
    "--format",
    "{{.Names}}",
  ];
  const ps = await deps.runner(args, { allowFailure: true });
  const containers = ps.stdout
    .split("\n")
    .map((item) => item.trim())
    .filter(Boolean)
    .filter((item) => !service || item.includes(service));
  if (!containers.length) {
    throw new Error(`No containers match ${service ?? "fhevm"}`);
  }
  const requested = service ? LOG_TARGETS[service] ?? service : undefined;
  const container = !requested ? containers[0] : containers.find((item) => item === requested) ?? containers.find((item) => item.endsWith(`-${requested}`)) ?? (() => {
    if (containers.length > 1) {
      throw new Error(`Multiple containers match ${service}: ${containers.join(", ")}`);
    }
    return containers[0];
  })();
  await deps.liveRunner(["docker", "logs", "--follow", "--tail", "200", container]);
};

const runClean = async (options: CleanOptions, deps: RuntimeDeps) => {
  log("[clean] start");
  const state = await loadState();
  if (options.images && state?.builtImages?.length) {
    log(`[clean] removing ${state.builtImages.length} owned image${state.builtImages.length === 1 ? "" : "s"}`);
    for (const image of state.builtImages) {
      const currentId = await inspectImageId(deps.runner, image.ref);
      if (!currentId || currentId !== image.id) {
        continue;
      }
      log(`[image] ${image.ref}`);
      await deps.runner(["docker", "image", "rm", image.ref], { allowFailure: true });
    }
  }
  await runDown(deps);
  if (await exists(STATE_DIR)) {
    log(`[clean] removing ${STATE_DIR}`);
  } else {
    log("[clean] no runtime state");
  }
  await remove(STATE_DIR);
  log("[clean] done");
};

const inheritedEnv = (deps: RuntimeDeps) =>
  Object.fromEntries(Object.entries(deps.env).filter((entry): entry is [string, string] => entry[1] !== undefined));

const runWithHeartbeat = async (argv: string[], label: string, deps: RuntimeDeps) => {
  const proc = Bun.spawn(argv, {
    stdin: "inherit",
    stdout: "pipe",
    stderr: "pipe",
    env: inheritedEnv(deps),
  });
  let lastOutput = Date.now();
  let announced = 0;
  const pump = async (stream: ReadableStream<Uint8Array> | null, writer: NodeJS.WriteStream) => {
    if (!stream) {
      return;
    }
    const reader = stream.getReader();
    try {
      while (true) {
        const { done, value } = await reader.read();
        if (done) {
          return;
        }
        if (value?.length) {
          lastOutput = Date.now();
          writer.write(Buffer.from(value));
        }
      }
    } finally {
      reader.releaseLock();
    }
  };
  const timer = setInterval(() => {
    const silent = Date.now() - lastOutput;
    if (silent >= 15_000 && silent - announced >= 15_000) {
      announced = silent;
      log(`[wait] ${label} still running (${Math.floor(silent / 1000)}s)`);
    }
  }, 5_000);
  const [code] = await Promise.all([
    proc.exited,
    pump(proc.stdout, process.stdout),
    pump(proc.stderr, process.stderr),
  ]);
  clearInterval(timer);
  if (code !== 0) {
    throw new Error(`${argv.join(" ")} failed (${code})`);
  }
};

const runTests = async (
  testName: string | undefined,
  grep: string | undefined,
  network: string,
  verbose: boolean,
  deps: RuntimeDeps,
) => {
  const state = await loadState();
  if (!state?.discovery?.actualFheKeyId) {
    throw new Error("Stack has not completed bootstrap; run `fhevm-cli up` first");
  }
  const filter = grep ?? (testName ? TEST_GREP[testName] : TEST_GREP["input-proof"]);
  if (!filter) {
    throw new Error(`Unknown test profile ${testName}`);
  }
  const label = testName ?? "custom";
  log(`[test] ${label} (${network})`);
  const started = Date.now();
  const command = ["cd /app/test-suite/e2e", "&&", "npx hardhat test", "--no-compile", verbose ? "--verbose" : "", "--grep", shellEscape(filter), "--network", shellEscape(network)]
    .filter(Boolean)
    .join(" ");
  try {
    await runWithHeartbeat(["docker", "exec", "fhevm-test-suite-e2e-debug", "sh", "-lc", command], `test ${label}`, deps);
    log(`[pass] ${label} (${Math.round((Date.now() - started) / 1000)}s)`);
  } catch (error) {
    log(`[fail] ${label} (${Math.round((Date.now() - started) / 1000)}s)`);
    throw error;
  }
};

const usage = () => {
  console.log(`Usage: fhevm-cli <command> [options]

Commands:
  up       start or resume the local stack
  deploy   alias for up
  down     stop stack containers
  clean    stop stack containers and delete .fhevm
  status   print state and running containers
  logs     follow container logs
  upgrade  rebuild and restart an active local runtime override
  pause    pause host or gateway contracts
  unpause  unpause host or gateway contracts
  test     run e2e tests in fhevm-test-suite-e2e-debug

up options:
  --target latest-main|latest-release|sha|devnet|testnet|mainnet
  --sha <git-sha>   required with --target sha
  --lock-file <path-to-bundle-json>
  --override <group[:svc1,svc2]>    repeated; groups: ${OVERRIDE_GROUPS.join(", ")}
  --coprocessors <n>
  --threshold <t>
  --instance-env <idx:KEY=VALUE>
  --instance-arg <idx:service=--flag=value>
  --from-step <${STEP_NAMES.join("|")}>   requires --resume, except in --dry-run
  --resume
  --dry-run

clean options:
  --images  remove CLI-owned local override images too
`);
};

export const main = async (argv = process.argv, deps: Partial<RuntimeDeps> = {}) => {
  const runtime = { ...defaultDeps, ...deps };
  try {
    const parsed = parseCli(argv);
    const command = parsed.command === "deploy" ? "up" : parsed.command;
    const fromStep = ensureStep(parsed.parsed.values["from-step"] as string | undefined);
    if (command === "up" && fromStep && !parsed.parsed.values.resume && !parsed.parsed.values["dry-run"]) {
      throw new Error("--from-step requires --resume or --dry-run");
    }
    switch (command) {
      case "up":
        if (parsed.parsed.values["dry-run"]) {
          await runUpDry(
            {
              target: parsed.target,
              sha: parsed.sha,
              overrides: parsed.overrides,
              topology: parsed.topology,
              fromStep,
              lockFile: parsed.parsed.values["lock-file"] as string | undefined,
            },
            runtime,
          );
          return;
        }
        await runUp(
          {
            target: parsed.target,
            sha: parsed.sha,
            overrides: parsed.overrides,
            topology: parsed.topology,
            fromStep,
            lockFile: parsed.parsed.values["lock-file"] as string | undefined,
            resume: parsed.parsed.values.resume,
            dryRun: parsed.parsed.values["dry-run"],
          },
          runtime,
        );
        return;
      case "down":
        await runDown(runtime);
        return;
      case "clean":
        await runClean({ images: parsed.parsed.values.images }, runtime);
        return;
      case "status":
        await runStatus(runtime);
        return;
      case "logs":
        await runLogs(parsed.parsed.positionals[0], runtime);
        return;
      case "upgrade":
        await runUpgrade(parsed.parsed.positionals[0], runtime);
        return;
      case "pause":
        await runPause(parsed.parsed.positionals[0], runtime);
        return;
      case "unpause":
        await runUnpause(parsed.parsed.positionals[0], runtime);
        return;
      case "test":
        await runTests(
          parsed.parsed.positionals[0],
          parsed.parsed.values.grep as string | undefined,
          parsed.parsed.values.network as string,
          parsed.parsed.values.verbose,
          runtime,
        );
        return;
      case "doctor":
        throw new Error("`doctor` was removed; use `fhevm-cli up --dry-run ...`");
      case "help":
      case undefined:
        usage();
        return;
      default:
        throw new Error(`Unknown command ${parsed.command}`);
    }
  } catch (error) {
    console.error(toError(error).message);
    process.exitCode = 1;
  }
};

if (import.meta.main) {
  await main();
}
