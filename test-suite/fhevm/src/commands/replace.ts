/**
 * Hot-swaps locally built binaries into running stack containers for debug loops.
 */
import path from "node:path";

import { PreflightError } from "../errors";
import { PROJECT, REPO_ROOT, RUNTIME_DIR, defaultHostChainKey, hostChainSuffix } from "../layout";
import { dockerInspect, postBootHealthGate, waitForContainer } from "../flow/readiness";
import { loadState } from "../state/state";
import { topologyForState } from "../stack-spec/stack-spec";
import type { State } from "../types";
import { ensureDir, exists } from "../utils/fs";
import { run, runWithHeartbeat } from "../utils/process";

type LocalBuild = {
  workspaceId: string;
  binaryName: string;
  cwd: string;
  package: string;
  env?: Record<string, string>;
};

type ReplaceTarget = {
  key: string;
  description: string;
  defaultBinary: string;
  containerPath: string;
  localBuild: LocalBuild;
  containers: (state: State) => string[];
};

const LINUX_RUST_IMAGE =
  "ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.91.0@sha256:5c561870f4cc043ff53f415e986ffdbc1099541e5f2a3e487e40858f88f0d67f";
const CONTAINER_REPO_ROOT = "/workspace/fhevm";
const CONTAINER_BUILD_HOME = "/tmp/fhevm-replace-home";

const coprocessorPrefix = (index: number) => (index === 0 ? "coprocessor-" : `coprocessor${index}-`);

const coprocessorRuntimeContainers = (suffix: string) => (state: State) =>
  Array.from({ length: topologyForState(state).count }, (_, index) => `${coprocessorPrefix(index)}${suffix}`);

const coprocessorChainContainers = (suffix: string) => (state: State) => {
  const defaultKey = defaultHostChainKey(state.scenario.hostChains);
  const chains = state.scenario.hostChains.length ? state.scenario.hostChains : [{ key: defaultKey }];
  return chains.flatMap((chain) => {
    const chainSuffix = hostChainSuffix(chain.key, defaultKey);
    return Array.from({ length: topologyForState(state).count }, (_, index) => `${coprocessorPrefix(index)}${suffix}${chainSuffix}`);
  });
};

const coprocessorBuild = (packageName: string, binaryName: string): LocalBuild => ({
  workspaceId: "coprocessor",
  binaryName,
  cwd: path.join(REPO_ROOT, "coprocessor", "fhevm-engine"),
  package: packageName,
  env: { SQLX_OFFLINE: "true", BUILD_ID: "local" },
});

const kmsConnectorBuild = (packageName: string, binaryName: string): LocalBuild => ({
  workspaceId: "kms-connector",
  binaryName,
  cwd: path.join(REPO_ROOT, "kms-connector"),
  package: packageName,
});

