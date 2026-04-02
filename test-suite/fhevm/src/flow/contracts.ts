import { PreflightError } from "../errors";
import { dockerArgs, envPath } from "../layout";
import { loadState } from "../state/state";
import { resolvedComposeEnv } from "../generate/compose";
import { readEnvFileIfExists } from "../utils/fs";
import { runStreaming } from "../utils/process";
import { ensureRuntimeArtifacts } from "./artifacts";
import { projectContainers } from "./runtime-compose";

/** Ensures contract tasks only run against a live stack, not stale persisted state. */
export const assertContractTaskStackRunning = (hasState: boolean, runningContainers: number) => {
  if (!hasState) {
    throw new PreflightError("Stack is not running; run `fhevm-cli up` first");
  }
  if (!runningContainers) {
    throw new PreflightError("Stack is stopped; run `fhevm-cli up --resume` first");
  }
};

/** Runs a host or gateway contract task inside its deploy container. */
export const runContractTask = async (
  component: "host-sc" | "gateway-sc",
  service: "host-sc-deploy" | "gateway-sc-deploy",
  command: string,
) => {
  const state = await loadState();
  if (!state) {
    assertContractTaskStackRunning(false, 0);
    return;
  }
  const runningState = state;
  assertContractTaskStackRunning(true, (await projectContainers()).length);
  await ensureRuntimeArtifacts(runningState, "contract task");
  await runStreaming(
    [...dockerArgs(component), "run", "--rm", "--no-deps", "--entrypoint", "sh", service, "-lc", command],
    { env: { ...resolvedComposeEnv(runningState), ...(await readEnvFileIfExists(envPath(component))) } },
  );
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
