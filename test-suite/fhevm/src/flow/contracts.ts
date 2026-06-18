import path from "node:path";

import { PreflightError } from "../errors";
import { resolvedComposeEnv } from "../generate/compose";
import { RUNTIME_DIR, dockerArgs, envPath } from "../layout";
import { loadState } from "../state/state";
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
const componentSurface = (component: "host-sc" | "gateway-sc"): ContractSurface =>
  component === "host-sc" ? "host" : "gateway";
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

export const snapshotContractSources = async (surface: ContractSurface) => {
  const state = await loadState();
  const running = await projectContainers();
  assertContractTaskStackRunning(!!state, running.length);

  const container = surface === "host" ? "host-sc-deploy" : "gateway-sc-deploy";
  const target = previousContractsDir(surface);
  await remove(target);
  await ensureDir(target);
  await run(["docker", "cp", `${container}:/app/contracts/.`, target]);
  console.log(`[rollout] snapshot ${surface} contracts -> ${target}`);
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