export const REPLACE_TARGETS: ReplaceTarget[] = [
  {
    key: "coprocessor:host-listener",
    description: "Replace host-listener binaries on every active host chain and coprocessor instance.",
    defaultBinary: "coprocessor/fhevm-engine/target/release/host_listener",
    containerPath: "/usr/local/bin/host_listener",
    localBuild: coprocessorBuild("host-listener", "host_listener"),
    containers: coprocessorChainContainers("host-listener"),
  },
  {
    key: "coprocessor:host-listener-poller",
    description: "Replace host-listener-poller binaries on every active host chain and coprocessor instance.",
    defaultBinary: "coprocessor/fhevm-engine/target/release/host_listener_poller",
    containerPath: "/usr/local/bin/host_listener_poller",
    localBuild: coprocessorBuild("host-listener", "host_listener_poller"),
    containers: coprocessorChainContainers("host-listener-poller"),
  },
  {
    key: "coprocessor:host-listener-consumer",
    description: "Replace host-listener-consumer binaries on every active coprocessor instance.",
    defaultBinary: "coprocessor/fhevm-engine/target/release/host_listener_consumer",
    containerPath: "/usr/local/bin/host_listener_consumer",
    localBuild: coprocessorBuild("host-listener", "host_listener_consumer"),
    containers: coprocessorRuntimeContainers("host-listener-consumer"),
  },
  {
    key: "coprocessor:gw-listener",
    description: "Replace gateway-listener binaries on every active coprocessor instance.",
    defaultBinary: "coprocessor/fhevm-engine/target/release/gw_listener",
    containerPath: "/usr/local/bin/gw_listener",
    localBuild: coprocessorBuild("gw-listener", "gw_listener"),
    containers: coprocessorRuntimeContainers("gw-listener"),
  },
  {
    key: "coprocessor:tfhe-worker",
    description: "Replace TFHE worker binaries on every active coprocessor instance.",
    defaultBinary: "coprocessor/fhevm-engine/target/release/tfhe_worker",
    containerPath: "/usr/local/bin/tfhe_worker",
    localBuild: coprocessorBuild("tfhe-worker", "tfhe_worker"),
    containers: coprocessorRuntimeContainers("tfhe-worker"),
  },
  {
    key: "coprocessor:sns-worker",
    description: "Replace SNS worker binaries on every active coprocessor instance.",
    defaultBinary: "coprocessor/fhevm-engine/target/release/sns_worker",
    containerPath: "/usr/local/bin/sns_worker",
    localBuild: coprocessorBuild("sns-worker", "sns_worker"),
    containers: coprocessorRuntimeContainers("sns-worker"),
  },
  {
    key: "coprocessor:transaction-sender",
    description: "Replace transaction-sender binaries on every active coprocessor instance.",
    defaultBinary: "coprocessor/fhevm-engine/target/release/transaction_sender",
    containerPath: "/usr/local/bin/transaction_sender",
    localBuild: coprocessorBuild("transaction-sender", "transaction_sender"),
    containers: coprocessorRuntimeContainers("transaction-sender"),
  },
  {
    key: "coprocessor:zkproof-worker",
    description: "Replace ZK proof worker binaries on every active coprocessor instance.",
    defaultBinary: "coprocessor/fhevm-engine/target/release/zkproof_worker",
    containerPath: "/usr/local/bin/zkproof_worker",
    localBuild: coprocessorBuild("zkproof-worker", "zkproof_worker"),
    containers: coprocessorRuntimeContainers("zkproof-worker"),
  },
  {
    key: "kms-connector:gw-listener",
    description: "Replace the KMS connector gateway listener binary.",
    defaultBinary: "kms-connector/target/release/gw-listener",
    containerPath: "/app/kms-connector/bin/gw-listener",
    localBuild: kmsConnectorBuild("gw-listener", "gw-listener"),
    containers: () => ["kms-connector-gw-listener"],
  },
  {
    key: "kms-connector:kms-worker",
    description: "Replace the KMS connector KMS worker binary.",
    defaultBinary: "kms-connector/target/release/kms-worker",
    containerPath: "/app/kms-connector/bin/kms-worker",
    localBuild: kmsConnectorBuild("kms-worker", "kms-worker"),
    containers: () => ["kms-connector-kms-worker"],
  },
  {
    key: "kms-connector:tx-sender",
    description: "Replace the KMS connector transaction sender binary.",
    defaultBinary: "kms-connector/target/release/tx-sender",
    containerPath: "/app/kms-connector/bin/tx-sender",
    localBuild: kmsConnectorBuild("tx-sender", "tx-sender"),
    containers: () => ["kms-connector-tx-sender"],
  },
];

export const REPLACE_TARGET_NAMES = REPLACE_TARGETS.map((target) => target.key);

const targetByName = new Map(REPLACE_TARGETS.map((target) => [target.key, target] as const));

const dockerProjectContainers = async () => {
  const result = await run(
    ["docker", "ps", "--filter", `label=com.docker.compose.project=${PROJECT}`, "--format", "{{.Names}}"],
    { allowFailure: true },
  );
  if (result.code !== 0) {
    throw new PreflightError(`Docker unavailable: ${result.stderr.trim() || "docker ps failed"}`);
  }
  return new Set(result.stdout.split("\n").map((name) => name.trim()).filter(Boolean));
};

export const replaceTargetsForState = (state: State, name: string) => {
  const target = targetByName.get(name);
  if (!target) {
    throw new PreflightError(`Unknown replace target ${name}. Valid: ${REPLACE_TARGET_NAMES.join(", ")}`);
  }
  return {
    ...target,
    binary: path.join(REPO_ROOT, target.defaultBinary),
    containers: target.containers(state),
  };
};

