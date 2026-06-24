import path from "node:path";

import { PreflightError } from "../errors";
import { resolvedComposeEnv } from "../generate/compose";
import { RUNTIME_DIR, dockerArgs, envPath } from "../layout";
import { loadState } from "../state/state";
import type { State } from "../types";
import { ensureDir, exists, readEnvFileIfExists, remove } from "../utils/fs";
import { run, runStreaming } from "../utils/process";
import { ensureRuntimeArtifacts } from "./artifacts";
import { maybeBuild, projectContainers } from "./runtime-compose";

/** Ensures contract tasks only run against a live stack, not stale persisted state. */
export const assertContractTaskStackRunning = (hasState: boolean, runningContainers: number) => {
  if (!hasState) {
    throw new PreflightError("Stack is not running; run `fhevm-cli up` first");
  }
  if (!runningContainers) {
    throw new PreflightError("Stack is stopped; run `fhevm-cli up --resume` first");
  }
};

export const contractTaskEnvArgs = (env: Record<string, string> = {}) =>
  Object.entries(env).flatMap(([key, value]) => ["--env", `${key}=${value}`]);

export type ContractSurface = "host" | "gateway";
type ContractComponent = "host-sc" | "gateway-sc";
type DeployService = "host-sc-deploy" | "gateway-sc-deploy";
const componentSurface = (component: ContractComponent): ContractSurface =>
  component === "host-sc" ? "host" : "gateway";
const surfaceComponent = (surface: ContractSurface): ContractComponent =>
  surface === "host" ? "host-sc" : "gateway-sc";
const deployService = (surface: ContractSurface): DeployService =>
  surface === "host" ? "host-sc-deploy" : "gateway-sc-deploy";
const previousContractsDir = (surface: ContractSurface) =>
  path.join(RUNTIME_DIR, "previous-contracts", surface);
const previousContractsSnapshotPath = "/app/previous-contracts-snapshot";
const previousContractsWorkPath = "/app/previous-contracts";
export const previousContractsMountArgs = (surface: ContractSurface, present: boolean) =>
  present ? ["--volume", `${previousContractsDir(surface)}:${previousContractsSnapshotPath}:ro`] : [];
const previousContractsVolume = async (component: "host-sc" | "gateway-sc") => {
  const surface = componentSurface(component);
  return previousContractsMountArgs(surface, await exists(previousContractsDir(surface)));
};

export const withPreviousContractsSnapshot = (command: string) => `
set -e
if [ -d ${previousContractsSnapshotPath} ]; then
  rm -rf ${previousContractsWorkPath}
  mkdir -p ${previousContractsWorkPath}
  cp -R ${previousContractsSnapshotPath}/. ${previousContractsWorkPath}
  chmod -R u+rwX ${previousContractsWorkPath}
fi
${command}
`;

// Copies /app/contracts out of a throwaway container started from the
// currently-locked deploy image. The persistent <surface>-sc-deploy container
// keeps the image it booted with, so after a contract hop its sources are stale;
// a multi-hop rollout needs the just-deployed (mid-version) sources as the next
// upgrade baseline. A detached idle container skips the deploy entrypoint, and
// docker cp writes host-side so no in-container bind-mount write access is needed.
const snapshotFromLockedImage = async (surface: ContractSurface, state: State, target: string) => {
  const component = surfaceComponent(surface);
  const service = deployService(surface);
  await ensureRuntimeArtifacts(state, "contract snapshot");
  await maybeBuild(component, state);
  const env = { ...resolvedComposeEnv(state), ...(await readEnvFileIfExists(envPath(component))) };
  const created = await run(
    [...dockerArgs(component), "run", "--detach", "--no-deps", "--entrypoint", "sh", service, "-c", "sleep 600"],
    { env },
  );
  const containerId = created.stdout.trim().split("\n").filter(Boolean).pop();
  if (!containerId) {
    throw new PreflightError(`could not start a ${service} container to snapshot the locked contract sources`);
  }
  try {
    await run(["docker", "cp", `${containerId}:/app/contracts/.`, target]);
  } finally {
    await run(["docker", "rm", "-f", containerId], { allowFailure: true });
  }
};

export const snapshotContractSources = async (
  surface: ContractSurface,
  options: { fromLockedImage?: boolean } = {},
) => {
  const state = await loadState();
  const running = await projectContainers();
  assertContractTaskStackRunning(!!state, running.length);

  const target = previousContractsDir(surface);
  await remove(target);
  await ensureDir(target);
  if (options.fromLockedImage) {
    await snapshotFromLockedImage(surface, state as State, target);
  } else {
    await run(["docker", "cp", `${deployService(surface)}:/app/contracts/.`, target]);
  }
  const source = options.fromLockedImage ? " (locked image)" : "";
  console.log(`[rollout] snapshot ${surface} contracts -> ${target}${source}`);
};

/** Runs a host or gateway contract task inside its deploy container. */
export const runContractTask = async (
  component: "host-sc" | "gateway-sc",
  service: "host-sc-deploy" | "gateway-sc-deploy",
  command: string,
  options: { env?: Record<string, string> } = {},
) => {
  const state = await loadState();
  if (!state) {
    throw new PreflightError("Stack is not running; run `fhevm-cli up` first");
  }
  const runningState = state;
  assertContractTaskStackRunning(true, (await projectContainers()).length);
  await ensureRuntimeArtifacts(runningState, "contract task");
  await maybeBuild(component, runningState);
  const argv = [
    ...dockerArgs(component),
    "run",
    "--rm",
    "--no-deps",
    ...contractTaskEnvArgs(options.env),
    ...(await previousContractsVolume(component)),
    "--entrypoint",
    "sh",
    service,
    "-lc",
    withPreviousContractsSnapshot(command),
  ];
  const env = { ...resolvedComposeEnv(runningState), ...(await readEnvFileIfExists(envPath(component))), ...options.env };
  await runStreaming(argv, { env });
};

/** Pauses the requested contract surface through its deploy task. */
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

/** Unpauses the requested contract surface through its deploy task. */
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