/** Prints the replace target catalog, optionally with active-stack container names. */
export const listReplaceTargets = async () => {
  const state = await loadState();
  for (const target of REPLACE_TARGETS) {
    console.log(target.key);
    console.log(`  ${target.description}`);
    console.log(`  default binary: ${target.defaultBinary}`);
    console.log(`  local build: cached Linux cargo build for package ${target.localBuild.package}`);
    console.log(`  container path: ${target.containerPath}`);
    if (state) {
      console.log(`  containers: ${target.containers(state).join(", ")}`);
    }
  }
};

const waitForRestart = async (container: string) => {
  const [inspect] = await dockerInspect(container);
  await waitForContainer(container, inspect?.State.Health ? "healthy" : "running");
};

const cargoProfileArgs = (profile: string) => {
  if (profile === "dev") return [];
  if (profile === "release") return ["--release"];
  return ["--profile", profile];
};

const cargoProfileDir = (profile: string) => (profile === "dev" ? "debug" : profile);

export const parseBuildProfile = (profile: string | undefined) => {
  const value = profile ?? "dev";
  if (/^[A-Za-z0-9][A-Za-z0-9._-]*$/.test(value)) {
    return value;
  }
  throw new PreflightError(
    `Invalid build profile ${value}; expected an alphanumeric name followed by letters, digits, dot, underscore, or dash`,
  );
};

const replaceBuildRoot = (build: Pick<LocalBuild, "workspaceId">) =>
  path.join(RUNTIME_DIR, "replace-build", build.workspaceId);

export const replaceBuildTargetDir = (build: Pick<LocalBuild, "workspaceId">) =>
  path.join(replaceBuildRoot(build), "target");

export const replaceBuildBinary = (build: LocalBuild, profile: string) =>
  path.join(replaceBuildTargetDir(build), cargoProfileDir(profile), build.binaryName);

const dockerUserArgs = () =>
  typeof process.getuid === "function" && typeof process.getgid === "function"
    ? ["--user", `${process.getuid()}:${process.getgid()}`]
    : [];

const containerImagePlatform = async (container: string) => {
  const containerImage = (
    await run(["docker", "inspect", "--format", "{{.Image}}", container])
  ).stdout.trim();
  if (!containerImage) {
    throw new PreflightError(`Could not resolve image id for ${container}`);
  }
  const platform = (
    await run(["docker", "image", "inspect", "--format", "{{.Os}}/{{.Architecture}}", containerImage])
  ).stdout.trim();
  if (!platform.startsWith("linux/")) {
    throw new PreflightError(`replace --local-build only supports Linux runtime containers; ${container} uses ${platform}`);
  }
  return platform;
};

const replacementPlatform = async (containers: string[]) => {
  const platforms = new Set<string>();
  for (const container of containers) {
    platforms.add(await containerImagePlatform(container));
  }
  if (platforms.size !== 1) {
    throw new PreflightError(`replace --local-build expected one runtime platform, got ${[...platforms].join(", ")}`);
  }
  return [...platforms][0];
};

const runLocalBuilds = async (
  plans: Array<{ containers: string[]; localBuild: LocalBuild }>,
  profile: string,
) => {
  const builds = new Map<string, LocalBuild>();
  for (const plan of plans) {
    builds.set(`${plan.localBuild.workspaceId}:${plan.localBuild.package}`, plan.localBuild);
  }
  const platform = await replacementPlatform(plans.flatMap((plan) => plan.containers));
  const cacheRoot = path.join(RUNTIME_DIR, "replace-build", "cargo");
  const homeRoot = path.join(RUNTIME_DIR, "replace-build", "home");
  await Promise.all([
    ensureDir(path.join(cacheRoot, "registry")),
    ensureDir(path.join(cacheRoot, "git")),
    ensureDir(homeRoot),
    ...[...builds.values()].map((build) => ensureDir(replaceBuildTargetDir(build))),
  ]);
  for (const build of builds.values()) {
    const workspace = path.relative(REPO_ROOT, build.cwd);
    const label = `build ${build.package} (${profile}, ${platform})`;
    const env = build.env ?? {};
    console.log(`[replace] ${label}`);
    await runWithHeartbeat(
      [
        "docker",
        "run",
        "--rm",
        "--platform",
        platform,
        ...dockerUserArgs(),
        "--mount",
        `type=bind,source=${REPO_ROOT},target=${CONTAINER_REPO_ROOT},readonly`,
        "-v",
        `${path.join(cacheRoot, "registry")}:/usr/local/cargo/registry`,
        "-v",
        `${path.join(cacheRoot, "git")}:/usr/local/cargo/git`,
        "-v",
        `${homeRoot}:${CONTAINER_BUILD_HOME}`,
        "-v",
        `${replaceBuildTargetDir(build)}:${path.posix.join(CONTAINER_REPO_ROOT, workspace, "target")}`,
        "-w",
        path.posix.join(CONTAINER_REPO_ROOT, workspace),
        "-e",
        `HOME=${CONTAINER_BUILD_HOME}`,
        "-e",
        `GIT_CONFIG_GLOBAL=${path.posix.join(CONTAINER_BUILD_HOME, ".gitconfig")}`,
        ...Object.entries(env).flatMap(([key, value]) => ["-e", `${key}=${value}`]),
        LINUX_RUST_IMAGE,
        "sh",
        "-lc",
        `git config --global --add safe.directory ${CONTAINER_REPO_ROOT} && cargo build ${cargoProfileArgs(profile).join(" ")} -p ${build.package}`,
      ],
      label,
    );
  }
};

/** Replaces selected running service binaries and restarts the affected containers. */
export const replace = async (
  names: string[],
  options: { binary?: string; localBuild?: boolean; buildProfile?: string } = {},
) => {
  if (!names.length || (names.length === 1 && names[0] === "list")) {
    await listReplaceTargets();
    return;
  }
  if (options.binary && names.length !== 1) {
    throw new PreflightError("--binary can only be used with one replace target");
  }
  if (options.binary && options.localBuild) {
    throw new PreflightError("--binary cannot be combined with --local-build");
  }
  if (options.buildProfile && !options.localBuild) {
    throw new PreflightError("--build-profile can only be used with --local-build");
  }
  const buildProfile = parseBuildProfile(options.buildProfile);
  const state = await loadState();
  if (!state) {
    throw new PreflightError("replace requires persisted stack state; run `fhevm-cli up` first");
  }
  const running = await dockerProjectContainers();
  if (!running.size) {
    throw new PreflightError("replace requires a running stack; run `fhevm-cli up --resume` first");
  }
  const plans = names.map((name) => {
    const plan = replaceTargetsForState(state, name);
    const binary = options.binary
      ? path.resolve(options.binary)
      : options.localBuild
        ? replaceBuildBinary(plan.localBuild, buildProfile)
        : plan.binary;
    const containers = plan.containers.filter((container) => running.has(container));
    if (!containers.length) {
      throw new PreflightError(`No running containers found for ${name}; run \`fhevm-cli replace list\``);
    }
    return { ...plan, name, binary, containers };
  });
  if (options.localBuild) {
    await runLocalBuilds(plans, buildProfile);
  }
  for (const plan of plans) {
    if (!(await exists(plan.binary))) {
      throw new PreflightError(
        `Missing binary for ${plan.name}: expected ${plan.binary}\nBuild it first or rerun \`fhevm-cli replace ${plan.name} --local-build\``,
      );
    }
  }
  const restarted = new Set<string>();
  for (const plan of plans) {
    console.log(`[replace] ${plan.name}`);
    for (const container of plan.containers) {
      console.log(`[replace] copy ${plan.binary} -> ${container}:${plan.containerPath}`);
      await run(["docker", "cp", plan.binary, `${container}:${plan.containerPath}`]);
    }
    console.log(`[replace] restart ${plan.containers.join(", ")}`);
    await run(["docker", "restart", ...plan.containers]);
    await Promise.all(plan.containers.map(waitForRestart));
    plan.containers.forEach((container) => restarted.add(container));
  }
  if (restarted.size) {
    await postBootHealthGate([...restarted]);
  }
};
